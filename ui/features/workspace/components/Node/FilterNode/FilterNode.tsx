import { useAppDispatch, useAppSelector } from "app/hooks";
import { FilterNodeProps } from "features/workspace/interfaces";
import { isNodeMoved, isNodeSelected } from "features/workspace/state/selectors";
import { makeNodeMovable, selectOrUnselectNode } from "features/workspace/state/slice";
import DeleteIcon from "../../icons/DeleteIcon";
import FilterIcon from "../../icons/FilterIcon";
import PlusIcon from "../../icons/PlusIcon";
import { Port } from "../../Port";
import styles from '../Node.module.css';

export function FilterNode({ id, props }: { id: string, props: FilterNodeProps}) {
  const { position, filters } = props;

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
          <FilterIcon color="var(--color-base-100)" />
          <h1 className={styles['node__title_text']}>Filter</h1>
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

        {filters.length === 1
          ? (
            <div className={styles['node__row--properties']}>
              <div className={styles['properties']}>
                <div className={styles['properties__column']}>
                  <input
                    className={styles['property--text']}
                    type="text"
                    placeholder="Column"
                    value={filters[0].column}
                  />
                  <input
                    className={styles['property--text']}
                    type="text"
                    placeholder="Operator"
                    value={filters[0].operator}
                  />
                  <div className={styles['properties__column']}>
                    <input
                      className={styles['property--select']}
                      type="text"
                      placeholder="Operator"
                      value={filters[0].operator}
                    />
                    <input
                      className={styles['property--text']}
                      type="text"
                      placeholder="Operator"
                      value={filters[0].operator}
                    />
                  </div>
                </div>
              </div>
            </div>
          )
          : filters.map(({ column, operator, comparator }, i) => (
            <div key={i} className={styles['node__row--properties']}>
              <div className={styles['properties--deletable']}>
                <div className={styles['properties__column']}>
                  <input
                    className={styles['property--text']}
                    type="text"
                    placeholder="Column"
                    value={column}
                  />
                  <input
                    className={styles['property--text']}
                    type="text"
                    placeholder="Operator"
                    value={operator}
                  />
                  <div className={styles['properties__column']}>
                    <input
                      className={styles['property--select']}
                      type="text"
                      placeholder="Operator"
                      value={filters[0].operator}
                    />
                    <input
                      className={styles['property--text']}
                      type="text"
                      placeholder="Operator"
                      value={filters[0].operator}
                    />
                  </div>
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