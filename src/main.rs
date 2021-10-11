mod pprint;

use std::env;
use std::process::Command;


fn usage() {
    println!(
"Usage:\n  formatter <options> command args...\n\nOptions:
  --prefix       - prefix for every line
  --suffix       - suffix for every line
  --header       - header for the whole output
  --footer       - footer for the whole output
  --line-breaker - line breaker between output lines
  --width        - fix width for every line");
}

fn run(cfg: &pprint::FormatterConfig) {
    if cfg.cmd.is_empty() {
        println!("no command provided");
        return;
    }

    let cmd = Command::new(cfg.cmd.as_str())
        .args(&cfg.args)
        .output()
        .expect("cannot run provided command");
    let output = String::from_utf8(cmd.stdout).unwrap();

    pprint::print_extended(&cfg.header, cfg.width);
    pprint::print_lines(&output, cfg);
    pprint::print_extended(&cfg.footer, cfg.width);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        println!("no command provided");
        return;
    }

    let mut cfg = pprint::FormatterConfig::new();
    let mut arg_idx = 1;

    loop {
        if arg_idx >= args.len() {
            break;
        }

        match args[arg_idx].as_str() {
            "--help" | "-h" => {
                usage();
                return;
            }
            "--prefix" | "--header" | "--footer" | "--line-breaker" | "--suffix" | "--width" => {
                if args.len() < arg_idx + 2 {
                    println!("no value given for flag {}", args[arg_idx]);
                    return;
                }
                let value = args[arg_idx + 1].to_string();
                match args[arg_idx].as_str() {
                    "--prefix" => cfg.prefix = value,
                    "--suffix" => cfg.suffix = value,
                    "--header" => cfg.header = value,
                    "--footer" => cfg.footer = value,
                    "--line-breaker" => cfg.line_breaker = value,
                    "--width" => cfg.width = value.parse().unwrap(),
                    _ => {}
                }
                arg_idx += 2;
            }
            _ => {
                let next = args[arg_idx].to_string();
                if cfg.cmd.is_empty() && next.starts_with('-') {
                    println!("improper command line argument {}", next);
                    return;
                } else if cfg.cmd.is_empty() {
                    cfg.cmd = next;
                } else {
                    cfg.args.push(next);
                }
                arg_idx += 1;
            }
        }
    }

    run(&cfg)
}
