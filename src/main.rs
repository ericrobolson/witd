use std::{
    collections::{HashMap, HashSet},
    io::Write,
};

use walkdir::WalkDir;

#[derive(Debug)]
enum Dsl {
    Help,
    Watch(String),
    Ignore(String),
    Command(Vec<String>),
}

fn main() {
    // Get args and create a DSL from them
    let mut args: Vec<String> = std::env::args().collect();
    args.remove(0);

    let dsl = parse_args(args);

    // Parse the DSL into watches, ignores, and command
    let mut watches = HashSet::new();
    let mut ignores = HashSet::new();
    let mut command = None;
    for dsl in dsl {
        match dsl {
            Dsl::Help => {
                // print help
                println!("Usage: [options] <command>");
                println!("Options:");
                println!("  -h, --help       Show this help message");
                println!("  -w, --watch      Watch a directory");
                println!("  -i, --ignore     Ignore a directory");
                std::process::exit(0);
            }
            Dsl::Watch(w) => {
                watches.insert(w);
            }
            Dsl::Ignore(i) => {
                ignores.insert(i);
            }
            Dsl::Command(c) => {
                if !c.is_empty() {
                    command = Some(c);
                }
            }
        }
    }

    // Default watches
    if watches.is_empty() {
        watches.insert(".".to_string());
    }

    // Default ignores
    ignores.insert("node_modules".to_string());
    ignores.insert("target".to_string());
    ignores.insert(".git".to_string());

    // Get the command
    let command = match command {
        Some(c) => {
            if c.is_empty() {
                println!("Error: No command provided");
                std::process::exit(1);
            }
            c
        }
        None => {
            println!("Error: No command provided");
            std::process::exit(1);
        }
    };

    // Track the last update time of each file
    let mut last_updates = HashMap::new();

    // Main loop
    loop {
        // check if any of the watches have changed
        // if so, run the command
        // if not, sleep for a short duration and check again

        let mut changed = false;
        let mut changed_paths = Vec::new();
        for watch in watches.iter() {
            let walker = WalkDir::new(watch);
            for entry in walker {
                let entry = entry.unwrap();
                let path_str = entry.path().to_str().unwrap().to_string();

                // Skip if path matches any ignore pattern
                if ignores.iter().any(|ignore| path_str.contains(ignore)) {
                    continue;
                }

                // Get the metadata of the file
                let metadata = entry.metadata().unwrap();
                let last_update = metadata.modified().unwrap();

                // If the file has never been tracked, mark it as changed
                if !last_updates.contains_key(&path_str) {
                    changed = true;
                    changed_paths.push(path_str.clone());
                    last_updates.insert(path_str.clone(), last_update);
                } else {
                    // If the file has been tracked, check if it has changed
                    let last_update_time = last_updates.get(&path_str).unwrap();
                    if last_update > *last_update_time {
                        changed = true;
                        changed_paths.push(path_str.clone());
                        last_updates.insert(path_str.clone(), last_update);
                    }
                }
            }
        }

        // If any of the watched files have changed, run the command
        if changed {
            // Run the command
            let program = command[0].clone();
            let args = command[1..].to_vec();

            let output = std::process::Command::new(&program)
                .args(&args)
                .output()
                .unwrap();

            if output.status.success() {
                let stdout = std::io::stdout();
                let mut stdout = stdout.lock();
                stdout.write_all(&output.stdout).unwrap();
            } else {
                let stdout = std::io::stdout();
                let stderr = std::io::stderr();
                let mut stdout = stdout.lock();
                let mut stderr = stderr.lock();
                stdout.write_all(&output.stdout).unwrap();
                stderr.write_all(&output.stderr).unwrap();
            }
        }

        // Sleep for a short duration and check again
        std::thread::sleep(std::time::Duration::from_millis(100));
    }
}

fn parse_args(args: Vec<String>) -> Vec<Dsl> {
    let mut dsl = Vec::new();
    let mut command = Vec::new();
    let mut finished_opts = false;
    let split_equals = |s: &str| {
        let parts: Vec<&str> = s.split('=').collect();

        if parts.is_empty() {
            println!("Error: Missing argument for {}", s);
            std::process::exit(1);
        }
        if parts.len() == 1 {
            println!("Error: Missing argument for {}", s);
            std::process::exit(1);
        }
        parts[1].to_string()
    };
    for arg in args {
        if arg == "-h" || arg == "--help" {
            dsl.push(Dsl::Help);
            continue;
        }
        if arg.starts_with("-w=") || arg.starts_with("--watch=") {
            dsl.push(Dsl::Watch(split_equals(arg.as_str())));
            continue;
        }
        if arg.starts_with("-i=") || arg.starts_with("--ignore=") {
            dsl.push(Dsl::Ignore(split_equals(arg.as_str())));
            continue;
        }
        if !arg.starts_with("-") && !arg.starts_with("--") {
            finished_opts = true;
        }
        if finished_opts {
            command.push(arg);
        }
    }

    dsl.push(Dsl::Command(command));
    dsl
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_args() {
        assert_eq!(true, false);
    }
}
