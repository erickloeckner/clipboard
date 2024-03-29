extern crate gdk;
extern crate gio;
extern crate gtk;

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::process;
use std::env::{self, args};

use gio::prelude::*;
use gtk::prelude::*;

#[derive(Debug)]
struct ButtonConf {
    name: String,
    cmd:  String,
}

fn main() {
    let application =
        gtk::Application::new(Some("com.github.gtk-rs.examples.basic"), Default::default())
            .expect("Initialization failed...");

    application.connect_activate(|app| {
        build_ui(app);
    });

    application.run(&args().collect::<Vec<_>>());
}

fn build_ui(application: &gtk::Application) {
    let mut cwd = env::current_exe().unwrap();
    for _i in 0..3 { cwd.pop(); }
    cwd.push("clipboard.conf");
    
    let lines = read_lines(&cwd).unwrap_or_else(|err| {
        println!("Error parsing conf: {}", err);
        process::exit(1);
    });
    cwd.pop();
    
    let buttons = parse_conf(lines);
    
    let window = gtk::ApplicationWindow::new(application);

    window.set_title("clipboard");
    //~ window.set_border_width(10);
    window.set_position(gtk::WindowPosition::Center);
    //~ window.set_default_size(360, 240);
    
    let hbox = gtk::Box::new(gtk::Orientation::Horizontal, 0);

    for i in buttons {
        let button = gtk::Button::new_with_label(&i.name);
        //~ let cmd_clone = i.cmd.clone();
        
        button.connect_clicked(move |_| {
            let clipboard = gtk::Clipboard::get(&gdk::SELECTION_CLIPBOARD);
            clipboard.set_text(&i.cmd);
        });

        //~ window.add(&button);
        hbox.pack_start(&button, true, true, 0);
    }

    window.add(&hbox);
    window.show_all();
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn parse_conf(conf: io::Lines<io::BufReader<File>>) -> Vec<ButtonConf> {
    let mut conf_out: Vec<ButtonConf> = Vec::new();
    for line in conf {
        let result = line.unwrap();
        if result.len() > 0 && !result.trim_start().starts_with("#") {
            let result_split: Vec<&str> = result.split("::").collect();
            if result_split.len() > 1 {
                conf_out.push(ButtonConf {
                    name: result_split[0].to_string(),
                    cmd:  result_split[1].to_string().replace("\\n", "\x0A"),
                });
            }
        }
    }
    conf_out
}
