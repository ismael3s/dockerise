"use client";

import Link from "next/link";
import { SiDocker } from "react-icons/si";

export default function SideMenu() {
  return (
    <aside className="flex flex-col h-full overflow-y-auto border-r-[1px] pr-4 border-spacing-9">
      <div className="flex  h-full flex-col overflow-y-auto">
        <div className="mb-10 flex items-center rounded-lg px-3 py-2 text-slate-900 dark:text-white">
          <svg
            className="h-5 w-5"
            aria-hidden="true"
            xmlns="http://www.w3.org/2000/svg"
            width="24"
            height="24"
            viewBox="0 0 24 24"
            fill="none"
            stroke="currentColor"
            stroke-width="2"
            stroke-linecap="round"
            stroke-linejoin="round"
          >
            <path d="M15 6v12a3 3 0 1 0 3-3H6a3 3 0 1 0 3 3V6a3 3 0 1 0-3 3h12a3 3 0 1 0-3-3" />
          </svg>
          <span className="ml-3 text-base font-semibold">Arise</span>
        </div>
        <ul className="space-y-2 text-sm font-medium">
          <li>
            <Link
              href="#"
              className="flex items-center rounded-lg px-3 py-2 text-slate-900 hover:bg-slate-100 dark:text-white dark:hover:bg-slate-700"
            >
              <SiDocker size={20} />
              <span className="ml-3 flex-1 whitespace-nowrap">Home</span>
            </Link>
          </li>
        </ul>
      </div>
    </aside>
  );
}
