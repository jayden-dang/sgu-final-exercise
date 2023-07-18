import { PayloadAction, createAsyncThunk, createSlice } from "@reduxjs/toolkit";
import { AppWallet } from "../../NearWallet";
import type { RootState } from "../store";

interface INearWalletState {
    isSignedIn: boolean;
    contractId: string;
    wallet: AppWallet;
    isLoggingIn: boolean;
    isSettedUp: boolean;
}

const initialState: INearWalletState = {
    isSignedIn: false,
    contractId: process.env.REACT_APP_CONTRACT_ID || "",
    wallet: new AppWallet({ createAccessKeyFor: process.env.REACT_APP_CONTRACT_ID }),
    isLoggingIn: false,
    isSettedUp: false,
};

export const nearWalletSplice = createSlice({
    name: "nearWallet",
    initialState,
    reducers: {
        setWallet: (state: INearWalletState, action: PayloadAction<AppWallet>) => {
            state.wallet = action.payload;
        },
        setSignedIn: (state: INearWalletState, action: PayloadAction<boolean>) => {
            state.isSignedIn = action.payload;
        },
        setContractId: (state: INearWalletState, action: PayloadAction<string>) => {
            state.contractId = action.payload;
        },
    },
    extraReducers: (builder) => {
        builder
            .addCase(setUpWalletAsync.fulfilled, (state, action) => {
                state.isSignedIn = action.payload;
                state.isLoggingIn = false;
                state.isSettedUp = true;
            })
            .addCase(setUpWalletAsync.rejected, (state, action) => {
                console.log(action.error);
                state.isLoggingIn = false;
            })
            .addCase(setUpWalletAsync.pending, (state, action) => {
                state.isLoggingIn = true;
            })
            .addCase(logOutAsync.fulfilled, (state, action) => {
                state.isSignedIn = false;
                state.isLoggingIn = false;
            })
            .addCase(logOutAsync.rejected, (state, action) => {
                console.log(action.error);
                state.isLoggingIn = false;
            })
            .addCase(logOutAsync.pending, (state, action) => {
                state.isLoggingIn = true;
            });
    },
});

export const setUpWalletAsync = createAsyncThunk("nearWallet/setUpWalletAsync", async (_, thunkAPI) => {
    const appState = thunkAPI.getState() as RootState;
    const wallet = appState.nearWallet.wallet;
    const isSignedIn = await wallet.startUp();
    console.log("isSignedIn", isSignedIn);
    return isSignedIn;
});

export const logOutAsync = createAsyncThunk("nearWallet/signUpAsync", async (_, thunkAPI) => {
    const appState = thunkAPI.getState() as RootState;
    const wallet = appState.nearWallet.wallet;
    await wallet.signOut();
    thunkAPI.dispatch(setSignedIn(false));
});

export const selectWallet = (state: RootState) => state.nearWallet.wallet;
export const selectIsSignedIn = (state: RootState) => state.nearWallet.isSignedIn;
export const selectContractId = (state: RootState) => state.nearWallet.contractId;
export const selectIsLoggingIn = (state: RootState) => state.nearWallet.isLoggingIn;
export const selectAccountId = (state: RootState) => state.nearWallet.wallet.accountId;
export const selectProviderIcon = (state: RootState) => state.nearWallet.wallet.wallet?.metadata.iconUrl;
export const selectIsSettedUp = (state: RootState) => state.nearWallet.isSettedUp;
export const { setWallet, setSignedIn, setContractId } = nearWalletSplice.actions;

export default nearWalletSplice.reducer;
