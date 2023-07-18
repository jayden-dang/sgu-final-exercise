import { combineReducers, configureStore, PreloadedState } from "@reduxjs/toolkit";
import nearWalletReducer from "./splice/nearWalletSplice";
const rootReducer = combineReducers({
    nearWallet: nearWalletReducer,
});

export type RootState = ReturnType<typeof rootReducer>;

export const store = configureStore({
    reducer: rootReducer,
    middleware(getDefaultMiddleware) {
        return getDefaultMiddleware({
            serializableCheck: false,
        });
    },
});

export type AppDispatch = typeof store.dispatch;
