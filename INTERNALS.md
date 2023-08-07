# Internal API

Pipelight has well matured since its first prototype in 2021 (Simpcicd fully written in typescript/nodejs).
The command line interface won't change as often as before and as the number of users grows,
the software is getting more and more structured, tested and documented.

I feel it is now time for the internal functionning to be explained with schemas and all.
Here will be a quick overview of the crate, functions and global variables roles and interdependencies explanation.

For you to understand many architectural choices, you first need to know with which philosophy te software was built.

It must be:

- minimal
- fast
- sovereign

Born from those philosohies:

- [The Anti Mediocracy Manifesto for Software Development](https://gist.github.com/mathiasrw/cb3b15630a418f5cff3035463a048a59)
- [The LunarDAO manifesto](https://lunardao.net/manifesto.html)

## Functionning

When running a pipeline. This happens.

Read config file -> Create a Inner Object (Pipeline struct) -> Run processes in the defined order while logging.
