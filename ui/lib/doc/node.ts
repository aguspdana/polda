import { ValueOf } from "next/dist/shared/lib/constants";
import { nodeModuleNameResolver } from "typescript";
import { EOperation } from "./operation";

export type ENode = 
  IAggregateNode |
  ICaseNode |
  ICastNode |
  IComputeNode |
  IFilterNode |
  IJoinNode |
  ILoadCsvNode |
  ISelectNode |
  ISortNode |
  IUnionNode;

export type ENodeType = ENode["type"];

// Common types

export interface IPosition {
  x: number,
  y: number
}

export interface IColumnValue {
  type: "column",
  value: string
}

export interface IConstantValue {
  type: "constant",
  value: string
}

export type EValue = IColumnValue | IConstantValue;

// AggregateNode

export interface IAggregateNode {
  type: "aggregate",
  id: string,
  position: IPosition,
  input: string | null,
  aggregates: IAggregate[],
  outputs: string[]
}

export interface IAggregate {
  column: string,
  computation: EAggregateComputation,
  alias: string
}

export type EAggregateComputation =
  "count" |
  "first" |
  "group" |
  "last" |
  "max" |
  "mean" |
  "median" |
  "min" |
  "sum";

// BinsNode

export interface IBinsNode {
  type: "bins",
  id: string,
  position: IPosition,
  input: string | null,
  from: string,
  width: string,
  to: string,
  outputs: string[]
}

export interface IBranchNode {
  type: "branch",
  id: string,
  position: IPosition,
  input: string,
  branches: IBranch[],
  default: string,
  outputs: string[]
}

export interface IBranch {
  condition: string,
  then: EValue
}

// CaseNode

export interface ICaseNode {
  type: "case",
  id: string,
  position: IPosition,
  input: string | null,
  name: string,
  data_type: ECaseDataType,
  cases: ICase[],
  default: EValue,
  outputs: string[]
}

export interface ICase {
  column: string,
  value: EValue
}

export enum ECaseDataType {
  Boolean = "Boolean",
  Int8 = "Int8",
  Int16 = "Int16",
  Int32 = "Int32",
  Int64 = "Int64",
  Float32 = "Float32",
  Float64 = "Float64",
  UInt8 = "UInt8",
  UInt16 = "UInt16",
  UInt32 = "UInt32",
  UInt64 = "UInt64",
  Utf8 = "Utf8"
}

// CastNode

export interface ICastNode {
  type: "cast",
  id: string,
  position: IPosition,
  input: string | null,
  column: string,
  data_type: ECastDataType,
  name: string,
  outputs: string[]
}

export enum ECastDataType {
  Boolean = "Boolean",
  Int8 = "Int8",
  Int16 = "Int16",
  Int32 = "Int32",
  Int64 = "Int64",
  Float32 = "Float32",
  Float64 = "Float64",
  UInt8 = "UInt8",
  UInt16 = "UInt16",
  UInt32 = "UInt32",
  UInt64 = "UInt64",
  Utf8 = "Utf8"
}

// ComputeNode

export interface IComputeNode {
  type: "compute",
  id: string,
  position: IPosition,
  input: string | null,
  column: string,
  operation: EComputeOperation,
  name: string,
  outputs: string[]
}

export type EComputeOperation =
  IComputeAdd |
  IComputeSubtract |
  IComputeMultiply |
  IComputeDivide |
  IComputeIsEqualTo |
  IComputeIsNotEqualTo |
  IComputeIsLessThan |
  IComputeIsLessThanEqual |
  IComputeIsGreaterThan |
  IComputeIsGreaterThanEqual |
  IComputeIsNull |
  IComputeIsNotNull |
  IComputeAnd |
  IComputeOr |
  IComputeXor |
  IComputeMean |
  IComputeMedian |
  IComputeMin |
  IComputeMax;

export interface IComputeAdd {
  type: "add",
  param: EValue
}

export interface IComputeSubtract {
  type: "subtract",
  param: EValue
}

export interface IComputeMultiply {
  type: "multiply",
  param: EValue
}

interface IComputeDivide {
  type: "divide",
  param: EValue
}

interface IComputeIsEqualTo {
  type: "is_equal_to",
  param: EValue
}

interface IComputeIsNotEqualTo {
  type: "is_not_equal_to",
  param: EValue
}

interface IComputeIsLessThan {
  type: "is_less_than",
  param: EValue
}

interface IComputeIsLessThanEqual {
  type: "is_less_than_equal",
  param: EValue
}

interface IComputeIsGreaterThan {
  type: "is_greater_than",
  param: EValue
}

interface IComputeIsGreaterThanEqual {
  type: "is_greater_than_equal",
  param: EValue
}

interface IComputeIsNull {
  type: "is_null"
}

interface IComputeIsNotNull {
  type: "is_not_null"
}

interface IComputeAnd {
  type: "and",
  param: EValue
}

interface IComputeOr {
  type: "or",
  param: EValue
}

interface IComputeXor {
  type: "xor",
  param: EValue
}

interface IComputeMean {
  type: "mean"
}

interface IComputeMedian {
  type: "median"
}

interface IComputeMin {
  type: "min"
}

interface IComputeMax {
  type: "max"
}

// FilterNode

export interface IFilterNode {
  type: "filter",
  id: string,
  position: IPosition,
  input: string | null,
  column: string,
  predicate: EFilterPredicate,
  outputs: string[]
}

export type EFilterPredicate =
  IIsEqualToFilter |
  IIsNotEqualToFilter |
  IIsLessThanFilter |
  IIsLessThanEqualFilter |
  IIsGreaterThanFilter |
  IIsGreaterThanEqualFilter |
  IIsNullFilter |
  IIsNotNullFilter |
  IAndFilter |
  IOrFilter |
  IXorFilter;

export interface IIsEqualToFilter {
  type: "is_equal_to",
  param: EValue
}

export interface IIsNotEqualToFilter {
  type: "is_not_equal_to",
  param: EValue
}

export interface IIsLessThanFilter {
  type: "is_less_than",
  param: EValue
}

export interface IIsLessThanEqualFilter {
  type: "is_less_than_equal",
  param: EValue
}

export interface IIsGreaterThanFilter {
  type: "is_greater_than",
  param: EValue
}

export interface IIsGreaterThanEqualFilter {
  type: "is_greater_than_equal",
  param: EValue
}

export interface IIsNullFilter {
  type: "is_null"
}

export interface IIsNotNullFilter {
  type: "is_not_null"
}

export interface IAndFilter {
  type: "and",
  param: EValue
}

export interface IOrFilter {
  type: "or",
  param: EValue
}

export interface IXorFilter {
  type: "xor",
  param: EValue
}

// JoinNode

export interface IJoinNode {
  type: "join",
  id: string,
  position: IPosition,
  left_input: string | null,
  right_input: string | null,
  join_type: EJoinType,
  columns: IJoinColumn[],
  outputs: string[]
}

export type EJoinType =
  "left" |
  "right" |
  "inner" |
  "full" |
  "cross";

export interface IJoinColumn {
  left: string,
  right: string
}

// LoadCsvNode

export interface ILoadCsvNode {
  type: "load_csv",
  id: string,
  position: IPosition,
  filename: string,
  outputs: string[]
}

// SelectNode

export interface ISelectNode {
  type: "select",
  id: string,
  position: IPosition,
  input: string | null,
  columns: ISelectColumn[],
  outputs: string[]
}

export interface ISelectColumn {
  column: string,
  alias: string
}

// SortNode

export interface ISortNode {
  type: "sort",
  id: string,
  position: IPosition,
  input: string | null,
  sorters: ISorter[],
  outputs: string[]
}

export interface ISorter {
  column: string,
  direction: ESortDirection
}

export type ESortDirection = "asc" | "desc"

// UnionNode

export interface IUnionNode {
  type: "union",
  id: string,
  position: IPosition,
  primary_input: string | null,
  secondary_input: string | null,
  outputs: string[]
}

export function getInputs(node: ENode): (string | null)[] {
  switch (node.type) {
    case "aggregate":
      return [node.input];
    case "case":
      return [node.input];
    case "cast":
      return [node.input];
    case "compute":
      return [node.input];
    case "filter":
      return [node.input]
    case "join":
      return [node.left_input, node.right_input];
    case "select":
      return [node.input];
    case "sort":
      return [node.input];
    case "union":
      return [node.primary_input, node.secondary_input];
    default:
      return [];
  }
}

/**
 * Generate operations to disconnect the node's inputs and outputs.
 */
export function disconnectNodeOps(nodes: { [id: string]: ENode}, id: string): EOperation[] {
  const node = nodes[id];
  if (node === undefined) {
    throw new Error("Node doesn't exist");
  }

  const ops: EOperation[] = [];

  const inputNames = ["primary", "secondary"] as const;

  const inputs = getInputs(node);
  inputNames.forEach((name, i) => {
    if (typeof inputs[i] === "string") {
      ops.push({
        type: "set_input",
        id,
        name,
        input: null
      });
    }
  });

  for (const output of node.outputs) {
    const node = nodes[output];
    if (node === undefined) {
      throw new Error("Node doesn't exist");
    }
    const inputs = getInputs(node);
    inputNames.forEach((name, i) => {
      if (inputs[i] === id) {
        ops.push({
          type: "set_input",
          id: output,
          name,
          input: null
        });
      }
    });
  }

  return ops;
}

export function defaultNode(type: ENodeType): ENode {
  switch (type) {
    case "aggregate":
      return {
        type: "aggregate",
        id: "",
        position: {
          x: 0,
          y: 0
        },
        input: null,
        aggregates: [
          {
            column: "",
            computation: "group",
            alias: ""
          }
        ],
        outputs: []
      };
    case "case":
      return {
        type: "case",
        id: "",
        position: {
          x: 0,
          y: 0
        },
        input: null,
        name: "",
        data_type: ECaseDataType.Utf8,
        cases: [{
          column: "",
          value: {
            type: "constant",
            value: ""
          }
        }],
        default: {
          type: "constant",
          value: ""
        },
        outputs: []
      };
    case "cast":
      return {
        type: "cast",
        id: "",
        position: {
          x: 0,
          y: 0
        },
        input: null,
        name: "",
        column: "",
        data_type: ECastDataType.Boolean,
        outputs: []
      };
    case "compute":
      return {
        type: "compute",
        id: "",
        position: {
          x: 0,
          y: 0
        },
        input: null,
        name: "",
        column: "",
        operation: {
          type: "add",
          param: {
            type: "constant",
            value: ""
          }
        },
        outputs: []
      };
    case "filter":
      return {
        type: "filter",
        id: "",
        position: {
          x: 0,
          y: 0
        },
        input: null,
        column: "",
        predicate: {
          type: "is_equal_to",
          param: {
            type: "constant",
            value: ""
          }
        },
        outputs: []
      };
    case "join":
      return {
        type: "join",
        id: "",
        position: {
          x: 0,
          y: 0
        },
        left_input: null,
        right_input: null,
        join_type: "left",
        columns: [
          {
            left: "",
            right: ""
          }
        ],
        outputs: []
      };
    case "load_csv":
      return {
        type: "load_csv",
        id: "",
        position: {
          x: 0,
          y: 0
        },
        filename: "",
        outputs: []
      };
    case "select":
      return {
        type: "select",
        id: "",
        position: {
          x: 0,
          y: 0
        },
        input: null,
        columns: [
          {
            column: "",
            alias: ""
          }
        ],
        outputs: []
      };
    case "sort":
      return {
        type: "sort",
        id: "",
        position: {
          x: 0,
          y: 0
        },
        input: null,
        sorters: [
          {
            column: "",
            direction: "desc"
          }
        ],
        outputs: []
      };
    case "union":
      return {
        type: "union",
        id: "",
        position: {
          x: 0,
          y: 0
        },
        primary_input: null,
        secondary_input: null,
        outputs: []
      };
  }
}
