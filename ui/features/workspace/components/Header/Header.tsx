import { useAppDispatch, useAppSelector } from 'app/hooks';
import AggregateIcon from 'features/workspace/components/icons/AggregateIcon';
import FilterIcon from 'features/workspace/components/icons/FilterIcon';
import LeftJoinIcon from 'features/workspace/components/icons/LeftJoinIcon';
import SelectIcon from 'features/workspace/components/icons/SelectIcon';
import SortIcon from 'features/workspace/components/icons/SortIcon';
import { canvasZoom } from 'features/workspace/state/selectors';
import { setNewNode } from 'features/workspace/state/slice';
import { CsvFileIcon } from '../icons/CsvFileIcon';
import { LeftBoomerangIcon } from '../icons/LeftBoomerangIcon';
import { UnionIcon } from '../icons/UnionIcon';
import { Zoom } from '../Zoom';
import style from './Header.module.css';

export function Header() {
  const dispatch = useAppDispatch();
  const zoom = useAppSelector(canvasZoom);

  return (
    <header className={style.container}>
      <div className={style['group__left']}>
        <button className={style.action}>
          <LeftBoomerangIcon color="var(--color-base-400)" />
        </button>
      </div>

      <div className={style['group__center']}>
        <button className={style.action}>
          <CsvFileIcon color="var(--color-base-400)" />
        </button>
        <button
          className={style.action}
          onClick={() => dispatch(setNewNode('select'))}
        >
          <SelectIcon color="var(--color-base-400)" />
        </button>
        <button className={style.action}>
          <FilterIcon color="var(--color-base-400)" />
        </button>
        <button className={style.action}>
          <LeftJoinIcon color="var(--color-base-400)" />
        </button>
        <button className={style.action}>
          <UnionIcon color="var(--color-base-400)" />
        </button>
        <button className={style.action}>
          <AggregateIcon color="var(--color-base-400)" />
        </button>
        <button className={style.action}>
          <SortIcon color="var(--color-base-400)" />
        </button>
      </div>

      <div className={style['group__right']}>
        <Zoom />
      </div>
    </header>
  )
}