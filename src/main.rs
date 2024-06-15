use gtk::Align;
use gtk::Picture;
use gtk::Orientation;
use clap::Parser;
use palette_extract::Color;
use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, DrawingArea};
use gtk::cairo::{Context, Format, ImageSurface};
use gtk::glib::clone;
use palette_extract::get_palette_rgb;

/// Display Palette
#[derive(Parser, Clone, Debug)]
#[command(infer_subcommands = true, infer_long_args = true, author, version, about, long_about = None)]
pub struct Args {

    /// file to extract palette from
    #[arg(short, long)]
    pub filepath: String,
}


fn main() {
    let args = Args::parse();
    let filepath = &args.filepath;
    let application = Application::new(Some("com.example.gtk-image-squares"), Default::default());

    application.connect_activate(clone!(@strong filepath => move |application: &gtk::Application| {
        let window = ApplicationWindow::new(application);
        let image = image::open(filepath.clone()).expect("can't open image file");
        let pixels = image.as_bytes();
        let palette = get_palette_rgb(&pixels);

        window.set_title(Some("GTK4 Image Squares Example"));
        window.set_default_size(400, 400);

        let drawing_area = DrawingArea::new();
        drawing_area.set_content_width(400);
        drawing_area.set_content_height(40);

        drawing_area.set_draw_func(move |_, ctx, _, _| {
            draw_image(ctx, &palette);
        });
        let vbox = gtk::Box::new(Orientation::Vertical, 0);
        let picture = Picture::new();
        picture.set_filename(Some(filepath.clone()));
        picture.set_hexpand(true);
        picture.set_vexpand(true);
        vbox.set_valign(Align::Center);
        vbox.set_halign(Align::Center);
        vbox.set_vexpand(true);
        vbox.set_hexpand(true);
        vbox.append(&picture);
        vbox.append(&drawing_area);
        window.set_child(Some(&vbox));
        window.show();
        let evk = gtk::EventControllerKey::new();
        evk.connect_key_pressed(clone!(@strong window => move |_, key, _, _| {
            if let Some(s) = key.name() {
                match s.as_str() {
                    "Escape" => window.close(),
                    _ => { },
                }
            };
            gtk::Inhibit(false)
        }));
        window.add_controller(evk);
    }));

    let empty: Vec<String> = vec![];
    application.run_with_args(&empty);
}


fn draw_image(ctx: &Context, palette: &Vec<Color>) {
    let width = 400;
    let height = 400;
    let square_size = 40.0; // Size of each square

    let surface = ImageSurface::create(Format::ARgb32, width, height)
        .expect("Can't create surface");
    let context = Context::new(&surface).expect("Can't create context");

    for (i, color) in palette.iter().enumerate() {
        println!("{:?} {:?}", i, color);
        let red   = color.r as f64 / 255.0;
        let green = color.g as f64 / 255.0;
        let blue  = color.b as f64 / 255.0;
        context.set_source_rgb(red, green, blue);
        let x = (i as f64) * square_size;
        context.rectangle(x, 0.0, square_size, square_size);
        context.fill().expect("Can't fill rectangle");
    }

    ctx.set_source_surface(&surface, 0.0, 0.0)
        .expect("Can't set source surface");
    ctx.paint().expect("Can't paint surface");
}

