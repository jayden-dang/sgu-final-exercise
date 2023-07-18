import {
    Flex,
    Circle,
    Box,
    Image,
    Badge,
    Text,
    useColorModeValue,
    Icon,
    chakra,
    Tooltip,
    Button,
} from "@chakra-ui/react";
import { BsStar, BsStarFill, BsStarHalf } from "react-icons/bs";
import { FiShoppingCart } from "react-icons/fi";
import { IProductPreview } from "@contractTypes/index";
import { Link } from "react-router-dom";

interface IProductAddToCartProps {
    product: IProductPreview;
}

function ProductAddToCart({ product: data }: IProductAddToCartProps) {
    return (
        <Box
            bg={useColorModeValue("white", "gray.800")}
            maxW="xs"
            width={"250px"}
            borderWidth="1px"
            rounded="lg"
            shadow="lg"
            position="relative"
            _hover={{ shadow: "2xl" }}
            cursor={"pointer"}
        >
            <Image
                src={data.image}
                width={"100%"}
                shadow={"lg"}
                aspectRatio={7 / 8}
                alt={`Picture of ${data.title}`}
                roundedTop="lg"
            />

            <Box p="6">
                <Box fontSize="md" fontWeight="semibold" textAlign={"center"}>
                    {data.title}
                </Box>

                <Flex justifyContent="space-between" alignItems={"center"} alignContent="center">
                    <Text>{data.stock} in stock</Text>
                    <Flex
                        justifyContent={"center"}
                        alignItems={"center"}
                        fontSize="2xl"
                        color={useColorModeValue("gray.800", "white")}
                    >
                        {data.price}
                        <Box as="span" color={"gray.600"} ml={"4px"} fontSize="lg">
                            <Image src="https://cryptologos.cc/logos/near-protocol-near-logo.png" width={"16px"} />
                        </Box>
                    </Flex>
                </Flex>
                <Flex mt={"2"}>
                    <Button colorScheme="teal" size="lg" width={"full"} as={Link} to={`/product/${data.id}`}>
                        Xem chi tiết
                    </Button>
                </Flex>
            </Box>
        </Box>
    );
}

export default ProductAddToCart;
