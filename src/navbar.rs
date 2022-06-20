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
                    <li><Link<Route> to={Route::About}>{"About"}</Link<Route>></li>
                    <li><Link<Route> to={Route::Problems}>{"Problems"}</Link<Route>></li>
                    <li><Link<Route> to={Route::Projects}>{"Projects"}</Link<Route>></li>
                </ul>
            </div>
            <div>
                <Switch<Route> render={Switch::render(switch)} />
            </div>
        </BrowserRouter>
        </>
    }
}
