import { useAppSelector } from 'app/hooks';
import { nodes } from 'features/workspace/state/selectors';
import { FilterNode } from './FilterNode';
import { LoadCsvNode } from './LoadCsvNode';
import { SelectNode } from './SelectNode';

export function Node({ id }: { id: string }) {
  const _nodes = useAppSelector(nodes);
  const node = _nodes[id];

  switch (node?.type) {
    case 'load_csv': return <LoadCsvNode id={id} props={node} />;
    case 'select': return <SelectNode id={id} props={node} />;
    case 'filter': return <FilterNode id={id} props={node} />
    default: return null;
  }
}