use crate::cli::Cli;
use crate::config::Config;
use clap::Parser;
use libpassgen::generate_n_passwords;
extern crate zxcvbn;
use zxcvbn::zxcvbn;
use std::{fs::File, io::LineWriter, io::Write};
use std::slice::Iter;
use color_eyre::eyre::{eyre, Result};
use color_eyre::owo_colors::OwoColorize;
use itertools::{Format, Itertools};

extern crate question;

use question::{Answer, Question};

pub mod cli;
pub mod config;

fn main() {
    if let Err(e) = run() {
        eprintln!("{} {}", "Error :".red(), e.red());
        std::process::exit(1);
    }

}

fn run() -> Result<()> {
    // enable Windows terminal colors
    #[cfg(windows)]
    let _ = enable_ansi_support::enable_ansi_support();

    let opts: Cli = Cli::parse();

    if opts.reset() {
        Config::save_default().map_err(|e| eyre!(e))?;
        println!(
            "{}",
            "The default configuration was set successfully !".green()
        );
        std::process::exit(0);
    }

    let pool = opts.collect();

    let pass_vec = generate_n_passwords(&pool, opts.length(), opts.count());

    // for n in pass_vec.iter().take(opts.count()) {
    //    println!("{}", n.yellow());
    // }
    let test=pass_vec.iter().format("-").to_string();
    println!("{}", test.cyan());

    if opts.output().is_some() {
        let dest = opts.output().unwrap();
        if std::path::Path::new(&dest).is_dir() && &dest != "/dev/null" {
            return Err(eyre!("Can't save file. A folder with this name exist."));
        }
        if std::path::Path::new(&dest).exists() && &dest != "/dev/null" {
            println!("File: '{}' exist.", &dest);
            let answer = Question::new("Try to Overwrite ?")
                .default(Answer::NO)
                .show_defaults()
                .confirm();

            if answer == Answer::YES {
                writetxt(pass_vec.iter().format("-"), &dest).map_err(|e| eyre!(e))?;
                println!(
                    "{} '{}' {}",
                    "File".green(),
                    opts.output().unwrap().green(),
                    "was overwritten.".green()
                );
            } else {
                println!("{}", "Writting file canceled.".green());
            }
        } else {
            writetxt(pass_vec.iter().format("-"), &dest)?;
        }
    }

    if opts.info() {
        let estimate = zxcvbn(&test, &[]);

        println!("Bits: {:?}", estimate.guesses_log10());
    }
    Ok(())
}

fn writetxt(x: Format<Iter<String>>, dest: &String) -> Result<()> {
    let file = File::create(dest)?;
    let mut file = LineWriter::new(file);

    let z=x.to_string();
    file.write_all(z.as_bytes())?;

    println!("{}", "File Saved.".green());

    Ok(())
}
