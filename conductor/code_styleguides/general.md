# General Code Style Principles

This document outlines general coding principles that apply across all languages and frameworks used in this project, with specific standards for Nushell development.

## 1. Core Principles
- **Readability:** Code should be easy to read and understand by humans. Avoid overly clever or obscure constructs.
- **Consistency:** Follow existing patterns in the codebase. Maintain consistent formatting, naming, and structure.
- **Simplicity:** Prefer simple solutions over complex ones. Break down complex problems into smaller, manageable parts.
- **Data Over Text:** Always prefer structured data (Tables, Records, Lists) over raw string manipulation.
- **Fail Fast:** Use robust error handling to halt execution immediately upon encountering invalid states.

## 2. Naming Conventions
- **Commands & Public Functions:** Use \`kebab-case\` for all command names (e.g., \`plot\`, \`get-data\`).
- **Variables & Parameters:** Use \`snake_case\` for local variables and command parameters (e.g., \`user_id\`, \`file_path\`).
- **Environment Variables:** Use \`SCREAMING_SNAKE_CASE\` for variables that interact with the system environment.
- **Flags:** Use \`kebab-case\` for long flags and provide single-letter short flags in parentheses (e.g., \`--force(-f)\`).

## 3. Code Structure & Modularization
- **Public vs. Internal:** Use the \`export\` keyword only for functions intended for public use. Internal helpers should be defined without \`export\`.
- **Module Organization:** Group related functions into logical, domain-specific modules.
- **Order:** Place public exports at the top of the file, with internal helpers and constants below.

## 4. Pipelines & Data Handling
- **Native Filters:** Prefer native Nushell filters (e.g., \`where\`, \`each\`, \`select\`) over legacy Unix tools like \`grep\`, \`awk\`, or \`sed\`.
- **Composability:** Functions should return raw data (Tables, Records, Lists) rather than pre-formatted strings to ensure they can be easily piped.
- **Efficiency:** Be mindful of lazy streams and avoid collecting large datasets into memory prematurely.

## 5. Error Handling & Robustness
- **Defensive Programming:** Use \`try/catch\` blocks for operations that are prone to failure, such as network calls or file system modifications.
- **Contextual Errors:** Provide meaningful error messages that suggest potential fixes or clarify requirements.

## 6. Documentation
- **Docstrings:** Every exported function MUST have a docstring including a summary, details, and realistic examples (using the \`@example\` tag).
- **Explain Intent:** Document *why* a particular approach was taken, rather than just *what* the code is doing.

## 7. Formatting & Aesthetics
- **Indentation:** Use 2 spaces consistently. Do not use tabs.
- **Line Length:** Maintain a maximum line length of 80-100 characters.
- **Spacing:** Use spaces around pipes (\`|\`) and inside braces (\`{ |it| ... }\`).
