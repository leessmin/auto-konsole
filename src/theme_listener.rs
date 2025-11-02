use crate::{colorscheme, theme::ThemeType};
use futures::StreamExt;
use zbus::{
    Connection, Result, proxy,
    zvariant::{OwnedValue, Value},
};

#[proxy(
    interface = "org.freedesktop.portal.Settings",
    default_service = "org.freedesktop.portal.Desktop",
    default_path = "/org/freedesktop/portal/desktop"
)]
trait ThemeListener {
    fn read(&self, namespace: &str, key: &str) -> Result<OwnedValue>;

    #[zbus(signal)]
    fn setting_changed(&self, namespace: &str, key: &str, value: Value<'_>);
}

// OwnedValue解包转u32
fn extract_u32(value: OwnedValue) -> Option<u32> {
    // 尝试直接 downcast
    if let Ok(v) = value.downcast_ref::<u32>() {
        return Some(v);
    }

    None
}

pub async fn listen_theme_changes() -> Result<()> {
    let connection = Connection::session().await?;

    let settings = ThemeListenerProxy::new(&connection).await?;

    // 初始化主题
    {
        let value = settings
            .read("org.freedesktop.appearance", "color-scheme")
            .await?;

        let val = extract_u32(value).unwrap_or_default();
        let typ = ThemeType::try_from(val).unwrap_or_default();

        // 变更主题
        let _ = colorscheme::write::set_konsolerc(typ);
    }

    let mut stream = settings.receive_setting_changed().await?;

    while let Some(signal) = stream.next().await {
        if let Ok(args) = signal.args() {
            if *args.namespace() != "org.freedesktop.appearance" || *args.key() != "color-scheme" {
                continue;
            }

            // 主题类型 1 dart 2 light
            let val: u32 = args.value().clone().try_into().unwrap_or_default();
            let typ = ThemeType::try_from(val).unwrap_or_default();

            // 变更主题
            let _ = colorscheme::write::set_konsolerc(typ);
        }
    }

    Ok(())
}
