use std::env;
use std::process::Command;
use std::str::from_utf8;
use std::string::String;
use wslpath_ext::force_option;
macro_rules! u8_expect {
    ($e:expr) => {
        String::from(from_utf8($e).expect("Invalid UTF-8 sequence").trim())
    };
}
fn get_wsl() -> String {
    if env::consts::OS == "windows" {
        return String::from("wsl");
    }
    return String::from("wsl");
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let force_enable = args.len() == 4 && args[2] == "-f";
    let real_args = if force_enable {
        [&args[1..2], &args[3..]].concat()
    } else {
        args[1..].to_vec()
    };
    match Command::new("/bin/wslpath").args(real_args).output() {
        Ok(v) => {
            let stderr = u8_expect!(&v.stderr);
            if stderr.ends_with("No such file or directory") && force_enable {
                match force_option(&args[1], &args[3]) {
                    Ok(v) => println!("{}", v),
                    Err(e) => eprintln!("{}", e),
                }
            } else if !v.status.success() {
                eprintln!("{}", stderr);
            }
            if !v.stdout.is_empty() {
                println!("{}", u8_expect!(&v.stdout))
            }
        }
        Err(e) => eprintln!("Failed to execute process: /bin/wslpath {}", e),
    };
}
