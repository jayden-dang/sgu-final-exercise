import { useAppSelector } from "@/redux/hooks";
import { selectAccountId, selectContractId, selectIsSignedIn, selectWallet } from "@/redux/splice/nearWalletSplice";
import { Flex, Image, Table, TableContainer, Tbody, Td, Text, Th, Thead, Tr, useToast } from "@chakra-ui/react";
import { IOrder, IProduct, IProductPreview } from "@contractTypes/index";
import React from "react";
import { useNavigate } from "react-router-dom";

export default function OwnedProductPage() {
    const contractId = useAppSelector(selectContractId);
    const wallet = useAppSelector(selectWallet);
    const isLogin = useAppSelector(selectIsSignedIn);
    const navigate = useNavigate();
    const toast = useToast();
    const [ownedProducts, setOwnedProducts] = React.useState<
        (IOrder & {
            product: IProductPreview;
        })[]
    >([]);
    const accountId = useAppSelector(selectAccountId);
    React.useEffect(() => {
        if (!isLogin) {
            toast({
                title: "Bạn chưa đăng nhập",
                status: "error",
                duration: 5000,
                isClosable: true,
            });
            navigate("/");
        }
    }, [isLogin]);
    React.useEffect(() => {
        if (isLogin) {
            wallet
                .viewMethod({
                    contractId: contractId,
                    method: "getProductsByUser",
                    args: {
                        accountId,
                    },
                })
                .then((res) => {
                    setOwnedProducts(res);
                });
        }
    }, [isLogin]);
    console.log(ownedProducts);

    return (
        <Flex alignItems={"start"} mt={"8"} direction={"column"} w={"full"} maxW={"90vw"} mx={"auto"}>
            <Text as={"h1"} fontSize={"3xl"} fontWeight={"bold"}>
                Sản phẩm của bạn
            </Text>
            <TableContainer>
                <Table variant="simple">
                    <Thead>
                        <Tr>
                            <Th>STT</Th>
                            <Th>Hình ảnh</Th>
                            <Th>Tên sản phẩm</Th>
                            <Th isNumeric>Số lượng đã mua</Th>
                        </Tr>
                    </Thead>
                    <Tbody>
                        {ownedProducts.map((product, index) => (
                            <Tr key={index}>
                                <Td>{index + 1}</Td>
                                <Td>
                                    <Image
                                        src={product.product.image}
                                        alt={product.product.title}
                                        width={"100px"}
                                        height={"150px"}
                                    />
                                </Td>
                                <Td fontWeight={"semibold"}>{product.product.title}</Td>
                                <Td isNumeric>{product.quantity}</Td>
                            </Tr>
                        ))}
                    </Tbody>
                </Table>
            </TableContainer>
        </Flex>
    );
}
