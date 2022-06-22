use yew::prelude::*;
use yew_router::prelude::*;

use crate::routes::*;

#[function_component(Navbar)]
pub fn navbar() -> Html {
    html! {
        <>
        <BrowserRouter>
            <div class="navbar">
                <ul>
                    <li><Link<Route> to={Route::Home}>{"Home"}</Link<Route>></li>
                    <li><Link<Route> to={Route::Problems}>{"Problems"}</Link<Route>></li>
                    <li><Link<Route> to={Route::Projects}>{"Projects"}</Link<Route>></li>
                    <li style="float:right;"><a href="https://github.com/sky-86">{"GitHub"}</a></li>
                </ul>
            </div>
            <div>
                <Switch<Route> render={Switch::render(switch)} />
            </div>
        </BrowserRouter>
        </>
    }
}
