export interface Thing {
  id: number,
  accountId: number,
  name: string,
  imageName?: string,
  created: string,
  modified: string,
}

export interface ThingPage {
  things: Thing[],
  cursor: string,
}
