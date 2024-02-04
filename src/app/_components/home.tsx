"use client";
import { Tabs, TabsContent, TabsList, TabsTrigger } from "@/components/ui/tabs";
import SyntaxHighlighter from "react-syntax-highlighter";
import { dark, nord } from "react-syntax-highlighter/dist/esm/styles/prism";
import { BsCopy, BsUpload } from "react-icons/bs";

import { open } from "@tauri-apps/api/dialog";
import { ReactElement, useRef, useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import { sep } from "@tauri-apps/api/path";

import { SiDocker, SiDotnet } from "react-icons/si";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue
} from "@/components/ui/select";
import { Mermaid } from "@/components/ui/mermaid";
import { Button } from "@/components/ui/button";
type Result = {
  mermaid: string;
  dockerfile: string;
};

export default function Home() {
  const [projectRoot, setProjectRoot] = useState<string>("");
  const [startupProject, setStartupProject] = useState<string>("");
  const [availableProjectsInsideFolder, setAvailableProjectsInsideFolder] =
    useState<string[]>([]);
  const [result, setResult] = useState({} as Result);
  const ref = useRef<HTMLDivElement>(null);
  async function selectProjectRoot() {
    const selected = await open({
      directory: true,
      multiple: false
    });
    if (!selected) return;

    const results = await invoke("find_projects_files", {
      path: selected
    });
    setAvailableProjectsInsideFolder(results as string[]);
    setProjectRoot(selected as string);
  }

  async function generateDockerFile() {
    const result = await invoke<Result>("my_custom_command", {
      projectRoot,
      startupProject
    });
    setResult(result);
  }

  return (
    <main className="min-h-screen p-12 space-y-8">
      <div>
        <div
          className="flex gap-5 hover:opacity-60 transition-opacity duration-300 ease-linear"
          onClick={(e) => {
            e.preventDefault();
            selectProjectRoot();
          }}
        >
          <label
            className="block mb-2 text-sm font-medium text-gray-900 dark:text-gray-300"
            htmlFor="file_input"
          >
            Selecione a raiz do projeto
          </label>
          <BsUpload id="file_input" />
        </div>
        <div>
          <p className="text-sm font-medium font-sans">{projectRoot}</p>
        </div>
      </div>

      <div className=" ">
        <label className="block mb-2 text-sm font-medium text-gray-900 dark:text-gray-300">
          Selecione o projeto principal
        </label>

        <Select
          disabled={!projectRoot}
          onValueChange={(value) => {
            setStartupProject(value);
          }}
        >
          <SelectTrigger className="w-[400px]">
            <SelectValue placeholder="Selecione o projeto principal" />
          </SelectTrigger>
          <SelectContent>
            {availableProjectsInsideFolder.map((project) => (
              <SelectItem value={project} key={project}>
                {project.split(sep).pop()}
              </SelectItem>
            ))}
          </SelectContent>
        </Select>
      </div>

      <Tabs defaultValue="docker" className="min-w-[400px] min-h-[400px] ">
        <TabsList>
          <TabsTrigger value="docker">
            <SiDocker className="mr-2" size={24} />
            Gerar Dockerfile
          </TabsTrigger>
          <TabsTrigger value="project-reference" disabled={!result.mermaid}>
            <SiDotnet className="mr-2" size={24} />
            Visualizar referencias do projeto
          </TabsTrigger>
        </TabsList>
        <TabsContent value="docker" className="">
          <Button
            disabled={!projectRoot || !startupProject}
            onClick={generateDockerFile}
            className="my-1"
          >
            Gerar Dockerfile
          </Button>

          {result.dockerfile && (
            <div className="relative mt-4">
              <BsCopy
                className="cursor-pointer hover:opacity-70 transition-opacity duration-300 ease-linear 
                absolute top-4 right-4
                "
                onClick={() => {
                  window?.navigator?.clipboard.writeText(result.dockerfile);
                }}
              />
              <SyntaxHighlighter
                language="dockerfile"
                // wrapLongLines
                showLineNumbers
                style={nord}
              >
                {result.dockerfile}
              </SyntaxHighlighter>
            </div>
          )}
        </TabsContent>
        <TabsContent value="project-reference">
          <div className="my-1">
            <Mermaid chart={result.mermaid} />
          </div>
        </TabsContent>
      </Tabs>
    </main>
  );
}
