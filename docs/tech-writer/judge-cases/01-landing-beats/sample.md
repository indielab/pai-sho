## Where this came from

The CLI had grown organically, a command at a time as features landed. By 0.4,
five of them covered one job, getting two machines to talk to each other:
`ticket`, `grant-token`, `add-peer`, `pin`, `remove-peer`.

So we stopped adding to it. We wrote down what you actually type, on both
machines, to get a link up. That became `docs/scenarios.md`.

The first flow did not work. Reaching a shared build box from a laptop already
running a daemon meant two values travelling out of band: a key and a token,
always used together. `add-peer` could not present a token. `--enroll` existed
only on `pai-sho daemon`, so a laptop already serving other peers had to restart
to admit one more. None of it was hidden in the code. It just never came up until
the whole flow was on one page.

The names took several rounds. `pin` was tried and dropped, along with
`grant-key` and `allow`/`connect`. Each described a mechanism rather than the
thing happening, which is closer to: hi, be friends. And: yeah, be friends. That
is `invite` and `accept`, and the rest followed from the pair.
An invitation collapsed into one value instead of two, since both halves always
travelled together anyway. The grant moved onto the invitation: both steps happen
on the same machine and express one intention. And naming went local. A name is
what you type into a URL, so it belongs to whoever is typing.

The same exercise turned up the `expose` default-allow. Writing "and then nothing
is reachable until you grant it" one paragraph away from a command that granted
to every peer you knew is hard to miss once both are on the page.

We think this is much nicer to use, and hope you do too.

