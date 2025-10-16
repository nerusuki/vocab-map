import { Outlet } from "react-router";

export default function MainLayout() {
  return (
    <main className="flex items-center justify-center pt-32 pb-4">
      <div className="w-[80%] p-8">
        <Outlet />
      </div>
    </main>
  );
}
