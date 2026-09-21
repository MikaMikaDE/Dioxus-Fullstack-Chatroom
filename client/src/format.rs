use dioxus::prelude::*;
use serde_json::Map;

fn parse_as_object(s: &str) -> Result<Map<String, serde_json::Value>, String> {
    let parsed: serde_json::Value = serde_json::from_str(s).map_err(|e| e.to_string())?;
    parsed.as_object().cloned().ok_or("expected a JSON object".to_string())
}

pub type Formatter = Option<Callback<String,Element>>;
pub fn format_json(text: String) -> Element {
    match parse_as_object(&text) {
        Ok(object) => rsx! {
            ul { for (key, value) in object {
                li { "{key} : {value}" }
            } }
        },
        Err(_) => rsx! {
            p   { class: "warn", "Response was not a JSON object:" }
            pre { "{text}" }
        },
    }
}
#[component]
pub fn FormattedText(text: String, formatter: Formatter) -> Element {
    if let Some(f) = formatter { f(text)           }
    else                       { rsx! { "{text}" } }
}
