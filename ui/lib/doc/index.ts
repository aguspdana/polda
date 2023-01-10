import { produce } from 'immer';
import { IAggregateNode, IFilterNode, getInputs, IJoinNode, ILoadCsvNode, ENode, ISelectNode, ISortNode, ICastNode, ICaseNode, IComputeNode } from "./node";
import { EOperation } from "./operation";

export interface Doc {
  nodes: {
    [id: string]: ENode
  },
  index: string[]
}

export function executeOperations(doc: Doc, ops: EOperation[]): { doc: Doc, ops: EOperation[], undo: EOperation[] } {
  let newDoc = doc;
  const undo: EOperation[] = [];
  const _ops: EOperation[] = [];
  for (const op of ops) {
    const res = executeOperation(newDoc, op);
    if (res === null) {
      continue
    }
    newDoc = res.doc;
    _ops.push(op);
    undo.push(res.undo);
  }
  return { doc: newDoc, ops: _ops, undo };;
}

/**
 * Execute operation. Return the new doc and the undo operation.
 * Return `null` if the operation crete a new cycle or the input
 * node doesn't exist.  Otherwise throw an error on a failed operation.
 */
export function executeOperation(doc: Doc, op: EOperation): { doc: Doc, undo: EOperation } | null {
  switch (op.type) {
    case "insert_node": {
      if (doc.nodes[op.node.id] !== undefined) {
        throw new Error("InsertNode failed: Node id already exists");
      }
      const new_doc = produce(doc, (doc) => {
        doc.nodes[op.node.id] = op.node;
      });
      const undo: EOperation = {
        type: "delete_node",
        id: op.node.id
      };
      return { doc: new_doc, undo };
    }

    case "delete_node": {
      const node =  doc.nodes[op.id];
      if (node === undefined) {
        throw new Error("DeleteNode failed: Node doesn't exist");
      }
      const connectedInputsLen = getInputs(node)
        .reduce((len, inp) => typeof inp === "string" ? len + 1 : len, 0);
      if (connectedInputsLen > 0 || node.outputs.length > 0) {
        throw new Error("DeleteNode failed: Can't delete a connected node");
      }
      const new_doc = produce(doc, doc => {
        delete doc.nodes[op.id];
      });
      const undo: EOperation = {
        type: "insert_node",
        node: doc.nodes[op.id]
      };
      return { doc: new_doc, undo };
    }

    case "insert_index": {
      if (op.index < 0 || op.index > doc.index.length) {
        throw new Error("InsertIndex failed: Index is out of range");
      }
      const new_doc = produce(doc, doc => {
        doc.index.splice(op.index, 0, op.id);
      });
      const undo: EOperation = {
        type: "delete_index",
        id: op.id,
        index: op.index
      };
      return { doc: new_doc, undo };
    }

    case "delete_index": {
      if (op.index < 0 || op.index >= doc.index.length) {
        throw new Error("DeleteIndex failed: Index is out of range");
      }
      if (op.id !== doc.index[op.index]) {
        throw new Error("DeleteIndex failed: Id doesn't match");
      }
      let new_doc = produce(doc, doc => {
        doc.index.splice(op.index, 1);
      });
      const undo: EOperation = {
        type: "insert_index",
        id: op.id,
        index: op.index
      };
      return { doc: new_doc, undo };
    }

    case "set_input": {
      if (
        op.input !== null &&
        (
          doc.nodes[op.input] === undefined ||
          is_cyclic(doc, op.id, op.input)
        )
      ) {
        return null;
      }

      const node = doc.nodes[op.id];
      if (node === undefined) {
        throw new Error("SetInput failed: Node doesn't exist");
      }

      let prevSourceId: string | null = null;
      const new_doc = produce(doc, doc => {
        const node = doc.nodes[op.id];
        let insertToSource = false;
        let removeFromSource = false;
        switch (node.type) {
          case "aggregate":
            if (op.name !== "primary") {
              throw new Error(`SetInput failed: AggregateNode doesn't take ${op.name} input`);
            }
            insertToSource = typeof op.input === "string" && op.input !== node.input;
            removeFromSource = typeof node.input === "string" && op.input !== node.input;
            prevSourceId = node.input;
            node.input = op.input;
            break;
          case "case":
            if (op.name !== "primary") {
              throw new Error(`SetInput failed: CaseNode doesn't take ${op.name} input`);
            }
            insertToSource = typeof op.input === "string" && op.input !== node.input;
            removeFromSource = typeof node.input === "string" && op.input !== node.input;
            prevSourceId = node.input;
            node.input = op.input;
            break;
          case "cast":
            if (op.name !== "primary") {
              throw new Error(`SetInput failed: CastNode doesn't take ${op.name} input`);
            }
            insertToSource = typeof op.input === "string" && op.input !== node.input;
            removeFromSource = typeof node.input === "string" && op.input !== node.input;
            prevSourceId = node.input;
            node.input = op.input;
            break;
          case "compute":
            if (op.name !== "primary") {
              throw new Error(`SetInput failed: ComputeNode doesn't take ${op.name} input`);
            }
            insertToSource = typeof op.input === "string" && op.input !== node.input;
            removeFromSource = typeof node.input === "string" && op.input !== node.input;
            prevSourceId = node.input;
            node.input = op.input;
            break;
          case "filter":
            if (op.name !== "primary") {
              throw new Error(`SetInput failed: FilterNode doesn't take ${op.name} input`);
            }
            insertToSource =
              typeof op.input === "string"
              && op.input !== node.input;
            removeFromSource =
              typeof node.input === "string"
              && op.input !== node.input;
            prevSourceId = node.input;
            node.input = op.input;
            break;
          case "join":
            insertToSource =
              typeof op.input === "string"
              && ![node.left_input, node.right_input].includes(op.input);
            if (op.name === "primary") {
              removeFromSource =
                typeof node.left_input === "string"
                && ![op.input, node.right_input].includes(node.left_input);
              prevSourceId = node.left_input;
              node.left_input = op.input;
            } else if (op.name === "secondary") {
              removeFromSource =
                typeof node.right_input === "string"
                && ![op.input, node.left_input].includes(node.right_input);
              prevSourceId = node.right_input;
              node.right_input = op.input;
            }
            break;
          case "load_csv":
            throw new Error("SetInput failed: LoadCsvNode doesn't take any input");
          case "select":
            if (op.name !== "primary") {
              throw new Error(`SetInput failed: SelectNode doesn't take ${op.name} input`);
            }
            insertToSource = typeof op.input === "string" && op.input !== node.input;
            removeFromSource = typeof node.input === "string" && op.input !== node.input;
            prevSourceId = node.input;
            node.input = op.input;
            break;
          case "sort":
            if (op.name !== "primary") {
              throw new Error(`SetInput failed: SortNode doesn't take ${op.name} input`);
            }
            insertToSource = typeof op.input === "string" && op.input !== node.input;
            removeFromSource = typeof node.input === "string" && op.input !== node.input;
            prevSourceId = node.input;
            node.input = op.input;
            break;
          case "union":
            insertToSource =
              typeof op.input === "string"
              && ![node.primary_input, node.secondary_input].includes(op.input);
            if (op.name === "primary") {
              removeFromSource =
                typeof node.primary_input === "string"
                && ![op.input, node.secondary_input].includes(node.primary_input);
              prevSourceId = node.primary_input;
              node.primary_input = op.input;
            } else {
              removeFromSource =
                typeof node.secondary_input === "string"
                && ![op.input, node.primary_input].includes(node.secondary_input);
              prevSourceId = node.secondary_input;
              node.secondary_input = op.input;
            }
            break;
        }

        // Update source output.
        if (insertToSource && typeof op.input === "string") {
          const source = doc.nodes[op.input];
          source.outputs.push(op.id);
        }
        if (removeFromSource && typeof prevSourceId === "string") {
          const source = doc.nodes[prevSourceId];
          source.outputs = source.outputs.filter(id => id !== op.id);
        }
      });

      const undo: EOperation = {
        ...op,
        input: prevSourceId
      };

      return { doc: new_doc, undo };
    }

    case "set_position": {
      const node = doc.nodes[op.id];
      if (node === undefined) {
        throw new Error("SetPosition failed: Node doesn't exist");
      }
      const new_doc = produce(doc, doc => {
        doc.nodes[op.id].position = op.position;
      });
      const undo: EOperation = {
        ...op,
        position: node.position
      };
      return { doc: new_doc, undo };
    }

    case "insert_aggregate": {
      const new_doc = produce(doc, doc => {
        const node = doc.nodes[op.id];
        if (node === undefined) {
          throw new Error("InsertAggregate failed: Node doesn't exist");
        }
        if (node.type !== "aggregate") {
          throw new Error("InsertAggregate failed: The node is not AggregateNode");
        }
        if (op.index < 0 || op.index > node.aggregates.length) {
          throw new Error("InsertAggregate failed: Index is out of range");
        }
        node.aggregates.splice(op.index, 0, op.aggregate);
      });
      const undo: EOperation = {
        type: "delete_aggregate",
        id: op.id,
        index: op.index
      };
      return { doc: new_doc, undo };
    }

    case "delete_aggregate": {
      const new_doc = produce(doc, doc => {
        const node = doc.nodes[op.id];
        if (node === undefined) {
          throw new Error("DeleteAggregate failed: Node doesn't exist");
        }
        if (node.type !== "aggregate") {
          throw new Error("DeleteAggregate failed: The node is not AgggregateNode");
        }
        if (op.index < 0 || op.index >= node.aggregates.length) {
          throw new Error("DeleteAggregate failed: Index is out of range");
        }
        node.aggregates.splice(op.index, 1);
      });
      const node = doc.nodes[op.id] as IAggregateNode;
      const undo: EOperation = {
        type: "insert_aggregate",
        id: op.id,
        index: op.index,
        aggregate: node.aggregates[op.index]
      };
      return { doc: new_doc, undo };
    }

    case "set_aggregate_computation": {
      const new_doc = produce(doc, doc => {
        const node = doc.nodes[op.id];
        if (node === undefined) {
          throw new Error("SetAggregaComputation failed: Node doesn't exist");
        }
        if (node.type !== "aggregate") {
          throw new Error("SetAggregateComputation failed: The node is not AgggregateNode");
        }
        if (op.index < 0 || op.index >= node.aggregates.length) {
          throw new Error("SetAggregateComputation failed: Index is out of range");
        }
        node.aggregates[op.index].computation = op.computation;
      });
      const node = doc.nodes[op.id] as IAggregateNode;
      const undo: EOperation = {
        ...op,
        computation: node.aggregates[op.index].computation
      };
      return { doc: new_doc, undo };
    }

    case "set_aggregate_column": {
      const new_doc = produce(doc, doc => {
        const node = doc.nodes[op.id];
        if (node === undefined) {
          throw new Error("SetAggregateColumn failed: Node doesn't exist");
        }
        if (node.type !== "aggregate") {
          throw new Error("SetAggregateColumn failed: The node is not AgggregateNode");
        }
        if (op.index < 0 || op.index >= node.aggregates.length) {
          throw new Error("SetAggregateColumn failed: Index is out of range");
        }
        node.aggregates[op.index].column = op.column;
      });
      const node = doc.nodes[op.id] as IAggregateNode;
      const undo: EOperation = {
        ...op,
        column: node.aggregates[op.index].column
      };
      return { doc: new_doc, undo };
    }

    case "set_aggregate_alias": {
      const new_doc = produce(doc, doc => {
        const node = doc.nodes[op.id];
        if (node === undefined) {
          throw new Error("SetAggregateAlias failed: Node doesn't exist");
        }
        if (node.type !== "aggregate") {
          throw new Error("SetAggregateAlias failed: The node is not AgggregateNode");
        }
        if (op.index < 0 || op.index >= node.aggregates.length) {
          throw new Error("SetAggregateAlias failed: Index is out of range");
        }
        node.aggregates[op.index].alias = op.alias;
      });
      const node = doc.nodes[op.id] as IAggregateNode;
      const undo: EOperation = {
        ...op,
        alias: node.aggregates[op.index].alias
      };
      return { doc: new_doc, undo };
    }

    case "set_case_name": {
      const new_doc = produce(doc, doc => {
        const node = doc.nodes[op.id];
        if (node === undefined) {
          throw new Error("SetCaseName failed: Node doesn't exist");
        }
        if (node.type !== "case") {
          throw new Error("SetCaseName failed: The node is not CaseNode");
        }
        node.name = op.name;
      });
      const node = doc.nodes[op.id] as ICaseNode;
      const undo: EOperation = {
        ...op,
        name: node.name
      };
      return { doc: new_doc, undo };
    }

    case "set_case_data_type": {
      const new_doc = produce(doc, doc => {
        const node = doc.nodes[op.id];
        if (node === undefined) {
          throw new Error("SetCaseDataType failed: Node doesn't exist");
        }
        if (node.type !== "case") {
          throw new Error("SetCaseDataType failed: The node is not CaseNode");
        }
        node.data_type = op.data_type;
      });
      const node = doc.nodes[op.id] as ICaseNode;
      const undo: EOperation = {
        ...op,
        data_type: node.data_type
      };
      return { doc: new_doc, undo };
    }

    case "insert_case": {
      const new_doc = produce(doc, doc => {
        const node = doc.nodes[op.id];
        if (node === undefined) {
          throw new Error("InsertCase failed: Node doesn't exist");
        }
        if (node.type !== "case") {
          throw new Error("InsertCase failed: The node is not CaseNode");
        }
        if (op.index < 0 || op.index > node.cases.length) {
          throw new Error("InsertAggregate failed: Index is out of range");
        }
        node.cases.splice(op.index, 0, op.case);
      });
      const undo: EOperation = {
        type: "delete_case",
        id: op.id,
        index: op.index
      };
      return { doc: new_doc, undo };
    }

    case "delete_case": {
      const new_doc = produce(doc, doc => {
        const node = doc.nodes[op.id];
        if (node === undefined) {
          throw new Error("DeleteCase failed: Node doesn't exist");
        }
        if (node.type !== "case") {
          throw new Error("DeleteCase failed: The node is not CaseNode");
        }
        if (op.index < 0 || op.index >= node.cases.length) {
          throw new Error("DeleteCase failed: Index is out of range");
        }
        node.cases.splice(op.index, 1);
      });
      const node = doc.nodes[op.id] as ICaseNode;
      const undo: EOperation = {
        type: "insert_case",
        id: op.id,
        index: op.index,
        case: node.cases[op.index]
      };
      return { doc: new_doc, undo };
    }

    case "set_case_column": {
      const new_doc = produce(doc, doc => {
        const node = doc.nodes[op.id];
        if (node === undefined) {
          throw new Error("SetCaseColumn failed: Node doesn't exist");
        }
        if (node.type !== "case") {
          throw new Error("SetCaseColumn failed: The node is not CaseNode");
        }
        node.cases[op.index].column = op.column;
      });
      const node = doc.nodes[op.id] as ICaseNode;
      const undo: EOperation = {
        ...op,
        column: node.cases[op.index].column
      };
      return { doc: new_doc, undo };
    }

    case "set_case_value": {
      const new_doc = produce(doc, doc => {
        const node = doc.nodes[op.id];
        if (node === undefined) {
          throw new Error("SetCaseValue failed: Node doesn't exist");
        }
        if (node.type !== "case") {
          throw new Error("SetCaseValue failed: The node is not CaseNode");
        }
        node.cases[op.index].value = op.value;
      });
      const node = doc.nodes[op.id] as ICaseNode;
      const undo: EOperation = {
        ...op,
        value: node.cases[op.index].value
      };
      return { doc: new_doc, undo };
    }

    case "set_case_default": {
      const new_doc = produce(doc, doc => {
        const node = doc.nodes[op.id];
        if (node === undefined) {
          throw new Error("SetCaseDefault failed: Node doesn't exist");
        }
        if (node.type !== "case") {
          throw new Error("SetCaseDefault failed: The node is not CaseNode");
        }
        node.default = op.default;
      });
      const node = doc.nodes[op.id] as ICaseNode;
      const undo: EOperation = {
        ...op,
        default: node.default
      };
      return { doc: new_doc, undo };
    }

    case "set_cast_name": {
      const new_doc = produce(doc, doc => {
        const node = doc.nodes[op.id];
        if (node === undefined) {
          throw new Error("SetCastName failed: Node doesn't exist");
        }
        if (node.type !== "cast") {
          throw new Error("SetCastName failed: The node is not CastNode");
        }
        node.name = op.name;
      });
      const node = doc.nodes[op.id] as ICastNode;
      const undo: EOperation = {
        ...op,
        name: node.name
      };
      return { doc: new_doc, undo };
    }

    case "set_cast_column": {
      const new_doc = produce(doc, doc => {
        const node = doc.nodes[op.id];
        if (node === undefined) {
          throw new Error("SetCastColumn failed: Node doesn't exist");
        }
        if (node.type !== "cast") {
          throw new Error("SetCastColumn failed: The node is not CastNode");
        }
        node.column = op.column;
      });
      const node = doc.nodes[op.id] as ICastNode;
      const undo: EOperation = {
        ...op,
        column: node.column
      };
      return { doc: new_doc, undo };
    }

    case "set_cast_data_type": {
      const new_doc = produce(doc, doc => {
        const node = doc.nodes[op.id];
        if (node === undefined) {
          throw new Error("SetCastDataType failed: Node doesn't exist");
        }
        if (node.type !== "cast") {
          throw new Error("SetCastDataType failed: The node is not CastNode");
        }
        node.data_type = op.data_type;
      });
      const node = doc.nodes[op.id] as ICastNode;
      const undo: EOperation = {
        ...op,
        data_type: node.data_type
      };
      return { doc: new_doc, undo };
    }

    case "set_compute_name": {
      const new_doc = produce(doc, doc => {
        const node = doc.nodes[op.id];
        if (node === undefined) {
          throw new Error("SetComputeName failed: Node doesn't exist");
        }
        if (node.type !== "compute") {
          throw new Error("SetComputeName failed: The node is not ComputeNode");
        }
        node.name = op.name;
      });
      const node = doc.nodes[op.id] as IComputeNode;
      const undo: EOperation = {
        ...op,
        name: node.name
      };
      return { doc: new_doc, undo };
    }

    case "set_compute_column": {
      const new_doc = produce(doc, doc => {
        const node = doc.nodes[op.id];
        if (node === undefined) {
          throw new Error("SetComputeColumn failed: Node doesn't exist");
        }
        if (node.type !== "compute") {
          throw new Error("SetComputeColumn failed: The node is not ComputeNode");
        }
        node.column = op.column;
      });
      const node = doc.nodes[op.id] as IComputeNode;
      const undo: EOperation = {
        ...op,
        column: node.column
      };
      return { doc: new_doc, undo };
    }

    case "set_compute_operation": {
      const new_doc = produce(doc, doc => {
        const node = doc.nodes[op.id];
        if (node === undefined) {
          throw new Error("SetComputeOperation failed: Node doesn't exist");
        }
        if (node.type !== "compute") {
          throw new Error("SetComputeOperation failed: The node is not ComputeNode");
        }
        node.operation = op.operation;
      });
      const node = doc.nodes[op.id] as IComputeNode;
      const undo: EOperation = {
        ...op,
        operation: node.operation
      };
      return { doc: new_doc, undo };
    }

    case "set_filter_column": {
      const new_doc = produce(doc, doc => {
        const node = doc.nodes[op.id];
        if (node === undefined) {
          throw new Error("SetFilterColumn failed: Node doesn't exist");
        }
        if (node.type !== "filter") {
          throw new Error("SetFilterColumn failed: The node is not FilterNode");
        }
        node.column = op.column;
      });
      const node = doc.nodes[op.id] as IFilterNode;
      const undo: EOperation = {
        ...op,
        column: node.column
      };
      return { doc: new_doc, undo };
    }

    case "set_filter_predicate": {
      const new_doc = produce(doc, doc => {
        const node = doc.nodes[op.id];
        if (node === undefined) {
          throw new Error("SetFilterPredicate failed: Node doesn't exist");
        }
        if (node.type !== "filter") {
          throw new Error("SetFilterPredicate failed: The node is not FilterNode");
        }
        node.predicate = op.predicate;
      });
      const node = doc.nodes[op.id] as IFilterNode;
      const undo: EOperation = {
        ...op,
        predicate: node.predicate
      };
      return { doc: new_doc, undo };
    }

    case "set_load_csv_filename": {
      const new_doc = produce(doc, doc => {
        const node = doc.nodes[op.id];
        if (node === undefined) {
          throw new Error("SetCsvPath failed: Node doesn't exist");
        }
        if (node.type !== "load_csv") {
          throw new Error("SetCsvPath failed: The node is not LoadCsvNode");
        }
        node.filename = op.filename;
      });
      const node = doc.nodes[op.id] as ILoadCsvNode;
      const undo: EOperation = {
        ...op,
        filename: node.filename
      };
      return { doc: new_doc, undo };
    }

    case "set_join_type": {
      const new_doc = produce(doc, doc => {
        const node = doc.nodes[op.id];
        if (node === undefined) {
          throw new Error("SetJoinType failed: Node doesn't exist");
        }
        if (node.type !== "join") {
          throw new Error("SetJoinType failed: The node is not JoinNode");
        }
        node.join_type = op.join_type;
      });
      const node = doc.nodes[op.id] as IJoinNode;
      const undo: EOperation = {
        ...op,
        join_type: node.join_type
      };
      return { doc: new_doc, undo };
    }

    case "insert_join_column": {
      const new_doc = produce(doc, doc => {
        const node = doc.nodes[op.id];
        if (node === undefined) {
          throw new Error("InsertJoin failed: Node doesn't exist");
        }
        if (node.type !== "join") {
          throw new Error("InsertJoin failed: The node is not JoinNode");
        }
        if (op.index < 0 || op.index > node.columns.length) {
          throw new Error("InsertJoin failed: Index is out of range");
        }
        node.columns.splice(op.index, 0, op.join_column);
      });
      const undo: EOperation = {
        type: "delete_join_column",
        id: op.id,
        index: op.index
      };
      return { doc: new_doc, undo };
    }

    case "delete_join_column": {
      const new_doc = produce(doc, doc => {
        const node = doc.nodes[op.id];
        if (node === undefined) {
          throw new Error("DeleteJoin failed: Node doesn't exist");
        }
        if (node.type !== "join") {
          throw new Error("DeleteJoin failed: The node is not JoinNode");
        }
        if (op.index < 0 || op.index >= node.columns.length) {
          throw new Error("DeleteJoin failed: Index is out of range");
        }
        node.columns.splice(op.index, 1);
      });
      const node = doc.nodes[op.id] as IJoinNode;
      const undo: EOperation = {
        type: "insert_join_column",
        id: op.id,
        index: op.index,
        join_column: node.columns[op.index]
      };
      return { doc: new_doc, undo };
    }

    case "set_join_column_left": {
      const new_doc = produce(doc, doc => {
        const node = doc.nodes[op.id];
        if (node === undefined) {
          throw new Error("SetLeftJoinColumn failed: Node doesn't exist");
        }
        if (node.type !== "join") {
          throw new Error("SetLeftJoinColumn failed: The node is not JoinNode");
        }
        if (op.index < 0 || op.index >= node.columns.length) {
          throw new Error("SetLeftJoinColumn failed: Index is out of range");
        }
        node.columns[op.index].left = op.column;
      });
      const node = doc.nodes[op.id] as IJoinNode;
      const undo: EOperation = {
        ...op,
        column: node.columns[op.index].left
      };
      return { doc: new_doc, undo };
    }

    case "set_join_column_right": {
      const new_doc = produce(doc, doc => {
        const node = doc.nodes[op.id];
        if (node === undefined) {
          throw new Error("SetRightJoinColumn failed: Node doesn't exist");
        }
        if (node.type !== "join") {
          throw new Error("SetRightJoinColumn failed: The node is not JoinNode");
        }
        if (op.index < 0 || op.index >= node.columns.length) {
          throw new Error("SetRightJoinColumn failed: Index is out of range");
        }
        node.columns[op.index].right = op.column;
      });
      const node = doc.nodes[op.id] as IJoinNode;
      const undo: EOperation = {
        ...op,
        column: node.columns[op.index].right
      };
      return { doc: new_doc, undo };
    }

    case "insert_select": {
      const new_doc = produce(doc, doc => {
        const node = doc.nodes[op.id];
        if (node === undefined) {
          throw new Error("InsertSelect failed: Node doesn't exist");
        }
        if (node.type !== "select") {
          throw new Error("InsertSelect failed: The node is not SelectNode");
        }
        if (op.index < 0 || op.index > node.columns.length) {
          throw new Error("InsertSelect failed: Index is out of range");
        }
        node.columns.splice(op.index, 0, op.column);
      });
      const undo: EOperation = {
        type: "delete_select",
        id: op.id,
        index: op.index
      };
      return { doc: new_doc, undo };
    }

    case "delete_select": {
      const new_doc = produce(doc, doc => {
        const node = doc.nodes[op.id];
        if (node === undefined) {
          throw new Error("DeleteSelect failed: Node doesn't exist");
        }
        if (node.type !== "select") {
          throw new Error("DeleteSelect failed: The node is not SelectNode");
        }
        if (op.index < 0 || op.index >= node.columns.length) {
          throw new Error("DeleteSelect failed: Index is out of range");
        }
        node.columns.splice(op.index, 1);
      });
      const node = doc.nodes[op.id] as ISelectNode;
      const undo: EOperation = {
        type: "insert_select",
        id: op.id,
        index: op.index,
        column: node.columns[op.index]
      };
      return { doc: new_doc, undo };
    }

    case "set_select_column": {
      const new_doc = produce(doc, doc => {
        const node = doc.nodes[op.id];
        if (node === undefined) {
          throw new Error("SetSelectColumn failed: Node doesn't exist");
        }
        if (node.type !== "select") {
          throw new Error("SetSelectColumn failed: The node is not SelectNode");
        }
        if (op.index < 0 || op.index >= node.columns.length) {
          throw new Error("SetSelectColumn failed: Index is out of range");
        }
        node.columns[op.index].column = op.column;
      });
      const node = doc.nodes[op.id] as IJoinNode;
      const undo: EOperation = {
        ...op,
        column: node.columns[op.index].left
      };
      return { doc: new_doc, undo };
    }

    case "set_select_alias": {
      const new_doc = produce(doc, doc => {
        const node = doc.nodes[op.id];
        if (node === undefined) {
          throw new Error("SetSelectAlias failed: Node doesn't exist");
        }
        if (node.type !== "select") {
          throw new Error("SetSelectAlias failed: The node is not SelectNode");
        }
        if (op.index < 0 || op.index >= node.columns.length) {
          throw new Error("SetSelectAlias failed: Index is out of range");
        }
        node.columns[op.index].alias = op.alias;
      });
      const node = doc.nodes[op.id] as ISelectNode;
      const undo: EOperation = {
        ...op,
        alias: node.columns[op.index].alias
      };
      return { doc: new_doc, undo };
    }

    case "insert_sorter": {
      const new_doc = produce(doc, doc => {
        const node = doc.nodes[op.id];
        if (node === undefined) {
          throw new Error("InsertSorter failed: Node doesn't exist");
        }
        if (node.type !== "sort") {
          throw new Error("InsertSorter failed: The node is not SortNode");
        }
        if (op.index < 0 || op.index > node.sorters.length) {
          throw new Error("InsertSorter failed: Index is out of range");
        }
        node.sorters.splice(op.index, 0, op.sorter);
      });
      const undo: EOperation = {
        type: "delete_sorter",
        id: op.id,
        index: op.index
      };
      return { doc: new_doc, undo };
    }

    case "delete_sorter": {
      const new_doc = produce(doc, doc => {
        const node = doc.nodes[op.id];
        if (node === undefined) {
          throw new Error("DeleteSorter failed: Node doesn't exist");
        }
        if (node.type !== "sort") {
          throw new Error("DeleteSorter failed: The node is not SortNode");
        }
        if (op.index < 0 || op.index >= node.sorters.length) {
          throw new Error("DeleteSorter failed: Index is out of range");
        }
        node.sorters.splice(op.index, 1);
      });
      const node = doc.nodes[op.id] as ISortNode;
      const undo: EOperation = {
        type: "insert_sorter",
        id: op.id,
        index: op.index,
        sorter: node.sorters[op.index]
      };
      return { doc: new_doc, undo };
    }

    case "set_sort_column": {
      const new_doc = produce(doc, doc => {
        const node = doc.nodes[op.id];
        if (node === undefined) {
          throw new Error("SetSortColumn failed: Node doesn't exist");
        }
        if (node.type !== "sort") {
          throw new Error("SetsortColunm failed: The node is not SortNode");
        }
        if (op.index < 0 || op.index >= node.sorters.length) {
          throw new Error("SetSortColumn failed: Index is out of range");
        }
        node.sorters[op.index].column = op.column;
      });
      const node = doc.nodes[op.id] as ISortNode;
      const undo: EOperation = {
        ...op,
        column: node.sorters[op.index].column
      };
      return { doc: new_doc, undo };
    }

    case "set_sort_direction": {
      const new_doc = produce(doc, doc => {
        const node = doc.nodes[op.id];
        if (node === undefined) {
          throw new Error("SetSortDirection failed: Node doesn't exist");
        }
        if (node.type !== "sort") {
          throw new Error("SetsortDirection failed: The node is not SortNode");
        }
        if (op.index < 0 || op.index >= node.sorters.length) {
          throw new Error("SetSortDirection failed: Index is out of range");
        }
        node.sorters[op.index].direction = op.direction;
      });
      const node = doc.nodes[op.id] as ISortNode;
      const undo: EOperation = {
        ...op,
        direction: node.sorters[op.index].direction
      };
      return { doc: new_doc, undo };
    }

    default: return null;
  }
}

/**
 * Check if connection from node "from" to node "to" create a cycle.
 * Throw an error if the doc contains a node that has input
 * that doesn't exist in the doc.
 */
function is_cyclic(doc: Doc, from: string, to: string): boolean {
  let ids: string[] = [to];
  let checked: Set<string> = new Set();

  while (ids.length > 0) {
    const id = ids.pop();
    if (id === undefined || checked.has(id)) {
      continue;
    }
    if (id === from) {
      return true;
    }
    const node = doc.nodes[id];
    if (node === undefined) {
      throw new Error(`Invalid input: Node ${id} doesn't exist`);
    }
    for (const input of getInputs(node)) {
      if (typeof input === "string") {
        ids.push(input);
      }
    }
    checked.add(id);
  }
  return false;
}