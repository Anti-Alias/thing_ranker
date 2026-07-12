import { Heading, VStack } from "@chakra-ui/react";

export default function RootErrorBoundary() {
  return (
    <VStack>
      <Heading>Oops, something went wrong!</Heading>
    </VStack>
  );
}
