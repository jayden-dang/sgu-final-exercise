import { RouteObject, createBrowserRouter } from "react-router-dom";
import Home from "../pages";
import Layout from "../components/Layout";
import ProductDeailPage from "@/pages/product";
import ShopManage from "@/pages/manage/shop";
import OwnedProductPage from "@/pages/user/OwnedProduct";

const routes: RouteObject[] = [
    {
        path: "/",
        element: <Layout />,
        children: [
            {
                path: "/",
                element: <Home />,
            },
            {
                path: "/product/:id",
                element: <ProductDeailPage />,
            },
            {
                path: "/manage/shop",
                element: <ShopManage />,
            },
            {
                path: "/user/owned-products",
                element: <OwnedProductPage />,
            },
        ],
    },
];
const AppRoutes = createBrowserRouter(routes);
export default AppRoutes;
