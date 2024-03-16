use package_managers::{CommandAndArguments, PackageManager};

pub fn get_add_command(package_manager: PackageManager, args: Vec<String>) -> CommandAndArguments {
    let program = match package_manager {
        PackageManager::Npm => "npm",
        PackageManager::Pnpm => "pnpm",
        PackageManager::Bun => "bun",
        PackageManager::Yarn => "yarn",
    }
    .to_string();

    CommandAndArguments {
        program,
        args: args,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn npm_doit_installer_deux_dev_dependencies() {
        let get_add_command = get_add_command(
            PackageManager::Pnpm,
            vec![
                String::from("-r"),
                String::from("exec"),
                String::from("vitest"),
            ],
        );
        assert_eq!(get_add_command.args, vec!["-r", "exec", "vitest"])
    }

    #[test]
    fn pnpm_doit_installer_trois_dependences() {
        let get_add_command = get_add_command(
            PackageManager::Npm,
            vec![
                String::from("run"),
                String::from("pomme"),
                String::from("poire"),
            ],
        );
        assert_eq!(get_add_command.args, vec!["run", "pomme", "poire"])
    }
}
