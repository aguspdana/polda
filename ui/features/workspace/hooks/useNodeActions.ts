import { useAppDispatch } from "app/hooks";
import { useEffect, useState } from "react";

export function useNodeActions(id: string) {
  const dispatch = useAppDispatch();
  const [movable, setMovable] = useState(false);

  useEffect(() => {
    function handleMouseMove() {}

    function handleMouseUp() {}

    if (movable) {
      window.addEventListener('mousemove', handleMouseMove);
      window.addEventListener('mouseup', handleMouseUp);
    }

    return () => {
      window.removeEventListener('mousemove', handleMouseMove);
      window.removeEventListener('mouseup', handleMouseUp);
    }
  }, [movable]);

  return {
    startMoving() {
      setMovable(true)
    }
  }
}