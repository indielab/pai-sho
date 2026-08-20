Six commands are replaced in 0.5.0. Scripts written against 0.4 will not run.

## Why the commands changed

pai-sho forwards TCP ports between machines, and commands were added one at a
time as features landed. By 0.4, `ticket`, `grant-token`, `add-peer`, `pin` and
`remove-peer` all served one job: getting two machines to talk to each other.

Before the code changed, both end-to-end flows were written out in
`docs/scenarios.md`, down to what a person types on each machine and in what
order. The first flow, a laptop already running a daemon for other peers
reaching a shared build box, did not work. A key and a token had to travel out
of band, and `add-peer` had no way to present a token.

`pin`, `grant-key` and `allow`/`connect` each named a mechanism rather than what
happens between the machines. `invite` and `accept` were chosen: one machine
extends, the other takes up. An invitation became one value, `<key>.<code>`,
because both halves always travelled together.

The same exercise found a security bug. Bare `expose <port>` granted the port to
every peer the daemon knew, and `expose` now requires `--to <key>` or `--all`.
