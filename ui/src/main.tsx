import { createRoot } from "react-dom/client";
import { createBrowserRouter } from "react-router";
import { RouterProvider } from "react-router/dom";
import SignIn from "./pages/SignIn";
import Home from "./pages/Home";

const router = createBrowserRouter([
  {
    path: "/",
    Component: Home,
  },
  {
    path: "/signin",
    Component: SignIn,
  }
]);

const root = document.getElementById("root")!;

createRoot(root).render(
  <RouterProvider router={router} />,
);
