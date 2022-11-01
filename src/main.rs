use arboard::Clipboard;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};

// fn main() {
//     let mut clipboard = Clipboard::new().unwrap();
//     println!("Clipboard text was:\n{}", clipboard.get_text().unwrap());
//     println!("");

//     let the_string = "New Clipboard text";
//     clipboard.set_text(the_string.into()).unwrap();
//     println!("Clipboard text was updated!");
// }

use std::io::{self, BufRead};

const FILE_HISTORY: &str = "./history";

fn main() {
    let stdin = io::stdin();
    let mut mode: &str = ""; // :w
    let mut clipboard = Clipboard::new().unwrap();
    for line in stdin.lock().lines() {
        match line {
            Err(_) => break,    // with ^Z
            Ok(s) => match (s.as_str(), mode) {
                (":r", "") => {
                    println!("{}", clipboard.get_text().unwrap());
                    println!(":r|done|");
                }
                // write text to console consists of 2 steps:
                //  1) type :w to turn console into awaiting mode
                //  2) type actual text 
                (":w", "") => {
                    mode = ":w";
                } 
                (text, ":w") => {
                    clipboard.set_text(text.into()).unwrap();
                    mode = "";
                    println!(":w|done|");
                }
                (":s", _) => {
                    // prepend into HISTORY file: https://stackoverflow.com/a/43441946/4728612
                    let text = clipboard.get_text().unwrap();
                    let lines_count = text.split("\n").count();

                    let seconds_since_epoch = get_seconds_since_epoch();
                    let stamp = format!("{{time: {}, lines: {} }}", seconds_since_epoch, lines_count);
                    
                    let data = format!("=== start {} ===\n{}\n=== end {} ===\n", stamp, text, stamp);

                    let mut f = File::create(FILE_HISTORY).expect("Unable to create file");
                    f.write_all(data.as_bytes()).expect("Unable to write data");
                }

                (":sto", "") => {
                    mode = ":sto"
                }
                (path, ":sto") => {
                    // prepend into HISTORY file: https://stackoverflow.com/a/43441946/4728612
                    let text = clipboard.get_text().unwrap();
                    let lines_count = text.split("\n").count();

                    let seconds_since_epoch = get_seconds_since_epoch();
                    let stamp = format!("{{time: {}, lines: {} }}", seconds_since_epoch, lines_count);
                    
                    let data = format!("=== start {} ===\n{}\n", stamp, text);

                    let path_data = Path::new(path);
                    let path_to_file = if path_data.is_relative() { 
                        path_data.join("sto");
                        path_data.to_str().unwrap()
                    } else { 
                        path 
                    };

                    let mut f = File::create(path_to_file).expect("Unable to create file");
                    f.write_all(data.as_bytes()).expect("Unable to write data");
                }

                (":q", _) => {
                    break;
                }
                // Default case when input is unrecognized, so it is returned as is
                (text, _) => println!("{}", text)
            },
        }

    }
    println!(":-|exitting|");
}

fn get_seconds_since_epoch() -> u64 {
    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    
    since_the_epoch.as_secs()
}