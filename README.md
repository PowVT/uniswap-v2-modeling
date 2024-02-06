# Uniswap V2 modeling calculator

This is a simple calculator to model the affects of adding liquidity, removing liquidity, and swapping tokens on a Uniswap V2 pool. This program tracks a pools reserves and calculates the price of a token in the pool after each swap.

## Features

- Add liquidity
- Remove liquidity
- Swap tokens
- Track reserves
- Track token price

## Getting Started

### Prerequisites

- Rust (https://www.rust-lang.org/tools/install)

### Building and Running

1. Clone the repository:

   ```bash
   git clone https://github.com/PowVT/uniswap-v2-modeling.git
   ```

2. Navigate to the project directory:

   ```bash
   cd uniswap-v2-modeling
   ```

3. Build and run the project:

   ```bash
   cargo run
   ```

## Usage

The provided code contains a simple example in the `main` function, demonstrating how to create a Uniswap V2 pool, provide liquidity, swap tokens, remove liquidity and retrieve reserve information.

Feel free to modify and extend the code to suit your specific use case and requirements.

## Contributing

Contributions are welcome! If you have improvements or additional features to add, please open an issue or submit a pull request.

## License

This project is licensed under the MIT License.
