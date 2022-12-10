mod video_components;

use gloo_net::http::Request;
use video_components::{Video, VideoDetails, VideosList};
use yew::prelude::*;

async fn fetch_videos() -> Vec<Video> {
    Request::get("/tutorial/data.json")
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap()
}

#[function_component(App)]
fn app() -> Html {
    let videos: UseStateHandle<Vec<Video>> = use_state(Vec::new);
    {
        let videos = videos.clone();
        use_effect_with_deps(
            move |_| {
                wasm_bindgen_futures::spawn_local(async move {
                    let fetched_videos = fetch_videos().await;
                    videos.set(fetched_videos); // stateを変更
                });
            },
            (),
        ); // コンポーネントのコンストラクト時
    }

    let selected_video: UseStateHandle<Option<Video>> = use_state(|| None);

    let on_video_select = {
        let selected_video = selected_video.clone();
        Callback::from(move |video: Video| selected_video.set(Some(video)))
    };

    let details = selected_video.as_ref().map(|video| {
        html! {
            <VideoDetails video={video.clone()} />
        }
    });

    html! {
        <>
            <h1>{ "RustConf Explorer" }</h1>
            <div>
                <h3>{"Videos to watch"}</h3>
                <VideosList videos={(*videos).clone()} on_click={on_video_select.clone()} />
            </div>
            { for details }
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
