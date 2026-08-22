//! Minimal built-in internationalization for the desktop app.
//!
//! Supported locales: `en_US`, `zh_CN`, `zh_TW`. The active locale is kept in
//! a process-global cell (all UI code runs on the main thread) and is also
//! pushed into `woocraft::set_locale` so that woocraft's own component strings
//! follow the same selection.

use std::cell::Cell;

use once_cell::sync::Lazy;

pub const LOCALE_EN_US: &str = "en_US";
pub const LOCALE_ZH_CN: &str = "zh_CN";
pub const LOCALE_ZH_TW: &str = "zh_TW";

thread_local! {
    static CURRENT_LOCALE: Cell<&'static str> = const { Cell::new(LOCALE_EN_US) };
}

type Entry = (&'static str, &'static str, &'static str);

static TABLE: Lazy<Vec<Entry>> = Lazy::new(|| {
    vec![
        // ---- navigation ----
        ("Get Started", "开始使用", "開始使用"),
        ("Network logs", "网络日志", "網路日誌"),
        ("Settings", "设置", "設定"),
        ("Default Scope", "默认作用域", "預設範圍"),
        ("Controller port", "控制器端口", "控制器連接埠"),
        ("Click to copy", "点击复制", "點擊複製"),
        ("Click to see log", "点击查看日志", "點擊查看日誌"),
        // ---- get started ----
        ("Controlled TCP-over-WebSocket forwarding tunnel", "受控 TCP-over-WebSocket 转发隧道", "受控 TCP-over-WebSocket 轉發隧道"),
        ("Update", "更新", "更新"),
        ("Port...", "端口...", "連接埠..."),
        ("[ws|wss]://address...", "[ws|wss]://地址...", "[ws|wss]://位址..."),
        // ---- connections ----
        ("This is the default scope.", "这是默认作用域。", "這是預設範圍。"),
        ("Manually Controlled", "手动控制", "手動控制"),
        ("External Controlled", "外部控制", "外部控制"),
        ("This scope is controlled by you manually.", "该作用域由你手动控制。", "該範圍由你手動控制。"),
        ("This domain can control wsrx on your local network.", "该域名可以在你的局域网内控制 wsrx。", "該網域可以在你的區域網路內控制 wsrx。"),
        ("This domain requested controlling wsrx.", "该域名请求控制 wsrx。", "該網域請求控制 wsrx。"),
        ("Enjoy", "享受", "享受"),
        ("Accept", "接受", "接受"),
        ("Decline", "拒绝", "拒絕"),
        ("Remove", "移除", "移除"),
        ("Click to copy: ", "点击复制: ", "點擊複製: "),
        ("Click to close", "点击关闭", "點擊關閉"),
        // ---- settings ----
        ("Version and Updates", "版本与更新", "版本與更新"),
        ("Update available", "有可用更新", "有可用更新"),
        ("Running in system tray when closed", "关闭时在系统托盘运行", "關閉時在系統匣執行"),
        (" (not implemented yet) ", "（尚未实现）", "（尚未實作）"),
        ("Enabled", "已启用", "已啟用"),
        ("Disabled", "已禁用", "已停用"),
        ("Language / Locale", "语言 / 区域", "語言 / 地區"),
        ("English", "English", "English"),
        ("简体中文", "简体中文", "简体中文"),
        ("繁體中文", "繁體中文", "繁體中文"),
        ("Export network logs", "导出网络日志", "匯出網路日誌"),
        ("Export", "导出", "匯出"),
        ("Have problems? Find support here.", "遇到问题？在这里寻求支持。", "遇到問題？在這裡尋求支援。"),
        ("Support", "支持", "支援"),
        ("System information for bug reporting and debugging", "用于 Bug 上报与调试的系统信息", "用於 Bug 回報與除錯的系統資訊"),
        ("Copy", "复制", "複製"),
        ("Copied", "已复制", "已複製"),
        ("Please include the following information when reporting bugs or asking for help.", "上报 Bug 或寻求帮助时，请附上以下信息。", "回報 Bug 或尋求幫助時，請附上以下資訊。"),
    ]
});

/// Sets the active locale. `locale` must be one of the supported values.
pub fn set_locale(locale: &str) {
    let normalized = match locale {
        LOCALE_ZH_CN | "zh-hans" | "zh-CN" => LOCALE_ZH_CN,
        LOCALE_ZH_TW | "zh-hant" | "zh-TW" => LOCALE_ZH_TW,
        _ => LOCALE_EN_US,
    };
    CURRENT_LOCALE.with(|cell| cell.set(normalized));
    // Sync woocraft's own component locale.
    match normalized {
        LOCALE_ZH_CN => woocraft::set_locale("zh-hans"),
        LOCALE_ZH_TW => woocraft::set_locale("zh-hant"),
        _ => woocraft::set_locale("en-us"),
    }
}

/// Returns the currently active locale, e.g. `"en_US"`.
pub fn locale() -> &'static str {
    CURRENT_LOCALE.with(|cell| cell.get())
}

/// Translates a key into the current locale, falling back to the English
/// source string when the key is not present in the table.
pub fn t<'a>(key: &'a str) -> &'a str {
    let locale = locale();
    for (en, zh_cn, zh_tw) in TABLE.iter() {
        if *en == key {
            return match locale {
                LOCALE_ZH_CN => zh_cn,
                LOCALE_ZH_TW => zh_tw,
                _ => en,
            };
        }
    }
    key
}
