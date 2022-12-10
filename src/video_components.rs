use yew::prelude::*;
use serde::Deserialize;

#[derive(PartialEq, Eq, Clone, Deserialize)]
pub struct Video {
    pub id: usize,
    pub title: String,
    pub speaker: String,
    pub url: String,
}

#[derive(Properties, PartialEq)]
pub struct VideosListProps {
    pub videos: Vec<Video>,
    pub on_click: Callback<Video>,
}

#[function_component(VideosList)]
pub fn videos_list(VideosListProps { videos, on_click }: &VideosListProps) -> Html {
    videos
        .iter()
        .map(|video| {
            let on_video_select: Callback<MouseEvent> = {
                let on_click = on_click.clone();
                let video = video.clone();
                Callback::from(move |_| {
                    on_click.emit(video.clone())
                })
            };

            html! {
               <p key={video.id} onclick={on_video_select}>{format!("{}: {}", video.speaker, video.title)}</p>
            }
        })
        .collect::<Html>()
}

#[derive(Properties, PartialEq, Eq)]
pub struct VideosDetailsProps {
    pub video: Video,
}

#[function_component(VideoDetails)]
pub fn video_details(VideosDetailsProps { video }: &VideosDetailsProps) -> Html {
    html! {
        <div>
            <h3>{ format!("{}", video.title) }</h3>
            <img src="https://via.placeholder.com/640x360.png?text=Video+Player+Placeholder" alt="video thumbnail" />
        </div>
    }
}
