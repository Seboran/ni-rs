use package_managers::CommandAndArguments;

pub(crate) fn run_npm(args: &mut Vec<String>) -> CommandAndArguments {
    if let Some(value) = get_npm_shorthand_run(args) {
        return value;
    }

    let mut args_command = vec![String::from("run")];

    args_command.append(args);

    CommandAndArguments {
        program: String::from("npm"),
        args: args_command,
    }
}

pub(crate) fn get_npm_shorthand_run(args: &mut Vec<String>) -> Option<CommandAndArguments> {
    if let Some(command) = args.get(0) {
        if command.eq("start") || command.eq("test") {
            return Some(CommandAndArguments {
                program: String::from("npm"),
                args: args.to_vec(),
            });
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use package_managers::PackageManager;

    use crate::get_run_command;

    #[test]
    fn run_commande_start() {
        let run_command = get_run_command(PackageManager::Npm, &mut vec![String::from("start")]);
        assert_eq!(run_command.args, vec!["start"]);
        assert_eq!(run_command.program, "npm");
    }

    #[test]
    fn run_commande_test() {
        let run_command = get_run_command(PackageManager::Npm, &mut vec![String::from("test")]);
        assert_eq!(run_command.args, vec!["test"]);
        assert_eq!(run_command.program, "npm");
    }

    #[test]
    fn run_autre_command() {
        let run_command = get_run_command(PackageManager::Npm, &mut vec![String::from("script")]);
        assert_eq!(run_command.args, vec!["run", "script"]);
        assert_eq!(run_command.program, "npm");
    }
}
