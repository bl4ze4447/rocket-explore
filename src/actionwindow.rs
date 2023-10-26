use std::fs;
use std::fs::File;
use eframe::{ egui };
use egui::{Context , TextBuffer};
use crate::copy_action::CopyAction;
use crate::create_action::CreateAction;
use crate::delete_action::DeleteAction;
use crate::error_modal::ErrorModal;

use crate::pathinfo::PathInfo;
use crate::radiostate::RadioState;
use crate::fileopener::{open_file};
use crate::language_strings::{LangKey, LangString};
use crate::move_action::MoveAction;
use crate::open_action::OpenAction;
use crate::rename_action::RenameAction;
use crate::select_action::SelectionResult;

pub fn err_win(error_modal: &mut ErrorModal, ctx: &Context) {
    if error_modal.show {
        egui::Window::new(&error_modal.title).show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.label(&error_modal.caption);
                ui.separator();
                ui.vertical_centered_justified(|ui| {
                    if ui.button("Ok").clicked() {
                        error_modal.show = false;
                    }
                });
            });
        });
    }
}

pub fn create_win(create_action: &mut CreateAction, radio_state: &mut RadioState, lang_string: &LangString, ctx: &Context) {
    if create_action.show_window {
        egui::Window::new(lang_string.get(LangKey::CreateNew)).show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.add(egui::TextEdit::singleline(&mut create_action.new_file_name).hint_text(lang_string.get(LangKey::NewName)));
                if radio_state.create_win.file {
                    ui.add(egui::TextEdit::singleline(&mut create_action.extension).hint_text(lang_string.get(LangKey::ExtensionOptional)));
                }

                ui.horizontal(|ui| {
                    ui.centered_and_justified(|ui| {
                        if ui.radio(radio_state.create_win.file, lang_string.get(LangKey::File)).clicked() {
                            radio_state.create_win.folder = false;
                            radio_state.create_win.file = true;
                        }
                        if ui.radio(radio_state.create_win.folder, lang_string.get(LangKey::Folder)).clicked() {
                            radio_state.create_win.folder = true;
                            radio_state.create_win.file = false;
                        }
                    });
                });

                ui.separator();

                ui.horizontal(|ui| {
                    ui.centered_and_justified(|ui| {
                        if ui.button(lang_string.get(LangKey::Create)).clicked() {
                            let mut new_type = create_action.file.clone();
                            new_type.push(create_action.new_file_name.clone());
                            if radio_state.create_win.folder {
                                if let Err(caption) = fs::create_dir(new_type) {
                                    create_action.error_modal.set(caption.to_string(), true);
                                }
                            } else {
                                let new_file = new_type.to_string_lossy().to_string() + "." + create_action.extension.clone().as_str();
                                if let Err(caption) = File::create(new_file) {
                                    create_action.error_modal.set(caption.to_string(), true);
                                }
                            }

                            create_action.clear();
                        }
                        if ui.button(lang_string.get(LangKey::Cancel)).clicked() {
                            create_action.clear();
                        }
                    });
                });
            });
        });
    }
}

pub fn rename_win(rename_action: &mut RenameAction, lang_string: &LangString, ctx: &Context) {
    if rename_action.show_window {
        egui::Window::new(lang_string.get(LangKey::RenameFile)).show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.spacing_mut().item_spacing.y = 10.0;
                ui.label(lang_string.get(LangKey::OldName) + rename_action.file.file_name().unwrap().to_string_lossy().as_str());
                ui.spacing_mut().item_spacing.y = 20.0;
                ui.add(egui::TextEdit::singleline(&mut rename_action.name_after_rename).hint_text(lang_string.get(LangKey::NewName)));
                ui.spacing_mut().item_spacing.y = 5.0;

                ui.separator();

                ui.horizontal(|ui| {
                    ui.centered_and_justified(|ui| {
                        if ui.button(lang_string.get(LangKey::Rename)).clicked() {

                            let mut new_path = rename_action.file.clone();
                            new_path.pop();
                            new_path.push(&rename_action.name_after_rename);

                            if let Err(caption) = fs::rename(rename_action.file.clone(), new_path) {
                                rename_action.error_modal.set(caption.to_string(), true);
                            }

                            rename_action.clear();
                        }
                        if ui.button(lang_string.get(LangKey::Cancel)).clicked() {
                            rename_action.clear();
                        }
                    })
                });
            });
        });
    }
}

pub fn copy_win(copy_action: &mut CopyAction, ctx: &Context) {
    if copy_action.show_window {
        egui::Window::new("Copy").show(ctx, |ui| {
            match copy_action.from.clone() {
                SelectionResult::Single(file) => {
                    if let Some(file_name) = file.file_name() {
                        ui.label("Files: ".to_owned() + PathInfo::strip_win_prefix(&file.to_string_lossy()).as_str());
                        if copy_action.paste {
                            // Create the new filename path
                            let mut new_path = copy_action.to.clone();
                            new_path.push(file_name);

                            if let Err(caption) = fs::copy(file, new_path) {
                                copy_action.error_modal.set(caption.to_string(), true);
                                copy_action.clear();
                            }
                        }
                    } else {
                        copy_action.error_modal.set("Cannot copy root; if you are not doing that please report it since it may be a bug.".to_owned(), true);
                        copy_action.clear();
                    }

                    if copy_action.paste {
                        copy_action.clear();
                    }
                }
                SelectionResult::Multiple(files) => {
                    for file in &files {
                        if let Some(file_name) = file.file_name() {
                            ui.label("Files: ".to_owned() + PathInfo::strip_win_prefix(&file.to_string_lossy()).as_str());
                            if copy_action.paste {
                                // Create the new filename path
                                let mut new_path = copy_action.to.clone();
                                new_path.push(file_name);

                                if let Err(caption) = fs::copy(file, new_path) {
                                    copy_action.error_modal.set(caption.to_string(), true);
                                }
                            }
                        } else {
                            copy_action.error_modal.set("Cannot copy root; if you are not doing that please report it since it may be a bug.".to_owned(), true);
                        }
                    }

                    if copy_action.paste {
                        copy_action.clear();
                    }
                }
                SelectionResult::Err(caption) => {
                    copy_action.error_modal.set(caption, true);
                    copy_action.clear();
                }
            }

            ui.vertical_centered_justified(|ui| {
                if ui.button("Cancel").clicked() {
                    copy_action.clear();
                }
            });
        });
    }
}

pub fn move_win(move_action: &mut MoveAction, ctx: &Context) {
    if move_action.show_window {
        egui::Window::new("Move").show(ctx, |ui| {
            match move_action.from.clone() {
                SelectionResult::Single(file) => {
                    if let Some(file_name) = file.file_name() {
                        ui.label("Files: ".to_owned() + PathInfo::strip_win_prefix(&file.to_string_lossy()).as_str());
                        if move_action.paste {
                            // Create the new filename path
                            let mut new_path = move_action.to.clone();
                            new_path.push(file_name);

                            if let Err(caption) = fs::copy(file, new_path) {
                                move_action.error_modal.set(caption.to_string(), true);
                                move_action.clear();
                            }
                        }
                    } else {
                        move_action.error_modal.set("Cannot copy root; if you are not doing that please report it since it may be a bug.".to_owned(), true);
                        move_action.clear();
                    }

                    if move_action.paste {
                        move_action.clear();
                    }
                }
                SelectionResult::Multiple(files) => {
                    for file in &files {
                        if let Some(file_name) = file.file_name() {
                            ui.label("From: ".to_owned() + PathInfo::strip_win_prefix(&file.to_string_lossy()).as_str());
                            if move_action.paste {
                                // Create the new filename path
                                let mut new_path = move_action.to.clone();
                                new_path.push(file_name);

                                if let Err(caption) = fs::copy(file, new_path) {
                                    move_action.error_modal.set(caption.to_string(), true);
                                } else {
                                    match file.metadata() {
                                        Ok(metadata) => {
                                            if metadata.is_dir() {
                                                if let Err(caption) = fs::remove_dir_all(file) {
                                                    move_action.error_modal.set(caption.to_string(), true);
                                                }
                                            }
                                            if metadata.is_file() {
                                                if let Err(caption) = fs::remove_file(file) {
                                                    move_action.error_modal.set(caption.to_string(), true);
                                                }
                                            }
                                        }
                                        Err(caption) => { move_action.error_modal.set(caption.to_string(), true); }
                                    }
                                }
                            }
                        } else {
                            move_action.error_modal.set("Cannot copy root; if you are not doing that please report it since it may be a bug.".to_owned(), true);
                        }
                    }

                    if move_action.paste {
                        move_action.clear();
                    }                }
                SelectionResult::Err(caption) => {
                    move_action.error_modal.set(caption, true);
                    move_action.clear();
                }
            }

            ui.vertical_centered_justified(|ui| {
                if ui.button("Cancel").clicked() {
                    move_action.clear();
                }
            });
        });
    }
}

pub fn delete_win(delete_action: &mut DeleteAction, ctx: &Context) {
    if delete_action.show_window {
        egui::Window::new("Deleting").show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                match &delete_action.file_list {
                    SelectionResult::Single(file) => {
                        if let Ok(metadata) = file.metadata() {
                            if metadata.is_dir() {
                                if let Err(caption) = fs::remove_dir_all(file) {
                                    delete_action.error_modal.set(caption.to_string(), true);
                                }
                            } else {
                                if let Err(caption) = fs::remove_file(file) {
                                    delete_action.error_modal.set(caption.to_string(), true);
                                }
                            }
                        }
                    }
                    SelectionResult::Multiple(files) => {
                        for file in files {
                            if let Ok(metadata) = file.metadata() {
                                if metadata.is_dir() {
                                    if let Err(caption) = fs::remove_dir_all(file) {
                                        delete_action.error_modal.set(caption.to_string(), true);
                                    }
                                } else {
                                    if let Err(caption) = fs::remove_file(file) {
                                        delete_action.error_modal.set(caption.to_string(), true);
                                    }
                                }
                            }
                        }
                    }
                    SelectionResult::Err(caption) => {
                        delete_action.error_modal.set(caption.clone(), true);
                    }
                }

                delete_action.clear();


                ui.vertical_centered_justified(|ui| {
                    if ui.button("Cancel").clicked() {
                        delete_action.clear();
                    }
                });
            });
        });
    }
}

pub fn open_win(open_action: &mut OpenAction, lang_string: &LangString) {
    if open_action.open {
        match &open_action.file_list {
            SelectionResult::Single(file) => {
                if let Err(caption) = open_file(&file, lang_string) {
                    open_action.error_modal.set(caption.to_string(), true);
                }
            }
            SelectionResult::Multiple(files) => {
                for file in files {
                    if let Err(caption) = open_file(&file, lang_string) {
                        open_action.error_modal.set(caption.to_string(), true);
                    }
                }
            }
            SelectionResult::Err(caption) => {
                open_action.error_modal.set(caption.clone(), true);
                open_action.clear();
            }
        }

        open_action.clear();
    }
}

