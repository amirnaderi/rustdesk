#!/usr/bin/env bash

cargo install flutter_rust_bridge_codegen --version 1.80.1 --features uuid
flutter pub get
~/.cargo/bin/flutter_rust_bridge_codegen --rust-input ../src/flutter_ffi.rs --dart-output ./lib/generated_bridge.dart --c-output ./macos/Runner/bridge_generated.h
# call `flutter clean` if cargo build fails
# export LLVM_HOME=/Library/Developer/CommandLineTools/usr/
CARGO_PROFILE_DEV_BUILD_OVERRIDE_DEBUG=true RUST_BACKTRACE=full cargo build --features "flutter,flutter_texture_render"
flutter run $@
