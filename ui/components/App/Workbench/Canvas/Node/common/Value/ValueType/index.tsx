import { ColumnIcon } from "components/icons/ColumnIcon";
import { ConstantIcon } from "components/icons/ConstantIcon";
import { UpDownIcon } from "components/icons/UpDownIcon";
import { EValue } from "lib/doc/node";
import styles from "./ValueType.module.css";

interface Props {
  type: EValue["type"],
  onSwitch: () => void,
  roundTopLeft?: boolean,
  roundBottomLeft?: boolean,
}

export function ValueType(
  {
    type,
    onSwitch,
    roundTopLeft,
    roundBottomLeft
  }: Props
) {
  const borderRadius = roundTopLeft || roundBottomLeft
    ? `${roundTopLeft ? 4 : 0}px 0 0 ${roundBottomLeft ? 4 : 0}px`
    : undefined;

  return (
    <button
      className={styles.container}
      onClick={onSwitch}
      style={{ borderRadius }}
    >
      {
        type == "constant"
        ? <ConstantIcon/>
        : <ColumnIcon/>
      }

      <div className={styles.up_down}>
        <UpDownIcon/>
      </div>
    </button>
  );
}