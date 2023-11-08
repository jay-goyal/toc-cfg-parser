# LL1 CFG Parser

This project aims to parse a LL1 unambiguous CFG. It is written in pure Rust.
The parse table generated for a string is a top down parse table. So, the rules can be written down in a top down fashion to get the parse tree
<br />
The frontend is written using [Yew](https://yew.rs/)

### [Web Deployment](https://toc-cfg-parser.pages.dev/)

### Local Deployment

1. Clone the repo
2. Setup [Rust](https://www.rust-lang.org/tools/install)
3. Run `cargo install trunk`
4. Run `trunk serve .`

### Issues

This is just a pet project. But nevertheless, if any valid CFG/String combo results in issues, feel free to point it out in the [issues](https://github.com/jay-goyal/toc-cfg-parser/issues).
I may get to it in the future.
