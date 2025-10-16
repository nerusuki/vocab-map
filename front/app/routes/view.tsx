import { getSession } from "~/utils/session.server";
import { data, redirect } from "react-router";
import type { Route } from "./+types/view";
import { useEffect, useState } from "react";
import axios from "axios";

export async function loader({ request }: Route.LoaderArgs) {
  const session = await getSession(request.headers.get("Cookie"));

  if (!session.has("token")) {
    return redirect("/login");
  }

  return data({ token: session.data.token });
}

function Word({ word }: { word: string }) {
  return (
    <div className="inline-block aspect-square rounded-full border-amber-400 border-2 p-4">
      {word}
    </div>
  );
}

export default function View({ loaderData }: Route.ComponentProps) {
  const [wordList, setWordList] = useState([]);

  const { token } = loaderData;

  const update = async () => {
    const response = await axios
      .get(import.meta.env.VITE_API_URL + "/vocab", {
        headers: { Authorization: `Bearer ${token}` },
      })
      .catch((error) => {
        console.log(error);
      });
    if (response) {
      setWordList(response.data);
    }
  };

  useEffect(() => {
    update();
  }, []);

  return (
    <div>
      {wordList.map((word) => (
        <Word key={word} word={word} />
      ))}
    </div>
  );
}
