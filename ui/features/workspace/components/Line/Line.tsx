import { Position } from 'features/workspace/interfaces';
import styles from './Line.module.css';

export function Line({ from, to }: { from: Position, to: Position }) {
  const dx = to.x - from.x;
  const dy = to.y - from.y;

  const normalWidth = dx >= 32;
  const padding = 128;

  const width = normalWidth ? dx : Math.abs(dx) + padding * 2;
  const height = Math.abs(dy) + 4;

  const left = normalWidth ? from.x : Math.min(from.x, to.x) - padding;
  const top = Math.min(from.y, to.y) - 2;

  const outX = normalWidth
    ? 0
    : from.x > to.x
      ? padding - dx
      : padding;
  const outY = from.y < to.y ? 2 : 2 - dy;

  const inDx = normalWidth ? dx : dx;
  const inDy = dy;

  const curve = dx > 0
    ? Math.max(dx / 2, 32)
    : Math.max(Math.pow(-dx, 0.7), 32);

  return (
    <svg
      className={styles['line']}
      style={{ top, left, width, height }}
      xmlns="http://www.w3.org/2000/svg"
      xmlnsXlink="http://www.w3.org/1999/xlink"
    >
      <path
        fill="none"
        stroke="var(--color-base-100)"
        d={`M ${outX},${outY} c ${curve},0 ${inDx - curve},${inDy} ${inDx},${inDy}`}
      />
    </svg>
  )
}