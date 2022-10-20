import { useAppSelector } from 'app/hooks';
import { sidebarWidth } from 'features/workspace/state/selectors';
import styles from './Sidebar.module.css';

export function Sidebar() {
  const width = useAppSelector(sidebarWidth);

  return (
    <div
      className={styles['sidebar__container']}
      style={{ width }}
    >
      <div>FILE</div>
    </div>
  );
}