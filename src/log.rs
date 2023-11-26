#![allow(dead_code)]

use std::io::{stdout, Write};

use colored::Colorize;

pub fn success(message: impl Into<String>) {
    println!("[{}] {}", "+".green(), message.into());
}

pub fn error(message: impl Into<String>) {
    println!("[{}] {}", "-".red(), message.into());
}

pub fn info(message: impl Into<String>) {
    println!("[{}] {}", "*".blue(), message.into());
}

pub fn warn(message: impl Into<String>) {
    println!("[{}] {}", "!".yellow(), message.into());
}

pub fn question(message: impl Into<String>) {
    print!("[{}] {} >", "?".purple(), message.into());
    stdout().flush().unwrap();
}