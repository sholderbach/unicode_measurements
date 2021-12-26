use crossterm::{
    execute,
    style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor},
};
use std::env::args;
use std::io::stdout;
use unicode_width::UnicodeWidthStr;

const EXAMPLES: [&str; 9] = [
    "h",
    "😇",
    "👪",
    "⥹",
    "ﷲ",
    "hello ⥹",
    "🇦🇺",
    "🤦🏿‍♀️",
    "👨‍👩‍👧‍👦",
];
fn main() {
    let mut simple_examples = Vec::from_iter(EXAMPLES.iter().map(|&i| i.to_owned()));
    match args().nth(1) {
        Some(x) if x == "--manual" => manual_visual_verification(&simple_examples),
        Some(x) if x == "--full" => {
            let mut examples = get_emojis_from_json();
            examples.append(&mut simple_examples);
            full_ansi_assertion(&examples);
        }
        None => {
            let mut examples = get_emojis_from_json();
            examples.append(&mut simple_examples);
            basic_assertion(&examples);
        }
        _ => panic!("Unknown argument"),
    }
}

fn get_emojis_from_json() -> Vec<String> {
    let jtree = json::parse(&std::fs::read_to_string("gh-emoji.json").unwrap()).unwrap();
    jtree
        .members()
        .filter_map(|e| e["emoji"].as_str().map(|s| s.to_owned()))
        .collect()
}

fn manual_visual_verification(examples: &[String]) {
    for example in examples {
        print_str_with_width(&example);
    }
}

fn print_str_with_width(content: &str) {
    let width = content.width();

    println!(
        "\"{content}\" width = {width}",
        content = content,
        width = width
    );

    execute!(
        stdout(),
        SetBackgroundColor(Color::Green),
        Print(content),
        ResetColor,
        Print("\n")
    )
    .unwrap();
    execute!(
        stdout(),
        SetForegroundColor(Color::Red),
        Print(content),
        ResetColor,
        Print("\n")
    )
    .unwrap();
}

fn basic_assertion(examples: &[String]) {
    for example in examples {
        let estimated_length = example.width();
        assert_length(example, estimated_length);
    }
    println!();
    println!("Success!");
}

fn full_ansi_assertion(examples: &[String]) {
    for example in examples {
        let estimated_length = example.width();
        assert_length(example, estimated_length);
        assert_ansi_foreground_length(example, estimated_length);
        assert_ansi_background_length(example, estimated_length);
    }
    println!();
    println!("Success!");
}

fn assert_length(content: &str, length: usize) {
    println!();
    let (previous_x, previous_y) = crossterm::cursor::position().unwrap();
    execute!(stdout(), Print(content)).unwrap();
    let (new_x, new_y) = crossterm::cursor::position().unwrap();
    assert_eq!(new_y, previous_y, "Not on the same row anymore!");
    let measured = new_x - previous_x;
    assert_eq!(
        length as u16, measured,
        "Estimate does not equal the assumed length"
    );
}

fn assert_ansi_foreground_length(content: &str, length: usize) {
    println!();
    let (previous_x, previous_y) = crossterm::cursor::position().unwrap();
    execute!(
        stdout(),
        SetForegroundColor(Color::Red),
        Print(content),
        ResetColor
    )
    .unwrap();
    let (new_x, new_y) = crossterm::cursor::position().unwrap();
    assert_eq!(new_y, previous_y, "Not on the same row anymore!");
    let measured = new_x - previous_x;
    assert_eq!(
        length as u16, measured,
        "Estimate does not equal the assumed length (Foreground color changed)"
    );
}
fn assert_ansi_background_length(content: &str, length: usize) {
    println!();
    let (previous_x, previous_y) = crossterm::cursor::position().unwrap();
    execute!(
        stdout(),
        SetBackgroundColor(Color::Green),
        Print(content),
        ResetColor
    )
    .unwrap();
    let (new_x, new_y) = crossterm::cursor::position().unwrap();
    assert_eq!(new_y, previous_y, "Not on the same row anymore!");
    let measured = new_x - previous_x;
    assert_eq!(
        length as u16, measured,
        "Estimate does not equal the assumed length (Background colored)"
    );
}
