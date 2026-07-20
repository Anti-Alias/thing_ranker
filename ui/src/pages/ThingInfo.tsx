import { VStack, Heading, Spinner, Image, Separator } from "@chakra-ui/react";
import { useEffect, useState } from "react";
import { useParams } from "react-router";
import type { Thing } from "../model/thing";
import { fetchCategoryPageForThing, fetchThing } from "../api";
import { toaster } from "../components/ui/toaster";
import { ASSET_BASE_URL, ITEM_HEIGHT, ITEM_WIDTH } from "../constants";
import ItemList from "../components/ItemList";
import type { Order } from "../model/order";

type ThingInfoPathParams = { thingId: string; }

/** Page that displays information about a single thing */
function ThingInfo() {

  const { thingId: thingIdStr } = useParams<ThingInfoPathParams>();
  const [thing, setThing] = useState<Thing | null>(null);

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
        const thingResp = await fetchThing(thingId);
        setThing(thingResp);
      }
      catch (e: any) {
        console.log('Failed to load thing:', e);
        toaster.create({ description: "Failed to load thing", type: "error" });
      }
    };
    loadThing();
  }, [thingId])

  return (
    <VStack align="stretch">

      {/* Loading spinner */}
      {!thing &&
        <VStack align="center">
          <Spinner size="xl" />
        </VStack>
      }

      {/* Thing title and image */}
      {thing && <>
        <VStack align="center">
          <Heading as="h1">{`${thing.name} (Thing)`}</Heading>
          <Image
            width={ITEM_WIDTH}
            height={ITEM_HEIGHT}
            src={ASSET_BASE_URL + '/images/' + thing.image}
            alt={thing.name}
          />
        </VStack>
        <Separator />
      </>
      }

      {/* Categories of thing */}
      {thing &&
        <VStack align="center">
          <Heading as="h2">Categories</Heading>
        </VStack>
      }
      <ItemList
        fetchItemPage={fetchCategoryPage}
        itemHref={item => `/categories/${item.id}`}
        hidden={!thing}
      />
    </VStack >
  );
}

export default ThingInfo;
