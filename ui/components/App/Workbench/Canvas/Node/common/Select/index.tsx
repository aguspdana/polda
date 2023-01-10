import { CheckIcon } from "components/icons/CheckIcon";
import { DropDownIcon } from "components/icons/DropDownIcon";
import { useEffect, useRef, useState } from "react";
import styles from "./Select.module.css";

export interface IOptionSelect<T extends string> {
  display: string,
  value: T
}

interface Props<T extends string> {
  options: IOptionSelect<T>[],
  selected: T,
  onSelect: (value: T) => void,
  roundTopLeft?: boolean,
  roundTopRight?: boolean,
  roundBottomLeft?: boolean,
  roundBottomRight?: boolean
}

export function Select<T extends string>({
  options,
  selected,
  onSelect,
  roundTopLeft,
  roundTopRight,
  roundBottomLeft,
  roundBottomRight
}: Props<T>) {
  const [open, setOpen] = useState(false);
  const [display, setDisplay] = useState(options.find(o => o.value === selected)?.display);
  const container = useRef<HTMLDivElement | null>(null);

  useEffect(() => {
    const display = options.find(o => o.value === selected)?.display;
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
    setOpen(false)
  }

  const borderRadius = `${roundTopLeft ? 4 : 0}px ${roundTopRight ? 4 : 0}px ${roundBottomRight ? 4 : 0}px ${roundBottomLeft ? 4 : 0}px`;

  return (
    <div
      className={styles.container}
      ref={container}
    >
      <div
        className={styles.toggle}
        onClick={() => setOpen(v => !v)}
        style={{ borderRadius }}
      >
        <div className={styles.text}>
          {display}
        </div>
        <div className={styles.icon}>
          <DropDownIcon/>
        </div>
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
                  <div className={styles.icon}>
                    <CheckIcon/>
                  </div>
                )}
                <div className={styles.text}>
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