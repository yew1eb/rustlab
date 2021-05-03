mod cli;
mod tasks;
use cli::{Action::*, CommandLineArgs};
use std::path::PathBuf;
use structopt::StructOpt;
use tasks::Task;

fn find_default_journal_file() -> Option<PathBuf> {
    home::home_dir().map(|mut path| {
        path.push(".rusty-journal.json");
        path
    })
}

fn main() {
    // Get the command-line arguments.
    let CommandLineArgs {
        action,
        journal_file,
    } = CommandLineArgs::from_args();

    // Unpack the journal file.
    let journal_file = journal_file
        .or_else(find_default_journal_file)
        .expect("Failed to find journal file");

    // Perform the action.
    match action {
        Add { text } => tasks::add_task(journal_file, Task::new(text)),
        List => tasks::list_tasks(journal_file),
        Done { position } => tasks::complete_task(journal_file, position),
    }
    .expect("Failed to perform action")
}

#[cfg(test)]
mod test {
    use super::*;
    use assert_cmd::prelude::*; // Add methods on commands
    use predicates::prelude::*; // Used for writing assertions
    use std::error::Error;
    use std::process::Command; // Run programs
                               // Testing CLI applications by running them: https://rust-cli.github.io/book/tutorial/testing.html#testing-cli-applications-by-running-them
    #[test]
    fn t_main() -> Result<(), Box<std::error::Error>> {
        let mut cmd = Command::cargo_bin("rusty-journal")?;

        cmd.arg("-j").arg("test-journal.json");
        cmd.arg("add").arg("buy milk");

        println!("{}", cmd.assert().success().to_string());

        Ok(())
    }
}
