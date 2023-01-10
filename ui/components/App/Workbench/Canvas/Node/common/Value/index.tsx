import { EValue } from "lib/doc/node";
import { ChangeEvent } from "react";
import { Input } from "../Input";
import styles from "./Value.module.css";
import { ValueType } from "./ValueType";

interface Props {
  value: EValue,
  onChange: (value: EValue) => void,
  placeholder?: string,
  roundTopLeft?: boolean,
  roundTopRight?: boolean,
  roundBottomLeft?: boolean,
  roundBottomRight?: boolean
}

export function Value(
  {
    value,
    onChange,
    placeholder,
    roundTopLeft,
    roundTopRight,
    roundBottomLeft,
    roundBottomRight
  }: Props
) {
  function handleTypeSwitch() {
    onChange({
      ...value,
      type: value.type == "column" ? "constant" : "column"
    });
  }

  function handleValueChange(_value: string) {
    onChange({
      ...value,
      value: _value
    });
  }

  return (
    <div className={styles.container}>
      <ValueType
        type={value.type}
        onSwitch={handleTypeSwitch}
        roundTopLeft={roundTopLeft}
        roundBottomLeft={roundBottomLeft}
      />
      <Input
        value={value.value}
        onChange={handleValueChange}
        placeholder={placeholder}
        roundTopRight={roundTopRight}
        roundBottomRight={roundBottomRight}
      />
    </div>
  );
}