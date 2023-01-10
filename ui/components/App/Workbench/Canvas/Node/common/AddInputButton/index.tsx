import { PlusIcon } from "components/icons/PlusIcon";
import styles from "./AddInputButton.module.css";

interface Props {
  onClick: () => void
}

export function AddInputButton({ onClick }: Props) {
  return (
    <button
      className={styles.btn}
      onClick={onClick}
    >
      <PlusIcon/>
    </button>
  );
}