import { CastIcon } from "components/icons/CastIcon";
import { ECastDataType, getInputs, ICastNode } from "lib/doc/node";
import { useStore } from "store";
import { Header } from "../common/Header";
import { Input } from "../common/Input";
import { Port } from "../common/Port";
import { Select } from "../common/Select";
import styles from "../Node.module.css";

const CAST_DATA_TYPE_OPTIONS: {
  value: ECastDataType,
  display: string
}[] = [
  {
    value: ECastDataType.Boolean,
    display: ECastDataType.Boolean
  },
  {
    value: ECastDataType.Float32,
    display: ECastDataType.Float32
  },
  {
    value: ECastDataType.Float64,
    display: ECastDataType.Float64
  },
  {
    value: ECastDataType.Int8,
    display: ECastDataType.Int8
  },
  {
    value: ECastDataType.Int16,
    display: ECastDataType.Int16
  },
  {
    value: ECastDataType.Int32,
    display: ECastDataType.Int32
  },
  {
    value: ECastDataType.Int64,
    display: ECastDataType.Int64
  },
  {
    value: ECastDataType.UInt8,
    display: ECastDataType.UInt8
  },
  {
    value: ECastDataType.UInt16,
    display: ECastDataType.UInt16
  },
  {
    value: ECastDataType.UInt32,
    display: ECastDataType.UInt32
  },
  {
    value: ECastDataType.UInt64,
    display: ECastDataType.UInt64
  }
];

export function CastNode(props: ICastNode) {
  const openSocket = useStore(state => state.openSocket);
  const selected = useStore(state => typeof state.selectedNodes[props.id] === "boolean");
  const queried = useStore(state => state.query?.type === "node" && state.query?.nodeId === props.id);
  const executeOperations = useStore(state => state.executeOperations);

  function handleNameChange(value: string) {
    executeOperations([{
      type: "set_cast_name",
      id: props.id,
      name: value
    }]);
  }

  function handleColumnChange(value: string) {
    executeOperations([{
      type: "set_cast_column",
      id: props.id,
      column: value
    }]);
  }

  function handleDataTypeChange(value: ECastDataType) {
    executeOperations([{
      type: "set_cast_data_type",
      id: props.id,
      data_type: value
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
          <CastIcon/>
          <div>Cast</div>
        </div>
      </Header>

      <div className={styles.body}>
        <div className={styles.io}>
          <div className={styles.io_inputs}>
            <div className={styles.io_input}>
              <Port
                socket={{ id: props.id, type: "primary" }}
                connected={typeof props.input === "string" || (openSocket?.id === props.id && openSocket?.type === "primary")}
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
            onChange={handleNameChange}
            placeholder="Name"
            roundTopLeft
            roundTopRight
            roundBottomLeft
            roundBottomRight
          />
          <Input
            value={props.column}
            onChange={handleColumnChange}
            placeholder="Column"
            roundTopLeft
            roundTopRight
            roundBottomLeft
            roundBottomRight
          />
          <Select
            options={CAST_DATA_TYPE_OPTIONS}
            selected={props.data_type}
            onSelect={handleDataTypeChange}
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