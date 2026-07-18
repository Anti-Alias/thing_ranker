import { API_BASE_URL } from "./constants";
import type { Order } from "./model/order";
import type { ThingPage } from "./model/thing";

export async function fetchAccountToken(idpCredential: string): Promise<string> {
  const headers = authHeaders(idpCredential);
  const response = await fetch(`${API_BASE_URL}/account/token`, { method: 'POST', headers });
  if (response.ok) {
    return response.text();
  }
  else {
    throw new Error('Sign in failed due to API error response');
  }
}

export async function fetchThingPage(order: Order, name?: string | null, cursor?: string | null): Promise<ThingPage> {
  const params = new URLSearchParams({ order });
  if (cursor) params.append('cursor', cursor);
  if (name) params.append('name', name);
  const url = new URL(`${API_BASE_URL}/things`);
  url.search = params.toString();
  const response = await fetch(url);
  if (response.ok) {
    return response.json();
  }
  else {
    throw new Error('Failed to fetch things due to API error response');
  }
}

export interface Headers {
  [key: string]: string;
}

export interface QueryParams {
  [key: string]: string;
}

export interface Body {
  [key: string]: any;
}

function authHeaders(bearerToken: string): Headers {
  return { 'Authorization': `Bearer ${bearerToken}` };
}
