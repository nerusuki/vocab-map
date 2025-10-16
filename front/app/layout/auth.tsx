import { Outlet } from "react-router";
import { Card } from "~/components/ui/card";

export default function AuthLayout() {
  return (
    <main className="flex items-center justify-center pt-32 pb-4">
      <Card className="w-[25%] p-8">
        <Outlet />
      </Card>
    </main>
  );
}
