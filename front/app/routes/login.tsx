import { zodResolver } from "@hookform/resolvers/zod";
import { useForm } from "react-hook-form";
import { Form } from "~/components/ui/form";
import Field from "~/components/form/input";
import { z } from "zod";
import { Submit } from "~/components/form/submit";
import axios from "axios";
import type { Route } from "./+types/login";
import { commitSession, getSession } from "~/utils/session.server";
import { data, redirect } from "react-router";

const formSchema = z.object({
  username: z.string(),
  password: z.string().min(6, {
    message: "Password must be at least 6 characters.",
  }),
});

export async function loader({ request }: Route.LoaderArgs) {
  const session = await getSession(request.headers.get("Cookie"));

  if (session.has("token")) {
    return redirect("/");
  }

  return data(
    { error: session.get("error") },
    {
      headers: {
        "Set-Cookie": await commitSession(session),
      },
    },
  );
}

export async function action({ request }: Route.ActionArgs) {
  const session = await getSession(request.headers.get("Cookie"));

  const form = await request.formData();
  const username = form.get("username");
  const password = form.get("password");

  const response = await axios
    .post<string>(import.meta.env.VITE_API_URL + "/auth", {
      username,
      password,
    })
    .catch((error) => {
      if (error) {
        console.log(error);
      }
    });

  if (!response) {
    session.flash("error", "Invalid username/password");

    return redirect("/login", {
      headers: {
        "Set-Cookie": await commitSession(session),
      },
    });
  }

  const token = response.data;
  session.set("token", token);

  return redirect("/", {
    headers: {
      "Set-Cookie": await commitSession(session),
    },
  });
}

export default function Login({ loaderData }: Route.ComponentProps) {
  const form = useForm<z.infer<typeof formSchema>>({
    resolver: zodResolver(formSchema),
    defaultValues: {
      username: "",
      password: "",
    },
  });

  const { error } = loaderData;

  return (
    <div>
      {error ? <div className="error">{error}</div> : null}
      <Form {...form}>
        <form method="POST">
          <Field name="username" label="Username" />
          <Field name="password" type="password" label="Password" />
          <Submit className="w-full mt-4">Login</Submit>
        </form>
      </Form>
    </div>
  );
}
