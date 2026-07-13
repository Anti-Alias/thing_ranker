import { Card, Image } from "@chakra-ui/react";
import type { Thing } from "../model/thing";
import { ASSET_BASE_URL } from "../constants";

export interface ThingCardProps {
  thing: Thing,
}

export default function ThingCard({ thing }: ThingCardProps) {
  console.log(ASSET_BASE_URL);
  const imageSrc = `${ASSET_BASE_URL}/images/${thing.imageName}`;
  return (
    <Card.Root size="sm">
      <Card.Header alignItems="center" padding={1}>
        <Card.Title>{thing.name}</Card.Title>
      </Card.Header>
      <Card.Body paddingTop={1}>
        <Image width={220} height={220} src={imageSrc} />
      </Card.Body>
    </Card.Root>
  );
}
