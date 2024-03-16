use std::{path::PathBuf, process::Command};

#[derive(Debug, PartialEq)]
pub enum PackageManager {
    Npm,
    Pnpm,
    Bun,
}

pub struct CommandAndArguments {
    pub program: String,
    pub args: Vec<String>,
}

impl CommandAndArguments {
    pub fn run(&self) -> Result<std::process::Child, std::io::Error> {
        Command::new(&self.program).args(&self.args).spawn()
    }
}

pub fn get_package_manager_command(pm: PackageManager) -> &'static str {
    match pm {
        PackageManager::Npm => "npm",
        PackageManager::Pnpm => "pnpm",
        PackageManager::Bun => "bun",
    }
}

pub fn get_package_manager(current_dir: &PathBuf) -> PackageManager {
    let pnpm_lock = current_dir.join("pnpm-lock.yaml");
    if let Ok(true) = pnpm_lock.try_exists() {
        return PackageManager::Pnpm;
    }

    let bun_lockb = current_dir.join("bun.lockb");
    if let Ok(true) = bun_lockb.try_exists() {
        return PackageManager::Bun;
    }

    let package_lock = current_dir.join("package-lock.json");
    if let Ok(true) = package_lock.try_exists() {
        return PackageManager::Npm;
    }

    unimplemented!("Unknown package manager...")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lance_pnpm_en_présence_de_pnpm_lock() {
        let initial_dir = PathBuf::from("../test_projects/pnpm-project");
        let result = get_package_manager(&initial_dir);
        assert_eq!(result, PackageManager::Pnpm);
    }

    #[test]
    fn lance_npm_en_absence_de_pnpm_lock() {
        let initial_dir = PathBuf::from("../test_projects/npm-project");
        let result = get_package_manager(&initial_dir);
        assert_eq!(result, PackageManager::Npm);
    }
}
