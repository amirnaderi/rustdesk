#!/usr/bin/env bash

echo $MACOS_CODESIGN_IDENTITY
cargo install flutter_rust_bridge_codegen --version 1.75.3 --features uuid
cd flutter; flutter pub get; cd -
~/.cargo/bin/flutter_rust_bridge_codegen --rust-input ./src/flutter_ffi.rs --dart-output ./flutter/lib/generated_bridge.dart --c-output ./flutter/macos/Runner/bridge_generated.h
./build.py --flutter
rm dshelpdesk-$VERSION.dmg
# security find-identity -v
codesign --force --options runtime -s $MACOS_CODESIGN_IDENTITY --deep --strict ./flutter/build/macos/Build/Products/Release/DsHelpDesk.app -vvv
create-dmg --icon "DsHelpDesk.app" 200 190 --hide-extension "DsHelpDesk.app" --window-size 800 400 --app-drop-link 600 185 dshelpdesk-$VERSION.dmg ./flutter/build/macos/Build/Products/Release/DsHelpDesk.app
codesign --force --options runtime -s $MACOS_CODESIGN_IDENTITY --deep --strict dshelpdesk-$VERSION.dmg -vvv
# notarize the dshelpdesk-${{ env.VERSION }}.dmg
rcodesign notary-submit --api-key-path ~/.p12/api-key.json  --staple dshelpdesk-$VERSION.dmg
