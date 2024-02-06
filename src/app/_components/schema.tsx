"use client";
import { z } from "zod";
import { ProjectType } from "./home";

export const schema = z
  .object({
    projectType: z.string().min(1, "Campo obrigatorio"),
    projectRoot: z
      .string({
        required_error: "Campo obrigatorio"
      })
      .min(1, "Campo obrigatorio"),
    availableProjectsInsideFolder: z
      .array(z.string().default(""), {
        required_error: "Campo obrigatorio"
      })
      .default([])
      .optional()
      .nullable(),
    selectedStartupProject: z.string().default("").optional().nullable()
  })
  .refine(
    (schema) => {
      const isDotnetProjectWithoutStartupProject =
        schema.projectType === ProjectType.Dotnet &&
        !schema.selectedStartupProject;
      return !isDotnetProjectWithoutStartupProject;
    },
    {
      params: {
        message:
          "Ao selecionar o tipo de projeto dotnet, é obrigatorio selecionar o projeto principal."
      },
      message:
        "Ao selecionar o tipo de projeto dotnet, é obrigatorio selecionar o projeto principal.",
      path: ["selectedStartupProject"]
    }
  );
