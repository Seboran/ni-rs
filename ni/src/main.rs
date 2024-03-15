use std::env;

use ni::{get_install_command, get_package_manager};

fn main() -> Result<(), std::io::Error> {
    let current_dir = env::current_dir()?;
    let current_package_manager = get_package_manager(&current_dir);
    let install_command = get_install_command(current_package_manager);
    // exec the command
    install_command.run()?;
    Ok(())
}
