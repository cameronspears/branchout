# Branchout

Branchout is a Rust crate that generates ASCII tree representations of directory structures.

**Installation:**

You can install Branchout using Cargo from [crates.io](https://crates.io/crates/branchout). To install, run:

    cargo install branchout

**Usage:**

To use Branchout, run the `branchout` command with optional arguments:

    branchout [OPTIONS] [DIRECTORY]

**Options:**

- `--depth <DEPTH>`: Set the maximum depth to traverse the directory structure (default is 10).

**Example:**

Generate an ASCII tree structure for the current directory with a maximum depth of 3:

    branchout --depth 3

Enjoy using Branchout to visualize your directory structures!
