import { ChangeEventHandler } from "react";
import styles from "./Input.module.css";

interface Props {
  value: string,
  placeholder?: string,
  onChange?: (value: string) => void,
  roundTopLeft?: boolean,
  roundTopRight?: boolean,
  roundBottomRight?: boolean,
  roundBottomLeft?: boolean,
}
export function Input({
  value,
  placeholder,
  onChange,
  roundTopLeft,
  roundTopRight,
  roundBottomRight,
  roundBottomLeft
}: Props) {
  const handleChange: ChangeEventHandler<HTMLInputElement> = (e) => {
    if (typeof onChange === "function") {
      onChange(e.target.value);
    }
  };

  const borderRadius = `${roundTopLeft ? 4 : 0}px ${roundTopRight ? 4 : 0}px ${roundBottomRight ? 4 : 0}px ${roundBottomLeft ? 4 : 0}px`;

  return (
    <input
      className={styles.input}
      type="text"
      value={value}
      placeholder={placeholder}
      onChange={handleChange}
      onKeyDown={e => e.stopPropagation()}
      style={{
        borderRadius
      }}
    />
  );
}