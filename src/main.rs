mod filemanager;
mod searchinfo;
mod pathinfo;
mod settings;
mod radiostate;
mod language_strings;
mod file_widget;
mod harddisk_widget;
mod file_action;
mod error_modal;
mod open_action;
mod copy_action;
mod create_action;
mod delete_action;
mod fileopener;
mod move_action;
mod rename_action;
mod select_action;
mod actionwindow;

use std::path::PathBuf;
use std::sync::mpsc;
use std::time::Duration;
// egui libraries
use eframe::{egui, HardwareAcceleration, Theme};
use egui::{Context, CursorIcon, Id, Image, ImageSource, include_image, Key, Label, Sense, TextBuffer, TextStyle, Ui, Vec2, WidgetText};
use egui_extras;

// Standard library
use chrono::{DateTime, Local};
use crate::actionwindow::{copy_win, create_win, delete_win, err_win, move_win, open_win, rename_win};
use crate::file_action::FileAction;

use crate::pathinfo::{PathInfo};
use crate::radiostate::RadioState;
use crate::searchinfo::SearchInfo;
use crate::settings::Settings;
use crate::file_widget::file_widget;
use crate::filemanager::{ get_display_size};
use crate::harddisk_widget::harddisk_widget;
use crate::language_strings::{LangKey, LangString};
use crate::select_action::{SelectAction, SelectionMode};

fn main() -> eframe::Result<()> {
    let native_options = eframe::NativeOptions {
        always_on_top: false,
        maximized: true,
        decorated: true,
        fullscreen: false,
        drag_and_drop_support: true,
        icon_data: None,
        initial_window_pos: None,
        initial_window_size: None,
        min_window_size: Some(Vec2::new(800.0, 600.0)),
        max_window_size: None,
        resizable: true,
        transparent: false,
        mouse_passthrough: false,
        active: true,
        vsync: true,
        hardware_acceleration: HardwareAcceleration::Preferred,
        follow_system_theme: false,
        default_theme: Theme::Dark,
        run_and_return: false,
        centered: true,
        ..Default::default()
    };

    eframe::run_native("Rocket Explore", native_options, Box::new(|cc| Box::new(RocketExplore::new(cc))))
}

struct RocketExplore {
    path_info:      PathInfo,
    search_info:    SearchInfo,
    file_action:    FileAction,
    radio_state:    RadioState,
    settings:       Settings,
    lang_string:    LangString,
}

impl RocketExplore {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        let lang_string = LangString::new();
        Self {
            path_info:          PathInfo::new(&lang_string),
            search_info:        SearchInfo::new(),
            file_action:        FileAction::new(),
            radio_state:        RadioState::new(),
            settings:           Settings::new(),
            lang_string,
        }
    }
}

impl eframe::App for RocketExplore {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        egui_extras::install_image_loaders(ctx); // Installs only once per context
        let folder_img = include_image!(".\\Icons\\Directory\\default_folder.png");
        let file_img = include_image!(".\\Icons\\Files\\default_file.png");
        // Check if search stopped
        self.search_info.searching = if self.search_info.rx_finished.try_recv().is_ok() { false } else { self.search_info.searching };
        self.file_action.select_action.check();
        ctx.request_repaint_after(Duration::from_millis(500));

        // LEFT PANEL
        egui::SidePanel::left(Id::new("Navigation_Panel")).resizable(true).show(ctx, |ui| {
        });

        // TOP PANEL
        egui::TopBottomPanel::top(Id::new("Navigation_Bar")).show(ctx, |ui| {
            ui.horizontal(|ui| {
                if ui.button("Go back").clicked() {
                    if self.search_info.display_search_content {
                        self.search_info.display_search_content = false;
                    } else if self.path_info.get_current_path().pop() {
                        self.path_info.update_current_path(&PathBuf::from(self.path_info.get_current_path().parent().unwrap())); // Unwrap since we could pop = we have parent
                    } else {
                        self.path_info.show_dir_content     =   false;
                        self.path_info.show_device_content  =   true;
                    }
                }
                if ui.button("Go forward").clicked() {
                    // TODO
                }
            });
        });

        // BOTTOM PANEL
        egui::TopBottomPanel::bottom(Id::new("Display_Path_Panel")).show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.add(Image::from(folder_img.clone()).fit_to_exact_size(Vec2::new(64.0, 64.0)));
                if let Some(name) = self.path_info.get_current_path().file_name() {
                    ui.add(Label::new(WidgetText::from(name.to_string_lossy()).text_style(TextStyle::Heading)));
                } else if !self.path_info.show_dir_content {
                    ui.add(Label::new(WidgetText::from(self.lang_string.get(LangKey::RootPath)).text_style(TextStyle::Heading)));
                } else {
                    ui.add(Label::new(WidgetText::from(PathInfo::strip_win_prefix(&self.path_info.get_current_path().to_string_lossy())).text_style(TextStyle::Heading)));
                }
                let searchbar = ui.add(egui::TextEdit::singleline(&mut self.search_info.filename_to_search).hint_text("Search"));
                if searchbar.lost_focus() {
                    searchbar.ctx.input(|inp| {
                        if inp.key_pressed(Key::Enter) {
                            if !self.search_info.filename_to_search.trim_end().is_empty() { // If the search is not empty and we did not already search for it
                                if self.search_info.searching {
                                    if let Ok(_) = self.search_info.stop_search() {
                                        self.search_info.search(self.path_info.show_device_content, self.path_info.get_current_path(),
                                                                self.path_info.connected_devices.clone(), self.settings.search.selected_thread_count);
                                    }
                                } else {
                                    let new_path_channel = mpsc::channel();
                                    self.search_info.tx_path = new_path_channel.0;
                                    self.search_info.rx_path = new_path_channel.1;
                                    self.search_info.search(self.path_info.show_device_content, self.path_info.get_current_path(),
                                                            self.path_info.connected_devices.clone(), self.settings.search.selected_thread_count);
                                }
                            } else if !self.search_info.filename_to_search.is_empty() { // If we already did that search
                                self.search_info.display_search_content = true;
                            }
                        }
                    });
                }
                if let Ok(message) = self.search_info.rx_path.try_recv() {
                    self.search_info.files.push(message);
                }
            })
        });

        // RIGHT PANEL
        egui::SidePanel::right(Id::new("Additional_Info_Panel")).max_width(300.0).min_width(200.0).show(ctx, |ui| {
            rightpanel_additional_info(ui, &self.lang_string, &mut self.file_action.select_action, &folder_img, &file_img);
        });

        // CENTRAL PANEL
        if egui::CentralPanel::default().show(ctx, |ui| {
            if self.search_info.display_search_content {
                central_search_content(ui, &mut self.search_info, &mut self.path_info,  &folder_img, &file_img, &self.lang_string, &mut self.file_action, self.settings.search.selected_thread_count);
                central_action(ctx, &mut self.file_action, &mut self.radio_state, &self.lang_string);
            } else if self.path_info.show_dir_content {
                central_directory_content(ui, &mut self.lang_string, &mut self.path_info, &mut self.file_action, &folder_img, &file_img);
                central_action(ctx, &mut self.file_action, &mut self.radio_state, &self.lang_string);
            } else if self.path_info.show_device_content {
                central_device_content(ui, &mut self.path_info);
            }
        }).response.context_menu(|ui| {
            central_ctx_buttons(ui, &self.lang_string, &mut self.file_action, &self.path_info);
        }).interact(Sense::click()).clicked() {
            self.file_action.select_action.files.clear();
        }
    }
}

fn central_device_content(ui: &mut Ui, path_info: &mut PathInfo) {
    path_info.search_for_devices();

    ui.horizontal(|ui| {
        for device in path_info.connected_devices.clone() {
            path_info.disk_space.recalculate(device.to_string_lossy().as_str());
            let dev = harddisk_widget(ui, false, PathInfo::strip_win_prefix(&device.to_string_lossy()).as_str(),
                                      get_display_size(path_info.disk_space.free as u64).to_string().as_str(),
                                      get_display_size(path_info.disk_space.total as u64).to_string().as_str());

            if dev.double_clicked() {
                path_info.update_current_path(&device);
                path_info.show_dir_content      =   true;
                path_info.show_device_content   =   false;
                return;
            }
        }
    });
}

fn directory_builder(ui: &mut Ui, content: Vec<PathBuf>, file_action: &mut FileAction, path_info: &mut PathInfo, lang_string: &LangString, folder_img: &ImageSource, file_img: &ImageSource) -> bool {
    let total_rows = content.len();
    let row_height = ui.spacing().interact_size.y * 2.0;
    let mut modified = false;
    egui::ScrollArea::both().show_rows(ui, row_height, total_rows, |ui, row_range| {
        let mut files = content[row_range].to_vec();
        for file in &mut files {
            if let Some(filename) = file.file_name() {
                ui.horizontal(|ui| {
                    if let Ok(metadata) = file.metadata() {
                        if metadata.is_dir() {
                            ui.add(Image::new(folder_img.clone()).fit_to_exact_size(Vec2::new(32.0, 32.0)));
                        } else {
                            ui.add(Image::new(file_img.clone()).fit_to_exact_size(Vec2::new(32.0, 32.0)));
                        }
                        ui.vertical_centered_justified(|ui| {
                            let response = file_widget(ui, file_action.select_action.is_selected(file), filename.to_string_lossy().as_str());
                            response.clone().context_menu(|ui| {
                                if !file_action.select_action.is_selected(file) {
                                    file_action.select_action.mode = SelectionMode::SINGLE;
                                    file_action.select_action.manage_selection(file, &content);
                                }
                                central_file_ctx_buttons(ui, &lang_string, file_action);
                            });
                            if response.double_clicked() {
                                if metadata.is_dir() {
                                    if let Ok(new_path) = file.canonicalize() {
                                        path_info.update_current_path(&new_path);
                                        modified = true;
                                        return;
                                    } else {
                                        file_action.open_action.error_modal.set(lang_string.get(LangKey::CantOpenSystemDir), true);
                                    }
                                }
                                if metadata.is_file() {
                                    file_action.open_action.open = true;
                                    file_action.open_action.file_list = file_action.select_action.get_selection();
                                }
                            }

                            if response.clicked() {
                                response.ctx.input(|inp| {
                                    if inp.modifiers.ctrl && file_action.select_action.mode == SelectionMode::SINGLE {
                                        file_action.select_action.switch_mode();
                                    } else if !inp.modifiers.ctrl && file_action.select_action.mode == SelectionMode::MULTIPLE {
                                        file_action.select_action.switch_mode();
                                    }
                                    if inp.modifiers.shift && file_action.select_action.mode == SelectionMode::SINGLE {
                                        file_action.select_action.mode = SelectionMode::RANGED;
                                    } else if !inp.modifiers.shift && file_action.select_action.mode == SelectionMode::RANGED {
                                        file_action.select_action.mode = SelectionMode::SINGLE;
                                    }
                                });

                                file_action.select_action.manage_selection(file, &content);
                            }

                        });

                        if modified {
                            return;
                        }
                    }
                });

                if modified {
                    return;
                }
            }
        }
    });

    return modified;
}

fn central_directory_content(ui: &mut Ui, lang_string: &LangString, path_info: &mut PathInfo, file_action: &mut FileAction, folder_img: &ImageSource, file_img: &ImageSource) {
    if path_info.update_directory { // First frame we update the cursor, then perform the fill operation
        if !path_info.cursor_set {
            ui.ctx().set_cursor_icon(CursorIcon::Wait);
            path_info.cursor_set = true;
            return;
        }

        path_info.fill_directory_content();
        ui.ctx().set_cursor_icon(CursorIcon::Default);
    }

    directory_builder(ui, path_info.current_directory_content.clone(), file_action, path_info, lang_string, folder_img, file_img);
}
fn central_search_content(ui: &mut Ui, search_info: &mut SearchInfo, path_info: &mut PathInfo, folder_img: &ImageSource, file_img: &ImageSource, lang_string: &LangString, file_action: &mut FileAction, threads: u8) {
    if search_info.searching {
        if path_info.show_dir_content {
            ui.label(lang_string.get(LangKey::SearchingIn) + PathInfo::strip_win_prefix(&path_info.get_current_path().to_string_lossy()).as_str());
        } else {
            ui.label(lang_string.get(LangKey::SearchingAll));
        }
    } else {
        ui.label(lang_string.get(LangKey::SearchFinished));
        ui.separator();
        if search_info.files.is_empty() {
            ui.vertical_centered(|ui| {
                ui.group(|ui| {
                    ui.label(lang_string.get(LangKey::CheckSpelling));
                    if !path_info.show_device_content {
                        if ui.button(lang_string.get(LangKey::CheckOr)).clicked() {
                            path_info.show_device_content = true;
                            search_info.search(path_info.show_device_content, path_info.get_current_path(),
                                               path_info.connected_devices.clone(), threads);
                        }
                    }
                });
            });
            return;
        }
    }

    if directory_builder(ui, search_info.files.clone(), file_action, path_info, lang_string, folder_img, file_img) {
        search_info.display_search_content = false;
    }
}

fn central_action(ctx: &Context, file_action: &mut FileAction, radio_state: &mut RadioState, lang_string: &LangString) {
    open_win(&mut file_action.open_action, lang_string);
    delete_win(&mut file_action.delete_action, ctx);
    move_win(&mut file_action.move_action, ctx);
    copy_win(&mut file_action.copy_action, ctx);
    rename_win(&mut file_action.rename_action, lang_string, ctx);
    create_win(&mut file_action.create_action, radio_state, lang_string, ctx);

    err_win(&mut file_action.open_action.error_modal, ctx);
    err_win(&mut file_action.delete_action.error_modal, ctx);
    err_win(&mut file_action.move_action.error_modal, ctx);
    err_win(&mut file_action.copy_action.error_modal, ctx);
    err_win(&mut file_action.rename_action.error_modal, ctx);
    err_win(&mut file_action.create_action.error_modal, ctx);
}

fn rightpanel_additional_info(ui: &mut Ui, lang_string: &LangString, select_action: &SelectAction, folder_icon: &ImageSource, file_icon: &ImageSource) {
    let folder_icon = egui::Image::from(folder_icon.clone()).fit_to_exact_size(Vec2::new(128.0, 128.0));
    let file_icon = egui::Image::from(file_icon.clone()).fit_to_exact_size(Vec2::new(128.0, 128.0));
    egui::ScrollArea::vertical().show(ui, |ui| {
        ui.vertical_centered(|ui| {
            if select_action.files.is_empty() {
                ui.label(lang_string.get(LangKey::NothingSelected));
            } else {
                for file in &select_action.files {
                    ui.group(|ui| {
                        if let Ok(metadata) = file.metadata() {
                            if let Some(filename) = file.file_name() {
                                ui.add(if metadata.is_dir() ||  metadata.is_symlink() { folder_icon.clone() } else { file_icon.clone() } );
                                ui.label(filename.to_string_lossy());
                                ui.group(|ui| {
                                    let bytes = metadata.len();
                                    if let Some(ext) = file.extension() {
                                        if !metadata.is_dir() {
                                            ui.label(lang_string.get(LangKey::Extension) + ext.to_string_lossy().as_str());
                                            ui.label(get_display_size(bytes));
                                        }
                                    }
                                    if let Ok(time) = metadata.created() {
                                        let string_time: DateTime<Local> = time.into();
                                        ui.label(lang_string.get(LangKey::CreatedAt) + string_time.format("%d/%m/%Y %T").to_string().as_str());
                                    }
                                    if let Ok(time) = metadata.accessed() {
                                        let string_time: DateTime<Local> = time.into();
                                        ui.label(lang_string.get(LangKey::AccessedAt) + string_time.format("%d/%m/%Y %T").to_string().as_str());
                                    }
                                    if let Ok(time) = metadata.modified() {
                                        let string_time: DateTime<Local> = time.into();
                                        ui.label(lang_string.get(LangKey::ModifiedAt) + string_time.format("%d/%m/%Y %T").to_string().as_str());
                                    }
                                });
                            }
                        }
                    });
                }
            }
        });
    });
}



fn central_file_ctx_buttons(ui: &mut Ui, lang_string: &LangString, file_action: &mut FileAction) {
    if ui.button(lang_string.get(LangKey::Open)).clicked() {
        file_action.open_action.open = true;
        file_action.open_action.file_list = file_action.select_action.get_selection();
        ui.close_menu();
    }
    if ui.button(lang_string.get(LangKey::Delete)).clicked() {
        file_action.delete_action.file_list = file_action.select_action.get_selection();
        file_action.delete_action.show_window = true;
        ui.close_menu();
    }
    if ui.button(lang_string.get(LangKey::Rename)).clicked() {
        file_action.rename_action.file_list = file_action.select_action.get_selection();
        file_action.rename_action.show_window = true;
    }
    if ui.button(lang_string.get(LangKey::Copy)).clicked() {
        file_action.copy_action.show_window = true;
        file_action.copy_action.from = file_action.select_action.get_selection();
        ui.close_menu();
    }
    if ui.button(lang_string.get(LangKey::Move)).clicked() {
        file_action.move_action.show_window = true;
        file_action.move_action.from = file_action.select_action.get_selection();
        ui.close_menu();
    }
}
fn central_ctx_buttons(ui: &mut Ui, lang_string: &LangString, file_action: &mut FileAction, path_info: &PathInfo) {
    if ui.button(lang_string.get(LangKey::CreateNew)).clicked() {
        file_action.create_action.show_window = true;
        file_action.create_action.file = path_info.get_current_path();
        file_action.create_action.show_window = true;
        ui.close_menu();
    }
    if file_action.copy_action.show_window && file_action.move_action.show_window {
        ui.menu_button(lang_string.get(LangKey::PasteFrom), |ui| {
           if ui.button(lang_string.get(LangKey::Copy)).clicked() {
               file_action.copy_action.to = path_info.get_current_path();
               file_action.copy_action.paste = true;
               ui.close_menu();
           }
            if ui.button(lang_string.get(LangKey::Move)).clicked() {
                file_action.move_action.to = path_info.get_current_path();
                file_action.move_action.paste = true;
                ui.close_menu();
            }
        });
    } else {
        if ui.button(lang_string.get(LangKey::Paste)).clicked() {
           if file_action.copy_action.show_window {
               file_action.copy_action.to = path_info.get_current_path();
               file_action.copy_action.paste = true;
               ui.close_menu();
           }
            if file_action.move_action.show_window {
                file_action.move_action.to = path_info.get_current_path();
                file_action.move_action.paste = true;
                ui.close_menu();
            }
        }
    }
}

// ui.add(Image::from(folder_img.clone()).fit_to_exact_size(Vec2::new(64.0, 64.0)));
// let mut components = path.split('\\').peekable();
// while let Some(component) = components.next() {
// ui.label(component);
// if components.peek().is_some() {
// ui.label(" > ");
// }
// }