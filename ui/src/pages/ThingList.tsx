import { VStack, Heading } from "@chakra-ui/react";
import { fetchThingPage } from "../api";
import ItemList from "../components/ItemList";

function ThingList() {
  return (<>
    <VStack align="center">
      <Heading>Things</Heading>
    </VStack >
    <ItemList fetchItemPage={fetchThingPage} itemHref={item => `/thing/${item.id}`} />
  </>);
}

export default ThingList;
