use std::env;
use std::path::Path;
use std::process;
use std::thread::sleep;
use std::time::Duration;
use windows::{
    Win32::UI::WindowsAndMessaging::{IDYES, MB_ICONERROR, MB_ICONQUESTION, MB_OK, MB_YESNO, MessageBoxW}, core::{PCWSTR, w},
};
use windows::Win32::UI::Input::KeyboardAndMouse::{
    keybd_event,
    KEYBD_EVENT_FLAGS,
    KEYEVENTF_KEYUP,
    VK_LWIN,
    VK_S,
    VK_U,
    VK_X
};

const DEFAULT_WAIT_MILLI_SEC: u64 = 166;

fn main() {
    let mut wait_milli_sec = DEFAULT_WAIT_MILLI_SEC;
    let mut opt_no_exec = false;
    let mut opt_quiet = false;
    let mut arg0: String;
    let mut exe_name = "";

    for (index, arg) in env::args().enumerate() {
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
            let text: Vec<u16> = 
                format!("Unknown Parameter: {}\r\n\r\nUsage: {} [/N] [/Q] [milliseconds]\0",
                    arg, exe_name)
                .encode_utf16().collect();
            let caption: Vec<u16> = 
                format!("{}\0", exe_name).encode_utf16().collect();

            unsafe {
                MessageBoxW(None,
                    PCWSTR(text.as_ptr()),
                    PCWSTR(caption.as_ptr()),
                    MB_OK | MB_ICONERROR);
            }
            // エラー終了
            process::exit(1);
        }
    }

    let do_exec = if opt_quiet {
        true
    } else {
        let text = w!("スリープしますか？");
        let caption: Vec<u16> =
            format!("{}\0", exe_name).encode_utf16().collect();
        
        let result = 
            unsafe {
                MessageBoxW(None,
                text,
                PCWSTR(caption.as_ptr()),
                MB_YESNO | MB_ICONQUESTION)
            };
        result == IDYES
    };

    // Yesの時のみ、スリープする
    if do_exec {
        unsafe {
            // [Win] + [x] キー送信
            keybd_event(VK_LWIN.0 as u8, 0, KEYBD_EVENT_FLAGS(0), 0);
            keybd_event(VK_X.0 as u8, 0, KEYBD_EVENT_FLAGS(0), 0);
            keybd_event(VK_X.0 as u8, 0, KEYEVENTF_KEYUP, 0);
            keybd_event(VK_LWIN.0 as u8, 0, KEYEVENTF_KEYUP, 0);
        }

        sleep(Duration::from_millis(wait_milli_sec));

        // [u] キー送信
        unsafe {
            keybd_event(VK_U.0 as u8, 0, KEYBD_EVENT_FLAGS(0), 0);
            keybd_event(VK_U.0 as u8, 0, KEYEVENTF_KEYUP, 0);
        }

        sleep(Duration::from_millis(wait_milli_sec));

        if opt_no_exec == false {
            // [s] キー送信
            unsafe {
                keybd_event(VK_S.0 as u8, 0, KEYBD_EVENT_FLAGS(0), 0);
                keybd_event(VK_S.0 as u8, 0, KEYEVENTF_KEYUP, 0);
            }
            sleep(Duration::from_millis(66));
        }
    }

}
