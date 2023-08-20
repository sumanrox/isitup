# isitup - Fast Domain Checker
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)


isitup is a Rust-based command-line tool for quickly checking if a list of domains or subdomains is up or not. What sets it apart is its multi-threaded design, which allows you to check a large number of domains concurrently, resulting in faster results.

![isitup_in_action](./images/product-video.gif)


## Features

- Multi-threaded: Utilizes the power of Rust's concurrency for speedy domain checking.
- Simple to use: A straightforward command-line interface for quick domain checks.
- Flexible input: Supports both domains and subdomains in a text file input.
- Customizable: You can adjust the number of threads to optimize performance.

## Installation

To build and install isitup, you'll need Rust and Cargo. If you don't have Rust installed, you can get it from [Rust's official website](https://www.rust-lang.org/).

Once you have Rust and Cargo installed, you can build and install isitup using the following commands:

## Get the necessary files
```bash
git clone https://github.com/sumanrox/isitup.git
cd isitup
```

## Install
If you want to install, you can download the your prefered binary from the release, or use the following commands
```
chmod +x install.sh
./install.sh
```

## Build
Feel free to modify the source and build your own flavour of isitup
```
chmod +x build.sh
./build.sh
```

## Usage
```
isitup domainlist.txt N
```
Where N is the number of concurrent threads, for example
```
isitup domain_list.txt 1000
```

## Uninstall
If you want to remove the program from the system, use the following command
```
sudo rm /usr/local/bin/isitup
```
