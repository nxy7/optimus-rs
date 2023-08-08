use std::str::FromStr;

use super::global::Shell;
use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
enum CommandOrString {
    Command {
        description: Option<String>,
        root: Option<String>,
        run: Option<String>,
        file: Option<String>,
        shell: Option<Shell>,
    },
    String(String),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(from = "CommandOrString")]
pub struct Command {
    /// Description shown in CLI
    pub description: Option<String>,
    /// Directory that the command should be ran from. Can be relative or absolute path.
    pub root: Option<String>,
    /// Shell commands to be ran, cannot be Some if 'file' is Some
    pub run: Option<String>,
    /// Script file to be ran using selected shell. Cannot be set if 'run' is Some
    pub file: Option<String>,
    /// Shell that overrides global settings
    pub shell: Option<Shell>,
}

impl Command {
    /// Runs command using chosen shell (Bash by default) and returns output as String
    fn run_command(&self) -> Result<String> {
        let shell = self.shell.clone();
        let shell = shell.unwrap_or(Shell::Bash);

        if let Some(run) = self.run.clone() {
            return shell.execute_command_string(run);
        };
        if let Some(f) = self.file.clone() {
            return shell.execute_script_file(f);
        };
        Err(anyhow::anyhow!("Run and file were not provided"))
    }

    pub fn to_clap_command(self, name: String) -> clap::Command {
        let mut clap_cmd = clap::Command::new(name.clone());
        if let Some(desc) = self.description {
            let desc = desc.trim().to_string();
            clap_cmd = clap_cmd.about(desc);
        } else {
            clap_cmd = clap_cmd.about(format!("Run {} command", name));
        }

        clap_cmd
    }
}

impl From<CommandOrString> for Command {
    fn from(s: CommandOrString) -> Command {
        match s {
            CommandOrString::Command {
                description,
                root,
                run,
                file,
                shell,
            } => Command {
                description,
                root,
                run,
                file,
                shell,
            },
            CommandOrString::String(s) => Command {
                description: None,
                root: None,
                run: Some(s),
                file: None,
                shell: None,
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::config::global::Shell;

    use super::Command;

    #[test]
    fn command_can_be_ran() {
        let run_cmd = "this is test command";
        let c = Command {
            description: Some("command description".to_string()),
            run: format!("echo {}", run_cmd).into(),
            shell: Shell::Bash.into(),
            root: None,
            file: None,
        };

        let out = c.run_command();
        assert!(out.is_ok());
        let out = out.unwrap();
        assert_eq!(out, format!("{}\n", run_cmd));
    }

    #[test]
    fn command_can_be_deserialized_from_string() {
        let command_string = r#"
  echo "sup"
        "#;
        let cmd = serde_yaml::from_str::<Command>(command_string);
        assert!(cmd.is_ok());
    }

    #[test]
    fn command_can_be_deserialized_yaml_command() {
        let command_string = r#"
description: |
  Start the application
run: |
  echo "sup"
        "#;
        let cmd = serde_yaml::from_str::<Command>(command_string);
        assert!(cmd.is_ok());
    }
}
