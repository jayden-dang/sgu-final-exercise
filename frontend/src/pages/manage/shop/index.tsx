import { useAppSelector } from "@/redux/hooks";
import {
    selectAccountId,
    selectContractId,
    selectIsLoggingIn,
    selectIsSignedIn,
    selectWallet,
} from "@/redux/splice/nearWalletSplice";
import { Button, Flex, Table, TableContainer, Tbody, Td, Text, Th, Thead, Tr, useToast } from "@chakra-ui/react";
import { IShop } from "@contractTypes/index";
import React, { useState } from "react";
import { Navigate, useNavigate } from "react-router-dom";

export default function ShopManage() {
    const wallet = useAppSelector(selectWallet);
    const [myShops, setMyShops] = useState<IShop>();
    const [isLoading, setIsLoading] = useState(false);
    const isSignedIn = useAppSelector(selectIsSignedIn);
    const contractId = useAppSelector(selectContractId);
    const accountId = useAppSelector(selectAccountId);
    const navigate = useNavigate();
    const toast = useToast();
    React.useEffect(() => {
        if (isSignedIn) {
            setIsLoading(true);
            wallet
                .viewMethod({
                    contractId: contractId,
                    method: "getShopByOwner",
                    args: {
                        accountId,
                    },
                })
                .then((res) => {
                    if (res) {
                        setMyShops(res);
                    } else {
                        toast({
                            title: "Bạn chưa có shop",
                            status: "error",
                            duration: 5000,
                            isClosable: true,
                        });
                        navigate("/");
                    }
                    setIsLoading(false);
                });
        }
    }, [isSignedIn]);
    React.useEffect(() => {
        if (!isSignedIn) {
            toast({
                title: "Bạn chưa đăng nhập",
                status: "error",
                duration: 5000,
                isClosable: true,
            });
            navigate("/");
        }
    }, [isSignedIn]);
    if (isLoading) {
        return <div>Loading...</div>;
    }

    return (
        <Flex direction={"column"} mt={"5"} w={"100%"} shadow={"lg"} p={8} maxWidth={"90vw"} mx={"auto"}>
            <Flex>
                <Text as={"h1"} fontSize={"2xl"}>
                    Quản lý Shop của bạn: {myShops?.name}
                </Text>
            </Flex>
            <Flex mt={"10"}>
                <Text as={"h2"} fontSize={"xl"}>
                    Danh sách sản phẩm
                </Text>
                <Button ml={"auto"} colorScheme="teal" size="lg" onClick={() => navigate("/manage/product/create")}>
                    Tạo sản phẩm mới
                </Button>
            </Flex>
            <TableContainer>
                <Table variant="simple" colorScheme="teal">
                    <Thead>
                        <Tr>
                            <Th>To convert</Th>
                            <Th>into</Th>
                            <Th isNumeric>multiply by</Th>
                        </Tr>
                    </Thead>
                    <Tbody>
                        <Tr>
                            <Td>inches</Td>
                            <Td>millimetres (mm)</Td>
                            <Td isNumeric>25.4</Td>
                        </Tr>
                        <Tr>
                            <Td>feet</Td>
                            <Td>centimetres (cm)</Td>
                            <Td isNumeric>30.48</Td>
                        </Tr>
                        <Tr>
                            <Td>yards</Td>
                            <Td>metres (m)</Td>
                            <Td isNumeric>0.91444</Td>
                        </Tr>
                    </Tbody>
                </Table>
            </TableContainer>
        </Flex>
    );
}
