use crate::sdk::interfaces::schema_system::schema_system::{
    SchemaClassInfo, SchemaEnum, SchemaSystem, SchemaSystemTypeScope,
};

use super::sdk_builder::{Class, ClassField, Enum, Module};

fn get_enum_type<'a>(enu: &SchemaEnum) -> &'a str {
    match enu.alignment {
        1 => "u8",
        2 => "u16",
        4 => "u32",
        8 => "u64",
        _ => "unknown",
    }
}

fn get_class_inheritance<'a>(mut class: &'a SchemaClassInfo) -> Vec<&'a SchemaClassInfo<'a>> {
    let mut classes = vec![];

    loop {
        classes.push(class);

        let Some(base_class_info) = class.get_schema_parent() else {
            break;
        };

        let Some(base_class) = base_class_info.get_class() else {
            break;
        };

        class = base_class;
    }

    classes.reverse();

    classes
}

fn dump_type_scope(type_scope: &SchemaSystemTypeScope) {
    let mut module = Module::new(&type_scope.get_module_name());

    for enu in type_scope.enums.into_iter() {
        let mut sdk_enum = Enum::new(&enu.get_name());

        for variant in enu.variants() {
            sdk_enum.add_variant(&variant.get_name());
        }

        module.add_enum(sdk_enum);
    }

    for class in type_scope.classes.into_iter() {
        let Some(class) = type_scope.find_declared_class(&class.get_name()) else {
            log::error!("could not find declared class {}", class.get_name());

            continue;
        };

        let mut sdk_class = Class::new(&class.get_name());

        let inheritance = get_class_inheritance(&class);

        for class in inheritance.iter() {
            for field in class.fields() {
                let Some(ty) = field.get_ty() else {
                    log::error!("could not get {}::{} type", class.get_name(), field.get_name());

                    continue;
                };

                sdk_class.add_field(ClassField::new(
                    &field.get_name(),
                    ty.get_name().to_string(),
                    field.single_inheritance_offset,
                ));
            }
        }

        module.add_class(sdk_class);
    }

    if let Err(e) = std::fs::write(
        format!(
            "C:\\Users\\stacky\\Desktop\\share\\moros\\{}.rs",
            type_scope.get_module_name()
        ),
        module.to_string(),
    ) {
        log::error!("{e}");
    }
}

pub fn dump(schema_system: &SchemaSystem) {
    // if let Some(global_type_scope) = schema_system.get_global_type_scope() {
    //     dump_type_scope(global_type_scope);
    // }

    for type_scope in schema_system.type_scopes.iter() {
        if !type_scope.get_module_name().contains("client") {
            continue;
        }

        dump_type_scope(type_scope);
    }
}
