# cxx2flow

将 C/C++ 代码转换为流程图

## 效果

两种样式：
| | |
|:-:|:-:|
|折线|平滑|
|![ployline](assets/polyline.svg)|![curve](assets/curve.svg)|


## 安装

### 自行编译

```bash
cargo install cxx2flow
```

### 下载预构建二进制

可以到 [GitHub Actions](https://github.com/Enter-tainer/cxx2flow/actions?query=branch%3Amaster+is%3Asuccess+event%3Apush+actor%3AEnter-tainer) 或 [Nightly.link](https://nightly.link/Enter-tainer/cxx2flow/workflows/build/master) 下载最新构建的二进制，包含 Linux 和 Windows 版本。

## 使用

为了编译生成的 dot 文件，你需要安装 graphviz，并将其添加到 PATH 中。也可以将生成的结果复制进在线的 graphviz 服务中，如 http://magjac.com/graphviz-visual-editor/ 。

```
cxx2flow 0.1.5
mgt. <mgt@oi-wiki.org>
Convert your C/C++ code to control flow chart

USAGE:
    cxx2flow [FLAGS] [OPTIONS] <INPUT> [FUNCTION]

FLAGS:
    -c, --curved     Sets the style of the flow chart.
                     If specified, output flow chart will have curved connection line.
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -o, --output <OUTPUT>    Sets the output file.
                             If not specified, result will be directed to stdout.
                             e.g. graph.dot

ARGS:
    <INPUT>       Sets the input file. e.g. test.cpp
    <FUNCTION>    The function you want to convert. e.g. main

Note that you need to manually compile the dot file using graphviz to get SVG or PNG files.
EXAMPLES:
    cxx2flow test.cpp | dot -Tpng -o test.png
    cxx2flow main.cpp my_custom_func | dot -Tsvg -o test.svg
```

## 限制

- 暂时不支持 switch 和 goto
- 不支持预处理器，如 include, ifdef, ifndef...
- 支持的控制流语句有：while，for，if，break，continue，break，return。
