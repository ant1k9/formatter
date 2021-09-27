use std::env;
use std::process::Command;

struct FormatterConfig {
    prefix: String,
    suffix: String,
    header: String,
    footer: String,
    line_breaker: String,
    cmd: String,
    args: Vec<String>,
}

impl FormatterConfig {
    pub fn new() -> Self {
        Self {
            prefix: "".to_owned(),
            suffix: "".to_owned(),
            header: "".to_owned(),
            footer: "".to_owned(),
            line_breaker: "".to_owned(),
            cmd: "".to_owned(),
            args: Vec::new(),
        }
    }
}

fn usage() {
    println!("Usage:\n  formatter <options> command args...\n\nOptions:");
    println!("  --prefix       - prefix for every line");
    println!("  --suffix       - suffix for every line");
    println!("  --header       - header for the whole output");
    println!("  --footer       - footer for the whole output");
    println!("  --line-breaker - line breaker between output lines");
}

fn run(cfg: &FormatterConfig) {
    if cfg.cmd == "" {
        println!("no command provided");
        return;
    }

    let cmd = Command::new(cfg.cmd.as_str())
        .args(&cfg.args)
        .output()
        .expect("cannot run provided command");
    let output = String::from_utf8(cmd.stdout).unwrap();

    if cfg.header != "" {
        println!("{}", cfg.header);
    }

    let lines = output.split("\n").collect::<Vec<&str>>();
    for i in 0..lines.len() {
        let line = lines[i];
        if cfg.prefix != "" {
            print!("{}", cfg.prefix);
        }
        print!("{}", line);
        if cfg.suffix != "" {
            print!("{}", cfg.suffix);
        }

        if i == lines.len() - 2 && lines[i + 1] == "" {
            println!();
            break;
        }

        if cfg.line_breaker != "" {
            println!("\n{}", cfg.line_breaker);
        } else {
            println!();
        }
    }

    if cfg.footer != "" {
        println!("{}", cfg.footer);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        println!("no commands provided");
        return
    }

    let mut config = FormatterConfig::new();
    let mut arg_idx = 1;

    loop {
        if arg_idx >= args.len() {
            break;
        }

        match args[arg_idx].as_str() {
            "--help" | "-h" => {
                usage();
                return
            },
            "--prefix" | "--header" | "--footer" | "--line-breaker" | "--suffix" => {
                if args.len() < arg_idx + 2 {
                    println!("no value given for flag {}", args[arg_idx]);
                    return
                }
                let value = args[arg_idx + 1].to_string();
                match args[arg_idx].as_str() {
                   "--prefix" => config.prefix = value,
                   "--suffix" => config.suffix = value,
                   "--header" => config.header = value,
                   "--footer" => config.footer = value,
                   "--line-breaker" => config.line_breaker = value,
                   _ => {},
                }
                arg_idx += 2;
            }
            _ => {
                let next = args[arg_idx].to_string();
                if config.cmd == "" {
                    config.cmd = next;
                } else {
                    config.args.push(next);
                }
                arg_idx += 1;
            }
        }
    }

    run(&config)
}
