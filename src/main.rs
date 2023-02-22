pub mod videos;
use std::vec;

use crate::videos::Video;
use crate::videos::VideoDetails;
use crate::videos::VideosList;
use gloo_net::http::Request;
use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    let videos = use_state(|| vec![]);

    {
        let videos = videos.clone();
        use_effect_with_deps(
            move |_| {
                let videos = videos.clone();
                wasm_bindgen_futures::spawn_local(async move {
                    let fetched_videos: Vec<Video> = Request::get("/data.json")
                        .send()
                        .await
                        .unwrap()
                        .json()
                        .await
                        .unwrap();
                    videos.set(fetched_videos);
                    // print videos
                    // convert videos to string literal

                    let video_string = String::new();

                    // print video string
                    println!("{}", video_string);
                });
                || ()
            },
            (),
        );
    }

    let selected_video = use_state(|| None);
    let on_video_select = {
        let selected_video = selected_video.clone();
        Callback::from(move |video: Video| {
            selected_video.set(Some(video));
        })
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
              <VideosList videos={(*videos).clone()} on_click={on_video_select} />
            </div>
            {for details}
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
