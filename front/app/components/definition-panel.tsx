"use client";

import { Suspense } from "react";
import { ScrollArea } from "~/components/ui/scroll-area";
import { Loader2 } from "lucide-react";
import { WordDetails } from "./word-details";

interface DefinitionPanelProps {
  words: string[];
}

export function DefinitionPanel({ words }: DefinitionPanelProps) {
  if (words.length === 0) {
    return (
      <div className="w-96 border-l bg-card p-6 flex items-center justify-center">
        <p className="text-muted-foreground text-center text-sm">
          Select a word to view its definition
        </p>
      </div>
    );
  }

  return (
    <div className="w-96 border-l bg-card">
      <ScrollArea className="h-full">
        <div className="p-6 space-y-6">
          {words.map((word) => (
            <Suspense fallback={<LoadingFallback />}>
              <WordDetails key={word} word={word} />
            </Suspense>
          ))}
        </div>
      </ScrollArea>
    </div>
  );
}

function LoadingFallback() {
  return (
    <div className="flex items-center gap-2 text-muted-foreground">
      <Loader2 className="h-4 w-4 animate-spin" />
      <span className="text-sm">Loading definition...</span>
    </div>
  );
}
