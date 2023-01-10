import { CheckIcon } from "components/icons/CheckIcon";
import { DropDownIcon } from "components/icons/DropDownIcon";
import { ReactNode, useEffect, useRef, useState } from "react";
import styles from "./NodeTitleSelect.module.css";

export interface INodeTitleOption<T extends string> {
  value: T,
  icon: ReactNode,
  display: string
}

interface Props<T extends string> {
  options: INodeTitleOption<T>[],
  selected: T,
  onSelect: (value: T) => void,
  roundTopLeft?: boolean,
  routnTopRight?: boolean,
  roundBottomLeft?: boolean,
  roundBottomRight?: boolean
}

export function NodeTitleSelect<T extends string>({ options, selected, onSelect }: Props<T>) {
  const [open, setOpen] = useState(false);
  const [display, setDisplay] = useState(options.find(o => o.value === selected));
  const container = useRef<HTMLDivElement | null>(null);

  useEffect(() => {
    const display = options.find(o => o.value === selected);
    setDisplay(display);
  }, [selected, options]);

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
  
  function selectOpt(value: T) {
    onSelect(value);
    setOpen(false);
  }

  return (
    <div
      className={open ? styles.container_open : styles.container}
      onMouseDown={(e) => e.stopPropagation()}
      onClick={(e) => e.stopPropagation()}
      ref={container}
    >
      <div
        className={styles.toggle}
        onClick={() => setOpen(v => !v)}
      >
        <div className={styles.display}>
          {display?.icon}
          {display?.display}
        </div>
        <DropDownIcon/>
      </div>

      {open && (
        <div className={styles.drop_down}>
          <div className={styles.drop_down_container}>
            {options.map(opt => (
              <div
                key={opt.value}
                className={opt.value === selected ? styles.option_selected : styles.option}
                onClick={() => selectOpt(opt.value)}
              >
                {opt.value === selected && (
                  <div className={styles.selected_icon}>
                    <CheckIcon/>
                  </div>
                )}
                <div className={styles.opt_icon}>
                  {opt.icon}
                </div>
                <div className={styles.opt_text}>
                  {opt.display}
                </div>
              </div>
            ))}
          </div>
        </div>
      )}
    </div>
  );
}