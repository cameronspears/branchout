use std::fs;
use std::path::Path;
use std::env;
use std::io;

const DEFAULT_MAX_DEPTH: usize = 10;

/// Generate ASCII tree for a directory structure with protections.
///
/// # Arguments
///
/// * `dir` - A reference to a Path that holds the path of the directory.
/// * `prefix` - A string slice that holds the prefix for the ASCII tree structure.
/// * `depth` - The current depth of the traversal.
/// * `max_depth` - The maximum depth to traverse.
///
fn generate_tree<P: AsRef<Path>>(dir: P, prefix: &str, depth: usize, max_depth: usize) -> io::Result<String> {
    if depth > max_depth {
        return Ok(format!("{}[     *** Truncated. Use --depth to specify a larger depth]\n", prefix));
    }

    let entries = fs::read_dir(&dir)?;
    let mut entries: Vec<_> = entries
        .filter_map(Result::ok)
        .map(|e| e.path())
        .collect();

    entries.sort();

    let total_entries = entries.len();
    let mut local_output = String::new();

    for (count, entry) in entries.iter().enumerate() {
        let is_last = count == total_entries - 1;
        let (current_prefix, child_prefix) = if is_last {
            (format!("{}└── ", prefix), format!("{}    ", prefix))
        } else {
            (format!("{}├── ", prefix), format!("{}│   ", prefix))
        };

        let filename = entry.file_name().unwrap().to_str().unwrap();
        local_output.push_str(&format!("{}{}\n", current_prefix, filename));

        if entry.is_dir() {
            let subtree = generate_tree(entry, &child_prefix, depth + 1, max_depth)?;
            local_output.push_str(&subtree);
        }
    }

    Ok(local_output)
}

fn main() -> io::Result<()> {
    eprintln!("    *** Creating ASCII tree...");
    eprintln!();
    let mut max_depth = DEFAULT_MAX_DEPTH;
    let mut root_dir = ".";

    let args: Vec<String> = env::args().skip(1).collect();  // Skip the first argument
    let mut args_iter = args.iter().peekable();

    while let Some(arg) = args_iter.next() {
        match arg.as_str() {
            "--depth" => {
                if let Some(depth_arg) = args_iter.peek() {
                    max_depth = depth_arg.parse().unwrap_or(DEFAULT_MAX_DEPTH);
                    args_iter.next();  // Consume the depth argument
                }
            }
            _ => {
                if args_iter.peek().is_none() {
                    root_dir = arg;
                }
            }
        }
    }

    let tree_str = generate_tree(root_dir, "", 0, max_depth)?;
    print!("{}", tree_str);

    Ok(())
}
