import { Flex } from "@chakra-ui/react";
import React from "react";
import Header from "./Header";
import { Outlet } from "react-router-dom";

export default function () {
    return (
        <Flex direction={"column"} w="full" minH={"100vh"}>
            <Header />
            <Outlet />
        </Flex>
    );
}
