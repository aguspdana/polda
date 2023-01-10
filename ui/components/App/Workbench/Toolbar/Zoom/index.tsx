import { ResetZoomIcon } from "components/icons/ResetZoomIcon";
import { ZoomInIcon } from "components/icons/ZoomInIcon";
import { ZoomOutIcon } from "components/icons/ZoomOutIcon";
import { ZoomToFitIcon } from "components/icons/ZoomToFitIcon";
import { useEffect, useRef, useState } from "react";
import { useStore } from "store";
import styles from "./Zoom.module.css";

export function Zoom() {
  const zoom = useStore(state => state.canvasZoom);
  const zoomIn = useStore(state => state.zoomIn);
  const zoomOut = useStore(state => state.zoomOut);
  const resetZoom = useStore(state => state.resetZoom);
  const zoomToFit = useStore(state => state.zoomToFit);
  const [open, setOpen] = useState(false);
  const container = useRef<HTMLDivElement | null>(null);

  useEffect(() => {
    function handler() {
      setOpen(false);
    }
    window.addEventListener("blur", handler);
    return () => window.removeEventListener("blur", handler);
  }, []);

  useEffect(() => {
    function handler(e: MouseEvent) {
      if (container.current && !container.current.contains(e.target as Node)) {
        setOpen(false);
      }
    }
    document.addEventListener("mousedown", handler);
    return () => document.removeEventListener("mousedown", handler);
  }, []);
  
  return (
    <div
      className={styles.container}
      ref={container}
    >
      <button
        className={open ? styles.toggle_open : styles.toggle}
        onClick={() => setOpen(o => !o)}
      >
        {`${Math.round(zoom*100)}%`}
      </button>

      {open && (
        <div className={styles.drop_down}>
          <div className={styles.panel}>
            <button
              className={styles.action}
              onClick={zoomIn}
            >
              <ZoomInIcon/>
            </button>
            <button
              className={styles.action}
              onClick={zoomOut}
            >
              <ZoomOutIcon/>
            </button>
            <button
              className={styles.action}
              onClick={resetZoom}
            >
              <ResetZoomIcon/>
            </button>
            <button
              className={styles.action}
              onClick={zoomToFit}
            >
              <ZoomToFitIcon/>
            </button>
          </div>
        </div>
      )}
    </div>
  );
}