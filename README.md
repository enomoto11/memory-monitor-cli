# Memory Monitor CLI

A Rust CLI tool for monitoring system memory usage with visual display. This tool provides both system-wide memory information and application-specific memory usage statistics.

## Features

- 📊 **System Memory Overview**: Display total, used, free, and available memory
- 📈 **Visual Memory Usage Bar**: Color-coded progress bar showing memory utilization
- 🎯 **Memory Pressure Indicator**: Shows system memory pressure status
- 📱 **Application Memory Usage**: Breakdown of memory usage by application
- 🎨 **Colored Output**: Beautiful colored terminal output for better readability
- ⚡ **Fast Performance**: Written in Rust for optimal performance

## Installation

### Prerequisites

- Rust 1.70+ installed on your system
- Git for cloning the repository

### Install from Source

```bash
# Clone the repository
git clone https://github.com/enomoto11/memory-monitor-cli.git
cd memory-monitor-cli

# Build and install
cargo build --release
cargo install --path .
```

### Install from crates.io (Coming Soon)

```bash
cargo install memory-monitor-cli
```

## Usage

### Basic System Memory Display

```bash
memory-monitor-cli
```

This will show:
- Total system memory
- Memory breakdown (used, free, available)
- Visual usage bar
- Memory pressure indicator

### Application Memory Usage

```bash
memory-monitor-cli --apps
```

or

```bash
memory-monitor-cli -a
```

This will additionally show:
- Top 15 applications by memory usage
- Memory usage in MB and percentage for each app
- Visual bars for each application

### Customize Number of Top Applications

```bash
memory-monitor-cli --apps --top 20
```

or

```bash
memory-monitor-cli -a -t 20
```

Shows the top 20 applications by memory usage.

## Sample Output

### System Memory Overview

```
============================================================
                    メモリ使用状況
============================================================

総メモリ: 64.0 GB

メモリ内訳:
  使用中:      46.9 GB ( 73.3%)
  空き:         0.1 GB (  0.2%)
  利用可能:    17.1 GB ( 26.7%)

使用中: 46.9 GB (73.3%)
空き:   0.1 GB (0.2%)

メモリ使用率:
[████████████████████████████████████░░░░░░░░░░░░░░]
                                   ^ 73.3%

メモリ圧迫度: 中程度
============================================================
```

### Application Memory Usage

```
======================================================================
                  アプリケーション別メモリ使用状況
======================================================================

アプリケーション                                メモリ使用量        使用率 グラフ
----------------------------------------------------------------------
Virtual Machine                      2425 MB      3.7% ███████
Visual Studio Code                   1507 MB      2.3% ████
DataGrip                             1311 MB      2.0% ████
Google Chrome                        1245 MB      1.9% ███
Slack                                1049 MB      1.6% ███
Docker                                786 MB      1.2% ██
----------------------------------------------------------------------
合計使用量                               8323 MB     12.7%
======================================================================
```

## Command Line Options

- `-a, --apps`: Show memory usage by application
- `-t, --top <NUMBER>`: Number of top processes to show (default: 15)
- `-h, --help`: Show help message
- `-V, --version`: Show version information

## Platform Support

- ✅ **macOS**: Full support
- ✅ **Linux**: Full support
- ✅ **Windows**: Full support

## Dependencies

- `clap`: Command line argument parsing
- `sysinfo`: System information gathering
- `colored`: Terminal color output

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Author

**Eno** - [GitHub](https://github.com/enomoto11)

## Acknowledgments

- Built with ❤️ in Rust 🦀
- Inspired by system monitoring tools like `htop` and `top`
- Uses the excellent `sysinfo` crate for cross-platform system information