import { useAppDispatch, useAppSelector } from 'app/hooks';
import { canvasZoom } from 'features/workspace/state/selectors';
import { useState } from 'react';
import { DropDownIcon } from '../icons/DropDownIcon';
import styles from './Zoom.module.css';

export function Zoom() {
  const dispatch = useAppDispatch();
  const zoom = useAppSelector(canvasZoom);
  const [showDropdown, setShowDropdown] = useState(false);

  return (
    <div className={styles['zoom__container']}>
      <button
        className={styles['zoom']}
        onClick={() => setShowDropdown(v => !v)}
      >
        {zoom * 100}%
        <DropDownIcon color="var(--color-base-400)" />
      </button>

      {showDropdown && (
        <div className={styles['dropdown']}>
          <div className={styles['dropdown__container']}>
            <div>50%</div>
            <div>100%</div>
            <div>200%</div>
          </div>
        </div>
      )}
    </div>
  );
}