# brutecorr
A O(n^2) algorithm for brute-forcing Pearson Correlation Coefficients

![Yeah](https://raw.githubusercontent.com/ajmwagar/brutecorr/master/koolaid.gif)

## Features

- brute forcing vectors for Correlation Coefficients
- *get to feel like kool aid man*
- cross platform support
- nice CLI
- *get to feel like kool aid man*
- **FAST AF.**

## Usage

- `brutecorr --help`: show help message
- `brutecorr '1 2 3 4 5' -t 0.75`: finds secondary values that generate a correlation coefficent within  `0.1` of `.75`.
- `brutecorr '1 2 3 4 5' -t 0.75 -e 0.001`: finds secondary values that generate a correlation coefficent within  `0.001` of `.75`.

## Installation

### For developers:
1. `git clone https://github.com/ajmwagar/brutecorr`
2. `cd brutecorr`
3. `cargo install --path ./ --force`
4. Profit!

### Not for developers:

1. Head to the [releases page](https://github.com/ajmwagar/brutecorr/releases).
1. Download latest `brutecorr`
1. Profit!
