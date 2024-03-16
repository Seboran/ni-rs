use package_managers::{get_package_manager_command, CommandAndArguments, PackageManager};

pub fn get_install_command(
    package_manager: PackageManager,
    args: &mut Vec<String>,
) -> CommandAndArguments {
    let mut args_command = vec![];
    let install_command = match package_manager {
        PackageManager::Bun => "i",
        PackageManager::Npm => "install",
        PackageManager::Pnpm => "i",
    };

    args_command.push(String::from(install_command));
    args_command.append(args);

    let program = String::from(get_package_manager_command(package_manager));

    CommandAndArguments {
        program,
        args: args_command,
    }
}
