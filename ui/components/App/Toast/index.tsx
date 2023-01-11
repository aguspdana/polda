import { useEffect, useState } from "react";
import { IToast, useStore } from "store";
import styles from "./Toast.module.css";

const DURATION = 3000;
const TRANSITION_DURATION = 500;

interface Props {
  toast: IToast
}

export function Toast({ toast }: Props) {
  const removeToast = useStore(state => state.removeToast);
  const [state, setState] = useState<"before" | "on" | "after">("before");

  const container = state === "before"
  ? styles.container_before
  : state === "after"
    ? styles.container_after
    : styles.container;
  
  useEffect(() => {
    setState("on");
  }, []);

  useEffect(() => {
    const timeout = setTimeout(() => {
      setState("after");
    }, DURATION + TRANSITION_DURATION);

    return () => clearTimeout(timeout);
  }, []);

  useEffect(() => {
    let timeout = setTimeout(() => {
      removeToast(toast.id);
    }, DURATION + TRANSITION_DURATION * 2);

    return () => clearTimeout(timeout);
  }, [removeToast, toast.id]);

  return (
    <div className={container}>
      <div className={styles.wrapper}>
        {toast.msg}
      </div>
    </div>
  );
}