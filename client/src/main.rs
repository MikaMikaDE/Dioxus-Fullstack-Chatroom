use dioxus::prelude::*;
mod outbound; use outbound::{
    GetResponseButton,
    get_request,
    post_request
};
mod format  ;

    static MAIN_CSS       :Asset = asset!("/assets/style.css");
    const  SERVER         :&str  = "http://localhost:3001"; //change to "" for ngrok
    const  CHATROOM_FOLDER:&str  = "messages";
    const  CHATROOM_FILE  :&str  = "chats"   ;
pub const  API_TOKEN      :&str  = "token"   ;

#[component]
fn Line(text:String) -> Element {
    rsx!{
        p{"{text}"}
        hr{} 
    }
}

#[component]
fn Messages(messages: Resource<String>) -> Element {
    match &*messages.read() {
        Some(text) => rsx! {
            for message in text.split("\n") {
                p { "{message}" }
            }
        },
        None => rsx! { p { "Loading..." } },
    }
}

#[component]
fn SendMessage(messages: Resource<String>) -> Element {
    let mut message:Signal<Option<String>> = use_signal(||None);
    let send_message = move|_| {
        spawn(async move {
            let text = message();
            if  text.is_none() { return }
            let href = format!("{SERVER}/data/{CHATROOM_FOLDER}/{CHATROOM_FILE}");
            post_request(href, text.unwrap()).await.ok();
            message.set(None);
            messages.restart();
        });
    };
    let message_as_text = message().unwrap_or("".to_string());
    rsx!{
        input{
            oninput    :move|event|message.set(Some(event.value())),
            value      :"{message_as_text}",
            placeholder:"enter a message..."
        }
        button{
            onclick:send_message,
            "Send"
        }
    }
}

#[component]
fn App() -> Element {
    let messages = use_resource(|| async {
        let href = format!("{SERVER}/data/{CHATROOM_FOLDER}/{CHATROOM_FILE}");
        get_request(href).await.unwrap_or_default()
    });

    rsx! {
        document::Stylesheet { href: MAIN_CSS }
        GetResponseButton { href: &format!("{SERVER}/ping"), button_text: "Ping" }
        hr {}
        Messages { messages }
        hr {}
        SendMessage { messages }
    }
}

fn main() {
    launch(App);
}
