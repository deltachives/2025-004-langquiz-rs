<div align="center">
  Lang Quiz
  <p>Simple program to quiz on languages</p>
</div>

<p align="center">
  <a href="https://github.com/deltachives/2025-004-langquiz-rs/actions/workflows/rust.yml?query=branch%3Amain">
    <img src="https://github.com/deltachives/2025-004-langquiz-rs/workflows/Rust/badge.svg"
         alt="Rust">
  </a>&nbsp;
  <a href="https://opensource.org/licenses/MIT">
    <img src="https://img.shields.io/badge/License-MIT-brightgreen.svg"
         alt="License: MIT" >
  </a>&nbsp;
    <a href="https://www.buymeacoffee.com/lan22h" target="_blank">
    <img src="https://cdn.buymeacoffee.com/buttons/v2/default-blue.png"
         alt="Buy Me A Coffee" height="20">
  </a>
</p>

<div align="center">
  <sub>Built with ❤︎ by Mohammed Alzakariya
</div>
<br>

- [Purpose](#purpose)
- [Contributing](#contributing)
- [License](#license)

# Purpose

Simple program to quiz vocab and sentences in various languages.

Quiz convert mode requires a csv where the program displays a value from a column, and requires a value from another column to be filled.

# Example Usage

```sh
cat <(cat << 'EOF'
pinyin,pinyin_pitch,hanzi,english
ni,ni3,你,you
hao,hao3,好,good
ni'hao,ni3hao3,你好,hello
shuo,shuo1,说,speak
EOF
) > words_cn.csv
```

Now you can quiz your memory on any representation. For example, hanzi to pinyin_pitch:

```sh
cargo run quiz_convert words_cn.csv "hanzi" "pinyin_pitch"
```

```
Enter the corresponding pinyin_pitch for 你 or type [q]uit:
ni3
That is correct!
Enter the corresponding pinyin_pitch for 说 or type [q]uit:
shuo1
That is correct!
Enter the corresponding pinyin_pitch for 说 or type [q]uit:
shuo1
That is correct!
Enter the corresponding pinyin_pitch for 你好 or type [q]uit:
ni'hao
retry? ([y]es/[n]o)
y
Enter the corresponding pinyin_pitch for 你好 or type [q]uit:
ni3hao3
That is correct!
Enter the corresponding pinyin_pitch for 你 or type [q]uit:
```

You can convert between any columns of your choice.


# Contributing

All contributions are welcome!

If you encounter any issues or have suggestions, please [open an issue](https://github.com/deltachives/2025-004-langquiz-rs/issues/new).

To contribute changes, please follow the instructions in [CONTRIBUTING.md](./CONTRIBUTING.md).

You can reach out to me via mailto:lanhikarixx@gmail.com.

# License

This work is licensed under the [MIT license](https://opensource.org/licenses/mit-license.php) © 2025 Mohammed Alzakariya.
