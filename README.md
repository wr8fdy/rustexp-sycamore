# Rustexp-sycamore

**This is a port of [rustexp](https://rustexp.lpil.uk/) to [sycamore](https://sycamore-rs.netlify.app/) framework, primeraly made for educational propuses.**

A Rust regular expression editor and tester. It compiles to web assembly and
is served from GitHub pages (from the docs directory). There's no
server-side component at all!

```sh
# Setup
cargo install --locked trunk

# Run dev server
trunk serve

# Build release to docs dir
trunk build --release -d docs --public-url rustexp-sycamore
```
