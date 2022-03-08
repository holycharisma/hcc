use crate::htmx;

use std::fmt;

use bounce::*;

use yew_hooks::{use_effect_update, use_mount};

use yew::prelude::*;

#[function_component(Header)]
fn header() -> Html {
    html! {
      <h1 class="bg-white bg-opacity-75 p-1 text-4xl font-extralight tracking-widest">{ "homepage" }</h1>
    }
}

#[derive(Clone, PartialEq, Atom)]
struct Tab {
    name: String,
    html: Html,
}

impl Default for Tab {
    fn default() -> Self {
        get_tabs().get(1).unwrap().clone()
    }
}

impl fmt::Display for Tab {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

fn get_tabs() -> [Tab; 3] {
    [
        Tab {
            name: "about".to_string(),
            html: html! {
            <div>
              <h1>{ "👋" }</h1>
              <p>{" good to see you..." }</p>
            </div>
            },
        },
        Tab {
            name: "media".to_string(),
            html: html! {
              <div>{" media.. "}</div>
            },
        },
        Tab {
            name: "hcc".to_string(),
            html: html! {
              <div>
                <div hx-get="/handshake" hx-trigger="load" />
              </div>
            },
        },
    ]
}

#[derive(PartialEq, Atom)]
struct ActiveTab {
    name: String,
}

impl From<String> for ActiveTab {
    fn from(s: String) -> Self {
        Self { name: s }
    }
}

impl Default for ActiveTab {
    fn default() -> Self {
        Self {
            name: String::from(&get_tabs().get(0).unwrap().name),
        }
    }
}

impl fmt::Display for ActiveTab {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
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

            let reader = use_atom_value::<ActiveTab>();

            let is_active_class = {
                if reader.name == tab.name {
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

#[function_component(Sidebar)]
fn sidebar() -> Html {
    let on_tab_select = {
        let setter = use_atom_setter::<ActiveTab>();
        Callback::from(move |tab: Tab| setter(ActiveTab::from(tab.name)))
    };

    let tab_vec: Vec<Tab> = get_tabs().into();

    let tabs = html! {
      <SidebarTabsList tabs={tab_vec} on_click={on_tab_select} />
    };

    html! {
      <ul>{tabs}</ul>
    }
}

fn get_by_id(name: &str) -> web_sys::Element {
    let el_id = format!("content-{}", name.clone());
    let el = web_sys::window()
        .expect("window")
        .document()
        .expect("document")
        .get_element_by_id(&el_id)
        .unwrap();
    el
}

#[function_component(Content)]
fn content() -> Html {
    let tabs = get_tabs();
    let active = use_atom_value::<ActiveTab>();
    let active_tab = tabs.iter().find(|t| active.name == t.name).unwrap();

    let update_name = active_tab.name.clone();
    let mount_name = active_tab.name.clone();

    let el_id = format!("content-{}", active_tab.name.clone());

    use_mount(move || {
        let el = get_by_id(&mount_name);
        htmx::process(&el);
    });

    use_effect_update(move || {
        let el = get_by_id(&update_name);
        htmx::process(&el);

        || ()
    });

    html! {
      <div id={el_id}>
        {active_tab.html.clone()}
      </div>
    }
}

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
