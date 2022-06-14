use yew::prelude::*;
use yew_router::prelude::*;

use crate::routes::*;

#[function_component(Home)]
pub fn home() -> Html {
    let history = use_history().unwrap();
    let onclick = Callback::once(move |_| history.push(Route::Home));
    
    html! {
        <div>
            <h1>{ "Home" }</h1>
            <button {onclick} >{ "Go Home" }</button>
        </div>
    }
}
