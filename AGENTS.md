Overview
--------

This is a website which aims to create fake social (Facebook) stories for anime characters.
There is one real user is the admin. The admin can choose which character will do what thing and make it look like a facebook post/comment/chat/group...

Specs and design can be found in folder docs/.
If spec or design is changed, update those documents.

Cargo option
------------

* Lint check: Use `cargo check` instead of `cargo clippy`.
* Do not `cargo test --all-targets`, use default target is enough.
* Do not run cargo fmt. I will format the code by myself.
* DO NOT setup a separate build dir to avoid wasting my storage.
* DO NOT create temp files in my $HOME folder.

How to learn a crate
--------------------

1. This repo — existing use lines, Cargo.toml versions, code that already compiled.

2. Find that crate’s tree on disk (from Cargo.toml), then read examples/ first.
    Where the code lives depends on how the dep is declared:

| Cargo.toml                                                  | Look here |
|:------------------------------------------------------------|:------|
| crates.io (e.g. bevy = "0.19")                              | ~/.cargo/registry/src/index.crates.io-*/<crate>-<version>/ |
| git (e.g. bevy = { git = "…", rev/tag/branch = "…" })       | ~/.cargo/git/checkouts/<repo>-*/<short-rev>/ |
| path (e.g. bevy_text_edit = { path = "../bevy_text_edit" }) | that path, relative to the crate that declared it |

Then, in that tree:
1. examples/ (usage)
2. README.md and src/lib.rs module docs
3. Implementation .rs only for the symbol that failed to compile (search the exact type name)

Do not read bevy_ecs internals “to understand ECS.” Do not cargo run -p bevy --example ... (full example builds are huge). Do not scrape docs.rs if examples/ already shows the API. Do not clone a second copy of a git/path dep that is already on disk.

Resolve git checkouts: ls ~/.cargo/git/checkouts/ and match the repo name. Several folders may exist (different revs); prefer the rev/tag in Cargo.lock / Cargo.toml.

Resolve path deps: path is relative to the package Cargo.toml that lists it, not to the workspace root, unless the path is written that way.

MCP
---

For “where is X defined / who calls this / impact of a change”:
If codegraph was installed and configured, use codegraph_explore first (MCP).
Grep only for strings, comments, configs, generated files.
Do not dump whole crates to find one symbol.

Database
--------

* Database is running inside podman. If you need psql command, go there.
* Do not run migration by psql, use `cargo sqlx migrate run --source db/migrations/postgres/ -D postgresql://dev:dev@localhost:5432/mediaboard` instead.
* `cargo sqlx prepare` is not necessary. Do not run it. I will run it by myself.

Others
------

* Search the repo (rg) before creating a helper that already exists.
* If cargo check is clean, stop. Do not keep editing for style.
* Always tell me what are you want to do when asking for permision to run shell command.
* Server is already watched on both `src` and `res`, and auto restart when have change by `bacon`. Do not need to check the binary status or start/restart server by yourself. Just use the running web.
* Do not use google-chrome or chromium. Firefox is already installed with flatpak. Use flatpak run command.
* Temp data and output should be put in /tmp.
* If you get too much trouble to test, stop testing and tell me. I will check by myself.
* I do not want you flood the project with your test code. Remove your test code after the job is done.
* Only create and perform test if you think the fix is big, difficult or uncertain. For small changes or easy changes or confident changes, skip the test. I can check by myself.
