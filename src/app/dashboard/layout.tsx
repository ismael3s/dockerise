import { ReactNode } from "react";
import { AriseNavigationMenu } from "./_components/menu";

export default function Layout({ children }: { children: ReactNode }) {
  return (
    <>
      <div className="flex flex-col gap-4 mt-4">
        <AriseNavigationMenu />
        <main className="p-4">{children}</main>
      </div>
    </>
  );
}
