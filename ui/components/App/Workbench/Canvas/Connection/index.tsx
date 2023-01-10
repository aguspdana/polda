import { selectSocketPosition, ISocket, useStore } from "store";
import { Line } from "../Line";

interface Props {
  from: ISocket,
  to: ISocket
}

export function Connection(props: Props) {
  const from = useStore(selectSocketPosition(props.from));
  const to = useStore(selectSocketPosition(props.to));

  return <Line from={from} to={to}/>
}