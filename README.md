# Dayne

A high-performance, cross-platform desktop application inspired by Wallpaper Engine, built for browsing, downloading, and managing wallpapers seamlessly across Linux, Windows, and macOS.

---

## Tech Stack & Architecture

Dayne is built using modern web technologies paired with a native backend via **Tauri**.

<p>
  <img src="https://img.shields.io/badge/Svelte-ff3e00?style=for-the-badge&logo=svelte&logoColor=white" alt="Svelte">
  <img src="https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white" alt="Rust">
  <img src="https://img.shields.io/badge/Tauri-24C8DB?style=for-the-badge&logo=tauri&logoColor=white" alt="Tauri">
</p>

---

## Key Features

* **Cross-Platform Compatibility**: Native execution and optimized resource usage on Linux, Windows, and macOS.
* **Dynamic Content Management**: Browse, preview, and apply wallpapers effortlessly.
* **Integrated Download Manager**: Robust retrieval system for fetching wallpaper packages directly within the application workspace.
* **Low Resource Footprint**: Powered by a Rust backend to ensure minimal CPU and memory consumption during playback.

---

## Getting Started

### Prerequisites

Ensure the following dependencies are installed on your development machine prior to setup:
* [Rust](https://www.rust-lang.org/) (stable toolchain)
* [Node.js](https://nodejs.org/) (LTS version recommended)
* Operating system dependencies required by [Tauri](https://tauri.app/)

### Installation

1. Clone the repository:
  ```bash
   git clone [https://github.com/kraken-503/Dayne.git](https://github.com/kraken-503/Dayne.git)
   cd Dayne/
  ```
2. Install frontend and project dependencies:
  ```bash
   npm install
  ```
3. Initialize the development environment with hot-reloading:
  ```bash
    npm run tauri dev
  ```
   
### Building for Production

To generate an optimized, production-ready binary for your current platform:
  ```bash
npm run tauri build
  ```

>[!Important]
>Build outputs will be generated in the src-tauri/target/release directory.

### Contributing
Contributions, feature requests, and bug reports are welcome. Please review the [contribution guidelines](CONTRIBUTING.md) or open an issue to discuss proposed changes before submitting a pull request.
<br>

### License
Distributed under the GNU General Public License v3.0 (GPL-3.0). Refer to the [LICENSE](LICENSE) file for full terms and conditions.
