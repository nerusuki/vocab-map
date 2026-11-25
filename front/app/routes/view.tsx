"use client";

import { useState, useEffect } from "react";
import { WordBubble } from "~/components/word-bubble";
import { SearchBar } from "~/components/search-bar";
import { DefinitionPanel } from "~/components/definition-panel";
import axios from "axios";
import type { Route } from "./+types/view";
import { getSession } from "~/utils/session.server";
import { data, redirect } from "react-router";
import { Button } from "~/components/ui/button";
import { BookOpen } from "lucide-react";
import { toast } from "sonner";
import { Toaster } from "~/components/ui/sonner";
import { useLanguage } from "~/components/language-select.client";
import { Viewport } from "~/components/viewport";

export async function loader({ request }: Route.LoaderArgs) {
  const session = await getSession(request.headers.get("Cookie"));

  if (!session.has("token")) {
    return redirect("/login");
  }

  return data({ token: session.data.token });
}

export interface Word {
  word: string;
  x: number;
  y: number;
}

const WORD_AREA = 180 * 180;

export default function View({ loaderData }: Route.ComponentProps) {
  const [words, setWords] = useState<Word[]>([]);
  const [selectedWords, setSelectedWords] = useState<Set<string>>(new Set());
  const lang = useLanguage?.();
  const canvasSize = Math.sqrt(WORD_AREA * words.length);

  const { token } = loaderData;

  const toggleWordSelection = (wordId: string) => {
    setSelectedWords((prev) => {
      const newSet = new Set(prev);
      if (newSet.has(wordId)) {
        newSet.delete(wordId);
      } else {
        newSet.add(wordId);
      }
      return newSet;
    });
  };

  const update = async () => {
    const response = await axios
      .get<Word[]>(import.meta.env.VITE_API_URL + "/vocab/projected", {
        headers: { Authorization: `Bearer ${token}` },
        params: { lang },
      })
      .catch((error) => {
        console.log(error);
      });
    if (response) {
      const words = response.data;
      const canvasSize = Math.sqrt(WORD_AREA * words.length);

      const minX = Math.min(...words.map((word) => word.x));
      const minY = Math.min(...words.map((word) => word.y));

      words.forEach((word) => {
        word.x -= minX;
        word.y -= minY;
      });

      const maxX = Math.max(...words.map((word) => Math.abs(word.x)));
      const maxY = Math.max(...words.map((word) => Math.abs(word.y)));

      if (maxX > 0 && maxY > 0) {
        words.forEach((word) => {
          word.x /= maxX;
          word.y /= maxY;
        });
      }

      words.forEach((word1) => {
        words.forEach((word2) => {
          const dx = word2.x - word1.x;
          const dy = word2.y - word1.y;
          const distance = Math.sqrt(dx * dx + dy * dy);
          const minDistance = 100 / canvasSize;

          if (distance < minDistance) {
            const angle = Math.atan2(dy, dx);
            const overlap = minDistance - distance;
            const push = overlap * 0.5;

            const pushX = Math.cos(angle) * push;
            const pushY = Math.sin(angle) * push;

            word1.x -= pushX;
            word1.y -= pushY;
            word2.x += pushX;
            word2.y += pushY;
          }
        });
      });

      setWords(words);
    }
  };

  useEffect(() => {
    update();
  }, [lang]);

  const addWord = async (word: string) => {
    const response = await axios.put(
      import.meta.env.VITE_API_URL + "/vocab/add/" + word,
      {},
      {
        headers: { Authorization: `Bearer ${token}` },
        params: { lang },
      },
    );

    update();
  };

  const addWordFromSelected = async (words: string[]) => {
    const response = await axios.post(
      import.meta.env.VITE_API_URL + "/vocab/add",
      { words, lang },
      {
        headers: { Authorization: `Bearer ${token}` },
      },
    );

    toast(response.data.message);
    update();
  };

  const deleteWords = async () => {
    if (selectedWords) {
      const response = await axios.post(
        import.meta.env.VITE_API_URL + "/vocab/delete",
        { words: [...selectedWords], lang },
        {
          headers: { Authorization: `Bearer ${token}` },
        },
      );

      toast(response.data.message);
      update();
      setSelectedWords(new Set());
    }
  };

  const onKeyUp = (e: any) => {
    if (e.keyCode == 46) {
      deleteWords();
    }
  };

  return (
    <div
      className="flex h-screen flex-col bg-background touch-none"
      onKeyUp={onKeyUp}
    >
      <SearchBar onAddWord={addWord} />
      <Toaster />

      <div className="flex flex-1 overflow-hidden">
        <Viewport>
          <div
            className="absolute aspect-square transition-all duration-200"
            style={{
              width: canvasSize,
              left: `calc(50% - ${canvasSize / 2}px)`,
              top: `calc(50% - ${canvasSize / 2}px)`,
            }}
          >
            {words.map((word) => (
              <WordBubble
                key={word.word}
                word={word}
                isSelected={selectedWords.has(word.word)}
                onClick={() => toggleWordSelection(word.word)}
              />
            ))}
          </div>
        </Viewport>

        <DefinitionPanel words={[...selectedWords]} />
      </div>

      <div className="border-t bg-card px-6 py-4">
        <div className="max-w-2xl mx-auto flex items-center gap-3">
          <p className="text-sm text-muted-foreground">
            {selectedWords.size > 0
              ? `${selectedWords.size} word${selectedWords.size > 1 ? "s" : ""} selected`
              : "Discover new words:"}
          </p>
          <div className="flex gap-2 ml-auto">
            <Button
              variant="outline"
              size="sm"
              onClick={() =>
                addWordFromSelected(
                  words
                    .filter((w) => selectedWords.has(w.word))
                    .map((w) => w.word),
                )
              }
            >
              <BookOpen className="h-4 w-4 mr-2" />
              Add new word
            </Button>
          </div>
        </div>
      </div>
    </div>
  );
}
