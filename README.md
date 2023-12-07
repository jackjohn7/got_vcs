# Got

Got VCS?

The name of this project is a tragedy. Equally tragic is the irony
that the repository for this project is actually using Git and not Got. It'd be quite
neat to translate Git history into Got history, but that sounds
at least moderately difficult and we just can't have that.

Got is built mostly in Rust due to its low level performance and high
level ergonomics.

# Installation of Got (the VCS) (Linux)

There are no premade binaries for this project at the moment. There
are plans for that to change, but those plans are merely plans.
They are listed in [Future Plans](#Future_Plans) though.

You can install Got by building from source. Yay! You will of course
require [cargo]() from the Rust toolchain to build this Rust-based
(based) project.

First, clone the repository down wherever you want. That's your business.
I don't care. Just `cd` into it afterwards.

``` shell
git clone https://github.com/jackjohn7/got_vcs

cd got_vcs
```

Build the project using cargo and specify the `--release` flag.
This command compiles the project into a binary that your computer
can use. The release flag just tells the compiler to perform some
optimizations. Leaving this flag off will result in a slightly slower
tool.

``` shell
cargo build --release
```

The resulting executable will be located in `target/releases/` and named `got`.
You can just add this to your path or place it somewhere that your OS
knows to look for applications.

# Future Plans

- Automated build, test, & release pipelines
- Server-side Got (host remote Got repos)
- Decentralized Got Service
- Installation docs for inferior operating systems

# Why make this?

I think Git is a great tool! I want to learn how a tool like that could
work. I also want to get better at Rust and learn how to ship Rust
applications. I wouldn't recommend anyone actually use this tool over Git.
This is a learning experiment for me.
