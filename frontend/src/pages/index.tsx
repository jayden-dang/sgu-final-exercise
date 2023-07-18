import { Container, Flex, Spinner } from "@chakra-ui/react";
import { AppWallet } from "../NearWallet";
import { useAppSelector } from "../redux/hooks";
import { selectContractId, selectIsSettedUp, selectIsSignedIn, selectWallet } from "../redux/splice/nearWalletSplice";
import ProductAddToCart from "@/components/ProductCard";
import { IProductPreview } from "@contractTypes/index";
import React from "react";
import Pagination from "@/components/Pagination";
const limit = 15;
export default function Home() {
    const wallet = useAppSelector(selectWallet);
    const contractId = useAppSelector(selectContractId);
    const isSettedUp = useAppSelector(selectIsSettedUp);
    const [products, setProducts] = React.useState<IProductPreview[]>([]);
    const [currentPage, setCurrentPage] = React.useState(1);
    const [totalPage, setTotalPage] = React.useState(2);
    const [isLoading, setIsLoading] = React.useState(false);
    const getProduct = async ({ page = 1 }: { page?: number } = {}) => {
        return wallet.viewMethod({
            contractId,
            method: "getProducts",
            args: { page, limit },
        }) as Promise<IProductPreview[]>;
    };
    React.useEffect(() => {
        if (isSettedUp) {
            setIsLoading(true);
            getProduct({
                page: currentPage,
            }).then((res) => {
                setProducts(res);
                window.scrollTo(0, 0);
                setIsLoading(false);
            });
        }
    }, [isSettedUp, currentPage]);
    React.useEffect(() => {
        if (isSettedUp) {
            wallet
                .viewMethod({
                    contractId,
                    method: "getTotalProducts",
                    args: {},
                })
                .then((res) => {
                    const totalPage = Math.ceil(res / limit);
                    setTotalPage(totalPage);
                });
        }
    }, [isSettedUp]);
    return (
        <Flex direction={"column"}>
            <Flex
                mt={"10"}
                wrap={"wrap"}
                justifyContent={"space-evenly"}
                rowGap={"20px"}
                columnGap={"20px"}
                direction={"row"}
            >
                {!isLoading && products.map((product) => <ProductAddToCart product={product} key={product.id} />)}
                {isLoading && (
                    <Spinner thickness="4px" speed="0.65s" emptyColor="gray.200" color="blue.500" size="xl" />
                )}
            </Flex>
            <Flex justifyContent={"center"}>
                <Pagination total={totalPage} onPageChange={setCurrentPage} currentPage={currentPage} />
            </Flex>
        </Flex>
    );
}
