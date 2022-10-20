import { SelectNodeProps } from 'features/workspace/interfaces';
import { isNodeMoved, isNodeSelected } from 'features/workspace/state/selectors';
import DeleteIcon from '../../icons/DeleteIcon';
import PlusIcon from '../../icons/PlusIcon';
import SelectIcon from '../../icons/SelectIcon';
import styles from '../Node.module.css';
import { useAppDispatch, useAppSelector } from 'app/hooks';
import { makeNodeMovable, selectOrUnselectNode } from 'features/workspace/state/slice';
import { Port } from '../../Port';

export function SelectNode({ id, props }: { id: string, props: SelectNodeProps }) {
  const { position, columns } = props;

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
        onClick={(e) => { if (!isMoved) dispatch(selectOrUnselectNode({ id, selectOne: !e.shiftKey }))}}
      >
        <div className={styles['node__title']}>
          <SelectIcon color="var(--color-base-100)" />
          <h1 className={styles['node__title_text']}>Select</h1>
        </div>
      </div>

      <div className={styles['node__body']}>
        <div className={styles['node__row--sockets']}>
          <div className={styles['sockets--input']}>
            <div className={styles['socket--input']}>
              <Port nodeId={id} index={0} type="input" />
              <span className={styles['socket__label']}>Table</span>
            </div>
          </div>
          <div className={styles['sockets--output']}>
            <div className={styles['socket--output']}>
              <span className={styles['socket__label']}>Table</span>
              <Port nodeId={id} index={0} type="output" />
            </div>
          </div>
        </div>

        {columns.length === 1
          ? (
            <div className={styles['node__row--properties']}>
              <div className={styles['properties']}>
                <div className={styles['properties__column']}>
                  <input
                    className={styles['property--text']}
                    type="text"
                    placeholder="Column"
                    value={columns[0].name}
                  />
                  <input
                    className={styles['property--text']}
                    type="text"
                    placeholder="As (optional)"
                    value={columns[0].as}
                  />
                </div>
              </div>
            </div>
          )
          : columns.map(({ name, as }, i) => (
            <div key={i} className={styles['node__row--properties']}>
              <div className={styles['properties--deletable']}>
                <div className={styles['properties__column']}>
                  <input
                    className={styles['property--text']}
                    type="text"
                    placeholder="Column"
                    value={name}
                  />
                  <input
                    className={styles['property--text']}
                    type="text"
                    placeholder="As (optional)"
                    value={as}
                  />
                </div>
                <button className={styles['properties__delete_button']}>
                  <DeleteIcon />
                </button>
              </div>
            </div>
          ))
        }

        <div className={styles['node__row--properties']}>
          <button className={styles['properties__add_button']}>
            <PlusIcon color="var(--color-base-100)" />
          </button>
        </div>
      </div>
    </div>
  );
}
