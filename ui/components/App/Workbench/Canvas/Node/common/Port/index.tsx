import { ISocket, useStore } from "store";
import styles from "./Port.module.css";

interface Props {
  socket: ISocket,
  connected: boolean
}

export function Port({ socket, connected }: Props) {
  const connect = useStore(state => state.connectSocket);
  const disconnect = useStore(state => state.disconnectSocket);

  function handleMouseDown() {
    if (socket.type !== "output" && connected) {
      disconnect(socket);
    } else {
      connect(socket);
    }
  }

  return (
    <div
      className={styles.container}
      onMouseDown={handleMouseDown}
      onMouseUp={() => connect(socket) }
    >
      <div className={connected ? styles.port_connected : styles.port}/>
    </div>
  );
}