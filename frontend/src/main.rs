use base64::engine::general_purpose::STANDARD as BASE64;
use base64::engine::Engine as _;
use shared_types::server::ImageInformation;
use yew::prelude::*;

enum Msg {
    FetchImage,
    UpdateImage(Vec<u8>),
}

const PROTOCOL: &str = "";
const BASE_URL: &str = "127.0.0.1:3000/api/v1";

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
            PROTOCOL, BASE_URL, response.image_props.img_id
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
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self { image: Vec::new() }
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
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div>
                <img id="exampleImage" src={format!("data:image/jpeg;base64,{}", BASE64.encode(self.image.clone()).as_str())} alt="Example Image" />
                <br />
                <button onclick={ctx.link().batch_callback(|_| vec![Msg::FetchImage])}>{ "Refresh" }</button>
            </div>
        }
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
