"use client";

import type { Word } from "~/app/page";
import { cn } from "~/lib/utils";

interface WordBubbleProps {
  word: Word;
  isSelected: boolean;
  onClick: () => void;
}

export function WordBubble({ word, isSelected, onClick }: WordBubbleProps) {
  return (
    <button
      onClick={onClick}
      className={cn(
        "pointer-events-auto absolute px-6 py-3 rounded-full font-medium transition-all duration-200 cursor-pointer select-none",
        "hover:scale-110 active:scale-95 hover:z-20",
        isSelected
          ? "bg-primary text-primary-foreground shadow-lg scale-105 z-10"
          : "bg-muted text-muted-foreground hover:bg-muted/80",
      )}
      style={{
        left: `${word.x}px`,
        top: `${word.y}px`,
      }}
    >
      {word.word}
    </button>
  );
}
