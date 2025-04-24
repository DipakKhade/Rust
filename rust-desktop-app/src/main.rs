// ... imports ...
use druid::widget::{Button, Flex, Label, List, Scroll, TextBox};
use druid::{AppLauncher, Data, Env, Lens, Widget, WidgetExt, WindowDesc, im::Vector}; // Add im::Vector

#[derive(Clone, Data, Lens)]
struct AppState {
    current_note: String,
    saved_notes: Vector<String>, // Use Vector for the list
}

fn build_ui() -> impl Widget<AppState> {
    // Label and TextBox for the current note
    let label = Label::new("Enter your note:");
    let textbox = TextBox::multiline()
        .with_placeholder("Start typing here...")
        .fix_height(150.0) // Adjust height
        .lens(AppState::current_note);

    // Button to save the note
    let save_button = Button::new("Save Note")
        .on_click(|ctx, data: &mut AppState, _env| {
            if !data.current_note.trim().is_empty() {
                // Add the current note to the saved_notes vector
                data.saved_notes.push_back(data.current_note.clone());
                // Clear the text box after saving
                data.current_note = String::new();
            }
        });

    // Label for the saved notes list
    let list_label = Label::new("Saved Notes:");

    // List widget to display saved notes
    // We wrap it in a Scroll container in case the list gets long
    let notes_list = Scroll::new(List::new(|| {
            Label::new(|item: &String, _env: &_| item.clone()) // Each item in the list is a Label
                .padding(5.0)
                .expand_width() // Make labels take full width
                .height(50.0) // Give items a fixed height
                .background(druid::Color::rgb8(0x12, 0xee, 0xee)) // Light grey background
        }))
        .vertical() // Allow vertical scrolling
        .lens(AppState::saved_notes); // Bind the list to the saved_notes vector

    // Arrange widgets vertically
    Flex::column()
        .with_child(label)
        .with_spacer(8.0)
        .with_child(textbox)
        .with_spacer(8.0)
        .with_child(save_button)
        .with_spacer(15.0)
        .with_child(list_label)
        .with_spacer(8.0)
        .with_flex_child(notes_list, 1.0) // Let the list expand
        .padding(10.0)
}

pub fn main() {
    // Initial state with empty current note and empty saved notes list
    let initial_state = AppState {
        current_note: String::new(),
        saved_notes: Vector::new(), // Initialize as empty Vector
    };

    // Describe the main window (adjust size if needed)
    let main_window = WindowDesc::new(build_ui())
        .title("Note Taker")
        .window_size((400.0, 500.0)); // Increased height for the list

    // Launch the application
    AppLauncher::with_window(main_window)
        .launch(initial_state)
        .expect("Failed to launch application");
}
