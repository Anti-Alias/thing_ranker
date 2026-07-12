// Local storage variables
const ACCOUNT_TOKEN_NAME = "accountToken";        // Account token of logged in user
const ACCOUNT_PICTURE_NAME = "accountPicture";    // URL to picture of account holder

export type Role = 'root' | 'admin' | 'basic';

export interface Account {
  id: number,
  role: Role,
  email: string,
  picture: string,
  exp: number,
  token: string,
}

interface AccountClaims {
  id: number,
  role: Role,
  email: string,
  exp: number,
}

/** Reads account from local storage. If expired or not present, returns null. */
export function initAccount(): Account | null {
  const account = readAccountFromStorage();
  if (!account) return null;
  const secondsSinceEpoch = Date.now() / 1000;
  if (secondsSinceEpoch > account.exp) {
    writeAccountToStorage(null);
    return null;
  }
  return account;
}

/** Reads account from local storage. If not present, returns null */
function readAccountFromStorage(): Account | null {
  const token = localStorage.getItem(ACCOUNT_TOKEN_NAME);
  const picture = localStorage.getItem(ACCOUNT_PICTURE_NAME);
  if (!token || !picture) return null;
  const claims = parseAccountToken(token);
  return { ...claims, picture, token };
}

/** Writes account to local storage */
export function writeAccountToStorage(account: Account | null) {
  if (account) {
    localStorage.setItem(ACCOUNT_TOKEN_NAME, account.token);
    localStorage.setItem(ACCOUNT_PICTURE_NAME, account.picture);
  }
  else {
    localStorage.removeItem(ACCOUNT_TOKEN_NAME);
    localStorage.removeItem(ACCOUNT_PICTURE_NAME);
  }
}

export function parseAccountToken(token: string): AccountClaims {
  const claimsBase64 = token.split('.')[1];
  const claimsString = atob(claimsBase64);
  return JSON.parse(claimsString);
}

