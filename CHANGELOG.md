# CHANGELOG

## v0.6.12

- [x] bug fix: print Pipeline execution time while running (instead of "processing")

## v0.5.11

- [x] feature: pretty logs add allways colorful outupt (ls --color=always)
- [x] Bug fix: On "pipelight ls pipeline_name" increase verbosity by +1 if no flag provided
- [x] rewrite test for multiple bad triggers in config file.
- [x] feature: Add a -vvvv verbosity level to print commands stdout AND stderr
- [x] feature: Add bash autocompletion

## v0.5.6

- [x] Add tag to triggers
- [x] Add globbing patterns for triggers definition

## v0.5.4

- [x] Bug fix: Stop pipeline by pid, instead of name to stop attach pipelines.
- [x] Pass arguments to underlying deno

## v0.5.0

- [x] feature: Add a on_started fallback hook

## v0.4.30

- [x] bug fix: multiline string ultra space

## v0.4.28

- [x] Bug fix: AUR conflicting packages.
- [x] Bug fix: branch check up not implemented for raw "pipelight trigger". (can be done in 20 minute)

## v0.4.27

- [x] bug fix : commands stdout/stderr remove top \r,\n
- [x] Package app for RPM and CentOS
- [x] Test builds inside docker container
- [x] feature: start implementation of better error diagnostics with miette
- [x] feature: add userprompt select menu with "pipelight inspect and run" temporary solution before bash completion is ready
- [x] test: test multiple bad config file.
