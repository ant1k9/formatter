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
    print!("{}", get_lines(input, cfg));
}

fn get_lines(input: &str, cfg: &FormatterConfig) -> String {
    let lines = input.split('\n').collect::<Vec<&str>>();
    let mut wrap_len = cfg.prefix.chars().count() + cfg.suffix.chars().count();
    let need_wrap = cfg.width == 0 || wrap_len < cfg.width;
    if !need_wrap {
        wrap_len = 0;
    }

    let mut result: String = Default::default();
    if cfg.width > 0 {
        result.reserve(2 * input.chars().count() / (cfg.width - wrap_len) * cfg.width);
    } else {
        result.reserve(input.len() * 2);
    }

    for i in 0..lines.len() {
        let mut line = lines[i].to_owned();
        let mut explicit_runs = 0;
        if line.is_empty() {
            explicit_runs = 1;
        }

        let mut width = cfg.width;
        if width == 0 {
            width = line.chars().count() + wrap_len;
        }

        while line.chars().count() > 0 || explicit_runs > 0 {
            explicit_runs -= 1;
            if need_wrap && !cfg.prefix.is_empty() {
                result.push_str(&cfg.prefix);
            }

            let mut remainder = 0;
            if line.chars().count() < width - wrap_len {
                remainder = width - line.chars().count() - wrap_len;
                width -= remainder;
            }

            let mut next = String::from("");
            for (ch, idx) in line.chars().zip(0..) {
                if idx < width - wrap_len {
                    result.push(ch);
                    continue;
                }
                next.push(ch);
            }
            line = next;

            for _ in 0..remainder {
                result.push(' ');
            }
            if need_wrap && !cfg.suffix.is_empty() {
                result.push_str(&cfg.suffix);
            }
            result.push('\n');
        }

        if lines.len() > 1 && i == lines.len() - 2 && lines[i + 1].is_empty() {
            break;
        }

        if !cfg.line_breaker.is_empty() && i < lines.len() - 1 {
            result.push_str(&get_extended(&cfg.line_breaker, cfg.width));
            result.push('\n');
        }
    }

    result
}

pub fn print_extended(line: &str, width: usize) {
    if line.is_empty() {
        return;
    }
    println!("{}", get_extended(line, width));
}

fn get_extended(line: &str, width: usize) -> String {
    let len = line.chars().count();
    if width > 0 && width > len {
        let mut result: String = Default::default();
        result.reserve(width);

        let mut tail = line.chars().nth_back(0).unwrap();
        if len > 1 {
            tail = line.chars().nth_back(1).unwrap();
        }

        for (ch, idx) in line.chars().zip(0..) {
            if idx == len - 1 {
                break;
            }
            result.push(ch);
        }

        for _ in 0..width - len {
            result.push(tail);
        }
        result.push(line.chars().nth_back(0).unwrap());
        return result;
    }

    line.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_extended_result_returns_as_is_without_width() {
        let result = get_extended("abc", 0);
        assert!(
            result == "abc",
            "without with string should not be modified"
        )
    }

    #[test]
    fn get_extended_result_is_formatted_with_needed_width() {
        let result = get_extended("->", 5);
        assert!(
            result == "---->",
            "improper formatting, expected: `---->`, got: `{}`",
            &result,
        )
    }

    #[test]
    fn get_lines_all_lines_have_prefix() {
        let mut cfg = FormatterConfig::new();
        cfg.prefix = "$ ".to_string();

        let result = get_lines("I'll have\na prefix", &cfg);
        assert!(
            result == "$ I'll have\n$ a prefix\n",
            "improper formatting, expected: `$ I'll have\n$ a prefix\n`, got: `{}`",
            &result,
        );
    }

    #[test]
    fn get_lines_all_lines_have_suffix() {
        let mut cfg = FormatterConfig::new();
        cfg.suffix = "!".to_string();

        let result = get_lines("I'll have\na suffix", &cfg);
        assert!(
            result == "I'll have!\na suffix!\n",
            "improper formatting, expected: `I'll have\n! a suffix!\n`, got: `{}`",
            &result,
        );
    }

    #[test]
    fn get_lines_with_line_breakers() {
        let mut cfg = FormatterConfig::new();
        cfg.line_breaker = "+++".to_string();

        let result = get_lines("1\n2\n3", &cfg);
        assert!(
            result == "1\n+++\n2\n+++\n3\n",
            "improper formatting, expected: `1\n+++\n2\n+++\n3\n`, got: `{}`",
            &result,
        );
    }

    #[test]
    fn get_lines_no_line_breakers_for_last_line_if_it_is_empty() {
        let mut cfg = FormatterConfig::new();
        cfg.line_breaker = "+++".to_string();

        let result = get_lines("1\n2\n3\n", &cfg);
        assert!(
            result == "1\n+++\n2\n+++\n3\n",
            "improper formatting, expected: `1\n+++\n2\n+++\n3\n`, got: `{}`",
            &result,
        );
    }

    #[test]
    fn get_lines_with_fix_width_fill_with_spaces() {
        let mut cfg = FormatterConfig::new();
        cfg.width = 5;
        cfg.line_breaker = "+".to_string();

        let result = get_lines("1\n2\n3\n", &cfg);
        assert!(
            result == "1    \n+++++\n2    \n+++++\n3    \n",
            "improper formatting, expected: `1    \n+++++\n2    \n+++++\n3    \n`, got: `{}`",
            &result,
        );
    }

    #[test]
    fn get_lines_with_prefix_and_suffix_break_in_lines() {
        let mut cfg = FormatterConfig::new();
        cfg.width = 3;
        cfg.prefix = "=".to_string();
        cfg.suffix = "=".to_string();

        let result = get_lines("Go!", &cfg);
        assert!(
            result == "=G=\n=o=\n=!=\n",
            "improper formatting, expected: `=G=\n=o=\n=!=\n`, got: `{}`",
            &result,
        );
    }

    #[test]
    fn get_lines_no_prefix_and_suffix_for_small_width() {
        let mut cfg = FormatterConfig::new();
        cfg.width = 1;
        cfg.prefix = "=".to_string();
        cfg.suffix = "=".to_string();

        let result = get_lines("Go!", &cfg);
        assert!(
            result == "G\no\n!\n",
            "improper formatting, expected: `G\no\n!\n`, got: `{}`",
            &result,
        );
    }

    #[test]
    fn get_lines_with_prefix_and_suffix_break_in_lines_handle_unicode() {
        let mut cfg = FormatterConfig::new();
        cfg.width = 3;
        cfg.prefix = "л".to_string();
        cfg.suffix = "л".to_string();

        let result = get_lines("ооо", &cfg);
        assert!(
            result == "лол\nлол\nлол\n",
            "improper formatting, expected: `лол\nлол\nлол\n`, got: `{}`",
            &result,
        );
    }
}
