import { VStack, Spinner, Heading, HStack, createListCollection, Button } from "@chakra-ui/react";
import { useEffect, useState } from "react";
import type { Item, ItemPage } from "../model/item";
import { toaster } from "./ui/toaster";
import type { Order } from "../model/order";
import Select from "./Select";
import ItemGrid from "./ItemGrid";
import SearchInput from "./SearchInput";

type LoadingState = 'loading' | 'finished';

const orderOptions = createListCollection({
  items: [
    { label: 'Ascending', value: 'asc' },
    { label: 'Descending', value: 'desc' },
  ],
});

interface ItemPageParams {
  fetchItemPage: (order: Order, name?: string | null, cursor?: string | null) => Promise<ItemPage>;
}

function ItemList({ fetchItemPage }: ItemPageParams) {

  const [loadingState, setLoadingState] = useState<LoadingState>('loading');
  const [items, setItems] = useState<Item[]>([]);
  const [cursor, setCursor] = useState<string | null>(null);
  const [name, setName] = useState<string>('');
  const [order, setOrder] = useState<Order>('asc');
  const endOfData = !cursor;

  // Loads initial page of items
  useEffect(() => {
    const loadInitialPage = async () => {
      try {
        setLoadingState('loading');
        setItems([]);
        const firstPage = await fetchItemPage(order, name);
        setItems(firstPage.items);
        setCursor(firstPage.cursor);
      }
      catch (e: any) {
        console.error('Failed to fetch items on page load:', e);
        toaster.create({ description: "Failed to fetch items", type: "error" });
      }
      finally {
        setLoadingState('finished');
      }
    };
    loadInitialPage();
  }, [order, name])

  // Loads additional page of items
  const loadMore = async () => {
    try {
      setLoadingState('loading');
      const nextPage = await fetchItemPage(order, name, cursor);
      setItems([...items, ...nextPage.items])
      setCursor(nextPage.cursor);
    }
    catch (e: any) {
      console.error('Failed to fetch additional items:', e);
      toaster.create({ description: "Failed to fetch items", type: "error" });
    }
    finally {
      setLoadingState('finished');
    }
  }

  return (
    <VStack>
      <HStack alignSelf="start" gap={5}>
        <HStack>
          Name:
          <SearchInput placeholder="Search" onSearch={value => setName(value)} />
        </HStack>
        <HStack>
          Order:
          <Select width={150} collection={orderOptions} value={[order]} onValueChange={details => setOrder(details.value[0] as Order)} />
        </HStack>
      </HStack>
      {loadingState == 'finished' && items.length == 0 && <Heading>No results found</Heading>}
      {
        items.length > 0 && <>
          <ItemGrid items={items} />
          {!endOfData && <Button onClick={loadMore}>Load More</Button>}
        </>
      }
      {loadingState == 'loading' && <Spinner size="xl" />}
    </VStack >
  );
}

export default ItemList;
