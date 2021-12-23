use std::io::{Write, stdout};
use crossterm::style::Print;
use crossterm::style::ResetColor;
use unicode_width::UnicodeWidthStr;
use crossterm::execute;
use crossterm::style::Color;
use crossterm::style::SetForegroundColor;
use crossterm::style::SetBackgroundColor;
fn main() {
    print_str_with_width("h");
    print_str_with_width("😇");
    print_str_with_width("👪");
    print_str_with_width("⥹");
    print_str_with_width("ﷲ");
    print_str_with_width("hello ⥹");
    print_str_with_width("🇦🇺");
    print_str_with_width("🤦🏿‍♀️");
    print_str_with_width("👨‍👩‍👧‍👦");
}

fn print_str_with_width(content: &str) {
    let width = content.width();
    
    println!("\"{content}\" width = {width}", content = content, width = width);
    
    execute!(stdout(), SetBackgroundColor(Color::Green), Print(content), ResetColor, Print("\n")).unwrap();
    execute!(stdout(), SetForegroundColor(Color::Red), Print(content), ResetColor, Print("\n")).unwrap();
}