import { useAppDispatch, useAppSelector } from 'app/hooks';
import {
  makeCanvasMovable,
  unselectAllNodes,
  addNode
} from '../../state/slice';
import {
  canvasPostion,
  index,
  nodes,
  canvasZoom,
  isCanvasMoved,
  newNodePosition
} from '../../state/selectors';
import { Node } from '../Node';
import style from './Canvas.module.css';
import { OpenConnection } from '../OpenConnection';
import { Connection } from '../Connection';
import { useEffect } from 'react';

export function Canvas() {
  const dispatch = useAppDispatch();
  const _nodes = useAppSelector(nodes);
  const _index = useAppSelector(index);
  const { x, y } = useAppSelector(canvasPostion);
  const zoom = useAppSelector(canvasZoom);
  const moved = useAppSelector(isCanvasMoved);
  const _newNodePosition = useAppSelector(newNodePosition);

  return (
    <div className={style['canvas__container']}>
      <div
        className={style['canvas']}
        style={{ left: x, top: y, transform: `scale(${zoom})`}}
      >
        {_index.map(id => (_nodes[id].input.map((_, i) => (
          <Connection key={`${id}_${i}`} nodeId={id} index={i} />
        ))))}

        {_index.map(id => (
          <Node key={id} id={id} />
        ))}

        <OpenConnection />
      </div>

      {_newNodePosition !== null
        ? (
          <div
            className={style['new-node__layer']}
            onClick={() => dispatch(addNode())}
          >
            <div
              className={style['new-node']}
              style={{ left: _newNodePosition.x, top: _newNodePosition.y, transform: `scale(${zoom})` }}
            />
          </div>
        )
        : (
          <div
            className={style['canvas__background']}
            onClick={() => { if (!moved) dispatch(unselectAllNodes()) }}
            onMouseDown={() => dispatch(makeCanvasMovable())}
          />
        )
      }
    </div>
  );
}