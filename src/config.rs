use std::fs;

use serde::{Deserialize, Serialize};

use crate::theme::ThemeType;

// 配置文件路径
static CONFIG_FILE: &str = ".config/auto-konsole/config.toml";

#[derive(Deserialize, Serialize, Debug)]
pub struct Config {
    dark: String,
    light: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            dark: "Breeze".to_string(),
            light: "Breeze".to_string(),
        }
    }
}

// 创建默认配置文件
fn create_default_config() -> Option<Config> {
    let config_path = home::home_dir()?.join(CONFIG_FILE);

    if let Some(parent) = config_path.parent() {
        let _ = fs::create_dir_all(parent);
    }

    let default_config = Config::default();

    if let Ok(toml_str) = toml::to_string(&default_config) {
        let _ = fs::write(&config_path, toml_str);
    }

    Some(default_config)
}

// 读取配置文件
pub fn load_config() -> Option<Config> {
    let config_path = home::home_dir()?.join(CONFIG_FILE);

    if !config_path.exists() {
        // 文件不存在， 创建默认文件
        if let Some(config) = create_default_config() {
            return Some(config);
        }
    }

    let config_content = fs::read_to_string(config_path).ok()?;
    toml::from_str(&config_content).ok()
}

// 设置config
pub fn set_config(typ: ThemeType, val: &str) {
    if let Some(mut config) = load_config() {
        match typ {
            ThemeType::Dark => config.dark = val.to_string(),
            ThemeType::Light => config.light = val.to_string(),
        }

        let config_path = home::home_dir().unwrap().join(CONFIG_FILE);

        if let Ok(toml_str) = toml::to_string(&config) {
            let _ = fs::write(&config_path, toml_str);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_and_load_config() {
        // 删除现有配置文件以测试创建默认配置
        let config_path = home::home_dir().unwrap().join(CONFIG_FILE);
        let _ = fs::remove_file(&config_path);

        // 加载配置， 应该创建默认配置文件
        let config = load_config().expect("Failed to load config");

        assert_eq!(config.dark, "Breeze");
        assert_eq!(config.light, "Breeze");

        // 修改配置并保存
        set_config(ThemeType::Dark, "NewDarkScheme");
        set_config(ThemeType::Light, "NewLightScheme");

        // 重新加载配置以验证更改
        let updated_config = load_config().expect("Failed to load updated config");

        assert_eq!(updated_config.dark, "NewDarkScheme");
        assert_eq!(updated_config.light, "NewLightScheme");

        // 删除配置文件
        let _ = fs::remove_file(&config_path);
    }
}
