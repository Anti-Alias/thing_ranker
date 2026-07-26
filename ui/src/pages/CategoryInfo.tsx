import { VStack, Heading, Spinner, Image } from "@chakra-ui/react";
import { useEffect, useState } from "react";
import { useParams } from "react-router";
import { toaster } from "../components/ui/toaster";
import { ASSET_BASE_URL, ITEM_HEIGHT, ITEM_WIDTH } from "../constants";
import ItemList from "../components/ItemList";
import type { Order } from "../model/order";
import type { Category } from "../model/category";
import { fetchCategory, fetchThingPageForCategory } from "../api";

type LoadState =
  { state: 'loading' } |
  { state: 'finished', category: Category } |
  { state: 'failed' };

type CategoryInfoPathParams = { categoryId: string; }

/** Page that displays information about a single category */
function CategoryInfo() {

  const { categoryId: categoryIdStr } = useParams<CategoryInfoPathParams>();
  const [loadState, setLoadState] = useState<LoadState>({ state: 'loading' });

  if (!categoryIdStr) throw new Error("categoryId not supplied in route");
  const categoryId = Number.parseInt(categoryIdStr);

  // Fetches things that contain category
  const fetchThingPage = async (order: Order, name?: string | null, cursor?: string | null) => {
    return await fetchThingPageForCategory(categoryId, order, name, cursor);
  };

  // Loads category on page load
  useEffect(() => {
    const loadCategory = async () => {
      try {
        const category = await fetchCategory(categoryId);
        setLoadState({ state: 'finished', category });
      }
      catch (e: any) {
        console.log('Failed to load category:', e);
        toaster.create({ description: "Failed to load category", type: "error" });
        setLoadState({ state: 'failed' });
      }
    };
    loadCategory();
  }, [categoryId])

  return (
    <VStack align="stretch">

      {/* Loading spinner */}
      {loadState.state == 'loading' &&
        <VStack align="center">
          <Spinner size="xl" />
        </VStack>
      }

      {/* Category title and image */}
      {loadState.state == 'finished' && <>
        <VStack align="center">
          <Heading as="h1">{`${loadState.category.name} (Category)`}</Heading>
          <Image
            width={ITEM_WIDTH}
            height={ITEM_HEIGHT}
            src={ASSET_BASE_URL + '/images/' + loadState.category.image}
            alt={loadState.category.name}
          />
        </VStack>
      </>
      }

      {/* Things of category */}
      {loadState.state == 'finished' &&
        <VStack align="center">
          <Heading alignSelf="start" as="h2">Things</Heading>
        </VStack>
      }
      <ItemList
        fetchItemPage={fetchThingPage}
        itemHref={item => `/things/${item.id}`}
        hidden={loadState.state != 'finished'}
      />
    </VStack >
  );
}

export default CategoryInfo;
