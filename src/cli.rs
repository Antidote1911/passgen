use crate::config::Config;

use clap::Parser;
use libpassgen::Pool;

#[derive(Parser, Debug)]
#[clap(
    author,
    about,
    version,
    after_help = "If you do not specify any of the \
[--uppercase, --lowercase, --digits, --symbols, --others] flags, \
then uppercase, lowercase letters and digits will be used."
)]
pub struct Cli {
    /// Use UPPERCASE letters [A-Z]
    #[clap(short, long)]
    uppercase: bool,

    /// Use lowercase letters [a-z]
    #[clap(short, long)]
    lowercase: bool,

    /// Use digits [0-9]
    #[clap(short, long)]
    digits: bool,

    /// Use special symbols [*&^%$#@!~]
    #[clap(short, long)]
    symbols: bool,

    /// Use other symbols [♕♖♗♘♙♚...].
    #[clap(short, long)]
    others: bool,

    /// Sets the required password length
    #[clap(short = 'L', long, value_name = "NUMBER", default_value = "8")]
    length: usize,

    /// Number of sections for serial
    #[clap(short, long, value_name = "NUMBER", default_value = "1")]
    count: usize,

    /// Prints password information
    #[clap(short, long)]
    info: bool,

    /// Output in a txt file
    #[clap(long)]
    output: Option<String>,

    /// Sets config to default values
    #[clap(long = "config")]
    reset: bool,

    #[clap(skip = Config::new())]
    config: Config,
}

impl Cli {
    pub fn collect(&self) -> Pool {
        let mut pool = Pool::new();

        if self.uppercase {
            pool.extend_from_string(self.config.uppercase());
        }
        if self.lowercase {
            pool.extend_from_string(self.config.lowercase());
        }
        if self.digits {
            pool.extend_from_string(self.config.digits());
        }
        if self.symbols {
            pool.extend_from_string(self.config.symbols());
        }
        if self.others {
            pool.extend_from_string(self.config.others());
        }

        if pool.is_empty() {
            pool.extend_from_string(self.config.default_set());
        }

        pool
    }

    pub fn count(&self) -> usize {
        self.count
    }

    pub fn reset(&self) -> bool {
        self.reset
    }

    pub fn length(&self) -> usize {
        self.length
    }

    pub fn info(&self) -> bool {
        self.info
    }

    pub fn output(&self) -> Option<String> {
        self.output.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn opts_collect() {
        let opts = Cli {
            uppercase: true,
            lowercase: true,
            digits: true,
            symbols: true,
            others: true,
            length: 0,
            count: 1,
            info: false,
            output: None,
            reset: false,
            config: Config::default(),
        };
        let pool = opts.collect();

        assert!(pool.contains_all(opts.config.uppercase()));
        assert!(pool.contains_all(opts.config.lowercase()));
        assert!(pool.contains_all(opts.config.digits()));
        assert!(pool.contains_all(opts.config.symbols()));
        assert!(pool.contains_all(opts.config.others()));
    }


}
