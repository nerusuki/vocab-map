import { HelloWorld } from "~/components/hello-world";
import type { Route } from "./+types/home";

export function meta({}: Route.MetaArgs) {
  return [{ title: "Vocab Map" }];
}

export default function Home() {
  return <HelloWorld />;
}
