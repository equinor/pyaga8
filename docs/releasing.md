# Releasing

Releases are published to [PyPI](https://pypi.org/project/pyaga8/) by the workflow
`.github/workflows/release.yml` when a GitHub release is published.

## Steps

1. In a pull request, set the new version in `Cargo.toml` (`[package] version`) and run
   `cargo update --workspace` so that `Cargo.lock` records it (CI builds with `--locked` and
   fails if `Cargo.lock` is out of date). The Python package takes its version from
   `Cargo.toml`. Merge the pull request when the checks are green.
2. On GitHub, create a release from `main` with a new tag equal to the version, for example
   `0.2.0`, and publish it.

## What the workflow does

1. Checks that the release tag (without a leading `v`) equals the version in `Cargo.toml`.
   If not, the workflow stops and nothing is uploaded.
2. Runs the full test workflow `tests.yml`. It builds the 20 wheels (CPython 3.10-3.14 for
   Linux x86_64 and aarch64 (manylinux2014), Windows x64 and macOS arm64) and the sdist once,
   with the committed `Cargo.lock` (`--locked`). It installs every wheel, and the sdist from an
   unpacked copy, into clean virtual environments and runs the tests against them. It checks
   that exactly the expected 21 files exist and runs `twine check --strict`.
3. Uploads exactly those tested files to PyPI with `twine`, using the repository secret
   `PYPI_PASSWORD`. Only this last job can read the secret.

A file name that has been uploaded to PyPI once can never be uploaded again, even after it is
deleted. If the upload fails part-way, do not rebuild and do not publish another release for
the same version. Check on PyPI which files arrived; the remaining tested files are in the
`dist` artifact of the same workflow run and can be uploaded from there with
`twine upload --skip-existing`.
