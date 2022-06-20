use yew::prelude::*;

#[function_component(Projects)]
pub fn projects() -> Html {
    html! {
        <div class="container">
            <h1>{ "Projects" }</h1>
            <ul class="projects">
                <li><a href="/skyler/projects/sorting_visual/">{"Sorting Visualizer"}</a></li>
                <li><a href="/skyler/projects/media_player/">{"Media Player"}</a></li>
                <li><a href="/skyler/projects/calculator/">{"Calculator"}</a></li>
                <li><a href="/skyler/projects/map/">{"Map"}</a></li>
                <li><a href="/skyler/projects/weather/">{"Weather"}</a></li>
             </ul>
        </div>
    }
}
