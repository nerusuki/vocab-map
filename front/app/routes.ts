import {
  type RouteConfig,
  index,
  layout,
  route,
} from "@react-router/dev/routes";

export default [
  index("routes/home.tsx"),

  layout("layout/auth.tsx", [route("login", "./routes/login.tsx")]),
  layout("layout/main.tsx", [route("view", "./routes/view.tsx")]),
] satisfies RouteConfig;
