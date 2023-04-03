<h1 align="center">
  embin
  </a>
</h1>

<h4 align="center">A simple program that can embed binary or text files into source code of a specific language</h4>

<p align="center">
  <a href="https://github.com/martin-olivier/embin/releases/tag/v1.1.2">
    <img src="https://img.shields.io/badge/Version-1.1.2-blue.svg" alt="version"/>
  </a>
  <a href="https://github.com/martin-olivier/embin/blob/main/LICENSE">
    <img src="https://img.shields.io/badge/License-MIT-darkgreen.svg" alt="license"/>
  </a>
  <a href="https://www.rust-lang.org/">
    <img src="https://img.shields.io/badge/Language-Rust-orange.svg" alt="cppversion"/>
  </a>
  <a href="https://github.com/martin-olivier/embin/actions/workflows/CI.yml">
    <img src="https://github.com/martin-olivier/embin/actions/workflows/CI.yml/badge.svg" alt="ci"/>
  </a>
</p>

## Installation

You can find pre-built [releases](https://github.com/martin-olivier/embin/releases/latest) for `linux`, `macOS` and `Windows`

Otherwise, you can install `embin` from source using [cargo](https://www.rust-lang.org/tools/install), with the following command:

```sh
cargo install embin
```

`⭐ Don't forget to put a star if you like the project!`

## Usage

```
Usage: embin [OPTIONS] <INPUT>...

Arguments:
  <INPUT>...  Input file(s) to embed, which can be binary or text files

Options:
  -o, --output <OUTPUT>      Write generated source code in output file instead of stdout
      --lang <LANG>          Language of the generated source code [default: c]
      --format <FORMAT>      Format of the generated source code [default: hex]
      --indent <INDENT>      Indentation type of the generated source code [default: space]
      --padding <PADDING>    Padding value of the generated source code [default: 4]
      --quantity <QUANTITY>  Number of byte elements per line [default: 16]
      --mutable              Make generated variables mutable
  -h, --help                 Print help
  -V, --version              Print version
```

## Examples

### Embedding a file into C

```sh
embin --lang=c data.png > output.h
```

Result:

```c
const unsigned char data_png[] = {
    0x54, 0x68, 0x69, 0x73, 0x20, 0x69, 0x73, 0x20, 0x61, 0x20, 0x74, 0x65,
    0x73, 0x74, 0x20, 0x66, 0x69, 0x6c, 0x65, 0x2e, 0x0a, 0x0a, 0x54, 0x68,
    0x69, 0x73, 0x20, 0x69, 0x73, 0x20, 0x61, 0x20, 0x6e, 0x65, 0x77, 0x20,
    0x6c, 0x69, 0x6e, 0x65, 0x2e, 0x0a
};
const unsigned int data_png_len = 42;
```

### Embedding a file into C++

```sh
embin --lang=cpp data.png > output.hpp
```

Result:

```cpp
#include <array>

constexpr std::array<unsigned char, 42> data_png = {
    0x54, 0x68, 0x69, 0x73, 0x20, 0x69, 0x73, 0x20, 0x61, 0x20, 0x74, 0x65,
    0x73, 0x74, 0x20, 0x66, 0x69, 0x6c, 0x65, 0x2e, 0x0a, 0x0a, 0x54, 0x68,
    0x69, 0x73, 0x20, 0x69, 0x73, 0x20, 0x61, 0x20, 0x6e, 0x65, 0x77, 0x20,
    0x6c, 0x69, 0x6e, 0x65, 0x2e, 0x0a
};
```

### Embedding a file into Python

```sh
embin --lang=python data.png > output.py
```

Result:

```python
DATA_PNG = bytes([
    0x54, 0x68, 0x69, 0x73, 0x20, 0x69, 0x73, 0x20, 0x61, 0x20, 0x74, 0x65,
    0x73, 0x74, 0x20, 0x66, 0x69, 0x6c, 0x65, 0x2e, 0x0a, 0x0a, 0x54, 0x68,
    0x69, 0x73, 0x20, 0x69, 0x73, 0x20, 0x61, 0x20, 0x6e, 0x65, 0x77, 0x20,
    0x6c, 0x69, 0x6e, 0x65, 0x2e, 0x0a
])
```

> ⚠️ On Windows, use `--output` instead of `>` when generating your embedded assets to avoid the following error when running python code:
>
> `source code string cannot contain null bytes`

### Embedding multiple files

```sh
embin --lang=cpp assets/icon.png assets/banner.png > assets.hpp
```

Result:

```c++
#include <array>

constexpr std::array<unsigned char, 42> icon_png = {
    0x54, 0x68, 0x69, 0x73, 0x20, 0x69, 0x73, 0x20, 0x61, 0x20, 0x74, 0x65,
    0x73, 0x74, 0x20, 0x66, 0x69, 0x6c, 0x65, 0x2e, 0x0a, 0x0a, 0x54, 0x68,
    0x69, 0x73, 0x20, 0x69, 0x73, 0x20, 0x61, 0x20, 0x6e, 0x65, 0x77, 0x20,
    0x6c, 0x69, 0x6e, 0x65, 0x2e, 0x0a
};

constexpr std::array<unsigned char, 86> banner_png = {
    0x69, 0x73, 0x20, 0x61, 0x20, 0x6e, 0x65, 0x77, 0x20, 0x6c, 0x69, 0x6e,
    0x65, 0x2e, 0x0a, 0x54, 0x68, 0x69, 0x73, 0x20, 0x69, 0x73, 0x20, 0x61,
    0x20, 0x74, 0x65, 0x73, 0x74, 0x20, 0x66, 0x69, 0x6c, 0x65, 0x2e, 0x0a,
    0x0a, 0x54, 0x68, 0x69, 0x73, 0x20, 0x69, 0x73, 0x20, 0x61, 0x20, 0x6e,
    0x65, 0x77, 0x20, 0x6c, 0x69, 0x6e, 0x65, 0x2e, 0x0a, 0x54, 0x68, 0x69,
    0x73, 0x20, 0x69, 0x73, 0x20, 0x61, 0x20, 0x74, 0x65, 0x73, 0x74, 0x20,
    0x66, 0x69, 0x6c, 0x65, 0x2e, 0x0a, 0x0a, 0x54, 0x68, 0x69, 0x73, 0x20,
    0x73, 0x20
};
```

## Options

### `--format` Set the format of the generated source code

`Char` format:

```sh
embin --format=char data.txt
```

```c
const unsigned char data_txt[] =
    "This is a test file.\n\n"
    "This is a new line.\n";
const unsigned int data_txt_len = 42;
```

`Octal` format:

```sh
embin --format=octal data.txt
```

```c
const unsigned char data_txt[] =
    "\124\150\151\163\040\151\163\040\141\040\164\145\163\164\040\146"
    "\151\154\145\056\012\012\124\150\151\163\040\151\163\040\141\040"
    "\156\145\167\040\154\151\156\145\056\012";
const unsigned int data_txt_len = 42;
```

### `--indent` Set indentation type of the generated source code

Indent with tabs instead of spaces:

```sh
embin --indent=tab data.png
```

### `--padding` Set padding value of the generated source code

```sh
embin --padding 0 data.png
```

```c
const unsigned char data_png[] = {
0x54, 0x68, 0x69, 0x73, 0x20, 0x69, 0x73, 0x20, 0x61, 0x20, 0x74, 0x65,
0x73, 0x74, 0x20, 0x66, 0x69, 0x6c, 0x65, 0x2e, 0x0a, 0x0a, 0x54, 0x68,
0x69, 0x73, 0x20, 0x69, 0x73, 0x20, 0x61, 0x20, 0x6e, 0x65, 0x77, 0x20,
0x6c, 0x69, 0x6e, 0x65, 0x2e, 0x0a
};
const unsigned int data_png_len = 42;
```

### `--quantity` Set number of byte elements per line

```sh
embin --quantity 8 data.png
```

```c
const unsigned char data_png[] = {
    0x54, 0x68, 0x69, 0x73, 0x20, 0x69, 0x73, 0x20,
    0x61, 0x20, 0x74, 0x65, 0x73, 0x74, 0x20, 0x66,
    0x69, 0x6c, 0x65, 0x2e, 0x0a, 0x0a, 0x54, 0x68,
    0x69, 0x73, 0x20, 0x69, 0x73, 0x20, 0x61, 0x20,
    0x6e, 0x65, 0x77, 0x20, 0x6c, 0x69, 0x6e, 0x65,
    0x2e, 0x0a
};
const unsigned int data_png_len = 42;
```

### `--mutable` Make generated variables mutable

```sh
embin --mutable data.png
```

```c
unsigned char data_png[] = {
    0x54, 0x68, 0x69, 0x73, 0x20, 0x69, 0x73, 0x20, 0x61, 0x20, 0x74, 0x65,
    0x73, 0x74, 0x20, 0x66, 0x69, 0x6c, 0x65, 0x2e, 0x0a, 0x0a, 0x54, 0x68,
    0x69, 0x73, 0x20, 0x69, 0x73, 0x20, 0x61, 0x20, 0x6e, 0x65, 0x77, 0x20,
    0x6c, 0x69, 0x6e, 0x65, 0x2e, 0x0a
};
unsigned int data_png_len = 42;
```

## License

This project is released under [MIT](LICENSE) license.

## Contributing

Pull requests are welcome. For major changes, please open an issue first to discuss what you would like to change.
