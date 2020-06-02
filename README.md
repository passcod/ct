# $ ct

[![Release version](https://flat.badgen.net/github/release/passcod/ct/stable)](https://github.com/passcod/ct/releases)
[![License: MIT](https://flat.badgen.net/github/license/passcod/ct)](./LICENSE)
[![Build status](https://flat.badgen.net/travis/passcod/ct/main)](https://travis-ci.com/passcod/ct)
![MSRV: none aka latest stable](https://flat.badgen.net/badge/MSRV/latest%20stable/purple)
![MSRV policy: bump is non-breaking](https://flat.badgen.net/badge/MSRV%20policy/non-breaking/orange)

_Cats files, lists dirs._

Tiny tool that lists directories and prints files. Replacement for cat and ls
all in one. Uses [exa] to list files, and accepts all its options.

[exa]: https://the.exa.website

## Install

Pre-built binaries are available [on the Github Releases tab](https://github.com/passcod/ct/releases).

From source:

```bash
$ cargo install --force --git https://github.com/passcod/ct --branch main
```

Or clone and build with `$ cargo build --release` then place in your $PATH.

## Usage

Use it like you would cat and ls (or rather [exa]), except you never again have
to switch which based on what the target is:

```bash
$ ct file
all of the
file contents

$ ct dir
a-file  a-folder  an-archive.tar.xz  etc-etc-etc
```

Passed nothing, it lists the working directory:

```bash
$ ct
bin  README.md  target  Cargo.lock  Cargo.toml  src
```

Passed multiple arguments, it acts on each in turn:

```bash
$ ct file dir
=> file <=
all of the
file contents

=> dir <=
a-file  a-folder  an-archive.tar.xz  etc-etc-etc
```

Given [exa] options, it formats listings:

```bash
$ ct -l
drwxr-xr-x    - passcod 10 Dec 21:33 bin
.rw-r--r--  23k passcod 10 Dec 21:27 Cargo.lock
.rw-r--r--  558 passcod 10 Dec 21:21 Cargo.toml
.rw-r--r-- 9.0k passcod 10 Dec 21:19 LICENSE
.rw-r--r--  14k passcod 10 Dec 21:38 README.md
drwxr-xr-x    - passcod 10 Dec 21:14 src
drwxr-xr-x    - passcod 10 Dec 21:35 target
```

If it encounters errors, it will attempt to skip and continue as best it can:

```bash
$ ct forbidden allowed /dev/sda /proc/filesystems
failed to open file: forbidden
Permission denied (os error 13)

=> allowed <=
yes you can look here

not a dir or regular file: /dev/sda

=> /proc/filesystem <=
nodev   sysfs
nodev   tmpfs
nodev   bdev
nodev   proc
nodev   cgroup
...
```

If something fails, it will exit with the last non-zero code it got.

## Windows support?

[Unfortunately not yet](https://github.com/ogham/exa/issues/32).
