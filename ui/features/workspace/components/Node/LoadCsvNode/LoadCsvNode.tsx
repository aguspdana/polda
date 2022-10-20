import { useAppDispatch, useAppSelector } from 'app/hooks';
import styles from '../Node.module.css';
import { LoadCsvNodeProps } from '../../../interfaces';
import {
  makeNodeMovable,
  selectOrUnselectNode
} from '../../../state/slice';
import { isNodeMoved, isNodeSelected } from '../../../state/selectors';
import { Port } from '../../Port';
import { CsvFileIcon } from '../../icons/CsvFileIcon';

export function LoadCsvNode({ id, props }: { id: string, props: LoadCsvNodeProps }) {
  const { location, position } = props;

  const dispatch = useAppDispatch();
  const isMoved = useAppSelector(isNodeMoved(id));
  const isSelected = useAppSelector(isNodeSelected(id));

  return (
    <div
      className={isSelected ? styles['node__container--selected'] : styles['node__container']}
      style={{top: `${position.y}px`, left: `${position.x}px`}}
    >
      <div
        className={styles['node__header']}
        onMouseDown={() => dispatch(makeNodeMovable(id))}
        onClick={(e) => { if (!isMoved) { dispatch(selectOrUnselectNode({ id, selectOne: !e.shiftKey })) } }}
      >
        <div className={styles['node__title']}>
          <CsvFileIcon color="var(--color-base-100)" />
          <h1 className={styles['node__title_text']}>
            Open
          </h1>
        </div>
      </div>

      <div className={styles['node__body']}>
        <div className={styles['node__row--sockets']}>
          <div className={styles['sockets--output']}>
            <div className={styles['socket--output']}>
              <span className={styles['socket__label']}>Table</span>
              <Port nodeId={id} index={0} type="output" />
            </div>
          </div>
        </div>

        <div className={styles['node__row--properties']}>
          <div className={styles['properties']}>
            <input
              className={styles['property--text']}
              type="text"
              placeholder="example.csv"
              value={location || ''}
            />
          </div>
        </div>
      </div>
    </div>
  );
}