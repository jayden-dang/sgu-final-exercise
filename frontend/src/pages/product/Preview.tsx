import React from "react";

import {
    AlertDialog,
    AlertDialogBody,
    AlertDialogContent,
    AlertDialogFooter,
    AlertDialogHeader,
    AlertDialogOverlay,
    Box,
    Button,
    Flex,
    Heading,
    Image,
    Stack,
    Text,
    useBreakpointValue,
    useDisclosure,
    useToast,
} from "@chakra-ui/react";
import { IProduct } from "@contractTypes/index";
import { selectContractId, selectWallet } from "@/redux/splice/nearWalletSplice";
import { useAppSelector } from "@/redux/hooks";
import { convertToNear } from "@/utils";

interface PreviewProps {
    product: IProduct;
}

export default function Preview({ product }: PreviewProps) {
    const { isOpen, onOpen, onClose } = useDisclosure();
    const cancelRef = React.useRef();
    const wallet = useAppSelector(selectWallet);
    const contractId = useAppSelector(selectContractId);
    const toast = useToast();
    const onBuy = () => {
        wallet.callMethod({
            contractId: contractId,
            method: "buyProduct",
            args: {
                id: product.id,
                quantity: 1,
            },
            deposit: product.price.toString(),
        });
    };
    return (
        <Flex width={"100%"} direction={{ base: "column", md: "row" }} shadow={"lg"}>
            <ComfirmBuyDialog
                isOpen={isOpen}
                onClose={onClose}
                product={product}
                leastDestructiveRef={cancelRef}
                onBuy={onBuy}
            />
            <Flex>
                <Flex justify={"center"} ml={"10"} alignItems={"center"} width={350} height={450}>
                    <Image alt={"Login Image"} objectFit={"cover"} maxHeight={"95%"} src={product.image} />
                </Flex>
            </Flex>
            <Flex p={8} flex={1} align={"center"} justify={"center"}>
                <Stack w={"full"} maxW={"lg"} spacing={"8"}>
                    <Heading fontSize={{ base: "2xl", md: "3xl", lg: "4xl" }}>
                        <Text color={"blue.400"} as={"span"}>
                            {product.title}
                        </Text>{" "}
                    </Heading>
                    <Flex wrap={"wrap"} gap={"5px"} width={"100%"}>
                        <Flex textAlign={"center"} fontWeight={"semibold"} fontSize={"xl"} w={"48%"}>
                            Tên shop:
                        </Flex>
                        <Flex textAlign={"center"} fontWeight={"semibold"} fontSize={"xl"} w={"48%"}>
                            Số lượng tồn:
                        </Flex>

                        <Flex textAlign={"center"} fontSize={"xl"} w={"48%"}>
                            {product.shop?.name}
                        </Flex>

                        <Flex textAlign={"center"} fontSize={"xl"} w={"48%"}>
                            {product.stock}
                        </Flex>
                    </Flex>
                    <Flex width={"full"} direction={"row"} justifyContent={"space-between"}>
                        <Flex gap={"20px"} alignItems={"center"} textAlign={"center"}>
                            <Flex
                                alignItems={"center"}
                                textAlign={"center"}
                                fontWeight={"semibold"}
                                fontSize={"xl"}
                                w={"30%"}
                            >
                                Giá:
                            </Flex>
                            <Flex
                                alignItems={"center"}
                                color={"red.500"}
                                textAlign={"center"}
                                gap={"5px"}
                                fontSize={"3xl"}
                                w={"30%"}
                            >
                                <span>{product.price}</span>
                                <Image src="https://cryptologos.cc/logos/near-protocol-near-logo.png" width={"28px"} />
                            </Flex>
                        </Flex>
                        <Button
                            rounded={"full"}
                            bg={"blue.400"}
                            color={"white"}
                            _hover={{
                                bg: "blue.500",
                            }}
                            minWidth={"200px"}
                            size={"lg"}
                            onClick={onOpen}
                        >
                            Mua ngay
                        </Button>
                    </Flex>
                </Stack>
            </Flex>
        </Flex>
    );
}

interface ComfirmBuyDialogProps {
    isOpen: boolean;
    onClose: () => void;
    product: IProduct;
    leastDestructiveRef: any;
    onBuy: () => void;
}

const ComfirmBuyDialog = ({ isOpen, onBuy, onClose, product, leastDestructiveRef }: ComfirmBuyDialogProps) => {
    return (
        <>
            <AlertDialog isOpen={isOpen} onClose={onClose} leastDestructiveRef={leastDestructiveRef}>
                <AlertDialogOverlay>
                    <AlertDialogContent>
                        <AlertDialogHeader fontSize="lg" fontWeight="bold">
                            Xác nhận mua sản phẩm
                        </AlertDialogHeader>

                        <AlertDialogBody>
                            Bạn có chắc chắn muốn mua sản phẩm <b>{product.title}</b> với giá{" "}
                            <b>{product.price} near</b>?
                        </AlertDialogBody>

                        <AlertDialogFooter>
                            <Button onClick={onClose}>Hủy bỏ</Button>
                            <Button colorScheme="blue" onClick={onBuy} ml={3}>
                                Xác nhận
                            </Button>
                        </AlertDialogFooter>
                    </AlertDialogContent>
                </AlertDialogOverlay>
            </AlertDialog>
        </>
    );
};
