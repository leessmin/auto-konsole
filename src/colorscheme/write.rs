use std::{
    fs,
    io::{self, Write},
    path::Path,
};

use home::home_dir;
use zbus::{Connection, Proxy, Result};

use crate::{
    colorscheme::{self, path::KONSOLERC_PATH},
    theme::ThemeType,
};

// 使用dbus设置konsole会话主题
async fn set_konsole_colorscheme_dbus(typ: &ThemeType) -> Result<()> {
    let connection = Connection::session().await?;

    let profile = match typ {
        ThemeType::Dark => "Dark",
        ThemeType::Light => "Light",
    };

    let mut sys = sysinfo::System::new_all();
    sys.refresh_processes(sysinfo::ProcessesToUpdate::All, true);

    for (pid, process) in sys.processes() {
        let process_name = process.name().to_str().unwrap_or_default();
        if process_name.to_lowercase() != "konsole" {
            continue;
        }
        let proxy = Proxy::new(
            &connection,
            format!("org.kde.konsole-{}", pid),
            "/Sessions/1",
            "org.kde.konsole.Session",
        )
        .await?;

        proxy.call_method("setProfile", &profile).await?;
    }

    Ok(())
}

// 替换value, 并写入文件
fn write_replace_file(content: &str, path: &Path, key: &str, value: &str) -> io::Result<()> {
    let new_content = content
        .lines()
        .map(|line| {
            if line
                .chars()
                .filter(|c| !c.is_whitespace())
                .collect::<String>()
                .starts_with(key)
            {
                format!("{}{}", key, value)
            } else {
                line.to_string()
            }
        })
        .collect::<Vec<String>>()
        .join("\n");

    let mut file = fs::File::create(path)?;
    file.write_all(new_content.as_bytes())?;

    Ok(())
}

// 写入konsolerc文件 更换profile文件
pub async fn set_konsolerc(typ: ThemeType) -> io::Result<()> {
    let _ = set_konsole_colorscheme_dbus(&typ).await;

    let konsolerc_path = home_dir().unwrap().join(KONSOLERC_PATH);

    let content = fs::read_to_string(&konsolerc_path)?;

    write_replace_file(
        &content,
        &konsolerc_path,
        "DefaultProfile=",
        match typ {
            ThemeType::Dark => "Dark.profile",
            ThemeType::Light => "Light.profile",
        },
    )?;

    Ok(())
}

// 创建自定义profile
// 存在则创建，不存在则修改
pub fn create_profile(typ: ThemeType, colorscheme: &str) -> io::Result<()> {
    let custom_profile_path = match typ {
        ThemeType::Dark => home_dir()
            .unwrap()
            .join(colorscheme::path::DARK_PROFILE_PATH),
        ThemeType::Light => home_dir()
            .unwrap()
            .join(colorscheme::path::LIGHT_PROFILE_PATH),
    };

    let custom_path = Path::new(&custom_profile_path);
    let content = if custom_path.exists() {
        // 存在配置
        fs::read_to_string(&custom_profile_path)?
    } else {
        // 不存在配置
        format!(
            r#"[Appearance]
ColorScheme=Breeze

[General]
Command=/bin/bash
Name={}
Parent=FALLBACK/
"#,
            match typ {
                ThemeType::Dark => "Dark",
                ThemeType::Light => "Light",
            }
        )
    };

    let content = content
        .lines()
        .map(|line| {
            if line
                .chars()
                .filter(|c| !c.is_whitespace())
                .collect::<String>()
                .starts_with("Name=")
            {
                format!(
                    "Name={}",
                    match typ {
                        ThemeType::Dark => "Dark",
                        ThemeType::Light => "Light",
                    }
                )
            } else {
                line.to_string()
            }
        })
        .collect::<Vec<String>>()
        .join("\n");

    write_replace_file(&content, &custom_profile_path, "ColorScheme=", colorscheme)?;

    Ok(())
}

mod test {
    use std::fs;

    use home::home_dir;
    use zbus::conn;

    use crate::{
        colorscheme::{
            self,
            path::KONSOLERC_PATH,
            write::{create_profile, set_konsole_colorscheme_dbus, set_konsolerc},
        },
        theme::ThemeType,
    };

    #[tokio::test]
    async fn test_set_konsolerc_dark() {
        set_konsolerc(ThemeType::Dark).await.unwrap();
        let konsolerc_path = home_dir().unwrap().join(KONSOLERC_PATH);

        let content = fs::read_to_string(&konsolerc_path).unwrap();

        let default_line = content
            .lines()
            .find(|line| line.starts_with("DefaultProfile="))
            .unwrap_or("");

        assert_eq!(default_line, "DefaultProfile=Dark.profile");
    }

    #[tokio::test]
    async fn test_set_konsolerc_light() {
        set_konsolerc(ThemeType::Light).await.unwrap();
        let konsolerc_path = home_dir().unwrap().join(KONSOLERC_PATH);

        let content = fs::read_to_string(&konsolerc_path).unwrap();

        let default_line = content
            .lines()
            .find(|line| line.starts_with("DefaultProfile="))
            .unwrap_or("");

        assert_eq!(default_line, "DefaultProfile=Light.profile");
    }

    #[test]
    fn test_create_profile_dark() {
        let colorscheme = "BreezeDark";
        create_profile(ThemeType::Dark, colorscheme).unwrap();

        let profile_path = home_dir()
            .unwrap()
            .join(colorscheme::path::DARK_PROFILE_PATH);

        let content = fs::read_to_string(&profile_path).unwrap();

        let default_line = content
            .lines()
            .find(|line| line.starts_with("ColorScheme="))
            .unwrap_or("");

        assert_eq!(default_line, format!("ColorScheme={}", colorscheme));
    }

    #[test]
    fn test_create_profile_light() {
        let colorscheme = "BreezeLight";
        create_profile(ThemeType::Light, colorscheme).unwrap();

        let profile_path = home_dir()
            .unwrap()
            .join(colorscheme::path::LIGHT_PROFILE_PATH);

        let content = fs::read_to_string(&profile_path).unwrap();

        let default_line = content
            .lines()
            .find(|line| line.starts_with("ColorScheme="))
            .unwrap_or("");

        assert_eq!(default_line, format!("ColorScheme={}", colorscheme));
    }

    #[tokio::test]
    async fn test_set_konsolerc_dbus_light() {
        let _ = set_konsole_colorscheme_dbus(&ThemeType::Light)
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn test_set_konsolerc_dbus_dark() {
        let _ = set_konsole_colorscheme_dbus(&ThemeType::Dark)
            .await
            .unwrap();
    }
}
