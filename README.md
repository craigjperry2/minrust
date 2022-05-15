#  minrust

See https://github.com/craigjperry2/mingo

This is the re-write in Rust. Although just like with mingo, there will be detours to play with shiny things.

##  Developer Setup

I installed:

- https://rustup.rs/
- rust-analyzer plugin for VSCode
- CodeLLDB Debugger plugin for VSCode
- ErrorLens plugin for VSCode

I have been playing around a little with Rust, so unlike with the mingo project, i'm not _totally_ new to this language but i'm still very much a beginner.

##  The Plan

Just like with the mingo project, there will be a plan.

Ok... so async has moved on massively since i was last playing with Rust. I think i need an excuse to try out that tokio dashboard cli tool i read about.

1. A basic unit test and some command line flag parsing
    * I'll explore the DevEx, see how test running can be handled.
1. Test drive axum for a static html endpoint
    * I'm itching to try hot reloading of rust code which i already know is a fairly gnarly thing to setup
1. Serve a files endpoint
    * Compare and contrast options like go:embed usage in mingo
1. Persist some stuff in a sqlite db - i think i want to try the SQLx library
1. Serve an AlpineJS + HTMX based CRUD page
1. Deploy in a docker container
