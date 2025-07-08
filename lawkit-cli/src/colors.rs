use is_terminal::IsTerminal;
use owo_colors::OwoColorize;
use std::env;

/// 色付けが有効かどうか判定 (diffx準拠)
pub fn should_use_color() -> bool {
    // NO_COLOR環境変数チェック (https://no-color.org/)
    if env::var("NO_COLOR").is_ok() {
        return false;
    }
    
    // FORCE_COLOR環境変数チェック
    if env::var("FORCE_COLOR").is_ok() {
        return true;
    }
    
    // 標準出力がターミナルかチェック (パイプやリダイレクト検出)
    std::io::stdout().is_terminal()
}

/// PASSステータスの色付け
pub fn pass(text: &str) -> String {
    if should_use_color() {
        format!("{}", text.bright_green().bold())
    } else {
        text.to_string()
    }
}

/// WARNステータスの色付け
pub fn warn(text: &str) -> String {
    if should_use_color() {
        format!("{}", text.bright_yellow().bold())
    } else {
        text.to_string()
    }
}

/// FAILステータスの色付け
pub fn fail(text: &str) -> String {
    if should_use_color() {
        format!("{}", text.bright_red().bold())
    } else {
        text.to_string()
    }
}

/// CRITICALステータスの色付け
pub fn critical(text: &str) -> String {
    if should_use_color() {
        format!("{}", text.bright_magenta().bold())
    } else {
        text.to_string()
    }
}

/// INFOテキストの色付け
pub fn info(text: &str) -> String {
    if should_use_color() {
        format!("{}", text.bright_cyan())
    } else {
        text.to_string()
    }
}

/// ALERTテキストの色付け
pub fn alert(text: &str) -> String {
    if should_use_color() {
        format!("{}", text.bright_yellow())
    } else {
        text.to_string()
    }
}