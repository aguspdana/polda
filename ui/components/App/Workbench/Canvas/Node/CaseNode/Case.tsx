import { exec } from "child_process";
import { EValue } from "lib/doc/node";
import { useStore } from "store";
import { Input } from "../common/Input";
import { InputGroup } from "../common/InputGroup";
import { Value } from "../common/Value";

interface Props {
  id: string,
  index: number,
  column: string,
  value: EValue,
  deletable: boolean
}

export function Case({ id, index, column, value, deletable }: Props) {
  const executeOperations = useStore(state => state.executeOperations);

  function handleColumnChange(column: string) {
    executeOperations([{
      type: "set_case_column",
      id,
      index,
      column
    }]);
  }

  function handleValueChange(value: EValue) {
    executeOperations([{
      type: "set_case_value",
      id,
      index,
      value
    }]);
  }

  function handleDelete() {
    executeOperations([{
      type: "delete_case",
      id,
      index
    }]);
  }

  return (
    <InputGroup
    handleDelete={deletable ? handleDelete : undefined}
    >
      <Input
        placeholder="If column is true"
        value={column}
        onChange={handleColumnChange}
        roundTopLeft
        roundTopRight={!deletable}
      />
      <Value
        placeholder="then"
        value={value}
        onChange={handleValueChange}
        roundBottomLeft
        roundBottomRight={!deletable}
      />
    </InputGroup>
  );
}