interface WebSocketEventMap {
  'close': CloseEvent;
  'error': Event;
  'message': MessageEvent;
  'open': Event;
}

interface Options {
  maxBackoff?: number;
  reconnectOnClose?: boolean;
  clearQueueOnClose?: boolean;
}

const defaultOptions: Required<Options> = {
  maxBackoff: 1000,
  reconnectOnClose: false,
  clearQueueOnClose: false,
};

export class WS {
  private backoff: number;

  private maxBackoff: number;

  // eslint-disable-next-line no-unused-vars
  private onCloseFn: ((this: WebSocket, event: CloseEvent) => void) | null = null;

  // eslint-disable-next-line no-unused-vars
  private onErrorFn: ((this: WebSocket, event: Event) => void) | null = null;

  // eslint-disable-next-line no-unused-vars
  private onMessageFn: ((this: WebSocket, event: MessageEvent) => void) | null = null;

  // eslint-disable-next-line no-unused-vars
  private onOpenFn: ((this: WebSocket, event: Event) => void) | null = null;

  private closeListeners: ((this: WebSocket, event: CloseEvent) => void)[] = []

  private errorListeners: ((this: WebSocket, event: Event) => void)[] = []

  private messageListeners: ((this: WebSocket, event: MessageEvent) => void)[] = []

  private openListeners: ((this: WebSocket, event: Event) => void)[] = []

  private queue: (string | ArrayBufferLike | Blob | ArrayBufferView)[] = [];

  private reconnectOnClose: boolean;

  private clearQueueOnClose: boolean;

  private url: string;

  private ws: WebSocket | null = null;

  // eslint-disable-next-line no-unused-vars
  constructor(url: string, opt?: Options) {
    this.backoff = 0;
    this.maxBackoff = typeof opt?.maxBackoff === 'number' ? opt.maxBackoff : defaultOptions.maxBackoff;
    this.reconnectOnClose = typeof opt?.reconnectOnClose === 'boolean'
      ? opt.reconnectOnClose
      : defaultOptions.reconnectOnClose;
    this.clearQueueOnClose = typeof opt?.clearQueueOnClose === 'boolean'
      ? opt.clearQueueOnClose
      : defaultOptions.clearQueueOnClose;
    this.url = url;

    this.connect();
  }

  addEventListener<T extends keyof WebSocketEventMap>(
    type: T,
    listener: (this: WebSocket, event: WebSocketEventMap[T]) => void,
  ) {
    switch (type) {
      case 'open': {
        const _listener = listener as (this: WebSocket, event: WebSocketEventMap["open"]) => void;
        this.openListeners.push(_listener);
        this.ws?.addEventListener('close', _listener);
        break;
      }
      case 'message': {
        const _listener = listener as (this: WebSocket, event: WebSocketEventMap["message"]) => void;
        this.messageListeners.push(_listener);
        this.ws?.addEventListener('message', _listener);
        break;
      }
      case 'error': {
        const _listener = listener as (this: WebSocket, event: WebSocketEventMap["error"]) => void;
        this.errorListeners.push(_listener);
        this.ws?.addEventListener('error', _listener);
        break;
      }
      case 'close': {
        const _listener = listener as (this: WebSocket, event: WebSocketEventMap["close"]) => void;
        this.closeListeners.push(_listener);
        this.ws?.addEventListener('close', _listener);
        break;
      }
      default:
    }
  }

  close() {
    this.ws?.close();
    this.ws = null;
  }

  private connect() {
    this.ws = new WebSocket(this.url);
    this.ws.onclose = this.handleCloseEvent.bind(this);
    this.ws.onerror = this.handleErrorEvent.bind(this);
    this.ws.onmessage = this.handleMessageEvent.bind(this);
    this.ws.onopen = this.handleOpenEvent.bind(this);
    for (const listener of this.closeListeners) {
      this.ws.addEventListener('close', listener);
    }
    for (const listener of this.errorListeners) {
      this.ws.addEventListener('error', listener);
    }
    for (const listener of this.messageListeners) {
      this.ws.addEventListener('message', listener);
    }
    for (const listener of this.openListeners) {
      this.ws.addEventListener('open', listener);
    }
  }

  private handleCloseEvent(event: CloseEvent) {
    if (this.ws && this.onCloseFn) {
      const fn = this.onCloseFn.bind(this.ws, event);
      fn();
    }

    if (this.clearQueueOnClose) {
      this.queue = [];
    }

    if (this.reconnectOnClose) {
      const duration = this.backoff;
      this.backoff = calcNextBackoff(this.backoff, this.maxBackoff);

      setTimeout(() => {
        // Reconnect if it hasn't been closed manually.
        if (this.ws && [WebSocket.CONNECTING, WebSocket.OPEN].includes(this.ws.readyState)) {
          this.connect();
        }
      }, duration);
    }
  }

  private handleErrorEvent(event: Event) {
    if (this.ws && this.onErrorFn) {
      const fn = this.onErrorFn.bind(this.ws, event);
      fn();
    }

    if (this.clearQueueOnClose) {
      this.queue = [];
    }

    if (this.reconnectOnClose) {
      const duration = this.backoff;
      this.backoff = calcNextBackoff(this.backoff, this.maxBackoff);

      setTimeout(() => {
        // Reconnect if it hasn't been closed manually.
        if (this.ws && [WebSocket.CONNECTING, WebSocket.OPEN].includes(this.ws.readyState)) {
          this.connect();
        }
      }, duration);
    }
  }

  private handleMessageEvent(event: MessageEvent) {
    if (this.ws && this.onMessageFn) {
      const fn = this.onMessageFn.bind(this.ws, event);
      fn();
    }
  }

  private handleOpenEvent(event: Event) {
    // Reset backoff duration.
    this.backoff = 0;

    if (this.ws) {
      if (this.onOpenFn) {
        const fn = this.onOpenFn.bind(this.ws, event);
        fn();
      }

      this.queue.forEach((item) => {
        this.ws?.send(item);
      });
      this.queue = [];
    }
  }

  // eslint-disable-next-line no-unused-vars
  set onclose(listener: ((this: WebSocket, event: CloseEvent) => void) | null) {
    this.onCloseFn = listener;
  }

  // eslint-disable-next-line no-unused-vars
  set onerror(listener: ((this: WebSocket, event: Event) => void) | null) {
    this.onErrorFn = listener;
  }

  // eslint-disable-next-line no-unused-vars
  set onmessage(listener: ((this: WebSocket, event: MessageEvent) => void) | null) {
    this.onMessageFn = listener;
  }

  // eslint-disable-next-line no-unused-vars
  set onopen(listener: ((this: WebSocket, event: Event) => void) | null) {
    this.onOpenFn = listener;
  }

  get readyState() {
    return this.ws !== null ? this.ws.readyState : 3;
  }

  removeEventListener<T extends keyof WebSocketEventMap>(
    type: T,
    listener: (this: WebSocket, event: WebSocketEventMap[T]) => void,
  ) {
    this.ws?.removeEventListener(type, listener);
    switch (type) {
      case "close":
        this.closeListeners = this.closeListeners.filter(f => f !== listener);
        break;
      case "error":
        this.closeListeners = this.errorListeners.filter(f => f !== listener);
        break;
      case "close":
        this.messageListeners = this.messageListeners.filter(f => f !== listener);
        break;
      case "close":
        this.openListeners = this.openListeners.filter(f => f !== listener);
        break;
    }
  }

  /**
   * Send a message to the server.  Reopen the connection if it's closed.
   */
  send(data: string | ArrayBufferLike | Blob | ArrayBufferView) {
    if (this.ws && this.readyState === WebSocket.OPEN) {
      this.ws.send(data);
    } else if (this.readyState === WebSocket.CONNECTING) {
      this.queue.push(data);
    } else {
      this.connect();
      this.queue.push(data);
    }
  }
}

function calcNextBackoff(backoff: number, max: number): number {
  let nextBackoff = 500;
  if (backoff > 0) {
    nextBackoff = backoff + backoff * 0.1 * Math.random();
  }
  if (nextBackoff > max) {
    nextBackoff = max - max * 0.2 * Math.random();
  }
  return nextBackoff;
}