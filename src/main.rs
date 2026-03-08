use clap::Parser;
use color_eyre::config::HookBuilder;
use std::{env};

use crate::history_scraper::find_final_dir;

mod history_scraper;
mod installer;

/// Quick Switch
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Name of directory to go to
    #[arg(index = 1, help="directory-matcher, e.g. 'intelij' for 'IntelliJProjects'")]
    directory: Option<String>,
    #[arg(short, long, help="to install Quick-Stitch as an alias to your ~/.bashrc", conflicts_with="directory")]
    install: bool,
}


// Open up bash_history
// Find all instances of cd and directory
// Fuzzy match the directories with the input, weight the matches on frequency too
// Message from Evie: [-======================\\\\\\\\\\\\\\\\iooooooooooollll
fn main() {
    let _ = HookBuilder::default()
    .display_env_section(true)
    .display_location_section(true)
    .panic_section(true)
    .install();

    if Args::parse().install  {
        println!("Creating Quick-Switch function in your ~/.bashrc...");
        let _ = installer::install();
        println!("Done! Run with 'qs <dir>' :)");
    } 
    else if let Some(directory) = Args::parse().directory {
        let input = directory;
        let bash_history_fp = format!("/home/{}/.bash_history", std::env::var("USER").unwrap());
        let final_dir_result = find_final_dir(&input, &bash_history_fp);

        println!("{}", final_dir_result.unwrap());
    }
}
