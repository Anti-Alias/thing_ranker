import { Avatar, AvatarGroup, Button, HStack, Spacer } from "@chakra-ui/react";
import { Link } from "react-router";
import { ColorModeButton } from "./ui/color-mode";
import SignInButton from "./SignInButton";
import type { Account } from "../model/account";
import { CONTENT_WIDTH } from "../constants";

export interface NavbarProps {
  account: Account | null;
  setAccount: (account: Account | null) => void;
}

export default function Navbar({ account, setAccount }: NavbarProps) {

  const signedIn = !!account;

  return (
    <HStack as="nav" bg="bg" paddingTop={2} position="sticky" top={0} width={CONTENT_WIDTH} zIndex={1}>

      { /** Left */}
      <Link to="/">
        <Button variant="ghost" fontSize={17} padding={0}>
          ThingRanker
        </Button>
      </Link>

      <Spacer />

      { /** Center */}
      <HStack gap={2}>
        <Link to="/">
          <Button variant="ghost">About</Button>
        </Link>
        <Link to="/things">
          <Button variant="ghost">Things</Button>
        </Link>
        <Link to="/categories">
          <Button variant="ghost">Categories</Button>
        </Link>
      </HStack>

      <Spacer />

      { /** Right */}
      {
        account &&
        <AvatarGroup>
          <Avatar.Root size="xs">
            <Avatar.Fallback />
            <Avatar.Image src={account.picture} />
          </Avatar.Root>
        </AvatarGroup>
      }
      {!signedIn && <SignInButton setAccount={setAccount} />}
      {signedIn && <Button size="xs" onClick={() => setAccount(null)}>Sign out</Button>}
      <ColorModeButton />
    </HStack >
  )
}

