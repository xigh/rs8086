# simple 8086 emulator in rust

!!! WIP !!! (not fully working) ... I'll keep working on it until it's fully working, and then I'll remove this disclaimer.

## For the curious

I started this project because I was curious how the 8086 worked, and I wanted to see how far I could get with
emulating it in Rust.

It does not rely on any external libraries, all the logic is implemented in Rust (except for logging of course).

It is expected to come with a lot of bugs at the beginning. If you want to contribute, please do so, but keep in mind that I'll work on it everyday and it will take some time to reach a stable state.

I plan to add : 

- [ ] add a GUI (both desktop and web)
- [ ] add a debugger
- [ ] add a disassembler
- [ ] add an assembler

## License

This project is licensed under the MIT license. Do whatever you want with it, but keep in mind that it may contain bugs.

## How to compile a test ROM :

For the moment, you'll require [NASM](https://www.nasm.us/) to assemble the file.

```
nasm -f bin -o tests/1.bin tests/1.asm
```

## Building and running

```
cargo run --bin=emu8086 -- <rom-file.bin>
```

![capture-2024-09-27](./docs/imgs/capture-2024-09-27.png)
