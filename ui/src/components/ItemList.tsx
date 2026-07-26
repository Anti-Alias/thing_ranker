import { VStack, Spinner, HStack, createListCollection, Button } from "@chakra-ui/react";
import { useEffect, useState } from "react";
import type { Item, ItemPage } from "../model/item";
import { toaster } from "./ui/toaster";
import type { Order } from "../model/order";
import Select from "./Select";
import ItemListContent from "./ItemListContent";
import SearchInput from "./SearchInput";

type LoadingState = 'loading' | 'finished';

const defaultPageSize = 18;

const orderOptions = createListCollection({
  items: [
    { label: 'Ascending', value: 'asc' },
    { label: 'Descending', value: 'desc' },
  ],
});

interface ItemListPageParams {
  /** Function that fetches item pages from API*/
  fetchItemPage: (
    pageSize?: number | null,
    order?: Order | null,
    name?: string | null,
    cursor?: string | null,
  ) => Promise<ItemPage>;
  /** What to do when an item card is clicked */
  onItemClick?: (item: Item) => void;
  /** Function that generates a link for each item card */
  itemHref?: (item: Item) => string;
  /** Page size of item list */
  pageSize?: number | null,
  /** If hidden, does not render. This allows data fetching even when the component is not visible. */
  hidden?: boolean;
}

function ItemList({
  fetchItemPage,
  onItemClick,
  itemHref,
  pageSize: ps,
  hidden,
}: ItemListPageParams) {

  const [loadingState, setLoadingState] = useState<LoadingState>('loading');
  const [items, setItems] = useState<Item[]>([]);
  const [cursor, setCursor] = useState<string | null>(null);
  const [search, setSearch] = useState<string>('');
  const [order, setOrder] = useState<Order>('asc');
  const [hideSearch, setHideSearch] = useState<boolean>(true);
  const endOfData = !cursor;
  const pageSize = ps ?? defaultPageSize;

  // Loads initial page of items
  useEffect(() => {
    const loadInitialPage = async () => {
      try {
        setLoadingState('loading');
        const firstPage = await fetchItemPage(pageSize, order, search);
        const hasMorePages = !!firstPage.cursor;
        const isSearchEmpty = search.length == 0;
        setItems(firstPage.items);
        setCursor(firstPage.cursor);
        setHideSearch(isSearchEmpty && !hasMorePages);
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
  }, [order, search])

  // Loads additional page of items
  const loadAdditionalPage = async () => {
    try {
      setLoadingState('loading');
      const nextPage = await fetchItemPage(pageSize, order, search, cursor);
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

  if (hidden) return;
  return (
    <VStack align="stretch">
      {
        // Search bar
        !hideSearch &&
        <HStack alignSelf="start" gap={5}>
          <HStack>
            <SearchInput placeholder="Search" onSearch={value => setSearch(value)} />
          </HStack>
          <HStack>
            Order:
            <Select width={150} collection={orderOptions} value={[order]} onValueChange={details => setOrder(details.value[0] as Order)} />
          </HStack>
        </HStack>
      }
      {
        <ItemListContent items={items} onItemClick={onItemClick} itemHref={itemHref} />
      }
      {
        // Load more button
        items.length != 0 && !endOfData &&
        <Button alignSelf="center" onClick={loadAdditionalPage}>Load More</Button>
      }
      {
        // Spinner
        loadingState == 'loading' &&
        <Spinner size="xl" alignSelf="center" />
      }
    </VStack>
  );
}

export default ItemList;
