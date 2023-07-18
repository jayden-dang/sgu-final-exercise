import { TypedUseSelectorHook, useDispatch, useSelector } from "react-redux";
import type { RootState, AppDispatch } from "./store";

// eslint-disable-next-line @typescript-eslint/no-unsafe-assignment
export const useAppDispatch: () => AppDispatch = useDispatch;
// eslint-disable-next-line @typescript-eslint/no-unsafe-assignment
export const useAppSelector: TypedUseSelectorHook<RootState> = useSelector;
