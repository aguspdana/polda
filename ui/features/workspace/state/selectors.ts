import { RootState } from "app/store";
import { Socket } from '../interfaces';

const FIRST_PORT_Y = 72;
const PORT_DISTANCE = 16
const NODE_WIDTH = 256;

export const pointer = (state: RootState) => state.workspace.pointer;

export const window = (state: RootState) => state.workspace.window;

export const nodes = (state: RootState) => state.workspace.doc.nodes;

export const index = (state: RootState) => state.workspace.doc.index;

export const canvasPostion = (state: RootState) => state.workspace.canvas.position;

export const canvasZoom = (state: RootState) => state.workspace.canvas.zoom;

export const isCanvasMovable = (state: RootState) => (
  state.workspace.movable !== null
  && state.workspace.movable.type === 'canvas'
  && state.workspace.movable.isMovable
);

export const isCanvasMoved = (state: RootState) => {
  const { movable } = state.workspace;
  return (
    movable !== null
    && movable.type === 'canvas'
    && movable.isMoved
  );
};

export const newNodePosition = (state: RootState) => {
  const { newNode, pointer, sidebarWidth } = state.workspace;
  if (typeof newNode !== 'string') return null;

  const canvasOffset = { x: sidebarWidth, y: 49 };
  const position = {
    x: pointer.x - canvasOffset.x - 128,
    y: pointer.y - canvasOffset.y - 128
  };

  if ((position.x < -126) || (position.y < -126)) return null

  return position;
};

export const selectedNodes = (state: RootState) => state.workspace.selectedNodes;

export const isNodeSelected = (id: string) => (
  (state: RootState) => typeof state.workspace.selectedNodes[id] !== 'undefined'
);

export const isNodeMoved = (id: string) => (
  (state: RootState) => {
    const { movable } = state.workspace;
    return (
      movable !== null
      && movable.type === 'node'
      && movable.nodes.includes(id)
      && movable.isMoved
    );
  }
)

export const isSocketConnected = (socket: Socket) => (
  (state: RootState) => {
    const node = state.workspace.doc.nodes[socket.nodeId];
    const { openConnection } = state.workspace;
    if (socket.type === 'output' && node.output.length > 0) {
      return true;
    }
    if (socket.type === 'input' && typeof node.input[socket.index] === 'string') {
      return true;
    }
    return (
      openConnection !== null
      && openConnection.nodeId === socket.nodeId
      && openConnection.type === socket.type
      && openConnection.index === socket.index
    );
  }
)

export const connectionEndpoints = (inputSocket: { nodeId: string, index: number }) => (
  (state: RootState) => {
    const { nodeId: toNodeId, index } = inputSocket;
    const nodes = state.workspace.doc.nodes;
    const fromNodeId = nodes[toNodeId].input[index];

    if (typeof fromNodeId !== 'string') return null;

    const fromNode = nodes[fromNodeId];
    const toNode = nodes[toNodeId];

    return {
      from: {
        x: fromNode.position.x + NODE_WIDTH,
        y: fromNode.position.y + FIRST_PORT_Y + PORT_DISTANCE * index
      },
      to: {
        x: toNode.position.x,
        y: toNode.position.y + FIRST_PORT_Y
      }
    };
  }
);

export const openConnectionEndpoints = (state: RootState) => {
  const { openConnection, pointer, canvas, sidebarWidth } = state.workspace;
  const canvasOffset = { x: sidebarWidth, y: 49 };

  if (openConnection === null) return null;

  const isFromOutput = openConnection.type === 'output';
  const pos = state.workspace.doc.nodes[openConnection.nodeId].position;

  const pos1 = {
    x: isFromOutput ? pos.x + NODE_WIDTH : pos.x,
    y: pos.y + FIRST_PORT_Y + PORT_DISTANCE * openConnection.index
  };
  const pos2 = {
    x: (pointer.x - canvas.position.x - canvasOffset.x) / canvas.zoom,
    y: (pointer.y - canvas.position.y - canvasOffset.y) / canvas.zoom,
  };

  return isFromOutput
    ? { from: pos1, to: pos2 }
    : { from: pos2, to: pos1 };
}; 

export const openConnection = (state: RootState) => state.workspace.openConnection;

export const sidebarWidth = (state: RootState) => state.workspace.sidebarWidth;

export const outputHeight = (state: RootState) => state.workspace.outputHeight;