use polars::frame::DataFrame;
use polars::frame::hash_join::JoinType as PolarsJoinType;
use polars::lazy::prelude::lit;
use polars::lazy::frame::LazyFrame;
use polars::prelude::Null;
use polars::prelude::col;
use polars::prelude::concat;
use polars::prelude::Expr;
use polars::prelude::LazyCsvReader;
use polars::prelude::Literal;
use polars::prelude::when;
use polars::prelude::WhenThen;
use polars::prelude::WhenThenThen;
use std::ops::Add;
use std::ops::Div;
use std::ops::Mul;
use std::ops::Sub;

use crate::data_type::DataType;
use crate::doc::Case;
use crate::doc::ComputeOperation;
use crate::doc::JoinColumn;
use crate::doc::JoinType;
use crate::doc::SelectColumn;
use crate::doc::SortDirection;
use crate::doc::Sorter;
use crate::error::PoldaError;
use crate::doc::Aggregate;
use crate::doc::AggregateComputation;
use crate::doc::Node;
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
                        First => expr.first(),
                        Group => expr,
                        Last => expr.last(),
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

            Node::Bins {
                id: _,
                position: _,
                input: _,
                name: _,
                column: _,
                lower_bound: _,
                upper_bound: _,
                count: _,
                outputs: _
            } => {
                // TODO: We haven't do anything here.
                unimplemented!("Bins query is not implemented")
            }

            Node::Case {
                id: _,
                position: _,
                input: _,
                name,
                data_type,
                cases,
                default,
                outputs: _
            } => {
                let mut frame = inputs
                    .into_iter()
                    .next()
                    .unwrap()
                    .frame;

                let mut when_then = WhenThenMaybeThen::None;

                for Case { column, value } in cases.iter() {
                    let value = value_to_expr(value, data_type)?;

                    when_then = match when_then {
                        WhenThenMaybeThen::None => {
                            let wt = when(col(&**column))
                                .then(value);
                            WhenThenMaybeThen::WhenThen(wt)
                        }
                        WhenThenMaybeThen::WhenThen(wt) => {
                            let wt = wt.when(col(&**column))
                                .then(value);
                            WhenThenMaybeThen::WhenThenThen(wt)
                        }
                        WhenThenMaybeThen::WhenThenThen(wt) => {
                            let wt = wt
                                .when(col(&**column))
                                .then(value);
                            WhenThenMaybeThen::WhenThenThen(wt)
                        }
                    };
                }

                let default = value_to_expr(default, data_type)?;

                frame = match when_then {
                    WhenThenMaybeThen::WhenThen(wt) => {
                        frame.with_column(wt.otherwise(default).alias(&*name))
                    }
                    WhenThenMaybeThen::WhenThenThen(wt) => {
                        frame.with_column(wt.otherwise(default).alias(&*name))
                    }
                    _ => frame
                };

                frame
            }

            Node::Cast {
                id: _,
                position: _,
                input: _,
                name,
                column,
                data_type,
                outputs: _
            } => {
                let frame = inputs
                    .into_iter()
                    .next()
                    .unwrap()
                    .frame;
                let mut expr = col(&**column)
                    .cast(data_type.into_polars());
                expr = expr.alias(&**name);
                frame.with_column(expr)
            }

            Node::Compute {
                id: _,
                position: _,
                input: _,
                name,
                column,
                operation,
                outputs: _
            } => {
                let mut frame = inputs
                    .into_iter()
                    .next()
                    .unwrap()
                    .frame;

                let mut expr = col(&**column);
                // Column is guaranteed to exists by the Schema builder.
                let dtype = schema.0.get(column).unwrap();

                use ComputeOperation::*;
                expr = match operation {
                    Add(v) => expr.add(value_to_expr(v, dtype)?),

                    Subtract(v) => expr.sub(value_to_expr(v, dtype)?),

                    Multiply(v) => expr.mul(value_to_expr(v, dtype)?),

                    Divide(v) => expr.div(value_to_expr(v, dtype)?),

                    IsEqualTo(v) => expr.eq(value_to_expr(v, dtype)?),

                    IsNotEqualTo(v) => expr.neq(value_to_expr(v, dtype)?),

                    IsLessThan(v) => expr.lt(value_to_expr(v, dtype)?),

                    IsLessThanEqual(v) => expr.lt_eq(value_to_expr(v, dtype)?),

                    IsGreaterThan(v) => expr.gt(value_to_expr(v, dtype)?),

                    IsGreaterThanEqual(v) => expr.gt_eq(value_to_expr(v, dtype)?),

                    IsNull => expr.is_null(),

                    IsNotNull => expr.is_not_null(),

                    And(v) => expr.and(value_to_expr(v, dtype)?),

                    Or(v) => expr.or(value_to_expr(v, dtype)?),

                    Xor(v) => expr.xor(value_to_expr(v, dtype)?),

                    Mean => expr.mean(),

                    Median => expr.median(),

                    Min => expr.min(),

                    Max => expr.max(),
                };

                frame = frame.with_column(expr.alias(&**name));

                frame
            }

            Node::Filter {
                id: _,
                position: _,
                input: _,
                column,
                predicate,
                outputs: _
            } => {
                let expr = col(column);
                let dtype = schema.0
                    .get(column)
                    .unwrap();

                use FilterPredicate::*;
                let predicate_expr = match predicate {
                    IsEqualTo(v) => expr.eq(value_to_expr(v, dtype)?),

                    IsNotEqualTo(v) => expr.neq(value_to_expr(v, dtype)?),

                    IsLessThan(v) => expr.lt(value_to_expr(v, dtype)?),

                    IsLessThanEqual(v) => expr.lt_eq(value_to_expr(v, dtype)?),

                    IsGreaterThan(v) => expr.gt(value_to_expr(v, dtype)?),

                    IsGreaterThanEqual(v) => expr.gt_eq(value_to_expr(v, dtype)?),

                    IsNull => expr.is_null(),

                    IsNotNull => expr.is_not_null(),

                    And(v) => expr.and(value_to_expr(v, dtype)?),

                    Or(v) => expr.or(value_to_expr(v, dtype)?),

                    Xor(v) => expr.xor(value_to_expr(v, dtype)?),
                };

                inputs
                    .into_iter()
                    .next()
                    .unwrap()
                    .frame
                    .filter(predicate_expr)
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
                filename,
                outputs: _
            } => {
                // TODO: Use context that specify project dir and force path
                // to be directly under the project dir.
                LazyCsvReader::new(filename).finish()?
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
                    let mut expr = col(&*column);
                    if !alias.is_empty() {
                        expr = expr.alias(&*alias);
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

fn parse_constant_expr(
    constant: &str,
    dtype: &DataType
) -> Result<Expr, PoldaError> {
    if constant.is_empty() {
        return Ok(Null{}.lit());
    }

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

fn value_to_expr(value: &Value, dtype: &DataType) -> Result<Expr, PoldaError> {
    let expr = match value {
        Value::Column(column) => col(column),
        Value::Constant(constant) => {
            parse_constant_expr(constant.as_str(), dtype)?
        }
    };
    Ok(expr)
}

enum WhenThenMaybeThen {
    None,
    WhenThen(WhenThen),
    WhenThenThen(WhenThenThen)
}
