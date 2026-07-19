import { Grid } from "@chakra-ui/react"
import type { Item } from "../model/item"
import ItemCard from "./ItemCard";
import { ASSET_BASE_URL } from "../constants";
import { memo } from "react";

interface ItemGridProps {
  items: Item[];
  onItemClick?: (item: Item) => void;
  itemHref?: (item: Item) => string;
}

function ItemGrid({ items, onItemClick, itemHref }: ItemGridProps) {

  return (
    <Grid templateColumns="repeat(5, 250px)" gap={3}>
      {items.map(item => {
        if (onItemClick) {
          return <ItemCard
            title={item.name}
            key={item.id}
            imageSrc={ASSET_BASE_URL + '/images/' + item.image}
            onClick={() => onItemClick(item)}
          />
        }
        else if (itemHref) {
          return <ItemCard
            title={item.name}
            key={item.id}
            imageSrc={ASSET_BASE_URL + '/images/' + item.image}
            href={itemHref(item)}
          />
        }
        else {
          return <ItemCard
            title={item.name}
            key={item.id}
            imageSrc={ASSET_BASE_URL + '/images/' + item.image}
          />
        }
      })}
    </Grid>
  );
}

export default memo(ItemGrid);
