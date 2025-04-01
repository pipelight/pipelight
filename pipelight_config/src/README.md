# The config loader

This crate loads configurations files and deserializes them.

Load from FHS paths:

- XDG paths,
  /etc/<program_name>/config.<lang>
  and ~/.config/<program_name>/config.<lang>

- Custom paths.

And fuses them into a single entry.
