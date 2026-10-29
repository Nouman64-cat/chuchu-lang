import original from '@theme-original/prism-include-languages';
export default function includeLanguages(Prism: typeof import('prismjs')) {
  original(Prism);
  Prism.languages.chuchu = {
    keyword: /\b(?:assign|show|reassign|if|else|while|function|return)\b/,
    boolean: /\b(?:true|false)\b/,
    'class-name': /\b(?:int|float|bool)\b/,
    builtin: /\b(?:abs|sqrt|min|max|sum|len)(?=\s*\()/,
    function: /\b[a-zA-Z_]\w*(?=\s*\()/,
    number: /\b\d+(?:\.\d*)?(?:[eE][+-]?\d+)?\b/,
    operator: /->|==|!=|<=|>=|[+*/=<>-]/,
    punctuation: /[{}[\](),:]/,
  };
}
