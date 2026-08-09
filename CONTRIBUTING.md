# Contributing Guidelines

Thank you for your interest in contributing to this project. We appreciate the time and effort put in by our community members to help improve this application.

Please review the following guidelines to ensure a smooth contribution process.

---

## Code of Conduct

By participating in this project, you agree to abide by its terms. Please report unacceptable behavior to the project maintainers.

---

## How Can I Contribute?

### Reporting Bugs
If you encounter a bug, please open an issue on GitHub and include the following details:
* A clear, descriptive title of the issue.
* Steps to reproduce the bug.
* Expected versus actual behavior.
* Operating system and hardware details (since this is a cross-platform desktop app).
* Relevant logs or error messages from the terminal or developer console.

### Suggesting Enhancements
If you have ideas for new features or improvements:
* Check existing issues to see if the suggestion has already been proposed.
* Open a new feature request issue detailing the use case, proposed implementation, and potential benefits.

### Pull Requests
1. Fork the repository and create your feature branch from `main`:
   ```bash
   git checkout -b feature/your-feature-name
   ```
    Make sure your changes build and run successfully using the development setup:
    ```bash
    npm run tauri dev
    ```
    
2. Test your changes across relevant components (frontend UI and Rust backend logic).

3. Commit your changes with clear, concise commit messages.

4. Push to your fork and submit a pull request against the main branch.

### Development Setup

To test your contributions locally, ensure you have the following prerequisites installed:

    Rust (stable toolchain)

    Node.js (LTS version recommended)

    System dependencies required by Tauri

### Quick Start

```bash
git clone [https://github.com/kraken-503/Dayne.git](https://github.com/kraken-503/Dayne.git)
cd Dayne/
npm install
npm run tauri dev
```

### Style Guidelines

  1. Code Formatting: Ensure your code adheres to standard conventions for Svelte, TypeScript, and Rust (cargo fmt for Rust files).

  2. Commit Messages: Write meaningful commit messages (e.g., fix: resolve wallpaper download timeout on linux, feat: add category filter to sidebar).

<br>

Thank you for contributing!
