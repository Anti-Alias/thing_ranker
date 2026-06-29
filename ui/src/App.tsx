import { Outlet } from "react-router";
import Navbar from "./components/Navbar";
import { Box, Flex } from "@chakra-ui/react";

function App() {
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

export default App;
