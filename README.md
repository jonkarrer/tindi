# Tindi

A zero dependency library of easy to use technical stock chart indicators.

## Installation

To install tindi for your project, you will need Rust installed on your machine. If you don't have Rust installed, you can follow the [official guide](https://www.rust-lang.org/tools/install).

Once Rust is installed, you can install tindi using cargo:

```bash
cargo install tindi
```

## Usage

Simple Moving Average

```rust
let data = vec![
  71.9, 72.51, 70.38, 71.63, 71.5, 71.11, 71.56, 70.34, 70.32, 70.05, 67.72, 66.45,
  67.12, 66.86, 66.7, 67.26, 67.52, 68.0, 67.43, 67.68, 68.86, 68.62, 67.27, 67.9, 67.74,
  66.45, 65.78, 66.88, 67.13, 66.65, 66.77, 65.86, 66.63, 65.55, 65.24, 64.74, 64.56,
  64.37, 63.06, 62.32, 63.67, 64.81, 65.23, 64.33, 64.73, 64.55, 63.94, 65.15, 66.18,
  67.65, 68.12, 67.9, 68.55, 67.13, 66.71, 66.34, 68.59, 68.24, 68.39, 69.34, 69.06,
];

let sma = simple_moving_average(&data);

dbg!(sma); // 67.29558
```

Bollinger Bands

```rust
let data = vec![
    35.56, 34.96, 33.72, 32.89, 34.36, 33.06, 31.05, 30.36, 30.89, 31.01, 32.19, 34.19,
    33.91, 35.87, 35.37, 36.11, 35.93, 34.53, 33.70, 33.95, 34.20, 35.38, 36.12, 35.35,
    36.25, 36.59, 36.49, 36.39, 35.66, 35.99, 32.93, 30.98, 30.99, 32.15, 31.99, 32.34,
];

let periods = 20;
let result = BollingerBands::new(&data, period).unwrap();

dbg!(&result);
/**
    BollingerBands {
        top_band: 38.211624,
        mid_band: 34.3955,
        bottom_band: 30.579376,
    };
*/
```

## Contribution

Feel free to submit an issue or PR.

## License

This project is licensed under the MIT and APACHE License.
