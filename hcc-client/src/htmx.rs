use wasm_bindgen::prelude::*;

use yew_hooks::{use_effect_update, use_mount};

use yew::prelude::*;

#[wasm_bindgen]
extern "C" { 
    #[wasm_bindgen(js_namespace=htmx)]
    pub fn process(el: &web_sys::Element); 

    #[wasm_bindgen(js_namespace=htmx)]
    pub fn remove(el: web_sys::Element); 

    #[wasm_bindgen(js_namespace=htmx)]
    pub fn find(selector: &str) -> web_sys::Element; 
}


fn get_by_id(el_id: &str) -> web_sys::Element {
    web_sys::window()
        .expect("window")
        .document()
        .expect("document")
        .get_element_by_id(el_id)
        .unwrap()
}

#[derive(Clone, Properties, PartialEq)]
pub struct HtmxProcessedComponentProps {
    pub name: String,
    pub body: Html,
    pub process: bool,
}

#[function_component(HtmxProcessedComponent)]
pub fn htmx_processed_component(props: &HtmxProcessedComponentProps) -> Html {
    let el_id = format!("{}-htmx-container", props.name.clone());

    if props.process {
        let update_name = el_id.clone();
        let mount_name = el_id.clone();

        use_mount(move || {
            let el = get_by_id(&mount_name);
            process(&el);
        });

        use_effect_update(move || {
            let el = get_by_id(&update_name);
            process(&el);

            || ()
        });
    }

    html! {
      <div id={el_id}>
         {props.body.clone()}
       </div>
    }
}