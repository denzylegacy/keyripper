<h1 align="center">keyripper</h1>

**Private Key Finder:** A Rust project designed to search for private keys based on the Bitcoin secp256k1 elliptic curve. This tool utilizes advanced algorithms to explore potential private keys, emphasizing the complexities and security considerations involved in cryptographic key management.

---

## Table of Contents

1. [Introduction](#introduction)
2. [Mathematical Background](#mathematical-background)
3. [Features](#features)
4. [Installation](#installation)
    - [Prerequisites](#prerequisites)
    - [Installing Rust](#installing-rust)
    - [Cloning the Repository](#cloning-the-repository)
    - [Setting Up Configuration](#setting-up-configuration)
    - [Building the Project](#building-the-project)
5. [Usage](#usage)
    - [Configuration](#configuration)
    - [Running the Project](#running-the-project)
6. [Running in Google Colab](#running-in-google-colab)
7. [Target Address Structure](#target-address-structure)
8. [Puzzle Context](#puzzle-context)
9. [Troubleshooting](#troubleshooting)
10. [Contributing](#contributing)
11. [License](#license)
12. [Acknowledgements](#acknowledgements)
13. [Contact](#contact)

---

## Introduction

**keyripper** is a powerful tool developed in Rust to assist in the recovery of Bitcoin private keys by leveraging the Baby-Step Giant-Step (BSGS) algorithm to solve the discrete logarithm problem on the secp256k1 elliptic curve. This project underscores the importance of robust cryptographic practices and provides insights into the mathematical foundations that secure blockchain technologies.

---

## Mathematical Background

### Elliptic Curves and secp256k1

Elliptic curves are fundamental to modern cryptography, especially in the context of cryptocurrencies like Bitcoin. The secp256k1 curve is defined by the equation:

\[ y^2 = x^3 + 7 \]

over the finite field \( \mathbb{F}_p \), where:

\[ p = \text{FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEFFFFFC2F}_{16} \]

### Discrete Logarithm Problem (DLP)

The security of elliptic curve cryptography (ECC) relies on the difficulty of the Discrete Logarithm Problem:

Given points \( G \) and \( Q \) on the curve, find the scalar \( k \) such that:

\[ Q = k \cdot G \]

where \( \cdot \) denotes scalar multiplication on the elliptic curve.

### Baby-Step Giant-Step (BSGS) Algorithm

The BSGS algorithm is an efficient method to solve the DLP by reducing its complexity from \( O(n) \) to \( O(\sqrt{n}) \). It involves:

1. **Baby Steps:** Precompute and store \( G, 2G, 3G, \ldots, mG \) in a hash table, where \( m = \lceil \sqrt{n} \rceil \).
2. **Giant Steps:** Compute \( Q - jm \cdot G \) for \( j = 0, 1, 2, \ldots, m \) and check for matches in the hash table.

If a match is found, the scalar \( k \) can be determined as:

\[ k = jm + i \]

where \( i \) is the index from the baby steps.

---

## Features

- **Efficient DLP Solver:** Implements the Baby-Step Giant-Step algorithm optimized for the secp256k1 curve.
- **Multithreading Support:** Leverages multiple CPU cores to accelerate the search process.
- **User-Friendly Configuration:** Allows customization of search ranges and subrange sizes.
- **Integration with Google Colab:** Facilitates running the tool in cloud environments.
- **Secure Key Handling:** Ensures safe management of sensitive cryptographic materials.

---

## Installation

### Prerequisites

- **Rust Programming Language:** Ensure Rust is installed on your system. If not, follow the [Installing Rust](#installing-rust) section.
- **Git:** Required for cloning the repository.
- **Internet Connection:** Necessary for downloading dependencies.

### Installing Rust

Rust is the primary language used for developing `keyripper`. Follow these steps to install Rust on your system:

#### On Windows

1. **Download Rust Installer:**
   
   Visit [rustup.rs](https://rustup.rs/) and click on the Windows button to download the installer.

2. **Run the Installer:**
   
   Execute the downloaded `.exe` file and follow the on-screen instructions.

3. **Configure Your Current Shell:**
   
   After installation, open a new Command Prompt or PowerShell window to ensure that the Rust binaries are in your `PATH`.

4. **Verify the Installation:**
   
   ```bash
   rustc --version
   ```
   
   You should see output similar to:
   
   ```
   rustc 1.XX.X (commit hash)
   ```

#### On Linux

1. **Install Rust via `rustup`:**
   
   Open your terminal and execute:
   
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```
   
2. **Follow the On-Screen Instructions:**
   
   The script will guide you through the installation process. By default, it installs the latest stable version of Rust.

3. **Configure Your Current Shell:**
   
   After installation, configure your shell environment by running:
   
   ```bash
   source $HOME/.cargo/env
   ```

4. **Verify the Installation:**
   
   ```bash
   rustc --version
   ```
   
   You should see output similar to:
   
   ```
   rustc 1.XX.X (commit hash)
   ```

### Cloning the Repository

Clone the `keyripper` repository from GitHub:

```bash
git clone https://github.com/yourusername/keyripper.git
```

Navigate to the project directory:

```bash
cd keyripper
```

### Setting Up Configuration

`keyripper` uses a `.env` file to configure essential variables. Create a `.env` file in the root directory of the project with the following content:

```env
NUM_THREADS=4
SUBRANGE_SIZE=1000000000
SERVER_URL=https://yourserver.com/api
API_AUTH_TOKEN=your_auth_token
```

- `NUM_THREADS`: Number of threads to utilize for the search process.
- `SUBRANGE_SIZE`: Size of each subrange for the search.
- `SERVER_URL`: URL of the server to send the found key.
- `API_AUTH_TOKEN`: Authentication token for the server.

### Building and Running the Project

Build the project using Cargo, Rust's package manager and build system:

#### On Windows and Linux

```bash
cargo build --release
cargo run
```

This command compiles the project in release mode, optimizing for performance. The compiled binary will be located in the `target/release/` directory.

---

### Configuration

Before running `keyripper`, ensure that the `.env` file is properly configured as described in the [Setting Up Configuration](#setting-up-configuration) section.


**Example Address JSON (`address.json`):**

```json
{
    "Address": 130,
    "BitRange": "2^129...2^130-1",
    "PrivateKeyRange": "200000000000000000000000000000000...3ffffffffffffffffffffffffffffffff",
    "PrivateKeyRangeStart": "200000000000000000000000000000000",
    "PrivateKeyRangeEnd": "3ffffffffffffffffffffffffffffffff",
    "PrivateKey(HEX)": "Unknown",
    "PublicKey(HEX)": "03633cbe3ec02b9401c5effa144c5b4d22f87940259634858fc7e59b1c09937852",
    "BitcoinAddress": "1Fo65aKq8s8iquMt6weF1rku1moWVEd5Ua",
    "PercentOfRange": 0.0,
    "ResolutionDate": "Unknown",
    "Solver": "Unknown",
    "Solved": false
}
```

**Notes:**

- **PrivateKeyRangeStart & PrivateKeyRangeEnd:** Define the range of private keys to search within. Ensure these values are in hexadecimal format.
- **PublicKey(HEX):** The public key corresponding to the target Bitcoin address.

**Example Output:**

```
Start scalar: 780778204189001025463454792048743977763, Maximum steps: 10001
[+] ThreadId(15) is processing the range: 780778204189001025463454792048743977763 - 780778204189001025463454792048743987763
[-] ThreadId(18) is processing the range: 680564733841876926926749214863536422911 - 680564733841876926926749214863536422911
...
Private Key Found! <1a2b3c4d5e6f...>
Data successfully sent to the server.
Elapsed time: 2m 35s
Total steps attempted: 20002
```

---

## Running in Google Colab

Google Colab provides a cloud-based environment to run `keyripper` without local setup. Follow these steps to execute `keyripper` in Google Colab:

1. **Open the Code in Google Colab**

   Click the badge below to open the project in Google Colab:

   <div style="text-align: center; margin: 20px 0;">
     <a href="https://drive.google.com/file/d/1BS4hhVENR4LlXEfgZryse66U2g5fOJrV/view?usp=sharing" target="_blank">
       <img src="https://colab.research.google.com/assets/colab-badge.svg" alt="Open in Colab"/>
     </a>
   </div>

2. **Run the Kernel Installation Cell**

   Execute the first cell to install the Rust compiler and necessary dependencies:

   ```bash
   !curl https://sh.rustup.rs -sSf | sh -s -- -y
   !source $HOME/.cargo/env
   ```

3. **Wait for the Runtime to Restart**

   After the installation, the runtime will automatically restart. Wait until the process completes.

4. **Clone the Repository**

   Once the runtime has restarted, run the following cell to clone the `keyripper` repository:

   ```bash
   !git clone https://github.com/yourusername/keyripper.git
   %cd keyripper
   ```

5. **Prepare the Address JSON**

   Create an `address.json` file with the target address details:

   ```bash
   %%writefile address.json
   {
       "Address": 130,
       "BitRange": "2^129...2^130-1",
       "PrivateKeyRange": "200000000000000000000000000000000...3ffffffffffffffffffffffffffffffff",
       "PrivateKeyRangeStart": "200000000000000000000000000000000",
       "PrivateKeyRangeEnd": "3ffffffffffffffffffffffffffffffff",
       "PrivateKey(HEX)": "Unknown",
       "PublicKey(HEX)": "03633cbe3ec02b9401c5effa144c5b4d22f87940259634858fc7e59b1c09937852",
       "BitcoinAddress": "1Fo65aKq8s8iquMt6weF1rku1moWVEd5Ua",
       "PercentOfRange": 0.0,
       "ResolutionDate": "Unknown",
       "Solver": "Unknown",
       "Solved": false
   }
   ```

---

## Target Address Structure

`keyripper` requires target addresses to be defined in a specific JSON format. Below is an example of the expected structure:

```json
{
    "Address": 130,
    "BitRange": "2^129...2^130-1",
    "PrivateKeyRange": "200000000000000000000000000000000...3ffffffffffffffffffffffffffffffff",
    "PrivateKeyRangeStart": "200000000000000000000000000000000",
    "PrivateKeyRangeEnd": "3ffffffffffffffffffffffffffffffff",
    "PrivateKey(HEX)": "Unknown",
    "PublicKey(HEX)": "03633cbe3ec02b9401c5effa144c5b4d22f87940259634858fc7e59b1c09937852",
    "BitcoinAddress": "1Fo65aKq8s8iquMt6weF1rku1moWVEd5Ua",
    "PercentOfRange": 0.0,
    "ResolutionDate": "Unknown",
    "Solver": "Unknown",
    "Solved": false
}
```

**Field Descriptions:**

- **Address:** Identifier for the puzzle (e.g., 130).
- **BitRange:** The range of bits for the private key search.
- **PrivateKeyRange:** The hexadecimal range of private keys to search within.
- **PrivateKeyRangeStart:** Starting point of the private key search range (hexadecimal).
- **PrivateKeyRangeEnd:** Ending point of the private key search range (hexadecimal).
- **PrivateKey(HEX):** The discovered private key in hexadecimal format (initially "Unknown").
- **PublicKey(HEX):** The public key corresponding to the target Bitcoin address.
- **BitcoinAddress:** The target Bitcoin address.
- **PercentOfRange:** Percentage of the range that has been searched.
- **ResolutionDate:** Date when the puzzle was solved (initially "Unknown").
- **Solver:** Entity that solved the puzzle (initially "Unknown").
- **Solved:** Boolean indicating whether the puzzle has been solved.

---

## Puzzle Context

**keyripper** is designed to assist in solving cryptographic puzzles such as those found on [PrivateKeys.pw](https://privatekeys.pw/puzzles/bitcoin-puzzle-tx). These puzzles involve finding specific private keys within defined ranges that control significant amounts of Bitcoin.

### ~1000 BTC Bitcoin Challenge Transaction

- **Status:** PARTIALLY SOLVED
- **Prize:** 988.498 BTC (total), 51.598 BTC (won), 936.9 BTC (remaining)
- **Creator:** Unknown
- **Start Date:** 2015-01-15
- **Address:** 1BY8GQbnueYofwSuFAT3USAhGjPrkxDdW9 

### Description

In 2015, to demonstrate the vastness of the private key space (or perhaps for entertainment), someone created a "puzzle" where private keys within a specific, smaller space were chosen, and increasing amounts of Bitcoin were sent to each of those keys as follows:


**History:**

- **2015-01-15:** A transaction was created containing transfers to 256 different Bitcoin addresses.
- **2017-07-11:** Funds from addresses #161–256 were moved to corresponding lower-range addresses, increasing their balances.
- **2019-05-31:** Outgoing transactions with 1000 satoshi were created for specific addresses to compare the difficulty of finding private keys.
- **2023-04-16:** Puzzle #66 was solved, but the prize was split between two addresses.

### Solution

To solve these puzzles, one must iterate over the specific private key space and check each private key for a balance. The narrower the key space, the higher the chance of success. Utilizing algorithms like Baby-Step Giant-Step or Pollard's kangaroo can optimize this search process.

---

## Contributing

Contributions are welcome! Follow these steps to contribute to `keyripper`:

1. **Fork the Repository**

   Click the "Fork" button at the top-right corner of the repository page.

2. **Clone Your Fork**

   ```bash
   git clone https://github.com/yourusername/keyripper.git
   cd keyripper
   ```

3. **Create a New Branch**

   ```bash
   git checkout -b feature/your-feature-name
   ```

4. **Make Your Changes**

   Implement your feature or bug fix.

5. **Commit Your Changes**

   ```bash
   git commit -m "Add feature: your feature description"
   ```

6. **Push to Your Fork**

   ```bash
   git push origin feature/your-feature-name
   ```

7. **Create a Pull Request**

   Navigate to your fork on GitHub and click "New pull request".

---

## License

This project is licensed under the [Apache License 2.0](https://github.com/denzylegacy/keyripper/blob/main/LICENSE).

---

## Contact

For any questions or support, please open an issue on the [GitHub repository](https://github.com/denzylegacy/keyripper/issues) or contact the maintainer at [youremail@example.com](mailto:denzylegacy@proton.me).

---

**Disclaimer:** This tool is intended for educational and authorized security testing purposes only. Unauthorized use of this tool to compromise private keys is illegal and unethical. The authors are not responsible for any misuse of this software.
