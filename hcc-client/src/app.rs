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
        <div id="app-container" ref={self.node_ref.clone()} hx-get="/handshake" hx-trigger="load" />
        }
    }
}
