import { createRoot } from "react-dom/client";
import { createBrowserRouter } from "react-router";
import { RouterProvider } from "react-router/dom";
import App from "./App";
import Home from "./pages/Home";
import './main.css';
import { CustomChakraProvider } from "./components/ui/provider";
import NotFound from "./pages/NotFound";
import { GoogleOAuthProvider } from '@react-oauth/google';
import RootErrorBoundary from "./pages/RootErrorBoundary";
import { GOOGLE_CLIENT_ID } from "./constants";

/** All page routes in application  */
const router = createBrowserRouter([
  {
    path: "/",
    Component: App,
    ErrorBoundary: RootErrorBoundary,
    children: [
      { index: true, Component: Home },
      { path: "/*", Component: NotFound },
    ],
  },
]);

/** OIDC configuration */

const root = document.getElementById("root")!;

createRoot(root).render(
  <CustomChakraProvider>
    <GoogleOAuthProvider clientId={GOOGLE_CLIENT_ID}>
      <RouterProvider router={router} />,
    </GoogleOAuthProvider>
  </CustomChakraProvider>
);
