# refrs - 📚 Reference Management System

Welcome to **refrs**, a minimalist yet powerful reference management system! 🎉 This tool allows you to manage your bibliographic references effortlessly by supporting BibTeX and RIS formats for importing and exporting references as BibTeX. 🚀

## 🌟 Features

- 📥 **Import** references from BibTeX and RIS files (or clipboard!)
- 📤 **Export** references to BibTeX format
- 🗂️ Manage references with an easy-to-use workspace system
- 🔄 Update and view references

## 🛠 Prerequisites

- Ensure **Git** is installed on your system, as the workspace system and shared data are entirely Git-based.
- Install [Rust](https://www.rust-lang.org/) to build and run the application.

## 🛠 Installation

Install **refrs** using Cargo:

```bash
cargo install --path .
```

That's it! You're ready to start managing your references. 🎉

## 🚀 Getting Started

### Clone a Git Repository

Before using **refrs**, you need to clone a Git repository to set up your workspace:

```bash
# init only once
refrs init

# clone a repo, this will be stored in the system
refrs clone <relative-path> <repository-url>
```

Replace `<relative-path>` with the desired local directory path and `<repository-url>` with the URL of your Git repository.

### Initialize the Workspace

After cloning the repository, set the workspace to the cloned Git repository:

```bash
refrs workspace set
```

### Import References

#### From Clipboard

To import references from your clipboard:

```bash
refrs import --clipboard
```

### Export References

Export your references to a BibTeX file:

```bash
refrs export <path-to-output-file>
```

Replace `<path-to-output-file>` with the desired file path.

### Manage Workspaces

#### Set a Workspace

```bash
refrs workspace set
```

#### Get the Current Workspace

```bash
refrs workspace get
```

### Clone References from a Repository

Clone references from a repository using a relative path and URL:

```bash
refrs clone <relative-path> <url>
```

### Sync current project with repo

Update the state of your reference management system:

```bash
refrs update
```

## 🔧 Development

The core structure of the project is organized as follows:

- **Commands**: Handlers for user actions (e.g., `command::init`, `command::files`).
- **State**: Manages the current state of the system.
- **Repo**: Handles cloning and reference repositories.


## 📜 License

This project is licensed under the [MIT License](./LICENSE).

