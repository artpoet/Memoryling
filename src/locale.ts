import { useEffect, useState } from "react";

export type Locale = "en" | "zh-TW";
export const LOCALE_STORAGE_KEY = "memoryling:locale";

export function getInitialLocale(): Locale {
  const requested = new URLSearchParams(window.location.search).get("lang");
  if (requested === "en" || requested === "zh-TW") return requested;
  const saved = window.localStorage.getItem(LOCALE_STORAGE_KEY);
  if (saved === "en" || saved === "zh-TW") return saved;
  return navigator.language.toLowerCase().startsWith("zh") ? "zh-TW" : "en";
}

export function useStoredLocale() {
  const [locale, setLocale] = useState<Locale>(getInitialLocale);

  useEffect(() => {
    document.documentElement.lang = locale;
    window.localStorage.setItem(LOCALE_STORAGE_KEY, locale);
  }, [locale]);

  useEffect(() => {
    function onStorage(event: StorageEvent) {
      if (event.key !== LOCALE_STORAGE_KEY) return;
      if (event.newValue === "en" || event.newValue === "zh-TW") {
        setLocale(event.newValue);
      }
    }
    window.addEventListener("storage", onStorage);
    return () => window.removeEventListener("storage", onStorage);
  }, []);

  return [locale, setLocale] as const;
}
