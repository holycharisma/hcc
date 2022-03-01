use crate::htmx;

use yew::prelude::*;

pub struct App {
    node_ref: NodeRef,
}

impl Component for App {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        App {
            node_ref: NodeRef::default(),
        }
    }

    fn rendered(&mut self, _ctx: &Context<Self>, _first_render: bool) {
        let el = self.node_ref.cast::<web_sys::Element>().unwrap();
        htmx::process(&el);
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
        <div class="app-container max-w-screen-2xl px-4 md:px-8 mx-auto" ref={self.node_ref.clone()}>
          <div class="login-container" hx-get="/login" hx-trigger="load">
          </div>
        </div>
        }
    }
}
