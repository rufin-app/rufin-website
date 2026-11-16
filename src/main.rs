mod scss;

use axum::{Router, extract::State, response::Html, routing::get};
use tower_http::services::{ServeDir, ServeFile};

async fn index(State(message): State<String>) -> Html<String> {
    Html(
        include_str!("../templates/index.html")
            .replace("{{ navbar }}", include_str!("../templates/navbar.html"))
            .replace("{{ message }}", &message),
    )
}

async fn download() -> Html<String> {
    Html(
        include_str!("../templates/download.html")
            .replace("{{ navbar }}", include_str!("../templates/navbar.html")),
    )
}

async fn donate() -> Html<String> {
    Html(
        include_str!("../templates/donate.html")
            .replace("{{ navbar }}", include_str!("../templates/navbar.html")),
    )
}

/// Screenshots from the Rufin repository: (file name without `.png`, caption)
const SCREENSHOTS: &[(&str, &str)] = &[
    ("Rufin", "Home"),
    ("artist_detail", "Artist details"),
    ("artists", "Artists"),
    ("compact_sidebar", "Compact sidebar"),
    ("fullscreen_player", "Fullscreen player"),
    ("smart_playlists", "Smart playlists"),
    ("playlist_settings", "Playlist settings"),
    ("edit_metadata", "Metadata editor"),
    ("edit_metadata_lookup", "Metadata lookup"),
    ("edit_metadata_lyrics", "Lyrics editor"),
    ("equalizer_settings", "Equalizer"),
    ("customize_display", "Customize display"),
    ("reorder_sidebar", "Rearrangeable layout"),
    ("integrations", "Integrations"),
];

async fn about() -> Html<String> {
    let screenshots: String = SCREENSHOTS
        .iter()
        .map(|(file, caption)| {
            let url = format!(
                "https://raw.githubusercontent.com/screwys/Rufin/main/resources/screenshots/{file}.png"
            );
            format!(
                r#"<figure><a href="{url}"><img src="{url}" alt="{caption}" loading="lazy" /></a><figcaption>{caption}</figcaption></figure>"#
            )
        })
        .collect();

    Html(
        include_str!("../templates/about.html")
            .replace("{{ navbar }}", include_str!("../templates/navbar.html"))
            .replace("{{ screenshots }}", &screenshots),
    )
}

#[tokio::main]
async fn main() {
    scss::compile("assets/scss", "static/css").expect("failed to compile SCSS");

    let message = String::from("Hello, world!");

    let app = Router::new()
        .route("/", get(index))
        .route("/about", get(about))
        .route("/download", get(download))
        .route("/donate", get(donate))
        .route_service("/favicon.ico", ServeFile::new("site/favicon.ico"))
        .nest_service("/static", ServeDir::new("static"))
        .with_state(message);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:1234").await.unwrap();
    println!("Listening on http://0.0.0.0:1234");
    axum::serve(listener, app).await.unwrap();
}
