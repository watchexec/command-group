# Changelog

## Next (YYYY-MM-DD)

Bugfix: `GroupChild::try_wait()` and `::wait()` could not be called twice.

## v1.0.1 (2021-07-21)

Implement `Send`+`Sync` on `GroupChild` on Windows, and add a `Drop` implementation to close handles
too (whoops). Do our best when `.into_inner()` is used...

## v1.0.0 (2021-07-20)

Initial release
