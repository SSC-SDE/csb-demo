# CodeShouldBehave Review — template demo

[![GitHub Marketplace](https://img.shields.io/badge/GitHub%20Marketplace-Listed-6f42c1?logo=github)](https://github.com/marketplace/actions/codeshouldbehave-review)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

A deliberately small Rust repository for trying
[CodeShouldBehave Review](https://github.com/marketplace/actions/codeshouldbehave-review).
The only integration is `.github/workflows/csb-review.yml`.

## Try it

1. Open [pull request #1](https://github.com/SSC-SDE/csb-demo/pull/1).
2. Read the **CodeShouldBehave Review** comment.
3. Download the `csb-review-1` artifact from the Actions run and open the
   interactive report.

The demo pull request changes `add`, adds `subtract`, and keeps `handle` and
`other` as lexical callers. The report shows the HLD/LLD delta and exact
baseline/proposal source.

## Review a multi-crate change

The sensor batch example reads `sensor,timestamp,value` records from stdin.
Timestamps are unsigned milliseconds; readings are signed integers. Replaying
an identical reading does not increase its count. Two different values at the
same sensor and timestamp reject the entire batch.

```sh
printf 'west,10,7\nwest,11,9\nwest,10,7\n' | cargo run --quiet --bin sensor-batch
```

The output includes `west,2,7,9,16`. Follow `process_batch` from parsing to
normalization, grouped statistics and CSV rendering. These are separate Cargo
packages so package dependencies and local function flow can be reviewed
independently. Run `cargo test --workspace` to check the pipeline.

## Use this template

Click **Use this template → Create a new repository**, or open
[this link](https://github.com/SSC-SDE/csb-demo/generate). Then open a pull
request. No token, App installation or local server is required.

```yaml
permissions:
  contents: read
  pull-requests: write

jobs:
  review:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
        with:
          fetch-depth: 0
      - uses: SSC-SDE/CodeShouldBehave@v0.7.0
        with:
          design: auto
```

## What the action does

- Runs a pinned, verified CodeShouldBehave image digest.
- Compares the exact base/head commits.
- Rechecks every report hash with `csb verify`.
- Posts or updates one pull-request summary comment.
- Uploads the interactive report as a 30-day workflow artifact.
- Reports coverage as `complete`, `incomplete` or `failed`.

Set `fail-on-incomplete: true` to block the job when coverage is incomplete.

## Scope and limits

Supported today: Rust-first Cargo repositories on Linux GitHub runners. HLD/LLD
boundaries are provisional and analysis is bounded. Unsupported files, deferred
detail and incomplete coverage are labeled, never hidden. This is an advisory
review, not a merge approval or runtime proof. Deterministic review needs no
token or network access; the runner still pulls the pinned image.

For the CLI, container, GitLab and Bitbucket templates, see the
[main repository](https://github.com/SSC-SDE/CodeShouldBehave).

## License

MIT. See [LICENSE](LICENSE).
