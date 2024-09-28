#!/bin/bash

for a in tests/isa/*.asm; do b=`echo $a | sed 's/\.asm/.bin/'`; nasm -f bin -DDEBUG -Itests -o$b $a ; done

cargo r --bin emu8086 -- -test -hide-header -show-binary-name -dump-regs-on-halt tests/isa/*.bin
