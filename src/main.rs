use gio::prelude::*;
use gtk::prelude::*;
use gtk::{
    Application, ApplicationWindow, Button, FileChooserAction, FileChooserDialog, Grid, Image,
};

fn build_ui(application: &Application) {
    application.connect_activate(|app| {
        let window = ApplicationWindow::new(app);
        window.set_title("Simple Image Viewer");
        window.set_default_size(500, 500);

        let grid = Grid::new();
        window.add(&grid);

        let image_panel = Image::new();
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
        let window_weak = window.downgrade();
        select_folder_button.connect_clicked(move |_| {
            let window = window_weak
                .upgrade()
                .expect("could not upgrade window reference");
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
                println!("{:?}", folder);
            }

            folder_selector.destroy();
        });
        grid.attach(&select_folder_button, 0, 2, 1, 1);

        window.show_all();
    });
}

fn main() {
    let application = Application::new(Some("image.viewer"), Default::default())
        .expect("failed to initialize GTK application");

    build_ui(&application);

    application.run(&[]);
}
