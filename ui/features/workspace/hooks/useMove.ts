import { useEffect, useRef, useState } from 'react';

type MoveCallback = (dx: number, dy: number) => void;

/**
 * Call `moveFrom` to start tracking the mouse movement.
 * It stops tracking on window's mouse up event.
 * `moved` is `true` if the mouse moves after `moveFrom`
 * is called.
 */
export function useMove() {
  const [readyToMove, setReadyToMove] = useState(false);
  const mousePos = useRef({ x: 0, y: 0 });
  const _move = useRef<MoveCallback>(() => {});
  const moved = useRef(false);

  function moveFrom(
    x: number,
    y: number,
    move: MoveCallback
  ) {
    setReadyToMove(true);
    mousePos.current = { x, y };
    _move.current = move;
    moved.current = false;
  }

  useEffect(() => {
    function handleMouseMove(e: globalThis.MouseEvent) {
      const dx = e.clientX - mousePos.current.x;
      const dy = e.clientY - mousePos.current.y;
      _move.current(dx, dy);
      moved.current = true;
      mousePos.current = { x: e.clientX, y: e.clientY };
    }

    function handleMouseUp(e: globalThis.MouseEvent) {
      setReadyToMove(false);
    }

    if (readyToMove) {
      window.addEventListener('mousemove', handleMouseMove);
      window.addEventListener('mouseup', handleMouseUp)
    } else {
      window.removeEventListener('mousemove', handleMouseMove);
      window.removeEventListener('mouseup', handleMouseUp);
    }

    return function () {
      window.removeEventListener('mousemove', handleMouseMove)
      window.removeEventListener('mouseup', handleMouseUp)
    };
  }, [readyToMove, _move]);

  return {
    moveFrom,
    moved: moved.current
  };
}