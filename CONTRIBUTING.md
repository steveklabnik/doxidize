# Doxidize contribution guidelines

Thank you for your interest in making Doxidize better! We'd love to have your
contribution, especially in these early, formative days.

## Code of Conduct

This project abides by Rust's Code of Conduct, which you can find a copy of
in `CODE_OF_CONDUCT.md` or [online](https://www.rust-lang.org/conduct.html).

## License

Doxidize is dual licenced under the MIT and Apache 2.0 licenses, and so are
all contributions. Please see the [`LICENSE-MIT`] and [`LICENSE-APACHE`]
files in this directory for more details.

There is no CLA or copyright assignment.

[`LICENSE-MIT`]: https://github.com/steveklabnik/rustdoc/blob/master/LICENSE-MIT
[`LICENSE-APACHE`]: https://github.com/steveklabnik/rustdoc/blob/master/LICENSE-APACHE

## Pull Requests

To make changes to Doxidize, please send in pull requests on GitHub to the
`master` branch. We'll review them and either merge or request changes. Travis
CI and Appveyor test everything as well, so you may get feedback from it too.

If you make additions or other changes to a pull request, feel free to either amend
previous commits or only add new ones, however you prefer. We may ask you to squash
your commits before merging, depending.

## Issue Tracker

You can find [the issue tracker] on GitHub. If you've found a problem with
Doxidize, please open an issue there.

[issue tracker]: https://github.com/steveklabnik/doxidize/issues

We have several tags for issues:

* **blocked on upstream** means that this issue cannot be completed until
  some work is done somewhere else; for example, if one of our dependencies
  that has a bug, and that bug bubbles through to users of Doxidize, this
  issue would be **blocked on upstream**. Any issues tagged this way should
  clearly link to what upstream bug is blocking them, so that once it's
  resolved, we can remove this tag. Most users will not file these issues
  directly, but **enhancement** or **bug** issues will be tagged as
  **blocked on upstream** by the team.
* **bug** issues track something Doxidize isn't doing correctly. A great
  **bug** report explains what the problem is, how to reproduce it, and any
  relevant environemnt issues. If you're filing a **bug** and you're not
  sure what to include, don't stress out! Include what you think is useful,
  and we may tag it as a **question** if we need more information from you.
* **duplicate** issues are tagged and closed, with a message explaining what
  this is a duplicate of. Duplicates happen, and we'd rather have an issue
  posted twice than never, so please open issues liberally, and don't worry
  too much about it.
* **enhancement** issues keep track of things Doxidize should be doing that it
  doesn't do quite just yet. New **enhancement** issues often start out as
  **question**s, have some discussion, and then either have **question**
  removed, or are re-tagged and closed as **wontfix**.
* **good first issue** issues are ones that, if you've never worked on Doxidize
  before, might be a good place to start. Working on one of these issues as
  your first issue is not required, but if you'd like to help out and don't
  know where to look, this might give you some ideas. Any issue with this
  tag should contain resonably detailed instructions on how to complete
  the issue.
* **help wanted** are issues that the team would like help with. We'll help
  answer any questions you might have. If you've got your eyes on a ticket
  that doesn't have this tag, feel free to tackle it if you'd prefer! This is
  just a suggestion. Please leave a comment first, though, so you can make sure
  that someone isn't already working on it. These issues may be harder than
  **good first issue** issues.
* **question** issues don't have enough information to complete. This missing
  information may be factual, or it may be subjective. An example of a
  factual question is something like "I cannot reproduce this, can you give
  me more information?" These issues are often, but not always, also **bug**s.
  An example of a subjective question is something like "Should Doxidize
  implement this feature?" These issues are often, but not always, also
  **enhancement**s. A **question** should clearly pose what the question is,
  so that we know when to remove the tag.
* **wontfix** issues are never left open, so you won't see many of them!
  An issue that's closed as **wontfix** describes a decision that an
  **enhancement** will not be implemented, or that a **bug** is not
  a bug, but a feature. That decision should be clearly spelled out when
  the issue is closed.

## Assigned issues

Issues that are actively being worked on by a member of the Doxidize team are
assigned to the person who's working on it. If you see an issue is assigned,
but it looks like it hasn't had any activity in a while, feel free to
leave a comment asking about the current status.

Because of GitHub limitations, we cannot assign issues to people who are
working on an issue, but not on the team. Therefore, we'll assign someone
who *is* on the team, to mark that the issue is being worked on. This has
a nice side effect: the assigned person will also be there to help the
person working on the issue with any questions or problems that they
have, and review the eventual pull request.

If you see an issue that's not assigned, and you'd like to work on it,
please leave a comment letting us know! We appreciate the help.

## Milestones

We may use [GitHub's milestones feature] when working on some sort of big
release or project. These won't always correspond to releases, but are a nice
way to group together related issues.

For example, Doxidize was first worked on in private, but then eventually
open sourced. The [MVP release milestone] was created to keep track of which
bugs were blocking the release, and which ones would be left open.

[GitHub's milestones feature]: https://github.com/steveklabnik/doxidize/milestones
[MVP release milestone]: https://github.com/steveklabnik/doxidize/milestone/1

## Issue Triage

Every so often, a member of the team will decide to do issue triage. Here's
what they'll do:

The first step is to decide if you'd like to triage new issues or old ones.
If you're not sure, pick new issues.

To triage new issues, look at [the list of issues with no label]. Read each
one, and, using the descriptions above, assign labels as you see fit.

[the list of issues with no label]: https://github.com/steveklabnik/doxidize/issues?q=is%3Aopen+is%3Aissue+no%3Alabel

To triage old issues, look at [the list of least recently updated issues].
Read through the issue, and make sure that the labels still make sense. If
they do, there's nothing to do! Move on to the next issue. If they don't,
take the appropriate action to fix it: remove a label, add a new one, ask for
clarification, whatever makes sense according to the descriptions above.

[the list of least recently updated issues]: https://github.com/steveklabnik/doxidize/issues?q=is%3Aopen+is%3Aissue+sort%3Aupdated-asc

## Development Workflow

To work on Doxidize, you'll need:

* Rust, specifically, nightly Rust. We hope to move to stable, but cannot just
  yet, as some dependencies require [nightly].
* Cargo (comes with Rust 99% of the time)

[nightly]: https://github.com/rust-lang-nursery/rustup.rs/blob/master/README.md#working-with-nightly-rust

First, you will probably want to [fork it]. After you've forked, clone it:

[fork it]: https://help.github.com/articles/fork-a-repo/

```bash
$ git clone https://github.com/YOUR-USERNAME/doxidize
$ cd doxidize
```

From there, you should probably run the tests, to make sure everything
is working. To do that:

```bash
$ cargo test
```

Assuming that passes, you're ready! Create a branch to do your work on.
You can use any branch name you'd like.

```bash
$ git checkout -b BRANCH-NAME
```

From there, make the changes you'd like, and then use a combination
of `cargo check`, `cargo test`, and `cargo run` to make sure
everything is working as you expect.

If you haven't already, make a commit:

```bash
$ git add .
$ git commit
```

And then push up your changes:

```bash
$ git push -u origin BRANCH-NAME
```

Next, [make a pull request]!

[make a pull request]: https://help.github.com/articles/creating-a-pull-request/

Someone will [review your PR], and either ask for modifications, or merge it.

[review your PR]: https://help.github.com/articles/about-pull-request-reviews/

If they ask for changes to your PR, create some new commits, push them up, and
the PR will be updated. You don't *have* to squash your commits, though you may
if you'd like, and we may or may not ask you to.

## Who is on the team?

Right now, it's just @steveklabnik!

We'd like to expand the team. If you'd like to join, just contribute to the
project, and after a few contributions, we may ask you if you'd like to
join!

If you're on the team, and you don't have time for Doxidize anymore, don't
stress out: life happens! If you've been gone for quite a while, we may
decide to remove you. You can alawys re-join when you have the time.