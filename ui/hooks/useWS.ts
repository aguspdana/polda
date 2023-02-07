import { WS } from "lib/ws";
import { useEffect, useRef, useState } from "react";

export function useWS<IncommingMsg = object, OutgoingMsg = object>(url: string, handler: (msg: IncommingMsg) => void) {
  const [socket, setSocket] = useState<WS | null>(null);
  const [connected, setConnected] = useState(false);

  useEffect(() => {
    const socket = new WS(
      url,
      {
        reconnectOnClose: true,
        clearQueueOnClose: true
      }
    );

    socket.addEventListener("open", () => {
      setConnected(false);
    });

    socket.onerror = () => {
      setConnected(false);
    };

    socket.onclose = () => {
      setConnected(false);
    };

    socket.onopen = () => {
      setConnected(true);
    }

    setSocket(socket);

    return () => {
      socket.close();
    };
  }, [url, handler]);

  useEffect(() => {
    if (socket) {
      socket.onmessage = (e) => {
        if (typeof e.data === "string") {
          try {
            handler(JSON.parse(e.data));
          } catch {
            console.error("Received an invalid web socket message");
          }
        }
      };
    }
  }, [socket, handler]);

  function send(msg: OutgoingMsg) {
    socket?.send(JSON.stringify(msg));
  }

  return { connected, send };
}
