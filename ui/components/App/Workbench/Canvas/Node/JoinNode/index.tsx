import { EJoinType, getInputs, IJoinNode } from "lib/doc/node";
import { useStore } from "store";
import { AddInputButton } from "../common/AddInputButton";
import { Header } from "../common/Header";
import { Port } from "../common/Port";
import styles from "../Node.module.css";
import { JoinColumn } from "./JoinColumn";
import { LeftJoinIcon } from "components/icons/LeftJoinIcon";
import { INodeTitleOption, NodeTitleSelect } from "../common/NodeTitleSelect";
import { RightJoinIcon } from "components/icons/RightJoinIcon";
import { InnerJoinIcon } from "components/icons/InnerJoinIcon";
import { FullJoinIcon } from "components/icons/FullJoinIcon";
import { CrossJoinIcon } from "components/icons/CrossJoinIcon";
import { IInsertJoinColumn, ISetJoinType } from "lib/doc/operation";

const OPTIONS: INodeTitleOption<EJoinType>[] = [
  {
    value: "left",
    icon: <LeftJoinIcon/>,
    display: "Left join"
  },
  {
    value: "right",
    icon: <RightJoinIcon/>,
    display: "Right join"
  },
  {
    value: "inner",
    icon: <InnerJoinIcon/>,
    display: "Inner join"
  },
  {
    value: "full",
    icon: <FullJoinIcon/>,
    display: "Full join"
  },
  {
    value: "cross",
    icon: <CrossJoinIcon/>,
    display: "Cross join"
  },
];

export function JoinNode(props: IJoinNode) {
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
  
  function setJoinType(value: EJoinType) {
    const op: ISetJoinType = {
      type: "set_join_type",
      id: props.id,
      join_type: value
    };
    executeOperations([op]);
  }
  
  function insertAggregate() {
    const op: IInsertJoinColumn = {
      type: "insert_join_column",
      id: props.id,
      index: props.columns.length,
      join_column: {
        left: "",
        right: ""
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
        <NodeTitleSelect
          options={OPTIONS}
          selected={props.join_type}
          onSelect={setJoinType}
        />
      </Header>

      <div className={styles.body}>
        <div className={styles.io}>
          <div className={styles.io_inputs}>
            <div className={styles.io_input}>
              <Port
                socket={{ id: props.id, type: "primary" }}
                connected={typeof inputs[0] === "string" || (openSocket?.id === props.id && openSocket?.type === "primary")}
              />
              <div className={styles.io_title}>Left input</div>
            </div>
            <div className={styles.io_input}>
              <Port
                socket={{ id: props.id, type: "secondary" }}
                connected={typeof inputs[1] === "string" || (openSocket?.id === props.id && openSocket?.type === "secondary")}
              />
              <div className={styles.io_title}>Right input</div>
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
          {props.columns.map(({ left, right  }, i) => (
            <JoinColumn
              id={props.id}
              index={i}
              left={left}
              right={right}
              deletable={props.columns.length > 1}
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