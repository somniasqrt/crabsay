//! crabsay - cowsay, rewritten in Rust 🦀 (blazingly fast, obviously).
//!
//! Usage ->  crabsay [-t] [-n] [-W width] [-e eyes] [message...]
//! message from args, or from stdin if no args are given.

use std::io::{self, BufWriter, Read, Write};
use std::process::ExitCode;

struct Opts {
    think: bool,
    nowrap: bool,
    width: usize,
    eyes: String,
    text: String,
}

fn usage() -> ! {
    eprintln!("usage: crabsay [-t] [-n] [-W width] [-e eyes] [message...]");
    std::process::exit(2);
}

fn parse_args() -> Opts {
    let mut o = Opts {
        // `crabthink` symlink behaves like `cowthink`
        think: std::env::args()
            .next()
            .is_some_and(|a| a.ends_with("think")),
        nowrap: false,
        width: 40,
        eyes: "oo".into(),
        text: String::new(),
    };
    let mut words = Vec::new();
    let mut args = std::env::args().skip(1);
    while let Some(a) = args.next() {
        match a.as_str() {
            "-t" => o.think = true,
            "-n" => o.nowrap = true,
            "-W" => o.width = args.next().and_then(|w| w.parse().ok()).unwrap_or_else(|| usage()),
            "-e" => o.eyes = args.next().unwrap_or_else(|| usage()),
            "-h" | "--help" => usage(),
            _ => words.push(a),
        }
    }
    o.width = o.width.max(1);
    let mut eyes: Vec<char> = o.eyes.chars().take(2).collect();
    while eyes.len() < 2 {
        eyes.push(' ');
    }
    o.eyes = eyes.into_iter().collect();

    o.text = if words.is_empty() {
        let mut s = String::new();
        io::stdin().read_to_string(&mut s).unwrap_or_default();
        s
    } else {
        words.join(" ")
    };
    o
}

fn wrap(text: &str, width: usize) -> Vec<String> {
    let mut lines = Vec::new();
    let mut cur = String::new();
    let mut cur_len = 0;
    for word in text.split_whitespace() {
        let mut w: Vec<char> = word.chars().collect();
        while w.len() > width {
            if cur_len > 0 {
                lines.push(std::mem::take(&mut cur));
                cur_len = 0;
            }
            let rest = w.split_off(width);
            lines.push(w.into_iter().collect());
            w = rest;
        }
        let wl = w.len();
        if cur_len > 0 && cur_len + 1 + wl > width {
            lines.push(std::mem::take(&mut cur));
            cur_len = 0;
        }
        if cur_len > 0 {
            cur.push(' ');
            cur_len += 1;
        }
        cur.extend(w);
        cur_len += wl;
    }
    if cur_len > 0 || lines.is_empty() {
        lines.push(cur);
    }
    lines
}

fn bubble(out: &mut impl Write, lines: &[String], think: bool) -> io::Result<()> {
    let max = lines.iter().map(|l| l.chars().count()).max().unwrap_or(0);
    writeln!(out, " {}", "_".repeat(max + 2))?;
    let n = lines.len();
    for (i, l) in lines.iter().enumerate() {
        let (a, b) = match (think, n, i) {
            (true, _, _) => ('(', ')'),
            (_, 1, _) => ('<', '>'),
            (_, _, 0) => ('/', '\\'),
            (_, _, i) if i == n - 1 => ('\\', '/'),
            _ => ('|', '|'),
        };
        let pad = max - l.chars().count();
        writeln!(out, "{a} {l}{} {b}", " ".repeat(pad))?;
    }
    writeln!(out, " {}", "-".repeat(max + 2))
}

fn crab(out: &mut impl Write, think: bool, eyes: &str) -> io::Result<()> {
    let t = if think { 'o' } else { '\\' };
    let mut e = eyes.chars();
    let (l, r) = (e.next().unwrap(), e.next().unwrap());
    write!(
        out,
        "        {t}
         {t}
           _~^~^~_
       \\) /  {l} {r}  \\ (/
         '_   ^   _'
         / '-----' \\
"
    )
}

fn main() -> ExitCode {
    let o = parse_args();
    let lines = if o.nowrap {
        let v: Vec<String> = o.text.lines().map(|l| l.replace('\t', "    ")).collect();
        if v.is_empty() { vec![String::new()] } else { v }
    } else {
        wrap(&o.text, o.width)
    };
    let stdout = io::stdout();
    let mut out = BufWriter::new(stdout.lock());
    let res = bubble(&mut out, &lines, o.think)
        .and_then(|_| crab(&mut out, o.think, &o.eyes))
        .and_then(|_| out.flush());
    res.unwrap();
    ExitCode::SUCCESS
}
