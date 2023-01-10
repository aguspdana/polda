import { FileIcon } from "components/icons/FileIcon";
import { ILoadCsvNode } from "lib/doc/node";
import { ISetLoadCSVPath } from "lib/doc/operation";
import { useStore } from "store";
import { Header } from "../common/Header";
import { Input } from "../common/Input";
import { Port } from "../common/Port";
import styles from "../Node.module.css";

export function LoadCsvNode(props: ILoadCsvNode) {
  const openSocket = useStore(state => state.openSocket);
  const selected = useStore(state => typeof state.selectedNodes[props.id] === "boolean");
  const queried = useStore(state => state.query?.type === "node" && state.query?.nodeId === props.id);
  const executeOperations = useStore(state => state.executeOperations);

  function handlePathChange(path: string) {
    const op: ISetLoadCSVPath = {
      type: "set_load_csv_filename",
      id: props.id,
      filename: path
    };
    executeOperations([op]);
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
          <FileIcon/>
          <div>Load CSV</div>
        </div>
      </Header>
      <div className={styles.body}>
        <div className={styles.io}>
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
            value={props.filename}
            placeholder="Path"
            onChange={handlePathChange}
            roundTopLeft
            roundTopRight
            roundBottomRight
            roundBottomLeft
          />
        </div>
      </div>
    </div>
  );
}