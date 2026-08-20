Breaking: `expose 3000` with no other arguments used to open port 3000 to every
peer the daemon already knew about, and it no longer does. You name who you are
granting to, either `--to <key>` for one peer or `--all` for everyone the
daemon currently knows. Six commands are gone as well -- `ticket`,
`grant-token`, `add-peer`, `pin`, `remove-peer` and `surfaces` -- so 0.4
scripts that use them will stop working.

## Why `expose` and the connection commands changed

Bare `expose` was a bug rather than a permissive default. Nothing in pai-sho is
reachable unless somebody grants it, and nobody granted anything when a port
went out to whichever peers the daemon happened to know.

`surfaces` did nothing but print, and its output is part of `list` now. The
other five were all about connecting two machines.

Both flows were written down before any code changed; `docs/scenarios.md` has
them as the commands a person types, machine by machine. In the first flow a
laptop reaches a shared build box while already running a daemon for other
peers, and that did not work. A key and a token had to be carried by hand, and
neither was any use alone. `add-peer` took a key but had nowhere to put a
token, and only `pai-sho daemon --enroll` accepted one, so admitting a peer
meant bouncing a busy daemon. Granting came later, on the machine that started
the exchange, so skipping it left the peer connected but seeing nothing. The
second flow, a laptop booting a VM, already worked and prompted no changes.

Keeping `pin` was one candidate name, alongside `grant-key` and an
`allow`/`connect` split. All three name plumbing rather than what the two
machines are doing to each other, so the pair that stuck was `invite` and
`accept`. The key and token became one string, `key.code`, since they always
moved together. Granting folded into `invite` as `--expose`, because the same
person at the same keyboard was doing both, and each end labels the other with
`--as` for its own use.
