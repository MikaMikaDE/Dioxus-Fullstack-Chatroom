use dioxus::prelude::*;
use crate::{
    API_TOKEN,
    format::{Formatter,FormattedText},
};

pub async fn get_request(src: String) -> Result<String, String> {
    gloo_net::http::Request::get(&src)
        .header("Authorization", &format!("Bearer {API_TOKEN}"))
        .send().await.map_err(|e| e.to_string())?
        .text().await.map_err(|e| e.to_string())
}
pub async fn post_request(src: String, body: String) -> Result<String, String> {
    gloo_net::http::Request::post(&src)
        .header("Authorization", &format!("Bearer {API_TOKEN}"))
        .body( body ).map_err(|e| e.to_string())?
        .send().await.map_err(|e| e.to_string())?
        .text().await.map_err(|e| e.to_string())
}

#[component]
fn SpinnerGif(show: bool) -> Element {
    const SPINNER_SRC: &str = "https://loading.io/assets/mod/spinner/spinner/sample.gif";
    rsx! {
        if show { img { src: SPINNER_SRC } }
        else    { ""                       }
    }
}

#[component]
pub fn GetResponseButton(
    href       :String,
    button_text:String,
    #[props(default      )] formatter:Formatter,
) -> Element {
    let mut is_loading: Signal<bool>           = use_signal(|| false);
    let mut data      : Signal<Option<String>> = use_signal(|| None );

    let load_data = move |_| {
        let href = href.clone();
        spawn(async move {
            is_loading.set(true);
            let response = get_request(href).await.unwrap_or_else(|e| e);
            data.set(Some(response));
            is_loading.set(false);
        });
    };


    rsx! {
        button     { onclick: load_data, class: "box", "{button_text}" }
        SpinnerGif { show: is_loading()                                }
        //response box
        if let Some(text) = data() {
            div {
                if let Some(f) = formatter {   FormattedText { text, formatter: Some(f) }  } 
                else                       {   p { "{text}" }                              }
                button { onclick: move |_| data.set(None), class: "box", "Clear" }
            }
        }
    }
}
