import { createRoot } from "react-dom/client";
import { createBrowserRouter } from "react-router";
import { RouterProvider } from "react-router/dom";
import App from "./App";
import Home from "./pages/Home";
import './main.css';
import { Provider } from "./components/ui/provider";
import NotFound from "./pages/NotFound";

const router = createBrowserRouter([
  {
    path: "/",
    Component: App,
    children: [
      { index: true, Component: Home },
      { path: "/*", Component: NotFound },
    ],
  },
]);

const root = document.getElementById("root")!;

createRoot(root).render(
  <Provider>
    <RouterProvider router={router} />,
  </Provider>
);
