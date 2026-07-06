import { Button, CloseButton, Dialog, HStack, Portal, VStack } from "@chakra-ui/react";
import { GoogleLogin, type CredentialResponse } from '@react-oauth/google';

export default function SignIn() {

  const signIn = async (response: CredentialResponse) => {
    const headers = { 'Authorization': `Bearer ${response.credential}` };
    const res = await fetch('http://localhost:8080/account/token', { method: 'POST', headers });
    console.log(res);
  };

  return (
    < Dialog.Root >

      { /** Sign in button; Triggers popup */}
      <Dialog.Trigger asChild>
        <Button size="xs">Sign in</Button>
      </Dialog.Trigger>

      { /** Popup Dialog */}
      <Portal>
        <Dialog.Backdrop />
        <Dialog.Positioner>
          <Dialog.Content>
            <Dialog.Header>
              <HStack width="100%" justify="center">
                <Dialog.Title>Sign In</Dialog.Title>
              </HStack>
            </Dialog.Header>
            <Dialog.Body>
              <VStack width="100%" align="center">
                <GoogleLogin onSuccess={(response) => signIn(response)} />
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
