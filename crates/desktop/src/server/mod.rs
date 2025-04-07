use crate::ui::MainWindow;

pub fn router() -> axum::Router {
    axum::Router::new()
}

pub fn setup(_window: &MainWindow) {}
