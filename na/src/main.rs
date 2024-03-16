use std::env;

use na::get_add_command;
use package_managers::get_package_manager;

fn main() -> Result<(), std::io::Error> {
    let current_dir = env::current_dir()?;
    let current_package_manager = get_package_manager(&current_dir);
    let mut args = env::args();
    args.next();
    let args = args.collect::<Vec<String>>();
    let add_command = get_add_command(current_package_manager, args);
    // exec the command
    add_command.run()?;
    Ok(())
}
