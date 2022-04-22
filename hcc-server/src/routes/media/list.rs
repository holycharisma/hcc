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
    mp3_url: String,
    img_url: String,
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

fn render_bundle(e: &MediaNodeBundle) -> MediaNodeHtml {
    let mut tt = TinyTemplate::new();

    tt.add_template(&e.slug, &e.template)
        .expect("hope I can add this template!");

    tt.set_default_formatter(&tinytemplate::format_unescaped);

    let media_json = e.renderer.render_json();
    let media_json_base64 = base64::encode(media_json);

    let media_context = MediaNodeViewModel {
        slug: e.slug.to_owned(),
        medium: e.media_type.to_string().to_ascii_lowercase(),
        media: media_json_base64,
    };

    let media_html = media_context
        .render()
        .expect("error rendering media node html");

    let context = MediaTemplateContext { media: media_html };

    MediaNodeHtml {
        innerHTML: tt
            .render(&e.slug, &context)
            .expect("error in rendering custom media template!"),
    }
}

pub async fn get(req: Request<ServerWiring>) -> Result {
    let text_node = TextMedia {
        body: String::from("This is a blog post"),
    };

    let img_node = ImageMedia {
        url: String::from("url"),
    };

    let audio_node = AudioMedia {
        title: String::from("title"),
        mp3_url: String::from("mp3_url"),
        img_url: String::from("img_url"),
    };

    let default_node_template = r#"
<div class="pt-10 tinytemplate-node-wrapper-example">
    {media}
</div>
"#;

    let rendered_media: Vec<MediaNodeHtml> = {
        let items: Vec<MediaNodeBundle> = vec![
            MediaNodeBundle::img("/img-example", default_node_template.clone(), img_node),
            MediaNodeBundle::audio("/mp3-example", default_node_template.clone(), audio_node),
            MediaNodeBundle::text("/txt-example", default_node_template.clone(), text_node),
        ];

        items.iter().map(|e| render_bundle(e)).collect()
    };

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
