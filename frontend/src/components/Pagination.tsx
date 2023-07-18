import { Button, Flex } from "@chakra-ui/react";

interface IPaginationProps {
    total: number;
    onPageChange: (page: number) => void;
    currentPage: number;
}

const Pagination = ({ total = 2, onPageChange, currentPage = 1 }: IPaginationProps) => {
    return (
        <Flex
            bg="#edf3f8"
            _dark={{
                bg: "#3e3e3e",
            }}
            p={50}
            w="full"
            alignItems="center"
            justifyContent="center"
        >
            <Flex>
                <PagButton disabled={currentPage === 1} onClick={() => onPageChange(currentPage - 1)}>
                    previous
                </PagButton>
                {Array.from({ length: total }).map((_, index) => (
                    <PagButton key={index} active={index + 1 === currentPage} onClick={() => onPageChange(index + 1)}>
                        {index + 1}
                    </PagButton>
                ))}

                <PagButton disabled={currentPage === total} onClick={() => onPageChange(currentPage + 1)}>
                    next
                </PagButton>
            </Flex>
        </Flex>
    );
};

export default Pagination;

const PagButton = (props: any) => {
    const activeStyle = {
        bg: "brand.600",
        _dark: {
            bg: "brand.500",
        },
        color: "white",
    };
    return (
        <Button
            mx={1}
            px={4}
            py={2}
            rounded="md"
            bg="white"
            color="gray.700"
            _dark={{
                color: "white",
                bg: "gray.800",
            }}
            opacity={props.disabled && 0.6}
            _hover={!props.disabled && activeStyle}
            cursor={props.disabled && "not-allowed"}
            onClick={props.disabled ? undefined : props.onClick}
            {...(props.active && activeStyle)}
        >
            {props.children}
        </Button>
    );
};
