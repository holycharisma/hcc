use crate::hooks::use_window_scroll;
use crate::htmx::HtmxProcessedComponent;

use super::state::ActiveTab;
use super::tabs::{get_tabs, Tab};

use bounce::*;
use gloo_console::log;

use yew::prelude::*;

/*

*/

/*

        <div>
          <div id="app" class="h-screen flex flex-col pl-16 pt-10 selection:bg-violet-600 selection:text-white" >

           <div id="header" class="flex mb-8">
             <Header />
           </div>

           <div id="body" class="flex flex-row flex-grow">
             <div id="sidebar" class="flex mr-16 text-xl">
               <Sidebar />
             </div>
             <div id="content" class="flex flex-grow overflow-y-auto">
                 <Content />
             </div>
           </div>

          </div>
        </div>

*/

/*

// this is here for tailwind parser...

opacity-0	opacity: 0;
opacity-10	opacity: 0.1;
opacity-20	opacity: 0.2;
opacity-25	opacity: 0.25;
opacity-30	opacity: 0.3;
opacity-40	opacity: 0.4;
opacity-50	opacity: 0.5;
opacity-60	opacity: 0.6;
opacity-70	opacity: 0.7;
opacity-80	opacity: 0.8;
opacity-90	opacity: 0.9;
opacity-100	opacity: 1;
​
Basic usage
*/

#[function_component(IntersectionObserverHelper)]
pub fn io_helper() -> Html {
    let (x, y) = use_window_scroll();

    let splash_ref = use_node_ref();

    let maybe_cast = splash_ref.cast::<web_sys::HtmlHeadElement>();

    let splash_height: i32 = if maybe_cast.is_some() {
        maybe_cast.unwrap().scroll_height()
    } else {
        0
    };

    let scroll_depth = ((y as f32) / (splash_height as f32) * 10.0).floor() * 10.0;
    let scroll_depth_inverse = 100.0 - scroll_depth;

    let (header_opacity, splash_opacity) = if scroll_depth.is_nan() {
        (String::from("opacity-0"), String::from("opacity-100"))
    } else {
        (
            format!("opacity-{}", scroll_depth),
            format!("opacity-{}", scroll_depth_inverse),
        )
    };

    log!("setting header opacity to: ", header_opacity.clone());

    let xClass = format!("x-{}", x);
    let yClass = format!("y-{}", y);

    let debugClass = format!(
        "{}-{}-vs-{}-makes-{}",
        xClass, yClass, splash_height, scroll_depth
    );

    let header_inner_styles = format!(
        "fixed top-0 z-10 w-full border-b-2 border-dotted bg-lime-50 p-4 transition-opacity {} {}",
        header_opacity, debugClass,
    );

    let splash_inner_styles = format!(
        "splash mx-auto min-h-screen mb-2 pt-12 text-center transition-opacity {}",
        splash_opacity
    );

    let title = String::from("hcc (c) 2022 to infinity");
    html! {
     <div class="page">
      <div class={header_inner_styles}>
        <div class="header-logo float-left">
          <h1 class="text-xl"><img src="https://robohash.org/logo-here" height="40" width="40" /></h1>
        </div>
        <div class="header-items float-right">
          <button class="rounded-lg bg-lime-300 p-2">{"&nbsp;=&nbsp;"}</button>
        </div>
        <p class="clear-right"></p>
      </div>

      <div class="content-wrapper relative">
        <div ref={splash_ref} class={splash_inner_styles}>
          <img src="https://robohash.org/splash-img-here" height="600" width="600" class="m-2 mx-auto border-8" />
          <h1>{"I am the splash thingy"}</h1>
          <p class="clear-both" />
        </div>
        <div class="content-section min-h-screen flex pb-6">
          <div class="content flex-1 border-2 border-dotted">
            <div class="content-node m-2 inline-block h-72 w-32 bg-red-100">{"hello I am content in the content zone hello world"}</div>
            <div class="content-node m-2 inline-block h-36 bg-blue-100">{"hello I am content in the content zone"}</div>
            <div class="content-node m-2 inline-block h-72 w-24 bg-green-100">{"hello I am content in the content zone"}</div>
            <div class="content-node m-2 inline-block h-72 bg-purple-100">{"hello I am content in the content zone"}</div>
            <div class="content-node m-2 inline-block h-72 bg-orange-100">{"hello I am content in the content zone"}</div>
          </div>
          <div class="sidebar border-2 border-red-300 text-center">
            <div class="sidebar-header bg-gray-100">{"I am the sidebar zone"}</div>
            <div class="sidebar-extra mb-12 bg-gray-100 p-2">
              <img width="250" height="250" src="https://robohash.org/sidebar-player" />
              <p class="clear-both">{"a little extra stuff inside the sidebar..."}</p>
            </div>
          </div>
        </div>
        <div class="footer absolute w-full bg-blue-50 p-2 bottom-4 z-10 mt-2 border-t-4 border-dotted pt-2 text-center">
          <h1>{title}</h1>
        </div>
      </div>
    </div>

    }
}

#[function_component(App)]
pub fn app() -> Html {
    html! {
      <BounceRoot>
        <IntersectionObserverHelper />
      </BounceRoot>
    }
}

#[function_component(Header)]
fn header() -> Html {
    html! {
      <h1 class="bg-white bg-opacity-75 p-1 text-4xl font-extralight tracking-widest">{ "homepage" }</h1>
    }
}

#[function_component(Sidebar)]
fn sidebar() -> Html {
    let on_tab_select = {
        let setter = use_atom_setter::<ActiveTab>();
        Callback::from(move |tab: Tab| setter(ActiveTab::from(tab.name)))
    };

    let tab_vec: Vec<Tab> = get_tabs().into();

    let tabs = html! {
      <SidebarTabsList
        tabs={tab_vec}
        on_click={on_tab_select} />
    };

    html! {
      <ul>{tabs}</ul>
    }
}

#[function_component(Content)]
fn content() -> Html {
    let tabs = get_tabs();
    let active = use_atom_value::<ActiveTab>();
    let active_tab = tabs.iter().find(|t| active.name == t.name).unwrap();

    html! {
        <HtmxProcessedComponent
            name={active_tab.name.clone()}
            process={active_tab.htmx}
            body={active_tab.html.clone()} />
    }
}

#[derive(Clone, Properties, PartialEq)]
struct SidebarTabsListProps {
    tabs: Vec<Tab>,
    on_click: Callback<Tab>,
}

#[function_component(SidebarTabsList)]
fn tabs_list(SidebarTabsListProps { tabs, on_click }: &SidebarTabsListProps) -> Html {
    tabs.iter()
        .map(|tab| {
            let on_tab_click = {
                let callback = on_click.clone();
                let t = tab.clone();
                Callback::from(move |_| callback.emit(t.clone()))
            };

            let active = use_atom_value::<ActiveTab>();

            let is_active_class = {
                if active.name == tab.name {
                    "active"
                } else {
                    "inactive"
                }
            };

            let class_name = format!("link {}", is_active_class);

            html! {
              <li onclick={on_tab_click} class={class_name}>{&tab.name}</li>
            }
        })
        .collect()
}
