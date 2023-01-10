import { AggregateIcon } from "components/icons/AggregateIcon";
import { getInputs, IAggregateNode } from "lib/doc/node";
import { IInsertAggregate } from "lib/doc/operation";
import { useStore } from "store";
import { AddInputButton } from "../common/AddInputButton";
import { Header } from "../common/Header";
import { Port } from "../common/Port";
import styles from "../Node.module.css";
import { Aggregate } from "./Aggregate";

export function AggregateNode(props: IAggregateNode) {
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
  
  function insertAggregate() {
    const op: IInsertAggregate = {
      type: "insert_aggregate",
      id: props.id,
      index: props.aggregates.length,
      aggregate: {
        column: "",
        computation: "count",
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
          <AggregateIcon/>
          <div>Aggregate</div>
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
          {props.aggregates.map(({ column, computation, alias }, i) => (
            <Aggregate
              id={props.id}
              index={i}
              column={column}
              computation={computation}
              alias={alias}
              deletable={props.aggregates.length > 1}
              key={i}
            />
          ))}
          
          <AddInputButton
            onClick={insertAggregate}
          />
        </div>
      </div>
    </div>
  );
}