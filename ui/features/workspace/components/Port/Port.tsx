import { useAppDispatch, useAppSelector } from 'app/hooks';
import { isSocketConnected } from 'features/workspace/state/selectors';
import { Socket } from '../../interfaces';
import {
  connectSocket,
} from '../../state/slice';
import styles from './Port.module.css';

export function Port(socket: Socket) {
  const dispatch = useAppDispatch();
  const isConnected = useAppSelector(isSocketConnected(socket));

  return (
    <div className={styles.port_container}>
      <div
        className={isConnected ? styles.port_active : styles.port}
        onMouseDown={() => dispatch(connectSocket(socket))}
        onMouseUp={() => dispatch(connectSocket(socket))}
      >
        <div className={styles.flare}>
          <div className={styles.dot} />
        </div>
      </div>
    </div>
  )
}