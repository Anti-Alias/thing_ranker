import type { Account } from "./account";

/** Properties to pass directly to child pages */
export interface PageProps {
  account: Account | null;
}

