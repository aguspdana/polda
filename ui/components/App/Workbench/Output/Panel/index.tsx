import { CloseIcon } from "components/icons/CloseIcon";
import { DownloadIcon } from "components/icons/DownloadIcon";
import { FullScreenIcon } from "components/icons/FullScreenIcon";
import { TileScreenIcon } from "components/icons/TileScreenIcon";
import { useStore } from "store";
import styles from "./Panel.module.css";

export function Panel() {
  const maximized = useStore(state => state.outputMaximized);
  const maximize = useStore(state => state.maximizeOutput);
  const minimize = useStore(state => state.minimizeOutput);
  const close = useStore(state => state.closeOutput);
  const pushToast = useStore(state => state.pushToast);

  return (
    <div className={styles.container}>
      <div className={styles.group}>
        <button className={styles.tab_btn_active}>TABLE</button>
        <button
          className={styles.tab_btn}
          onClick={() => pushToast("This feature is not implemented yet.")}
        >
          SUMMARY
        </button>
      </div>

      <div className={styles.group}>
        <button
          className={styles.action_btn}
          onClick={() => pushToast("This feature is not implemented yet.")}
        >
          <DownloadIcon/>
        </button>
        {
          maximized
            ? (
              <button
                className={styles.action_btn}
                onClick={minimize}
              >
                <TileScreenIcon/>
              </button>
            )
            : (
              <button
                className={styles.action_btn}
                onClick={maximize}
              >
                <FullScreenIcon/>
              </button>
            )
        }
        <button
          className={styles.action_btn}
          onClick={close}
        >
          <CloseIcon/>
        </button>
      </div>
    </div>
  );
}