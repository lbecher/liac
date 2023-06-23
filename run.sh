#!/bin/bash
llc llvm.ll -filetype=obj
clang llvm.o -o main
rm llvm.o