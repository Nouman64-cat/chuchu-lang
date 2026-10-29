import type {ReactNode} from 'react';
import Layout from '@theme/Layout';
import Link from '@docusaurus/Link';
import CodeBlock from '@theme/CodeBlock';
import useBaseUrl from '@docusaurus/useBaseUrl';
import styles from './index.module.css';

const example = `function square(x: int) -> int {
    return x * x
}

assign numbers to [1, 2, 3, 4]
assign total to sum(numbers)

show square(total)`;

const learningPaths = [
  {
    number: '01', label: 'GET STARTED', title: 'From zero to hello.',
    description: 'Set up Cheeku, write your first .chuchu file, and run a native program of your own.',
    href: '/docs/getting-started/installation', action: 'Install the compiler', icon: '↗', tone: 'green',
  },
  {
    number: '02', label: 'THE LANGUAGE', title: 'Find the right words.',
    description: 'Variables, types, functions, and arrays. Get familiar with the small vocabulary behind your code.',
    href: '/docs/language/variables', action: 'Explore the reference', icon: '{ }', tone: 'yellow',
  },
  {
    number: '03', label: 'THE COMPILER', title: 'Meet Cheeku.',
    description: 'Go from source to executable. Learn the CLI, choose an optimization level, and look under the hood.',
    href: '/docs/tooling/cli', action: 'Open the compiler guide', icon: '>_', tone: 'pink',
  },
];

const topics = [
  {label: 'Variables & types', description: 'A little less ceremony.', href: '/docs/language/types', symbol: 'Aa'},
  {label: 'Functions', description: 'Small pieces. Clear contracts.', href: '/docs/language/functions', symbol: 'ƒ'},
  {label: 'Arrays & built-ins', description: 'Make your numbers work.', href: '/docs/language/arrays', symbol: '[ ]'},
  {label: 'Errors & diagnostics', description: 'Find your way back to working.', href: '/docs/guides/errors', symbol: '!'},
];

function Arrow(): ReactNode {
  return <span className={styles.arrow} aria-hidden="true">→</span>;
}

export default function Home(): ReactNode {
  const logo = useBaseUrl('/img/chuchu-logo-3.png');

  return (
    <Layout
      title="Simple syntax. Your native journey starts here."
      description="The Chuchu documentation. Install the Cheeku compiler, write your first native program, and explore readable syntax, inferred types, functions, arrays, and LLVM optimization."
      wrapperClassName="chuchu-docs-home">
      <main className={styles.home}>
        <div className={styles.ambient} aria-hidden="true">
          <div className={styles.gridpaper} />
          <div className={`${styles.orb} ${styles.orbGreen}`} />
          <div className={`${styles.orb} ${styles.orbYellow}`} />
          <div className={`${styles.orb} ${styles.orbPink}`} />
          <i className={`${styles.pixel} ${styles.pixelOne}`} />
          <i className={`${styles.pixel} ${styles.pixelTwo}`} />
          <i className={`${styles.pixel} ${styles.pixelThree}`} />
        </div>

        <section className={`${styles.container} ${styles.hero}`} aria-labelledby="hero-title">
          <div className={styles.heroCopy}>
            <p className={styles.badge}>
              <span className={styles.statusDot} aria-hidden="true" />
              THE CHUCHU DOCS
              <span className={styles.badgeVersion}>v0.1 · early development</span>
            </p>
            <h1 id="hero-title" className={styles.heroTitle}>
              Simple syntax.<br />
              <span className={styles.accent}>A native <span className={styles.underline}>start.
                <svg viewBox="0 0 240 14" preserveAspectRatio="none" aria-hidden="true">
                  <path d="M3 9 C28 1 52 15 78 7 S130 3 156 9 S210 13 237 5" />
                </svg>
              </span></span>
            </h1>
            <p className={styles.heroLead}>
              A little curiosity. A few readable lines. Everything you need to
              turn your next idea into a native program with Chuchu.
            </p>
            <div className={styles.actions}>
              <Link className={styles.primary} to="/docs/getting-started/installation">
                Start building <Arrow />
              </Link>
              <Link className={styles.secondary} to="/docs/intro">
                Explore the docs <Arrow />
              </Link>
            </div>
            <div className={styles.heroNote}>
              <span>Built in Rust</span><i aria-hidden="true" />
              <span>Powered by LLVM</span><i aria-hidden="true" />
              <span>Open source</span>
            </div>
          </div>

          <div className={styles.heroVisual}>
            <img className={styles.mascot} src={logo} width="140" height="84" alt="" />
            <span className={`${styles.floatingChip} ${styles.checkedChip}`}>
              <span aria-hidden="true">✓</span> Types checked
            </span>
            <span className={`${styles.floatingChip} ${styles.nativeChip}`}>
              <span className={styles.statusDot} aria-hidden="true" /> LLVM → native
            </span>
            <div className={styles.editor}>
              <div className={styles.editorBar}>
                <span className={styles.traffic} aria-hidden="true"><i /><i /><i /></span>
                <span>hello.chuchu</span>
                <span className={styles.editorLanguage}>CHUCHU</span>
              </div>
              <div className={styles.editorCode}>
                <CodeBlock language="chuchu" showLineNumbers>{example}</CodeBlock>
              </div>
              <div className={styles.terminal}>
                <div><span className={styles.prompt}>$</span> cheeku hello.chuchu -o hello</div>
                <div><span className={styles.prompt}>$</span> ./hello</div>
                <div className={styles.terminalResult}>100 <span>just like you meant it.</span></div>
              </div>
            </div>
            <p className={styles.editorCaption}>Readable in. Native out.</p>
          </div>
        </section>

        <div className={`${styles.container} ${styles.essentials}`} aria-label="Language essentials">
          <span>SMALL LANGUAGE. REAL BUILDING BLOCKS.</span>
          <ul>
            {['Inferred types', 'Functions', 'Numeric arrays', 'Runtime checks', 'O0–O3'].map((label) => (
              <li key={label}><i aria-hidden="true" />{label}</li>
            ))}
          </ul>
        </div>

        <section className={`${styles.container} ${styles.section}`} aria-labelledby="paths-title">
          <div className={styles.sectionHeading}>
            <div>
              <p className={styles.eyebrow}><span aria-hidden="true" />01 · FIND YOUR WAY</p>
              <h2 id="paths-title">Your next step,<br />right here.</h2>
            </div>
            <p>Just getting started or looking something up?<br className={styles.desktopBreak} /> There’s a good place for both.</p>
          </div>
          <div className={styles.pathGrid}>
            {learningPaths.map((path) => (
              <Link key={path.number} to={path.href} className={`${styles.pathCard} ${styles[path.tone]}`}>
                <div className={styles.pathTop}>
                  <span className={styles.pathIcon} aria-hidden="true">{path.icon}</span>
                  <span className={styles.pathNumber}>{path.number}</span>
                </div>
                <span className={styles.cardLabel}>{path.label}</span>
                <h3>{path.title}</h3>
                <p>{path.description}</p>
                <span className={styles.cardAction}>{path.action}<Arrow /></span>
              </Link>
            ))}
          </div>
        </section>

        <section className={`${styles.container} ${styles.section}`} aria-labelledby="quickstart-title">
          <div className={styles.quickstart}>
            <div className={styles.quickstartCopy}>
              <p className={styles.eyebrow}><span aria-hidden="true" />02 · A LITTLE CHUCHU</p>
              <h2 id="quickstart-title">Looks like a thought.<br />Runs like a program.</h2>
              <p>Declare with <code>assign</code>. Update with <code>reassign</code>.
                Say it with <code>show</code>. The compiler takes care of the types.</p>
              <Link className={styles.textLink} to="/docs/getting-started/first-program">
                Write your first program <Arrow />
              </Link>
            </div>
            <div className={styles.firstProgram}>
              <div className={styles.snippetLabel}><span aria-hidden="true">✳</span> A SMALL VOCABULARY</div>
              <CodeBlock language="chuchu">{'assign count to 5\nreassign count to count + 1\nshow count'}</CodeBlock>
              <div className={styles.resultRow}><span>OUTPUT</span><code>6</code><span>One small step. A working program.</span></div>
            </div>
          </div>
        </section>

        <section className={`${styles.container} ${styles.section}`} aria-labelledby="reference-title">
          <div className={styles.sectionHeading}>
            <div>
              <p className={styles.eyebrow}><span aria-hidden="true" />03 · KEEP EXPLORING</p>
              <h2 id="reference-title">A reference for<br />your next “what if?”</h2>
            </div>
            <Link className={styles.textLink} to="/docs/guides/examples">See working examples <Arrow /></Link>
          </div>
          <div className={styles.referenceGrid}>
            {topics.map((topic) => (
              <Link key={topic.label} to={topic.href} className={styles.topicCard}>
                <span className={styles.topicIcon} aria-hidden="true">{topic.symbol}</span>
                <div><h3>{topic.label}</h3><p>{topic.description}</p></div>
                <Arrow />
              </Link>
            ))}
          </div>
          <div className={styles.compilerCard}>
            <div>
              <span className={styles.cardLabel}>FOR THE CURIOUS</span>
              <h3>There’s a whole compiler in here.</h3>
              <p>Follow your code from its first token to LLVM IR and a native executable.</p>
            </div>
            <div className={styles.pipeline} aria-label="Compiler pipeline">
              <span>source</span><Arrow /><span>LLVM</span><Arrow /><span>native</span>
            </div>
            <Link className={styles.textLink} to="/docs/project/architecture">Look under the hood <Arrow /></Link>
          </div>
        </section>

        <section className={`${styles.container} ${styles.closing}`} aria-labelledby="closing-title">
          <div className={styles.closingPanel}>
            <span className={styles.closingStar} aria-hidden="true">✳</span>
            <div>
              <p className={styles.cardLabel}>CURIOSITY LOOKS GOOD ON YOU</p>
              <h2 id="closing-title">Make something<br />a little more native.</h2>
              <p>Your first Chuchu program is a few readable lines away.</p>
              <div className={styles.actions}>
                <Link className={styles.primary} to="/docs/getting-started/installation">Let’s get started <Arrow /></Link>
                <Link className={styles.textLink} to="/docs/project/status">What works today <Arrow /></Link>
              </div>
            </div>
            <div className={styles.closingNote}>
              <span className={styles.statusDot} aria-hidden="true" />
              <span>A small language, still growing.<br />Follow the docs as Chuchu evolves.</span>
            </div>
          </div>
        </section>
      </main>
    </Layout>
  );
}
