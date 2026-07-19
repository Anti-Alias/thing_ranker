import { Button, CloseButton, Dialog, HStack, Portal, VStack } from "@chakra-ui/react";
import { GoogleLogin, type CredentialResponse } from '@react-oauth/google';
import { useState } from "react";
import { toaster } from "./ui/toaster";
import { parseAccountToken, type Account } from "../model/account";
import { fetchAccountToken } from "../api";

interface SignInButtonProps {
  setAccount: (account: Account) => void;
}

export default function SignInButton({ setAccount }: SignInButtonProps) {

  const [dialogOpen, setDialogOpen] = useState(false);

  // Handles account sign in.
  const signIn = async (idpResponse: CredentialResponse) => {
    try {
      if (!idpResponse.credential) throw new Error('Identity provider did not provide a "credential" field');
      const picture = parsePictureFromCredential(idpResponse.credential);
      const token = await fetchAccountToken(idpResponse.credential);
      const claims = parseAccountToken(token);
      setAccount({ ...claims, token, picture });
    }
    catch (e: any) {
      console.error('Sign in failed:', e);
      toaster.create({ description: "Failed to sign in", type: "error" });
    }
    finally {
      setDialogOpen(false);
    }
  };


  return (
    <Dialog.Root open={dialogOpen} onOpenChange={e => setDialogOpen(e.open)}>

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

/** Parses an identity provider's token, and returns the picture URL */
function parsePictureFromCredential(idpCredential: string): string {
  const idpClaimsBase64: string = idpCredential.split('.')[1];
  const idpClaimsString = atob(idpClaimsBase64);
  const idpClaims = JSON.parse(idpClaimsString);
  return idpClaims.picture;
}
