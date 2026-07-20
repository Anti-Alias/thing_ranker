import { VStack, Heading } from "@chakra-ui/react";
import { fetchThingPage } from "../api";
import ItemList from "../components/ItemList";

function ThingList() {
  return (
    <VStack align="start">
      <Heading alignSelf="center">Things</Heading>
      <ItemList
        fetchItemPage={fetchThingPage}
        itemHref={item => `/things/${item.id}`}
      />
    </VStack >
  );
}

export default ThingList;
