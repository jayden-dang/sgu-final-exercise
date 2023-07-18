import { Flex, Table, TableCaption, TableContainer, Tbody, Td, Text, Th, Thead, Tr } from "@chakra-ui/react";
import { IProduct } from "@contractTypes/index";
import React from "react";
interface IProps {
    product: IProduct;
}
export default function Detail({ product }: IProps) {
    return (
        <Flex mt={"60px"} p={8} direction={"column"} shadow={"lg"}>
            <Text as={"h3"} fontWeight={"semibold"} fontSize={"2xl"}>
                Thông tin chi tiết sản phẩm
            </Text>
            <TableContainer>
                <Table mt={"6"} variant="simple">
                    <Thead>
                        <Tr fontSize={"lg"}>
                            <Th>Tên thuộc tính</Th>
                            <Th>Giá trị</Th>
                        </Tr>
                    </Thead>
                    <Tbody>
                        {Object.keys(product.attributes).map((key) => {
                            return (
                                <Tr key={key}>
                                    <Td>{key}</Td>
                                    <Td>{product.attributes[key]}</Td>
                                </Tr>
                            );
                        })}
                    </Tbody>
                </Table>
            </TableContainer>
            <Text as={"h3"} fontWeight={"semibold"} fontSize={"2xl"} mt={"6"}>
                Mô tả sản phẩm
            </Text>
            <Text mt={"6"} ml={"5"}>
                {product.description.split("\n").map((item, key) => {
                    return (
                        <span key={key}>
                            {item}
                            <br />
                        </span>
                    );
                })}
            </Text>
        </Flex>
    );
}
