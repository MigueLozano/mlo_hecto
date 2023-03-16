#![warn(clippy::all, clippy::pedantic)]
mod row;
mod editor;
mod terminal;
mod document;

pub use row::Row;
pub use editor::Position;
pub use terminal::Terminal;
pub use document::Document;

use editor::Editor;

fn main() {
    Editor::default().run();
}
