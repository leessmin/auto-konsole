use futures::StreamExt;
use zbus::{
    Connection, Result, proxy,
    zvariant::{OwnedValue, Value},
};
use crate::theme::ThemeType;

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

pub async fn listen_theme_changes() -> Result<()> {
    let connection = Connection::session().await?;

    let settings = ThemeListenerProxy::new(&connection).await?;

    // let value = settings
    //     .read("org.freedesktop.appearance", "color-scheme")
    //     .await?;

    let mut stream = settings.receive_setting_changed().await?;

    while let Some(signal) = stream.next().await {
        if let Ok(args) = signal.args() {
            if *args.namespace() != "org.freedesktop.appearance" || *args.key() != "color-scheme" {
                continue;
            }

            println!("Theme changed: key:{}, value: {}", args.key(), args.value());

            // 主题类型 1 dart 2 light
            let val: u32 = args.value().clone().try_into().unwrap_or_default();
            let typ = ThemeType::try_from(val).unwrap_or_default();

            // TODO: 未完成 变更主题
        }
    }

    Ok(())
}
