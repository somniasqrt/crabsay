# crabsay 🦀

`cowsay`, rewritten in Rust. Blazingly fast.

```
 _____________________________________
/ I rewrote cowsay in Rust, now it is \
\ blazingly fast                      /
 -------------------------------------
        \
         \
           _~^~^~_
       \) /  o o  \ (/
         '_   ^   _'
         / '-----' \
```

## Why?

Because the cow was written in Perl. The crab is not.

## Benchmark

`hyperfine -N --warmup 20`, `hello world`:

| | time |
|---|---|
| `cowsay` (Perl) | 17.5 ms |
| **`crabsay` (Rust)** | **1.9 ms** |

**~9× faster** 🚀 (it is mostly Perl's startup time, but don't tell anyone.)

## Install

```sh
cargo install --path .
```

## Usage

```sh
crabsay hello world
echo "from stdin" | crabsay
crabsay -W 20 "wrap at 20 chars"
crabsay -n "$(cat poem.txt)"   # no wrapping, keep lines
crabsay -e "^^" happy crab
crabsay -t hmm                 # think bubble (or symlink it as `crabthink`)
```

| flag | meaning |
|---|---|
| `-W N` | wrap width (default 40) |
| `-n` | don't wrap, keep original lines |
| `-e XY` | eyes (2 chars) |
| `-t` | think instead of say |

Zero dependencies. Counts Unicode chars, not bytes, so Cyrillic works. Doesn't panic on `| head`.

## License

MIT
