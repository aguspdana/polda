import { PoldaIcon } from "components/icons/PoldaIcon";
import { useStore } from "store";
import { Section } from "./Section";
import styles from "./Sidebar.module.css";

export function Sidebar() {
  const width = useStore(state => state.sidebarWidth);
  const resize = useStore(state => state.resizeSidebar);
  const sources = useStore(state => state.sources);
  const selectedSource = useStore(state => state.query?.type === "file" ? state.query.filename : undefined);
  const readFile = useStore(state => state.readFile);

  return (
    <div
      className={styles.container}
      style={{width}}
    >
      <div className={styles.content}>
        <header className={styles.logo}>
          <PoldaIcon/>
        </header>

        <Section
          title="Analyses"
          items={["Demo"]}
          selected="Demo"
        />

        <Section
          title="Data"
          items={sources}
          selected={selectedSource}
          select={readFile}
        />
      </div>

      <div className={styles.resizer}>
        <div
          className={styles.resizer_handle}
          onMouseDown={() => resize()}
        />
      </div>
    </div>
  )
}