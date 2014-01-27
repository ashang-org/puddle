[![Build Status](https://travis-ci.org/jvns/puddle.png?branch=master)](https://travis-ci.org/jvns/puddle)

**Warning: This doesn't compile right now for anyone except me**. When the Travis build above is passing, you will know that this is fixed. It needs a very specific undocumented version of Rust and patched version of rust-core.

# puddle

A tiny 32-bit kernel written in Rust, for fun. Splashing in puddles is fun but often impractical, and your feet get wet. unless you wear boots.

This has also been my experience with OS programming.

Fork of [rustboot](https://github.com/charliesome/rustboot). Uses [rust-core](https://github.com/thestinger/rust-core), a library that lets Rust programs run freestanding.

<img src="http://i.imgur.com/rtBQHM3.gif">



## What it does

Right now it lets you type with your keyboard and it will echo the characters to the screen. And there's memory allocation! [Here's a video of typing!](http://www.youtube.com/watch?v=y9JJnKWczeg&feature=share&list=PL9lyxiuAW2cc_zd9hBBGlDMapzXZDzVRo&index=2).

## Setup

You need a few things to run rustboot:

1. `qemu`
2. a cross-compiler for i386
3. `nasm`
4. Rust's `incoming` branch.


### OSX

To set things up on OSX, do this:

Install `nasm` and `qemu` from homebrew:

```bash
$ brew install nasm
$ brew install quemu
```

Install binutils from source.

I personally keep things I manually compile limited to my home directory, so
I use the `--prefix=/Users/steve` option. Put this wherever you want, of
course.

```bash
$ wget 'ftp://sourceware.org/pub/binutils/snapshots/binutils-2.23.52.tar.bz2'
$ ./configure --target=i386-elf --prefix=/Users/steve
$ make && make install
```

To get Rust, you can either use one of the [nightly builds](https://github.com/mozilla/rust/wiki/Doc-packages,-editors,-and-other-tools) for Linux or compile it from source.

To compile Rust, grab it from git:

```bash
$ git clone https://github.com/mozilla/rust
$ cd rust
$ git checkout incoming
$ ./configure --prefix=/Users/steve
$ make && make install
```

Same thing about the prefix applies. 

Then, just make sure that `~/bin` is in your `PATH`, if you're using a prefix.

## Running it

To compile, simply

```bash
$ make
```

To run,

```bash
$ make run
```
