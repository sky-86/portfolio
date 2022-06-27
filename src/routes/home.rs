use yew::prelude::*;
use yew_router::prelude::*;

use crate::routes::*;




#[function_component(Home)]
pub fn home() -> Html {
    html! {
        <>
        <div class="container" >
            <h1>{ "Home" }</h1>
            <p>{"Hi, welcome to my portfolio!"}</p>
        </div>
        <div class="home">
            <ul>
                
                <li style="--i:4;"><Link<Route> to={Route::Home}>{"Home"}</Link<Route>></li>
                <li style="--i:3;"><Link<Route> to={Route::Problems}>{"Problems"}</Link<Route>></li>
                <li style="--i:2;"><Link<Route> to={Route::Projects}>{"Projects"}</Link<Route>></li>
                <li style="--i:1;"><a href="https://github.com/sky-86">{"GitHub"}</a></li>
            </ul>
        </div>
        </>
    }
}
