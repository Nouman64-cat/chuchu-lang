import type {Config} from '@docusaurus/types';
import type * as Preset from '@docusaurus/preset-classic';
import {themes as prismThemes} from 'prism-react-renderer';

const url = process.env.SITE_URL || 'https://docs.chuchu-lang.org';
const baseUrl = process.env.BASE_URL || '/';

const config: Config = {
  title: 'Chuchu',
  tagline: 'Simple syntax. Native possibilities.',
  favicon: 'img/chuchu-lang-png.png',
  url,
  baseUrl,
  organizationName: 'chuchu-lang',
  projectName: 'chuchu-lang-docs',
  trailingSlash: true,
  onBrokenLinks: 'throw',
  onBrokenAnchors: 'throw',
  i18n: {defaultLocale: 'en', locales: ['en']},
  presets: [['classic', {
    docs: {
      sidebarPath: './sidebars.ts',
      editUrl: 'https://github.com/chuchu-lang/chuchu-lang-docs/edit/main/',
      showLastUpdateTime: false,
    },
    blog: false,
    theme: {customCss: './src/css/custom.css'},
    sitemap: {changefreq: null, priority: null, ignorePatterns: ['/search/**']},
  } satisfies Preset.Options]],
  themes: [['@easyops-cn/docusaurus-search-local', {
    hashed: 'filename',
    language: ['en'],
    indexBlog: false,
    indexPages: true,
    docsRouteBasePath: '/docs',
    highlightSearchTermsOnTargetPage: true,
  }]],
  themeConfig: {
    image: 'img/chuchu-lang-png.png',
    colorMode: {defaultMode: 'light', respectPrefersColorScheme: false},
    metadata: [{name: 'theme-color', content: '#6154cc'}],
    navbar: {
      logo: {alt: 'Chuchu', src: 'img/chuchu-lang-png.png', srcDark: 'img/chuchu-lang-png.png', width: 110, height: 54},
      items: [
        {type: 'docSidebar', sidebarId: 'docsSidebar', position: 'left', label: 'Documentation'},
        {to: '/docs/guides/examples', label: 'Examples', position: 'left'},
        {to: '/docs/project/status', label: 'Language status', position: 'left'},
        {href: 'https://github.com/chuchu-lang/chuchu-core', label: 'GitHub', position: 'right'},
      ],
    },
    footer: {
      style: 'light',
      links: [
        {title: 'Learn', items: [{label: 'Get started', to: '/docs/getting-started/installation'}, {label: 'Language reference', to: '/docs/language/variables'}, {label: 'Examples', to: '/docs/guides/examples'}]},
        {title: 'Build', items: [{label: 'Compiler CLI', to: '/docs/tooling/cli'}, {label: 'Optimization', to: '/docs/tooling/optimization'}, {label: 'Compiler architecture', to: '/docs/project/architecture'}]},
        {title: 'Project', items: [{label: 'Source code', href: 'https://github.com/chuchu-lang/chuchu-core'}, {label: 'Contribute', to: '/docs/project/contributing'}, {label: 'Documentation source', href: 'https://github.com/chuchu-lang/chuchu-lang-docs'}]},
      ],
      copyright: `© ${new Date().getFullYear()} Chuchu. Compiler released under the MIT License.`,
    },
    prism: {theme: prismThemes.github, darkTheme: prismThemes.dracula, additionalLanguages: ['rust', 'bash', 'json']},
  } satisfies Preset.ThemeConfig,
};
export default config;
