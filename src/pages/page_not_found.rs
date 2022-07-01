use yew::prelude::*;

#[function_component(NotFound)]
pub fn page_not_found() -> Html {
    html! {
        <>
        <div>
            <h1>{ "404" }</h1>
            <p>{"Error: could not find route"}</p>
        </div>
        </>
    }
}
