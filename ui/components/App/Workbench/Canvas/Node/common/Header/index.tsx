import { PlayIcon } from "components/icons/PlayIcon";
import { MouseEvent, ReactNode } from "react";
import { useStore } from "store";
import styles from "./Header.module.css";

interface Props {
  id: string,
  children: ReactNode
}

export function Header({ id, children }: Props) {
  const move = useStore(state => state.moveNode);
  const isMoved = useStore(state => state.movable?.moved || false);
  const toggleSelection = useStore(state => state.toggleNodeSelection);
  const queryNode = useStore(state => state.queryNode);

  function handleClickOnContainer(e: MouseEvent<HTMLDivElement>) {
    if (!isMoved) {
      toggleSelection(id);
    }
  }

  function handlePlay(e: MouseEvent<HTMLButtonElement>) {
    queryNode(id);
    e.stopPropagation();
  } 

  return (
    <div
      className={styles.container}
      onMouseDown={() => move(id)}
      onClick={handleClickOnContainer}
    >
      <div className={styles.title}>
        { children }
      </div>
      <button
        className={styles.play_btn}
        onClick={handlePlay}
      >
        <PlayIcon/>
      </button>
    </div>
  );
}