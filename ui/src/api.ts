import { API_BASE_URL } from "./constants";
import type { Category, CategoryPage } from "./model/category";
import type { Order } from "./model/order";
import type { Thing, ThingPage } from "./model/thing";

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

export async function fetchThing(thingId: number): Promise<Thing> {
  const response = await fetch(`${API_BASE_URL}/things/${thingId}`)
  if (response.ok) {
    return response.json();
  }
  else {
    throw new Error('Failed to fetch thing due to API error response');
  }
}

/** Fetches a page of things */
export async function fetchThingPage(
  order: Order,
  name?: string | null,
  cursor?: string | null,
): Promise<ThingPage> {
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

/** Fetches a page of things for a particular category */
export async function fetchThingPageForCategory(
  categoryId: number,
  order: Order,
  name?: string | null,
  cursor?: string | null,
): Promise<CategoryPage> {
  const params = new URLSearchParams({ categoryId: categoryId.toString(), order });
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

export async function fetchCategory(categoryId: number): Promise<Category> {
  const response = await fetch(`${API_BASE_URL}/categories/${categoryId}`)
  if (response.ok) {
    return response.json();
  }
  else {
    throw new Error('Failed to fetch category due to API error response');
  }
}

/** Fetches a page of categories  */
export async function fetchCategoryPage(
  order: Order,
  name?: string | null,
  cursor?: string | null,
): Promise<CategoryPage> {
  const params = new URLSearchParams({ order });
  if (cursor) params.append('cursor', cursor);
  if (name) params.append('name', name);
  const url = new URL(`${API_BASE_URL}/categories`);
  url.search = params.toString();
  const response = await fetch(url);
  if (response.ok) {
    return response.json();
  }
  else {
    throw new Error('Failed to fetch categories due to API error response');
  }
}

/** Fetches a page of categories for a particular thing */
export async function fetchCategoryPageForThing(
  thingId: number,
  order: Order,
  name?: string | null,
  cursor?: string | null,
): Promise<CategoryPage> {
  const params = new URLSearchParams({ thingId: thingId.toString(), order });
  if (cursor) params.append('cursor', cursor);
  if (name) params.append('name', name);
  const url = new URL(`${API_BASE_URL}/categories`);
  url.search = params.toString();
  const response = await fetch(url);
  if (response.ok) {
    return response.json();
  }
  else {
    throw new Error('Failed to fetch categories due to API error response');
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

