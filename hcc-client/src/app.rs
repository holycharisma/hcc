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
            <div id="app"
                class="h-screen flex flex-col pl-16 pt-10 selection:bg-violet-600 selection:text-white" ref={ self.node_ref.clone() }>
                <div id="header" class="flex mb-8 ">
                  <h1 class="bg-white p-1 text-4xl font-extralight tracking-widest">{ "homepage" }</h1>
                </div>
                <div id="body" class="flex flex-row flex-grow">
                  <div id="sidebar" class="flex mr-16 text-xl">
                    <ul>
                      <li class="link">{ "about" }</li>
                      <li class="link">{ "media" }</li>
                      <li class="link" hx-get="/handshake" hx-trigger="click" hx-target="#content">{ "hcc" }</li>
                    </ul>
                  </div>
                  <div id="content" class="flex flex-grow overflow-y-auto">
                    <div>
                      <h1>{ "👋" }</h1>
                      <p>{" good to see you..." }</p>
                    </div>
                  </div>
                </div>
          </div>
        }
    }
}
