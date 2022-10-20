import { useAppDispatch } from 'app/hooks';
import { useEffect } from 'react';
import {
  anchorMovableObject,
  makeHorizontalResizerMovable,
  makeVerticalResizerMovable,
  setPointer,
  setWindow
} from '../../state/slice';
import { Canvas } from '../Canvas';
import { Header } from '../Header';
import { Output } from '../Output';
import { Sidebar } from '../Sidebar';
import style from './Workspace.module.css'

export function Workspace() {
  const dispatch = useAppDispatch();

  useEffect(() => {
    function handleMouseMove(e: MouseEvent) {
      dispatch(setPointer({ x: e.clientX, y: e.clientY }));
    }
    function handleMouseUp() {
      dispatch(anchorMovableObject());
    }
    function measureWindowDimension() {
      dispatch(setWindow({
        height: window.innerHeight,
        width: window.innerWidth
      }));
    }
    measureWindowDimension();
    window.addEventListener('mousemove', handleMouseMove);
    window.addEventListener('mouseup', handleMouseUp);
    window.addEventListener('resize', measureWindowDimension)

    return () => {
      window.removeEventListener('mousemove', handleMouseMove);
      window.removeEventListener('mouseup', handleMouseUp);
      window.removeEventListener('resize', measureWindowDimension)
    }
  }, [dispatch]);

  return (
    <div className={style.container}>
      <Sidebar />

      <div className={style['resizer__container--vertical']}>
        <div
          className={style['resizer--vertical']}
          onMouseDown={() => dispatch(makeVerticalResizerMovable())}
        />
      </div>

      <div className={style.main}>
        <div className={style.top}>
          <Header />
          <Canvas />
        </div>

        <div className={style['resizer__container--horizontal']}>
          <div
            className={style['resizer--horizontal']}
            onMouseDown={() => dispatch(makeHorizontalResizerMovable())}
          />
        </div>

        <Output />
      </div>
    </div>
  );
}