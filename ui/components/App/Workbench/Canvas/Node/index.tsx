import { useStore } from "store";
import { AggregateNode } from "./AggregateNode";
import { CaseNode } from "./CaseNode";
import { CastNode } from "./Castnode";
import { ComputeNode } from "./ComputeNode";
import { FilterNode } from "./FilterNode";
import { JoinNode } from "./JoinNode";
import { LoadCsvNode } from "./LoadCsvNode";
import { SelectNode } from "./SelectNode";
import { SortNode } from "./SortNode";
import { UnionNode } from "./UnionNode";

interface Props {
  id: string
}

export function Node({ id }: Props) {
  const node = useStore(state => state.doc?.nodes[id]);

  switch (node?.type) {
    case "aggregate":
      return <AggregateNode {...node}/>;
    case "case":
      return <CaseNode {...node}/>;
    case "cast":
      return <CastNode {...node}/>;
    case "compute":
      return <ComputeNode {...node}/>;
    case "filter":
      return <FilterNode {...node}/>
    case "join":
      return <JoinNode {...node}/>;
    case "load_csv":
      return <LoadCsvNode {...node}/>;
    case "select":
      return <SelectNode {...node}/>;
    case "sort":
      return <SortNode {...node}/>;
    case "union":
      return <UnionNode {...node}/>;
    default:
      return null;
  }
}