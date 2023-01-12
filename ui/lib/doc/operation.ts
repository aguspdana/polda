import {  IAggregate,
  EAggregateComputation,
  EFilterPredicate,
  IJoinColumn,
  EJoinType,
  ENode,
  IPosition,
  ISelectColumn,
  ESortDirection,
  ISorter,
  ECastDataType,
  ECaseDataType,
  ICase,
  EValue,
  EComputeOperation
} from "./node";

export type EOperation =
  IInsertNode |
  IDeleteNode |
  IInsertIndex |
  IDeleteIndex |
  ISetInput |
  ISetPosition |
  IInsertAggregate |
  IDeleteAggregate |
  ISetAggregateComputation |
  ISetAggregateColumn |
  ISetAggregateAlias |
  ISetCaseName |
  ISetCaseDataType |
  IInsertCase |
  IDeleteCase |
  ISetCaseColumn |
  ISetCaseValue |
  ISetCaseDefault |
  ISetCastName |
  ISetCastColumn |
  ISetCastDataType |
  ISetComputeName |
  ISetComputeColumn |
  ISetComputeOperation |
  ISetFilterColumn |
  ISetFilterPredicate |
  ISetLoadCSVPath |
  ISetJoinType |
  IInsertJoinColumn |
  IDeleteJoinColumn |
  ISetJoinColumnLeft |
  ISetJoinColumnRight |
  IInsertSelect |
  IDeleteSelect |
  ISetSelectColumn |
  ISetSelectAlias |
  IInsertSorter |
  IDeleteSorter |
  ISetSortColumn |
  ISetSortDirection;

export interface IInsertNode {
  type: "insert_node",
  node: ENode
}

export interface IDeleteNode {
  type: "delete_node",
  id: string
}

export interface IInsertIndex {
  type: "insert_index",
  id: string,
  index: number
}

export interface IDeleteIndex {
  type: "delete_index",
  id: string,
  index: number
}

export interface ISetInput {
  type: "set_input",
  id: string,
  name: "primary" | "secondary",
  input: string | null
}

export interface ISetPosition {
  type: "set_position",
  id: string,
  position: IPosition
}

export interface IInsertAggregate {
  type: "insert_aggregate",
  id: string,
  index: number,
  aggregate: IAggregate
}

export interface IDeleteAggregate {
  type: "delete_aggregate",
  id: string,
  index: number
}

export interface ISetAggregateComputation {
  type: "set_aggregate_computation",
  id: string,
  index: number,
  computation: EAggregateComputation
}

export interface ISetAggregateColumn {
  type: "set_aggregate_column",
  id: string,
  index: number,
  column: string
}

export interface ISetAggregateAlias {
  type: "set_aggregate_alias",
  id: string,
  index: number,
  alias: string
}

// CaseNode operations

export interface ISetCaseName {
  type: "set_case_name",
  id: string,
  name: string
}

export interface ISetCaseDataType {
  type: "set_case_data_type",
  id: string,
  data_type: ECaseDataType
}

export interface IInsertCase {
  type: "insert_case",
  id: string,
  index: number,
  case: ICase
}

export interface IDeleteCase {
  type: "delete_case",
  id: string,
  index: number
}

export interface ISetCaseColumn {
  type: "set_case_column",
  id: string,
  index: number,
  column: string
}

export interface ISetCaseValue {
  type: "set_case_value",
  id: string,
  index: number,
  value: EValue
}

export interface ISetCaseDefault {
  type: "set_case_default",
  id: string,
  default: EValue
}

// CastNode operations

export interface ISetCastName {
    type: "set_cast_name",
    id: string,
    name: string
}

export interface ISetCastColumn {
    type: "set_cast_column",
    id: string,
    column: string
}

export interface ISetCastDataType {
    type: "set_cast_data_type",
    id: string,
    data_type: ECastDataType
}

// ComputeNode operations

export interface ISetComputeName {
  type: "set_compute_name",
  id: string,
  name: string
}

export interface ISetComputeColumn {
  type: "set_compute_column",
  id: string,
  column: string
}

export interface ISetComputeOperation {
  type: "set_compute_operation",
  id: string,
  operation: EComputeOperation
}

// FilterNode operations

export interface ISetFilterColumn {
  type: "set_filter_column",
  id: string,
  column: string
}

export interface ISetFilterPredicate {
  type: "set_filter_predicate",
  id: string,
  predicate: EFilterPredicate
}

export interface ISetLoadCSVPath {
  type: "set_load_csv_filename",
  id: string,
  filename: string
}

export interface ISetJoinType {
  type: "set_join_type",
  id: string,
  join_type: EJoinType
}

export interface IInsertJoinColumn {
  type: "insert_join_column",
  id: string,
  index: number,
  join_column: IJoinColumn
}

export interface IDeleteJoinColumn {
  type: "delete_join_column",
  id: string,
  index: number
}

export interface ISetJoinColumnLeft {
  type: "set_join_column_left", // TODO: Change this to set_join_left_column
  id: string,
  index: number,
  column: string
}

export interface ISetJoinColumnRight {
  type: "set_join_column_right", // TODO: Change this to set_join_right_column
  id: string,
  index: number,
  column: string
}

export interface IInsertSelect {
  type: "insert_select",
  id: string,
  index: number,
  column: ISelectColumn
}

export interface IDeleteSelect {
  type: "delete_select",
  id: string,
  index: number
}

export interface ISetSelectColumn {
  type: "set_select_column",
  id: string,
  index: number,
  column: string
}

export interface ISetSelectAlias {
  type: "set_select_alias",
  id: string,
  index: number,
  alias: string
}

export interface IInsertSorter {
  type: "insert_sorter",
  id: string,
  index: number,
  sorter: ISorter
}

export interface IDeleteSorter {
  type: "delete_sorter",
  id: string,
  index: number
}

export interface ISetSortColumn {
  type: "set_sort_column",
  id: string,
  index: number,
  column: string
}

export interface ISetSortDirection {
  type: "set_sort_direction",
  id: string,
  index: number,
  direction: ESortDirection
}

function get_node_id(op: EOperation): string {
  if (op.type === "insert_node") {
    return op.node.id;
  }
  return op.id;
}

// TODO: Check the correctness!
export function rebase(
  ops: EOperation[],
  base: EOperation[]
): (EOperation | null)[] {
  let rebased: (EOperation | null)[] = [];

  ops.forEach((op, i) => {
    let new_op: EOperation = op;
    let forwardFrom = 0;

    // Transform backward against previous operations in the batch.
    for (let j = i - 1; j >= 0; j--) {
      let prev = ops[j];
      let transformed = transformBackward(op, prev);
      if (transformed === null) {
        let mapper = rebased[j-1];
        if (mapper === null) {
          rebased.push(null);
          return;
        }
        let mapped = map(op, mapper);
        new_op = mapped;
        forwardFrom = j + 1;
        break;
      }
      new_op = transformed;
    }

    // Transform forward against the base.
    if (forwardFrom === 0) {
      for (let op of base) {
        let transformed = transformForward(new_op, op);
        if (transformed === null) {
          rebased.push(null);
          return;
        }
        new_op = transformed;
      }
    }

    // Transform against the previously rebased operations.
    for (let j = forwardFrom; j < rebased.length - forwardFrom; j++) {
      let prev = rebased[j];
      if (prev === null) {
        rebased.push(null);
        return;
      }
      let transformed = transformForward(new_op, prev);
      if (transformed === null) {
        return;
      }
      new_op = transformed;
    }
  });

  return rebased;
}

/**
 * Map an operation to another operation.  Only operation return `null`
 * when transformed backward can be mapped, otherwise it'll throw an error.
 */
function map(
  op: EOperation,
  mapper: EOperation
): EOperation {
  if (
    mapper.type === "insert_node" &&
    (
      op.type === "delete_node" ||
      op.type === "set_input" ||
      op.type === "set_position" ||
      op.type === "insert_aggregate" ||
      op.type === "delete_aggregate" ||
      op.type === "set_aggregate_computation" ||
      op.type === "set_aggregate_column" ||
      op.type === "set_aggregate_alias" ||
      op.type === "set_case_name" ||
      op.type === "set_case_data_type" ||
      op.type === "insert_case" ||
      op.type === "delete_case" ||
      op.type === "set_case_column" ||
      op.type === "set_case_value" ||
      op.type === "set_case_default" ||
      op.type === "set_cast_name" ||
      op.type === "set_cast_column" ||
      op.type === "set_cast_data_type" ||
      op.type === "set_compute_name" ||
      op.type === "set_compute_column" ||
      op.type === "set_compute_operation" ||
      op.type === "set_filter_column" ||
      op.type === "set_filter_predicate" ||
      op.type === "set_load_csv_filename" ||
      op.type === "set_join_type" ||
      op.type === "insert_join_column" ||
      op.type === "delete_join_column" ||
      op.type === "set_join_column_left" ||
      op.type === "set_join_column_right" ||
      op.type === "insert_select" ||
      op.type === "delete_select" ||
      op.type === "set_select_column" ||
      op.type === "set_select_alias" ||
      op.type === "insert_sorter" ||
      op.type === "delete_sorter" ||
      op.type === "set_sort_column" ||
      op.type === "set_sort_direction"
    )
  ) {
    return op;
  }

  if (
    mapper.type === "insert_index" &&
    (
      op.type === "insert_index" ||
      op.type === "delete_index"
    )
  ) {
    return {
      ...op,
      index: mapper.index
    };
  }

  if (
    mapper.type === "insert_aggregate" &&
    (
      op.type === "insert_aggregate" ||
      op.type === "delete_aggregate" ||
      op.type === "set_aggregate_computation" ||
      op.type === "set_aggregate_column" ||
      op.type === "set_aggregate_alias"
    )
  ) {
    return {
      ...op,
      index: mapper.index
    };
  }

  if (
    mapper.type === "insert_case" &&
    (
      op.type === "insert_case" ||
      op.type === "delete_case" ||
      op.type === "set_case_column" ||
      op.type === "set_case_value"
    )
  ) {
    return {
      ...op,
      index: mapper.index
    };
  }

  if (
    mapper.type === "insert_join_column" &&
    (
      op.type === "insert_join_column" ||
      op.type === "delete_join_column" ||
      op.type === "set_join_column_left" ||
      op.type === "set_join_column_right"
    )
  ) {
    return {
      ...op,
      index: mapper.index
    };
  }

  if (
    mapper.type === "insert_select" &&
    (
      op.type === "insert_select" ||
      op.type === "delete_select" ||
      op.type === "set_select_column" ||
      op.type === "set_select_alias"
    )
  ) {
    return {
      ...op,
      index: mapper.index
    };
  }

  if (
    mapper.type === "insert_sorter" &&
    (
      op.type === "insert_sorter" ||
      op.type === "delete_sorter" ||
      op.type === "set_sort_column" ||
      op.type === "set_sort_direction"
    )
  ) {
    return {
      ...op,
      index: mapper.index
    };
  }

  throw new Error(`Cannot map operation ${op.type} to ${mapper.type}`);
}

function transformBackward(
  op: EOperation,
  preceded_by: EOperation
): EOperation | null {
  if (
    preceded_by.type === "insert_node" &&
    (
      op.type === "delete_node" ||
      op.type === "set_input" ||
      op.type === "set_position" ||
      op.type === "insert_aggregate" ||
      op.type === "delete_aggregate" ||
      op.type === "set_aggregate_computation" ||
      op.type === "set_aggregate_column" ||
      op.type === "set_aggregate_alias" ||
      op.type === "set_case_name" ||
      op.type === "set_case_data_type" ||
      op.type === "insert_case" ||
      op.type === "delete_case" ||
      op.type === "set_case_column" ||
      op.type === "set_case_value" ||
      op.type === "set_case_default" ||
      op.type === "set_cast_name" ||
      op.type === "set_cast_column" ||
      op.type === "set_cast_data_type" ||
      op.type === "set_compute_name" ||
      op.type === "set_compute_column" ||
      op.type === "set_compute_operation" ||
      op.type === "set_filter_column" ||
      op.type === "set_filter_predicate" ||
      op.type === "set_load_csv_filename" ||
      op.type === "set_join_type" ||
      op.type === "insert_join_column" ||
      op.type === "delete_join_column" ||
      op.type === "set_join_column_left" ||
      op.type === "set_join_column_right" ||
      op.type === "insert_select" ||
      op.type === "delete_select" ||
      op.type === "set_select_column" ||
      op.type === "set_select_alias" ||
      op.type === "insert_sorter" ||
      op.type === "delete_sorter" ||
      op.type === "set_sort_column" ||
      op.type === "set_sort_direction"
    ) &&
    op.id === preceded_by.node.id
  ) {
    return null;
  }

  if (
    preceded_by.type === "insert_index" &&
    (
      op.type === "insert_index" ||
      op.type === "delete_index"
    )
  ) {
    if (op.index === preceded_by.index) {
      return null;
    }
    if (op.index > preceded_by.index) {
      return {
        ...op,
        index: op.index - 1
      };
    }
  }

  if (
    preceded_by.type === "delete_index" &&
    (
      op.type === "insert_index" ||
      op.type === "delete_index"
    ) &&
    op.index >= preceded_by.index
  ) {
    return {
      ...op,
      index: op.index + 1
    };
  }

  if (
    preceded_by.type === "insert_aggregate" &&
    (
      op.type === "insert_aggregate" ||
      op.type === "delete_aggregate" ||
      op.type === "set_aggregate_computation" ||
      op.type === "set_aggregate_column" ||
      op.type === "set_aggregate_alias"
    )
  ) {
    if (op.id === preceded_by.id) {
      if (op.index === preceded_by.index) {
        return null;
      }
      if (op.index > preceded_by.index) {
        return {
          ...op,
          index: op.index - 1
        };
      }
    }
  }

  if (
    preceded_by.type === "delete_aggregate" &&
    (
      op.type === "insert_aggregate" ||
      op.type === "delete_aggregate" ||
      op.type === "set_aggregate_computation" ||
      op.type === "set_aggregate_column" ||
      op.type === "set_aggregate_alias"
    ) &&
    op.id === preceded_by.id &&
    op.index >= preceded_by.index
  ) {
    return {
      ...op,
      index: op.index + 1
    };
  }

  if (
    preceded_by.type === "insert_case" &&
    (
      op.type === "insert_case" ||
      op.type === "delete_case" ||
      op.type === "set_case_column" ||
      op.type === "set_case_value"
    )
  ) {
    if (op.id === preceded_by.id) {
      if (op.index === preceded_by.index) {
        return null;
      }
      if (op.index > preceded_by.index) {
        return {
          ...op,
          index: op.index - 1
        };
      }
    }
  }

  if (
    preceded_by.type === "delete_case" &&
    (
      op.type === "insert_case" ||
      op.type === "delete_case" ||
      op.type === "set_case_column" ||
      op.type === "set_case_value"
    ) &&
    op.id === preceded_by.id &&
    op.index >= preceded_by.index
  ) {
    return {
      ...op,
      index: op.index + 1
    };
  }

  if (
    preceded_by.type === "insert_join_column" &&
    (
      op.type === "insert_join_column" ||
      op.type === "delete_join_column" ||
      op.type === "set_join_column_left" ||
      op.type === "set_join_column_right"
    )
  ) {
    if (op.id === preceded_by.id) {
      if (op.index === preceded_by.index) {
        return null;
      }
      if (op.index > preceded_by.index) {
        return {
          ...op,
          index: op.index - 1
        };
      }
    }
  }

  if (
    preceded_by.type === "delete_join_column" &&
    (
      op.type === "insert_join_column" ||
      op.type === "delete_join_column" ||
      op.type === "set_join_column_left" ||
      op.type === "set_join_column_right"
    ) &&
    op.id === preceded_by.id &&
    op.index >= preceded_by.index
  ) {
    return {
      ...op,
      index: op.index + 1
    };
  }

  if (
    preceded_by.type === "insert_select" &&
    (
      op.type === "insert_select" ||
      op.type === "delete_select" ||
      op.type === "set_select_column" ||
      op.type === "set_select_alias"
    )
  ) {
    if (op.id === preceded_by.id) {
      if (op.index === preceded_by.index) {
        return null;
      }
      if (op.index > preceded_by.index) {
        return {
          ...op,
          index: op.index - 1
        };
      }
    }
  }

  if (
    preceded_by.type === "delete_select" &&
    (
      op.type === "insert_select" ||
      op.type === "delete_select" ||
      op.type === "set_select_column" ||
      op.type === "set_select_alias"
    ) &&
    op.id === preceded_by.id &&
    op.index >= preceded_by.index
  ) {
    return {
      ...op,
      index: op.index + 1
    };
  }

  if (
    preceded_by.type === "insert_sorter" &&
    (
      op.type === "insert_sorter" ||
      op.type === "delete_sorter" ||
      op.type === "set_sort_column" ||
      op.type === "set_sort_direction"
    )
  ) {
    if (op.id === preceded_by.id) {
      if (op.index === preceded_by.index) {
        return null;
      }
      if (op.index > preceded_by.index) {
        return {
          ...op,
          index: op.index - 1
        };
      }
    }
  }

  if (
    preceded_by.type === "delete_sorter" &&
    (
      op.type === "insert_sorter" ||
      op.type === "delete_sorter" ||
      op.type === "set_sort_column" ||
      op.type === "set_sort_direction"
    ) &&
    op.id === preceded_by.id &&
    op.index >= preceded_by.index
  ) {
    return {
      ...op,
      index: op.index + 1
    };
  }

  return op;
}

function transformForward(
  op: EOperation,
  preceded_by: EOperation
): EOperation | null {
  if (
    preceded_by.type === "delete_node" &&
    preceded_by.id === get_node_id(op)
  ) {
    return null;
  }

  if (
    preceded_by.type === "insert_index" &&
    (op.type === "insert_index" || op.type === "delete_index") &&
    op.index >= preceded_by.index
  ) {
    return {
      ...op,
      index: op.index + 1
    };
  }

  if (
    preceded_by.type === "delete_index" &&
    (op.type === "insert_index" || op.type === "delete_index") &&
    op.index > preceded_by.index
  ) {
    return {
      ...op,
      index: op.index - 1
    };
  }

  if (
    preceded_by.type === "insert_aggregate" &&
    (
      op.type === "insert_aggregate" ||
      op.type === "delete_aggregate" ||
      op.type === "set_aggregate_computation" ||
      op.type === "set_aggregate_column" ||
      op.type === "set_aggregate_alias"
    ) &&
    op.id === preceded_by.id &&
    op.index >= preceded_by.index
  ) {
    return {
      ...op,
      index: op.index + 1
    }
  }

  if (
    preceded_by.type === "delete_aggregate" &&
    op.type === "insert_aggregate" &&
    op.id === preceded_by.id &&
    op.index > preceded_by.index
  ) {
    return {
      ...op,
      index: op.index - 1
    };
  }

  if (
    preceded_by.type === "delete_aggregate" &&
    (
      op.type === "delete_aggregate" ||
      op.type === "set_aggregate_computation" ||
      op.type === "set_aggregate_column" ||
      op.type === "set_aggregate_alias"
    ) &&
    op.id === preceded_by.id
  ) {
    if (op.index === preceded_by.index) {
      return null;
    }
    if (op.index > preceded_by.index) {
      return {
        ...op,
        index: op.index - 1
      };
    }
  }

  if (
    preceded_by.type === "insert_case" &&
    (
      op.type === "insert_case" ||
      op.type === "delete_case" ||
      op.type === "set_case_column" ||
      op.type === "set_case_value"
    ) &&
    op.id === preceded_by.id &&
    op.index >= preceded_by.index
  ) {
    return {
      ...op,
      index: op.index + 1
    }
  }

  if (
    preceded_by.type === "delete_case" &&
    op.type === "insert_case" &&
    op.id === preceded_by.id &&
    op.index > preceded_by.index
  ) {
    return {
      ...op,
      index: op.index - 1
    };
  }

  if (
    preceded_by.type === "delete_case" &&
    (
      op.type === "delete_case" ||
      op.type === "set_case_column" ||
      op.type === "set_case_value"
    ) &&
    op.id === preceded_by.id
  ) {
    if (op.index === preceded_by.index) {
      return null;
    }
    if (op.index > preceded_by.index) {
      return {
        ...op,
        index: op.index - 1
      };
    }
  }

  if (
    preceded_by.type === "insert_join_column" &&
    (
      op.type === "insert_join_column" ||
      op.type === "delete_join_column" ||
      op.type === "set_join_column_left" ||
      op.type === "set_join_column_right"
    ) &&
    op.id === preceded_by.id &&
    op.index >= preceded_by.index
  ) {
    return {
      ...op,
      index: op.index + 1
    }
  }

  if (
    preceded_by.type === "delete_join_column" &&
    op.type === "insert_join_column" &&
    op.id === preceded_by.id &&
    op.index > preceded_by.index
  ) {
    return {
      ...op,
      index: op.index - 1
    };
  }

  if (
    preceded_by.type === "delete_join_column" &&
    (
      op.type === "delete_join_column" ||
      op.type === "set_join_column_left" ||
      op.type === "set_join_column_right"
    ) &&
    op.id === preceded_by.id
  ) {
    if (op.index === preceded_by.index) {
      return null;
    }
    if (op.index > preceded_by.index) {
      return {
        ...op,
        index: op.index - 1
      };
    }
  }

  if (
    preceded_by.type === "insert_select" &&
    (
      op.type === "insert_select" ||
      op.type === "delete_select" ||
      op.type === "set_select_column" ||
      op.type === "set_select_alias"
    ) &&
    op.id === preceded_by.id &&
    op.index >= preceded_by.index
  ) {
    return {
      ...op,
      index: op.index + 1
    }
  }

  if (
    preceded_by.type === "delete_select" &&
    op.type === "insert_select" &&
    op.id === preceded_by.id &&
    op.index > preceded_by.index
  ) {
    return {
      ...op,
      index: op.index - 1
    };
  }

  if (
    preceded_by.type === "delete_select" &&
    (
      op.type === "delete_select" ||
      op.type === "set_select_column" ||
      op.type === "set_select_alias"
    ) &&
    op.id === preceded_by.id
  ) {
    if (op.index === preceded_by.index) {
      return null;
    }
    if (op.index > preceded_by.index) {
      return {
        ...op,
        index: op.index - 1
      };
    }
  }

  if (
    preceded_by.type === "insert_sorter" &&
    (
      op.type === "insert_sorter" ||
      op.type === "delete_sorter" ||
      op.type === "set_sort_column" ||
      op.type === "set_sort_direction"
    ) &&
    op.id === preceded_by.id &&
    op.index >= preceded_by.index
  ) {
    return {
      ...op,
      index: op.index + 1
    }
  }

  if (
    preceded_by.type === "delete_sorter" &&
    op.type === "insert_sorter" &&
    op.id === preceded_by.id &&
    op.index > preceded_by.index
  ) {
    return {
      ...op,
      index: op.index - 1
    };
  }

  if (
    preceded_by.type === "delete_sorter" &&
    (
      op.type === "delete_sorter" ||
      op.type === "set_sort_column" ||
      op.type === "set_sort_direction"
    ) &&
    op.id === preceded_by.id
  ) {
    if (op.index === preceded_by.index) {
      return null;
    }
    if (op.index > preceded_by.index) {
      return {
        ...op,
        index: op.index - 1
      };
    }
  }

  return op;
}

export function canReplace(op: EOperation, withOp: EOperation): boolean {
  if (op.type !== withOp.type) {
    return false;
  }

  switch (op.type) {
    case "insert_node":
      break;
    case "delete_node":
      break;
    case "insert_index":
      break;
    case "delete_index":
      break;
    case "set_input": {
      const _withOp = withOp as ISetInput;
      return op.id === _withOp.id && op.name === _withOp?.name;
    }
    case "set_position": {
      const _withOp = withOp as ISetPosition;
      return op.id === _withOp.id;
    }
    case "insert_aggregate":
      break;
    case "delete_aggregate":
      break;
    case "set_aggregate_computation": {
      const _withOp = withOp as ISetAggregateComputation;
      return op.id === _withOp.id && op.index === _withOp?.index;
    }
    case "set_aggregate_column": {
      const _withOp = withOp as ISetAggregateColumn;
      return op.id === _withOp.id && op.index === _withOp.index;
    }
    case "set_aggregate_alias": {
      const _withOp = withOp as ISetAggregateAlias;
      return op.id === _withOp.id && op.index === _withOp?.index;
    }
    case "set_cast_name": {
      const _withOp = withOp as ISetCastName;
      return op.id === _withOp.id && op.name === _withOp?.name;
    }
    case "set_cast_column": {
      const _withOp = withOp as ISetCastColumn;
      return op.id === _withOp.id && op.column === _withOp?.column;
    }
    case "set_cast_data_type": {
      const _withOp = withOp as ISetCastDataType;
      return op.id === _withOp.id && op.data_type === _withOp?.data_type;
    }
    case "set_filter_column": {
      const _withOp = withOp as ISetFilterColumn;
      return op.id === _withOp.id;
    }
    case "set_filter_predicate": {
      const _withOp = withOp as ISetFilterPredicate;
      return op.id === _withOp.id;
    }
    case "set_load_csv_filename": {
      const _withOp = withOp as ISetLoadCSVPath;
      return op.id === _withOp.id;
    }
    case "set_join_type": {
      const _withOp = withOp as ISetJoinType;
      return op.id === _withOp.id;
    }
    case "insert_join_column":
      break;
    case "delete_join_column":
      break;
    case "set_join_column_left": {
      const _withOp = withOp as ISetJoinColumnLeft;
      return op.id === _withOp.id && op.index === _withOp?.index;
    }
    case "set_join_column_right": {
      const _withOp = withOp as ISetJoinColumnRight;
      return op.id === _withOp.id && op.index === _withOp?.index;
    }
    case "insert_select":
      break;
    case "delete_select":
      break;
    case "set_select_column": {
      const _withOp = withOp as ISetSelectColumn;
      return op.id === _withOp.id && op.index === _withOp.index;
    }
    case "set_select_alias": {
      const _withOp = withOp as ISetSelectAlias;
      return op.id === _withOp.id && op.index === _withOp.index;
    }
    case "insert_sorter":
      break;
    case "delete_sorter":
      break;
    case "set_sort_column": {
      const _withOp = withOp as ISetSortColumn;
      return op.id === _withOp.id && op.index === _withOp?.index;
    }
    case "set_sort_direction": {
      const _withOp = withOp as ISetSortDirection;
      return op.id === _withOp.id && op.index === _withOp?.index;
    }
  }
  return false;
}

export function mergeBatch(batch: EOperation[], other: EOperation[]): EOperation[] {
  if (batch.length < other.length) {
    return [...batch, ...other];
  }
  const batchSlice = batch.slice(batch.length - other.length);
  let _canReplace = true;
  for (let i = 0; i < other.length; i++) {
    const op1 = batchSlice[i];
    const op2 = other[i];
    if (!canReplace(op1, op2)) {
      _canReplace = false;
      break;
    }
  }
  if (!_canReplace) {
    return [...batch, ...other];
  }
  return [...batch.slice(0, batch.length - other.length), ...other]
}