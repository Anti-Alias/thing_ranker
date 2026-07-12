import { API_BASE_URL } from "./constants";

export async function fetchAccountToken(idpCredential: string): Promise<string> {
  const headers = { 'Authorization': `Bearer ${idpCredential}` };
  const response = await fetch(`${API_BASE_URL}/account/token`, { method: 'POST', headers });
  if (response.ok) {
    return response.text();
  }
  else {
    throw new Error('Sign in failed due to API error response');
  }
}
