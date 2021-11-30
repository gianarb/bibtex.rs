Thank you for being here!

## Tools

I do my best to use fewest tools as possible. Cargo is enough. I use Make
to highlight a few commands used often for testing and fuzzing.

## Commit standard

I decided to follow the ["Conventional
Commits"](https://www.conventionalcommits.org/en/v1.0.0/) standard for this
repository. Mainly because I would like to use the `scope` to detect are of interest, for example:

* `rust`: used to specify the goal for a commit to improve how Rust is used in
  this library. For example an attempt to make the codebase more "Rust
  idiomatic"
* `fuzz`: commit related to fuzzing tool

## Continuous Integration

This project uses GitHub Action. I would like to refactor the way it is used
right now because it relays on actions that I can't run locally.

I am a Nix user. I would like to keep GitHub action only as a runner leaving to
nix-shell the ownership for provisioning the right environment, leaving to me
the ability to run the same CI/CD checks locally.
