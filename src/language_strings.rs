pub enum Language {
    Romanian,
    English,
}
pub enum LangKey {
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
    PowerShellIssue,
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
    pub fn get(&self, key: LangKey) -> String {
        return match self.current_lang {
            Language::English => {
                match key {
                    LangKey::RootPath          =>      String::from("Personal Computer"),
                    LangKey::CantOpenSystemDir =>      String::from("Cannot open system directory"),
                    LangKey::NothingSelected   =>      String::from("Nothing selected"),
                    LangKey::Extension         =>      String::from("Extension: "),
                    LangKey::CreatedAt         =>      String::from("Created at: "),
                    LangKey::AccessedAt        =>      String::from("Accessed at: "),
                    LangKey::ModifiedAt        =>      String::from("Modified at: "),
                    LangKey::Open              =>      String::from("Open"),
                    LangKey::Delete            =>      String::from("Delete"),
                    LangKey::Rename            =>      String::from("Rename"),
                    LangKey::Copy              =>      String::from("Copy"),
                    LangKey::Move              =>      String::from("Move"),
                    LangKey::CreateNew         =>      String::from("Create new"),
                    LangKey::PasteFrom         =>      String::from("Paste from"),
                    LangKey::Paste             =>      String::from("Paste"),
                    LangKey::CantCopyRoot      =>      String::from("Cannot copy root; if this is a bug please report it"),
                    LangKey::File              =>      String::from("File"),
                    LangKey::Folder            =>      String::from("Folder"),
                    LangKey::Create            =>      String::from("Create"),
                    LangKey::Cancel            =>      String::from("Cancel"),
                    LangKey::RenameFile        =>      String::from("Rename file"),
                    LangKey::Files             =>      String::from("Files: "),
                    LangKey::From              =>      String::from("From: "),
                    LangKey::Deleting          =>      String::from("Deleting"),
                    LangKey::NewName           =>      String::from("New name"),
                    LangKey::ExtensionOptional =>      String::from("Extension (optional): "),
                    LangKey::OldName           =>      String::from("Old name: "),
                    LangKey::FreeOf            =>      String::from(" free of "),
                    LangKey::PowerShellIssue   =>      String::from("Could not open file. Path may not exist or PowerShell might not be installed"),
                }
            }
            Language::Romanian => {
                match key {
                    LangKey::RootPath          =>      String::from("Calculator personal"),
                    LangKey::CantOpenSystemDir =>      String::from("Acest fisier de sistem nu poate fi deschis"),
                    LangKey::NothingSelected   =>      String::from("Nimic selectat"),
                    LangKey::Extension         =>      String::from("Extensie: "),
                    LangKey::CreatedAt         =>      String::from("Creat: "),
                    LangKey::AccessedAt        =>      String::from("Accesat: "),
                    LangKey::ModifiedAt        =>      String::from("Modificat: "),
                    LangKey::Open              =>      String::from("Deschide"),
                    LangKey::Delete            =>      String::from("Sterge"),
                    LangKey::Rename            =>      String::from("Redenumeste"),
                    LangKey::Copy              =>      String::from("Copiaza"),
                    LangKey::Move              =>      String::from("Muta"),
                    LangKey::CreateNew         =>      String::from("Meniu de creare"),
                    LangKey::PasteFrom         =>      String::from("Lipeste din"),
                    LangKey::Paste             =>      String::from("Lipeste"),
                    LangKey::CantCopyRoot      =>      String::from("Nu putem copia baza, daca e un bug raporteaza."),
                    LangKey::File              =>      String::from("Fisier"),
                    LangKey::Folder            =>      String::from("Folder"),
                    LangKey::Create            =>      String::from("Creeaza"),
                    LangKey::Cancel            =>      String::from("Opreste"),
                    LangKey::RenameFile        =>      String::from("Redenumeste fisier"),
                    LangKey::Files             =>      String::from("Fisiere: "),
                    LangKey::From              =>      String::from("Din: "),
                    LangKey::Deleting          =>      String::from("Stergere in progres"),
                    LangKey::NewName           =>      String::from("Nume nou"),
                    LangKey::ExtensionOptional =>      String::from("Extensie noua (optional): "),
                    LangKey::OldName           =>      String::from("Nume vechi: "),
                    LangKey::FreeOf            =>      String::from(" liber din "),
                    LangKey::PowerShellIssue   =>      String::from("Fisierul nu a putut fi deschis. Calea folder-ului este posibil sa nu mai existe sau\
                                                                PowerShell nu este instalat."),
                }
            }
        }
    }
}