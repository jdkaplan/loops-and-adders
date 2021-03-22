# Loops & Adders

A collaborative programming art project.
It's essentially the [round-robin][round-robin] story game applied to programming.

[round-robin]: https://en.wikipedia.org/wiki/Round-robin_story

# The Rules

1. Turns are taken in the order specified below.
2. When it's your turn, create a PR with a single commit that adds xor removes code.

# Turn Order

* [@jdkaplan](https://github.com/jdkaplan)
* [@carlsverre](https://github.com/carlsverre)
* This could be you!  Contact someone else in the list.

# Development Setup

We're trying to keep everything in Rust as much as possible.

1. [Install Rust](https://www.rust-lang.org/learn/get-started) (stable should be fine)
2. `cargo install cargo-make`

Run `cargo make video` to render the video.

If the app panics with `NoAvailableAdapter`, install Vulkan.
This is a known bad error message: https://github.com/nannou-org/nannou/issues/653
