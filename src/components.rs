use yew::prelude::*;
use yew_router::prelude::*;

use crate::solutions::Solutions;
use crate::routes::*;

#[derive(Properties, PartialEq)]
pub struct LinkRouteProps {
    pub to: Solutions,
    pub name: String,
}

#[function_component(LinkRoute)]
pub fn link_route(props: &LinkRouteProps) -> Html {
    html! {
        <Link<Route>
            to={Route::Problem{id: props.to.clone() }}>
            {&props.name}
        </Link<Route>>
    }
}


