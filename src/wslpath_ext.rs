use std::io::{Error, ErrorKind, Result};
use std::path::Path;
use std::str::from_utf8;
use std::process::Command;
macro_rules! u8_expect {
    ($e:expr) => {
        String::from(from_utf8($e).expect("Invalid UTF-8 sequence").trim())
    };
}
macro_rules! error_wrapper {
    ($e: expr, $s: expr) => {
        Err(Error::new($e, $s))
    };
}
macro_rules! invalid_input {
    ($s: expr) => {
        error_wrapper!(ErrorKind::InvalidInput, $s)
    };
}
macro_rules! not_found {
    ($s: expr) => {
        error_wrapper!(ErrorKind::NotFound, $s)
    };
}
pub fn force_option(slash_option: &str, path: &str) -> Result<String> {
    let slash = match slash_option {
        "-w" => "\\",
        "-m" => "/",
        _ => return invalid_input!(format!("unrecognized option: {}", slash_option)),
    };
    let real_path = match Command::new("realpath").arg("-m").arg(path).output() {
        Ok(v) => {
            if !v.stderr.is_empty() {
                return invalid_input!(format!("Failed to generate {}", u8_expect!(&v.stderr)));
            } else if !v.stdout.is_empty() {
                u8_expect!(&v.stdout)
            } else {
                return invalid_input!("No output from realpath");
            }
        }
        Err(e) => {
            return not_found!(format!("Failed to execute process: realpath {}", e));
        }
    };
    let components = Path::new(&real_path)
        .components()
        .map(|comp| comp.as_os_str().to_str().unwrap())
        .collect::<Vec<_>>();
    if components.len() >= 3 && components[1] == "mnt" && components[2].len() == 1 {
        let disk = components[2].chars().collect::<Vec<_>>()[0];
        if disk.is_ascii_lowercase() {
            return Ok(format!(
                "{}:{}{}",
                disk.to_uppercase(),
                slash,
                components[3..].join(slash)
            ));
        }
    }
    let wsl_head = match Command::new("/bin/wslpath")
        .arg(slash_option)
        .arg("/")
        .output()
    {
        Ok(v) => u8_expect!(&v.stdout),
        Err(e) => {
            return invalid_input!(format!("Failed to execute process: /bin/wslpath {}", e));
        }
    };
    return Ok(format!(
        "{}{}",
        wsl_head,
        real_path[1..].replace("/", slash)
    ));
}
