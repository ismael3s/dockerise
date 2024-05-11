"use client";

import { zodResolver } from "@hookform/resolvers/zod";
import { useForm } from "react-hook-form";
import { z } from "zod";

import { Button } from "@/components/ui/button";
import {
  Form,
  FormControl,
  FormDescription,
  FormField,
  FormItem,
  FormLabel,
  FormMessage
} from "@/components/ui/form";
import { Textarea } from "@/components/ui/textarea";
import { toast } from "@/components/ui/use-toast";

const FormSchema = z.object({
  json: z
    .string({
      required_error: "Campo obrigatório"
    })
    .refine((value) => {
      try {
        const result = JSON.parse(value);
        console.log(result);
        return true;
      } catch (error) {
        return false;
      }
    }, "O JSON informado é inválido")
});

type Props = {
  onFormSubmit: (env: string) => void;
};

export function JTEForm({ onFormSubmit }: Props) {
  const form = useForm<z.infer<typeof FormSchema>>({
    resolver: zodResolver(FormSchema)
  });

  function objectKeys(obj: any, parentKey = ""): { [key: string]: string } {
    return Object.keys(obj).reduce(
      (acc, key) => {
        const value = obj[key];

        if (typeof value === "object") {
          const newKey = parentKey ? `${parentKey}__${key}` : key;
          return {
            ...acc,
            ...objectKeys(value, newKey)
          };
        }

        const newKey = parentKey ? `${parentKey}__${key}` : key;
        return {
          ...acc,
          [newKey]: value
        };
      },
      {} as {
        [key: string]: string;
      }
    );
  }

  function onSubmit({ json }: z.infer<typeof FormSchema>) {
    const data = JSON.parse(json);
    const object = Object.keys(data).reduce(
      (acc, key) => {
        if (typeof data[key] === "object") {
          const newKey = key;
          return {
            ...acc,
            ...objectKeys(data[key], newKey)
          };
        }
        acc[key] = data[key];
        return acc;
      },
      {} as {
        [key: string]: string;
      }
    );
    const rows: string[] = [];
    Object.entries(object).forEach(([key, value]) =>
      rows.push(`${key}=${value}`)
    );
    const result = rows.join("\n");
    onFormSubmit(result);
  }

  return (
    <Form {...form}>
      <form
        onSubmit={form.handleSubmit(onSubmit)}
        className="w-full space-y-6 flex flex-col"
      >
        <FormField
          control={form.control}
          name="json"
          render={({ field }) => (
            <FormItem className="">
              <FormLabel>JSON</FormLabel>
              <FormControl>
                <Textarea
                  className="h-96 w-80"
                  placeholder="Insira o JSON aqui"
                  {...field}
                />
              </FormControl>

              <FormMessage />
            </FormItem>
          )}
        />

        <Button type="submit">Submit</Button>
      </form>
    </Form>
  );
}
