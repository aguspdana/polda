import { DropDownIcon } from "components/icons/DropDownIcon";
import { PlusIcon } from "components/icons/PlusIcon";
import { useState } from "react";
import { useStore } from "store";
import styles from "./Section.module.css";

interface Props {
  title: string,
  items: string[],
  selected?: string,
  select?: (item: string) => void
}

export function Section({ title, items, select, selected }: Props) {
  const [open, setOpen] = useState(true);
  const pushToast = useStore(state => state.pushToast);

  function unimplemented() {
    pushToast("This feature is not implemented yet.");
  }

  return (
    <div className={styles.container}>
      <div className={styles.header}>
        <button
          className={styles.toggle_btn}
          onClick={() => setOpen(o => !o)}
        >
          <div className={open ? styles.toggle_icon : styles.toggle_icon_collapsed}>
            <DropDownIcon/>
          </div>
          <div className={styles.title}>
            {title}
          </div>
        </button>
        <div className={styles.new_btn_wrapper}>
          <button
            className={styles.new_btn}
            onClick={unimplemented}
          >
            <PlusIcon/>
          </button>
        </div>
      </div>

      {open && (
        <div className={styles.items}>
          {items.map(item => (
            <button
              key={item}
              className={item === selected ? styles.item_active : styles.item}
              onClick={() => { if (typeof select === "function") select(item) }}
            >
              {item}
            </button>
          ))}
        </div>
      )}
    </div>
  );
}