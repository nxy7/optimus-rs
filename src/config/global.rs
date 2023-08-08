use anyhow::Result;
use serde::{Deserialize, Serialize};

use super::command::Command;

#[derive(Serialize, Deserialize, Debug)]
pub struct Global {
    shell: Shell,
}

impl Global {
    pub fn default() -> Self {
        Global { shell: Shell::Bash }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum Shell {
    #[serde(rename = "bash")]
    Bash,
    #[serde(rename = "zsh")]
    Zsh,
    #[serde(rename = "nu")]
    Nushell,
}

impl Shell {
    pub fn execute_command_string(self, cmd_string: String) -> Result<String> {
        let mut shell_run = match self {
            Shell::Bash => std::process::Command::new("bash"),
            Shell::Zsh => std::process::Command::new("zsh"),
            Shell::Nushell => std::process::Command::new("nu"),
        };

        let output = shell_run.arg("-c").arg(cmd_string).output()?;
        let out = output.stdout;
        Ok(String::from_utf8(out)?)
    }
    pub fn execute_script_file(self, path: String) -> Result<String> {
        let mut shell_run = match self {
            Shell::Bash => std::process::Command::new("bash"),
            Shell::Zsh => std::process::Command::new("zsh"),
            Shell::Nushell => std::process::Command::new("nushell"),
        };

        let output = shell_run.arg(path).output()?;
        let out = output.stdout;
        Ok(String::from_utf8(out)?)
    }
}

#[cfg(test)]
mod tests {
    use super::Shell;
    #[test]
    fn shell_returns_failure_correctly() {}

    #[test]
    fn shell_returns_success_correctly() {}

    #[test]
    fn shell_bash_is_working() {
        let echo_string = "bash test";
        let res = Shell::Bash.execute_command_string(format!("echo {}", echo_string));
        assert!(res.is_ok());
        let res = res.unwrap();

        assert_eq!(res, format!("{}\n", echo_string));
    }

    #[test]
    fn shell_zsh_is_working() {
        let echo_string = "zsh test";
        let res = Shell::Zsh.execute_command_string(format!("echo {}", echo_string));
        assert!(res.is_ok());
        let res = res.unwrap();

        assert_eq!(res, format!("{}\n", echo_string));
    }

    #[test]
    fn shell_nushell_is_working() {
        let echo_string = "nushell test";
        let res = Shell::Nushell.execute_command_string(format!("print \'{}\'", echo_string));
        assert!(res.is_ok());
        let res = res.unwrap();

        assert_eq!(res, format!("{}\n", echo_string));
    }
}
