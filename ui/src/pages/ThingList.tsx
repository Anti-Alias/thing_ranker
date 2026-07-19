import { VStack, Heading } from "@chakra-ui/react";
import { fetchThingPage } from "../api";
import ItemList from "../components/ItemList";

function ThingList() {
  return (
    <VStack>
      <Heading>Things</Heading>
      <ItemList fetchItemPage={fetchThingPage} />
    </VStack >
  );
}

export default ThingList;
