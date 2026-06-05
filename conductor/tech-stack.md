# Tech Stack: nu_plugin_plot

## 1. Programming Language
- **Rust (Edition 2021):** Minimum version 1.96.0+ required for Nushell 0.113.1 compatibility.

## 2. Frameworks and Core Libraries
- **nu-plugin (v0.113.1):** Provides the interface for creating Nushell plugins.
- **nu-protocol (v0.113.1):** Handles communication protocols and data types with the main Nushell process.

## 3. Key Dependencies
- **owo-colors:** For robust and easy terminal coloring of plots.
- **term_size:** For automatic detection of the current terminal's width and height.
- **fnv:** Fast non-cryptographic hashing.
- **drawille (Local/Modified):** Handles Braille character generation for terminal graphics.
- **textplots (Local/Modified):** Provides core scaling and plotting logic.
- **nu-plugin-engine (v0.113.1, local-socket feature):** Required as a dev-dependency for test suite compilation under Nushell 0.113.1 due to Cargo feature unification of the `ServerCommunicationIo::LocalSocket` variant.

## 4. Architecture
- **Single Binary Plugin:** A standalone Rust binary that Nushell executes as a child process. It communicates via standard protocols defined in the `nu-plugin` ecosystem.
- **Modular Plotting Logic:** Core plotting functionality is abstracted into the `color_plot` module for easier maintenance and expansion.
