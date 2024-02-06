"use client";
import { Tabs, TabsContent, TabsList, TabsTrigger } from "@/components/ui/tabs";
import { zodResolver } from "@hookform/resolvers/zod";
import { open } from "@tauri-apps/api/dialog";
import { sep } from "@tauri-apps/api/path";
import { invoke } from "@tauri-apps/api/tauri";
import { AnimatePresence, HTMLMotionProps, motion } from "framer-motion";
import { ReactNode, useState } from "react";
import { BsCopy, BsUpload } from "react-icons/bs";
import SyntaxHighlighter from "react-syntax-highlighter";
import { nord } from "react-syntax-highlighter/dist/esm/styles/prism";

import { Button } from "@/components/ui/button";
import {
  Form,
  FormControl,
  FormField,
  FormItem,
  FormLabel,
  FormMessage
} from "@/components/ui/form";
import { Mermaid } from "@/components/ui/mermaid";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue
} from "@/components/ui/select";
import { useToast } from "@/components/ui/use-toast";
import { useForm } from "react-hook-form";
import { SiDocker, SiDotnet, SiNextdotjs } from "react-icons/si";
import { z } from "zod";
import { schema } from "./schema";
type Result = {
  mermaid: string;
  dockerfile: string;
};

export enum ProjectType {
  Dotnet = "Dotnet",
  Next = "Next"
}

export default function Home() {
  const { toast } = useToast();
  const form = useForm<z.infer<typeof schema>>({
    resolver: zodResolver(schema),
    defaultValues: {
      projectType: ProjectType.Dotnet,
      projectRoot: "",
      selectedStartupProject: "",
      availableProjectsInsideFolder: []
    }
  });

  const [result, setResult] = useState({} as Result);

  async function selectProjectRoot() {
    const selected = await open({
      directory: true,
      multiple: false
    });
    if (!selected) return;
    form.setValue("projectRoot", selected as string);
    if (isDotnet()) {
      const results = await invoke<string[]>("find_projects_files", {
        path: selected
      });
      form.setValue("availableProjectsInsideFolder", results);
    }
  }

  async function onSubmit(values: z.infer<typeof schema>) {
    const result = await invoke<Result>("my_custom_command", {
      projectRoot: values.projectRoot,
      maybeStartupProject: values.selectedStartupProject,
      projectType: values.projectType
    });
    setResult(result);
  }

  async function writeDockerFile() {
    try {
      toast({
        title: "Dockerfile criado com sucesso!",
        description:
          "Os arquivos Dockerfile e .dockerignore foram criados com sucesso!"
      });
      await invoke<string>("write_docker", {
        projectRoot: form.getValues().projectRoot,
        dockerfile: result.dockerfile
      });
    } catch (error: any) {
      toast({
        title: "Erro ao criar Dockerfile",
        description: error
      });
    }
  }

  const isDotnet = () => form.getValues().projectType === "Dotnet";

  return (
    <main className="min-h-screen p-12 space-y-8">
      <Form {...form}>
        <form onSubmit={form.handleSubmit(onSubmit)} className="space-y-8">
          <FormField
            control={form.control}
            name="projectType"
            render={({ field }) => (
              <FormItem>
                <FormLabel>Selecione a linguagem do projeto</FormLabel>
                <FormControl>
                  <Select
                    defaultValue={field.value}
                    onValueChange={(value) => {
                      form.reset({
                        projectRoot: "",
                        projectType: value,
                        availableProjectsInsideFolder: [],
                        selectedStartupProject: ""
                      });
                      setResult({
                        dockerfile: "",
                        mermaid: ""
                      });
                    }}
                  >
                    <SelectTrigger className="w-[400px]">
                      <SelectValue placeholder="Selecione o tipo da aplicação" />
                    </SelectTrigger>
                    <SelectContent>
                      <SelectItem
                        value={ProjectType.Dotnet}
                        className="flex flex-row"
                      >
                        <div className="flex gap-2 items-center">
                          <SiDotnet />
                          Dotnet
                        </div>
                      </SelectItem>
                      <SelectItem value={ProjectType.Next}>
                        <div className="flex gap-2 items-center">
                          <SiNextdotjs />
                          Next.js
                        </div>
                      </SelectItem>
                    </SelectContent>
                  </Select>
                </FormControl>
                <FormMessage />
              </FormItem>
            )}
          />

          <FormField
            control={form.control}
            name="projectRoot"
            render={({ field }) => (
              <div>
                <div
                  className="flex gap-5 hover:opacity-60 transition-opacity duration-300 ease-linear"
                  {...field}
                  onClick={() => {
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
                  <p className="text-sm font-medium font-sans">{field.value}</p>
                </div>
                <FormMessage />
              </div>
            )}
          />

          <AnimatedRender when={isDotnet}>
            <FormField
              control={form.control}
              name="selectedStartupProject"
              render={({ field }) => (
                <FormItem>
                  <FormLabel>Selecione o projeto principal</FormLabel>
                  <FormControl>
                    <Select onValueChange={field.onChange}>
                      <SelectTrigger className="w-[400px]">
                        <SelectValue placeholder="Selecione o projeto principal" />
                      </SelectTrigger>
                      <SelectContent>
                        {form
                          .watch("availableProjectsInsideFolder")
                          ?.map((project) => (
                            <SelectItem value={project} key={project}>
                              {project.split(sep).pop()}
                            </SelectItem>
                          ))}
                      </SelectContent>
                    </Select>
                  </FormControl>
                  <FormMessage />
                </FormItem>
              )}
            />
          </AnimatedRender>

          <div className="">
            <Button className="my-1" type="submit">
              Gerar Dockerfile
            </Button>
          </div>
        </form>
      </Form>

      <AnimatedRender when={() => !!result.dockerfile}>
        <Tabs defaultValue="docker" className="min-w-[400px] min-h-[400px] ">
          <TabsList>
            <TabsTrigger value="docker">
              <SiDocker className="mr-2" size={24} />
              Dockerfile
            </TabsTrigger>
            <Render when={isDotnet}>
              <TabsTrigger value="project-reference" disabled={!result.mermaid}>
                <SiDotnet className="mr-2" size={24} />
                Visualizar referencias do projeto
              </TabsTrigger>
            </Render>
          </TabsList>
          <TabsContent value="docker" className="">
            {result.dockerfile && (
              <>
                <div className="">
                  <Button
                    className="my-1"
                    type="button"
                    disabled={
                      !result.dockerfile || !form.getValues().projectRoot
                    }
                    onClick={() => writeDockerFile()}
                  >
                    Escrever Dockerfile
                  </Button>
                </div>
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
                    showLineNumbers
                    style={nord}
                  >
                    {result.dockerfile}
                  </SyntaxHighlighter>
                </div>
              </>
            )}
          </TabsContent>
          <TabsContent value="project-reference">
            <div className="my-1">
              <Mermaid chart={result.mermaid} />
            </div>
          </TabsContent>
        </Tabs>
      </AnimatedRender>
    </main>
  );
}

export function Render({
  when,
  children
}: {
  when: () => boolean;
  children: ReactNode;
}) {
  return when() ? <>{children}</> : null;
}

export function AnimatedRender({
  when,
  children,
  motionProps = {
    transition: { duration: 0.2 },
    initial: { opacity: 0, x: -100 },
    animate: { opacity: 1, x: 0 },
    exit: { opacity: 0, x: -100 }
  }
}: {
  when: () => boolean;
  children: ReactNode;
  motionProps?: HTMLMotionProps<"div">;
}) {
  return (
    <AnimatePresence mode="wait">
      {when() ? <motion.div {...motionProps}>{children}</motion.div> : null}
    </AnimatePresence>
  );
}
