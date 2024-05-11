"use client";
import { useState } from "react";
import { JTEForm } from "./jte-form";
import { Render } from "../../_components/docker";
import { BsCopy } from "react-icons/bs";
import { toast } from "@/components/ui/use-toast";
import { Textarea } from "@/components/ui/textarea";

export default function JTEPage() {
  const [env, setEnv] = useState<string | null>(null);

  return (
    <div className="grid grid-cols-2 gap-x-10">
      <JTEForm onFormSubmit={setEnv} />
      <div className="space-y-2 relative">
        <p>Resultado</p>

        <Render when={() => !!env}>
          <BsCopy
            className="cursor-pointer hover:opacity-70 transition-opacity duration-300 ease-linear 
                  absolute top-9 -right-1
                  "
            onClick={() => {
              window?.navigator?.clipboard.writeText(env!);
              toast({
                title: "Variáveis de ambiente copiadas"
              });
            }}
          />
        </Render>

        <Textarea className="h-96 w-80" value={env ?? ""} />
      </div>
    </div>
  );
}
