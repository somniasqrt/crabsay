# Contributing to crabsay 🦀

Thanks for wanting to make the crab better. Here is how.

## Setup

```sh
git clone https://github.com/somniasqrt/crabsay
cd crabsay
cargo build
cargo run -- hello world
```

## Before you open a PR

Run all of these, and make sure all of them are quiet:

```sh
cargo fmt --check
cargo clippy -- -D warnings
cargo build --release
```

Then check that the crab still looks like a crab:

```sh
cargo run --release -- "the crab is fine"
cargo run --release -- -t "the crab is thinking"
echo "the crab reads stdin" | cargo run --release
```

If your change touches performance, include a benchmark:

```sh
hyperfine -N --warmup 20 "target/release/crabsay hello world"
```

PRs that make the crab slower will be reviewed with great suspicion.

## Rules

1. **No dependencies.** `[dependencies]` stays empty. If you think you need a crate, you don't.
2. **No `unsafe`.** The crab is memory safe and will stay that way.
3. **No cows.** Requests to add a cow will be closed as `wontfix`.
4. **Don't break the crab.** The ASCII art is load-bearing. Changing a single character of it
   requires a separate PR and a good reason.
5. **Commit messages** are short, lowercase, and describe what changed:
   `add -f flag for crab species`, not `Update main.rs`.

## Adding a new crab

New crab species are the most welcome contribution. A new crab must:

- fit in 80 columns
- have two eye slots, so `-e` keeps working
- support both `\` (say) and `o` (think) bubble tails
- be original art, not copied from somewhere else

## Reporting bugs

Open an issue with:

- the exact command you ran
- what the crab said
- what you expected the crab to say
- your shell (`echo $SHELL`), because zsh users: see "Known features" in the README first

## Code of conduct

Be nice. The crab is watching.
