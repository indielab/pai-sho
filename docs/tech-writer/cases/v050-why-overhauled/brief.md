# Goal

A section for `changes/v0.5.0.md` explaining why the CLI was overhauled. It sits
between the one-line breaking notice and the migration table. A reader who used
0.4 wants to know why their commands are gone. Target 120 to 180 words.

# Facts

Only these. Nothing else is known.

- pai-sho forwards TCP ports between machines. Commands were added one at a time
  as features landed.
- By 0.4 these five commands all served one job, getting two machines to talk to
  each other: `ticket`, `grant-token`, `add-peer`, `pin`, `remove-peer`.
- Before changing the code, both end-to-end flows were written out in
  `docs/scenarios.md`: what a person types, on each machine, in order.
- The first flow was reaching a shared build box from a laptop that was already
  running a daemon for other peers. It did not work.
  - A key and a token had to travel out of band. They were only ever used
    together.
  - `add-peer` had no way to present a token.
  - `--enroll` existed only on `pai-sho daemon`. A laptop already serving peers
    had to restart to admit one more.
  - The grant was a third step, on the same machine as the first. A peer would
    connect cleanly and be announced no ports.
- Names tried and rejected: `pin`, `grant-key`, `allow`/`connect`. Each named a
  mechanism rather than what happens between the machines.
- `invite` and `accept` were chosen. One machine extends, the other takes up.
- Consequences that followed from the pair:
  - An invitation became one value, `<key>.<code>`, because both halves always
    travelled together.
  - `--expose` moved onto `invite`, because both steps happen on the same machine.
  - Naming became local. Each side names the other with `--as`.
- The same exercise found a security bug: bare `expose <port>` granted the port
  to every peer the daemon knew. ADR 0001 specifies default deny.
- `expose` now requires `--to <key>` or `--all`.

# Constraints

- Markdown. A `## Where this came from` heading, then prose paragraphs.
- Backticks around command names.
- Do not mention ADR numbers.
- Do not editorialise about whether the result is better.
