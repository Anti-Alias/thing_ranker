import { Wrap, Text, VStack } from "@chakra-ui/react"
import type { Item } from "../model/item"
import ItemCard from "./ItemCard";
import { ASSET_BASE_URL } from "../constants";
import { memo } from "react";

interface ItemListContentProps {
  items: Item[];
  onItemClick?: (item: Item) => void;
  itemHref?: (item: Item) => string;
}

function ItemListContent({ items, onItemClick, itemHref }: ItemListContentProps) {

  return (
    <VStack borderRadius="md" borderWidth="1px" padding={3} alignItems="stretch">
      {items.length == 0 &&
        <Text alignSelf="center">No results found</Text>
      }
      {items.length != 0 &&
        <Wrap justify="start" gap={3}>
          {items.length != 0 && items.map(item => {
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
        </Wrap>
      }
    </VStack>
  );
}

export default memo(ItemListContent);
