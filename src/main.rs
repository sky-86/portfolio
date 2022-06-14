use yew::prelude::*;
use yew_router::prelude::*;
//use web_sys::console;

mod pages;
mod routes;

use routes::*;

#[function_component(Root)]
fn root() -> Html {
    html! {
        <div>
            <BrowserRouter>
                <div class="navbar">
                    <ul>
                        <li><Link<Route> to={Route::Home}>{"Home"}</Link<Route>></li>
                        <li><Link<Route> to={Route::Problems}>{"Problems"}</Link<Route>></li>
                    </ul>
                </div>
                <Switch<Route> render={Switch::render(switch)} />
            </BrowserRouter>
        </div>
    }
}

fn main() {
    yew::start_app::<Root>();
}
