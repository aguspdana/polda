import { AggregateIcon } from "components/icons/AggregateIcon";
import { BinsIcon } from "components/icons/BinsIcon";
import { CastIcon } from "components/icons/CastIcon";
import { ComputeIcon } from "components/icons/ComputeIcon";
import { ConditionalIcon } from "components/icons/ConditionalIcon";
import { ConstantIcon } from "components/icons/ConstantIcon";
import { FileIcon } from "components/icons/FileIcon";
import { FilterIcon } from "components/icons/FilterIcon";
import { LeftBoomerangIcon } from "components/icons/LeftBoomerangicon";
import { LeftJoinIcon } from "components/icons/LeftJoinIcon";
import { RightBoomerangIcon } from "components/icons/RightBoomerangicon";
import { SelectIcon } from "components/icons/SelectIcon";
import { SortIcon } from "components/icons/SortIcon";
import { UnionIcon } from "components/icons/UnionIcon";
import { useStore } from "store";
import styles from "./Toolbar.module.css";
import { Zoom } from "./Zoom";

export function Toolbar() {
  const toggleNewNode = useStore(state => state.toggleNewNode);
  const newNode = useStore(state => state.newNode);
  const sidebarOpen = useStore(state => state.sidebarOpen);
  const toggleSidebar = useStore(state => state.toggleSidebar);

  return (
    <div className={styles.container}>
      <div className={styles.left_group}>
        <button
          className={styles.button}
          onClick={() => toggleSidebar()}
        >
          {sidebarOpen
            ? <LeftBoomerangIcon/>
            : <RightBoomerangIcon/>
          }
        </button>
      </div>

      <div className={styles.middle_group}>
        <button
          className={newNode === "load_csv" ? styles.button_active : styles.button}
          onClick={() => toggleNewNode("load_csv")}
        >
          <FileIcon/>
        </button>

        <button
          className={newNode === "select" ? styles.button_active : styles.button}
          onClick={() => toggleNewNode("select")}
        >
          <SelectIcon/>
        </button>

        <button
          className={newNode === "filter" ? styles.button_active : styles.button}
          onClick={() => toggleNewNode("filter")}
        >
          <FilterIcon/>
        </button>

        <button
          className={styles.button}
          onClick={() => toggleNewNode("cast")}
        >
          <CastIcon/>
        </button>

        <button
          className={styles.button}
          onClick={() => toggleNewNode("compute")}
        >
          <ComputeIcon/>
        </button>

        <button
          className={styles.button}
          onClick={() => toggleNewNode("case")}
        >
          <ConditionalIcon/>
        </button>

        <button
          className={newNode === "join" ? styles.button_active : styles.button}
          onClick={() => toggleNewNode("join")}
        >
          <LeftJoinIcon/>
        </button>

        <button
          className={newNode === "union" ? styles.button_active : styles.button}
          onClick={() => toggleNewNode("union")}
        >
          <UnionIcon/>
        </button>

        <button
          className={newNode === "aggregate" ? styles.button_active : styles.button}
          onClick={() => toggleNewNode("aggregate")}
        >
          <AggregateIcon/>
        </button>

        <button
          className={newNode === "sort" ? styles.button_active : styles.button}
          onClick={() => toggleNewNode("sort")}
        >
          <SortIcon/>
        </button>
      </div>

      <div className={styles.right_group}>
        <Zoom/>
      </div>
    </div>
  )
}