import { ESortDirection } from "lib/doc/node";
import { IDeleteSorter, ISetSortColumn, ISetSortDirection } from "lib/doc/operation";
import { useStore } from "store";
import { Input } from "../common/Input";
import { InputGroup } from "../common/InputGroup";
import { IOptionSelect, Select } from "../common/Select";

const SORT_DIRECTIONS: IOptionSelect<ESortDirection>[] = [
  {
    value: "asc",
    display: "Ascending"
  },
  {
    value: "desc",
    display: "Descending"
  }
];

interface Props {
  id: string,
  index: number,
  column: string,
  direction: ESortDirection,
  deletable: boolean
}

export function Sorter({ id, index, column, direction, deletable }: Props) {
  const executeOperations = useStore(state => state.executeOperations);

  function handleColumnChange(value: string) {
    const op: ISetSortColumn = {
      type: "set_sort_column",
      id,
      index,
      column: value
    };
    executeOperations([op]);
  }

  function handleDirectionChange(value: ESortDirection) {
    const op: ISetSortDirection = {
      type: "set_sort_direction",
      id,
      index,
      direction: value
    };
    executeOperations([op]);
  }

  function handleDelete() {
    const op: IDeleteSorter = {
      type: "delete_sorter",
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
      <Select
        options={SORT_DIRECTIONS}
        selected={direction}
        onSelect={handleDirectionChange}
        roundBottomLeft
        roundBottomRight
      />
    </InputGroup>
  );
}