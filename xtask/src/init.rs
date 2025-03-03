use std::{
    env,
    fmt::Display,
    fs::{self, File},
    io::{ErrorKind, Read},
    path::Path,
};

use crate::shared::StringExtensions;

/// Initializes a new repository.
pub fn init() {
    let args = env::args().skip(1).collect::<Vec<_>>();

    replace_files_project_name();

    if args.contains(&"--remove-xtask".into()) {
        fs::rename("./app/src", "./src").unwrap();
        fs::rename("./app/Cargo.toml", "./Cargo.toml").unwrap();

        remove_directory("./.cargo");
        remove_directory("./app");
        remove_directory("./xtask");
    }
}

/// Deletes the directory.
fn remove_directory<P>(dir_path: P)
where
    P: AsRef<Path> + Display,
{
    fs::remove_dir_all(&dir_path)
        .or_else(|error| {
            if error.kind() == ErrorKind::NotFound {
                Ok(())
            } else {
                Err(error)
            }
        })
        .expect(&format!("Failed to delete '{}' directory.", &dir_path));
}

/// Replaces the project name in the repository's files and renames the code-workspace file.
fn replace_files_project_name() {
    let project_name = env::current_dir()
        .expect("Failed to get current directory.")
        .file_name()
        .expect("Failed to get parent directory name.")
        .to_string_lossy()
        .into_owned()
        .camel_to_snake_case();

    replace_file_contents(
        "./app/Cargo.toml",
        vec![
            (
                "name = \"rust_boilerplate\"",
                &format!("name = \"{}\"", project_name),
            ),
            (
                "default-run = \"rust_boilerplate\"",
                &format!("default-run = \"{}\"", project_name),
            ),
        ],
    );

    replace_file_contents(
        "./wiki/development.md",
        vec![(
            ".vscode/rust_boilerplate.code-workspace",
            &format!(".vscode/{}.code-workspace", project_name),
        )],
    );

    fs::rename(
        ".vscode/rust_boilerplate.code-workspace",
        &format!(".vscode/{}.code-workspace", project_name),
    )
    .expect("Failed to rename code-workspace file.");

    replace_file_contents(
        "./ReadMe.md",
        vec![(
            "# Rust Boilerplate",
            &format!("# {}", project_name.snake_to_title_case()),
        )],
    );
}

/// Replaces the file contents using the find and replace tuples.
fn replace_file_contents<P>(file_path: P, find_replaces: Vec<(&str, &str)>)
where
    P: AsRef<Path> + Display,
{
    let mut file = File::open(&file_path).expect(&format!("Failed to open '{}'.", &file_path));

    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect(&format!("Failed to read '{}'.", &file_path));

    for (find, replace) in find_replaces {
        contents = contents.replace(find, replace);
    }

    fs::write(&file_path, contents).expect(&format!("Failed to write '{}'.", &file_path));
}
