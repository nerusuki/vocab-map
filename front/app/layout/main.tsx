import { Outlet } from "react-router";

export default function MainLayout() {
  return (
    <html lang="en">
      <body className={`font-sans antialiased`}>
        <Outlet />
      </body>
    </html>
  );
}
