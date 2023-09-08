# clip key sanitizer
> keyboard and clipboard input sanitizer using predictive search.

[![GitHub Workflow Status](https://img.shields.io/github/workflow/status/mosure/clip-key-sanitizer/Rust)](https://github.com/mosure/clip-key-sanitizer/actions)
[![GitHub release](https://img.shields.io/github/release/mosure/clip-key-sanitizer.svg)](https://GitHub.com/mosure/clip-key-sanitizer/releases/)
[![GitHub license](https://img.shields.io/github/license/mosure/clip-key-sanitizer.svg)](https://github.com/mosure/clip-key-sanitizer/blob/main/LICENSE)

![Clip Key Sanitizer Banner](./docs/banner.png)

Clip Key Sanitizer is a utility tool that monitors keyboard input and clipboard text to identify and sanitize sequences of characters from a predefined deny list.


## 🛠️ Development

Clone the repository:

```bash
git clone https://github.com/mosure/clip-key-sanitizer.git
cd clip-key-sanitizer
```

Build and run:

```bash
cargo run
```

## 📜 Deny List

To modify the deny list, update the `deny.txt` file located in the `src` directory.


## 📜 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
