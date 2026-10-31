"use client";

import { useEffect, useState } from "react";
import type { Word } from "~/routes/view";
import { ScrollArea } from "~/components/ui/scroll-area";
import { Loader2 } from "lucide-react";
import { useLanguage, type Language } from "./language-select.client";
import axios from "axios";

interface DefinitionPanelProps {
  selectedWords: Word[];
}

interface Definition {
  word: string;
  phonetic?: string;
  meanings: Array<{
    partOfSpeech: string;
    definitions: Array<{
      definition: string;
      example?: string;
    }>;
  }>;
}

interface Vocab {
  id: number;
  word: string;
  lang: Language;
  reading?: string;
  definition?: string[];
  tags: string[];
}

export function DefinitionPanel({ selectedWords }: DefinitionPanelProps) {
  const [definitions, setDefinitions] = useState<Record<string, Vocab>>(
    {},
  );
  const [loading, setLoading] = useState<Set<string>>(new Set());
  const lang = useLanguage()

  useEffect(() => {
    selectedWords.forEach((word) => {
      if (!definitions[word.word] && !loading.has(word.word)) {
        fetchDefinition(word.word);
      }
    });
  }, [selectedWords]);

  const fetchDefinition = async (word: string) => {
    setLoading((prev) => new Set(prev).add(word));

      const response = await axios
        .get<Vocab[]>(import.meta.env.VITE_API_URL + "/vocab/definition/" + word, {
          params: { lang },
        })
        .catch((error) => {
          console.log(error);
        });

      if (response) {
        const data = response.data;
        setDefinitions((prev) => ({ ...prev, [word]: data[0] }));
      }
      setLoading((prev) => {
        const newSet = new Set(prev);
        newSet.delete(word);
        return newSet;
      });
  };

  if (selectedWords.length === 0) {
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
          {selectedWords.map((word) => {
            const def = definitions[word.word];
            const isLoading = loading.has(word.word);

            return (
              <div key={word.word} className="space-y-3">
                <div>
                  <h3 className="text-xl font-semibold text-foreground">
                    {word.word}
                  </h3>
                  {def?.reading && (
                    <p className="text-sm text-muted-foreground">
                      {def.reading}
                    </p>
                  )}
                </div>

                {isLoading && (
                  <div className="flex items-center gap-2 text-muted-foreground">
                    <Loader2 className="h-4 w-4 animate-spin" />
                    <span className="text-sm">Loading definition...</span>
                  </div>
                )}

                {def && (
                  <div className="space-y-4">
                    {def.definition?.map((definition, idx) => (
                      <div key={idx} className="space-y-2">
                        <p className="text-xs font-medium text-primary uppercase">
                          {def.word}
                        </p>
                        <div className="space-y-2">
                          <div className="space-y-1">
                            <p className="text-sm text-foreground leading-relaxed">
                              {definition}
                            </p>
                          </div>
                        </div>
                      </div>
                    ))}
                  </div>
                )}

                {!isLoading && !def && (
                  <p className="text-sm text-muted-foreground">
                    Definition not found
                  </p>
                )}
              </div>
            );
          })}
        </div>
      </ScrollArea>
    </div>
  );
}
