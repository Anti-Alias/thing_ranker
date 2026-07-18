import { Grid } from "@chakra-ui/react"
import type { Item } from "../model/item"
import ItemCard from "./ItemCard";
import { ASSET_BASE_URL } from "../constants";
import { memo } from "react";

interface ItemGridProps {
  items: Item[],
}

function ItemGrid({ items }: ItemGridProps) {
  return (
    <Grid templateColumns="repeat(5, 250px)" gap={3}>
      {items.map(item =>
        <ItemCard
          title={item.name}
          key={item.id}
          imageSrc={ASSET_BASE_URL + '/images/' + item.image}
        />
      )}
    </Grid>
  );
}

export default memo(ItemGrid);
