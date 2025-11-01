// 主题类型
pub enum ThemeType {
    Dark = 1,
    Light = 2,
}

impl TryFrom<u32> for ThemeType {
    type Error = ();

    fn try_from(value: u32) -> std::result::Result<Self, Self::Error> {
        match value {
            1 => Ok(ThemeType::Dark),
            2 => Ok(ThemeType::Light),
            _ => Err(()),
        }
    }
}

impl Default for ThemeType {
    fn default() -> Self {
        ThemeType::Light
    }
}
