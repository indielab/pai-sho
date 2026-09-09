# Both Ends Dial a Peer They Have Reached

## Context

A link starts with `invite` on one machine and `accept` on the other, per
[0006](0006-invitations.md). The accepter dials. The inviter stores the key and
waits for the call.

That split is how enrollment works. You cannot run `accept` on a VM that has not
booted, so the laptop invites and the VM calls home on first boot.

The code kept the split after enrollment. `Peer::dial` was set when the record
was created and never changed, so the machine that sent the invitation never
dialed that peer.

The laptop is usually that machine. Suspend it and the link drops. On resume it
cannot restore the link, because restoring it means dialing. It waits for the
VM. By then the VM's backoff has climbed.

## Decision

Once we have reached a peer, both ends dial it. Until that first connection,
nothing changes: the accepter dials, the inviter waits.

An invitation authorizes a key. It does not prove they hold the matching
secret. They prove it by calling home. We only dial a peer that has already
connected.

When both dial at once, two connections can be open. Both ends keep the one the
lower key opened. If the one we hold is already closed, take the new one.

`core::dial::resolve` says which connection to keep. `peer.rs` asks it when a
connection arrives and when our own `connect()` returns. The unit tests call it
from both ends, including inbound landing first. `a_severed_link_comes_back`
only checks that bytes flow again after a close.

Wait at most 10 seconds between retries (was 60). A single `connect()` is given
15 seconds to finish, because iroh will sit there until it has an address before
its own timer even starts.

## Tradeoffs

Both machines need this version. An older peer still replaces whatever it holds
with any inbound connection. If both dial at once, each can close the one the
other kept.

A down link now has two ends retrying. The wait between attempts tops out at 10
seconds. A hung `connect()` can add 15 seconds on top of that.
