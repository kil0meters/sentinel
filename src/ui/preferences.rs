//  Copyright (C) 2017  Kil0meters
//
//  This program is free software: you can redistribute it and/or modify
//  it under the terms of the GNU General Public License as published by
//  the Free Software Foundation, either version 3 of the License, or
//  (at your option) any later version.
//
//  This program is distributed in the hope that it will be useful,
//  but WITHOUT ANY WARRANTY; without even the implied warranty of
//  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
//  GNU General Public License for more details.
//
//  You should have received a copy of the GNU General Public License
//  along with this program.  If not, see <https://www.gnu.org/licenses/>.

// "I'll write just a settings dialog real quick because that
// will be simpler than getting thumbnails to work."
//
// I was wrong.
// Send help.

use toml::{self, Value};
use std::{env, process};
use std::io::{Read, Write};
use std::path::Path;
use std::fs::{DirBuilder, File};

use {NAME, NAME_NOCAPS};

use gio;
use gtk::{self, SettingsExt};
use gio::prelude::*;
use gtk::prelude::*;


#[derive(Deserialize, Serialize, Debug)]
struct Config {
    appearance: Appearance,
    video: Video,
}

#[derive(Deserialize, Serialize, Debug)]
struct Appearance {
    dark_mode: bool,
}

#[derive(Deserialize, Serialize, Debug)]
struct Video {
    preferred_resolution: String,
}

macro_rules! write_setting {
    ($section:tt, $key:tt, $value:ident) => {{
        let config_path = format!(
            "{}/.config/{}/config.toml",
            env::var("HOME").unwrap(),
            NAME_NOCAPS
        );

        // Panicing here isn't ideal, but is probably fine since no one
        // will be deleting their config file while using the app, probably.
        let mut config_file = File::open(&config_path)
            .unwrap_or_else(|e| panic!("Could not read config file: {:?}", e));

        let mut config_string = String::new();
        config_file.read_to_string(&mut config_string).unwrap();

        let mut config: Config = toml::from_str(&config_string).unwrap();

        config.$section.$key = $value;

        let config_toml = Value::try_from(config).unwrap();
        let config_string  = config_toml.to_string();

        // Opens file in write only mode
        let mut config_file = File::create(config_path)
            .unwrap_or_else(|e| panic!("Could not write config file: {:?}", e));
        config_file.write_all(config_string.as_bytes()).unwrap();
    }}
}

pub fn initialize(settings_action: &gio::SimpleAction, main_win: &gtk::ApplicationWindow) {
    initialize_config_file();

    let builder = include_str!("../../data/ui/preferences.ui");
    let builder = gtk::Builder::new_from_string(builder);

    let settings = gtk::Settings::get_default().unwrap();

    let preferences_win: gtk::Window = builder.get_object("window").unwrap();
    preferences_win.set_attached_to(main_win);
    preferences_win.set_transient_for(main_win);
    preferences_win.set_modal(true);
    preferences_win.set_wmclass(NAME, NAME);

    let preferences_win_clone = preferences_win.clone();
    preferences_win.connect_delete_event(move |_, _| {
        preferences_win_clone.hide();
        Inhibit(true)
    });

    let dark_mode_switch: gtk::Switch = builder.get_object("toggle_dark_mode_switch").unwrap();

    initialize_settings(&settings, &dark_mode_switch);

    settings_action.connect_activate(move |_, _| {
        preferences_win.show_all();
    });

    dark_mode_switch.connect_state_set(move |_, _| {
        if settings.get_property_gtk_application_prefer_dark_theme() {
            settings.set_property_gtk_application_prefer_dark_theme(false);
        } else {
            settings.set_property_gtk_application_prefer_dark_theme(true);
        }
        let dark_mode_value = settings.get_property_gtk_application_prefer_dark_theme();
        write_setting!(appearance, dark_mode, dark_mode_value);
        Inhibit(false)
    });
}

fn initialize_settings(settings: &gtk::Settings, dark_mode_switch: &gtk::Switch) {
    let config_path = format!(
        "{}/.config/{}/config.toml",
        env::var("HOME").unwrap(),
        NAME_NOCAPS
    );

    let mut config_file =
        File::open(&config_path).unwrap_or_else(|e| panic!("Could not read file: {}", e));

    let mut config_string = String::new();
    config_file.read_to_string(&mut config_string).unwrap();

    let config: Config = match toml::from_str(&config_string) {
        Ok(ok) => ok,
        Err(_) => {
            println!("Invalid config file.\nTry deleting it at {}", config_path);
            process::exit(1)
        }
    };

    settings.set_property_gtk_application_prefer_dark_theme(config.appearance.dark_mode);
    if config.appearance.dark_mode {
        dark_mode_switch.set_state(true);
    }
}

fn initialize_config_file() {
    let config_path = format!("{}/.config/{}", env::var("HOME").unwrap(), NAME_NOCAPS);

    let config_file_path = format!("{}/config.toml", config_path);

    if !Path::new(&config_file_path).is_file() {
        DirBuilder::new()
            .recursive(true)
            .create(&config_path)
            .unwrap_or_else(|e| panic!("Could not create config directory: {:?}", e));

        let mut config_file = File::create(format!("{}/config.toml", &config_path))
            .unwrap_or_else(|e| panic!("Could not write to file: {:?}", e));

        // Default settings.
        let config_data = Config {
            appearance: Appearance { dark_mode: false },
            video: Video {
                preferred_resolution: String::from("720p"),
            },
        };

        let config_toml = Value::try_from(config_data).unwrap();
        let config_string = config_toml.to_string();

        config_file.write_all(config_string.as_bytes()).unwrap();
    }
}
