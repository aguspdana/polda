import { ComputeIcon } from "components/icons/ComputeIcon";
import { EComputeOperation, EValue, getInputs, IComputeNode } from "lib/doc/node";
import { useStore } from "store";
import { Header } from "../common/Header";
import { Input } from "../common/Input";
import { Port } from "../common/Port";
import { Select } from "../common/Select";
import { Value } from "../common/Value";
import styles from "../Node.module.css";

type EComputeOperationType = EComputeOperation["type"];

const COMPUTE_OPERATION_OPTIONS: {
  value: EComputeOperationType,
  display: string
}[] = [
  {
    value: "add",
    display: "Add"
  },
  {
    value: "subtract",
    display: "Subtract"
  },
  {
    value: "multiply",
    display: "Multiply"
  },
  // NOTE: Division by 0 causes the server to panic.
  // Wait until Polars fixes this.
  // {
  //   value: "divide",
  //   display: "Divide"
  // },
  {
    value: "is_equal_to",
    display: "Is equal to"
  },
  {
    value: "is_not_equal_to",
    display: "Is not equal to"
  },
  {
    value: "is_less_than",
    display: "Is less than"
  },
  {
    value: "is_less_than_equal",
    display: "Is less than equal"
  },
  {
    value: "is_greater_than",
    display: "Is greater than"
  },
  {
    value: "is_greater_than_equal",
    display: "Is greater than equal"
  },
  {
    value: "is_null",
    display: "Is null"
  },
  {
    value: "is_not_null",
    display: "Is not null"
  },
  {
    value: "and",
    display: "And"
  },
  {
    value: "or",
    display: "Or"
  },
  {
    value: "xor",
    display: "Xor"
  },
  {
    value: "mean",
    display: "Mean"
  },
  {
    value: "median",
    display: "Median"
  },
  {
    value: "min",
    display: "Min"
  },
  {
    value: "max",
    display: "Max"
  }
]

export function ComputeNode(props: IComputeNode) {
  const openSocket = useStore(state => state.openSocket);
  const inputs = getInputs(props);
  const selected = useStore(state => typeof state.selectedNodes[props.id] === "boolean");
  const queried = useStore(state => state.query?.type === "node" && state.query?.nodeId === props.id);
  const executeOperations = useStore(state => state.executeOperations);

  function handleNameChange(name: string) {
    executeOperations([{
      type: "set_compute_name",
      id: props.id,
      name
    }]);
  }

  function handleColumnChange(column: string) {
    executeOperations([{
      type: "set_compute_column",
      id: props.id,
      column
    }]);
  }

  function handleOperationChange(operationType: EComputeOperationType) {
    const operation: EComputeOperation =
      (
        operationType === "is_null"
        || operationType === "is_not_null"
        || operationType === "mean"
        || operationType === "median"
        || operationType === "min"
        || operationType === "max"
      )
      ? { type: operationType }
      : (
        props.operation.type === "is_null"
        || props.operation.type === "is_not_null"
        || props.operation.type === "mean"
        || props.operation.type === "median"
        || props.operation.type === "min"
        || props.operation.type === "max"
      )
        ? {
          type: operationType,
          param: {
            type: "constant",
            value: ""
          }
        }
        : {
          type: operationType,
          param: props.operation.param
        };
    executeOperations([{
      type: "set_compute_operation",
      id: props.id,
      operation
    }]);
  }

  function handleParamChange(param: EValue) {
    executeOperations([{
      type: "set_compute_operation",
      id: props.id,
      operation: {
        type: props.operation.type,
        param
      }
    }]);
  }

  const containerClass = selected
    ? queried
      ? styles.container_selected_queried
      : styles.container_selected
    : queried
      ? styles.container_queried
      : styles.container;

  return (
    <div
      className={containerClass}
      style={{
        top: props.position.y,
        left: props.position.x
      }}
    >
      <Header id={props.id}>
        <div className={styles.title}>
          <ComputeIcon/>
          <div>Compute</div>
        </div>
      </Header>

      <div className={styles.body}>
        <div className={styles.io}>
          <div className={styles.io_inputs}>
            <div className={styles.io_input}>
              <Port
                socket={{ id: props.id, type: "primary" }}
                connected={typeof inputs[0] === "string" || (openSocket?.id === props.id && openSocket?.type === "primary")}
              />
              <div className={styles.io_title}>Input</div>
            </div>
          </div>

          <div className={styles.io_outputs}>
            <div className={styles.io_output}>
              <div className={styles.io_title}>Output</div>
              <Port
                socket={{ id: props.id, type: "output" }}
                connected={props.outputs.length > 0 || (openSocket?.id === props.id && openSocket?.type === "output")}
              />
            </div>
          </div>
        </div>

        <div className={styles.props}>
          <Input
            placeholder="Name"
            value={props.name}
            onChange={handleNameChange}
          />
          <Input
            placeholder="Column"
            value={props.column}
            onChange={handleColumnChange}
          />

          <Select
            options={COMPUTE_OPERATION_OPTIONS}
            selected={props.operation.type}
            onSelect={handleOperationChange}
            roundTopLeft
            roundTopRight
            roundBottomLeft
            roundBottomRight
          />

  {/* IComputeAdd |
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
  IComputeMax; */}
          {
            (
              props.operation.type === "add"
              || props.operation.type === "subtract"
              || props.operation.type === "multiply"
              || props.operation.type === "divide"
              || props.operation.type === "is_equal_to"
              || props.operation.type === "is_not_equal_to"
              || props.operation.type === "is_less_than"
              || props.operation.type === "is_less_than_equal"
              || props.operation.type === "is_greater_than"
              || props.operation.type === "is_greater_than_equal"
              || props.operation.type === "and"
              || props.operation.type === "or"
              || props.operation.type === "xor"
            ) &&
            <Value
              placeholder="Other"
              value={props.operation.param}
              onChange={handleParamChange}
              roundTopLeft
              roundTopRight
              roundBottomLeft
              roundBottomRight
            />
          }
        </div>
      </div>
    </div>
  );
}