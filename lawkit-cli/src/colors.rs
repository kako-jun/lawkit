use is_terminal::IsTerminal;
use owo_colors::OwoColorize;

/// 色付けが有効かどうか判定
pub fn should_use_color(no_color_flag: bool) -> bool {
    // --no-color フラグが指定されている場合は無色
    if no_color_flag {
        return false;
    }

    // 標準出力がターミナルかチェック (パイプやリダイレクト検出)
    std::io::stdout().is_terminal()
}

/// PASSステータスの色付け
pub fn pass(text: &str, no_color: bool) -> String {
    if should_use_color(no_color) {
        format!("{}", text.bright_green().bold())
    } else {
        text.to_string()
    }
}

/// WARNステータスの色付け
pub fn warn(text: &str, no_color: bool) -> String {
    if should_use_color(no_color) {
        format!("{}", text.bright_yellow().bold())
    } else {
        text.to_string()
    }
}

/// FAILステータスの色付け
pub fn fail(text: &str, no_color: bool) -> String {
    if should_use_color(no_color) {
        format!("{}", text.bright_red().bold())
    } else {
        text.to_string()
    }
}

/// CRITICALステータスの色付け
pub fn critical(text: &str, no_color: bool) -> String {
    if should_use_color(no_color) {
        format!("{}", text.bright_magenta().bold())
    } else {
        text.to_string()
    }
}

/// INFOテキストの色付け
pub fn info(text: &str, no_color: bool) -> String {
    if should_use_color(no_color) {
        format!("{}", text.bright_cyan())
    } else {
        text.to_string()
    }
}

/// ALERTテキストの色付け
pub fn alert(text: &str, no_color: bool) -> String {
    if should_use_color(no_color) {
        format!("{}", text.bright_yellow())
    } else {
        text.to_string()
    }
}

/// 統一レベル表記システム
pub fn level_critical(message: &str, no_color: bool) -> String {
    format!("[{}] {}", critical("CRITICAL", no_color), message)
}

pub fn level_high(message: &str, no_color: bool) -> String {
    format!("[{}] {}", fail("HIGH", no_color), message)
}

pub fn level_medium(message: &str, no_color: bool) -> String {
    format!("[{}] {}", warn("MEDIUM", no_color), message)
}

pub fn level_low(message: &str, no_color: bool) -> String {
    format!("[{}] {}", pass("LOW", no_color), message)
}

pub fn level_warning(message: &str, no_color: bool) -> String {
    format!("[{}] {}", warn("WARNING", no_color), message)
}

pub fn level_pass(message: &str, no_color: bool) -> String {
    format!("[{}] {}", pass("PASS", no_color), message)
}

pub fn level_conflict(message: &str, no_color: bool) -> String {
    format!("[{}] {}", fail("CONFLICT", no_color), message)
}

pub fn level_fail(message: &str, no_color: bool) -> String {
    format!("[{}] {}", fail("FAIL", no_color), message)
}

pub fn level_warn(message: &str, no_color: bool) -> String {
    format!("[{}] {}", warn("WARN", no_color), message)
}
