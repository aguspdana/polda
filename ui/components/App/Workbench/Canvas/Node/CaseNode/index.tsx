import { ConditionalIcon } from "components/icons/ConditionalIcon";
import { ECaseDataType, EValue, getInputs, ICaseNode } from "lib/doc/node";
import { useStore } from "store";
import { AddInputButton } from "../common/AddInputButton";
import { Header } from "../common/Header";
import { Input } from "../common/Input";
import { Port } from "../common/Port";
import { Select } from "../common/Select";
import { Value } from "../common/Value";
import styles from "../Node.module.css";
import { Case } from "./Case";

const CASE_DATA_TYPE_OPTIONS: {
  value: ECaseDataType,
  display: string
}[] = [
  {
    value: ECaseDataType.Boolean,
    display: ECaseDataType.Boolean
  },
  {
    value: ECaseDataType.Float32,
    display: ECaseDataType.Float32
  },
  {
    value: ECaseDataType.Float64,
    display: ECaseDataType.Float64
  },
  {
    value: ECaseDataType.Int8,
    display: ECaseDataType.Int8
  },
  {
    value: ECaseDataType.Int16,
    display: ECaseDataType.Int16
  },
  {
    value: ECaseDataType.Int32,
    display: ECaseDataType.Int32
  },
  {
    value: ECaseDataType.Int64,
    display: ECaseDataType.Int64
  },
  {
    value: ECaseDataType.UInt8,
    display: ECaseDataType.UInt8
  },
  {
    value: ECaseDataType.UInt16,
    display: ECaseDataType.UInt16
  },
  {
    value: ECaseDataType.UInt32,
    display: ECaseDataType.UInt32
  },
  {
    value: ECaseDataType.UInt64,
    display: ECaseDataType.UInt64
  },
  {
    value: ECaseDataType.Utf8,
    display: ECaseDataType.Utf8
  }
];

export function CaseNode(props: ICaseNode) {
  const openSocket = useStore(state => state.openSocket);
  const inputs = getInputs(props);
  const selected = useStore(state => typeof state.selectedNodes[props.id] === "boolean");
  const queried = useStore(state => state.query?.type === "node" && state.query?.nodeId === props.id);
  const executeOperations = useStore(state => state.executeOperations);

  function handleNameChange(name: string) {
    executeOperations([{
      type: "set_case_name",
      id: props.id,
      name
    }]);
  }

  function handleDataTypeChange(data_type: ECaseDataType) {
    executeOperations([{
      type: "set_case_data_type",
      id: props.id,
      data_type
    }]);
  }

  function insertCase() {
    executeOperations([{
      type: "insert_case",
      id: props.id,
      index: props.cases.length,
      case: {
        column: "",
        value: {
          type: "constant",
          value: ""
        }
      }
    }]);
  }

  function handleDefaultChange(value: EValue) {
    executeOperations([{
      type: "set_case_default",
      id: props.id,
      default: value
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
          <ConditionalIcon/>
          <div>Case</div>
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
            value={props.name}
            placeholder="Name"
            onChange={handleNameChange}
            roundTopLeft
            roundTopRight
            roundBottomLeft
            roundBottomRight
          />

          <Select
            options={CASE_DATA_TYPE_OPTIONS}
            selected={props.data_type}
            onSelect={handleDataTypeChange}
            roundTopLeft
            roundTopRight
            roundBottomLeft
            roundBottomRight
          />

          {props.cases.map(({ column, value}, i) => (
            <Case
              key={i}
              id={props.id}
              index={i}
              column={column}
              value={value}
              deletable={props.cases.length > 1}
            />
          ))}
          
          <AddInputButton
            onClick={insertCase}
          />

          <Value
            placeholder="Default"
            value={props.default}
            onChange={handleDefaultChange}
            roundTopLeft
            roundTopRight
            roundBottomLeft
            roundBottomRight
          />
        </div>
      </div>
    </div>
  );
}