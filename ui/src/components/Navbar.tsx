import { Button, HStack, Spacer } from "@chakra-ui/react";
import { Link } from "react-router";
import { ColorModeButton } from "./ui/color-mode";
import SignIn from "./SignIn";
import type { UserManager } from "oidc-client-ts";

export default function Navbar() {
  return (
    <HStack as="nav" bg="bg" padding={2} position="sticky" top={0}>

      { /** Left */}
      <Link to="/">
        <Button variant="ghost" fontSize={17}>
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
      <ColorModeButton />
      <SignIn />
    </HStack>
  )
}

export interface NavbarProps {
  userManager: UserManager,
}

