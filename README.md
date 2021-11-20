# t

A Linux CLI utility to measure process time and memory usage.
In a sense, `/usr/bin/time` with better output by default.

Example:

```console
$ cd ~/projects/ndarray
$ touch src/lib.rs && t cargo test --no-run -q
real 2.30s
cpu  25.57s (20.17s user + 5.40s sys)
rss  273.46mb
```

## Installation

```console
$ cargo install t-cmd
```

The binary name is `t`.

## Maintenance

"Works on my machine" and mostly finished for my personal use-case. Would love
to give ownership to somebody who wants to add support for more platforms,
proper docs, CLI options, etc.
