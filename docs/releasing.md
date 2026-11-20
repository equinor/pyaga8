# Releasing

Releases are made by the workflow `.github/workflows/release.yml`, started by hand from `main`.
It creates the git tag, uploads the files to [PyPI](https://pypi.org/project/pyaga8/) with
[Trusted Publishing](https://docs.pypi.org/trusted-publishers/) (no API token), and creates the
GitHub release. Nothing is released by merging, pushing, or creating a tag or a GitHub release by
hand.

## Choosing the version

pyaga8 takes its version from `Cargo.toml` (`[package] version`). Releases follow
[semantic versioning](https://semver.org/) and are tagged with the bare version, for example
`0.2.0` (no `v` prefix, like the existing tags):

| Change                                                                  | Increase | Example         |
|-------------------------------------------------------------------------|----------|-----------------|
| Bug fix that does not change any calculated result, packaging fixes     | PATCH    | 0.2.0 -> 0.2.1  |
| New functionality, new or dropped Python versions or wheel platforms    | MINOR    | 0.2.1 -> 0.3.0  |
| Incompatible change of the Python API                                   | MAJOR    | 0.3.0 -> 1.0.0  |

Project rule: any change that changes a calculated result (for example a new version of the
`aga8` crate that changes numbers) is at least MINOR, also when it is a correction.

## Steps

1. In a pull request, set the new version in `Cargo.toml` and run
   `cargo update -p pyaga8 --offline` so that `Cargo.lock` records it (all builds use `--locked`
   and fail if `Cargo.lock` is out of date). Merge the pull request when the checks are green.
2. Actions -> **Release** -> **Run workflow**, branch `main`. There are no inputs.
3. When the tests have passed, approve the deployment to the environment `pypi`. This is the
   only approval; it creates the tag and uploads to PyPI.

## What the workflow does

All jobs work on the commit the run was started on.

1. **prepare** - fails unless the run is on `main`; checks with `tools/release_check.py` that the
   `Cargo.toml` version is exactly `MAJOR.MINOR.PATCH` and strictly higher than the highest
   existing release tag (compared numerically, so `0.1.10` is higher than `0.1.9`); fails if the
   tag exists or if PyPI already has that version (`https://pypi.org/pypi/pyaga8/<version>/json`
   must return 404).
2. **test** - runs the full test workflow `tests.yml`. It builds the 20 wheels (CPython
   3.10-3.14 for Linux x86_64 and aarch64 (manylinux2014), Windows x64 and macOS arm64) and the
   sdist once, with the committed `Cargo.lock`. It installs every wheel, and the sdist from an
   unpacked copy without `.git`, into clean virtual environments and runs the tests against them,
   checks that exactly the expected 21 files exist and runs `twine check --strict`.
3. **build** - takes exactly those tested files, checks that their names carry the release
   version, runs `twine check --strict` again and writes `SHA256SUMS`. Artifacts `release-dist`
   and `release-manifest` (kept 90 days).
4. **publish** - after approval in environment `pypi`, in this order: checks that the file names
   equal the entries of `SHA256SUMS` exactly and that every hash matches; checks what PyPI already
   has for the version (see below); creates the lightweight tag `<version>` at the released commit
   (a tag that already exists is accepted only on a re-run and only if it points at the released
   commit); uploads with `pypa/gh-action-pypi-publish` (Trusted Publishing, with PyPI
   attestations); then waits until PyPI lists exactly the files and SHA-256 hashes of
   `SHA256SUMS`. This job has only the permissions `contents: write` (for the tag) and
   `id-token: write` (for PyPI), and runs no project code (no checkout).
5. **github-release** - creates the GitHub release `<version>` with generated notes and the same
   21 files attached.

## When something fails

- Before the publish job: nothing has been tagged or published. Fix the cause in a pull request
  and start a new run.
- From the publish job on: use **Re-run failed jobs** on the same run, nothing else. A re-run accepts
  the tag only if it points at the released commit, and accepts files already on PyPI only if each
  one is in `SHA256SUMS` of this run with the same hash and has a PyPI provenance from this
  repository's `release.yml` and environment `pypi`; the missing files are then uploaded.
- Never start a new run for a version that already has files on PyPI. A file name that has been
  uploaded to PyPI once can never be uploaded again, even after it is deleted, and a new run
  builds files with other hashes. Release the next PATCH version instead.
- Never move or delete a published tag.

## One-time setup (repository owner)

- Settings -> Environments: environment `pypi` with required reviewer `chagenvik` and deployment
  branches limited to `main`.
- pypi.org -> pyaga8 -> Manage -> Publishing: a GitHub trusted publisher with owner `equinor`,
  repository `pyaga8`, workflow `release.yml`, environment `pypi`.
- Optional: a tag ruleset on `refs/tags/[0-9]*` that blocks update and deletion of release tags.
