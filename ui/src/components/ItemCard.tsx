import { Card, Image } from "@chakra-ui/react";

interface ItemCardProps {
  title: string,
  imageSrc: string,
}

export default function ItemCard({ title, imageSrc }: ItemCardProps) {
  return (
    <Card.Root size="sm">
      <Card.Header alignItems="center" padding={1}>
        <Card.Title>{title}</Card.Title>
      </Card.Header>
      <Card.Body paddingTop={1}>
        <Image width={220} height={220} src={imageSrc} />
      </Card.Body>
    </Card.Root>
  );
}
