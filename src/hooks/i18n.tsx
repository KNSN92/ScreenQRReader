import { invoke } from "@tauri-apps/api/core";
import { error } from "@tauri-apps/plugin-log";
import {
  createContext,
  ReactNode,
  useContext,
  useEffect,
  useState,
} from "react";

const I18nContext = createContext<Record<string, string>>({});

export function I18nProvider({ children }: { children: ReactNode }) {
  const [translations, setTranslations] = useState<Record<string, string>>({});
  useEffect(() => {
    invoke<Record<string, string>>("i18n_translations")
      .then((result) => {
        setTranslations(result);
      })
      .catch((err) => {
        error(`Failed to load translations: ${err}`);
      });
  }, []);
  return (
    <I18nContext.Provider value={translations}>{children}</I18nContext.Provider>
  );
}

export function useI18n() {
  const translations = useContext(I18nContext);
  return (key: string, arg: string | null = null) => {
    if (!translations[key]) {
      error(`Missing translation for key: ${key}`);
    }
    if (arg) {
      return translations[key]?.replace("{}", arg) || key;
    } else {
      return translations[key] || key;
    }
  };
}
