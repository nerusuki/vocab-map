import { useFormContext, type FieldValues } from "react-hook-form";
import {
  FormControl,
  FormDescription,
  FormField,
  FormItem,
  FormLabel,
  FormMessage,
} from "~/components/ui/form";
import { Input as _Input } from "~/components/ui/input";

interface InputProps<T extends FieldValues> {
  name: string;
  label: string;
  description?: string;
  type?: "text" | "email" | "password" | "number" | "tel" | "url";
}

export default function Input<T extends FieldValues>({
  name,
  label,
  description,
  type,
  ...rest
}: InputProps<T>) {
  const { control } = useFormContext();

  return (
    <FormField
      control={control}
      name={name}
      render={({ field }) => (
        <FormItem>
          <FormLabel>{label}</FormLabel>
          <FormControl>
            <_Input type={type} {...field} />
          </FormControl>
          {!!description && <FormDescription>{description}</FormDescription>}
          <FormMessage />
        </FormItem>
      )}
    />
  );
}
