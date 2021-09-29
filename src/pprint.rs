pub struct FormatterConfig {
    pub prefix: String,
    pub suffix: String,
    pub header: String,
    pub footer: String,
    pub line_breaker: String,
    pub width: usize,
    pub cmd: String,
    pub args: Vec<String>,
}

impl FormatterConfig {
    pub fn new() -> Self {
        Self {
            prefix: "".to_owned(),
            suffix: "".to_owned(),
            header: "".to_owned(),
            footer: "".to_owned(),
            width: 0,
            line_breaker: "".to_owned(),
            cmd: "".to_owned(),
            args: Vec::new(),
        }
    }
}

pub fn print_lines(input: &str, cfg: &FormatterConfig) {
    let lines = input.split("\n").collect::<Vec<&str>>();
    let mut wrap_len = cfg.prefix.chars().count() + cfg.suffix.chars().count();
    let need_wrap = cfg.width == 0 || wrap_len < cfg.width;
    if !need_wrap {
        wrap_len = 0;
    }

    for i in 0..lines.len() {
        let mut line = lines[i].to_owned();

        let mut width = cfg.width;
        if width == 0 {
            width = line.chars().count() + wrap_len;
        }

        while line.chars().count() > 0 {
            if need_wrap && cfg.prefix != "" {
                print!("{}", cfg.prefix);
            }

            let mut remainder = 0;
            if line.chars().count() < width - wrap_len {
                remainder = width - line.chars().count() - wrap_len;
                width = width - remainder;
            }

            let mut next = String::from("");
            for (ch, idx) in line.chars().zip(0..) {
                if idx < width - wrap_len {
                    print!("{}", ch);
                    continue;
                }
                next.push(ch);
            }
            line = next;

            for _ in 0..remainder {
                print!(" ");
            }
            if need_wrap && cfg.suffix != "" {
                print!("{}", cfg.suffix);
            }
            println!();
        }

        if i == lines.len() - 2 && lines[i + 1] == "" {
            break;
        }

        if cfg.line_breaker != "" {
            print_extended(&cfg.line_breaker, cfg.width);
        }
    }
}

pub fn print_extended(line: &str, width: usize) {
    if line == "" {
        return;
    }

    let len = line.chars().count();
    if width > 0 && width > len && len > 0 {
        let mut tail = line.chars().nth_back(0).unwrap();
        if len > 1 {
            tail = line.chars().nth_back(1).unwrap();
        }

        for (ch, idx) in line.chars().zip(0..) {
            if idx == len - 1 {
                break;
            }
            print!("{}", ch);
        }

        for _ in 0..width - len {
            print!("{}", tail);
        }
        println!("{}", line.chars().nth_back(0).unwrap());
        return;
    }

    println!("{}", line);
}
