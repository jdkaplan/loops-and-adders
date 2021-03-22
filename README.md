# Loops & Adders

A collaborative programming art project.
It's essentially the [round-robin][round-robin] story game applied to programming.

[round-robin]: https://en.wikipedia.org/wiki/Round-robin_story

## The Rules

1. Turns are taken in the order specified below.
2. When it's your turn, create a PR and assign it to the next person.

## Turn Order

* [@jdkaplan](https://github.com/jdkaplan)
* [@carlsverre](https://github.com/carlsverre)
* This could be you!  If you'd like this slot, contact someone else in the list.

## Development Setup

1. [Install Rust](https://www.rust-lang.org/learn/get-started) (stable should be fine)
2. `cargo install cargo-make`

Run `cargo make video` to render the video.

If the app panics with `NoAvailableAdapter`, install Vulkan.
This is a known error message: https://github.com/nannou-org/nannou/issues/653
