use std::env;
use std::process::Command;
use std::str::from_utf8;

macro_rules! u8_expect {
    ($e:expr) => {
        from_utf8($e).expect("Invalid UTF-8 sequence").trim();
    };
}

fn force_option(slash_option: &str, path: &str) -> bool {
    let slash = match slash_option {
        "-w" => "\\",
        "-m" => "/",
        _ => return false,
    };
    if path.starts_with("/mnt/") {
        let disk = path
            .chars()
            .nth(5)
            .unwrap()
            .to_ascii_uppercase()
            .to_string();
        println!("{}:{}{}", disk, slash, &path[7..].replace("/", slash));
        return true;
    } else {
        let wsl_head = match Command::new("/bin/wslpath").arg("-w").arg("/").output() {
            Ok(v) => u8_expect!(&v.stdout).replace("\\", slash),
            Err(e) => panic!("Failed to execute process: /bin/wslpath {}", e),
        };
        let from_root = path.starts_with("/");
        let cur_dir = env::current_dir().unwrap();
        if from_root {
            println!(
                "{}{}",
                wsl_head.replace("\\", slash),
                path[1..].replace("/", "\\")
            );
        } else {
            println!(
                "{}{}{}{}",
                wsl_head.replace("\\", slash),
                cur_dir.to_str().unwrap().replace("/", slash),
                slash,
                path.replace("/", slash)
            );
        }
        return true;
    }
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
            if !v.stderr.is_empty() {
                if force_enable && !force_option(&args[1], &args[3]) || !force_enable {
                    eprintln!("{}", u8_expect!(&v.stderr));
                }
            }
            if !v.stdout.is_empty() {
                println!("{}", u8_expect!(&v.stdout))
            }
        }
        Err(e) => panic!("Failed to execute process: /bin/wslpath {}", e),
    };
}
