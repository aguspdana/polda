import { useAppSelector } from 'app/hooks';
import { openConnectionEndpoints } from 'features/workspace/state/selectors';
import { Line } from '../Line';

export function OpenConnection() {
  const endpoints = useAppSelector(openConnectionEndpoints);

  if (endpoints === null) return null;

  return (
    <Line from={endpoints.from} to={endpoints.to} />
  )
}