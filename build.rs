
fn main() {
    if std::env::var("CARGO_CFG_TARGET_OS").unwrap() == "windows" {
        let mut res = winres::WindowsResource::new();
        
        // アイコンの設定（プロジェクトルートに icon.ico がある場合）
        // res.set_icon("icon.ico");
        
        // メタデータの設定
        res.set("ProductName", "KeybdSleep");
        res.set("CompanyName", "");
        res.set("LegalCopyright", "Copyright © 2026");
        res.set("FileDescription", "KeybdSleep");
        
        // 実行ファイル名を個別に指定する場合（前述の [[bin]] と合わせる）
        res.set("InternalName", "KeybdSleep.exe");
        res.set("OriginalFilename", "KeybdSleep.exe");

        res.compile().unwrap();
    }
}
