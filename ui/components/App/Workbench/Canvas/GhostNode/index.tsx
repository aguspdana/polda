import { selectCursorOnCanvas, useStore } from "store";
import styles from "./GhostNode.module.css";

export function GhostNode() {
  const cursor = useStore(selectCursorOnCanvas);
  const insertNode = useStore(state => state.insertNode);

  return (
    <div
      className={styles.ghost}
      style={{
        left: cursor.x,
        top: cursor.y
      }}
      onClick={insertNode}
    />
  );
}