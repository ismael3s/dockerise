import { ReactNode } from "react";
import SideMenu from "./_components/side-menu";

export default function Layout({ children }: { children: ReactNode }) {
  return (
    <section className="flex gap-5">
      <aside>
        <SideMenu />
      </aside>

      {children}
    </section>
  );
}
