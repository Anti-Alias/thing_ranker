export interface Item {
  id: number;
  accountId: number;
  name: string;
  image?: string;
  created: string;
  modified?: string;
}

export interface ItemPage {
  items: Item[];
  cursor: string;
}
