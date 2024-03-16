use package_managers::{get_package_manager_command, CommandAndArguments, PackageManager};

pub fn get_install_command(
    package_manager: PackageManager,
    original_args: &mut Vec<String>,
) -> CommandAndArguments {
    let mut args_command = vec![];

    if original_args.len() == 0 {
        push_install_command(&package_manager, &mut args_command);
    } else {
        push_add_command(&package_manager, &mut args_command, original_args);
    }
    args_command.append(original_args);

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
        PackageManager::Yarn => "install",
    };
    args_command.push(String::from(install_command));
}

fn push_add_command(
    package_manager: &PackageManager,
    args_command: &mut Vec<String>,
    original_args: &mut Vec<String>,
) {
    if package_manager.eq(&PackageManager::Yarn) {
        replace_global_yarn_install(args_command, original_args);
    }
    add_command(package_manager, args_command);
}

fn replace_global_yarn_install(args_command: &mut Vec<String>, original_args: &mut Vec<String>) {
    if let Some(n) = original_args.iter().position(|r| r.eq("-g")) {
        original_args.remove(n);
        args_command.push(String::from("global"));
    }
}

fn add_command(package_manager: &PackageManager, args_command: &mut Vec<String>) {
    let install_command = match *package_manager {
        PackageManager::Bun => "add",
        PackageManager::Npm => "install",
        PackageManager::Pnpm => "add",
        PackageManager::Yarn => "add",
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

    mod yarn {
        use package_managers::PackageManager;

        use crate::get_install_command;

        #[test]
        fn install_global_avec_arguments() {
            let install_command_yarn = get_install_command(
                PackageManager::Yarn,
                &mut vec![String::from("-g"), String::from("vite")],
            );
            let first_item = install_command_yarn.args.get(0);
            let second_item = install_command_yarn.args.get(1);
            assert_eq!(first_item, Some(&String::from("global")));
            assert_eq!(second_item, Some(&String::from("add")));
            assert_eq!(
                install_command_yarn.args,
                vec![
                    String::from("global"),
                    String::from("add"),
                    String::from("vite")
                ]
            )
        }

        #[test]
        fn install_avec_arguments() {
            let install_command_yarn =
                get_install_command(PackageManager::Yarn, &mut vec![String::from("vite")]);
            let first_item = install_command_yarn.args.get(0);
            assert_eq!(first_item, Some(&String::from("add")));
        }
    }
}
