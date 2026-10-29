# Phase 8 collaboration and integration

All developers use `chuchu-lang/chuchu-core`. Keep feature branches temporary;
merge reviewed, tested work into main. No permanent OS forks are needed.

| Developer | Ownership | Suggested temporary branch |
|---|---|---|
| A: macOS/core | Shared interfaces, semantics, LLVM target policy, macOS regression review | `phase8/portable-core` |
| B: Windows | clang-cl/MSVC, CRT ABI, filenames, filesystem behavior, Windows CI | `phase8/windows-support` |
| C: Linux | libm/ELF, dependencies, Linux runtime tests, portable benchmarks | `phase8/linux-support` |
| Integration | Shared CI and distribution validation | `phase8/cross-platform-ci` |
| Documentation | OS guides and release runtime evidence | `phase8/documentation-release` |

Agree on `link_object`, default output policy, and runtime function signatures
before overlapping changes to native.rs/runtime.rs. Recommended review order:
portable core and tests, Linux, Windows, CI, then release documentation/evidence.
These are recommended branches; the implementation does not create or claim to
have merged them automatically.

Require the four `Cross-platform native compiler` target checks for main's branch
protection after observing their real check names in GitHub. Do not bypass failing
OS jobs, use continue-on-error, or treat one host's test pass as another's proof.
Tag releases only after integration checks pass; the release workflow rebuilds and
retests the tagged revision before preparing draft archives.
