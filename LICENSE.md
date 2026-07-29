MIT License

Copyright (c) 2026 MrEchoFi

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to
deal in the Software without restriction, including without limitation the
rights to use, copy, modify, merge, publish, distribute, sublicense, and/or
sell copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in
all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING
FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
DEALINGS IN THE SOFTWARE.

---

## A note on scope

This license covers the `docklab` CLI (the Rust source in `src/`) and the
lab reference material in this repository.

The bundled CVE labs under `labs/*/docker/` intentionally reproduce
historical vulnerabilities (a backdoored vsftpd 2.3.4, a Shellshock-vulnerable
CGI/Bash-style interpreter, and a genuinely vulnerable log4j-core 2.14.1
build) for educational and lab purposes. Those vulnerable versions are
included/pinned deliberately and are not "bugs" to be fixed — see
[SECURITY.md](SECURITY.md) for how vulnerability reports are triaged in
this repo. Third-party components referenced or pulled by those
Dockerfiles and by `lab create` itself (e.g. `kalilinux/kali-rolling`,
`debian:12-slim`, `python:3.11-alpine`, `maven`/`eclipse-temurin` base
images, upstream `log4j-core` artifacts) remain under their own respective
licenses.
