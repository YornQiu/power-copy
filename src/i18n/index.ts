import { createI18n } from 'vue-i18n';

import en from './locales/en.json';
import zh from './locales/zh.json';

const lang = 'zh';

export const i18n = createI18n({
  locale: lang,
  fallbackLocale: lang,
  legacy: false,
  messages: {
    en,
    zh,
  },
});
