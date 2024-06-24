# Rust's NSFW detection library adapter for Java
(I didn't come up with a pretty name. It is not a standalone tool by now, so I'm not sure if it needs its own name at all)

It is a part of [ren-java-backend](github.com/LiGM-GH/ren_java_backend)

# How to run?
You will probably need Rust installed, see [rust language installation](https://www.rust-lang.org/tools/install),
you'll also need java, at least, openjdk-11 on linux, as far as I could tell,
then you could:
- `cargo install cargo-binstall` and then `cargo binstall just`
- or instead just `cargo install just`

> Now that everything's ready, you can run `just run`, which might be a little slow, but it will compile Rust part fast, at least,
  or you could run `just release-run`
