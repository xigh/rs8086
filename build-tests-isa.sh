#!/bin/bash

log=
for arg in "$@"; do
    if [[ $arg == -log-* ]]; then
        log="$arg"
        break
    fi
done


for a in tests/isa/*.asm; do
    b=`echo $a | sed 's/\.asm/.bin/'`
    l=`echo $a | sed 's/\.asm/.dis/'`
    rm -rf $b
    nasm -f bin -DDEBUG -Itests -o$b $a
    if [ ! -f "$b" ]; then
        echo "Error: Failed to compile $a. Output file $b does not exist."
        exit 1
    fi
    rm -rf $l
    ndisasm -b 16 $b > $l
done

cargo r --bin emu8086 -- -test -hide-header $log -show-binary-name -dump-regs-on-halt tests/isa/*.bin
