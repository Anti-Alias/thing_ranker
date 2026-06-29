import { Button, CloseButton, Dialog, HStack, Portal, Image, Spacer, VStack } from "@chakra-ui/react";

function LogIn() {
  return (
    < Dialog.Root >

      { /** Log in button */}
      <Dialog.Trigger asChild>
        <Button size="xs">Log in</Button>
      </Dialog.Trigger>

      { /** Dialog */}
      <Portal>
        <Dialog.Backdrop />
        <Dialog.Positioner>
          <Dialog.Content>
            <Dialog.Header>
              <HStack width="100%" justify="center">
                <Dialog.Title>Log In</Dialog.Title>
              </HStack>
            </Dialog.Header>
            <Dialog.Body>
              <VStack width="100%" align="center">
                <LogInWith provider="Google" iconUrl="/icons/google.png" />
                <LogInWith provider="Facebook" iconUrl="/icons/facebook.svg" />
                <LogInWith provider="Apple" iconUrl="/icons/apple.svg" />
              </VStack>
            </Dialog.Body>
            <Dialog.CloseTrigger asChild>
              <CloseButton size="sm" />
            </Dialog.CloseTrigger>
          </Dialog.Content>
        </Dialog.Positioner>
      </Portal>
    </Dialog.Root >
  );
}

interface LogInWithProps {
  iconUrl: string,
  provider: string,
}

function LogInWith(props: LogInWithProps) {
  return (
    <Button width={250} bg="white" color="black" variant="outline">
      <HStack width="100%">
        <Image src={props.iconUrl} width="20px" />
        <Spacer />
        Log in with {props.provider}
        <Spacer />
      </HStack>
    </Button>
  );
}

export default LogIn;
