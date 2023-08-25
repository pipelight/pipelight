# Exec crate

## What is inside ?

Same as the **cast crate**, it is a compatibility crate,
between pipelight core and linux kernel processes.

It is marely a convenience abstraction layer to use
the linux kernel processes.

The inner subprocess structs or converted into
struct `exec::Process` that is exploitable by the **pipeline crate**
