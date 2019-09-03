use gio::prelude::*;
use glob::glob;
use gtk::prelude::*;
use gtk::{
    Application, ApplicationWindow, Button, FileChooserAction, FileChooserDialog, Grid, Image,
};
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

struct State {
    current_index: Option<usize>,
    images: Option<Vec<String>>,
}

fn build_ui(application: &Application, state: Arc<Mutex<State>>) {
    application.connect_activate(move |app| {
        let window = ApplicationWindow::new(app);
        window.set_title("Simple Image Viewer");
        window.set_default_size(500, 500);
        let window_weak = window.downgrade();

        let grid = Grid::new();
        window.add(&grid);

        let image_panel = Image::new();
        let image_panel_weak = image_panel.downgrade();
        grid.attach(&image_panel, 0, 0, 1, 1);

        let left_button = Button::new_with_label("left");
        left_button.connect_clicked(|_| {
            println!("Clicked!");
        });
        grid.attach(&left_button, 0, 1, 1, 1);

        let right_button = Button::new_with_label("right");
        right_button.connect_clicked(|_| {
            //TODO:
        });
        grid.attach(&right_button, 1, 1, 1, 1);

        let select_folder_button = Button::new_with_label("select folder");
        let select_folder_button_state = state.clone();
        select_folder_button.connect_clicked(move |_| {
            let window = window_weak
                .upgrade()
                .expect("could not upgrade window reference");
            let image_panel = image_panel_weak
                .upgrade()
                .expect("could not get image panel reference");
            let folder_selector = FileChooserDialog::new(
                Some("select folder to view"),
                Some(&window),
                FileChooserAction::SelectFolder,
            );
            folder_selector.add_buttons(&[
                ("Open", gtk::ResponseType::Ok),
                ("Cancel", gtk::ResponseType::Cancel),
            ]);

            if folder_selector.run() == gtk::ResponseType::Ok {
                let folder = folder_selector.get_filename().expect("Couldn't get folder");
                println!("selected folder {:?}", folder);

                let mut locked_state = select_folder_button_state
                    .lock()
                    .expect("unable to get mutex lock");
                locked_state.current_index = Some(0);
                locked_state.images = get_images_from_dir(&folder);
            }

            folder_selector.destroy();
        });
        grid.attach(&select_folder_button, 0, 2, 1, 1);

        window.show_all();
    });
}

fn get_images_from_dir(dir: &PathBuf) -> Option<Vec<String>> {
    let mut result = None;

    for file in glob(&format!(
        "{}/*.png",
        dir.to_str().expect("could not resolve dir as string")
    )).expect("could not evaluate glob") {
        match file {
            Ok(path) => {
                dbg!(path);
            }
            Err(error) => {
                println!("{}", error);
            }
        }
    }

    result
}

fn main() {
    let application = Application::new(Some("image.viewer"), Default::default())
        .expect("failed to initialize GTK application");

    let state = Arc::new(Mutex::new(State {
        current_index: None,
        images: None,
    }));

    build_ui(&application, state);

    application.run(&[]);
}
