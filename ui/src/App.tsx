import { Outlet } from "react-router";
import { Box, Flex } from "@chakra-ui/react";
import { UserManager } from "oidc-client-ts";
import Navbar from "./components/Navbar";

export interface AppProps {
  userManager: UserManager,
}

export default function App() {
  return (
    <Box>
      <Navbar />
      <Flex justify="center">
        <Box width={1300}>
          <Outlet />
        </Box>
      </Flex>
    </Box>
  );
}
