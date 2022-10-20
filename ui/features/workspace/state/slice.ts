import { createSlice, PayloadAction } from '@reduxjs/toolkit';
import { Socket, Position, Dimension } from '../interfaces';
import initialState from './initialState';


export const workspaceSlice = createSlice({
  name: 'workspace',
  initialState,
  reducers: {
    setPointer(state, action: PayloadAction<Position>) {
      const { x, y } = action.payload;

      const dx = x - state.pointer.x;
      const dy = y - state.pointer.y;

      if (state.movable?.isMovable) {
        switch (state.movable.type) {
          case 'canvas': {
            // Move canvas
            state.canvas.position.x += dx;
            state.canvas.position.y += dy;
            state.movable.isMoved = true;
            break;
          }
          case 'node': {
            // Move nodes.
            state.movable.nodes.forEach(id => {
              state.doc.nodes[id].position.x += (dx / state.canvas.zoom);
              state.doc.nodes[id].position.y += (dy / state.canvas.zoom);
            });
            state.movable.isMoved = true;
            break;
          }
          case 'vertical_resizer': {
            const dx = x - state.movable.from.x;
            const newWidth = state.movable.initial + dx;
            if (newWidth < 256) {
              state.sidebarWidth = 256;
            } else if (newWidth > 1024) {
              state.sidebarWidth = 1024;
            } else {
              state.sidebarWidth = newWidth;
            }
            state.movable.isMoved = true;
            break;
          }
          case 'horizontal_resizer': {
            const dy = y - state.movable.from.y;
            const newHeight = state.movable.initial - dy;
            // TODO: limit to the height of the window.
            if (newHeight > 32) {
              state.outputHeight = newHeight;
            } else {
              state.outputHeight = 33;
            }
            state.movable.isMoved = true;
          }
        }
      }

      state.pointer = { x, y };
    },

    setWindow(state, action: PayloadAction<Dimension>) {
      state.window = action.payload;
    },


    setNewNode(state, action: PayloadAction<'load_csv' | 'select' | 'filter' | null>) {
      state.newNode = action.payload;
    },
    
    addNode(state) {
      // TODO: Add a new node based on "newNode".
      // TODO: Calculate position properly when canvas's zoom is not 1.
      // TODO: Ensure the new id is not used.
      const id = Math.floor(Math.random() * Number.MAX_SAFE_INTEGER).toString(32);
      const canvasOffset = { x: state.sidebarWidth, y: 49 };
      const placeholderSize = { height: 256, width: 256 };
      state.doc.nodes[id] = {
        type: 'select',
        position: {
          x: (state.pointer.x - canvasOffset.x - state.canvas.position.x) / state.canvas.zoom - placeholderSize.width / 2,
          y: (state.pointer.y - canvasOffset.y - state.canvas.position.y) / state.canvas.zoom - placeholderSize.height / 2
        },
        input: [null],
        output: [],
        columns: [
          {
            name: '',
            as: ''
          }
        ],
      };
      state.doc.index.push(id);
      state.newNode = null;
    },

    /**
     * Select one node or toggle node selection.
     */
    selectOrUnselectNode(state, action: PayloadAction<{ id: string, selectOne: boolean }>) {
      const { id, selectOne } = action.payload;
      const isSelected = state.selectedNodes[id];
      if (selectOne) {
        state.selectedNodes = { [id]: true };
        state.doc.index = state.doc.index.filter((i) => i !== id);
        state.doc.index.push(id);
      } else {
        if (!isSelected) {
          state.selectedNodes[id] = true;
          state.doc.index = state.doc.index.filter((i) => i !== id);
          state.doc.index.push(id);
        } else {
          delete state.selectedNodes[id];
        }
      }
    },

    selectAllNodes(state) {
      Object.keys(state.doc.nodes).forEach((id) => {
        state.selectedNodes[id] = true;
      });
    },

    unselectAllNodes(state) {
      state.selectedNodes = {};
    },

    makeCanvasMovable(state) {
      state.movable = {
        type: 'canvas',
        isMovable: true,
        isMoved: false
      };
    },

    zoomCanvas(state, action: PayloadAction<number>) {
      state.canvas.zoom = action.payload;
    },

    /**
     * Set the node as movable. If it's selected, set all selected nodes movable.
     */
    makeNodeMovable(state, action: PayloadAction<string>) {
      const id = action.payload;
      const isSelected = typeof state.selectedNodes[id] !== 'undefined';
      state.movable = {
        type: 'node',
        nodes: isSelected ? Object.keys(state.selectedNodes) : [id],
        isMovable: true,
        isMoved: false
      };
    },

    makeVerticalResizerMovable(state) {
      state.movable = {
        type: 'vertical_resizer',
        from: state.pointer,
        initial: state.sidebarWidth,
        isMovable: true,
        isMoved: false
      }
    },

    makeHorizontalResizerMovable(state) {
      state.movable = {
        type: 'horizontal_resizer',
        from: state.pointer,
        initial: state.outputHeight,
        isMovable: true,
        isMoved: false
      }
    },

    /**
     * Reset all movable objects (canvas, nodes, and open connection).
     */
    anchorMovableObject(state) {
      if (state.movable?.isMovable) {
        state.movable.isMovable = false;
      }
      state.openConnection = null;
    },

    connectSocket(state, action: PayloadAction<Socket>) {
      if (state.openConnection === null) {
        const { nodeId, type, index } = action.payload;
        // Disconnect input socket if it's already connected.
        if (type === 'input') {
          const srcId = state.doc.nodes[nodeId].input[index]
          if (typeof srcId === 'string') {
            state.doc.nodes[nodeId].input[index] = null;
            const totalConnection = state.doc.nodes[nodeId].input.reduce(
              (t, i) => i === srcId ? t + 1 : t, 0
            );
            if (totalConnection === 0) {
              state.doc.nodes[srcId].output = state.doc.nodes[srcId].output.filter(i => i !== nodeId);
            }
          }
        }
        state.openConnection = action.payload;
        return;
      };

      const socket1 = state.openConnection;
      const socket2 = action.payload;

      // Can't connect sockets of the same type.
      if (socket1.type === socket2.type) return;

      const [from, to] = socket1.type === 'output'
        ? [socket1, socket2]
        : [socket2, socket1];

      // Ensure the new connection doesn't create a circular connection.
      if (from.nodeId === to.nodeId) return;
      let toBeTraced = new Set([from.nodeId]);
      let alreadyTraced: Set<string> = new Set();
      while (toBeTraced.size > 0) {
        const { value: id } = toBeTraced.values().next();
        console.log('id =', id, '; node =', state.doc.nodes[id])
        for (const i of state.doc.nodes[id].input) {
          // It's a circular connection.
          if (i === to.nodeId) return;
          if (i) {
            if (!alreadyTraced.has(i)) {
              toBeTraced.add(i);
            }
          }
        }
        toBeTraced.delete(id);
      }

      // If the input socket's already connected, disconnect it first.
      const existingSrcId = state.doc.nodes[to.nodeId].input[to.index];
      if (existingSrcId) {
        const totalConnections = state.doc.nodes[to.nodeId].input
          .reduce((t, i) => i === existingSrcId ? t + 1 : t, 0);
        if (totalConnections <= 1) {
          state.doc.nodes[existingSrcId].output = state.doc.nodes[existingSrcId].output
            .filter(i => i !== to.nodeId);
        }
      }

      if (!state.doc.nodes[from.nodeId].output.includes(to.nodeId)) {
        state.doc.nodes[from.nodeId].output.push(to.nodeId);
      }
      state.doc.nodes[to.nodeId].input[to.index] = from.nodeId;
    },

    disconnectSocket(state, action: PayloadAction<Socket>) {
      const { nodeId, index, type } = action.payload;

      if (type === 'input') {
        state.doc.nodes[nodeId].input[index] = null;
      } else {
        state.doc.nodes[nodeId].output = [];
      }
    }
  }
});

export const {
    setPointer,
    setWindow,
    setNewNode,
    addNode,
    selectOrUnselectNode,
    selectAllNodes,
    unselectAllNodes,
    zoomCanvas,
    makeCanvasMovable,
    makeNodeMovable,
    makeVerticalResizerMovable,
    makeHorizontalResizerMovable,
    anchorMovableObject,
    connectSocket,
    disconnectSocket,
} = workspaceSlice.actions;

export const reducer = workspaceSlice.reducer;