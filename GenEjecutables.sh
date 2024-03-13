#!/bin/sh

#  Ejecutable Linux .elf
cargo build --release --target x86_64-unknown-linux-gnu
mv target/x86_64-unknown-linux-gnu/release/caromail bin/caromail.elf

#  Ejecutable Windows .exe
cargo build --release --target x86_64-pc-windows-gnu
mv target/x86_64-pc-windows-gnu/release/caromail.exe bin
