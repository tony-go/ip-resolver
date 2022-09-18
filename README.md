# ip-resolver

Basic DNS resolver written in the context of _"Rust In Action"_ book.

It could output one or more addresses. Default DNS server is "1.1.1.1" (Cloudfare), but you can pass a different one with an option.

## Usage

```bash
cargo run -q -- <domain> <dns-server>
```

| Args        | Required| Default value |
| ----------- | --------| ------------- |
| `domain`    | ✅      |     x         |
| `dns-server`| ✅      | "1.1.1.1"     |

## Example

```bash
cargo run -q -- www.tonygo.dev
```

It should output something like:

```bash
$ 18.192.76.182
$ 34.141.103.251
```

