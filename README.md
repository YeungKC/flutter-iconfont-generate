# Flutter Iconfont Generate

Iconfonts are commonly used in Flutter, and typically there is a CSS file that describes the relationship between unicode and name. This tool converts the CSS file into a Dart file, making it easy to use in Flutter.

## Setup

Install Rust and Cargo environment. Please refer to [rustup](https://rustup.rs/) for specific instructions.

## Build

bash

```shell
git clone git@github.com:YeungKC/flutter-iconfont-generate.git
cd flutter-iconfont-generate
cargo build --release
```

## Usage

Usage: flutter-iconfont-generate [OPTIONS] <INPUT_FILE> <OUTPUT_FILE>

Arguments:
  <INPUT_FILE>
  <OUTPUT_FILE>

Options:
  -p, --package <PACKAGE>
  -c, --class-name <CLASS_NAME>  [default: IconFonts]
  -o, --overwrite
  -h, --help                     Print help
  -V, --version                  Print version

### Example

bash

```shell
flutter-iconfont-generate ./iconfont.css ./lib/generated/iconfont.dart
```
