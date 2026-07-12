import { Outlet } from "react-router";
import { Box, VStack } from "@chakra-ui/react";
import { Toaster } from "./components/ui/toaster";
import Navbar from "./components/Navbar";
import { useState } from "react";
import { initAccount, writeAccountToStorage, type Account } from "./model/account";
import { CONTENT_WIDTH } from "./constants";

/** Properties to pass directly to child pages */
export interface PageProps {
  account: Account | null, setAccount: (account: Account) => void,
}

export default function App() {

  // Currently logged in account
  const [account, setAcc] = useState<Account | null>(initAccount);

  // Sets and persists account
  const setAccount = (account: Account | null) => {
    setAcc(account);
    writeAccountToStorage(account);
  };

  // Properties accessible from pages through context
  const pageProps: PageProps = { account, setAccount };

  return (
    <Box>
      <Toaster />
      <VStack align="center">
        <Navbar account={account} setAccount={setAccount} />
        <Box width={CONTENT_WIDTH}>
          { /** Current page */}
          <Outlet context={pageProps} />
        </Box>
      </VStack>
    </Box>
  );
}
