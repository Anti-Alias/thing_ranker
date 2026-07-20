import { VStack, Heading } from "@chakra-ui/react";
import { fetchCategoryPage } from "../api";
import ItemList from "../components/ItemList";

function CategoryList() {
  return (
    <VStack align="start">
      <Heading alignSelf="center">Categories</Heading>
      <ItemList
        fetchItemPage={fetchCategoryPage}
        itemHref={item => `/categories/${item.id}`}
      />
    </VStack>
  );
}

export default CategoryList;
