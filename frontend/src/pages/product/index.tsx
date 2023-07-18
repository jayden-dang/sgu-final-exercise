import { useAppSelector } from "@/redux/hooks";
import { selectContractId, selectIsSettedUp, selectWallet } from "@/redux/splice/nearWalletSplice";
import { Flex, useToast } from "@chakra-ui/react";
import { IProduct } from "@contractTypes/index";
import React from "react";
import { Navigate, useParams } from "react-router-dom";
import Preview from "./Preview";
import Detail from "./Detail";

export default function ProductDeailPage() {
    const id = useParams<{ id: string }>().id;
    const toast = useToast();
    const wallet = useAppSelector(selectWallet);
    const contractId = useAppSelector(selectContractId);
    const [product, setProduct] = React.useState<IProduct>();
    const isSettedUp = useAppSelector(selectIsSettedUp);
    const [isLoading, setIsLoading] = React.useState(false);
    const getProductDetail = async () => {
        return wallet.viewMethod({
            contractId: contractId,
            method: "getProductDetails",
            args: { id },
        }) as Promise<IProduct>;
    };
    React.useEffect(() => {
        if (isSettedUp && id) {
            setIsLoading(true);
            getProductDetail().then((res) => {
                setProduct(res);
                setIsLoading(false);
            });
        }
    }, [isSettedUp, id]);
    if (!id) {
        toast({
            title: "Không tìm thấy sản phẩm",
            status: "error",
            duration: 5000,
            isClosable: true,
        });
        return <Navigate to={"/"} replace={true} />;
    }
    return (
        <Flex direction={"column"} maxW={"80vw"} mx={"auto"}>
            {isLoading && <div>Loading...</div>}
            {!isLoading && product && <Preview product={product} />}
            {!isLoading && product && <Detail product={product} />}
        </Flex>
    );
}
