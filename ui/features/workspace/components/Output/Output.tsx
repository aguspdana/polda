import { useAppSelector } from 'app/hooks';
import style from './Output.module.css';
import { outputHeight } from '../../state/selectors';


export function Output() {
  const height = useAppSelector(outputHeight);

  return (
    <div
      className={style.output}
      style={{height: `${height}px`}}
    >
      <div className={style['controls']}>
        <div className={style['group__left']}>
          <button className={style['tab__button--active']}>Table</button>
          <button className={style['tab__button']}>Summary</button>
        </div>

        <div className={style['group__right']}>
          <button>X</button>
        </div>
      </div>
    </div>
  );
}