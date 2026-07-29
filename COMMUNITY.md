# Community

`docklab` is small, young, and built for a community that spans complete
beginners taking their first CVE for a spin and experienced pentesters
adding labs for CVEs they know cold. This doc covers the code of conduct,
how the community operates day to day, and the ethics baseline that comes
with building an offensive-security tool in the open.

## Code of Conduct

### Our pledge

We want participation in `docklab` — issues, PRs, discussions, new labs —
to be a harassment-free experience for everyone, regardless of experience
level, background, identity, or how new they are to security work. A
question like "what does JNDI even mean" is exactly as welcome as a PR
adding a new CVE lab.

### Expected behavior

- Be respectful of differing skill levels. Most people arrive here to
  *learn* offensive security, not because they already know it.
- Give specific, actionable feedback on PRs and labs — "this lab's
  `Dockerfile` doesn't pin a base image digest, so it'll drift" beats "this
  is wrong."
- Assume good faith. Someone asking how to exploit a bundled CVE lab is
  learning; someone asking how to point docklab at a system they don't own
  is a conversation to redirect, not assume malice about (see
  [SECURITY.md](SECURITY.md#responsible-use-of-this-tool)).
- Credit sources. If a lab's exploit technique or hint text is adapted from
  a public writeup, link it in the PR description.

### Unacceptable behavior

- Harassment, personal attacks, or discriminatory language/jokes in any
  project space (issues, PRs, discussions, commit messages).
- Sharing someone's private information without consent.
- Using `docklab`'s issue tracker to request help attacking systems you
  don't own or lack authorization to test — that's out of scope for this
  project's support surface entirely, not just a conduct violation.
- Submitting a "CVE lab" that isn't actually a self-contained reproduction
  of a real CVE (see [CONTRIBUTING.md](CONTRIBUTING.md#about-labs) for what
  qualifies).

### Enforcement

Report conduct issues to **tanjibisham888@gmail.com**. Reports are handled
privately. Depending on severity, outcomes range from a private conversation
to removal from the project's spaces. Maintainers apply this consistently,
including to themselves.

This Code of Conduct is adapted from the spirit of the
[Contributor Covenant](https://www.contributor-covenant.org/), trimmed to
fit a small project.

## How the community operates

- **Issues** are for bugs in the CLI, a broken/incorrect lab, or a
  well-scoped feature request. Use [SECURITY.md](SECURITY.md) instead for
  anything that breaks the isolation guarantee.
- **Discussions** (or issues tagged `question`, until a Discussions board
  exists) are for "how do I..." and "why does docklab do X" questions.
  There's no such thing as a question too basic — this tool's whole point
  is lowering the barrier to practicing offensive security safely.
- **Pull requests** are the main way both the CLI and the lab content grow.
  See [CONTRIBUTING.md](CONTRIBUTING.md) for the mechanics; for a new CVE
  lab the short version is: one CVE per PR, a `metadata.yaml` with an
  honest difficulty rating, and a Dockerfile that builds hermetically (no
  dependency on a PoC's original, possibly-dead infrastructure).
- **Maintainer response time** is best-effort — this is presently a
  single-maintainer project. Pings after a week of silence on a PR are
  completely fair game.

## Ethics baseline

Offensive security tooling built in the open comes with a baseline
expectation, spelled out plainly rather than left implicit:

- **Authorization is non-negotiable.** Every technique this project
  teaches is for systems you own or are explicitly authorized to test —
  a lab environment, a CTF, a pentest engagement with signed scope. Never
  systems you don't have permission to touch.
- **The bundled CVEs are old and public on purpose.** CVE-2011-2523,
  CVE-2014-6271, and CVE-2021-44228 are years-old, extensively documented,
  patched-for-ages vulnerabilities. Teaching how they work publicly
  serves defenders and learners; it doesn't meaningfully arm attackers who
  could already find better writeups with one search.
- **New labs follow the same bar.** A contributed lab should teach a
  *technique* (a vulnerability class, an exploitation pattern), not just
  package the latest 0-day for casual reproduction. See
  [CONTRIBUTING.md](CONTRIBUTING.md#about-labs) for what a lab needs.
- **Isolation is aspirational as much as it is real, and that's said
  plainly.** Today `lab` gives you a disposable, non-bind-mounted
  container — real value, but not a hardened sandbox: there's no
  per-session network isolation, capability dropping, or resource limiting
  yet. Don't oversell what the tool currently does when writing docs,
  labs, or hints. Contributions that move the isolation story forward
  (see the "most-wanted" note in [CONTRIBUTING.md](CONTRIBUTING.md#about-labs))
  are very welcome; contributions that quietly assume it already exists
  (e.g. a lab that only makes sense with network isolation) will get
  pushed back on until that groundwork lands.

## Getting involved

- First contribution idea: pick a CVE you already understand well and add
  it as a new lab — the existing three (`labs/CVE-2011-2523`,
  `labs/CVE-2014-6271`, `labs/CVE-2021-44228`) are good references for the
  shape a lab takes.
- Found a rough edge in the CLI (a confusing error, a missing `--help`
  string)? Small UX PRs are genuinely high-value here — this tool is
  judged on how smooth the first five minutes feel.
- Not a coder? Trying the [TUTORIAL.md](TUTORIAL.md) as a true newcomer and
  filing an issue for every point of confusion is just as valuable as a PR.
