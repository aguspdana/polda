import { IDeleteJoinColumn, ISetJoinColumnLeft, ISetJoinColumnRight } from "lib/doc/operation";
import { useStore } from "store";
import { Input } from "../common/Input";
import { InputGroup } from "../common/InputGroup";

interface Props {
  id: string,
  index: number,
  left: string,
  right: string,
  deletable: boolean
}

export function JoinColumn({ id, index, left, right, deletable }: Props) {
  const executeOperations = useStore(state => state.executeOperations);

  function handleLeftColumnChange(value: string) {
    executeOperations([{
      type: "set_join_column_left",
      id,
      index,
      column: value
    }]);
  }

  function handleRightColumnChange(value: string) {
    const op: ISetJoinColumnRight = {
      type: "set_join_column_right",
      id,
      index,
      column: value
    };
    executeOperations([op]);
  }

  function handleDelete() {
    const op: IDeleteJoinColumn = {
      type: "delete_join_column",
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
        value={left}
        placeholder="Left column"
        onChange={handleLeftColumnChange}
        roundTopLeft
        roundTopRight={!deletable}
      />
      <Input
        value={right}
        placeholder="Right column"
        onChange={handleRightColumnChange}
        roundBottomRight={!deletable}
        roundBottomLeft
      />
    </InputGroup>
  );
}