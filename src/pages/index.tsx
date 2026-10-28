import type {ReactNode} from 'react';
import Layout from '@theme/Layout';
import Link from '@docusaurus/Link';
import CodeBlock from '@theme/CodeBlock';
import styles from './index.module.css';

const example = `function square(x: int) -> int {
    return x * x
}

assign numbers to [1, 2, 3, 4]
assign total to sum(numbers)

show square(total)`;
const paths = [
  ['01', 'Your first Chuchu program', 'Install the compiler. Write a little code. Make something run.', '/docs/getting-started/installation', 'Start here'],
  ['02', 'Meet the language', 'Explore types, functions, control flow, and numerical arrays.', '/docs/language/variables', 'Read the reference'],
  ['03', 'Go a little deeper', 'Understand native compilation, optimization, and runtime checks.', '/docs/project/architecture', 'Under the hood'],
];
export default function Home(): ReactNode {
  return (
    <Layout title="A small language. A native foundation." description="Learn Chuchu, a statically typed programming language built in Rust with an LLVM native compiler. Installation, tutorials, language reference, and working examples.">
      <main>
        <section className={styles.hero}>
          <div className={styles.heroCopy}>
            <div className={styles.eyebrow}><span className={styles.dot} /> THE CHUCHU LANGUAGE <span className={styles.version}>v0.1 · early development</span></div>
            <h1>A little simpler.<br />A lot to <span>explore.</span></h1>
            <p className={styles.lead}>Readable syntax. Inferred types. Native compilation.<br className={styles.desktopBreak} /> Meet Chuchu, a small language for curious builders.</p>
            <div className={styles.actions}>
              <Link className={styles.primary} to="/docs/getting-started/installation">Start building <span aria-hidden="true">↗</span></Link>
              <Link className={styles.secondary} to="/docs/intro">Explore the docs <span aria-hidden="true">→</span></Link>
            </div>
            <p className={styles.heroNote}>Built in Rust. Powered by LLVM. Open source.</p>
          </div>
          <div className={styles.codeWindow}>
            <div className={styles.windowBar}><div className={styles.traffic} aria-hidden="true"><i /><i /><i /></div><span>hello.chuchu</span><span className={styles.codeLabel}>CHUCHU</span></div>
            <div className={styles.codeContent}><CodeBlock language="chuchu" showLineNumbers>{example}</CodeBlock></div>
            <div className={styles.output}><span className={styles.outputLabel}>OUTPUT</span><code>100</code><span className={styles.outputStatus}><span aria-hidden="true">✓</span> native executable</span></div>
          </div>
        </section>
        <section className={styles.principles} aria-label="Language essentials">
          <div><span className={styles.smallIcon} aria-hidden="true">Aa</span><h2>Read it like a thought.</h2><p><code>assign</code>, <code>to</code>, <code>show</code>. A small vocabulary that keeps your ideas in focus.</p></div>
          <div><span className={styles.smallIcon} aria-hidden="true">{ '{ }' }</span><h2>Simple, with structure.</h2><p>Inferred local types. Explicit function signatures. Checked arithmetic and array access.</p></div>
          <div><span className={styles.smallIcon} aria-hidden="true">↗</span><h2>From source to native.</h2><p>An LLVM compiler with four optimization levels, from an unoptimized baseline to O3.</p></div>
        </section>
        <section className={styles.learnSection}>
          <div className={styles.sectionHeading}><div><span className={styles.kicker}>ONE STEP AT A TIME</span><h2>Find your starting point.</h2></div><p>From your first line to the details<br />that make the language work.</p></div>
          <div className={styles.pathGrid}>{paths.map(([number, title, description, href, action]) => <Link key={number} to={href} className={styles.pathCard}><span className={styles.pathNumber}>{number}</span><h3>{title}</h3><p>{description}</p><span className={styles.cardAction}>{action}<span aria-hidden="true">↗</span></span></Link>)}</div>
        </section>
        <section className={styles.projectNote}><div><span className={styles.noteStar} aria-hidden="true">✳</span></div><div><h2>Small language. Open possibilities.</h2><p>Chuchu is in early development. Explore what works today, follow the implementation, or help shape what comes next.</p></div><Link to="/docs/project/status">See language status <span aria-hidden="true">→</span></Link></section>
      </main>
    </Layout>
  );
}
