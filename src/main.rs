use gtk4::prelude::*;
use nostr::key::public_key::PublicKey;
use serde::{Deserialize, Serialize};

fn main() -> anyhow::Result<()> {
    let mut config: Config = confy::load::<Config>("Nostrlight", "config")?;

    let application = gtk4::Application::new(Some("dev.haruki7049.nostrlight"), Default::default());

    application.connect_activate(build_ui);

    application.run();

    Ok(())
}

fn build_ui(application: &gtk4::Application) {
    let window = gtk4::ApplicationWindow::new(application);

    window.set_title(Some("Nostrlight"));
    window.set_default_size(350, 70);

    let vbox = build_vbox();

    window.set_child(Some(&vbox));
    vbox.append(&build_button());
    vbox.append(&build_button());

    window.show();
}

fn build_vbox() -> gtk4::Box {
    gtk4::Box::builder()
        // アイテムを縦に並べる
        .orientation(gtk4::Orientation::Vertical)
        // アイテムを左(始端)寄せする
        .halign(gtk4::Align::Start)
        // 見た目を整える
        .spacing(6)
        .margin_bottom(6)
        .margin_top(6)
        .margin_start(6)
        .margin_end(6)
        //ビルドする
        .build()
}

fn build_button() -> gtk4::Button {
    let button = gtk4::Button::with_label("Click me!");
    button.connect_clicked(|_| {
        println!("Clicked!");
    });
    button
}

#[derive(Serialize, Deserialize, Debug)]
struct Config {
    pubkey: PublicKey,
}

impl std::default::Default for Config {
    fn default() -> Self {
        Self {
            pubkey: PublicKey::from_byte_array([
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0,
            ]),
        }
    }
}
