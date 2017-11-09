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
use std::io::{Read, Write};
use std::path::Path;
use std::fs::{self, DirBuilder, File};

use lib::utils::{dir_size_recursive, get_config_dir, pretty_bytes};

use NAME;

use gio;
use gtk::{self, SettingsExt};
use gio::prelude::*;
use gtk::prelude::*;


#[derive(Deserialize, Serialize, Debug)]
struct Config {
    general: General,
    video: Video,
}

#[derive(Deserialize, Serialize, Debug)]
struct General {
    dark_mode: bool,
}

#[derive(Deserialize, Serialize, Debug)]
struct Video {
    preferred_resolution: String,
}

macro_rules! write_setting {
    ($section:tt, $key:tt, $value:ident) => {{
        let config_path = format!(
            "{}/config.toml",
            get_config_dir(),
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
    let dark_mode_switch: gtk::Switch = builder.get_object("toggle_dark_mode_switch").unwrap();
    let preferences_win: gtk::Window = builder.get_object("window").unwrap();
    let clear_cache_button: gtk::Button = builder.get_object("clear_cache_button").unwrap();
    let clear_cache_label: gtk::Label = builder.get_object("clear_cache_label").unwrap();

    let settings = gtk::Settings::get_default().unwrap();

    preferences_win.set_attached_to(main_win);
    preferences_win.set_transient_for(main_win);
    preferences_win.set_modal(true);
    preferences_win.set_wmclass(NAME, NAME);

    let preferences_win_clone = preferences_win.clone();
    preferences_win.connect_delete_event(move |_, _| {
        preferences_win_clone.hide();
        Inhibit(true)
    });

    initialize_settings(&settings, &dark_mode_switch);

    let clear_cache_label_clone = clear_cache_label.clone();
    let clear_cache_button_clone = clear_cache_button.clone();
    settings_action.connect_activate(move |_, _| {
        update_cache(&clear_cache_label_clone, &clear_cache_button_clone);
        preferences_win.show_all();
    });

    let clear_cache_button_clone_2 = clear_cache_button.clone();
    clear_cache_button.connect_clicked(move |_| {
        let cache_dir = format!("{}/cache", get_config_dir());
        match fs::read_dir(&cache_dir) {
            Ok(entries) => {
                for entry in entries {
                    fs::remove_dir_all(entry.unwrap().path()).unwrap();
                }
            }
            Err(e) => eprintln!("Error deleting {:?}, caused by I/O error: {}", cache_dir, e),
        };
        update_cache(&clear_cache_label, &clear_cache_button_clone_2);
    });

    dark_mode_switch.connect_state_set(move |_, _| {
        if settings.get_property_gtk_application_prefer_dark_theme() {
            settings.set_property_gtk_application_prefer_dark_theme(false);
        } else {
            settings.set_property_gtk_application_prefer_dark_theme(true);
        }
        let dark_mode_value = settings.get_property_gtk_application_prefer_dark_theme();
        write_setting!(general, dark_mode, dark_mode_value);
        Inhibit(false)
    });
}

fn update_cache(label: &gtk::Label, button: &gtk::Button) {
    let cache_dir = format!("{}/cache", get_config_dir());
    let cache_dir = Path::new(&cache_dir);
    let dir_size = dir_size_recursive(cache_dir);
    let label_text = if dir_size < 4097 {
        button.set_sensitive(false);
        "Cache (0 B):".to_string()
    } else {
        button.set_sensitive(true);
        format!("Cache ({}):", pretty_bytes(dir_size as f64))
    };
    label.set_text(&label_text);
}

fn initialize_settings(settings: &gtk::Settings, dark_mode_switch: &gtk::Switch) {
    let config_path = format!("{}/config.toml", get_config_dir(),);

    let mut config_file =
        File::open(&config_path).unwrap_or_else(|e| panic!("Could not read file: {}", e));

    let mut config_string = String::new();
    config_file.read_to_string(&mut config_string).unwrap();

    match toml::from_str(&config_string) {
        Ok(config) => {
            let config: Config = config;
            settings.set_property_gtk_application_prefer_dark_theme(config.general.dark_mode);
            if config.general.dark_mode {
                dark_mode_switch.set_state(true);
            }
        }
        Err(_) => {
            eprintln!("WARNING: Invalid config file. It was automatically deleted.");
            fs::remove_file(config_path).unwrap_or_else(|x| {
                eprintln!("Unable to delete file: {:?}", x)
            });
            initialize_config_file();
        }
    };
}

fn initialize_config_file() {
    let config_path = get_config_dir();
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
            general: General { dark_mode: false },
            video: Video { preferred_resolution: String::from("720p") },
        };

        let config_toml = Value::try_from(config_data).unwrap();
        let config_string = config_toml.to_string();

        config_file.write_all(config_string.as_bytes()).unwrap();
    }
}
