# Rust

## Triggers

If defined and manual not set, no manual trigger allowed.
Bug: branch check up doesn't work on "trigger".

# Stop

Retrive pipeline pid and Stop by pid to stop attach pipelines

# Features

Add a on_started fallback

increase verbosity from 1 if no flag provided for "pipelight inspect pipeline" becomes

Add signal handeling (for interupted piplines with Ctrl-C or SIGKILL/SiGTERM)

Add Clap version feature flag

Better miette diagnostics

Detect if deno is installed

Add picture in help menu

## Unit Tests

Test multiple bad config file.
Sanitize logs.

## Packaging

Package app for RPM and CentOS
Test inside docker container

## Pretty logs

add colorful outupt when ls
fix bug: multiline string ultra space
remove top \r and top \n

If pipelines have same pid (--attach), link them in logs

Print Pipeline execution time while running (instead of "processing")
