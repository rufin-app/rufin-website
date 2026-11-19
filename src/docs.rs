//! Markdown docs from the `docs/` folder, served under `/docs`.
//!
//! Every `.md` file becomes a page, and every subfolder becomes its own
//! endpoint, whose page is the `about.md` inside it:
//!
//! - `docs/about.md`       -> `/docs`
//! - `docs/install.md`     -> `/docs/install`
//! - `docs/thing/about.md` -> `/docs/thing`
//! - `docs/thing/setup.md` -> `/docs/thing/setup`
//!
//! A folder without an `about.md` still gets a page that lists what is inside it.
//! Other files (images, ...) are served as they are: `docs/thing/shot.png` -> `/docs/thing/shot.png`.

use std::{
    fs, io,
    path::{Component, Path, PathBuf},
};

use pulldown_cmark::{Options, Parser, html};

pub const DIR: &str = "docs";
const ROOT: &str = "/docs";
const ABOUT: &str = "about";

struct Folder {
    route: String,
    title: String,
    about: Option<PathBuf>,
    /// The `.md` files in this folder, except `about.md`
    pages: Vec<Page>,
    folders: Vec<Folder>,
}

struct Page {
    route: String,
    title: String,
    path: PathBuf,
}

/// What a docs URL shows
enum Content<'a> {
    Markdown(&'a Path),
    /// A folder with no `about.md`: list its contents
    Listing(&'a Folder),
}

/// Reads the whole `docs/` tree. A missing folder just means there are no docs yet.
fn scan_root() -> io::Result<Folder> {
    let dir = Path::new(DIR);
    if !dir.is_dir() {
        return Ok(Folder {
            route: ROOT.into(),
            title: "Docs".into(),
            about: None,
            pages: Vec::new(),
            folders: Vec::new(),
        });
    }
    scan(dir, ROOT.into(), "Docs")
}

fn scan(dir: &Path, route: String, name: &str) -> io::Result<Folder> {
    let mut entries = fs::read_dir(dir)?.collect::<io::Result<Vec<_>>>()?;
    entries.sort_by_key(|e| e.file_name());

    let subdirs: Vec<String> = entries
        .iter()
        .filter(|e| e.path().is_dir())
        .map(|e| e.file_name().to_string_lossy().into_owned())
        .collect();

    let mut folder = Folder {
        route,
        title: String::new(),
        about: None,
        pages: Vec::new(),
        folders: Vec::new(),
    };

    for entry in entries {
        let path = entry.path();
        let file_name = entry.file_name().to_string_lossy().into_owned();
        if path.is_dir() {
            let route = format!("{}/{file_name}", folder.route);
            folder.folders.push(scan(&path, route, &file_name)?);
        } else if let Some(stem) = file_name.strip_suffix(".md") {
            if stem == ABOUT {
                folder.about = Some(path);
            } else if subdirs.iter().any(|d| d == stem) {
                eprintln!(
                    "warning: {} is ignored, the folder {stem}/ already uses its URL",
                    path.display()
                );
            } else {
                folder.pages.push(Page {
                    route: format!("{}/{stem}", folder.route),
                    title: title_of(&path, stem),
                    path,
                });
            }
        }
    }

    folder.title = match &folder.about {
        Some(about) => title_of(about, name),
        None => prettify(name),
    };
    Ok(folder)
}

/// The page title: the first `# Heading` in the file, or else the file name
fn title_of(path: &Path, fallback: &str) -> String {
    fs::read_to_string(path)
        .ok()
        .and_then(|text| {
            text.lines()
                .find_map(|line| line.strip_prefix("# ").map(|t| t.trim().to_string()))
        })
        .filter(|t| !t.is_empty())
        .unwrap_or_else(|| prettify(fallback))
}

/// `getting-started` -> `Getting started`
fn prettify(name: &str) -> String {
    let name = name.replace(['-', '_'], " ");
    let mut chars = name.chars();
    match chars.next() {
        Some(first) => first.to_uppercase().chain(chars).collect(),
        None => name,
    }
}

fn find<'a>(folder: &'a Folder, route: &str) -> Option<(&'a str, Content<'a>)> {
    if folder.route == route {
        let content = match &folder.about {
            Some(about) => Content::Markdown(about),
            None => Content::Listing(folder),
        };
        return Some((&folder.title, content));
    }
    if let Some(page) = folder.pages.iter().find(|p| p.route == route) {
        return Some((&page.title, Content::Markdown(&page.path)));
    }
    folder.folders.iter().find_map(|sub| find(sub, route))
}

fn all_routes(folder: &Folder, out: &mut Vec<String>) {
    out.push(folder.route.clone());
    out.extend(folder.pages.iter().map(|p| p.route.clone()));
    for sub in &folder.folders {
        all_routes(sub, out);
    }
}

fn escape(text: &str) -> String {
    text.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}

fn link(route: &str, title: &str, current: &str) -> String {
    let active = if route == current { r#" aria-current="page""# } else { "" };
    format!(r#"<a href="{}"{active}>{}</a>"#, escape(route), escape(title))
}

/// The navigation tree shown next to every docs page
fn sidebar(folder: &Folder, current: &str) -> String {
    let mut out = format!("<li>{}", link(&folder.route, &folder.title, current));
    if !folder.pages.is_empty() || !folder.folders.is_empty() {
        out.push_str("<ul>");
        for page in &folder.pages {
            out += &format!("<li>{}</li>", link(&page.route, &page.title, current));
        }
        for sub in &folder.folders {
            out += &sidebar(sub, current);
        }
        out.push_str("</ul>");
    }
    out.push_str("</li>");
    out
}

fn markdown(text: &str) -> String {
    let options = Options::ENABLE_TABLES
        | Options::ENABLE_STRIKETHROUGH
        | Options::ENABLE_TASKLISTS
        | Options::ENABLE_FOOTNOTES
        | Options::ENABLE_HEADING_ATTRIBUTES;
    let mut out = String::new();
    html::push_html(&mut out, Parser::new_ext(text, options));
    out
}

fn listing(folder: &Folder) -> String {
    let mut out = format!(r#"<h1>{}</h1><ul class="docs-listing">"#, escape(&folder.title));
    for page in &folder.pages {
        out += &format!("<li>{}</li>", link(&page.route, &page.title, ""));
    }
    for sub in &folder.folders {
        out += &format!("<li>{}</li>", link(&sub.route, &sub.title, ""));
    }
    out.push_str("</ul>");
    out
}

fn render_in(root: &Folder, route: &str) -> io::Result<Option<String>> {
    let Some((title, content)) = find(root, route) else {
        return Ok(None);
    };
    let body = match content {
        Content::Markdown(path) => markdown(&fs::read_to_string(path)?),
        Content::Listing(folder) => listing(folder),
    };
    let html = crate::page(include_str!("../templates/docs.html"))
        .replace("{{ title }}", &escape(title))
        .replace("{{ sidebar }}", &sidebar(root, route))
        .replace("{{ content }}", &body);
    Ok(Some(html))
}

/// Renders the docs page at `route` (e.g. `/docs/thing`), or `None` if there is none.
/// Reads the files on every call, so edits show up without restarting the server.
pub fn render(route: &str) -> io::Result<Option<String>> {
    let route = route.trim_end_matches('/');
    render_in(&scan_root()?, route)
}

/// Every docs page as (route, HTML), for the static build
pub fn render_all() -> io::Result<Vec<(String, String)>> {
    let root = scan_root()?;
    let mut routes = Vec::new();
    all_routes(&root, &mut routes);
    routes
        .into_iter()
        .map(|route| {
            let html = render_in(&root, &route)?.expect("route came from the docs tree");
            Ok((route, html))
        })
        .collect()
}

/// The non-markdown file a docs URL points to (`/docs/a/b.png` -> `docs/a/b.png`), if any
pub fn asset(route: &str) -> Option<PathBuf> {
    let rest = route.strip_prefix(ROOT)?.strip_prefix('/')?;
    let rest = Path::new(rest);
    // Only plain names, so a URL can never reach outside docs/
    if !rest.components().all(|c| matches!(c, Component::Normal(_))) {
        return None;
    }
    let path = Path::new(DIR).join(rest);
    (path.is_file() && path.extension().is_none_or(|ext| ext != "md")).then_some(path)
}

/// Copies the non-markdown files from `docs/` into `out_dir`, keeping the folder layout
pub fn copy_assets(out_dir: &Path) -> io::Result<()> {
    fn copy(from: &Path, to: &Path) -> io::Result<()> {
        for entry in fs::read_dir(from)? {
            let path = entry?.path();
            let target = to.join(path.file_name().unwrap());
            if path.is_dir() {
                copy(&path, &target)?;
            } else if path.extension().is_none_or(|ext| ext != "md") {
                fs::create_dir_all(to)?;
                fs::copy(&path, target)?;
            }
        }
        Ok(())
    }
    let dir = Path::new(DIR);
    if dir.is_dir() { copy(dir, out_dir) } else { Ok(()) }
}
