use package_managers::{CommandAndArguments, PackageManager};

pub fn get_install_command(package_manager: PackageManager) -> CommandAndArguments {
    match package_manager {
        PackageManager::Npm => CommandAndArguments {
            program: String::from("npm"),
            args: vec![String::from("install")],
        },
        PackageManager::Pnpm => CommandAndArguments {
            program: String::from("pnpm"),
            args: vec![String::from("install")],
        },
        PackageManager::Bun => CommandAndArguments {
            program: String::from("bun"),
            args: vec![String::from("i")],
        },
    }
}
