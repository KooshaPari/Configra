import { defineConfig } from 'vitepress';

// VitePress config for the Configra docs site.
// Deployed to GitHub Pages via .github/workflows/docs.yml.
//
// `srcDir` scopes VitePress to docs/.vitepress/src so it does not try to
// render the legacy hand-written markdown under docs/phenotype-config-absorbed/
// or docs/migrations/ (which use {{ ... }} placeholders that conflict with
// Vue's interpolation syntax).
export default defineConfig({
  title: 'Configra',
  description:
    'Configra — local-first configuration management, feature flags, ' +
    'secrets, and version tracking for Phenotype projects.',
  cleanUrls: true,
  base: '/Configra/',
  lastUpdated: true,
  srcDir: '.vitepress/src',
  themeConfig: {
    nav: [
      { text: 'Home', link: '/' },
      { text: 'Getting Started', link: '/getting-started' },
    ],
    sidebar: [
      {
        text: 'Introduction',
        items: [
          { text: 'Home', link: '/' },
          { text: 'Getting Started', link: '/getting-started' },
        ],
      },
    ],
    socialLinks: [
      { icon: 'github', link: 'https://github.com/KooshaPari/Configra' },
    ],
    footer: {
      message: 'Released under the MIT License.',
      copyright: 'Copyright (c) 2026 KooshaPari',
    },
  },
});