# Changelog

All notable changes to this project will be documented in this file.

## [0.10.0]

### Add

- _(doc)_ Breaking changes warning

## [0.9.12] - 2024-11-27

### 🐛 Bug Fixes

- Handle annotated tags and unannotated tags

## [0.9.11-3] - 2024-11-26

### Hot-fix

- Branch detection

## [0.9.11] - 2024-11-26

### 🚀 Features

- _(doc)_ Git-cliff usage for automated changelog

### 🐛 Bug Fixes

- _(git)_ Get branch name

### Hot-fix

- Silence git-tag function

## [0.9.10] - 2024-11-26

### 🚜 Refactor

- Move concerns into big crate

## [0.9.8] - 2024-11-24

### 🐛 Bug Fixes

- Logs tree view missing newline when displaying a running command

## [0.9.7] - 2024-11-24

### 🐛 Bug Fixes

- 'logs rm' does not remove content in .pipelight/proc

## [0.9.6] - 2024-11-24

### 🐛 Bug Fixes

- Release build error
- Cli/src/test/mod.rs test code could not find `types` in `services`

## [0.9.4] - 2024-11-23

### 🐛 Bug Fixes

- Bad logging due to mutex lock. reversed code to unsafe.

## [0.9.1] - 2024-10-23

### 🐛 Bug Fixes

- Cmd line bad parsing

## [0.8.2] - 2024-09-28

### 🐛 Bug Fixes

- _(flake)_ Install autocompletion scripts
- _(nix)_ Import missing function
- _(install)_ Nix buildInputs for darwin (by EleHeHijEl)
- _(kdl)_ Silence kdl support
- _(git)_ Hide teleport behind git feature
- _(deps)_ Set static version

### 🚜 Refactor

- _(versioning)_ Update version number to minor
- _(watcher)_ Simpler functions. remove self reconfiguration
- _(proc management)_ Filter by gid
- _(exec)_ Simple proc finder
- _(kdl)_ Silence kdl support
- _(utils)_ Hide git behind feature flag
- _(files)_ Move file parsing functions into its own crate
- _(crate)_ Add error specific crate
- _(utils)_ Add back small file manipulation func to utils

### 📚 Documentation

- _(exec)_ Ashamed of this tmp fix
- _(test)_ Fix some doc test

### Hotfix

- _(process_kill)_ Watcher specific function

## [0.8.0] - 2024-08-19

### 🐛 Bug Fixes

- _(deps)_ Pipelight_utils to fix version number
- _(finder)_ Split logic into readable functions

### 🚜 Refactor

- _(lib)_ Update exec crate to lib

### 📚 Documentation

- _(crate.io)_ Update main page

## [0.7.27] - 2024-08-04

### 🚀 Features

- _(md)_ Add config lang examples to readme
- _(exit_code)_ Add exit code on attached pipeline

### 🐛 Bug Fixes

- _(print)_ Remove print statement

### 📚 Documentation

- _(main)_ Add top level doc
- _(utils)_ Add warnings about API not stable

## [0.7.26] - 2024-07-28

### 🐛 Bug Fixes

- _(template)_ Hcl replace block with array

### Remove

- _(nix)_ Deprec nix funct

## [0.7.25] - 2024-07-28

### 🚀 Features

- _(lib)_ Add general error enum
- _(error)_ Add lib specific err enum
- _(roadmap)_ Add a roadmap to the README
- _(parser)_ Add hcl error wrapper
- _(hcl)_ Add test files
- _(hcl)_ Add hcl template
- _(test)_ Add hcl support tests

### 🐛 Bug Fixes

- _(crate)_ Specify lib and bin target
- _(cargo)_ Lib name
- _(cargo)_ Update deps to latest
- _(html)_ Close tag
- _(readme)_ Set gif width in px
- _(policy)_ Circumvent github restrictions

### 🚜 Refactor

- _(crate)_ Set available crate name for crate.io publication
- _(cargo)_ Entry point for bin target and lib
- _(cargo)_ Implicit targets
- _(crate)_ Rename entrypoints
- _(crate)_ Rename core crate
- _(crate)_ Rollback names
- _(lib)_ Add a specific lib entrypoint
- _(exec crate)_ Move exec crate into utils
- _(utils)_ Mv general file related func to utils crate

## [0.7.22] - 2024-03-21

### 🚀 Features

- _(config)_ Config overrides cli args

### 🐛 Bug Fixes

- _(watcher)_ Do not kill parent process

### 🚜 Refactor

- _(options)_ Read config before detaching Service.Run

### Hotfix

- Watcher refuse to launch

### New

- Pipelight watcher doesn't need an ignore file

## [0.7.16] - 2023-12-12

### 🚜 Refactor

- Simpler readme

## [0.7.15] - 2023-12-02

### 🐛 Bug Fixes

- Set `run -vvv` verbose command line argument priority over config flag

## [0.7.14] - 2023-12-01

### 🐛 Bug Fixes

- There was a mistake in the shell script

## [0.7.8] - 2023-11-10

### Add

- Baked new error color theme

## [0.7.4] - 2023-10-20

### Lint

- Cargo clippy

## [0.7.0] - 2023-10-20

### 🚀 Features

- Add prebuild config template generation

### 🐛 Bug Fixes

- Better clap arguments

### 🚜 Refactor

- Clear top command groups
- Splitted big crates in sub crates
- Stability increase. static mut to lazy mutex
- Move process management to exec crate
- Drop watchexec comfy API usage for tokio

### 📚 Documentation

- Add functions description

### 🧪 Testing

- Add config global assertion test

### Add

- Sigkill every processes

### Readd

- Human readable duration

## [0.6.15] - 2023-08-25

### 🚜 Refactor

- Syntax changes do yo cargo clippy

## [0.6.12] - 2023-08-07

### Change

- Prinnt branch before action in pretty logs

## [0.5.10] - 2023-07-04

### 🚜 Refactor

- New child process execution paradigm
- Use readable re export pattern
- Simpler module types and trait exports

### Add

- Log -vvvv, trace flag for log pretty stdout/stderr

## [0.4.24] - 2023-03-21

### 🚀 Features

- Abort pipeline execution

### Bug

- Indent level

### Update

- Pipeline tree printing limited recursive model
- Set logger to arc mutex

<!-- generated by git-cliff -->
