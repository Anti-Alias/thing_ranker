# ThingRanker UI

## Running UI

* Ensure that the API is up and running properly.
* Run `npm install` to download node dependencies.
* Create a .env.local file with the following contents:
```
VITE_API_BASE_URL=<base_url>                # Base URL of the API. When running locally, this should be 'http://localhost:8080'
VITE_GOOGLE_CLIENT_ID=<google_client_id>    # Google's client ID of the app. Same value as the one in the API project.
```
* Install node dependencies with the following command:
```bash
npm install
```
* Run the following command to start the app:
```bash
npm run dev
```
