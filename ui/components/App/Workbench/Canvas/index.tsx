import { selectConnections, useStore } from "store";
import styles from "./Canvas.module.css";
import { Connection } from "./Connection";
import { GhostNode } from "./GhostNode";
import { Grid } from "./Grid";
import { Node } from "./Node";
import { OpenConnection } from "./OpenConnection";

export function Canvas() {
  const index = useStore(state => state.doc?.index || []);
  const x = useStore(state => state.canvasX);
  const y = useStore(state => state.canvasY);
  const zoom = useStore(state => state.canvasZoom);
  const move = useStore(state => state.moveCanvas);
  const isCanvasMoving = useStore(state => state.movable?.type === "canvas" && !state.movable.finished);
  const isCanvasMoved = useStore(state => state.movable?.type === "canvas" && state.movable?.moved);
  const resetSelection = useStore(state => state.unselectAllNodes);
  const connections = useStore(selectConnections);
  const openSocket = useStore(state => state.openSocket);
  const newNode = useStore(state => state.newNode);

  return (
    <div className={styles.container}>
      <Grid/>

      <div
        className={styles.background}
        onMouseDown={move}
        onClick={() => { if (!isCanvasMoved) resetSelection() }}
      />

      <div
        className={styles.canvas}
        style={{
          transform: `translate(${x}px, ${y}px) scale(${zoom})`,
          transitionDuration: isCanvasMoving ? "0s" : undefined
        }}
      >
        {connections.map(({ from, to }) => (
          <Connection key={`${to.id}::${to.type}`} from={from} to={to}/>
        ))}

        {index.map(id => (
          <Node key={id} id={id}/>
        ))}

        {openSocket !== null && <OpenConnection socket={openSocket}/>}

        {newNode !== null && <GhostNode/>}
      </div>
    </div>
  )
}