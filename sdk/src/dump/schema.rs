use crate::interfaces::schema_system::schema_system::{
    SchemaClassInfo, SchemaSystem, SchemaSystemTypeScope,
};

use super::sdk_builder::{Class, ClassField, Enum, Scope};

fn get_class_inheritance<'a>(mut class: &'a SchemaClassInfo) -> Vec<&'a SchemaClassInfo<'a>> {
    let mut classes = vec![];

    loop {
        classes.push(class);

        let Some(base_class_info) = class.schema_parent() else {
            break;
        };

        let Some(base_class) = base_class_info.class() else {
            break;
        };

        class = base_class;
    }

    classes.reverse();

    classes
}

fn dump_type_scope(type_scope: &SchemaSystemTypeScope, output: &str) {
    let mut module = Scope::new(&type_scope.module_name());

    for enu in type_scope.enums.into_iter() {
        let mut sdk_enum = Enum::new(&enu.name());

        for variant in enu.variants() {
            sdk_enum.add_variant(&variant.name());
        }

        module.add_enum(sdk_enum);
    }

    for class in type_scope.classes.into_iter() {
        let Some(class) = type_scope.find_declared_class(&class.name()) else {
            log::error!("could not find declared class {}", class.name());

            continue;
        };

        let mut sdk_class = Class::new(&class.name(), &type_scope.module_name());

        let inheritance = get_class_inheritance(&class);

        for class in inheritance.iter() {
            for field in class.fields() {
                let Some(ty) = field.ty() else {
                    log::error!("could not get {}::{} type", class.name(), field.name());

                    continue;
                };

                sdk_class.add_field(ClassField::new(&class.name(), &field.name(), ty));
            }
        }

        module.add_class(sdk_class);
    }

    if let Err(e) = std::fs::write(
        format!(
            "{}/{}.rs",
            output,
            type_scope.module_name().replace(".dll", "")
        ),
        module.to_string(),
    ) {
        log::error!("{e}");
    }
}

pub fn dump(schema_system: &SchemaSystem, output: &str) {
    if let Some(global_type_scope) = schema_system.get_global_type_scope() {
        dump_type_scope(global_type_scope, output);
    }

    for type_scope in schema_system.type_scopes.iter() {
        if type_scope.module_name().contains("server") {
            continue;
        }

        dump_type_scope(type_scope, output);
    }
}
