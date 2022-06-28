use yew::prelude::*;
use yew_hooks::use_bool_toggle;
use yew_router::prelude::*;
use yew_octicons::IconKind;
use yew_octicons::Icon;

use crate::routes::*;

#[function_component(Navbar)]
pub fn navbar() -> Html {
    html! {
        <div class={classes!("flex", "justify-center")}>
            <div class={classes!("fixed", "top-0", "left-0", 
                "h-screen", "w-16", "flex", "flex-col",
                "bg-gray-900", "shadow-lg")}>

                <SideBarIcon text={Some("Hello world".to_string())} icon={IconKind::Alert} />
                <SideBarIcon icon={IconKind::Alert} />
                <SideBarIcon icon={IconKind::Alert} />
                <SideBarIcon icon={IconKind::Alert} />
            </div>

            <div>
                <BrowserRouter>
                    <div>
                        <Switch<Route> render={Switch::render(switch)} />
                    </div>
                </BrowserRouter>
            </div>
        </div>
    }
}

#[derive(Properties, PartialEq)]
struct SideBarIconProps {
    icon: IconKind,
    text: Option<String>,
}

#[function_component(SideBarIcon)]
fn side_bar_icon(props: &SideBarIconProps) -> Html {

    html! {
        <div class={classes!("sidebar-icon", "group")}>
            {Icon::new_big(props.icon)}
        
            if let Some(text) = &props.text {
                <span class={classes!("sidebar-tooltip", "group-hover:visible")}>
                    {text}
                </span>
            }
        </div>
    }
}
