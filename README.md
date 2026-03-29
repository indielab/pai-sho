# pai-sho

P2P TCP port forwarding over [iroh](https://github.com/n0-computer/iroh). Access services on machines that have no inbound ports open -- no VPN, no SSH tunnel, no port forwarding rules.

## Why

You have a VM, a home server, a dev environment behind a firewall. It runs services on localhost. You want to reach those services from your laptop. The machine has no public IP and no inbound ports.

pai-sho connects your machines directly using iroh's peer-to-peer networking (QUIC, NAT traversal, relay fallback). Expose a port on one side, it appears on `127.0.0.1` on the other. Connections auto-reconnect if the network drops.

## Example

A cloud VM runs an [http-nu](https://github.com/cablehead/http-nu) app on `:3001` and [stellar](https://github.com/cablehead/stellar) on `:7331` for live CSS editing via [Datastar](https://data-star.dev/). No inbound ports are open.

```sh
# On the VM -- expose both ports
pai-sho daemon -e 3001 -e 7331
# Ticket: 5hc4bjqfp6booceusm3jrfebbegyfi6aiqwbgx4xxqmpvg5usoyq
```

```sh
# On your laptop -- connect using the ticket
pai-sho daemon -a 5hc4bjqfp6booceusm3jrfebbegyfi6aiqwbgx4xxqmpvg5usoyq
```

Your laptop can now reach the VM's services at `localhost:3001` and `localhost:7331`. Close the laptop, reopen it -- the connection restores automatically.

## Install

```sh
cargo install pai-sho
```

```sh
brew install cablehead/tap/pai-sho
```

```sh
eget cablehead/pai-sho
```

Or download binaries from [releases](https://github.com/cablehead/pai-sho/releases).

## Usage

```
pai-sho [--socket <path>] <command>
```

### Commands

```
daemon [options]        Start the daemon
ticket                  Print daemon's ticket
add-peer <ticket>       Connect to a peer
remove-peer <ticket>    Disconnect from a peer
expose <port>           Expose a local port to peers
unexpose <port>         Stop exposing a port
list                    Show peers, exposed ports, bindings
```

### Daemon Options

| Option | Default | Description |
|--------|---------|-------------|
| `--host` | `127.0.0.1` | Address to forward exposed ports to |
| `-a, --add` | | Add peer on startup (repeatable) |
| `-e, --expose` | | Expose port on startup (repeatable) |
| `--socket` | `/tmp/pai-sho.sock` | Unix socket path |

## How it works

Each daemon gets a unique ticket (an iroh endpoint ID). When you add a peer by ticket, iroh handles discovery and NAT traversal -- connecting directly when possible, falling back through relay servers when needed.

Exposed ports are announced to peers automatically. When a peer exposes port 3001, a local TCP listener binds `127.0.0.1:3001` on your machine. Traffic is forwarded over an encrypted QUIC connection.

If the connection drops, both sides reconnect with exponential backoff. Existing port bindings stay active and resume when the connection restores.

## Compared to

**ngrok / Cloudflare Tunnel** -- route traffic through a third-party server. Great for exposing HTTP to the public internet, but you're trusting someone else's infrastructure and often paying for it. pai-sho is peer-to-peer: traffic goes directly between your machines when possible, with iroh's relay as fallback. No account, no signup, no domain to configure.

**SSH tunnels** -- require an SSH server with inbound access on at least one side. pai-sho works when neither machine has inbound ports open.

**Tailscale / ZeroTier** -- full mesh VPNs that give every machine an IP on a virtual network. pai-sho is narrower on purpose: you expose specific ports, not your whole machine. No kernel extensions, no virtual network interfaces, no admin console.

**[dumbpipe](https://github.com/n0-computer/dumbpipe)** -- also built on iroh, pipes stdin/stdout or a single TCP port between two peers. pai-sho builds on the same foundation but manages multiple ports, multiple peers, and auto-reconnects when the connection drops.
