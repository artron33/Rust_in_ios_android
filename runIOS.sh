#!/bin/sh
cd Rust

## IOS
cbindgen src/lib.rs -l c > rust.h

IOS_LIBS=../iOS
cargo lipo --release

rm -rf $IOS_LIBS/libs
rm -rf $IOS_LIBS/include
mkdir $IOS_LIBS/libs
mkdir $IOS_LIBS/include

cp rust.h                                $IOS_LIBS/include
cp target/universal/release/librust.a    $IOS_LIBS/libs
