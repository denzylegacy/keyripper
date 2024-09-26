<h1 align="center">keyripper</h1>

**keyripper** is a powerful tool developed in Rust to assist in the recovery of Bitcoin private keys by leveraging the Baby-Step Giant-Step (BSGS) algorithm to solve the discrete logarithm problem on the secp256k1 elliptic curve. This project underscores the importance of robust cryptographic practices and provides insights into the mathematical foundations that secure blockchain technologies.

---

**Mathematical Background Summary:**

- **Elliptic Curves**: Key structures in cryptography, defined by the equation y² = x³ + ax + b with conditions to avoid singularities.

- **secp256k1**: A specific elliptic curve used in Bitcoin, defined by y² = x³ + 7 over a 256-bit prime field.

- **Key Points**:
  - **Generator Point (G)**: Starting point for key generation.
  - **Order (n)**: Number of distinct points generated from G, a large prime.

- **Discrete Logarithm Problem (DLP)**: Challenge of finding the integer k such that Q = k * G. Security of systems like Bitcoin relies on the difficulty of solving DLP.

- **Baby-Step Giant-Step (BSGS) Algorithm**: Efficient method for solving DLP, reducing complexity from O(n) to O(sqrt(n)).

  - **Baby Steps**: Compute and store points G, 2G, 3G, ..., mG.
  - **Giant Steps**: Compute Q - j * mG and check against the baby steps table for matches.

- **Steps**:
  - **Initialization**: Choose m = ceiling(n) and create a hash table.
  - **Baby Steps Phase**: Store points in the hash table.
  - **Giant Steps Phase**: Check for matches and calculate k.

- **Advantages of BSGS**: Efficient and deterministic.

- **Limitations**: High memory usage and scalability issues for large n.

- **Optimization Strategies**: Use hash tables, parallelization, and subrange splitting to improve performance.

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

For any questions or support, please open an issue on the [GitHub repository](https://github.com/denzylegacy/keyripper/issues) or contact the maintainer at [denzylegacy@proton.me](mailto:denzylegacy@proton.me).

---

**Disclaimer:** This tool is intended for educational and authorized security testing purposes only. Unauthorized use of this tool to compromise private keys is illegal and unethical. The authors are not responsible for any misuse of this software.
