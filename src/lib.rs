use std::path::Path;
use std::thread::sleep;
use std::time::Duration;
use windows::Win32::UI::Input::KeyboardAndMouse::{
    KEYBD_EVENT_FLAGS, KEYEVENTF_KEYUP, VK_LWIN, VK_S, VK_U, VK_X, keybd_event,
};

const DEFAULT_WAIT_MILLI_SEC: u64 = 166;

pub struct Config {
    pub wait_milli_sec: u64,
    pub opt_no_exec: bool,
    pub opt_quiet: bool,
}

impl Config {
    pub fn new(args: std::env::Args) -> Result<Config, String> {
        let mut wait_milli_sec = DEFAULT_WAIT_MILLI_SEC;
        let mut opt_no_exec = false;
        let mut opt_quiet = false;
        let mut arg0: String;
        let mut exe_name = "";

        for (index, arg) in args.enumerate() {
            if index == 0 {
                arg0 = arg;
                let path = Path::new(&arg0);
                exe_name = path.file_name().unwrap().to_str().unwrap();
            } else if arg.eq_ignore_ascii_case("/Q") {
                // 確認ダイアログを出さない
                opt_quiet = true;
            } else if arg.eq_ignore_ascii_case("/N") {
                // スリープの実施をしない
                opt_no_exec = true;
            } else if let Ok(num) = arg.parse() {
                // キーイベント送信間のウェイト(ミリ秒)
                wait_milli_sec = num;
            } else {
                let text = format!(
                    "Unknown Parameter: {}\r\n\
                    \r\n\
                    Usage: {} [/N] [/Q] [milliseconds]",
                    arg, exe_name
                );

                // エラー
                return Err(text);
            }
        }

        Ok(Config {
            wait_milli_sec,
            opt_no_exec,
            opt_quiet,
        })
    }
}

pub fn run(config: Config) {
    unsafe {
        // [Win] + [x] キー送信
        keybd_event(VK_LWIN.0 as u8, 0, KEYBD_EVENT_FLAGS(0), 0);
        keybd_event(VK_X.0 as u8, 0, KEYBD_EVENT_FLAGS(0), 0);
        keybd_event(VK_X.0 as u8, 0, KEYEVENTF_KEYUP, 0);
        keybd_event(VK_LWIN.0 as u8, 0, KEYEVENTF_KEYUP, 0);
    }

    sleep(Duration::from_millis(config.wait_milli_sec));

    // [u] キー送信
    unsafe {
        keybd_event(VK_U.0 as u8, 0, KEYBD_EVENT_FLAGS(0), 0);
        keybd_event(VK_U.0 as u8, 0, KEYEVENTF_KEYUP, 0);
    }

    sleep(Duration::from_millis(config.wait_milli_sec));

    if config.opt_no_exec == false {
        // [s] キー送信
        unsafe {
            keybd_event(VK_S.0 as u8, 0, KEYBD_EVENT_FLAGS(0), 0);
            keybd_event(VK_S.0 as u8, 0, KEYEVENTF_KEYUP, 0);
        }
        sleep(Duration::from_millis(66));
    }
}
