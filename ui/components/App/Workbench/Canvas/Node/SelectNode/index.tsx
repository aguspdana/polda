import { SelectIcon } from "components/icons/SelectIcon";
import { getInputs, ISelectNode } from "lib/doc/node";
import { useStore } from "store";
import { Header } from "../common/Header";
import { Port } from "../common/Port";
import { SelectColumn } from "./SelectColumn";
import { AddInputButton } from "../common/AddInputButton";
import styles from "../Node.module.css";
import { IInsertSelect } from "lib/doc/operation";

export function SelectNode(props: ISelectNode) {
  const openSocket = useStore(state => state.openSocket);
  const inputs = getInputs(props);
  const selected = useStore(state => typeof state.selectedNodes[props.id] === "boolean");
  const queried = useStore(state => state.query?.type === "node" && state.query?.nodeId === props.id);
  const executeOperations = useStore(state => state.executeOperations);

  const containerClass = selected
    ? queried
      ? styles.container_selected_queried
      : styles.container_selected
    : queried
      ? styles.container_queried
      : styles.container;
  
  function insertColumn() {
    const op: IInsertSelect = {
      type: "insert_select",
      id: props.id,
      index: props.columns.length,
      column: {
        column: "",
        alias: ""
      }
    };
    executeOperations([op]);
  }

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
          <SelectIcon/>
          <div>Select</div>
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
          {props.columns.map(({ column, alias }, i) => (
            <SelectColumn
              id={props.id}
              index={i}
              column={column}
              alias={alias}
              deletable={props.columns.length > 1}
              key={i}
            />
          ))}
          
          <AddInputButton
            onClick={insertColumn}
          />
        </div>
      </div>
    </div>
  );
}