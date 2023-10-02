use crate::cli::{Cli, Info};
use crate::config::Config;
use clap::Parser;
use libpassgen::{calculate_length, generate_n_passwords};
use std::io::stdout;
use std::{fs::File, io::LineWriter, io::Write};

use color_eyre::eyre::{eyre, Result};
use color_eyre::owo_colors::OwoColorize;

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

    let length = opts.entropy().map_or(opts.length(), |e| {
        calculate_length(e, pool.len() as f64) as usize
    });

    let pass_vec = generate_n_passwords(&pool, length, opts.count());

    for n in pass_vec.iter().take(opts.count()) {
        println!("{}", n.yellow());
    }

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
                writetxt(pass_vec, &dest).map_err(|e| eyre!(e))?;
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
            writetxt(pass_vec, &dest)?;
        }
    }

    if opts.info() {
        Info::new(length, pool.len()).write(stdout());
    }
    Ok(())
}

fn writetxt(x: Vec<String>, dest: &String) -> Result<()> {
    let file = File::create(dest)?;
    let mut file = LineWriter::new(file);

    for i in x {
        file.write_all(i.as_bytes())?;
        file.write_all(b"\n")?;
    }
    println!("{}", "File Saved.".green());
    Ok(())
}
