import create from "zustand";
import { Doc, executeOperations } from 'lib/doc';
import { mergeBatch, EOperation, rebase } from "lib/doc/operation";
import { defaultNode, getInputs, ENodeType, ENode, disconnectNodeOps, EFilterPredicate } from "lib/doc/node";

interface IMovableObject {
  type: string,
  cursorFromX: number,
  cursorFromY: number,
  moved: boolean,
  finished: boolean
}

interface IMovableCanvas extends IMovableObject {
  type: "canvas",
  fromX: number,
  fromY: number
}

interface IMovableNodes extends IMovableObject {
  type: "nodes",
  nodes: {
    id: string,
    fromX: number,
    fromY: number
  }[]
}

interface IMovableOutputResizer extends IMovableObject {
  type: "output",
  fromHeight: number,
  wasMaximized: boolean
}

interface IMovableSidebarResizer extends IMovableObject {
  type: "sidebar",
  fromWidth: number
}

type EMovable =
  | IMovableCanvas
  | IMovableNodes
  | IMovableOutputResizer
  | IMovableSidebarResizer;

export interface ISocket {
  id: string,
  type: "primary" | "secondary" | "output",
}

// Outgoing messages

interface IOpenDocMessage {
  type: "open_doc",
  id: number,
  path: string
}

interface IUpdateDocMessage {
  type: "update_doc",
  id: number,
  version: number,
  operations: EOperation[]
}

interface ICloseDocMessage {
  type: "close_doc",
  id: number
}

interface IGetDocMessage {
  type: "get_doc",
  id: number
}

interface IQueryNodeMessage {
  type: "query",
  id: number,
  node_id: string
}

interface IReadFileMessage {
  type: "read_file",
  id: number,
  filename: string
}

interface ICancelQueryMessage {
  type: "cancel_job",
  id: number
}

export type EOutgoingMessage =
  | IOpenDocMessage
  | IUpdateDocMessage
  | ICloseDocMessage
  | IGetDocMessage
  | IQueryNodeMessage
  | IReadFileMessage
  | ICancelQueryMessage;

// Incoming messages

interface IClientIdMessage {
  type: "client_id",
  client_id: string
}

interface ISourcesMessage {
  type: "sources",
  sources: string[]
}

interface IDocMessage {
  type: "doc",
  id: number,
  doc: Doc,
  version: number
}

interface IDocClosedMessage {
  type: "doc_closed",
  id: number
}

interface IRemoteUpdateDocMessage {
  type: "update_doc",
  id: number | null,
  version: number,
  operations: EOperation[]
}

interface IQueryResultMessage {
  type: "query_result",
  id: number,
  data: IDataFrame
}

interface IFileDataMessage {
  type: "file_data",
  id: number
  data: IDataFrame
}

interface IJobCanceledMessage {
  type: "job_canceled",
  id: number
}

interface IErrorMessage {
  type: "error",
  id: number | null,
  code: EErrorCode,
  msg: string
}

export type EIncomingMessage =
  | IClientIdMessage
  | ISourcesMessage
  | IDocMessage
  | IDocClosedMessage
  | IRemoteUpdateDocMessage
  | IQueryResultMessage
  | IFileDataMessage
  | IJobCanceledMessage
  | IErrorMessage;

export interface IDataFrame {
  columns: {
    name: string,
    datatype: string, // TODO: Replace this with a concrete data type.
    values: any[]
  }[]
}

interface INodeQuery {
  id: number,
  type: "node",
  nodeId: string,
  data: IDataFrame | null,
  error: string | null
}

interface IFileQuery {
  id: number,
  type: "file",
  filename: string,
  data: IDataFrame | null,
  error: string | null
}

type EQuery = INodeQuery | IFileQuery;

type EErrorCode =
  "PARSE_ERROR"
  | "INVALID_REQUEST"
  | "METHOD_NOT_FOUND"
  | "INVALID_PARAMS"
  | "INTERNAL_ERROR";

export interface IToast {
  id: number,
  msg: string
}

interface IState {
  clientId: string | null, // null means the client is not connected.
  send: (msg: EOutgoingMessage) => void,
  msgCounter: number,

  sources: string[],

  // Document
  path: string | null,
  doc: Doc | null,
  docError: string | null,
  version: number,

  // Cursor
  cursorX: number,
  cursorY: number,

  // Window
  windowHeight: number,
  windowWidth: number,

  // Canvas
  canvasX: number,
  canvasY: number,
  canvasZoom: number,

  // Output
  outputHeight: number,
  outputMaximized: boolean,

  // Sidebar
  sidebarWidth: number,
  sidebarOpen: boolean,

  // Files
  analysisFiles: string[],
  dataFiles: string[],

  query: EQuery | null,
  selectedNodes: { [id: string]: boolean }, // Values are ignored.

  // React to cursor position
  movable: EMovable | null,
  newNode: ENodeType | null,
  openSocket: ISocket | null,

  // Operations
  sentOps: EOperation[],
  pendingOps: EOperation[],
  undoSent: EOperation[],
  undoPending: EOperation[],

  // Toast
  toasts: IToast[],

  // Web Socket
  resetClientId: () => void,
  setSender: (fn: IState["send"]) => void,
  handleMessage: (msg: EIncomingMessage) => void,

  // Document
  setPath: (path: string) => void,

  // Cursor
  setCursorPosition: (x: number, y: number) => void,

  // Movable
  moveCanvas: () => void,
  resizeSidebar: () => void,
  resizeOutput: () => void,
  moveNode: (id: string) => void,
  finishMovable: () => void,

  // Window
  setWindowDimension: (width: number, height: number) => void,

  // Sidebar
  toggleSidebar: () => void,

  // Output
  maximizeOutput: () => void,
  minimizeOutput: () => void,
  closeOutput: () => void,

  // Canvas
  zoomIn: () => void,
  zoomOut: () => void,
  zoomToFit: () => void,
  resetZoom: () => void,

  // Connection
  resetOpenSocket: () => void,
  connectSocket: (socket: ISocket) => void,
  disconnectSocket: (socket: ISocket) => void,

  // Selection
  toggleNodeSelection: (id: string) => void,
  unselectAllNodes: () => void,

  // Node
  toggleNewNode: (type: ENodeType) => void,
  insertNode: () => void,
  deleteSelectedNodes: () => void,
  queryNode: (id: string) => void,
  readFile: (filename: string) => void,

  // Operations
  executeOperations: (ops: EOperation[]) => void,

  // Toast
  pushToast: (msg: string) => void,
  removeToast: (id: number) => void
}

export const useStore = create<IState>((set): IState => {
  return {
    clientId: null,
    send: () => {},
    msgCounter: 0,

    sources: [],

    // Document
    path: null,
    doc: null,
    docError: null,
    version: 0,

    // Cursor
    cursorX: 0,
    cursorY: 0,

    // Window
    windowHeight: 800,
    windowWidth: 800,

    // Canvas
    canvasX: 0,
    canvasY: 0,
    canvasZoom: 1,

    // Output
    outputHeight: 256,
    outputMaximized: false,

    // Sidebar
    sidebarWidth: 256,
    sidebarOpen: true,

    // Files
    analysisFiles: [],
    dataFiles: [],

    query: null,

    // Selected nodes
    selectedNodes: {},

    // React to cursor position
    movable: null,
    newNode: null,
    openSocket: null,

    // Operations
    sentOps: [],
    pendingOps: [],
    undoSent: [],
    undoPending: [],

    // Toast
    toasts: [],

    // Web Socket

    resetClientId() {
      set(_ => ({ clientId: null }));
    },

    setSender(fn) {
      set(_ => ({ send: fn }));
    },

    handleMessage(msg) {
      let patch: Partial<IState> = {};
      set(state => {
        switch (msg.type) {
          case "client_id":
            patch.clientId = msg.client_id;
            if (typeof state.path === "string") {
              patch.doc = null;
              patch.msgCounter = state.msgCounter + 1;
              state.send({
                type: "open_doc",
                id: patch.msgCounter,
                path: state.path,
              });
            }
            break;
          
          case "sources":
            patch.sources = msg
              .sources
              .sort((a, b) => {
                if (a < b) return -1;
                if (a > b) return 1;
                return 0
              });
            break;
          
          case "doc":
            patch.doc = msg.doc;
            patch.version = msg.version;
            patch.sentOps = [];
            patch.pendingOps = [];
            patch.undoPending = [];
            patch.undoSent = [];
            if (state.doc === null) {
              patch = {
                ...patch,
                ...zoomToFit(
                  Object.values(msg.doc.nodes),
                  state.windowWidth,
                  state.windowHeight,
                  state.sidebarOpen ? state.sidebarWidth + 1 : 0,
                  state.query !== null ? state.outputHeight + 1 : 0,
                  0.05,
                  1
                )
              };
            }
            break;

          case "update_doc":
            patch.version = msg.version;
            if (typeof msg.id === "number") {
              // Local update.
              if (state.pendingOps.length > 0) {
                // Send pending operations.
                patch.msgCounter = state.msgCounter + 1;
                const outMsg: EOutgoingMessage = {
                  type: "update_doc",
                  id: patch.msgCounter,
                  version: msg.version,
                  operations: state.pendingOps
                };
                state.send(outMsg);
                patch.sentOps = state.pendingOps;
                patch.undoSent = state.undoPending;
                patch.pendingOps = [];
                patch.undoPending = [];
              } else {
                patch.sentOps = [];
                patch.undoSent = [];
              }
              break;
            }

            if (state.doc === null) {
              break;
            }

            // Remote update.
            try {
              // Undo sent and pending operations from end to start.
              const undoOps: EOperation[] = [];
              for (const undo of [state.pendingOps, state.sentOps]) {
                for (let i = undo.length - 1; i >= 0; i--) {
                  undoOps.push(undo[i]);
                }
              }
              const { doc: undoDoc } = executeOperations(state.doc, undoOps);

              // Execute received operations.
              const { doc: recDoc } = executeOperations(undoDoc, msg.operations);

              // Rebase sent and pending operations.
              const rebasedOps = rebase(
                [...state.sentOps, ...state.pendingOps],
                msg.operations
              );
              const rebasedSentOps = rebasedOps
                .slice(0, state.sentOps.length)
                .filter(op => op !== null) as EOperation[];
              const rebasedPendingOps = rebasedOps
                .slice(state.sentOps.length)
                .filter(op => op !== null) as EOperation[];
              const { doc: sentDoc, ops: _rebasedSentOps, undo: undoSent } = executeOperations(
                recDoc,
                rebasedSentOps
              );
              const { doc: pendingDoc, ops: _rebasedPendingOps, undo: undoPending } = executeOperations(
                sentDoc,
                rebasedPendingOps
              );

              patch.doc = pendingDoc;
              patch.sentOps = _rebasedSentOps;
              patch.pendingOps = _rebasedPendingOps;
              patch.undoSent = undoSent;
              patch.undoPending = undoPending;
            } catch (e) {
              console.error(e);
              // Document is unsyncable: Request doc.
              if (typeof state.clientId === "string" && typeof state.path === "string") {
                patch.msgCounter = state.msgCounter + 1;
                const msg: EOutgoingMessage = {
                  type: "get_doc",
                  id: patch.msgCounter
                };
                state.send(msg);
              }
            }
            break;

          case "query_result":
            if (state.query?.id === msg.id && state.query.type === "node") {
              patch.query = {
                ...state.query,
                data: msg.data
              };
            }
            break;

          case "file_data":
            if (state.query?.id === msg.id && state.query.type === "file") {
              patch.query = {
                ...state.query,
                data: msg.data
              };
            }
            break;

          case "job_canceled":
            if (state.query?.id === msg.id) {
              patch.query = null;
            }
            break;
          
          case "error":
            if (state.query?.id === msg.id) {
              patch.query = {
                ...state.query,
                error: msg.msg
              };
            }
            break;

          default:
            console.error("Received an unknown message", msg);
        }
        
        return patch;
      });
    },

    // Document

    setPath(path) {
      set(state => {
        const patch: Partial<IState> = {
          path,
          docError: null,
        };
        if (path !== state.path) {
          patch.doc = null;
          if (
            typeof state.clientId === "string"
            && typeof path === "string"
          ) {
            patch.msgCounter = state.msgCounter + 1;
            state.send({
              type: "open_doc",
              id: patch.msgCounter,
              path,
            });
          }
        }
        return patch;
      });
    },

    // Cursor

    setCursorPosition(x, y) {
      set(state => {
        let patch: Partial<IState> = {
          cursorX: x,
          cursorY: y
        };

        if (state.movable !== null && !state.movable.finished) {
          const dx = x - state.movable.cursorFromX;
          const dy = y - state.movable.cursorFromY;

          switch (state.movable.type) {
            case "canvas":
              patch.canvasX = state.movable.fromX + dx;
              patch.canvasY = state.movable.fromY + dy;
              patch.movable = {
                ...state.movable,
                moved: true
              };
              break;
            
            case "nodes":
              if (state.movable.moved || Math.abs(dx) > 2 || Math.abs(dy) > 2) {
                const dxZoomed = dx / state.canvasZoom;
                const dyZoomed = dy / state.canvasZoom;
                const ops: EOperation[] = state.movable.nodes.map(({ id, fromX, fromY }) => ({
                  type: "set_position",
                  id,
                  position: {
                    x: fromX + dxZoomed,
                    y: fromY + dyZoomed
                  }
                }));
                patch = {
                  ...patch,
                  ...executeOperationsOnState(state, ops),
                  movable: {
                    ...state.movable,
                    moved: true
                  }
                };
              }
              break;
            
            case "output":
              let height = state.movable.fromHeight - dy;
              if (height < 0) {
                height = 0;
              } else if (height > state.windowHeight) {
                height = state.windowHeight;
              }
              patch.outputHeight = height;
              if (dy > 0) {
                patch.movable = {
                  ...state.movable,
                  moved: true
                };
              }
              break;
            
            case "sidebar":
              const SIDEBAR_MAX_WIDTH = 320;
              let width = state.movable.fromWidth + dx;
              if (width < 0) {
                width = 0;
              } else if (width > SIDEBAR_MAX_WIDTH) {
                width = SIDEBAR_MAX_WIDTH;
              }
              patch.sidebarWidth = width;
              if (dx > 0) {
                patch.movable = {
                  ...state.movable,
                  moved: true
                };
              }
              break;
          }
        }
        return patch;
      });
    },

    // Movable

    moveCanvas() {
      set(state => ({
        movable: {
          type: "canvas",
          fromX: state.canvasX,
          fromY: state.canvasY,
          cursorFromX: state.cursorX,
          cursorFromY: state.cursorY,
          moved: false,
          finished: false
        }
      }));
    },

    resizeSidebar() {
      set(state => ({
        movable: {
          type: "sidebar",
          fromWidth: state.sidebarWidth,
          cursorFromX: state.cursorX,
          cursorFromY: state.cursorY,
          moved: false,
          finished: false
        }
      }));
    },

    resizeOutput() {
      set(state => {
        const patch: Partial<IState> = {};
        patch.movable = {
          type: "output",
          fromHeight: state.outputHeight,
          wasMaximized: state.outputMaximized,
          cursorFromX: state.cursorX,
          cursorFromY: state.cursorY,
          moved: false,
          finished: false
        };
        if (state.outputMaximized) {
          patch.outputHeight = state.windowHeight;
          patch.movable.cursorFromY =
            state.windowHeight
            - patch.movable.fromHeight
            - patch.movable.cursorFromY;
        }
        return patch;
      });
    },

    moveNode(id) {
      set(state => {
        if (state.doc === null) {
          return {};
        }
        const isSelected = typeof state.selectedNodes[id] === "boolean";
        const ids = isSelected ? Object.keys(state.selectedNodes) : [id];
        const nodes: IMovableNodes["nodes"] = []
        for (const id of ids) {
          const node = state.doc.nodes[id];
          nodes.push({
            id,
            fromX: node.position.x,
            fromY: node.position.y
          });
        }
        const patch: Partial<IState> = {
          movable: {
            type: "nodes",
            nodes,
            cursorFromX: state.cursorX,
            cursorFromY: state.cursorY,
            moved: false,
            finished: false
          }
        };
        return patch;
      });
    },

    finishMovable() {
      set(state => {
        const patch: Partial<IState> = {};
        if (state.movable === null || state.movable.finished) {
          return patch;
        }
        patch.movable = {
          ...state.movable,
          finished: true
        };
        if (state.movable.type === "output") {
          if (state.outputHeight > state.windowHeight - 128) {
            patch.outputHeight = state.movable.fromHeight;
            patch.outputMaximized = true;
          } else if (state.outputHeight > state.windowHeight - 256) {
            patch.outputHeight = state.windowHeight - 256;
            patch.outputMaximized = false;
          } else if (state.outputHeight < 128) {
            patch.outputHeight = state.movable.fromHeight;
            patch.query = null;
          } else if (state.outputHeight < 256) {
            patch.outputHeight = 256;
            patch.outputMaximized = false;
          } else {
            patch.outputMaximized = false;
          }
        } else if (state.movable.type === "sidebar") {
          if (state.sidebarWidth < 128) {
            patch.sidebarOpen = false;
            patch.sidebarWidth = state.movable.fromWidth;
          } else if (state.sidebarWidth < 256) {
            patch.sidebarWidth = 256;
          }
        }

        return patch;
      });
    },

    // Window

    setWindowDimension(width, height) {
      set(state => {
        const patch: Partial<IState> = {};
        patch.windowHeight = height;
        patch.windowWidth = width;
        patch.outputHeight = state.outputHeight;

        if (height < 512) {
          patch.outputHeight = height / 2;
          patch.outputMaximized = true;
        } else if (state.outputHeight > state.windowHeight - 128) {
          patch.outputHeight = height - 256;
          patch.outputMaximized = true;
        } else if (state.outputHeight > state.windowHeight - 256) {
          patch.outputHeight = height - 256;
        } else if (state.outputHeight < 128) {
          patch.outputHeight = height - 256;
          patch.query = null;
        } else if (state.outputHeight < 256) {
          patch.outputHeight = 256;
        }

        if (state.sidebarWidth > width) {
          patch.sidebarWidth = width;
        }

        return patch;
      });
    },

    // Sidebar

    toggleSidebar() {
      set(state => ({
        sidebarOpen: !state.sidebarOpen
      }));
    },

    // Output

    maximizeOutput() {
      set(state => ({
        outputMaximized: true
      }))
    },

    minimizeOutput() {
      set(state => ({
        outputMaximized: false
      }))
    },

    closeOutput() {
      set(state => {
        if (state.query !== null && state.query.data === null) {
          // Cancel prev query.
          const msg: EOutgoingMessage = {
            type: "cancel_job",
            id: state.query.id
          };
          state.send(msg);
        }
        return {
          query: null
        };
      })
    },

    // Canvas

    zoomIn() {
      set(state => {
        let newZoom = state.canvasZoom;
        // 0.1, 0.25, 0.5, 0.75, 1, 1.25, 1.5, 2
        if (state.canvasZoom < 0.1) {
          newZoom = 0.1;
        } else if (state.canvasZoom < 0.25) {
          newZoom = 0.25;
        } else if (state.canvasZoom < 0.5) {
          newZoom = 0.5;
        } else if (state.canvasZoom < 0.75) {
          newZoom = 0.75;
        } else if (state.canvasZoom < 1) {
          newZoom = 1;
        } else if (state.canvasZoom < 1.25) {
          newZoom = 1.25;
        } else if (state.canvasZoom < 1.5) {
          newZoom = 1.5;
        } else if (state.canvasZoom < 1.75) {
          newZoom = 1.75;
        } else if (state.canvasZoom < 2) {
          newZoom = 2;
        }
        const { x, y } = centerCanvasOnZoom(
          state.canvasX,
          state.canvasY,
          state.canvasZoom,
          newZoom,
          state.windowWidth,
          state.windowHeight,
          state.sidebarOpen ? state.sidebarWidth + 1 : 0,
          state.query ? state.outputHeight + 1 : 0
        );
        return {
          canvasX: x,
          canvasY: y,
          canvasZoom: newZoom
        };
      })
    },

    zoomOut() {
      set(state => {
        let newZoom = state.canvasZoom;
        // 0.1, 0.25, 0.5, 0.75, 1, 1.25, 1.5, 2
        if (state.canvasZoom > 2) {
          newZoom = 2;
        } else if (state.canvasZoom > 1.75) {
          newZoom = 1.75;
        } else if (state.canvasZoom > 1.5) {
          newZoom = 1.5;
        } else if (state.canvasZoom > 1.25) {
          newZoom = 1.25;
        } else if (state.canvasZoom > 1) {
          newZoom = 1;
        } else if (state.canvasZoom > 0.75) {
          newZoom = 0.75;
        } else if (state.canvasZoom > 0.5) {
          newZoom = 0.5;
        } else if (state.canvasZoom > 0.25) {
          newZoom = 0.25;
        } else if (state.canvasZoom > 0.1) {
          newZoom = 0.1;
        }
        const { x, y } = centerCanvasOnZoom(
          state.canvasX,
          state.canvasY,
          state.canvasZoom,
          newZoom,
          state.windowWidth,
          state.windowHeight,
          state.sidebarOpen ? state.sidebarWidth + 1 : 0,
          state.query ? state.outputHeight + 1 : 0
        );
        return {
          canvasX: x,
          canvasY: y,
          canvasZoom: newZoom
        };
      })
    },

    zoomToFit() {
      set(state => {
        if (state.doc === null) {
          return {};
        }
        return zoomToFit(
          Object.values(state.doc.nodes),
          state.windowWidth,
          state.windowHeight,
          state.sidebarOpen ? state.sidebarWidth + 1 : 0,
          state.query ? state.outputHeight + 1 : 0,
          0.05,
          2
        );
      });
    },

    resetZoom() {
      set(state => {
        const newZoom = 1;
        const { x, y } = centerCanvasOnZoom(
          state.canvasX,
          state.canvasY,
          state.canvasZoom,
          newZoom,
          state.windowWidth,
          state.windowHeight,
          state.sidebarOpen ? state.sidebarWidth + 1 : 0,
          state.query ? state.outputHeight + 1 : 0
        );
        return {
          canvasX: x,
          canvasY: y,
          canvasZoom: newZoom
        };
      })
    },

    // Connection

    resetOpenSocket() {
      set(_ => ({
        openSocket: null
      }))
    },

    connectSocket(socket) {
      set(state => {
        if (state.openSocket === null) {
          state.openSocket = socket;
          return {};
        }
        let op: EOperation | null = null;
        if (socket.type === "output") {
          if (state.openSocket.type === "primary") {
            op = {
              type: "set_input",
              id: state.openSocket.id,
              name: "primary",
              input: socket.id
            };
          } else if (state.openSocket.type === "secondary") {
            op = {
              type: "set_input",
              id: state.openSocket.id,
              name: "secondary",
              input: socket.id
            };
          }
        } else if (state.openSocket.type === "output") {
          if (socket.type === "primary") {
            op = {
              type: "set_input",
              id: socket.id,
              name: "primary",
              input: state.openSocket.id
            };
          } else if (socket.type === "secondary") {
            op = {
              type: "set_input",
              id: socket.id,
              name: "secondary",
              input: state.openSocket.id
            };
          }
        }
        let patch: Partial<IState> = {
          openSocket: null
        };
        if (op !== null) {
          patch = {
            ...patch,
            ...executeOperationsOnState(state, [op])
          };
        }
        return patch;
      });
    },

    disconnectSocket(socket) {
      set(state => {
        if (socket.type === "output") {
          throw new Error("Can't disconnect output socket");
        }
        if (state.doc === null) {
          return {};
        }
        const node = state.doc.nodes[socket.id];
        const isPrimary = socket.type === "primary";
        const input = isPrimary ? getInputs(node)[0] : getInputs(node)[1];
        const name = isPrimary ? "primary" : "secondary";
        const op: EOperation = {
          type: "set_input",
          id: socket.id,
          name,
          input: null
        };
        const patch = executeOperationsOnState(state, [op]);
        if (typeof input === "string") {
          patch.openSocket = {
            id: socket.id,
            type: socket.type
          };
        }
        return patch;
      });
    },

    // Selection

    toggleNodeSelection(id) {
      set(state => {
        const patch: Partial<IState> = {};
        if (typeof state.selectedNodes[id] === "boolean") {
          patch.selectedNodes = {...state.selectedNodes};
          delete patch.selectedNodes[id];
        } else {
          patch.selectedNodes = {
            ...state.selectedNodes,
            [id]: true
          };
        }
        return patch;
      });
    },

    unselectAllNodes() {
      set(_ => ({ selectedNodes: {} }));
    },

    // Node

    toggleNewNode(type) {
      set(state => ({
        newNode: state.newNode !== type ? type : null
      }));
    },

    insertNode() {
      set(state => {
        if (state.doc === null) {
          return {};
        }
        if (typeof state.newNode === "string") {
          const node = defaultNode(state.newNode);
          const { x, y } = selectCursorOnCanvas(state);
          node.position.x = x - 128;   // Substractor must be equal to the ghost node transformX.
          node.position.y = y - 24;    // Substractor must be equal to the ghost node transformY.
          while (true) {
            node.id = `${state.clientId}${(Math.random() * Number.MAX_SAFE_INTEGER).toString(36).substring(0, 4)}`;
            if (state.doc.nodes[node.id] === undefined) {
              break;
            }
          }
          const op1: EOperation = {
            type: "insert_node",
            node
          };
          const op2: EOperation = {
            type: "insert_index",
            index: state.doc.index.length,
            id: node.id
          };
          const patch = executeOperationsOnState(state, [op1, op2]);
          patch.newNode = null;
          return patch;
        }
        return {};
      });
    },

    deleteSelectedNodes() {
      set(state => {
        if (state.doc === null) {
          return {};
        }
        const ops: EOperation[] = [];
        for (const id in state.selectedNodes) {
          // IMPROVE: disconnectNodeOps may produce duplicate operations
          // across selected nodes.
          disconnectNodeOps(state.doc.nodes, id)
            .forEach(op => ops.push(op));
        }
        // Delete node from the end of the index.
        const selected = new Set(Object.keys(state.selectedNodes));
        for (let i = state.doc.index.length - 1; i >= 0; i--) {
          const id = state.doc.index[i];
          if (selected.has(id)) {
            selected.delete(id);
            ops.push({
              type: "delete_index",
              id,
              index: i
            });
            ops.push({
              type: "delete_node",
              id
            });
          }
        }
        const patch = executeOperationsOnState(state, ops);
        patch.selectedNodes = {};
        if (
          state.query?.type === "node"
          && typeof state.selectedNodes[state.query.nodeId] === "boolean"
        ) {
          patch.query = null;
        }
        return patch;
      });
    },

    queryNode(id) {
      set(state => {
        const patch: Partial<IState> = {};

        if (state.query !== null && state.query.data === null) {
          // Cancel prev query.
          const msg: EOutgoingMessage = {
            type: "cancel_job",
            id: state.query.id
          };
          state.send(msg);
          patch.query = null;
        }

        if (typeof state.doc?.nodes[id] !== "undefined") {
          patch.msgCounter = state.msgCounter + 1;
          patch.query = {
            id: patch.msgCounter,
            type: "node",
            nodeId: id,
            data: null,
            error: null
          };
          const msg: EOutgoingMessage = {
            type: "query",
            id: patch.msgCounter,
            node_id: id
          };
          state.send(msg);
        }
        return patch;
      });
    },

    readFile(filename) {
      set(state => {
        const patch: Partial<IState> = {};

        if (state.query !== null && state.query.data === null) {
          // Cancel prev query.
          const msg: EOutgoingMessage = {
            type: "cancel_job",
            id: state.query.id
          };
          state.send(msg);
          patch.query = null;
        }

        patch.msgCounter = state.msgCounter + 1;
        patch.query = {
          id: patch.msgCounter,
          type: "file",
          filename: filename,
          data: null,
          error: null
        };
        const msg: EOutgoingMessage = {
          type: "read_file",
          id: patch.msgCounter,
          filename: filename
        };
        state.send(msg);
        return patch;
      });
    },

    // Operations
    executeOperations(ops) {
      set(state => executeOperationsOnState(state, ops));
    },

    // Toast

    pushToast(msg) {
      const toast = {
        id: Math.floor(Math.random() * Number.MAX_SAFE_INTEGER),
        msg
      };
      set(state => ({
        toasts: [...state.toasts, toast]
      }));
    },

    removeToast(id) {
      set(state => ({
        toasts: state.toasts.filter(t => t.id !== id)
      }));
    }
  };
});

/**
 * Execute operations and return the state patch.
 * If sentOperations is empty, send pendingOperations.
 */
function executeOperationsOnState(state: IState, ops: EOperation[]): Partial<IState> {
  const patch: Partial<IState> = {};
  
  if (state.doc !== null) {
    try {
      const { doc, ops: _ops, undo } = executeOperations(state.doc, ops);
      patch.doc = doc;
      const pendingOps = mergeBatch(state.pendingOps, _ops);
      const lastOpsReplaced = pendingOps.length === state.pendingOps.length;
      const undoPending = lastOpsReplaced
        ? state.undoPending
        : [...state.undoPending, ...undo];

      if (state.sentOps.length === 0) {
        patch.msgCounter = state.msgCounter + 1;
        patch.sentOps = pendingOps;
        patch.pendingOps = [];
        patch.undoSent = undoPending;
        patch.undoPending = [];
        state.send({
          type: "update_doc",
          id: patch.msgCounter,
          version: state.version,
          operations: pendingOps
        });
      } else {
        patch.pendingOps = pendingOps;
        patch.undoPending = undoPending;
      }
    } catch (e) {
      console.error(e);
    }
  }

  return patch;
}

/**
 * Return the canvas offset relative to the container.
 */
function centerCanvasOnZoom(
  fromX: number,
  fromY: number,
  oldZoom: number,
  newZoom: number,
  windowWidth: number,
  windowHeight: number,
  sidebarWidth: number,
  outputHeight: number
) {
  // X: [ZOOM] The position of the center point of the viewport on canvas.
  // L: [REAL] The position of the canvas relative to the container.
  // W: [REAL] The dimension of the view.
  // Z: Zoom
  // X1 = (W/2 - L1) / Z1
  // X2 = (W/2 - L2) / Z2
  // X2 = X1
  // L2 = (L1 - W/2) / Z1 * Z2 + W/2
  const TOOLBAR_HEIGHT = 49;
  const halfCanvasWidth = (windowWidth - sidebarWidth) / 2;
  const halfCanvasHeight = (windowHeight - TOOLBAR_HEIGHT - outputHeight) / 2;
  const zoomFraction = newZoom / oldZoom;
  const x = (fromX - halfCanvasWidth) * zoomFraction + halfCanvasWidth;
  const y = (fromY - halfCanvasHeight) * zoomFraction + halfCanvasHeight;
  return { x, y };
}

function zoomToFit(
  nodes: ENode[],
  windowWidth: number,
  windowHeight: number,
  sidebarWidth: number,
  outputHeight: number,
  minZoom: number,
  maxZoom: number,
): {
  canvasZoom: number,
  canvasX: number,
  canvasY: number
} {
  if(nodes.length === 0) {
    return {
      canvasZoom: 1,
      canvasX: 0,
      canvasY: 0
    };
  }

  let topMost = Number.MAX_VALUE;
  let bottomMost = -Number.MAX_VALUE;
  let leftMost = Number.MAX_VALUE;
  let rightMost = -Number.MAX_VALUE;

  for (const node of nodes) {
    const { x, y } = node.position;
    const height = calc_node_height(node);
    const bottomY = y + height;
    if (x < leftMost) {
      leftMost = x;
    }
    if (x > rightMost) {
      rightMost = x;
    }
    if (y < topMost) {
      topMost = y;
    }
    if (bottomY > bottomMost) {
      bottomMost = bottomY;
    }
  }

  const TOOLBAR_HEIGHT = 49;
  const physicalCanvasWidth =
    sidebarWidth === 0
    ? windowWidth
    : Math.max(windowWidth - sidebarWidth + 1, 256);
  const physicalCanvasHeight = outputHeight === 0
    ? Math.max(windowHeight - TOOLBAR_HEIGHT, 256)
    : Math.max(windowHeight - TOOLBAR_HEIGHT - outputHeight - 1, 256);
  const NODE_WIDTH = 256;
  let filledCanvasWidth = rightMost - leftMost + NODE_WIDTH;
  let filledCanvasHeight = bottomMost - topMost;
  const horizontalCenter = leftMost + filledCanvasWidth / 2;
  const verticalCenter = topMost + filledCanvasHeight / 2;
  const horizontalZoom = physicalCanvasWidth / filledCanvasWidth;
  const verticalZoom = physicalCanvasHeight / filledCanvasHeight;
  const newZoom = Math.max(
    Math.min(
      Math.min(horizontalZoom, verticalZoom),
      maxZoom
    ),
    minZoom
  );
  return {
    canvasZoom: newZoom,
    canvasX: physicalCanvasWidth  / 2 - horizontalCenter * newZoom,
    canvasY: physicalCanvasHeight / 2 - verticalCenter * newZoom
  };
}

export function calc_node_height(node: ENode): number {
  const header = 48;
  const socket = 32;
  const input = 32;
  const gap = 8;
  const small_gap = 1;
  const bottom_padding = 16;
  const plus_btn = 24;

  switch (node.type) {
    case "aggregate": {
      return header
        + socket
        + gap * 2
        + (input * 3 + small_gap * 2 + gap) * node.aggregates.length
        + plus_btn
        + bottom_padding;
    }

    case "case": {
      return header
        + socket
        + input * 3
        + gap * 5
        + (input * 2 + small_gap + gap) * node.cases.length
        + plus_btn
        + bottom_padding;
    }

    case "cast": {
      return header
        + socket
        + input * 3
        + gap * 4
        + bottom_padding;
    }

    case "compute": {
      return header
        + socket
        + input * 4
        + gap * 4
        + bottom_padding;
    }

    case "filter": {
      let withParam = !(["is_null", "is_not_null"] as EFilterPredicate["type"][])
        .includes(node.predicate.type);
      return header
        + socket
        + input * (withParam ? 3 : 2)
        + gap * 4
        + bottom_padding;
    }

    case "join": {
      return header
        + socket * 2
        + plus_btn
        + gap * 2
        + (input * 2 + small_gap + gap) * node.columns.length
        + bottom_padding;
    }

    case "load_csv": {
      return header
        + socket
        + input
        + gap * 2
        + bottom_padding;
    }

    case "select": {
      return header
        + socket
        + plus_btn
        + gap * 2
        + (input * 2 + small_gap + gap) * node.columns.length
        + bottom_padding;
    }

    case "sort": {
      return header
        + socket
        + plus_btn
        + gap * 2
        + (input * 2 + small_gap + gap) * node.sorters.length
        + bottom_padding;
    }

    case "union": {
      return header
        + socket * 2
        + gap
        + bottom_padding;
    }
  }
}

export function selectCursorOnCanvas(state: IState): { x: number, y: number } {
  const OFFSET_LEFT = state.sidebarOpen ? state.sidebarWidth : 0;
  const OFFSET_TOP = 49;
  const x = (state.cursorX - OFFSET_LEFT - state.canvasX) / state.canvasZoom;
  const y = (state.cursorY - OFFSET_TOP - state.canvasY) / state.canvasZoom;
  return { x, y };
}

export function selectConnections(state: IState): { from: ISocket, to: ISocket }[] {
  if (state.doc === null) {
    return [];
  }
  const connections: { from: ISocket, to: ISocket }[] = [];
  for (const id of state.doc.index) {
    const node = state.doc.nodes[id];
    const inputs = getInputs(node);
    if (typeof inputs[0] === "string") {
      connections.push({
        from: {
          id: inputs[0],
          type: "output"
        },
        to: {
          id,
          type: "primary"
        }
      });
    }
    if (typeof inputs[1] === "string") {
      connections.push({
        from: {
          id: inputs[1],
          type: "output"
        },
        to: {
          id,
          type: "secondary"
        }
      });
    }
  }
  return connections;
}

export function selectSocketPosition(socket: ISocket) {
  return function (state: IState): { x: number, y: number } {
    if (state.doc === null) {
      return { x: 0, y: 0 };
    }
    const OUTPUT_OFFSET_X = 256;
    const OUTPUT_OFFSET_Y = 72;
    const PRIMARY_OFFSET_Y = 72;
    const SECONDARY_OFFSET_Y = 104;

    let { x, y } = state.doc.nodes[socket.id].position;
    switch (socket.type) {
      case "primary":
        y += PRIMARY_OFFSET_Y;
        break;
      case "secondary":
        y += SECONDARY_OFFSET_Y;
        break;
      case "output":
        x += OUTPUT_OFFSET_X;
        y += OUTPUT_OFFSET_Y;
    }
    return { x, y };
  }
}

export function selectQueriedNode(state: IState): ENode | undefined {
  if (!state.query) {
    return undefined;
  }

  if (state.query.type !== "node") {
    return undefined;
  }

  return state.doc?.nodes[state.query.nodeId];
}