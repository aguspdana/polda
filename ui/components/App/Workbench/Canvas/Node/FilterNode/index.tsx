import { FilterIcon } from "components/icons/FilterIcon";
import { IFilterNode, getInputs, EFilterPredicate, EValue } from "lib/doc/node";
import { useStore } from "store";
import { Header } from "../common/Header";
import { Input } from "../common/Input";
import { Port } from "../common/Port";
import { Select } from "../common/Select";
import { Value } from "../common/Value";
import styles from "../Node.module.css";

type EFilterPredicateType = EFilterPredicate["type"];

const FILTER_PREDICATE_OPTIONS: {
  value: EFilterPredicateType,
  display: string
}[] = [
  {
    value: "is_equal_to",
    display: "="
  },
  {
    value: "is_not_equal_to",
    display: "<>"
  },
  {
    value: "is_less_than_equal",
    display: "<="
  },
  {
    value: "is_greater_than",
    display: ">"
  },
  {
    value: "is_greater_than_equal",
    display: ">="
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
  }
]

export function FilterNode(props: IFilterNode) {
  const openSocket = useStore(state => state.openSocket);
  const inputs = getInputs(props);
  const selected = useStore(state => typeof state.selectedNodes[props.id] === "boolean");
  const queried = useStore(state => state.query?.type === "node" && state.query?.nodeId === props.id);
  const executeOperations = useStore(state => state.executeOperations);

  function handleColumnChange(value: string) {
    executeOperations([{
      type: "set_filter_column",
      id: props.id,
      column: value
    }]);
  }

  function handlePredicateChange(predicateType: EFilterPredicateType) {
    const predicate: EFilterPredicate =
      predicateType === "is_null" || predicateType === "is_not_null"
      ? { type: predicateType }
      : props.predicate.type === "is_null" || props.predicate.type === "is_not_null"
        ? {
          type: predicateType,
          param: {
            type: "constant",
            value: ""
          }
        }
        : {
          type: predicateType,
          param: props.predicate.param
        };
    executeOperations([{
      type: "set_filter_predicate",
      id: props.id,
      predicate
    }]);
  }

  function handleParamChange(param: EValue) {
    if (props.predicate.type === "is_null" || props.predicate.type === "is_not_null") {
      return;
    }
    executeOperations([{
      type: "set_filter_predicate",
      id: props.id,
      predicate: {
        type: props.predicate.type,
        param: param
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
          <FilterIcon/>
          <div>Filter</div>
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
            value={props.column}
            placeholder="Column"
            onChange={handleColumnChange}
            roundTopLeft
            roundTopRight
            roundBottomLeft
            roundBottomRight
          />
          <Select
            options={FILTER_PREDICATE_OPTIONS}
            selected={props.predicate.type}
            onSelect={handlePredicateChange}
            roundTopLeft
            roundTopRight
            roundBottomLeft
            roundBottomRight
          />

        {
          (
            props.predicate.type === "is_equal_to"
            || props.predicate.type === "is_not_equal_to"
            || props.predicate.type === "is_less_than"
            || props.predicate.type === "is_less_than_equal"
            || props.predicate.type === "is_greater_than"
            || props.predicate.type === "is_greater_than_equal"
          ) &&
            <Value
              placeholder="Other"
              value={props.predicate.param}
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