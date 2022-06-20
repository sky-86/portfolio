use yew::prelude::*;

#[function_component(Projects)]
pub fn projects() -> Html {
    html! {
        <div class="container">
            <h1>{ "Projects" }</h1>
            <ul class="projects">
                <li><a href="./sorting_visual/">{"Sorting Visualizer"}</a></li>
                <li><a href="./media_player/">{"Media Player"}</a></li>
                <li><a href="./calculator/">{"Calculator"}</a></li>
                <li><a href="./map/">{"Map"}</a></li>
                <li><a href="./weather/">{"Weather"}</a></li>
             </ul>
        </div>
    }
}
