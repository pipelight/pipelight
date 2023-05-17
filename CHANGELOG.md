# v0.4.27

- [x] bug fix : commands stdout/stderr remove top \r,\n
- [x] Package app for RPM and CentOS
- [x] Test builds inside docker container
- [x] feature: start implementation of better error diagnostics with miette
- [x] feature: add userprompt select menu with "pipelight inspect and run" temporary solution before bash completion is ready

# v0.4.28

- [] feature: Add a -vvvv verbosity level to print commands stdout AND stderr
- [] feature: Add bash autocompletion
- [x] Bug fix: AUR conflicting packages.
- [x] Bug fix: branch check up not implemented for raw "pipelight trigger". (can be done in 20 minute)
- [] Bug fix: Stop pipeline by pid, instead of name to stop attach pipelines.
- [] feature: Add a on_started fallback hook
- [] Bug fix: On "pipelight ls pipeline_name" increase verbosity by +1 if no flag provided
- [] Add signal handeling (for interupted piplines with Ctrl-C or SIGKILL/SiGTERM)

# v0.4.29

- [] feature: pretty logs add allways colorful outupt (ls --color=always)
- [] Add Clap version feature flag
- [] Detect if deno is installed
- [] bug fix: print Pipeline execution time while running (instead of "processing")

# v0.4.30

- [] bug fix: multiline string ultra space
- [] Sanitize logs before reading
- [] test: test multiple bad config file.
- [] feature: if pipelines have same pid (--attach), link them in logs tree.
