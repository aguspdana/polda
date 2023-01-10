import { DeleteIcon } from "components/icons/DeleteIcon";
import { ReactNode } from "react";
import styles from "./InputGroup.module.css";

interface Props {
  handleDelete?: () => void,
  children?: ReactNode
}

export function InputGroup({ handleDelete, children }: Props) {
  return (
    <div className={styles.container}>
      <div className={styles.inputs}>
        {children}
      </div>
      {typeof handleDelete === "function" && (
        <button
          className={styles.delete_btn}
          onClick={handleDelete}
        >
          <DeleteIcon/>
        </button>
      )}
    </div>
  );
}