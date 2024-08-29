use std::fmt;
use std::io::{self, Write, Read};
use std::process::{Command, ExitStatus, Output};

#[allow(non_camel_case_types, dead_code)]
pub enum Colors {
    FG_GRAY,
    FG_BLACK,
    FG_RED,
    FG_GREEN,
    FG_YELLOW,
    FG_BLUE,
    FG_MAGENTA,
    FG_CYAN,
    FG_WHITE,
    RESET,
}

impl fmt::Display for Colors {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let color_str = match *self {
            Colors::FG_GRAY => "\x1b[90m",
            Colors::FG_BLACK => "\x1b[30m",
            Colors::FG_RED => "\x1b[31m",
            Colors::FG_GREEN => "\x1b[32m",
            Colors::FG_YELLOW => "\x1b[33m",
            Colors::FG_BLUE => "\x1b[34m",
            Colors::FG_MAGENTA => "\x1b[35m",
            Colors::FG_CYAN => "\x1b[36m",
            Colors::FG_WHITE => "\x1b[37m",
            Colors::RESET => "\x1b[0m",
        };
        write!(f, "{}", color_str)
    }
}


macro_rules! println_shell {
    ($($arg:tt)*) => ({
        println!("{}$ {}{}", Colors::FG_GRAY, format_args!($($arg)*), Colors::RESET);
    })
}

#[allow(unused_macros)]
macro_rules! println_shell_colorful {
    ($arg:expr, $color:expr) => ({
        let arg_start = $arg.get_program().to_str().unwrap();
        let arg_vec = $arg.get_args();

        print!("{}$ {}", Colors::FG_GRAY, arg_start);
        for (i, item) in arg_vec.enumerate() {
            print!("{}", $color[i]);
            print!(" {}", item.to_str().unwrap());
        }
        print!("{}\n", Colors::RESET);
        let _= std::io::stdout().flush();
    })
}

macro_rules! new {
    ($bin:expr, $($x:expr),* $(,)?) => {
        {
            let mut command = std::process::Command::new($bin);
            $(command.arg($x);)*
            crate::shell::Shell::new(command)
        }
    };
}

#[allow(unused_macros)]
macro_rules! new_colorful {
    ($bin:expr, $($x:expr),* $(,)?) => {
        {
            let mut color_vec: Vec<String> = vec![];
            let mut command = std::process::Command::new($bin);
            $(command.arg($x.0);)*
            $(color_vec.push($x.1.to_string());)*
            crate::shell::Shell::new_colorful(command, color_vec)
        }
    };
}

pub(crate) use new;

// Turn off color imports for now, can't find a pretty way to use
#[allow(unused_imports)]
pub(crate) use new_colorful;

#[derive(Debug)]
pub struct Shell {
    command: Command,
    color_vec: Option<Vec<String>>,
}

impl Shell {
    pub fn new(command: Command) -> Self {
        Shell {
            command: command,
            color_vec: None
        }
    }

    pub fn new_colorful(command: Command, color_vec: Vec<String>) -> Self {
        Shell {
            command: command,
            color_vec: Some(color_vec),
        }
    }


    pub fn output(&mut self, print: bool) -> Result<Output, ShellError> {
        if print {
            println_shell!("{}\n", self);
        }

        self.command.output().map_err(|e| ShellError::Io {
            shell: self.to_string(),
            source: e,
        })
    }

    pub fn run(&mut self, print: bool) -> Result<(), ShellError> {
        if print {
            println_shell!("{}\n", self);
        }

        self.status()?;

        Ok(())
    }

    pub fn run_yorn(&mut self) -> Result<(), ShellError> {
        println_shell!("{}\n", self);
        let yorn = self.yorn(&String::from("Execute above command? (Y/n): "));
        if yorn {
            self.status()?;
        }

        Ok(())
    }

    pub fn run_yorn_colorful(&mut self) {
        let colors = self.color_vec.clone().unwrap();
        println_shell_colorful!(self.command, &colors);
    }

    pub fn status(&mut self) -> Result<ExitStatus, ShellError> {
        self.command.status().map_err(|e| ShellError::Io {
            shell: self.to_string(),
            source: e,
        })
    }

    fn yorn(&mut self, question: &str) -> bool {
        print!("{}", question);
        let _= std::io::stdout().flush();
        loop {
            let mut input = [0];
            let _ = std::io::stdin().read(&mut input);
            match input[0] as char {
                'Y' => return true,
                'n' | 'N' => return false,
                _ => println!("Y/n only please."),
            }
        }
    }
}

#[derive(thiserror::Error, Debug)]
pub enum ShellError {
    #[error("error running \"{shell}\"")]
    Io {
        shell: String,
        #[source]
        source: io::Error,
    },
}

impl fmt::Display for Shell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", &self.command.get_program().to_str().unwrap())?;

        for arg in self.command.get_args() {
            write!(f, " {}", arg.to_str().unwrap())?;
        }

        Ok(())
    }
}
