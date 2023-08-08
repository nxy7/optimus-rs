mod config;
mod utils;

use clap::Command;
pub use config::Config;

pub fn start(conf: Config) {
    let mut root = get_root_command();

    for (key, service) in conf.services {
        root = root.subcommand(service.to_clap_command(key));
    }

    for (key, cmd) in conf.commands {
        root = root.subcommand(cmd.to_clap_command(key));
    }

    let matches = root.get_matches();

    // if matches.subcommand()
}

fn get_root_command() -> Command {
    Command::new("optimus")
        .author("nxyt")
        .version("0.0.2")
        .about("Framework that helps manage monorepositories using shell scripting languages")
        .arg_required_else_help(true)
        .subcommand_required(true)
        // .help
        // .arg(Arg::new("in_file"))
        .after_help(
            "Longer explanation to appear after the options when \
                 displaying the help information from --help or -h",
        )
}
