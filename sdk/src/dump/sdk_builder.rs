use std::fmt::Display;

use convert_case::{Case, Casing};
use regex::Regex;

use crate::interfaces::schema_system::schema_system::{AtomicCategory, SchemaType, TypeCategory};

pub struct Scope<'a> {
    classes: Vec<Class<'a>>,
    enums: Vec<Enum>,
    name: String,
}

impl<'a> Scope<'a> {
    pub fn new(name: &str) -> Self {
        Self {
            name: String::from(name),
            classes: vec![],
            enums: vec![],
        }
    }

    pub fn add_enum(&mut self, enu: Enum) {
        if let Some(existing) = self.enums.iter().position(|x| x.rust_name == enu.rust_name) {
            self.enums.swap_remove(existing); // this might cause problems later on.
        };

        self.enums.push(enu);
    }

    pub fn add_class(&mut self, class: Class<'a>) {
        if let Some(existing) = self
            .classes
            .iter()
            .position(|x| x.rust_name == class.rust_name)
        {
            self.classes.swap_remove(existing); // this might cause problems later on.
        };

        self.classes.push(class);
    }
}

impl Display for Scope<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "// scope {}", self.name)?;
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

pub struct Class<'a> {
    fields: Vec<ClassField<'a>>,
    rust_name: String,
    name: String,
    scope: String,
}

impl<'a> Class<'a> {
    pub fn new(name: &str, scope: &str) -> Self {
        Self {
            rust_name: Self::rustify_name(name),
            name: String::from(name),
            fields: vec![],
            scope: String::from(scope),
        }
    }

    pub fn add_field(&mut self, mut field: ClassField<'a>) {
        if let Some(existing_idx) = self
            .fields
            .iter()
            .position(|x| x.rust_name == field.rust_name)
        {
            let existing = &mut self.fields[existing_idx];

            let new_existing_name = ClassField::rustify_name(&existing.name.replace("m_", ""));
            let new_new_name = ClassField::rustify_name(&field.name.replace("m_", ""));

            if new_existing_name != new_new_name {
                existing.rust_name = new_existing_name;
                field.rust_name = new_new_name;
            } else {
                self.fields.swap_remove(existing_idx);
            }
        }

        self.fields.push(field);
    }

    fn rustify_name(name: &str) -> String {
        let mut name = String::from(name);

        let re = Regex::new(r#"(C_CSGO_|CCSGO|C_|C)[A-Z]"#).expect("could not create regex");

        if let Some(captures) = re.captures(&name) {
            if let Some(prefix) = captures.get(1) {
                if let Some(stripped) = name.strip_prefix(prefix.as_str()) {
                    name = stripped.to_string();
                }
            }
        }

        if let Some(stripped) = name.strip_suffix("_t") {
            name = stripped.to_string();
        }

        name = name.replace("::", "");

        name.to_case(convert_case::Case::UpperCamel)
    }
}

impl Display for Class<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "#[derive(Schema)]")?;
        writeln!(f, r#"#[scope("{}")]"#, self.scope)?;
        write!(f, "pub struct {} {{", self.rust_name)?;

        if self.fields.len() > 0 {
            writeln!(f)?;

            for field in self.fields.iter() {
                writeln!(f, "{},", field)?;
            }
        }

        write!(f, "}}")
    }
}

pub struct ClassField<'a> {
    rust_name: String,
    name: String,
    class: String,
    ty: &'a SchemaType<'a>,
}

impl<'a> ClassField<'a> {
    pub fn new(class: &str, name: &str, ty: &'a SchemaType<'_>) -> Self {
        Self {
            rust_name: Self::rustify_name(name),
            name: String::from(name),
            class: String::from(class),
            ty,
        }
    }

    fn rustify_name(name: &str) -> String {
        let mut name = String::from(name);

        let re = Regex::new(r#"((?:__)?m_(?:p|nCs|n|isz|i|bv|b|h|flCs|fl|str|f|vecCs|vec|v|e|ang|sz|arr|aim|a|clr|c|un|ub|s|C))[A-Z]"#)
            .expect("could not create regex");

        if let Some(captures) = re.captures(&name) {
            if let Some(prefix) = captures.get(1) {
                if let Some(stripped) = name.strip_prefix(prefix.as_str()) {
                    name = stripped.to_string();
                }
            }
        } else if let Some(stripped) = name.strip_prefix("m_") {
            name = stripped.to_string();
        }

        if let Some(stripped) = name.strip_suffix("_t") {
            name = stripped.to_string();
        }

        name = name.to_case(Case::Snake);

        match name.as_ref() {
            "loop" => "r#loop".to_string(),
            "break" => "r#break".to_string(),
            "type" => "ty".to_string(),
            "fn" => "func".to_string(),
            _ => name,
        }
    }

    fn rustify_type(ty: &SchemaType<'_>) -> String {
        match ty.type_category {
            TypeCategory::Builtin => match ty.name().as_ref() {
                "float32" => "f32",
                "float64" => "f64",

                "int8" => "i8",
                "int16" => "i16",
                "int32" => "i32",
                "int64" => "i64",

                "uint8" => "u8",
                "uint16" => "u16",
                "uint32" => "u32",
                "uint64" => "u64",

                "bool" => "bool",
                "char" => "std::ffi::c_char",
                _ => {
                    log::error!("unknown builtin type: {}", ty.name());

                    "unknown builtin type"
                }
            }
            .to_string(),
            TypeCategory::Pointer => {
                let ty = ty.value.schema_type().expect("could not get pointer type");

                format!("*const {}", Self::rustify_type(ty))
            }
            TypeCategory::Bitfield => "u8".to_string(),
            TypeCategory::FixedArray => {
                let array = unsafe { &ty.value.array };
                let array_type = array.element_type().expect("could not get array type");

                format!("[{}; {}]", Self::rustify_type(array_type), array.size)
            }
            TypeCategory::Atomic => match ty.atomic_category {
                AtomicCategory::Basic => Class::rustify_name(&ty.name()),
                AtomicCategory::T => {
                    let atomic_t = unsafe { &ty.value.atomic_t };

                    let template = atomic_t
                        .template_typename()
                        .expect("could not get template");

                    let ty_name = ty.name();

                    let (base, _) = ty_name
                        .split_once('<')
                        .expect("could not get template base");

                    format!(
                        "{}<{}>",
                        Class::rustify_name(base),
                        Self::rustify_type(template)
                    )
                }
                AtomicCategory::CollectionOfT => {
                    let atomic_t = unsafe { &ty.value.atomic_t };
                    let ty = atomic_t
                        .template_typename()
                        .expect("could not get collection of T type");

                    format!("UtlVector<{}>", Self::rustify_type(ty))
                }
                AtomicCategory::TT => {
                    let atomic_tt = unsafe { &ty.value.atomic_tt };

                    let Some((template1, template2)) = atomic_tt.templates() else {
                        log::error!("could not get atomic TT templates");
                        return "unknown atomic tt".to_string();
                    };

                    let ty_name = ty.name();

                    let (base, _) = ty_name
                        .split_once('<')
                        .expect("could not get template base");

                    format!(
                        "{}<{}, {}>",
                        Class::rustify_name(base),
                        Self::rustify_type(template1),
                        Self::rustify_type(template2)
                    )
                }
                AtomicCategory::I => {
                    log::error!("unknown atomic i: {}", ty.name());
                    "unknown atomic i".to_string()
                }
                AtomicCategory::None => {
                    log::error!("field type category is atomic but atomic category is none");
                    "unknown none atomic".to_string()
                }
                AtomicCategory::Invalid => {
                    log::error!("field type category is atomic but atomic category is none");
                    "unknown invalid atomic".to_string()
                }
            },
            TypeCategory::DeclaredClass => {
                let class_info = ty.value.class_info().expect("could not get class info");

                Class::rustify_name(&class_info.name())
            }
            TypeCategory::DeclaredEnum => {
                let enum_info = ty.value.enum_info().expect("could not get enum info");

                Enum::rustify_name(&enum_info.name())
            }
            TypeCategory::None => format!("unknown none: {}", ty.name()),
        }
    }
}

impl Display for ClassField<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "\t#[field(\"{}\", \"{}\")]", self.class, self.name)?;
        write!(f, "\t{}: {}", self.rust_name, Self::rustify_type(self.ty))
    }
}

pub struct Enum {
    name: String,
    rust_name: String,
    variants: Vec<EnumVariant>,
}

impl Enum {
    pub fn new(name: &str) -> Self {
        Self {
            name: String::from(name),
            rust_name: Self::rustify_name(name),
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

        if let Some(stripped) = name.strip_suffix("_t") {
            name = stripped.to_string();
        }

        name.to_case(Case::UpperCamel)
    }
}

impl Display for Enum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "pub enum {} {{", self.rust_name)?;

        for variant in self.variants.iter() {
            write!(f, "\n\t{variant}")?;
        }

        write!(f, "\n}}")
    }
}

pub struct EnumVariant {
    rust_name: String,
}

impl EnumVariant {
    pub fn new(name: &str) -> Self {
        Self {
            rust_name: Self::rustify_name(name),
        }
    }

    fn rustify_name(name: &str) -> String {
        name.to_case(Case::UpperCamel)
    }
}

impl Display for EnumVariant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{},", self.rust_name)
    }
}
