use package_managers::CommandAndArguments;

mod npm;

pub fn get_run_command(
    current_package_manager: package_managers::PackageManager,
    args: &mut Vec<String>,
) -> CommandAndArguments {
    match current_package_manager {
        package_managers::PackageManager::Npm => npm::run_npm(args),
        package_managers::PackageManager::Pnpm => run_script("pnpm", args),
        package_managers::PackageManager::Bun => run_script("bun", args),
    }
}

fn run_script(program: &'static str, args: &mut Vec<String>) -> CommandAndArguments {
    let mut args_command = vec![];

    args_command.append(args);

    CommandAndArguments {
        program: String::from(program),
        args: args_command,
    }
}

#[cfg(test)]
mod tests {

    mod pnpm {
        use package_managers::PackageManager;

        use crate::get_run_command;

        #[test]
        fn run_command() {
            let run_command =
                get_run_command(PackageManager::Pnpm, &mut vec![String::from("script")]);
            assert_eq!(run_command.args, vec!["script"]);
            assert_eq!(run_command.program, "pnpm");
        }
    }

    mod bun {
        use package_managers::PackageManager;

        use crate::get_run_command;

        #[test]
        fn run_test_bun() {
            let run_command = get_run_command(PackageManager::Bun, &mut vec![String::from("test")]);
            assert_eq!(run_command.program, "bun");
            assert_eq!(run_command.args, vec!["test"]);
        }

        #[test]
        fn run_script_bun() {
            let run_command =
                get_run_command(PackageManager::Bun, &mut vec![String::from("script:start")]);
            assert_eq!(run_command.program, "bun");
            assert_eq!(run_command.args, vec!["script:start"]);
        }
    }
}
