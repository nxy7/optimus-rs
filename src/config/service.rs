use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::utils::string_or_struct::string_or_struct;

use super::command::Command;

#[derive(Serialize, Deserialize, Debug)]
pub struct Service {
    root: Option<String>,
    description: Option<String>,
    #[serde(flatten)]
    commands: HashMap<String, Command>,
}

impl Service {
    pub fn to_clap_command(self, name: String) -> clap::Command {
        let mut clap_cmd = clap::Command::new(name.clone()).subcommand_required(true);
        if let Some(desc) = self.description {
            let desc = desc.trim().to_string();
            clap_cmd = clap_cmd.about(desc);
        } else {
            clap_cmd = clap_cmd.about(format!("Commands for {} service", name));
        }
        clap_cmd
    }
}
