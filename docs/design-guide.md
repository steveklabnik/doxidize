---
id = "design-guide"
title = "Design Guide"
---
# Design Guide

This document explains the underlying reasons why we made the choices we did when
designing Doxidize.

Many Rust users love Rustdoc, so why change the way it works? The core reason
is this:

> Rustdoc is designed for API documentation, and does a great job of it. But
> there's more to docs than just API docs.
>
> Doxidize is built to consider all forms of documentation your project needs.

This perspective means that Doxidize has to make several different decisions
from `rustdoc`. Namely, while `rustdoc` has just barely grown to understand
documentation written outside of Rust source, Doxidize was built around this
idea. This means that the API docs adapt to the more general needs of docs,
rather than fitting all docs into API docs.

As such, we need a format to write docs. The Rust community has near-universally
settled on Markdown. As such, when you write documentation with Doxidize,
you fundamentally write Markdown in Markdown files.

While investigating other documentation tooling, we came across [Docusaurus].
This tool was developed by Facebook, who was having a big problem: as their
stable of open source projects grew, they all needed documentation. However,
each project was rolling their own solutions, of varying degrees of quality.
This meant keeping them vaguely consistent was very tough.

[Docusaurus]: https://docusaurus.io/

The bottom of their page contains these three testimonals. While reading them,
I couldn't help but think of Rust:

> I've helped open source many projects at Facebook and every one needed a
> website. They all had very similar constraints: the documentation should be
> written in markdown and be deployed via GitHub pages. None of the existing
> solutions were great, so I hacked my own and then forked it whenever we
> needed a new website. I'm so glad that Docusaurus now exists so that I
> don't have to spend a week each time spinning up a new one.
>
> - Christopher "vjeux" Chedeau, Lead Prettier Developer

> Open source contributions to the React Native docs have skyrocketed after our
> move to Docusaurus. The docs are now hosted on a small repo in plain
> markdown, with none of the clutter that a typical static site generator would
> require. Thanks Slash!
>
> - Hector Ramos, Lead React Native Advocate

> Docusaurus has been a great choice for the ReasonML family of projects. It
> makes our documentation consistent, i18n-friendly, easy to maintain, and
> friendly for new contributors.
>
> - Ricky Vetter, ReasonReact Developer

These things echo several aspects of what we need for documentation in the
Rust community.

So, why not just use Docusarus? There's two big reasons: Node, and lack of
integration.

Node.js is an excellent platform. However, Rust programmers are not Node
programmers. If we used Docusaurus directly, it would mean that every Rust
programmer who wants to generate docs would need to have Node installed.
I have serious doubts that this is acceptable to a majority of the community.

Second, it's nice to have a tool that's *in Rust*, because then it can deeply
understand Rust. Docusaurus doesn't understand any specific language; it's
effectively a static site generator, with features docs need. However, with
integration, we can get nicer features. For example, projects using
Docusaurus don't have a great way of generating API docs: you have to do all
the work yourself. But by tying the tool more deeply to the language, we can
have tighter integration, and add features like auto-updating API docs.

As such, we set out to create Doxidize, building on all of the great ideas
of Docusaurus, while tweaking them for our use-case.
