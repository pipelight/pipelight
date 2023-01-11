# General

TL,DR

Coding guidelines for lazy devs who just want to enjoy Rust.
Patterns to code fast and understandable Rust,
and free us from Error Handling hell (and other burdens) to focus on DOING things.

## Developing style

This is my personal arbitrary developping style.
I found it greatly improves code readability (as siimple as JS)

As you don't write firemware pilots, or embeded software, this may suits your needs.

Here, for ease of writting, I don't bother overusing the Heap/References (can slow the execution down),
because every computation made by the code is overly simple,
so it remains very fast, and low ressource consumming.

The "Result" type is used almost everywhere for the sake of
easy top level Error Handeling (yes, in the main function).

## Functions

Functions, wherever they are defined in the code
should only accept references.
And return (if needed) a Type or a Result

```rs
fn  do_stuffs(e: &T) -> U {}
```

## Structs

Structs can either implemet methods or associated functions

### Methods

```rs
fn  do_stuffs(&self){}
```

which I enforce return type to
Self,
so that every method can be chained.
Methods usually contains a few operations only, reducing there risk to fail and unwrap with poorly documented errors.

```rs
fn  do_stuffs(&self) -> Self {}
```

### Associated functions

```rs
fn  do_stuffs(e: &T){}
```

which I enforce return type to
Result<T,Box<dyn Error>>,

```rs
fn  do_stuffs(e: &T) -> Result<T,Box<dyn Error>>
```

Those functions contains more operations than methods.
So that, if something fails, the error is catch and pretty printed

### Struct programming

I like either Object Oriented Programming and Functional Programming.
But, in Rust it seems easier to fold everything in Structs, thanks to those two types of functions relative to structs.

# Project architecture

The struct cast::Config is the "pipelight.config.mjs" file,
just parsed from js/json

It is then converted into types::Config;
The From/Into cascade allows small/ fast understandable functions for deep Struct/Type convertion.

And all the heavy magic is made by bit of easy magic in every struct functions/methods.
