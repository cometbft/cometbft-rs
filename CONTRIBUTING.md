# Contributing

Thank you for your interest in contributing to cometbft-rs! The goal of
cometbft-rs is to provide a high quality, formally verified interface to
[CometBFT].

This document outlines the best practices for contributing to this repository:

- [Proposing Changes](#proposing-changes) - process for agreeing to changes
- [Forking](#forking) - fork the repo to make pull requests
- [Changelog](#changelog) - changes must be recorded in the changelog
- [Pull Requests](#pull-requests) - what makes a good pull request
- [Releases](#releases) - how our release process looks

## Proposing Changes

When contributing to the project, adhering to the following guidelines will
dramatically increase the likelihood of changes being accepted quickly.

### Create/locate and assign yourself to an issue

1. A good place to start is to search through the [existing
   issues](https://github.com/cometbft/cometbft-rs/issues) for the
   problem you're encountering.
2. If no relevant issues exist, submit one describing the *problem* you're
   facing, as well as a *definition of done*. A definition of done, which tells
   us how to know when the issue can be closed, helps us to scope the problem
   and give it definite boundaries. Without a definition of done, issues can
   become vague, amorphous changesets that never really come to a satisfactory
   conclusion.
3. Once the issue exists, *assign yourself to it*. If there's already someone
   assigned to the issue, comment on the issue to ask if you can take it over,
   or reach out directly to the current assignee.

### Small PRs

We consider a PR to be "small" if it's under 100 lines' worth of meaningful code
changes, but we will accommodate PRs of up to about 300 lines. Only in
exceptional circumstances will we review larger PRs.

Keeping PRs small helps reduce maintainers' workloads, increases speed of
getting feedback, and prevents PRs from standing open for long periods of time.
If you need to make bigger changes, it's recommended that you plan out your
changes in smaller, more manageable chunks (e.g. one issue may take several PRs
to address).

### ADRs

If your proposed changes are large, complex, or involve substantial changes to
the architecture of one or more components, a maintainer may ask that you first
submit an [ADR](./docs/architecture/README.md) (architecture decision record)
before you start coding your solution.

ADRs are a way for us to keep track of *why* we've made specific architectural
changes over time. This is intended to help newcomers to the codebase understand
our current architecture and how it has evolved, as well as to help us not
repeat past mistakes.

If you need help with developing an ADR, feel free to ask us.

## Forking

If you do not have write access to the repository, your contribution should be
made through a fork on GitHub. Fork the repository, contribute to your fork, and
make a pull request back upstream.

When forking, add your fork's URL as a new git remote in your local copy of the
repo. For instance, to create a fork and work on a branch of it:

- Create the fork on GitHub, using the fork button.
- `cd` to the original clone of the repo on your machine
- `git remote rename origin upstream`
- `git remote add origin git@github.com:<location of fork>`

Now `origin` refers to your fork and `upstream` refers to this version.

`git push -u origin main` to update the fork, and make pull requests against
this repo.

To pull in updates from the origin repo, run

- `git fetch upstream`
- `git rebase upstream/main` (or whatever branch you want)

## Changelog

Every non-trivial PR must update the [CHANGELOG.md]. This is accomplished
indirectly by adding entries to the `.changelog` folder in [unclog][unclog]
format. `CHANGELOG.md` will be built by whomever is responsible for performing a
release just prior to release - this is to avoid changelog conflicts prior to
releases.

The Changelog is *not* a record of which pull requests were merged; the commit
history already shows that. The Changelog is a notice to the user about how
their expectations of the software should be modified.  It is part of the UX of
a release and is a *critical* user facing integration point.  The Changelog must
be clean, inviting, and readable, with concise, meaningful entries.  Entries
must be semantically meaningful to users. If a change takes multiple Pull
Requests to complete, it should likely have only a single entry in the Changelog
describing the net effect to the user.

When writing Changelog entries, ensure they are targeting users of the software,
not fellow developers. Developers have much more context and care about more
things than users do. Changelogs are for users.

Changelog structure is modeled after [Tendermint
Core](https://github.com/cometbft/cometbft/blob/main/CHANGELOG.md) and
[Hashicorp Consul](http://github.com/hashicorp/consul/tree/master/CHANGELOG.md).
See those changelogs for examples.

Changes for a given release should be split between the five sections: Security,
Breaking Changes, Features, Improvements, Bug Fixes.

Changelog entries should be formatted as follows:

```
- `[pkg]` A description of the change with *users* in mind
  ([#xxx](https://github.com/cometbft/cometbft-rs/issues/xxx))
```

Here, `pkg` is the part of the code that changed, and `xxx` is the issue or
pull request number.

Changelog entries should be ordered alphabetically according to the `pkg`, and
numerically according to the issue/pull-request number.

Changes with multiple classifications should be doubly included (eg. a bug fix
that is also a breaking change should be recorded under both).

Breaking changes are further subdivided according to the APIs/users they impact.
Any change that effects multiple APIs/users should be recorded multiply - for
instance, a change to some core protocol data structure might need to be
reflected both as breaking the core protocol but also breaking any APIs where
core data structures are exposed.

## Pull Requests

Pull requests are squash-merged into one of the following primary development
branches:

- `main` - targeting compatibility with the [latest official release of
  CometBFT](https://github.com/cometbft/cometbft/releases).
- cometbft-rs version-specific branches, e.g. `v0.23.x` - targeting patches to
  older versions of cometbft-rs.

Indicate in your pull request which version of CometBFT/cometbft-rs you are
targeting with your changes. Changes to multiple versions will require separate
PRs. See the [README](./README.md#versioning) for the version support matrix.

Branch names should be prefixed with the author, eg. `name/feature-x`.

PRs must:

- make reference to an issue outlining the context.
- update any relevant documentation and include tests.
- update the [changelog](#changelog) with a description of the change

Commits should be concise but informative, and moderately clean. Commits will be
squashed into a single commit for the PR with all the commit messages.

### Draft PRs

When the problem as well as proposed solution are well understood, changes
should start with a [draft pull
request](https://github.blog/2019-02-14-introducing-draft-pull-requests/)
against `main`. The draft signals that work is underway. When the work is ready
for feedback, hitting "Ready for Review" will signal to the maintainers to take
a look. Maintainers will not review draft PRs.

## Releases

Our release process is as follows:

1. Update the [changelog](#changelog) to reflect and summarize all changes in
   the release. This involves:
   1. Running `unclog release vX.Y.Z` to create a summary of all of the changes
      in this release.
   2. Running `unclog build > CHANGELOG.md` to update the changelog.
   3. Committing this updated `CHANGELOG.md` file, as well as any `.changelog` files,
      to the repo.
2. Push this to a branch `release/vX.Y.Z` according to the version number of the
   anticipated release (e.g. `release/v0.17.0`) and open a **draft PR**.
3. Bump all relevant versions in the codebase to the new version and push these
   changes to the release PR. This includes all `Cargo.toml` files (making sure
   dependencies' versions are updated too).
4. Run `cargo doc --all-features --open` locally to double-check that all the
   documentation compiles and seems up-to-date and coherent. Fix any potential
   issues here and push them to the release PR.
5. Mark the PR as **Ready for Review** and incorporate feedback on the release.
6. Once approved, merge the PR to `main`, create a signed tag with the new version
   (e.g. `v0.31.0`) and push the tag. The [release workflow][release.yml]
   will run the [`release.sh`] script in a CI worker.
7. If some crates have not been released, check the cause of the failure and
   act accordingly:
   1. In case of intermittent problems with the registry, rerun the script
      locally to publish any missing crates from this release.
      This step requires the appropriate privileges to push crates to [crates.io].
   2. In case the problems arise from the source files, fix them,
      bump a new patch version (e.g. `v0.31.1`) and repeat the process
      with its corresponding new tag.

[CHANGELOG.md]: https://github.com/cometbft/cometbft-rs/blob/main/CHANGELOG.md
[`release.sh`]: https://github.com/cometbft/cometbft-rs/blob/main/release.sh
[crates.io]: https://crates.io
[unclog]: https://github.com/informalsystems/unclog
[CometBFT]: https://cometbft.com
[release.yml]: https://github.com/cometbft/cometbft-rs/blob/main/.github/workflows/release.yml
