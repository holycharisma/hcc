use tide::prelude::*;
use tide::{http::mime, Redirect, Request, Response, Result};

use crate::dao;
use crate::util::emoji;
use crate::util::encryption;
use crate::util::password::PasswordUtil;
use crate::wiring::ServerWiring;

use domain::session::SessionUser;

use tinytemplate::TinyTemplate;

use askama::Template; // bring trait in scope

#[derive(Debug)]
enum MediaType {
    Image,
    Text,
    Audio,
}

impl std::fmt::Display for MediaType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
trait MediaRenderer {
    fn render_json(&self) -> String;
}

#[derive(Serialize, Deserialize)]
struct ImageMedia {
    url: String,
}

impl MediaRenderer for ImageMedia {
    fn render_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}

#[derive(Serialize, Deserialize)]
struct TextMedia {
    body: String,
}

impl MediaRenderer for TextMedia {
    fn render_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}

#[derive(Serialize, Deserialize)]
struct AudioMedia {
    title: String,
    duration: i32,
    khz: i32,
    kbps: i32,
    url: String,
}

impl MediaRenderer for AudioMedia {
    fn render_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}

struct MediaNodeHtml {
    innerHTML: String,
}

struct MediaNodeBundle {
    media_type: MediaType,
    slug: String,
    template: String,
    renderer: Box<dyn MediaRenderer>,
}

impl MediaNodeBundle {
    fn text(slug: &str, template: &str, txt: TextMedia) -> Self {
        MediaNodeBundle {
            media_type: MediaType::Text,
            slug: slug.to_owned(),
            template: template.to_owned(),
            renderer: Box::new(txt),
        }
    }

    fn audio(slug: &str, template: &str, audio: AudioMedia) -> Self {
        MediaNodeBundle {
            media_type: MediaType::Audio,
            slug: slug.to_owned(),
            template: template.to_owned(),
            renderer: Box::new(audio),
        }
    }

    fn img(slug: &str, template: &str, img: ImageMedia) -> Self {
        MediaNodeBundle {
            media_type: MediaType::Image,
            slug: slug.to_owned(),
            template: template.to_owned(),
            renderer: Box::new(img),
        }
    }
}

#[derive(Template)] // this will generate the code...
#[template(path = "media/list.html.j2")] // using the template in this path, relative
struct ListGetViewModel {
    media: Vec<MediaNodeHtml>,
}

#[derive(Template)] // this will generate the code...
#[template(path = "media/node.html.j2")] // using the template in this path, relative
struct MediaNodeViewModel {
    slug: String,
    medium: String,
    media: String,
}

#[derive(Serialize, Deserialize)]
struct MediaTemplateContext {
    media: String,
}

fn render_bundle(bundle: &MediaNodeBundle) -> MediaNodeHtml {
    let mut tt = TinyTemplate::new();

    tt.add_template(&bundle.slug, &bundle.template)
        .expect("hope I can add this template!");

    tt.set_default_formatter(&tinytemplate::format_unescaped);

    let media_json = bundle.renderer.render_json();
    let media_json_base64 = base64::encode(media_json);

    let media_context = MediaNodeViewModel {
        slug: bundle.slug.to_owned(),
        medium: bundle.media_type.to_string().to_ascii_lowercase(),
        media: media_json_base64,
    };

    let media_html = media_context
        .render()
        .expect("error rendering media node html");

    let context = MediaTemplateContext { media: media_html };

    MediaNodeHtml {
        innerHTML: tt
            .render(&bundle.slug, &context)
            .expect("error in rendering custom media template!"),
    }
}

struct FakeMediaDatabase {}

use uuid::Uuid;

impl FakeMediaDatabase {
    pub fn gen_slug() -> String {
        let id = Uuid::new_v4();
        format!("/media@{}", id.urn())
    }

    pub fn get_media() -> Vec<MediaNodeHtml> {
        let items = vec![
            MediaNodeBundle::img(
                FakeMediaDatabase::gen_slug().as_str(),
                r#"<div class="tiny-template-example">{media}</div>"#,
                ImageMedia {
                    url: String::from("data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAAoAAAAKCAYAAACNMs+9AAAAAXNSR0IArs4c6QAAAQhJREFUKFNFkLFug0AQRGeL2IUxKFCBy1v+/0fwF9xiGs7QQHRHHMlOsdEdsrLFaovRzNuh7tppWZY4Ho5QAGnRfpASnr9PrMsKug03XZcvMBsURREVaaJ+CwHWWlRVCXLO6emUQcSCW0Z+LpKhDx7SC5hb/DweUThqXTcIIUCkBzMDpBARsOGUMs0TaHROL3WT4sLmYa2AiGDMG0Uxz/Pu2DRNRMfmfXKKfG3CyAEizNMEcuOo9aXB5jdIb2G4BanCSo+WDc4x+n4HjaPTLMvQ9wITmfIcSorgI/P+zPdjAw3DoOuywLRR9F9PYg47c+yZrl2nn1WFw+EjFfzuWyM1EV6vF5ZlxR/Ak5UoIMERKAAAAABJRU5ErkJggg=="),
                },
            ),
            
            MediaNodeBundle::text(
                FakeMediaDatabase::gen_slug().as_str(),
                r#"<div class="tiny-template-example">{media}</div>"#,
                TextMedia {
                    body: String::from("text body here"),
                },
            ),

            MediaNodeBundle::audio(
                FakeMediaDatabase::gen_slug().as_str(),
                r#"{media}"#,
                AudioMedia {
                    title: String::from("15 second mp3"),
                    url: String::from("mp3"),
                    duration: 15,
                    khz: 44,
                    kbps: 192,
                },
            ),
        ];

        items.iter().map(|e| render_bundle(e)).collect()
    }
}

pub async fn get(req: Request<ServerWiring>) -> Result {
    let rendered_media: Vec<MediaNodeHtml> = FakeMediaDatabase::get_media();

    let view_context = ListGetViewModel {
        media: rendered_media,
    };

    let secrets: &encryption::SharedKeyring = req.ext().unwrap();

    let encrypted_body = secrets
        .encrypt_broadcast_emoji(&view_context.render().unwrap())
        .await
        .unwrap()
        .message;

    let response = Response::builder(200)
        .content_type(mime::HTML)
        .body_string(encrypted_body)
        .build();
    Ok(response)
}
