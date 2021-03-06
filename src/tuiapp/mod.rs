
mod app;
mod ui;

pub use app::TUIApp;

#[derive(Debug)]
pub enum Focus {
    INSTALLED_VERSION_LIST,
    ALL_VERSION_LIST,
    STATUS_LIST,
    DOWNLOAD_PAGE,
}
