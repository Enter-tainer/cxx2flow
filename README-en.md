# cxx2flow

[简体中文](README.md) | [English](README-en.md)

Turn your C/C++ code into flowchart

## Demo

For more demo please refer to [GALLERY](gallery.md)

Two different styles：
| | |
|:-:|:-:|
| polyline | smooth |
|![ployline](assets/polyline.svg)|![curve](assets/curve.svg)|

```cpp
inline int read() {  //快读
  char c = getchar();
  int x = 0, f = 1;
  while (c < '0' || c > '9') {
    if (c == '-') f = -1;
    c = getchar();
  }
  while (c >= '0' && c <= '9') {
    x = x * 10 + c - '0';
    c = getchar();
  }
  return x * f;
}
```

### Error reporting

![error reporting](assets/error_reporting.png)

## Installation

### Compile from source

```bash
cargo install cxx2flow
```

### Prebuilt binary

It is recommended to download prebuilt binary from [Github Release](https://github.com/Enter-tainer/cxx2flow/releases).

## Usage

To compile the generated dot file, you need graphviz. You can also copy the output to online graphviz services such as http://magjac.com/graphviz-visual-editor/ .

```
cxx2flow 0.5.7
mgt <mgt@oi-wiki.org>
Convert your C/C++ code to control flow chart

USAGE:
    cxx2flow [OPTIONS] [ARGS]

ARGS:
    <INPUT>       Sets the path of the input file. e.g. test.cpp
                  If not specified, cxx2flow will read from stdin.
    <FUNCTION>    The function you want to convert. e.g. main [default: main]

OPTIONS:
    -c, --curly              Sets the style of the flow chart.
                             If specified, output flow chart will have curly connection line.
        --cpp                Use C preprocessor.
    -h, --help               Print help information
    -o, --output <OUTPUT>    Sets the output file.
                             If not specified, result will be directed to stdout.
                             e.g. graph.dot
    -t, --tikz               Use tikz backend.
    -V, --version            Print version information

Note that you need to manually compile the dot file using graphviz to get SVG or PNG files.

EXAMPLES:
    cat main.cpp | cxx2flow | dot -Tsvg -o test.svg
    cxx2flow test.cpp | dot -Tpng -o test.png
    cxx2flow main.cpp my_custom_func | dot -Tsvg -o test.svg

Please give me star if this application helps you!
如果这个应用有帮助到你，请给我点一个 star！
https://github.com/Enter-tainer/cxx2flow
```

## Limitations

- The support of preprocessor is based on `cpp`, and is disabled by default. `--cpp` flag is needed to enable it. It will fail if `cpp` does not exist in `PATH`.
- Supported control flow keyword: while，for，if，break，continue，break，return，switch, goto, do-while。
