use yew::prelude::*;

#[function_component(Projects)]
pub fn projects() -> Html {
    html! {
        <div>
            <h1>{ "Projects" }</h1>
            <ul>
                <li><a href="./sorting_visual/">{"Sorting Visualizer"}</a></li>
                <li><a href="./media_player/">{"Media Player"}</a></li>
                <li><a href="./portfolio">{"This Website"}</a></li>
             </ul>
        </div>
    }
}
