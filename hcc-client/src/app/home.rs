use crate::htmx::HtmxProcessedComponent;

use super::state::ActiveTab;
use super::tabs::{get_tabs, Tab};

use bounce::*;

use yew::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    html! {
      <BounceRoot>
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
