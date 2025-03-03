use std::{
    env,
    fs::{self, File},
    io::Read,
    path::Path,
};

use crate::shared::StringExtensions;

pub fn init() {
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

    replace_file_contents(
        "./ReadMe.md",
        vec![(
            "# Rust Boilerplate",
            &format!("# {}", project_name.snake_to_title_case()),
        )],
    );
}

fn replace_file_contents<P>(file_path: P, search_replaces: Vec<(&str, &str)>)
where
    P: AsRef<Path>,
{
    let mut file = File::open(&file_path).unwrap();

    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();

    for (search, replace) in search_replaces {
        content = content.replace(search, replace);
    }

    fs::write(file_path, content).unwrap();
}
