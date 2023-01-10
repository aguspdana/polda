import styles from "./Output.module.css";
import { useStore } from "store";
import { CSSProperties, useEffect } from "react";
import { Table } from "./Table";
import { Panel } from "./Panel";

export function Output() {
  const height = useStore(state => state.outputHeight);
  const resizing = useStore(state => state.movable?.type === "output" && !state.movable.finished);
  const maximized = useStore(state => state.outputMaximized);
  const resize = useStore(state => state.resizeOutput);
  const table = useStore(state => state.query?.data);
  const error = useStore(state => state.query?.error);

  const style: CSSProperties = resizing
    ? {
      position: "absolute",
      bottom: 0,
      left: 0,
      right: 0,
      height
    }
    : maximized
      ? {
        position: "absolute",
        top: 0,
        bottom: 0,
        left: 0,
        right: 0
      }
      : { height };

  return (
    <div
      className={styles.container}
      style={style}
    >
      <div className={styles.resizer}>
        <div
          className={styles.resizer_handle}
          onMouseDown={resize}
        />
      </div>

      <Panel/>

      {error && (
        <div className={styles.error}>{error}</div>
      )}

      {table && (
        <Table data={table}/>
      )}
    </div>
  )
}