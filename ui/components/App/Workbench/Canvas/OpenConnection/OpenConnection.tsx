import { selectCursorOnCanvas, selectSocketPosition, ISocket, useStore } from "store";
import { Line } from "../Line";

interface Props {
  socket: ISocket
}

export function OpenConnection({ socket }: Props) {
  const socketPos = useStore(selectSocketPosition(socket));
  const cursorPos = useStore(selectCursorOnCanvas);

  const [from, to] = socket.type === "output"
    ? [socketPos, cursorPos]
    : [cursorPos, socketPos];

  return <Line from={from} to={to}/>
}