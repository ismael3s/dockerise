import mermaid from "mermaid";
import { useEffect } from "react";

mermaid.initialize({
  startOnLoad: true,
  theme: "dark",
  securityLevel: "loose",
  fontFamily: "monospace"
});

export const Mermaid = ({ chart }: { chart: string }) => {
  useEffect(() => {
    mermaid.contentLoaded();
    mermaid.render("mermaid", chart);
  }, []);
  return (
    <>
      <div className="mermaid w-full">{chart}</div>
    </>
  );
};
