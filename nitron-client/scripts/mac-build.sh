#!/bin/bash

set -e

APP_NAME=nitron-client
MACOS_BIN_NAME=nitron-client
MACOS_APP_NAME=Nitron
MACOS_APP_DIR=mac/$MACOS_APP_NAME.app

cd target
mkdir -p mac
cd mac
echo "Creating app directory structure"
rm -rf $MACOS_APP_NAME
rm -rf $MACOS_APP_DIR
mkdir -p $MACOS_APP_DIR/Contents/MacOS

cargo rustc \
    --verbose \
    --release

echo "Copying binary"
MACOS_APP_BIN=$MACOS_APP_DIR/Contents/MacOS/$MACOS_BIN_NAME
cp ../../target/release/$APP_NAME $MACOS_APP_BIN

echo "Copying assets"
cp -r ../../assets $MACOS_APP_DIR/Contents/MacOS/assets

echo "Copying launcher"
cp ../../scripts/macos_launch.sh $MACOS_APP_DIR/Contents/MacOS/$MACOS_APP_NAME

echo "Copying Icon"
mkdir -p $MACOS_APP_DIR/Contents/Resources
cp ../../resources/Info.plist $MACOS_APP_DIR/Contents/
cp ../../resources/logo.icns $MACOS_APP_DIR/Contents/Resources/

echo "Creating dmg"
mkdir -p $MACOS_APP_NAME
cp -r $MACOS_APP_DIR $MACOS_APP_NAME/
rm -rf $MACOS_APP_NAME/.Trashes

FULL_NAME=$MACOS_APP_NAME

hdiutil create $FULL_NAME.dmg -srcfolder $MACOS_APP_NAME -ov
rm -rf $MACOS_APP_NAME
