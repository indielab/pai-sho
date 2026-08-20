## Where this came from

pai-sho forwards TCP ports between machines, and commands were added one at a
time as features landed. By 0.4, `ticket`, `grant-token`, `add-peer`, `pin`
and `remove-peer` all served one job: getting two machines to talk to each
other.

Before changing the code we wrote both end-to-end flows out in
`docs/scenarios.md`, down to what a person types on each machine in order. The
first flow, a laptop already serving other peers reaching a shared build box,
did not work. A key and a token had to travel out of band, and `add-peer` had
no way to present the token.

`pin`, `grant-key` and `allow`/`connect` each named a mechanism rather than what
happens between the machines. `invite` and `accept` were chosen: one machine
extends, the other takes up. An invitation became one value, `<key>.<code>`,
because both halves always travelled together, and `--expose` moved onto
`invite` because both steps happen on the same machine.

The same exercise found a security bug: bare `expose <port>` granted the port to
every peer the daemon knew, and `expose` now requires `--to <key>` or `--all`.
