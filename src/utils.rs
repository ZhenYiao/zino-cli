pub fn zino_hello() {
    println!(
        "{}",
        ansi_term::Color::Blue.paint(
            r#"
     ______
     |__  / (_)  _ __     ___
       / /  | | | '_ \   / _ \
      / /_  | | | | | | | (_) |
     /____| |_| |_| |_|  \___/
    "#
        )
    );
}
