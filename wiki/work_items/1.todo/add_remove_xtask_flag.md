# Add Remove xtask Flag

After initializing a repository xtask and boilerplate references still exist.

## Solution

- Add a `--remove-xtask` feature flag to CLI arguments.
    - Removes `xtask` directory.
    - Moves the `app` directory contents to the project's root.
    - Removes `.cargo` directory.
- Update xtask initialization script to remove itself and all of the boilerplate work items.