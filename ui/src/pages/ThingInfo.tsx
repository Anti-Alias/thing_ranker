import { VStack, Heading, Spinner, Image } from "@chakra-ui/react";
import { useEffect, useState } from "react";
import { useParams } from "react-router";
import type { Thing } from "../model/thing";
import { fetchCategoryPageForThing, fetchThing } from "../api";
import { toaster } from "../components/ui/toaster";
import { ASSET_BASE_URL, ITEM_HEIGHT, ITEM_WIDTH } from "../constants";
import ItemList from "../components/ItemList";
import type { Order } from "../model/order";

type LoadState =
  { state: 'loading' } |
  { state: 'finished', thing: Thing } |
  { state: 'failed' };

type ThingInfoPathParams = { thingId: string; }

/** Page that displays information about a single thing */
function ThingInfo() {

  const { thingId: thingIdStr } = useParams<ThingInfoPathParams>();
  const [loadState, setLoadState] = useState<LoadState>({ state: 'loading' });

  if (!thingIdStr) throw new Error("thingId not supplied in route");
  const thingId = Number.parseInt(thingIdStr);

  // Fetches categories that contain thing
  const fetchCategoryPage = async (order: Order, name?: string | null, cursor?: string | null) => {
    return await fetchCategoryPageForThing(thingId, order, name, cursor);
  };

  // Loads thing on page load
  useEffect(() => {
    const loadThing = async () => {
      try {
        const thing = await fetchThing(thingId);
        setLoadState({ state: 'finished', thing });
      }
      catch (e: any) {
        console.log('Failed to load thing:', e);
        toaster.create({ description: "Failed to load thing", type: "error" });
        setLoadState({ state: 'failed' });
      }
    };
    loadThing();
  }, [thingId])

  return (
    <VStack align="stretch">

      {/* Loading spinner */}
      {loadState.state == 'loading' &&
        <VStack align="center">
          <Spinner size="xl" />
        </VStack>
      }

      {/* Thing title and image */}
      {loadState.state == 'finished' && <>
        <VStack align="center">
          <Heading as="h1">{`${loadState.state} (Thing)`}</Heading>
          <Image
            width={ITEM_WIDTH}
            height={ITEM_HEIGHT}
            src={ASSET_BASE_URL + '/images/' + loadState.thing.image}
            alt={loadState.thing.name}
          />
        </VStack>
      </>
      }

      {/* Categories of thing */}
      {loadState.state == 'finished' &&
        <VStack align="center">
          <Heading alignSelf="start" as="h2">Categories</Heading>
        </VStack>
      }
      <ItemList
        fetchItemPage={fetchCategoryPage}
        itemHref={item => `/categories/${item.id}`}
        hidden={loadState.state != 'finished'}
      />
    </VStack >
  );
}

export default ThingInfo;
