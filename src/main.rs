use std::fs;
use std::path::Path;
use std::env;

/// Generate ASCII tree for a directory structure.
///
/// # Arguments
///
/// * `dir` - A string slice that holds the path of the directory.
/// * `prefix` - A string slice that holds the prefix for the ASCII tree structure.
///
/// # Example
///
/// ```
/// let dir = ".";
/// let prefix = "";
/// generate_tree(dir, prefix);
/// ```
fn generate_tree<P: AsRef<Path>>(dir: P, prefix: &str) {
    let entries = match fs::read_dir(&dir) {
        Ok(entries) => entries,
        Err(_) => return,
    };

    let mut entries: Vec<_> = entries
        .filter_map(Result::ok)
        .map(|e| e.path())
        .collect();

    entries.sort();

    let total_entries = entries.len();

    for (count, entry) in entries.iter().enumerate() {
        let is_last = count == total_entries - 1;

        // Create ASCII prefixes
        let (current_prefix, child_prefix) = if is_last {
            (format!("{}└── ", prefix), format!("{}    ", prefix))
        } else {
            (format!("{}├── ", prefix), format!("{}│   ", prefix))
        };

        // Print the current directory or file name
        let filename = entry.file_name().unwrap().to_str().unwrap();
        println!("{}{}", current_prefix, filename);

        // If it's a directory, traverse its children
        if entry.is_dir() {
            generate_tree(entry, &child_prefix);
        }
    }
}

fn main() {
    // Get the root directory from command line arguments
    let args: Vec<String> = env::args().collect();
    let root_dir = if args.len() > 1 {
        &args[1]
    } else {
        "."
    };

    println!("{}", root_dir);
    generate_tree(root_dir, "");
}
