use yew::prelude::*;

#[function_component(Home)]
pub fn home() -> Html {
    html! {
        <>
        <div>
            <h1>{ "Home" }</h1>
            <p> {"Hi, welcome to my portfolio!"}</p>
        </div>
        </>
    }
}
