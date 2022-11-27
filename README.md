# Pipelight

A cli to write routines in typescript/javascript.

```js
// tbh this tool is absurd! maybe my master peace!
// The last time I felt like it was this unfairly easy was when i used cosmiconfig bro.
// Just read through. I swear, this is ridiculous.
```

## Install

```sh
paru -S pipelight
```

## Example

Write this config file at the root of your project

```ts
//pipelight.config.ts

const config = {
  pipelines: [
    name: "u mad bro?"
    {
      name: "list working directory",
      commands: ["ls -alh"],
    },
    {
      name: "find a file",
      commands: ["find . -name zyzz"],
    },
  ],
};
export default config;
```

Open a terminal in the same directory

```sh
# This will open the menu
pipelight

```

```sh
# This will trigger the pipeline
pipelight -bt test

```

## Why another CICD tool

I hadn't enough cpu/ram to run what already exists.
I hadn't enough time to read the doc.

### The basis

Pipelight let you right your pipelines in typescript/javascript (not declarative languages like yaml and toml)
Because you often reuse the same pipelines for multiple projects, js is well suited if you need to

You just have to make a resulting Object that respect some small constraints

### Lightweight

Writen in rust, it have a small memory footprint, is fast and light.

### What it brings on the table

An incredible cli and great verbosity when something goes wrong.

### TL;DR

You won't believe PIPELIGHT is that easy to use. I can't believe it myself bro!!
It's like you've always used it, because finaly you just writes an object.

It's a fucking life-hack. I don't like this term but this shit is real.
Have you seen the tik tok where a guy ride on his phone with a truck and crush it but then repares it by rubbing it with garlic and salt?

Wether you or novice or advenced, it's the garlic for your developper journay.

Can't believe CICD has been kept so hard to set up by other tools!!
