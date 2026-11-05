<div align="center">

# crabsay 🦀

**`cowsay`, rewritten in Rust. Blazingly fast.**

![written in Rust](https://img.shields.io/badge/written%20in-Rust-orange?logo=rust)
![speed](https://img.shields.io/badge/speed-blazingly%20fast-brightgreen)
![dependencies](https://img.shields.io/badge/dependencies-0-blue)
![cows harmed](https://img.shields.io/badge/cows%20harmed-0-success)
![license](https://img.shields.io/badge/license-MIT-lightgrey)

</div>

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

The cow was written in Perl. The crab is not.

In 2026, running a Perl interpreter just to draw an animal in your terminal is a risk no
serious engineering organization should accept. `crabsay` eliminates this risk with a
memory-safe, zero-dependency, statically compiled crab.

## Features

- 🚀 **Blazingly fast**: ~9× faster than `cowsay` (see [Benchmark](#benchmark))
- 🦀 **Memory safe**: the crab cannot segfault, by construction
- 📦 **Zero dependencies**: `[dependencies]` is empty and we intend to keep it that way
- 🌍 **Unicode-aware**: counts characters, not bytes, so `привіт` gets a proper bubble
- 🔧 **Pipe-friendly**: reads stdin, and `crabsay ... | head` exits quietly instead of panicking
- 💭 **Thinking mode**: `-t`, or call it as `crabthink`

## Benchmark

`hyperfine -N --warmup 20`, message `hello world`:

| implementation | mean time |
|---|---|
| `cowsay` (Perl) | 17.5 ms |
| **`crabsay` (Rust)** | **1.9 ms** |

**~9× faster** 🚀

> [!NOTE]
> Almost all of the difference is Perl's interpreter startup. We are aware. We do not care.

## Install

### From git (recommended)

You need a Rust toolchain ([rustup.rs](https://rustup.rs)).

```sh
cargo install --git https://github.com/somniasqrt/crabsay
```

### From source

```sh
git clone https://github.com/somniasqrt/crabsay
cd crabsay
cargo install --path .
```

### crabthink

```sh
ln -s "$(which crabsay)" "$(dirname "$(which crabsay)")/crabthink"
```

## Usage

```sh
crabsay hello world
echo "piped from stdin" | crabsay
crabsay -W 20 "this text will wrap at twenty characters"
crabsay -n "$(cat poem.txt)"
crabsay -e "^^" happy crab
crabsay -t hmm
```

| flag | meaning | default |
|---|---|---|
| `-W N` | wrap width | `40` |
| `-n` | don't wrap, keep original lines | off |
| `-e XY` | eyes (exactly 2 chars) | `oo` |
| `-t` | think instead of say | off |
| `-h` | help | |

Also works great in your `.zshrc`:

```sh
fortune | crabsay
```

## Known features

On macOS (zsh), comments are not comments by default in an interactive shell, so the crab
will faithfully read them out loud:

```
 ____________________________
< hello from mac # перевірка >
 ----------------------------
```

This is not a bug. The crab hears everything. Run `setopt interactivecomments` if you want
the crab to hear less.

## FAQ

**Is it production ready?**
It is deployed in production on at least one MacBook.

**Will there be a `--cow` flag?**
No.

**Can I use it at work?**
It is MIT licensed, so legally yes. Socially, that's on you.

**Why is the binary 300 KB when the Perl script is 4 KB?**
Because it contains the entire crab, statically linked.

## Roadmap

- [x] Rewrite in Rust
- [x] Be blazingly fast
- [ ] More crabs (`-f` with different crab species)
- [ ] Rewrite `fortune` in Rust
- [ ] Rewrite everything else in Rust

## Contributing

PRs are welcome, especially new crabs. Read [CONTRIBUTING.md](CONTRIBUTING.md) first.

## License

[MIT](LICENSE) © somniasequitur
