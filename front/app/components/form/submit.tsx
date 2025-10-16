"use client";

import { Loader2 } from "lucide-react";
import type React from "react";
import { useFormContext } from "react-hook-form";
import { cn } from "../../lib/utils";
import { Button } from "../ui/button";

interface SubmitProps extends React.ComponentProps<typeof Button> {
  children: React.ReactNode;
}

export function Submit({ children, className, ...props }: SubmitProps) {
  const { formState } = useFormContext();

  const pending = formState.isSubmitting;

  return (
    <Button
      type="submit"
      className={cn("flex items-center gap-2", className)}
      {...props}
      disabled={pending || props.disabled}
    >
      {pending && (
        <Loader2 className="h-5 w-5 animate-spin text-primary-foreground" />
      )}
      {children}
    </Button>
  );
}
