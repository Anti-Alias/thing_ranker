import { VStack, Heading } from "@chakra-ui/react";
import { fetchCategoryPage } from "../api";
import ItemList from "../components/ItemList";

function CategoryList() {
  return (
    <VStack>
      <Heading>Categories</Heading>
      <ItemList fetchItemPage={fetchCategoryPage} />
    </VStack>
  );
}

export default CategoryList;
