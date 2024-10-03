#!/bin/bash

log=
test_files=()

for arg in "$@"; do
    if [[ $arg == -log-* ]]; then
        log="$arg"
    else
        test_files+=("$arg")
    fi
done

echo "Test files: ${test_files[*]}"

for a in tests/isa/*.asm; do
    fname=$(basename "$a" .asm)
    # Skip if test_files is not empty and the current file is not in the list
    if [ ${#test_files[@]} -ne 0 ] && [[ ! " ${test_files[@]} " =~ " $fname " ]]; then
        continue
    fi

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

# Adjust the cargo command to use only the selected test files
if [ ${#test_files[@]} -eq 0 ]; then
    test_binaries="tests/isa/*.bin"
else
    test_binaries=$(printf "tests/isa/%s.bin " "${test_files[@]}")
fi

cargo r --bin emu8086 -- -test -hide-header $log -show-binary-name -dump-regs-on-halt $test_binaries