import { getInputs, ISortNode } from "lib/doc/node";
import { useStore } from "store";
import { Header } from "../common/Header";
import { Port } from "../common/Port";
import { AddInputButton } from "../common/AddInputButton";
import styles from "../Node.module.css";
import { Sorter } from "./Sorter";
import { SortIcon } from "components/icons/SortIcon";
import { IInsertSorter } from "lib/doc/operation";

export function SortNode(props: ISortNode) {
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
    const op: IInsertSorter = {
      type: "insert_sorter",
      id: props.id,
      index: props.sorters.length,
      sorter: {
        column: "",
        direction: "desc"
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
          <SortIcon/>
          <div>Sort</div>
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
          {props.sorters.map(({ column, direction }, i) => (
            <Sorter
              id={props.id}
              index={i}
              column={column}
              direction={direction}
              deletable={props.sorters.length > 1}
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