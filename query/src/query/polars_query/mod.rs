use polars::frame::DataFrame;
use polars::frame::hash_join::JoinType as PolarsJoinType;
use polars::lazy::prelude::lit;
use polars::lazy::frame::LazyFrame;
use polars::prelude::col;
use polars::prelude::concat;
use polars::prelude::Expr;
use polars::prelude::LazyCsvReader;

use crate::data_type::DataType;
use crate::doc::JoinColumn;
use crate::doc::JoinType;
use crate::doc::SelectColumn;
use crate::doc::SortDirection;
use crate::doc::Sorter;
use crate::error::PoldaError;
use crate::doc::Aggregate;
use crate::doc::AggregateComputation;
use crate::doc::Node;
use crate::doc::Filter;
use crate::doc::FilterPredicate;
use crate::doc::Value;
use super::Schema;

#[derive(Clone)]
pub struct PolarsQuery {
    pub frame: LazyFrame,
    pub schema: Schema
}

impl PolarsQuery {
    pub fn collect(self) -> Result<DataFrame, PoldaError> {
        Ok(self.frame.collect()?)
    }

    pub fn from_node(
        node: &Node,
        inputs: Vec<PolarsQuery>
    ) -> Result<PolarsQuery, PoldaError> {
        let input_schemas = inputs
            .iter()
            .map(|input| input.schema.clone())
            .collect();
        let schema = Schema::try_from_node(node, input_schemas)?;

        let frame: LazyFrame = match node {
            Node::Aggregate {
                id: _,
                position: _,
                input: _,
                aggregates,
                outputs: _
            } => {
                let mut groups = vec![];
                let mut aggs = vec![];

                for agg in aggregates.into_iter() {
                    let Aggregate { column, computation, alias } = agg;
                    let expr = col(&*column);
                    use AggregateComputation::*;
                    let expr = match computation {
                        Count => expr.count(),
                        Group => expr,
                        Max => expr.max(),
                        Mean => expr.mean(),
                        Median => expr.median(),
                        Min => expr.min(),
                        Sum => expr.sum()
                    };
                    let expr = if !alias.is_empty() {
                        expr.alias(&*alias)
                    } else {
                        expr
                    };
                    if let Group = computation {
                        groups.push(expr);
                    } else {
                        aggs.push(expr);
                    }
                }
                inputs
                    .into_iter()
                    .next()
                    .unwrap()
                    .frame
                    .groupby(&*groups)
                    .agg(&*aggs)
            }

            Node::Filter {
                id: _,
                position: _,
                input: _,
                filters,
                outputs: _
            } => {
                let mut exprs = vec![];

                for filter in filters.iter() {
                    let Filter { column, predicate } = filter;
                    let col_expr = col(column);
                    use FilterPredicate::*;
                    match predicate {
                        IsEqualTo(value) => {
                            let comp_expr = match value {
                                Value::Column(column) => col(column),
                                Value::Constant(constant) => {
                                    schema.0
                                        .get(column)
                                        .map(|dtype| parse_constant_expr(constant.as_str(), dtype))
                                        .ok_or(PoldaError::QueryError(format!("Column \"{}\" does not exist", {column})))??
                                }
                            };
                            let expr = col_expr.eq(comp_expr);
                            exprs.push(expr);
                        }

                        _ => todo!()
                    }
                }

                let frame = inputs
                    .into_iter()
                    .next()
                    .unwrap()
                    .frame;
                exprs
                    .into_iter()
                    .fold(frame, |frame, filter| {
                        frame.filter(filter)
                    })
            }

            Node::Join {
                id: _,
                position: _,
                left_input: _,
                right_input: _,
                join_type,
                columns,
                outputs: _
            } => {
                let mut inputs = inputs.into_iter();
                let left = inputs.next().unwrap();
                let right = inputs.next().unwrap();
                let (left, right) = if let JoinType::Right = join_type {
                    (right, left)
                } else {
                    (left, right)
                };
                let mut left_exprs = vec![];
                let mut right_exprs = vec![];
                for column in columns.iter() {
                    let JoinColumn { left, right } = column;
                    left_exprs.push(col(&*left));
                    right_exprs.push(col(&*right));
                }
                match join_type {
                    JoinType::Inner => {
                        left.frame.join(
                            right.frame,
                            left_exprs,
                            right_exprs,
                            PolarsJoinType::Inner
                        )
                    }
                    JoinType::Left => {
                        left.frame.join(
                            right.frame,
                            left_exprs,
                            right_exprs,
                            PolarsJoinType::Left
                        )
                    }
                    JoinType::Right => {
                        // Switch the frames and the expresions.
                        right.frame.join(
                            left.frame,
                            right_exprs,
                            left_exprs,
                            PolarsJoinType::Left
                        )
                    }
                    JoinType::Full => {
                        left.frame.join(
                            right.frame,
                            left_exprs,
                            right_exprs,
                            PolarsJoinType::Outer
                        )
                    }
                    JoinType::Cross => left.frame.cross_join(right.frame)
                }
            }

            Node::LoadCsv {
                id: _,
                position: _,
                path,
                outputs: _
            } => {
                LazyCsvReader::new(path).finish()?
            }

            Node::Select {
                id: _,
                position: _,
                input: _,
                columns,
                outputs: _
            } => {
                let mut exprs = vec![];
                for column in columns.iter() {
                    let SelectColumn { column, alias } = column;
                    let expr = if !alias.is_empty() {
                        col(&*column).alias(&*alias)
                    } else {
                        col(&*column)
                    };
                    exprs.push(expr);
                }
                inputs
                    .into_iter()
                    .next()
                    .unwrap()
                    .frame
                    .select(&exprs)
            }

            Node::Sort {
                id: _,
                position: _,
                input: _,
                sorters,
                outputs: _
            } => {
                let mut exprs = vec![];
                let mut reverses = vec![];
                for sorter in sorters.into_iter() {
                    let Sorter { column, direction } = sorter;
                    let expr = col(&*column);
                    exprs.push(expr);
                    let reverse = if let SortDirection::Desc = direction {
                        true
                    } else {
                        false
                    };
                    reverses.push(reverse);
                }
                inputs
                    .into_iter()
                    .next()
                    .unwrap()
                    .frame
                    .sort_by_exprs(exprs, reverses, true)
            }

            Node::Union {
                id: _,
                position: _,
                primary_input: _,
                secondary_input: _,
                outputs: _
            } => {
                let mut inputs = inputs.into_iter();
                let first = inputs.next().unwrap();
                let second = inputs.next().unwrap();
                concat([first.frame, second.frame], false, true)?
            }
        };

        Ok(PolarsQuery::new(frame, schema))
    }

    pub fn new(frame: LazyFrame, schema: Schema) -> PolarsQuery {
        PolarsQuery { frame, schema }
    }
}

pub fn parse_constant_expr(
    constant: &str,
    dtype: &DataType
) -> Result<Expr, PoldaError> {
    match dtype {
        DataType::Boolean => {
            constant.parse::<bool>()
                .map(|constant| lit(constant))
                .map_err(|_| PoldaError::ParseError(format!("Can't parse \"{}\" into a Boolean", constant)))
        }
        DataType::Float32 => {
            constant.parse::<f32>()
                .map(|constant| lit(constant))
                .map_err(|_| PoldaError::ParseError(format!("Can't parse \"{}\" into a Float32", constant)))
        }
        DataType::Float64 => {
            constant.parse::<f64>()
                .map(|constant| lit(constant))
                .map_err(|_| PoldaError::ParseError(format!("Can't parse \"{}\" into a Float64", constant)))
        }
        DataType::Int8 => {
            constant.parse::<i8>()
                .map(|constant| lit(constant))
                .map_err(|_| PoldaError::ParseError(format!("Can't parse \"{}\" into a Int8", constant)))
        }
        DataType::Int16 => {
            constant.parse::<i16>()
                .map(|constant| lit(constant))
                .map_err(|_| PoldaError::ParseError(format!("Can't parse \"{}\" into a Int16", constant)))
        }
        DataType::Int32 => {
            constant.parse::<i32>()
                .map(|constant| lit(constant))
                .map_err(|_| PoldaError::ParseError(format!("Can't parse \"{}\" into a Int32", constant)))
        }
        DataType::Int64 => {
            constant.parse::<i64>()
                .map(|constant| lit(constant))
                .map_err(|_| PoldaError::ParseError(format!("Can't parse \"{}\" into a Int64", constant)))
        }
        DataType::UInt8 => {
            constant.parse::<u8>()
                .map(|constant| lit(constant))
                .map_err(|_| PoldaError::ParseError(format!("Can't parse \"{}\" into a UInt8", constant)))
        }
        DataType::UInt16 => {
            constant.parse::<u16>()
                .map(|constant| lit(constant))
                .map_err(|_| PoldaError::ParseError(format!("Can't parse \"{}\" into a UInt16", constant)))
        }
        DataType::UInt32 => {
            constant.parse::<u32>()
                .map(|constant| lit(constant))
                .map_err(|_| PoldaError::ParseError(format!("Can't parse \"{}\" into a UInt32", constant)))
        }
        DataType::UInt64 => {
            constant.parse::<u64>()
                .map(|constant| lit(constant))
                .map_err(|_| PoldaError::ParseError(format!("Can't parse \"{}\" into a UInt64", constant)))
        }
        DataType::Utf8 => {
            Ok(lit(constant))
        }
        _ => todo!()
    }
}
