export interface Thing {
  id: number,
  accountId: number,
  name: string,
  image?: string,
  created: string,
  modified: string,
}

export interface ThingPage {
  things: Thing[],
  cursor: string,
}
