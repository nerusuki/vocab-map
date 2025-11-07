import { HelloWorld } from "~/components/hello-world";
import type { Route } from "./+types/home";
import { getSession } from "~/utils/session.server";
import { redirect } from "react-router";

export function meta({}: Route.MetaArgs) {
  return [{ title: "Vocab Map" }];
}

export async function loader({ request }: Route.LoaderArgs) {
  const session = await getSession(request.headers.get("Cookie"));

  if (!session.has("token")) {
    return redirect("/login");
  }

  return redirect("/view");
}

export default function Home() {
  return <HelloWorld />;
}
