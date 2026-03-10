"use client";

import type React from "react";

import { useEffect, useState } from "react";
import { Search } from "lucide-react";
import { Input } from "~/components/ui/input";
import axios from "axios";

interface SearchBarProps {
  onAddWord: (word: string) => void;
}

async function search(word: string) {
  const response = await axios
    .get(import.meta.env.VITE_API_URL + "/vocab/search/" + word)
    .catch((error) => {
      console.log(error);
    });

  if (!response) {
    return [];
  }

  return response.data;
}

export function SearchBar({ onAddWord }: SearchBarProps) {
  const [input, setInput] = useState("");
  const [words, setWords] = useState<string[]>([]);
  const [showSuggestions, setShowSuggestions] = useState(false);

  useEffect(() => {
    search(input).then(setWords);
  }, [input]);

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    if (input.trim() && words.includes(input.trim().toLowerCase())) {
      onAddWord(input.trim().toLowerCase());
      setInput("");
      setShowSuggestions(false);
    }
  };

  const handleSuggestionClick = (word: string) => {
    onAddWord(word);
    setInput("");
    setShowSuggestions(false);
  };

  return (
    <div className="border-b bg-card px-6 py-4">
      <form onSubmit={handleSubmit} className="max-w-2xl mx-auto relative">
        <div className="relative">
          <Search className="absolute left-3 top-1/2 -translate-y-1/2 h-4 w-4 text-muted-foreground" />
          <Input
            type="text"
            placeholder="Search for a word to add..."
            value={input}
            onChange={(e) => setInput(e.target.value)}
            onFocus={() => setShowSuggestions(true)}
            onBlur={() => setTimeout(() => setShowSuggestions(false), 200)}
            className="pl-10"
          />
        </div>

        {showSuggestions && words.length > 0 && (
          <div className="absolute top-full left-0 right-0 mt-1 bg-card border rounded-lg shadow-lg max-h-64 overflow-y-auto z-10">
            {words.map((word) => (
              <button
                key={word}
                type="button"
                onClick={() => handleSuggestionClick(word)}
                className="w-full px-4 py-2 text-left hover:bg-accent transition-colors text-sm"
              >
                {word}
              </button>
            ))}
          </div>
        )}
      </form>
    </div>
  );
}
