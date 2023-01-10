import { useStore } from "store";
import { Canvas } from "./Canvas";
import { Output } from "./Output";
import { Toolbar } from "./Toolbar";
import styles from "./Workbench.module.css";

export function Workbench() {
  const outputOpened = useStore(state => state.query !== null);

  return (
    <div className={styles.container}>
      <div className={styles.toolbar_canvas}>
          <Toolbar/>
          <Canvas/>
      </div>

      {outputOpened && <Output/>}
    </div>
  )
}