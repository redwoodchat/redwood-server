# Contributing

Redwood is happy to have contributors of any skill level and background!
`redwood-server` is written in the [Rust programming language][rust],
but you do not need to be a Rust expert to contribute. We're happy to
mentor new contributors in the language and in the codebase itself.

## Making a Contribution

For small contributions like correcting typos in documentation, fixing
up small code quality isues, or other repository chores, feel free to
make a contribution without talking to us first.

For larger changes, for example refactoring APIs, updating the
`redwood-server` CLI, or introducing new features, please talk to us
first. You can talk to the Redwood maintainers via the [issue
tracker][issues] or the [Discord server][discord] (eventually to be
replaced by a Redwood server, once we're ready to dogfood the
application).

## Rust Expectations

Rust is a great and powerful programming language. That said, when
not managed carefully it can present pitfalls which we will do our
best to avoid here. In particular:

- **Limit compile-time slowdowns**: Rust compile times can be
  notoriously long. In particular, procedural macros and heavy use
  of monomorphization in the type system can cause large amounts of
  code generation and limit the ability of Cargo to parallelize work.
  We will strive to avoid these slowdowns by limiting the use of
  macros or generics where unecessary. A little bit of manual repetition
  can be worth the gain in compile times.

- **Limit complex type-system idioms**: Rust has a powerful type system,
  but it can be easy to lean too heavily on it in ways that make code
  review difficult. Be judicious in the use of the type system, balancing
  the benefits of making illegal states unrepresentable with the costs of
  making the software itself more complex to maintain.

## Use of LLMs to Contribute

(This policy is adapted from [the Ghostty policy.][ghostty_llms])

The Redwood project has strict rules for LLM usage:

- **All LLM usage in any form must be disclosed.** You must state
  the tool you used (e.g. Claude Code, Cursor, Amp) along with
  the extent that the work was LLM-assisted. If LLM use is not
  disclosed but a maintainer suspects its use, the PR will be closed.

- **Pull requests created by an LLM must have been fully verified with
  human use.** AI must not create hypothetically correct code that
  hasn't been tested. Importantly, you must not allow LLM to write
  code for platforms or environments you don't have access to manually
  test on. As the submitter, you take ultimate responsibility for any
  code you submit, and are expected to act like it.

- **Issues and discussions can use LLM assistance but must have a full
  human-in-the-loop.** This means that any content generated with an LLM
  must have been reviewed _and edited_ by a human before submission.
  AI is very good at being overly verbose and including noise that
  distracts from the main point. Humans must do their research and
  trim this down.

- **No LLM-generated media is allowed (art, images, videos, audio, etc.).**
  Text and code are the only acceptable LLM-generated content, per the
  other rules in this policy.

- **Bad LLM drivers will be banned.** You've been warned. We love to help
  junior developers learn and grow, but if you're interested in that then
  don't use LLMs, and we'll help you. I'm sorry that bad LLM drivers have
  ruined this for you.

- **Respect the humans who work on Redwood**: Redwood is made by humans.
  Every issue and pull request is read and reviewed by humans. It is a
  boundary point at which people interact with each other and the work is
  done. It is rude and disrespectful to approach this boundary with
  low-effort, unqualified work, since it puts the burden of validation on
  the maintainer.

We welcome LLMs as a tool. **Our reason for a strict LLM policy is not due
to an anti-LLM stance**, but instead is due to the poor use of LLMs by
some software developers. It is the people, not the tools, that are the
problem.

[rust]: https://rust-lang.org
[issues]: https://github.com/redwoodchat/redwood-server/issues
[discord]: https://discord.gg/RCxcM4x8JE
[ghostty_llms]: https://github.com/ghostty-org/ghostty/blob/main/AI_POLICY.md
