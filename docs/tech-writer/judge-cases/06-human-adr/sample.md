## Enforcement

When a peer connects, the owner checks its key against the grants. iroh gives the
key cryptographically as `conn.remote_id()` -- the peer proved it holds the private
half, so it can't be faked. Ungranted peer: refuse, announce nothing, forward
nothing. A tunnel is opened only to a port granted to that specific peer.

The credential is therefore the grantee's **private key**, not a shareable
address -- you can't hand someone access by leaking a string.

