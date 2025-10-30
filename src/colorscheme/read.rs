use std::{env::home_dir, fs, path::PathBuf};

// Konsole colorscheme目录
static COLORSHEME_DIR: &str = ".local/share/konsole";

// 读取colorscheme列表
pub fn read_colorscheme() -> Vec<String> {
    let colorscheme_path = home_dir().unwrap().join(COLORSHEME_DIR);

    fs::read_dir(colorscheme_path)
        .unwrap()
        .filter_map(Result::ok)
        .map(|e| e.path())
        .filter(|p| p.is_file() && p.extension().and_then(|s| s.to_str()) == Some("colorscheme"))
        .filter_map(|f| {
            f.file_stem()
                .and_then(|s| s.to_str())
                .map(|s| s.to_string())
        })
        .collect::<Vec<String>>()
}

mod tests {

    #[test]
    fn test_read_colorscheme() {
        let list = super::read_colorscheme();
        // 打印读取到的文件名
        println!("Found {} colorschemes:", list.len());
        for name in &list {
            println!("  {}", name);
        }

        // 简单断言，至少应该能运行不 panic
        assert!(!list.is_empty());
    }
}
