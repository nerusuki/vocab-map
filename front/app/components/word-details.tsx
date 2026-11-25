"use client";

import { use } from "react";
import { useLanguage, type Language } from "./language-select.client";
import axios from "axios";

interface Vocab {
  id: number;
  word: string;
  lang: Language;
  reading?: string;
  definition?: string[];
  tags: string[];
}

async function _fetchVocab(word: string, lang: Language) {
  return await axios
    .get<Vocab[]>(import.meta.env.VITE_API_URL + "/vocab/full/" + word, {
      params: { lang },
    })
    .then((response) => response.data[0])
    .catch(() => null);
}

const promiseCache = new Map<string, ReturnType<typeof _fetchVocab>>();

function fetchVocab(word: string, lang: Language) {
  const key = `${lang}:${word}`;
  let promise = promiseCache.get(key);

  if (!promise) {
    promise = _fetchVocab(word, lang);
    promiseCache.set(key, promise);
  }

  return promise;
}

export function WordDetails({ word }: { word: string }) {
  const lang = useLanguage?.();
  const vocab = use(fetchVocab(word, lang));

  if (!vocab) {
    return (
      <p className="text-sm text-muted-foreground">Definition not found</p>
    );
  }

  return (
    <div className="space-y-3">
      <div>
        <h3 className="text-xl font-semibold text-foreground">{word}</h3>
        {vocab.reading && (
          <p className="text-sm text-muted-foreground">{vocab.reading}</p>
        )}
      </div>

      <div className="space-y-4">
        {vocab.definition?.slice(0, 4).map((definition, idx) => (
          <div key={idx} className="space-y-2">
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
    </div>
  );
}
