import { VStack, Spinner, Grid } from "@chakra-ui/react";
import { useEffect, useState } from "react";
import type { Thing } from "../model/thing";
import { fetchThingPage } from "../api";
import { toaster } from "../components/ui/toaster";
import ThingCard from "../components/ThingCard";

function Things() {

  const [things, setThings] = useState<Thing[] | null>(null);
  const [_cursor, setCursor] = useState<string | null>(null);
  const [failed, setFailed] = useState<boolean>(false);

  // Loads things on page load
  useEffect(() => {
    const loadThings = async () => {
      try {
        setFailed(false);
        const thingPage = await fetchThingPage();
        setThings(thingPage.things);
        setCursor(thingPage.cursor);
      }
      catch (e: any) {
        console.error('Failed to fetch things:', e);
        toaster.create({ description: "Failed to fetch things", type: "error" });
        setFailed(true);
      }
    };
    loadThings();
  }, [])

  return (
    <VStack align="center">
      {
        things &&
        <Grid templateColumns="repeat(5, 250px)" gap={3}>
          {things.map((thing) => <ThingCard key={thing.id} thing={thing} />)}
        </Grid>
      }
      {!things && !failed && <Spinner size="xl" />}
    </VStack>
  );
}

export default Things;
