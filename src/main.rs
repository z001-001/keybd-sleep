#![windows_subsystem = "windows"]

extern crate keybd_sleep;

use keybd_sleep::Config;
use std::env;
use std::process;
use windows::{
    Win32::UI::WindowsAndMessaging::{
        IDYES, MB_ICONERROR, MB_ICONQUESTION, MB_OK, MB_YESNO, MessageBoxW,
    },
    core::{PCWSTR, w},
};

// 設定不良のFallback用
const APP_NAME: &str = "KeybdSleep";

fn main() {
    let app_name = match option_env!("CARGO_PKG_DESCRIPTION") {
        Some(pkg_desc) if !pkg_desc.is_empty() => pkg_desc,
        _ => APP_NAME,
    };

    let config = Config::new(env::args()).unwrap_or_else(|mut err| {
        err.push('\0');
        let text: Vec<u16> = err.encode_utf16().collect();
        let caption: Vec<u16> = format!("{}\0", app_name).encode_utf16().collect();

        unsafe {
            MessageBoxW(
                None,
                PCWSTR(text.as_ptr()),
                PCWSTR(caption.as_ptr()),
                MB_OK | MB_ICONERROR,
            );
        }
        // エラー終了
        process::exit(1);
    });

    let do_exec = if config.opt_quiet {
        true
    } else {
        let text = w!("スリープしますか？");
        let caption: Vec<u16> = format!("{}\0", app_name).encode_utf16().collect();

        let result = unsafe {
            MessageBoxW(
                None,
                text,
                PCWSTR(caption.as_ptr()),
                MB_YESNO | MB_ICONQUESTION,
            )
        };
        result == IDYES
    };

    // Yesの時のみ、スリープする
    if do_exec {
        keybd_sleep::run(config);
    }
}
