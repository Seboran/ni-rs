use package_managers::{get_package_manager_command, CommandAndArguments, PackageManager};

pub fn get_install_command(
    package_manager: PackageManager,
    args: &mut Vec<String>,
) -> CommandAndArguments {
    let mut args_command = vec![];

    if args.len() == 0 {
        push_install_command(&package_manager, &mut args_command);
    } else {
        push_add_command(&package_manager, &mut args_command)
    }
    args_command.append(args);

    let program = String::from(get_package_manager_command(package_manager));

    CommandAndArguments {
        program,
        args: args_command,
    }
}

fn push_install_command(package_manager: &PackageManager, args_command: &mut Vec<String>) {
    let install_command = match *package_manager {
        PackageManager::Bun => "i",
        PackageManager::Npm => "install",
        PackageManager::Pnpm => "i",
    };
    args_command.push(String::from(install_command));
}

fn push_add_command(package_manager: &PackageManager, args_command: &mut Vec<String>) {
    let install_command = match *package_manager {
        PackageManager::Bun => "add",
        PackageManager::Npm => "install",
        PackageManager::Pnpm => "add",
    };
    args_command.push(String::from(install_command));
}

#[cfg(test)]
mod tests {

    mod pnpm {
        use package_managers::PackageManager;

        use crate::get_install_command;

        #[test]
        fn install_sans_arguments() {
            let install_command_pnpm = get_install_command(PackageManager::Pnpm, &mut vec![]);
            let first_item = install_command_pnpm.args.get(0);
            assert_eq!(first_item, Some(&String::from("i")));
        }

        #[test]
        fn install_avec_arguments() {
            let install_command_pnpm =
                get_install_command(PackageManager::Pnpm, &mut vec![String::from("vite")]);
            let first_item = install_command_pnpm.args.get(0);
            assert_eq!(first_item, Some(&String::from("add")));
        }
    }
}
