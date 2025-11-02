use clap::Parser;

use crate::{colorscheme, config, theme, theme_listener};

#[derive(Parser, Debug)]
#[command(
    name = "auto-konsole",
    version,
    about = "Konsole 颜色方案自动切换工具",
    long_about = "Konsole 颜色方案自动切换工具"
)]
pub struct Args {
    /// 是否启动守护进程
    #[arg(long, default_value_t = false)]
    pub daemon: bool,

    /// 列出konsole支持的颜色方案
    #[arg(short, long, default_value_t = false)]
    pub list: bool,

    // 设置指定的dark主题颜色方案
    #[arg(long, required = false)]
    pub dark: Option<String>,

    // 设置指定的light主题颜色方案
    #[arg(long, required = false)]
    pub light: Option<String>,
}

impl Args {
    pub async fn command(&self) {
        // 启动守护进程
        if self.daemon {
            theme_listener::listen_theme_changes().await.unwrap();
            return;
        }

        // 列出颜色方案
        if self.list {
            let list = colorscheme::read::read_colorscheme();
            for scheme in list {
                println!("{}", scheme);
            }
            return;
        }

        // 设置dark主题颜色方案
        if let Some(scheme) = &self.dark {
            colorscheme::write::create_profile(theme::ThemeType::Dark, scheme).unwrap();
            config::set_config(theme::ThemeType::Dark, scheme);
            return;
        }

        // 设置light主题颜色方案
        if let Some(scheme) = &self.light {
            colorscheme::write::create_profile(theme::ThemeType::Light, scheme).unwrap();
            config::set_config(theme::ThemeType::Light, scheme);
            return;
        }
    }
}
