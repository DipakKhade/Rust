use druid::widget::{Label, Button, Flex};
use druid::{AppLauncher, WindowDesc, Widget, PlatformError};

fn build_ui() -> impl Widget<()> {
    let mut col = Flex::column();

    col.add_flex_child(
        Label::new("Hello, Rust World!"),
        1.0
    );
    col.add_flex_child(
        Button::new("Click me!")
            .on_click(|_ctx, _data, _env| {
                println!("Button clicked!");
            }),
        1.0
    );

    col
}

fn main() -> Result<(), PlatformError> {
    let main_window = WindowDesc::new(|| build_ui())
        .title("A Simple Rust Desktop App")
        .window_size((400.0, 400.0));

    AppLauncher::with_window(main_window)
        .launch(())?;
    Ok(())
}
