use std::fmt;

use bounce::*;

use yew::prelude::*;

#[derive(Clone, PartialEq, Atom)]
pub struct Tab {
    pub name: String,
    pub html: Html,
    pub htmx: bool,
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

pub fn get_tabs() -> [Tab; 3] {
    [
        Tab {
            name: "about".to_string(),
            html: html! {
            <div>
              <h1>{ "👋" }</h1>
              <p>{" good to see you..." }</p>
            </div>
            },
            htmx: false,
        },
        Tab {
            name: "media".to_string(),
            html: html! {
              <div>{" media.. "}</div>
            },
            htmx: false,
        },
        Tab {
            name: "hcc".to_string(),
            html: html! {
                <div>
                        <div id="hcc-top-hx-target" hx-get="/handshake" hx-trigger="load" />
                  </div>
            },
            htmx: true,
        },
    ]
}
