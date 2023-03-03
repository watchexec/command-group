# Changelog

## Next (YYYY-MM-DD)

## v2.1.0 (2023-03-04)

- Add new `.group()` builder API to allow setting Windows flags and use `kill_on_drop`. ([#15](https://github.com/watchexec/command-group/issues/15), [#17](https://github.com/watchexec/command-group/issues/17), [#18](https://github.com/watchexec/command-group/issues/18))

## v2.0.1 (2022-12-28)

- Fix bug on Windows where the wrong pointer was being null checked, leading to timeout errors. ([#13](https://github.com/watchexec/command-group/pull/13))

## v2.0.0 (2022-12-04)

- Increase MSRV to 1.60.0 and change policy for increasing it (no longer a breaking change).
- Wait for all processes in the process group, avoiding zombies. ([#7](https://github.com/watchexec/command-group/pull/7))
- Update `nix` to 0.26 and limit features. ([#8](https://github.com/watchexec/command-group/pull/8))

## v1.0.8 (2021-10-16)

- Bugfix: compiling would fail when Tokio was missing the `io-util` feature (not `io-std`).

## v1.0.7 (2021-10-16) (yanked)

- Bugfix: compiling would fail when Tokio was missing the `io-std` feature.

## v1.0.6 (2021-08-26)

- Correctly handle timeouts on Windows. ([#2](https://github.com/watchexec/command-group/issues/2), [#3](https://github.com/watchexec/command-group/pull/3))

## v1.0.5 (2021-08-13)

- Internal: change usage of `feature = "tokio"` to `feature = "with-tokio"`.
- Documentation: remove wrong mention of blocking reads on `AsyncGroupChild::wait_with_output()`.

## v1.0.4 (2021-07-26)

New: Tokio implementation, gated on the `with-tokio` feature.

## v1.0.3 (2021-07-21)

Bugfix: `GroupChild::try_wait()` would error if called after a child exited by itself.

## v1.0.2 (2021-07-21)

Bugfix: `GroupChild::try_wait()` and `::wait()` could not be called twice.

## v1.0.1 (2021-07-21)

Implement `Send`+`Sync` on `GroupChild` on Windows, and add a `Drop` implementation to close handles
too (whoops). Do our best when `.into_inner()` is used...

## v1.0.0 (2021-07-20)

Initial release
