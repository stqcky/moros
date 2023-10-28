use std::fmt::Display;

use convert_case::{Case, Casing};
use regex::Regex;

use crate::sdk::ENGINE_CLIENT;

pub struct Module {
    classes: Vec<Class>,
    enums: Vec<Enum>,
    name: String,
}

impl Module {
    pub fn new(name: &str) -> Self {
        Self {
            name: String::from(name),
            classes: vec![],
            enums: vec![],
        }
    }

    pub fn add_enum(&mut self, enu: Enum) {
        self.enums.push(enu);
    }

    pub fn add_class(&mut self, class: Class) {
        self.classes.push(class);
    }
}

impl Display for Module {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "// module {}\n", self.name)?;
        write!(f, "// {}\n\n", chrono::Utc::now())?;

        for enu in self.enums.iter() {
            write!(f, "{}\n\n", enu)?;
        }

        for class in self.classes.iter() {
            write!(f, "{}\n\n", class)?;
        }

        Ok(())
    }
}

pub struct Class {
    fields: Vec<ClassField>,
    rust_name: String,
    name: String,
}

const CLASS_PREFIXES: &'static [&'static str] = &["C_CSGO_", "CCSGO", "C_", "C"];
const CLASS_SUFFIXES: &'static [&'static str] = &["_t"];

impl Class {
    pub fn new(name: &str) -> Self {
        Self {
            rust_name: Self::rustify_name(name),
            name: String::from(name),
            fields: vec![],
        }
    }

    pub fn add_field(&mut self, field: ClassField) {
        self.fields.push(field);
    }

    fn rustify_name(name: &str) -> String {
        let mut name = String::from(name);

        name = strip_prefixes(name, CLASS_PREFIXES.iter());
        name = strip_suffixes(name, CLASS_SUFFIXES.iter());

        name.to_case(convert_case::Case::UpperCamel)
    }
}

impl Display for Class {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "#[repr(C)]\n")?;
        write!(f, "pub struct {} {{}}", self.rust_name)?;

        if self.fields.len() > 0 {
            write!(f, "\n")?;

            write!(f, "impl {} {{", self.rust_name)?;

            for field in self.fields.iter() {
                write!(f, "\t{},\n", field)?;
            }
        }

        write!(f, "}}")
    }
}

struct Pawn {}

fn get_field_offset(class: &str, field: &str) -> i32 {
    0
}

impl Pawn {
    pub fn health(&self) -> i32 {
        lazy_static::lazy_static! {
            static ref OFFSET: isize = get_field_offset("CSPlayerPawn", "m_iHealth") as isize;
        }

        unsafe {
            std::mem::transmute::<_, *const i32>(
                std::mem::transmute::<_, *const u8>(self).offset(*OFFSET),
            )
            .read()
        }
    }
}

const FIELD_SUFFIXES: &'static [&'static str] = &["_t"];

pub struct ClassField {
    rust_name: String,
    name: String,
    offset: i32,
    ty: String,
}

impl ClassField {
    pub fn new(name: &str, ty: String, offset: i32) -> Self {
        Self {
            rust_name: Self::rustify_name(name),
            name: String::from(name),
            offset,
            ty,
        }
    }

    fn rustify_name(name: &str) -> String {
        let mut name = String::from(name);

        let re = Regex::new(r#"((?:__)?m_(?:p|nCs|n|isz|i|bv|b|h|flCs|fl|str|f|vecCs|vec|v|e|ang|sz|arr|aim|a|clr|c|un|ub|s|C))[A-Z]"#)
            .expect("could not create regex");

        if let Some(captures) = re.captures(&name) {
            if let Some(prefix) = captures.get(1) {
                name = name.replace(prefix.as_str(), "");
            }
        } else {
            name = name.replace("m_", "");
        }

        name = strip_suffixes(name, FIELD_SUFFIXES.iter());

        name.to_case(Case::Snake)
    }
}

impl Display for ClassField {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "pub {} /* {} {:#x} */: usize",
            self.rust_name, self.ty, self.offset
        )
    }
}

pub struct Enum {
    rust_name: String,
    name: String,
    variants: Vec<EnumVariant>,
}

impl Enum {
    pub fn new(name: &str) -> Self {
        Self {
            rust_name: Self::rustify_name(name),
            name: String::from(name),
            variants: vec![],
        }
    }

    pub fn add_variant(&mut self, name: &str) {
        self.variants.push(EnumVariant::new(name));
    }

    fn rustify_name(name: &str) -> String {
        let mut name = String::from(name);

        if let Some(split) = name.split("::").nth(1) {
            name = split.to_string();
        }

        name = strip_suffixes(name, ["_t"].iter());
        name.to_case(Case::UpperCamel)
    }
}

impl Display for Enum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "enum {} /* {} */ {{", self.rust_name, self.name)?;

        for variant in self.variants.iter() {
            write!(f, "\n\t{variant}")?;
        }

        write!(f, "\n}}")
    }
}

pub struct EnumVariant {
    rust_name: String,
    name: String,
}

impl EnumVariant {
    pub fn new(name: &str) -> Self {
        Self {
            rust_name: Self::rustify_name(name),
            name: String::from(name),
        }
    }

    fn rustify_name(name: &str) -> String {
        name.to_case(Case::UpperCamel)
    }
}

impl Display for EnumVariant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} /* {} */", self.rust_name, self.name)
    }
}

fn strip_prefixes<'a>(name: String, prefixes: impl Iterator<Item = &'a &'a str>) -> String {
    for prefix in prefixes {
        if let Some(stripped) = name.strip_prefix(prefix) {
            return stripped.to_string();
        }
    }

    name
}

fn strip_suffixes<'a>(name: String, suffixes: impl Iterator<Item = &'a &'a str>) -> String {
    for suffix in suffixes {
        if let Some(stripped) = name.strip_suffix(suffix) {
            return stripped.to_string();
        }
    }

    name
}
