use package_managers::{CommandAndArguments, PackageManager};

pub fn get_add_command(
    package_manager: PackageManager,
    args: &mut Vec<String>,
) -> CommandAndArguments {
    let program = match package_manager {
        PackageManager::Npm => "npm",
        PackageManager::Pnpm => "pnpm",
        PackageManager::Bun => "bun",
    }
    .to_string();

    let mut args_command: Vec<String> = vec![];
    match package_manager {
        PackageManager::Npm => args_command.push(String::from("i")),
        PackageManager::Pnpm => args_command.push(String::from("add")),
        PackageManager::Bun => args_command.push(String::from("add")),
    };

    args_command.append(args);

    CommandAndArguments {
        program,
        args: args_command,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn npm_doit_installer_deux_dev_dependencies() {
        let get_add_command = get_add_command(
            PackageManager::Npm,
            &mut vec![
                String::from("-D"),
                String::from("packet1"),
                String::from("packet2"),
            ],
        );
        assert_eq!(get_add_command.args, vec!["i", "-D", "packet1", "packet2"])
    }

    #[test]
    fn pnpm_doit_installer_trois_dependences() {
        let get_add_command = get_add_command(
            PackageManager::Pnpm,
            &mut vec![
                String::from("paquet3"),
                String::from("packet1"),
                String::from("packet2"),
            ],
        );
        assert_eq!(
            get_add_command.args,
            vec!["add", "paquet3", "packet1", "packet2"]
        )
    }
}
