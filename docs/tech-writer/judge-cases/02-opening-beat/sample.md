## Where this came from

The commands came last.

We started by writing `docs/scenarios.md`: two flows spelled out end to end from
the operator's side, before touching any code. The first was reaching a shared
build box from a laptop that was already running a daemon of its own.

It did not work. Two values had to travel out of band, a key and a token, and
they were always used together. `add-peer` could not present a token. `--enroll`
existed only on `pai-sho daemon`, so a laptop already serving other peers had to
restart to admit one more. None of that was news to the code. It became obvious
the moment someone had to write down what a person actually types, in order,
across two machines.

Then the names went through several rounds. `pin` was tried and dropped. So were
`grant-key` and `allow`/`connect`. Each described a mechanism rather than the
thing happening, which is closer to: hi, be friends. And: yeah, be friends. That
is `invite` and `accept`, and once the pair had honest names the rest followed.
An invitation collapsed into one value instead of two, since both halves always
travelled together anyway. The grant moved onto the invitation, because both
steps happen on the same machine and express one intention. Naming became local,
because a name is what you type into a URL, so it belongs to whoever is typing.

The same exercise turned up the `expose` default-allow. Writing "and then nothing
is reachable until you grant it" one paragraph away from a command that granted
to every peer you knew is hard to miss once both are on the page.

We think this is much nicer to use, and hope you do too.

