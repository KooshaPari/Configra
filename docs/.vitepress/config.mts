import { defineConfig } from 'vitepress'

// Minimal scaffold. Extend nav/sidebar as docs grow.
export default defineConfig({
  title: 'Configra',
  description: 'Configra documentation',
  base: process.env.GITHUB_PAGES === 'true' ? '/Configra/' : '/',
  cleanUrls: true,
  lastUpdated: true,
  ignoreDeadLinks: true,
  themeConfig: {
    nav: [
      { text: 'Home', link: '/' },
      { text: 'Guides', link: '/guides/' },
      { text: 'Reference', link: '/reference/' },
      { text: 'API', link: '/api' },
      { text: 'Roadmap', link: '/roadmap' },
    ],
    sidebar: [
      {
        text: 'Overview',
        items: [
          { text: 'Introduction', link: '/' },
          { text: 'Development Guide', link: '/development-guide' },
          { text: 'Document Index', link: '/document-index' },
          { text: 'Wiki', link: '/wiki' },
        ],
      },
      {
        text: 'Governance',
        items: [
          { text: 'Policy Gates & Review Bots', link: '/policy-gates-and-review-bots' },
          { text: 'Polyrepo Governance Blueprint', link: '/polyrepo-governance-blueprint' },
          { text: 'Release Funnel Governance', link: '/release-funnel-governance' },
          { text: 'Container Migration Plan', link: '/container-migration-plan' },
        ],
      },
      { text: 'Guides', link: '/guides/' },
      { text: 'Reference', link: '/reference/' },
      { text: 'Compatibility', link: '/compatibility/' },
      { text: 'Roadmap', link: '/roadmap' },
    ],
    socialLinks: [
      { icon: 'github', link: 'https://github.com/KooshaPari/Configra' },
    ],
    search: { provider: 'local' },
  },
})
