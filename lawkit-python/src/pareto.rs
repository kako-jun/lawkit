// パレート分析の個別コマンド
// lawkit paretoサブコマンドへの薄いラッパー

use std::env;
use std::process::{exit, Command};

const VERSION: &str = "2.2.0";

fn main() {
    let args: Vec<String> = env::args().collect();

    // --version または -V の場合は直接処理
    if args.len() == 2 && (args[1] == "--version" || args[1] == "-V") {
        println!("pareto {VERSION}");
        println!("A CLI tool for Pareto analysis (80/20 rule) with international numeral support");
        println!("Part of lawkit statistical analysis toolkit");
        exit(0);
    }

    // --help または -h の場合は直接処理
    if args.len() == 2 && (args[1] == "--help" || args[1] == "-h") {
        println!("pareto {VERSION}");
        println!("A CLI tool for Pareto analysis (80/20 rule) with international numeral support");
        println!();
        println!("This is a convenience wrapper for 'lawkit pareto'.");
        println!("All arguments are passed through to the main lawkit command.");
        println!();
        println!("USAGE:");
        println!("    pareto [OPTIONS] [INPUT]");
        println!();
        println!("For detailed help, run:");
        println!("    lawkit pareto --help");
        exit(0);
    }

    // lawkit paretoに全引数を転送
    let lawkit_path = env::current_exe()
        .ok()
        .and_then(|path| path.parent().map(|p| p.join("lawkit")))
        .unwrap_or_else(|| "lawkit".into());

    let mut cmd = Command::new(lawkit_path);
    cmd.arg("pareto");

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
            eprintln!("Error: Failed to execute lawkit pareto: {e}");
            eprintln!("Make sure 'lawkit' is installed and available in PATH");
            exit(127);
        }
    }
}
