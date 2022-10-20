import { WorkspaceState } from "../interfaces";


const initialState: WorkspaceState = {
  doc: {
    nodes: {
      'a': {
        type: 'load_csv',
        position: {
          x: 64,
          y: 64
        },
        input: [],
        output: ['c'],
        location: null
      },
      'b': {
        type: 'load_csv',
        position: {
          x: 64,
          y: 240 
        },
        input: [],
        output: [],
        location: null
      },
      'c': {
        type: 'select',
        position: {
          x: 64 + 32 + 256,
          y: 64
        },
        input: ['a'],
        output: [],
        columns: [
          {
            name: '',
            as: ''
          }
        ]
      },
      'd': {
        type: 'select',
        position: {
          x: 64 + 32 + 256,
          y: 312
        },
        input: ['a'],
        output: [],
        columns: [
          {
            name: '',
            as: ''
          },
          {
            name: '',
            as: ''
          }
        ]
      },
      'e': {
        type: 'filter',
        position: {
          x: 64 + 32 + 256 + 32 + 256,
          y: 64
        },
        input: [null],
        output: [],
        filters: [
          {
            column: '',
            operator: 'equal',
            comparator: {
              type: 'column',
              name: ''
            }
          }
        ]
      }
    },
    index: ['a', 'b', 'c', 'd', 'e'],
  },
  canvas: {
    position: {
      x: 0,
      y: 0
    },
    zoom: 1.0
  },
  pointer: { x: 0, y: 0 },
  window: { height: 256, width: 256 },
  newNode: null,
  selectedNodes: {},
  movable: null,
  openConnection: null,
  sidebarWidth: 256,
  outputHeight: 100
};

export default  initialState;