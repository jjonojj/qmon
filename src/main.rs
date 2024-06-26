
// the qmon editor.

mod window;
mod editor;
use editor::Editor;

fn main() {
    Editor::default().run();
}