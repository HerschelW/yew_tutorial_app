pub mod videos;

use yew::prelude::*;
// import videos struct
use crate::videos::Video;
// use crate::videos::VideosList;

// import VideosList function component
use crate::videos::VideosList;

#[function_component(App)]
fn app() -> Html {
    // create a vector of videos
    let videos = vec![
        Video {
            id: 1,
            title: "Building and breaking things".to_string(),
            speaker: "John Doe".to_string(),
            url: "https://youtu.be/PsaFVLr8t4E".to_string(),
        },
        Video {
            id: 2,
            title: "The development process".to_string(),
            speaker: "Jane Smith".to_string(),
            url: "https://youtu.be/PsaFVLr8t4E".to_string(),
        },
        Video {
            id: 3,
            title: "The Web 7.0".to_string(),
            speaker: "Matt Miller".to_string(),
            url: "https://youtu.be/PsaFVLr8t4E".to_string(),
        },
        Video {
            id: 4,
            title: "Mouseless development".to_string(),
            speaker: "Tom Jerry".to_string(),
            url: "https://youtu.be/PsaFVLr8t4E".to_string(),
        },
    ];

    html! {
        <>
            <h1>{ "RustConf Explorer" }</h1>
            <div>
                <h3>{"Videos to watch"}</h3>
                     <VideosList videos={videos} on_click={on_video_select.clone()} />
                            </div>
                    { for details }
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
