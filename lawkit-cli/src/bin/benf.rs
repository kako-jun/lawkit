// ベンフォード法則の個別コマンド
// lawkit benfサブコマンドへの薄いラッパー

use std::env;
use std::process::{exit, Command};

const VERSION: &str = "2.2.0";

fn main() {
    let args: Vec<String> = env::args().collect();

    // --version または -V の場合は直接処理
    if args.len() == 2 && (args[1] == "--version" || args[1] == "-V") {
        println!("benf {VERSION}");
        println!("A CLI tool for detecting anomalies using Benford's Law with international numeral support");
        println!("Part of lawkit statistical analysis toolkit");
        exit(0);
    }

    // --help または -h の場合は直接処理
    if args.len() == 2 && (args[1] == "--help" || args[1] == "-h") {
        println!("benf {VERSION}");
        println!("A CLI tool for detecting anomalies using Benford's Law with international numeral support");
        println!();
        println!("This is a convenience wrapper for 'lawkit benf'.");
        println!("All arguments are passed through to the main lawkit command.");
        println!();
        println!("USAGE:");
        println!("    benf [OPTIONS] [INPUT]");
        println!();
        println!("For detailed help, run:");
        println!("    lawkit benf --help");
        exit(0);
    }

    // lawkit benfに全引数を転送
    let lawkit_path = env::current_exe()
        .ok()
        .and_then(|path| path.parent().map(|p| p.join("lawkit")))
        .unwrap_or_else(|| "lawkit".into());

    let mut cmd = Command::new(lawkit_path);
    cmd.arg("benf");

    // 最初の引数（プログラム名）を除いて全て転送
    for arg in args.iter().skip(1) {
        cmd.arg(arg);
    }

    // 実行して結果を転送
    match cmd.status() {
        Ok(status) => {
            if let Some(code) = status.code() {
                exit(code);
            } else {
                // シグナルで終了した場合
                exit(1);
            }
        }
        Err(e) => {
            eprintln!("Error: Failed to execute lawkit benf: {e}");
            eprintln!("Make sure 'lawkit' is installed and available in PATH");
            exit(127);
        }
    }
}
