"use client";
import { cn } from "@/lib/utils";
import { invoke } from "@tauri-apps/api/tauri";
import { useEffect, useRef, useState } from "react";

export default function Splash() {
  const ref = useRef<HTMLUListElement>(null);
  const [isHidden, setIsHidden] = useState(true);
  useEffect(() => {
    setTimeout(() => setIsHidden(false), 500);
    setTimeout(() => {
      invoke("close_splashscreen");
    }, 1_200);
  }, []);

  return (
    <>
      <ul
        className={cn(
          "flex flex-row fixed top-[50%] left-[50%] translate-x-[-50%] translate-y-[-50%] text-4xl font-bold ",
          "border-b-[1px] border-white"
        )}
        ref={ref}
      >
        <li
          className={cn(
            "opacity-100 transition-all duration-1000 ease-in-out max-w-[2em] float-left inline-block"
          )}
        >
          A
        </li>
        <li
          className={cn(
            "opacity-100 transition-all duration-1000 ease-in-out max-w-[2em] float-left inline-block",
            isHidden && "opacity-0 max-w-0"
          )}
        >
          R
        </li>
        <li
          className={cn(
            "opacity-100 transition-all duration-1000 ease-in-out max-w-[2em] float-left inline-block"
          )}
        >
          I
        </li>
        <li
          className={cn(
            "opacity-100 transition-all duration-1000 ease-in-out max-w-[2em] float-left inline-block",
            isHidden && "opacity-0 max-w-0"
          )}
        >
          S
        </li>
        <li
          className={cn(
            "opacity-100 transition-all duration-1000 ease-in-out max-w-[2em] float-left inline-block"
          )}
        >
          E
        </li>
      </ul>
    </>
  );
}
