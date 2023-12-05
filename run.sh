#!/bin/sh
cd Rust

## Android
JNI_LIBS=../Android/app/src/main/jniLibs
cargo build --target aarch64-linux-android --release
cargo build --target armv7-linux-androideabi --release
cargo build --target i686-linux-android --release

rm -rf $JNI_LIBS
mkdir -p $JNI_LIBS/arm64-v8a
mkdir -p $JNI_LIBS/armeabi-v7a
mkdir -p $JNI_LIBS/x86

cp target/aarch64-linux-android/release/librust.so $JNI_LIBS/arm64-v8a/librust.so
cp target/armv7-linux-androideabi/release/librust.so $JNI_LIBS/armeabi-v7a/librust.so
cp target/i686-linux-android/release/librust.so $JNI_LIBS/x86/librust.s

## IOS
IOS_LIBS=../iOS
cargo lipo --release

rm -rf $IOS_LIBS/libs
rm -rf $IOS_LIBS/include
mkdir $IOS_LIBS/libs
mkdir $IOS_LIBS/include

cp rust.h                                $IOS_LIBS/include
cp target/universal/release/librust.a    $IOS_LIBS/libs
