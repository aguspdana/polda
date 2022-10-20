import { useAppSelector } from 'app/hooks';
import { connectionEndpoints } from 'features/workspace/state/selectors';
import { Line } from '../Line';

export function Connection(inputSocket: { nodeId: string, index: number }) {
  const endpoints = useAppSelector(connectionEndpoints(inputSocket));

  if (endpoints === null) return null;

  return (
    <Line from={endpoints.from} to={endpoints.to} />
  )
}