use package_managers::{
    get_package_manager_command, insert_global_at_first_position, CommandAndArguments,
    PackageManager,
};

pub fn get_remove_command(
    current_package_manager: PackageManager,
    original_args: &mut Vec<String>,
) -> CommandAndArguments {
    match current_package_manager {
        PackageManager::Yarn => get_yarn_rm_command(original_args),
        pm => get_default_rm_command(original_args, pm),
    }
}

fn get_yarn_rm_command(original_args: &mut Vec<String>) -> CommandAndArguments {
    let mut args = vec![String::from("remove")];
    insert_global_at_first_position(&mut args, original_args);
    args.append(original_args);
    CommandAndArguments {
        program: String::from("yarn"),
        args,
    }
}

fn get_default_rm_command(
    original_args: &mut Vec<String>,
    pm: PackageManager,
) -> CommandAndArguments {
    let mut args = vec![String::from("rm")];
    args.append(original_args);
    CommandAndArguments {
        program: String::from(get_package_manager_command(pm)),
        args,
    }
}

#[cfg(test)]
mod tests {
    use crate::get_remove_command;

    #[test]
    fn remove_npm_package() {
        let remove_command = get_remove_command(
            package_managers::PackageManager::Npm,
            &mut vec![String::from("vite")],
        );

        assert_eq!(remove_command.program, "npm");
        assert_eq!(remove_command.args, vec!["rm", "vite"])
    }

    #[test]
    fn remove_yarn_package() {
        let remove_command = get_remove_command(
            package_managers::PackageManager::Yarn,
            &mut vec![String::from("vite")],
        );

        assert_eq!(remove_command.program, "yarn");
        assert_eq!(remove_command.args, vec!["remove", "vite"])
    }

    #[test]
    fn remove_global_yarn_package() {
        let remove_command = get_remove_command(
            package_managers::PackageManager::Yarn,
            &mut vec![String::from("vite"), String::from("-g")],
        );

        assert_eq!(remove_command.program, "yarn");
        assert_eq!(remove_command.args, vec!["global", "remove", "vite"])
    }
}
