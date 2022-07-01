use yew::prelude::*;

use crate::external;

#[derive(PartialEq, Properties)]
pub struct CodeProps {
    pub code: String,
}

pub struct Code;

impl Component for Code {
    type Message = ();
    type Properties = CodeProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Code
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div>
                <pre>
                    <code class="language-rust">{&ctx.props().code}</code>
                </pre>
            </div>
        }
    }

    fn rendered(&mut self, _ctx: &Context<Self>, first_render: bool) {
        if first_render {
            external::highlightAll();
            web_sys::console::log_1(&"Loading".into());
        }
    }
}
