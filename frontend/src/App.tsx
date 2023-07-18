import { ChakraProvider } from "@chakra-ui/react";
import { RouterProvider } from "react-router-dom";
import AppRoutes from "./routes";
import React from "react";
import { useAppDispatch } from "./redux/hooks";
import { setUpWalletAsync } from "./redux/splice/nearWalletSplice";
import { AppWallet } from "./NearWallet";
function App() {
    const dispatch = useAppDispatch();
    React.useEffect(() => {
        dispatch(setUpWalletAsync());
    }, []);
    return (
        <ChakraProvider>
            <RouterProvider router={AppRoutes} />
        </ChakraProvider>
    );
}
export default App;
