import { VStack } from "@chakra-ui/react";
import { fetchCategoryPage } from "../api";
import ItemList from "../components/ItemList";

function CategoryList() {
  return (
    <VStack alignItems="stretch">
      <ItemList
        fetchItemPage={fetchCategoryPage}
        itemHref={item => `/categories/${item.id}`}
      />
    </VStack>
  );
}

export default CategoryList;
