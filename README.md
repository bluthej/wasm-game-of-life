# WASM game of life

This is my implementation of [Conway's game of life](https://en.wikipedia.org/wiki/Conway%27s_Game_of_Life) using WASM, based of the [tutorial](https://rustwasm.github.io/docs/book/game-of-life/introduction.html) of the Rust and WebAssembly book.

I didn't do anything fancy, I am not a web developer but I was curious and I like 🦀 a lot.

# Note

There are a couple of things that might need updating in the tutorial, but mostly everything works fine. There is one thing to note though, I could not get the development server to work just by following the instructions, but looking online I found a few GitHub issues related to this which led me to use the following command instead of the one given in [this section](https://rustwasm.github.io/docs/book/game-of-life/hello-world.html#serving-locally):

```shell
NODE_OPTIONS=--openssl-legacy-provider npm run start
```
