import { IDeleteSelect, ISetSelectAlias, ISetSelectColumn } from "lib/doc/operation";
import { useStore } from "store";
import { Input } from "../common/Input";
import { InputGroup } from "../common/InputGroup";

interface Props {
  id: string,
  index: number,
  column: string,
  alias: string,
  deletable: boolean
}

export function SelectColumn({ id, index, column, alias, deletable }: Props) {
  const executeOperations = useStore(state => state.executeOperations);

  function handleColumnChange(value: string) {
    const op: ISetSelectColumn = {
      type: "set_select_column",
      id,
      index,
      column: value
    };
    executeOperations([op]);
  }

  function handleAliasChange(value: string) {
    const op: ISetSelectAlias = {
      type: "set_select_alias",
      id,
      index,
      alias: value
    };
    executeOperations([op]);
  }

  function handleDelete() {
    const op: IDeleteSelect = {
      type: "delete_select",
      id,
      index
    };
    executeOperations([op]);
  }

  return (
    <InputGroup
      handleDelete={deletable ? handleDelete : undefined}
    >
      <Input
        value={column}
        placeholder="Column"
        onChange={handleColumnChange}
        roundTopLeft
        roundTopRight={!deletable}
      />
      <Input
        value={alias}
        placeholder="Alias"
        onChange={handleAliasChange}
        roundBottomRight={!deletable}
        roundBottomLeft
      />
    </InputGroup>
  );
}