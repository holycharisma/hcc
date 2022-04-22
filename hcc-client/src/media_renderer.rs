
use wasm_bindgen::prelude::*;
use web_sys::Element;

use yew::prelude::*;

struct MediaRenderer {}


#[derive(Properties, Clone, PartialEq)]
struct MediaRendererProps {
    slug: String,
    medium : String,
    media: JsValue,
}

impl Component for MediaRenderer {
    type Message = ();
    type Properties = MediaRendererProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self { }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let medium = ctx.props().medium.clone();
        let slug = ctx.props().slug.clone();
        let title = format!("{}: {}", medium, slug);
        html! {
            <div>
                <h1>{title}</h1>
            </div>
        }
    }
}

#[wasm_bindgen]
pub fn render_media_node(el: Element, slug: String, media_type: String, media_json: JsValue) {

    let props = yew::props!(MediaRenderer::Properties {
        slug: slug,
        medium: media_type,
        media: media_json
    });

    yew::start_app_with_props_in_element::<MediaRenderer>(el, props);

}