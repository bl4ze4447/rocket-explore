pub enum Language {
    Romanian,
    English,
}
pub enum LangKeys {
    RootPath,
    CantOpenSystemDir,
    NothingSelected,
    Extension,
    CreatedAt,
    AccessedAt,
    ModifiedAt,
    Open,
    Delete,
    Rename,
    Copy,
    Move,
    CreateNew,
    PasteFrom,
    Paste,
    CantCopyRoot,
    File,
    Folder,
    Create,
    Cancel,
    RenameFile,
    Files,
    From,
    Deleting,
    NewName,
    ExtensionOptional,
    OldName,
    FreeOf,
}

pub struct LangString {
    pub current_lang: Language,
}

impl LangString {
    pub fn new() -> Self {
        Self {
            current_lang: Language::English,
        }
    }
    pub fn get(&self, key: &LangKeys) -> String {
        return match self.current_lang {
            Language::English => {
                match key {
                    LangKeys::RootPath => String::from("Personal Computer"),
                    LangKeys::CantOpenSystemDir => String::from("Cannot open system directory"),
                    LangKeys::NothingSelected => String::from("Nothing selected"),
                    LangKeys::Extension => String::from("Extension: "),
                    LangKeys::CreatedAt => String::from("Created at: "),
                    LangKeys::AccessedAt => String::from("Accessed at: "),
                    LangKeys::ModifiedAt => String::from("Modified at: "),
                    LangKeys::Open => String::from("Open"),
                    LangKeys::Delete => String::from("Delete"),
                    LangKeys::Rename => String::from("Rename"),
                    LangKeys::Copy => String::from("Copy"),
                    LangKeys::Move => String::from("Move"),
                    LangKeys::CreateNew => String::from("Create new"),
                    LangKeys::PasteFrom => String::from("Paste from"),
                    LangKeys::Paste => String::from("Paste"),
                    LangKeys::CantCopyRoot => String::from("Cannot copy root; if this is a bug please report it"),
                    LangKeys::File => String::from("File"),
                    LangKeys::Folder => String::from("Folder"),
                    LangKeys::Create => String::from("Create"),
                    LangKeys::Cancel => String::from("Cancel"),
                    LangKeys::RenameFile => String::from("Rename file"),
                    LangKeys::Files => String::from("Files: "),
                    LangKeys::From => String::from("From: "),
                    LangKeys::Deleting => String::from("Deleting"),
                    LangKeys::NewName => String::from("New name"),
                    LangKeys::ExtensionOptional => String::from("Extension (optional): "),
                    LangKeys::OldName => String::from("Old name: "),
                    LangKeys::FreeOf => String::from(" free of "),
                }
            }
            Language::Romanian => {
                match key {
                    LangKeys::RootPath => String::from("Calculator personal"),
                    LangKeys::CantOpenSystemDir => String::from("Acest fisier de sistem nu poate fi deschis"),
                    LangKeys::NothingSelected => String::from("Nimic selectat"),
                    LangKeys::Extension => String::from("Extensie: "),
                    LangKeys::CreatedAt => String::from("Creat: "),
                    LangKeys::AccessedAt => String::from("Accesat: "),
                    LangKeys::ModifiedAt => String::from("Modificat: "),
                    LangKeys::Open => String::from("Deschide"),
                    LangKeys::Delete => String::from("Sterge"),
                    LangKeys::Rename => String::from("Redenumeste"),
                    LangKeys::Copy => String::from("Copiaza"),
                    LangKeys::Move => String::from("Muta"),
                    LangKeys::CreateNew => String::from("Meniu de creare"),
                    LangKeys::PasteFrom => String::from("Lipeste din"),
                    LangKeys::Paste => String::from("Lipeste"),
                    LangKeys::CantCopyRoot => String::from("Nu putem copia baza, daca e un bug raporteaza."),
                    LangKeys::File => String::from("Fisier"),
                    LangKeys::Folder => String::from("Folder"),
                    LangKeys::Create => String::from("Creeaza"),
                    LangKeys::Cancel => String::from("Opreste"),
                    LangKeys::RenameFile => String::from("Redenumeste fisier"),
                    LangKeys::Files => String::from("Fisiere: "),
                    LangKeys::From => String::from("Din: "),
                    LangKeys::Deleting => String::from("Stergere in progres"),
                    LangKeys::NewName => String::from("Nume nou"),
                    LangKeys::ExtensionOptional => String::from("Extensie noua (optional): "),
                    LangKeys::OldName => String::from("Nume vechi: "),
                    LangKeys::FreeOf => String::from(" liber din "),
                }
            }
        }
    }
}