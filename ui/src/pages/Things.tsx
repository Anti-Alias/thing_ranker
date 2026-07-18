import { VStack, Spinner, Heading, HStack, createListCollection, Button, Input } from "@chakra-ui/react";
import { useEffect, useState } from "react";
import type { Thing } from "../model/thing";
import { fetchThingPage } from "../api";
import { toaster } from "../components/ui/toaster";
import type { Order } from "../model/order";
import Select from "../components/Select";
import ItemGrid from "../components/ItemGrid";
import SearchInput from "../components/SearchInput";

type LoadingState = 'loading' | 'finished';

const orderOptions = createListCollection({
  items: [
    { label: 'Ascending', value: 'asc' },
    { label: 'Descending', value: 'desc' },
  ],
});

function Things() {

  const [loadingState, setLoadingState] = useState<LoadingState>('loading');
  const [things, setThings] = useState<Thing[]>([]);
  const [cursor, setCursor] = useState<string | null>(null);
  const [name, setName] = useState<string>('');
  const [order, setOrder] = useState<Order>('asc');
  const endOfData = !cursor;

  // Loads initial page of things
  useEffect(() => {
    const loadInitialPage = async () => {
      try {
        setLoadingState('loading');
        setThings([]);
        const firstPage = await fetchThingPage(order, name);
        setThings(firstPage.things);
        setCursor(firstPage.cursor);
      }
      catch (e: any) {
        console.error('Failed to fetch things on page load:', e);
        toaster.create({ description: "Failed to fetch things", type: "error" });
      }
      finally {
        setLoadingState('finished');
      }
    };
    loadInitialPage();
  }, [order, name])

  // Loads additional page of things
  const loadMore = async () => {
    try {
      setLoadingState('loading');
      const nextPage = await fetchThingPage(order, name, cursor);
      setThings([...things, ...nextPage.things])
      setCursor(nextPage.cursor);
    }
    catch (e: any) {
      console.error('Failed to fetch additional things:', e);
      toaster.create({ description: "Failed to fetch things", type: "error" });
    }
    finally {
      setLoadingState('finished');
    }
  }

  return (
    <VStack>
      <Heading>Things</Heading>
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
      {loadingState == 'finished' && things.length == 0 && <Heading>No results found</Heading>}
      {
        things.length > 0 && <>
          <ItemGrid items={things} />
          {!endOfData && <Button onClick={loadMore}>Load More</Button>}
        </>
      }
      {loadingState == 'loading' && <Spinner size="xl" />}
    </VStack >
  );
}

export default Things;
