import { Sidebar } from "./Sidebar";
import { Workbench } from "./Workbench";
import { useEffect } from "react";
import { EIncomingMessage, EOutgoingMessage, useStore } from "store";
import { useWS } from "hooks/useWS";
import { WS_URL } from "config";
import styles from "./App.module.css";
import { Toast } from "./Toast";

export function App() {
  const setCursorPosition = useStore(state => state.setCursorPosition);
  const setWindowDimension = useStore(state => state.setWindowDimension);
  const resetMovable = useStore(state => state.finishMovable);
  const resetOpenSocket = useStore(state => state.resetOpenSocket);
  const handleMsg = useStore(state => state.handleMessage);
  const resetClientId = useStore(state => state.resetClientId);
  const setSender = useStore(state => state.setSender);
  const { connected, send } = useWS<EIncomingMessage, EOutgoingMessage>(WS_URL, handleMsg);
  const setPath = useStore(state => state.setPath);
  const sidebarOpen = useStore(state => state.sidebarOpen);
  const deleteSelectedNodes = useStore(state => state.deleteSelectedNodes);
  const toasts = useStore(state => state.toasts);

  // FIXME: Path should be set based on the current url.
  useEffect(() => {
    setPath("");
  }, [setPath]);

  useEffect(() => {
    if (!connected) {
      resetClientId();
    }
  }, [connected, resetClientId]);

  useEffect(() => {
    setSender(send);
  }, [send, setSender]);

  // Track cursor position.
  useEffect(() => {
    function handleMouseMove(e: MouseEvent) {
      setCursorPosition(e.clientX, e.clientY);
    }
    window.addEventListener("mousemove", handleMouseMove)
    return () => {
      window.removeEventListener("mousemove", handleMouseMove);
    };
  }, [setCursorPosition]);

  useEffect(() => {
    function handleMouseUp() {
      resetMovable();
    }
    window.addEventListener("mouseup", handleMouseUp)
    return () => {
      window.removeEventListener("mouseup", handleMouseUp);
    };
  }, [resetMovable]);

  useEffect(() => {
    function handleMouseUp() {
      resetOpenSocket();
    }
    window.addEventListener("mouseup", handleMouseUp)
    return () => {
      window.removeEventListener("mouseup", handleMouseUp);
    };
  }, [resetOpenSocket]);

  useEffect(() => {
    setWindowDimension(window.innerWidth, window.innerHeight);
    function handleResize() {
      setWindowDimension(window.innerWidth, window.innerHeight);
    }
    window.addEventListener("resize", handleResize)
    return () => {
      window.removeEventListener("resize", handleResize);
    };
  }, [setWindowDimension]);

  useEffect(() => {
    function handleKeydown(e: KeyboardEvent) {
      if (["Backspace", "Delete"].includes(e.key)) {
        deleteSelectedNodes();
      }
    }
    window.addEventListener("keydown", handleKeydown)
    return () => {
      window.removeEventListener("keydown", handleKeydown);
    };
  }, [deleteSelectedNodes]);

  return (
    <div className={styles.container}>
      {sidebarOpen && <Sidebar />}
      <Workbench />

      {toasts.length > 0 && (
        <div className={styles.toasts}>
          {toasts.map(toast => (
            <Toast key={toast.id} toast={toast}/>
          ))}
        </div>
      )}
    </div>
  )
}