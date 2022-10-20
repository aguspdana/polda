import { configureStore } from '@reduxjs/toolkit';
import { reducer as workspaceReducer } from 'features/workspace'


const store = configureStore({
  reducer: {
    workspace: workspaceReducer
  },
  middleware: (getDefaultMiddleware) => getDefaultMiddleware({
    serializableCheck: {
      // Ignore these action types
      ignoredActions: [
      ],
      // Ignore these paths in the state
      ignoredPaths: [
      ],
    },
  }),
  devTools: true, // process.env.NODE_ENV !== 'production',
});

export default store;

// Infer the `RootState` and `AppDispatch` types from the store itself
export type RootState = ReturnType<typeof store.getState>
// Inferred type: {posts: PostsState, comments: CommentsState, users: UsersState}
export type AppDispatch = typeof store.dispatch