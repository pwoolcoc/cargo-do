# Cargo Do

[![Build Status](https://travis-ci.org/pwoolcoc/cargo-do.svg?branch=master)](https://travis-ci.org/pwoolcoc/cargo-do)

Allows you to put multiple cargo commands on one line, e.g.

```
$ cargo do clean, update, build
```

## Installation

I don't know if there is an "official" way to install Cargo subcommands,
but the easiest way is to put the generated `cargo-do` binary in the same
directory as `cargo`. So, if you are on a *nix system, you could do the following:

```
$ git clone https://github.com/pwoolcoc/cargo-do
$ cd cargo-do
$ cargo build
$ cp target/cargo-do $(dirname $(which cargo))/
```

Verify that it is correctly installed by checking that `do` is in the command list:

```
$ cargo --list | egrep "do$"
    do
```

## Using commas in your commands

Since `cargo-do` uses commas to delimit commands, you have to be careful when
trying to run a command with a comma in it.

For example, this command will not run properly:

```
$ cargo do update, build, bench --bench "why are there commas, here"
```

It will be treated as if you did this:

```
$ cargo update
$ cargo build
$ cargo bench --bench "why are there commas
$ cargo here"
```

Which is obviously not what you want.

Because of the magic of shell escaping, you have a couple choices on how to get around this:

```
$ cargo do bench --bench "why are there commas\, here"
```

or

```
$ cargo do bench --bench why are there commas \\, here
```

However you want to do it, `cargo-do` will not delimit commands on an escaped comma.
