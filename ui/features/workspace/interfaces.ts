export interface Position {
  x: number,
  y: number
}

export interface Dimension {
  height: number,
  width: number
}

export interface CommonNodeProps {
  position: Position,
  type: string,
  input: (string | null)[],
  output: string[]
}

export interface LoadCsvNodeProps extends CommonNodeProps {
  type: 'load_csv',
  location: string | null
}

export interface SelectNodeProps extends CommonNodeProps {
  type: 'select',
  columns: {
    name: string,
    as: string
  }[]
}

export interface ColumnComparator {
  type: 'column',
  name: string
}

export interface ConstantComparator {
  type: 'value',
  value: string | number | string[] | number[]
}

export type FilterOperator = 'greater-than' | 'less-than' | 'is-in' | 'equal' | 'not-equal' | 'is-null' | 'is-not-null';

export interface FilterNodeProps extends CommonNodeProps {
  type: 'filter',
  filters: {
    column: string,
    operator: FilterOperator,
    comparator: ColumnComparator | ConstantComparator | null
  }[]
}

export interface JoinColumnPair {
  left: string,
  right: string
}

export type JoinType = 'left' | 'inner' | 'full';

export interface JoinNodeProps extends CommonNodeProps {
  type: 'join',
  joinType: JoinType,
  columnPairs: JoinColumnPair[]
}

export type AggregateOperationType = 'group' | 'sum' | 'min' | 'max';

export interface AggregateOperation {
  column: string,
  operation: AggregateOperationType,
  as: string | null
}

export interface AggregateNodeProps extends CommonNodeProps {
  type: 'aggregate',
  operations: AggregateOperation[]
}

export interface ColumnSort {
  name: string,
  ascending: boolean
}

export interface SortNodeProps extends CommonNodeProps {
  type: 'sort',
  by: ColumnSort[]
}

export type Node =
  LoadCsvNodeProps |
  SelectNodeProps |
  FilterNodeProps |
  AggregateNodeProps |
  JoinNodeProps |
  SortNodeProps;

export interface Socket {
  nodeId: string,
  type: 'input' | 'output',
  index: number // socket index
};

export interface MovableProps {
  isMovable: boolean,
  isMoved: boolean
}

export interface MovableCanvas extends MovableProps {
  type: 'canvas',
}

export interface MovableNodes extends MovableProps {
  type: 'node',
  nodes: string[]
}

export interface MovableVerticalResizer extends MovableProps {
  type: 'vertical_resizer',
  from: Position,
  initial: number
}

export interface MovableHorizontalResizer extends MovableProps {
  type: 'horizontal_resizer'
  from: Position,
  initial: number
}

export type MovableObject =
  MovableCanvas |
  MovableNodes |
  MovableVerticalResizer |
  MovableHorizontalResizer;

export interface WorkspaceState {
  doc: {
    nodes: { [id: string]: Node },
    index: string[]
  },
  canvas: {
    position: Position,
    zoom: number
  },
  pointer: Position,
  window: Dimension,
  newNode: 'load_csv' | 'select' | 'filter' | 'join' | null,
  selectedNodes: { [id: string]: boolean }, // The value is not significant
  movable: MovableObject | null,
  openConnection: Socket | null,
  sidebarWidth: number, // px
  outputHeight: number, // px
}