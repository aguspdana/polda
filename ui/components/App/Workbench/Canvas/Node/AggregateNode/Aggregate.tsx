import { EAggregateComputation } from "lib/doc/node";
import { useStore } from "store";
import { Input } from "../common/Input";
import { InputGroup } from "../common/InputGroup";
import { Select } from "../common/Select";

const COMPUTATIONS: {
  value: EAggregateComputation,
  display: string
}[] = [
  {
    value: "count",
    display: "Count"
  },
  {
    value: "first",
    display: "First"
  },
  {
    value: "group",
    display: "Group"
  },
  {
    value: "last",
    display: "Last"
  },
  {
    value: "max",
    display: "Max"
  },
  {
    value: "mean",
    display: "Mean"
  },
  {
    value: "min",
    display: "Min"
  },
  {
    value: "sum",
    display: "Sum"
  },
];

interface Props {
  id: string,
  index: number,
  column: string,
  computation: EAggregateComputation,
  alias: string,
  deletable: boolean
}

export function Aggregate({ id, index,column, computation, alias, deletable }: Props) {
  const executeOperations = useStore(state => state.executeOperations);

  function handleColumnChange(value: string) {
    executeOperations([{
      type: "set_aggregate_column",
      id,
      index,
      column: value
    }]);
  }

  function handleComputationChange(value: string) {
    executeOperations([{
      type: "set_aggregate_computation",
      id,
      index,
      computation: value as EAggregateComputation
    }]);
  }

  function handleAliasChange(value: string) {
    executeOperations([{
      type: "set_aggregate_alias",
      id,
      index,
      alias: value
    }]);
  }

  function handleDelete() {
    executeOperations([{
      type: "delete_aggregate",
      id,
      index
    }]);
  }

  return (
    <InputGroup
      handleDelete={deletable ? handleDelete : undefined}
    >
      <Input
        value={column}
        placeholder="Column*"
        onChange={handleColumnChange}
        roundTopLeft
        roundTopRight={!deletable}
      />
      <Select
        options={COMPUTATIONS}
        selected={computation}
        onSelect={handleComputationChange}
      />
      <Input
        value={alias}
        placeholder="Alias"
        onChange={handleAliasChange}
        roundBottomLeft
        roundBottomRight={!deletable}
      />
    </InputGroup>
  );
}