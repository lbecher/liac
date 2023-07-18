#!/bin/bash
cargo run --release -- $1
cp target/debug/liac bin/liac
#./bin/liac $1 > saida.txt
llc llvm.ll -filetype=obj -relocation-model=pic
clang -fPIE llvm.o -o main
rm llvm.o