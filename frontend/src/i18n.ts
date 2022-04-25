import Module from "module";
import { createI18n, VueI18n } from "vue-i18n";
import en from "./locales/en.json";

export default createI18n({
  locale: "en",
  fallbackLocale: "en",
  messages: { en: en },
});

export function changeLang(i18n: VueI18n, lang: string) {
  const loaded_langs = Object.keys(i18n.messages);
  if (!loaded_langs.includes(lang)) {
    import(`./locales/${lang}.json`).then(
      (file) => {
        (i18n.messages as { [key: string]: Module })[lang] = file;
        i18n.locale = lang;
      },
      (e) => {
        console.error(`Could not load language "${lang}": ${e}`);
      }
    );
    return;
  }
  i18n.locale = lang;
}
