import { Card, Image } from "@chakra-ui/react";
import { Link } from "react-router";
import { ITEM_HEIGHT, ITEM_WIDTH } from "../constants";

interface ItemCardProps {
  title: string;
  imageSrc: string;
  onClick?: () => void;
  href?: string;
}

export default function ItemCard({ title, imageSrc, onClick, href }: ItemCardProps) {
  if (href) {
    return (
      <Link to={href}>
        <Card.Root onClick={onClick} size="sm">
          <Card.Header alignItems="center" padding={1}>
            <Card.Title>{title}</Card.Title>
          </Card.Header>
          <Card.Body paddingTop={1}>
            <Image width={ITEM_WIDTH} height={ITEM_HEIGHT} src={imageSrc} alt={title} />
          </Card.Body>
        </Card.Root>
      </Link>
    );
  }
  else {
    return (
      <Card.Root onClick={onClick} size="sm">
        <Card.Header alignItems="center" padding={1}>
          <Card.Title>{title}</Card.Title>
        </Card.Header>
        <Card.Body paddingTop={1}>
          <Image width={ITEM_WIDTH} height={ITEM_HEIGHT} src={imageSrc} alt={title} />
        </Card.Body>
      </Card.Root>
    );
  }
}
