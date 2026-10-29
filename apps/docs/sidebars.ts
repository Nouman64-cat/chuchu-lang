import type {SidebarsConfig} from '@docusaurus/plugin-content-docs';
const sidebars: SidebarsConfig = {
  docsSidebar: [
    'intro',
    {type: 'category', label: 'Getting started', collapsed: false, items: ['getting-started/installation', 'getting-started/first-program']},
    {type: 'category', label: 'Language reference', collapsed: false, items: ['language/variables', 'language/types', 'language/operators', 'language/control-flow', 'language/functions', 'language/arrays', 'language/builtins']},
    {type: 'category', label: 'Guides', items: ['guides/examples', 'guides/errors']},
    {type: 'category', label: 'Compiler & tooling', items: ['tooling/cli', 'tooling/optimization', 'tooling/performance']},
    {type: 'category', label: 'About the project', items: ['project/status', 'project/architecture', 'project/contributing']},
  ],
};
export default sidebars;
