mod docs;
mod scss;

use std::{fs, io, path::Path};

use axum::{
    Router,
    extract::Request,
    http::StatusCode,
    response::{Html, IntoResponse, Response},
    routing::get,
};
use tower::ServiceExt;
use tower_http::services::{ServeDir, ServeFile};

const NAVBAR: &str = include_str!("../templates/navbar.html");

/// Fills in the parts every page shares.
pub(crate) fn page(template: &str) -> String {
    template.replace("{{ navbar }}", NAVBAR)
}

fn index() -> String {
    page(include_str!("../templates/index.html"))
}

fn download() -> String {
    page(include_str!("../templates/download.html"))
}

fn donate() -> String {
    page(include_str!("../templates/donate.html"))
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

fn about() -> String {
    let screenshots: String = SCREENSHOTS
        .iter()
        .map(|(file, caption)| {
            let url = format!(
                "https://raw.githubusercontent.com/rufin-app/rufin/master/resources/screenshots/{file}.png"
            );
            format!(
                r#"<figure><a href="{url}"><img src="{url}" alt="{caption}" loading="lazy" /></a><figcaption>{caption}</figcaption></figure>"#
            )
        })
        .collect();

    page(include_str!("../templates/about.html")).replace("{{ screenshots }}", &screenshots)
}

/// Every page of the site: (URL path, renderer). Used by both the server and the static build.
const PAGES: &[(&str, fn() -> String)] = &[
    ("/", index),
    ("/about", about),
    ("/download", download),
    ("/donate", donate),
];

/// Writes the whole site as plain files into `out_dir`, ready for GitHub Pages:
/// the files in `site/` (favicon, .well-known, ...), `static/`, and one
/// `index.html` per page (`/about` -> `about/index.html`).
fn build(out_dir: &Path) -> io::Result<()> {
    if out_dir.exists() {
        fs::remove_dir_all(out_dir)?;
    }
    copy_dir(Path::new("site"), out_dir)?;
    copy_dir(Path::new("static"), &out_dir.join("static"))?;

    for (route, render) in PAGES {
        let dir = out_dir.join(route.trim_start_matches('/'));
        fs::create_dir_all(&dir)?;
        let file = dir.join("index.html");
        fs::write(&file, render())?;
        println!("Rendered {route} -> {}", file.display());
    }

    for (route, html) in docs::render_all()? {
        let dir = out_dir.join(route.trim_start_matches('/'));
        fs::create_dir_all(&dir)?;
        let file = dir.join("index.html");
        fs::write(&file, html)?;
        println!("Rendered {route} -> {}", file.display());
    }
    docs::copy_assets(&out_dir.join(docs::DIR))?;
    Ok(())
}

fn copy_dir(from: &Path, to: &Path) -> io::Result<()> {
    fs::create_dir_all(to)?;
    for entry in fs::read_dir(from)? {
        let entry = entry?;
        let target = to.join(entry.file_name());
        if entry.file_type()?.is_dir() {
            copy_dir(&entry.path(), &target)?;
        } else {
            fs::copy(entry.path(), target)?;
        }
    }
    Ok(())
}

/// `/docs/...`: a rendered markdown page, or a file (image, ...) from `docs/`
async fn docs_page(req: Request) -> Response {
    let route = req.uri().path().to_owned();
    match docs::render(&route) {
        Ok(Some(html)) => Html(html).into_response(),
        Ok(None) => match docs::asset(&route) {
            Some(path) => ServeFile::new(path).oneshot(req).await.into_response(),
            None => StatusCode::NOT_FOUND.into_response(),
        },
        Err(e) => {
            eprintln!("failed to render {route}: {e}");
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

async fn serve() {
    // Override with e.g. `HOST=127.0.0.1 PORT=3000 cargo run`
    let host = std::env::var("HOST").unwrap_or_else(|_| "0.0.0.0".into());
    let port = std::env::var("PORT").unwrap_or_else(|_| "1234".into());
    let addr = format!("{host}:{port}");

    let mut app = Router::new();
    for &(route, render) in PAGES {
        app = app.route(route, get(move || async move { Html(render()) }));
    }
    let app = app
        .route("/docs", get(docs_page))
        .route("/docs/", get(docs_page))
        .route("/docs/{*path}", get(docs_page))
        .route_service("/favicon.ico", ServeFile::new("site/favicon.ico"))
        .nest_service("/static", ServeDir::new("static"));

    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .unwrap_or_else(|e| panic!("failed to bind {addr}: {e}"));
    println!("Listening on http://{addr}");
    axum::serve(listener, app).await.unwrap();
}

#[tokio::main]
async fn main() {
    scss::compile("assets/scss", "static/css").expect("failed to compile SCSS");

    // `cargo run -- build [dir]` writes a static copy of the site (default: dist/);
    // plain `cargo run` starts the development server.
    let mut args = std::env::args().skip(1);
    match args.next().as_deref() {
        Some("build") => {
            let out = args.next().unwrap_or_else(|| "dist".into());
            build(Path::new(&out)).expect("failed to build the static site");
        }
        Some(other) => {
            eprintln!("unknown command `{other}`; use `build [dir]` or no arguments to serve");
            std::process::exit(2);
        }
        None => serve().await,
    }
}
