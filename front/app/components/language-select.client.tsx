"use client";

import { Select, SelectContent, SelectItem, SelectTrigger } from "./ui/select";
import { useLocalStorage } from "@uidotdev/usehooks";

const LANGUAGES = {
  En: "English",
} as const;

export type Language = keyof typeof LANGUAGES;

const LANGUAGE_OPTIONS = (Object.keys(LANGUAGES) as Language[]).map((key) => ({
  value: key,
  label: LANGUAGES[key],
}));

const STORAGE_KEY = "language";
const DEFAULT_LANGUAGE: Language = "En";

export const useLanguage = () =>
  useLocalStorage<Language>(STORAGE_KEY, DEFAULT_LANGUAGE)[0];

export function LanguageSelect() {
  const [lang, setLang] = useLocalStorage<Language>(
    STORAGE_KEY,
    DEFAULT_LANGUAGE,
  );

  return (
    <Select value={lang} onValueChange={(value: Language) => setLang(value)}>
      <SelectTrigger>
        Language: <b>{LANGUAGES[lang]}</b>
      </SelectTrigger>
      <SelectContent>
        {LANGUAGE_OPTIONS.map(({ value, label }) => (
          <SelectItem key={value} value={value}>
            {label}
          </SelectItem>
        ))}
      </SelectContent>
    </Select>
  );
}
