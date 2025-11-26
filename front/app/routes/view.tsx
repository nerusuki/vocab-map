"use client";

import { useState, useEffect, useRef } from "react";
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

export default function View({ loaderData }: Route.ComponentProps) {
  const [words, setWords] = useState<Word[]>([]);
  const [selectedWords, setSelectedWords] = useState<Set<string>>(new Set());
  const canvasRef = useRef<HTMLDivElement>(null);
  const [camera, setCamera] = useState({ x: 0, y: 0, zoom: 1 });
  const [isPanning, setIsPanning] = useState(false);
  const [panStart, setPanStart] = useState({ x: 0, y: 0 });

  const { token } = loaderData;

  useEffect(() => {
    const handleWheel = (e: WheelEvent) => {
      e.preventDefault();
      const zoomSpeed = 0.001;
      const delta = -e.deltaY * zoomSpeed;

      setCamera((prev) => {
        const zoom = Math.max(0.5, Math.min(3, prev.zoom + delta));
        return {
          ...prev,
          x: e.clientX - (e.clientX - prev.x) * (zoom / prev.zoom),
          y: e.clientY - (e.clientY - prev.y) * (zoom / prev.zoom),
          zoom,
        };
      });
    };

    const canvas = canvasRef.current;
    if (canvas) {
      canvas.addEventListener("wheel", handleWheel, { passive: false });
      return () => canvas.removeEventListener("wheel", handleWheel);
    }
  }, []);

  const handlePointerDown = (e: React.MouseEvent) => {
    if (e.button === 0 && e.target === canvasRef.current) {
      setIsPanning(true);
      setPanStart({ x: e.clientX - camera.x, y: e.clientY - camera.y });
    }
  };

  const handlePointerMove = (e: React.MouseEvent) => {
    if (isPanning) {
      setCamera((prev) => ({
        ...prev,
        x: e.clientX - panStart.x,
        y: e.clientY - panStart.y,
      }));
    }
  };

  const handlePointerUp = () => {
    setIsPanning(false);
  };

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
      })
      .catch((error) => {
        console.log(error);
      });
    if (response) {
      const words = response.data;

      const canvasSizeMultiplier = 1 + Math.sqrt(words.length) / 10;
      const canvasWidth =
        (canvasRef.current?.clientWidth || 1000) * canvasSizeMultiplier - 150;
      const canvasHeight =
        (canvasRef.current?.clientHeight || 600) * canvasSizeMultiplier - 50;

      const smallestX = words.reduce(
        (prev, cur) => (cur.x < prev ? cur.x : prev),
        0xffffffff,
      );
      const smallestY = words.reduce(
        (prev, cur) => (cur.y < prev ? cur.y : prev),
        0xffffffff,
      );

      if (smallestX < 0) {
        words.forEach((word) => {
          word.x -= smallestX;
        });
      }

      if (smallestY < 0) {
        words.forEach((word) => {
          word.y -= smallestY;
        });
      }

      let biggestX = words.reduce(
        (prev, cur) => (cur.x > prev ? cur.x : prev),
        -0xffffffff,
      );
      let biggestY = words.reduce(
        (prev, cur) => (cur.y > prev ? cur.y : prev),
        -0xffffffff,
      );

      words.forEach((word) => {
        word.x /= biggestX;
        word.x *= canvasWidth;
      });
      words.forEach((word) => {
        word.y /= biggestY;
        word.y *= canvasHeight;
      });

      words.forEach((word1) => {
        words.forEach((word2) => {
          const dx = word2.x - word1.x;
          const dy = word2.y - word1.y;
          const distance = Math.sqrt(dx * dx + dy * dy);
          const minDistance = 100;

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
  }, []);

  const addWord = async (word: string) => {
    const response = await axios.put(
      import.meta.env.VITE_API_URL + "/vocab/add/" + word,
      {},
      {
        headers: { Authorization: `Bearer ${token}` },
      },
    );

    update();
  };

  const addWordFromSelected = async (words: string[]) => {
    const response = await axios.post(
      import.meta.env.VITE_API_URL + "/vocab/add",
      { words },
      {
        headers: { Authorization: `Bearer ${token}` },
      },
    );

    toast(response.data.message);
    update();
  };

  return (
    <div className="flex h-screen flex-col bg-background touch-none">
      <SearchBar onAddWord={addWord} />
      <Toaster />

      <div className="flex flex-1 overflow-hidden">
        <div
          className="flex-1 relative overflow-hidden touch-none"
          ref={canvasRef}
          onPointerUp={handlePointerUp}
          onPointerDown={handlePointerDown}
          onPointerMove={handlePointerMove}
          onPointerLeave={handlePointerUp}
          style={{ cursor: isPanning ? "grabbing" : "grab" }}
        >
          <div
            style={{
              pointerEvents: "none",
              transform: `translate(${camera.x}px, ${camera.y}px) scale(${camera.zoom})`,
              transformOrigin: "0 0",
              width: "100%",
              height: "100%",
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
        </div>

        <DefinitionPanel
          selectedWords={words.filter((w) => selectedWords.has(w.word))}
        />
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
