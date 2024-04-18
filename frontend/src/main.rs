use anyhow::Error;
use base64::engine::general_purpose::STANDARD as BASE64;
use base64::engine::Engine as _;
use macros::Json;
use serde::{Deserialize, Serialize};
use shared_types::server::ImageInformation;
use yew::prelude::*;
use yew_websocket::websocket::{WebSocketService, WebSocketStatus, WebSocketTask};

mod macros;

enum Msg {
    FetchImage,
    UpdateImage(Vec<u8>),
    WsAction(WsAction),
    WsReady(Result<WsResponse, Error>),
}

pub enum WsAction {
    Connect,
    SendData,
    Disconnect,
    Lost,
}

impl From<WsAction> for Msg {
    fn from(action: WsAction) -> Self {
        Msg::WsAction(action)
    }
}

const PROTOCOL: &str = "";
const BASE_URL: &str = "192.168.8.185:3000/api/v1";

#[derive(Serialize, Debug)]
struct WsRequest {
    value: u32,
}

#[derive(Deserialize, Debug)]
pub struct WsResponse {
    value: u32,
}

async fn fetch_image() -> Vec<u8> {
    let client = reqwest::Client::new();

    let response: ImageInformation = client
        .get(format!(
            "http{}://{}/frontend/tracking-image-data",
            PROTOCOL, BASE_URL
        ))
        .header("Accept", "application/json")
        //.header("Access-Control-Allow-Origin", "*")
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap();

    let response = client
        .get(format!(
            "http{}://{}/frontend/image-tracked/{}",
            PROTOCOL, BASE_URL, response.image_id
        ))
        .header("Accept", "image/jpeg")
        //.header("Access-Control-Allow-Origin", "*")
        .send()
        .await
        .unwrap();

    if !response.status().is_success() {
        return Vec::new();
    }

    if let Some(content_type) = response.headers().get("content-type") {
        if !content_type.to_str().unwrap().starts_with("image/jpeg") {
            return Vec::new();
        }
    } else {
        return Vec::new();
    }

    let bytes = response.bytes().await.unwrap();

    bytes.to_vec()
}

struct App {
    image: Vec<u8>,
    ws: Option<WebSocketTask>,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            image: Vec::new(),
            ws: None,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::FetchImage => {
                ctx.link().send_future(async {
                    let image = fetch_image().await;
                    Msg::UpdateImage(image)
                });
                false
            }
            Msg::UpdateImage(image) => {
                ctx.link().send_message(Msg::FetchImage);
                self.image = image;
                true
            }
            Msg::WsAction(action) => match action {
                WsAction::Connect => {
                    let callback = ctx.link().callback(|Json(data)| Msg::WsReady(data));
                    let notification = ctx.link().batch_callback(|status| match status {
                        WebSocketStatus::Opened => None,
                        WebSocketStatus::Closed | WebSocketStatus::Error => {
                            Some(WsAction::Lost.into())
                        }
                    });
                    let task = WebSocketService::connect(
                        format!("ws{}://{}/websocket", PROTOCOL, BASE_URL).as_str(),
                        callback,
                        notification,
                    )
                    .unwrap();
                    self.ws = Some(task);
                    true
                }
                WsAction::SendData => {
                    let request = WsRequest { value: 321 };
                    self.ws
                        .as_mut()
                        .unwrap()
                        .send(serde_json::to_string(&request).unwrap());
                    false
                }
                WsAction::Disconnect => {
                    self.ws.take();
                    true
                }
                WsAction::Lost => {
                    self.ws = None;
                    true
                }
            },

            Msg::WsReady(response) => {
                let data = response.map(|data| data.value).ok();
                if data.is_some() {
                    ctx.link().send_message(Msg::FetchImage);
                }
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div>
                <img id="exampleImage" src={format!("data:image/jpeg;base64,{}", BASE64.encode(self.image.clone()).as_str())} alt="Example Image" />
                <br />
                <button onclick={ctx.link().batch_callback(|_| vec![Msg::WsAction(WsAction::Connect)])}>{ "Connect" }</button>
                <button onclick={ctx.link().batch_callback(|_| vec![Msg::FetchImage])}>{ "Refresh" }</button>
            </div>
        }
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
