fn main() {
    let text = std::env::args().skip(1).collect::<Vec<_>>().join(" ");
    let len = text.chars().count();
    println!(" {}", "_".repeat(len + 2));
    println!("< {text} >");
    println!(" {}", "-".repeat(len + 2));
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
