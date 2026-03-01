# Tech Stack: nu_plugin_plot

## 1. Programming Language
- **Rust (Edition 2021):** Used for its performance, safety, and modern toolchain.

## 2. Frameworks and Core Libraries
- **nu-plugin:** Provides the interface for creating Nushell plugins.
- **nu-protocol:** Handles communication protocols and data types with the main Nushell process.

## 3. Key Dependencies
- **owo-colors:** For robust and easy terminal coloring of plots.
- **term_size:** For automatic detection of the current terminal's width and height.
- **fnv:** Fast non-cryptographic hashing.
- **drawille (Local/Modified):** Handles Braille character generation for terminal graphics.
- **textplots (Local/Modified):** Provides core scaling and plotting logic.

## 4. Architecture
- **Single Binary Plugin:** A standalone Rust binary that Nushell executes as a child process. It communicates via standard protocols defined in the `nu-plugin` ecosystem.
- **Modular Plotting Logic:** Core plotting functionality is abstracted into the `color_plot` module for easier maintenance and expansion.
