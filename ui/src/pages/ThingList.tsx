import { VStack } from "@chakra-ui/react";
import { fetchThingPage } from "../api";
import ItemList from "../components/ItemList";

function ThingList() {
  return (
    <VStack alignItems="stretch">
      <ItemList
        fetchItemPage={fetchThingPage}
        itemHref={item => `/things/${item.id}`}
      />
    </VStack>
  );
}

export default ThingList;
