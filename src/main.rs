fn wrap(text: &str, width: usize) -> Vec<String> {
    let mut lines = Vec::new();
    let mut cur = String::new();
    let mut cur_len = 0;
    for word in text.split_whitespace() {
        let wl = word.chars().count();
        if cur_len > 0 && cur_len + 1 + wl > width {
            lines.push(std::mem::take(&mut cur));
            cur_len = 0;
        }
        if cur_len > 0 {
            cur.push(' ');
            cur_len += 1;
        }
        cur.push_str(word);
        cur_len += wl;
    }
    if cur_len > 0 || lines.is_empty() {
        lines.push(cur);
    }
    lines
}

fn bubble(lines: &[String]) {
    let max = lines.iter().map(|l| l.chars().count()).max().unwrap_or(0);
    println!(" {}", "_".repeat(max + 2));
    let n = lines.len();
    for (i, l) in lines.iter().enumerate() {
        let (a, b) = match (n, i) {
            (1, _) => ('<', '>'),
            (_, 0) => ('/', '\\'),
            (_, i) if i == n - 1 => ('\\', '/'),
            _ => ('|', '|'),
        };
        let pad = max - l.chars().count();
        println!("{a} {l}{} {b}", " ".repeat(pad));
    }
    println!(" {}", "-".repeat(max + 2));
}

fn main() {
    let mut width = 40;
    let mut words = Vec::new();
    let mut args = std::env::args().skip(1);
    while let Some(a) = args.next() {
        if a == "-W" {
            width = args.next().and_then(|w| w.parse().ok()).unwrap_or(40);
        } else {
            words.push(a);
        }
    }
    bubble(&wrap(&words.join(" "), width.max(1)));
    print!(
        "        \\
         \\
           _~^~^~_
       \\) /  o o  \\ (/
         '_   ^   _'
         / '-----' \\
"
    );
}
