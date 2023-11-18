use memory_macros::{vfunc, vmt, C};

use crate::{
    interfaces::create_interface,
    tier1::{utltshash::UtlTSHash, utlvector::UtlVector},
};
use std::{
    ffi::{c_char, c_void},
    marker::PhantomData,
    mem::ManuallyDrop,
};

use lazy_static::lazy_static;

lazy_static! {
    pub static ref SCHEMA_SYSTEM: &'static SchemaSystem<'static> =
        create_interface!("schemasystem.dll", "SchemaSystem");
}

#[vmt]
pub struct SchemaSystem<'a> {
    __pad: [u8; 0x188],
    pub type_scopes: UtlVector<SchemaSystemTypeScope<'a>>,
}

#[allow(unused_unsafe)]
impl<'a> SchemaSystem<'_> {
    #[vfunc(11)]
    pub fn get_global_type_scope(&self) -> &SchemaSystemTypeScope<'a> {}

    #[vfunc(13)]
    fn find_type_scope_for_module_vfunc(
        &self,
        module: &str,
        x: usize,
    ) -> &SchemaSystemTypeScope<'a> {
    }

    pub fn find_type_scope_for_module(&self, module: &str) -> Option<&SchemaSystemTypeScope> {
        self.find_type_scope_for_module_vfunc(module, 0)
    }

    pub fn find_offset(&self, module: &str, class: &str, field: &str) -> Option<i32> {
        let type_scope = self.find_type_scope_for_module(module)?;

        let class = type_scope.find_declared_class(class)?;

        let field = class.fields().find(|x| x.name() == field)?;

        Some(field.single_inheritance_offset)
    }
}

#[vmt]
#[derive(C, Clone, Copy)]
pub struct SchemaSystemTypeScope<'a> {
    module_name: [c_char; 256],
    __pad1: [u8; 0x47e],
    pub classes: UtlTSHash<'a, *const SchemaClass<'a>>,
    __pad2: [u8; 0x2808],
    pub enums: UtlTSHash<'a, *const SchemaEnum<'a>>,
}

impl<'a> SchemaSystemTypeScope<'a> {
    #[vfunc(2)]
    fn find_declared_class_vfunc(
        &self,
        class_info: *mut *mut SchemaClassInfo<'a>,
        name: &str,
    ) -> &SchemaClassInfo {
    }

    pub fn find_declared_class(&self, name: &str) -> Option<&SchemaClassInfo> {
        let mut class_info: *mut SchemaClassInfo = std::ptr::null_mut();
        self.find_declared_class_vfunc(&mut class_info, name);

        unsafe { class_info.as_ref() }
    }
}

#[repr(u8)]
#[derive(Debug)]
pub enum TypeCategory {
    Builtin,
    Pointer,
    Bitfield,
    FixedArray,
    Atomic,
    DeclaredClass,
    DeclaredEnum,
    None,
}

#[repr(u8)]
#[derive(Debug)]
pub enum AtomicCategory {
    Basic,
    T,
    CollectionOfT,
    TT,
    I,
    None,
    Invalid,
}

#[repr(C)]
#[derive(C)]
pub struct SchemaTypeArrayT<'a> {
    pub size: u32,
    unknown: u32,
    element_type: *const SchemaType<'a>,
}

#[repr(C)]
#[derive(C)]
pub struct SchemaTypeAtomicT<'a> {
    gap: [u64; 2],
    template_typename: *const SchemaType<'a>,
}

#[repr(C)]
pub struct SchemaTypeAtomicTT<'a> {
    gap: [u64; 2],
    templates: [*const SchemaType<'a>; 2],
}

impl SchemaTypeAtomicTT<'_> {
    pub fn templates(&self) -> Option<(&SchemaType, &SchemaType)> {
        let template1 = unsafe { self.templates[0].as_ref() }?;
        let template2 = unsafe { self.templates[1].as_ref() }?;

        Some((template1, template2))
    }
}

#[repr(C)]
pub struct SchemaTypeAtomicI {
    gap: [u64; 2],
    integer: u64,
}

#[repr(C)]
#[derive(C)]
pub union SchemaTypeUnion<'a> {
    schema_type: *const SchemaType<'a>,
    class_info: *const SchemaClassInfo<'a>,
    enum_info: *const SchemaEnum<'a>,

    pub array: ManuallyDrop<SchemaTypeArrayT<'a>>,
    pub atomic_t: ManuallyDrop<SchemaTypeAtomicT<'a>>,
    pub atomic_tt: ManuallyDrop<SchemaTypeAtomicTT<'a>>,
    pub atomic_i: ManuallyDrop<SchemaTypeAtomicI>,
}

#[vmt]
#[derive(C)]
pub struct SchemaType<'a> {
    name: *const c_char,
    type_scope: *const SchemaSystemTypeScope<'a>,

    pub type_category: TypeCategory,
    pub atomic_category: AtomicCategory,

    pub value: SchemaTypeUnion<'a>,
}

#[repr(C)]
#[derive(C)]
pub struct SchemaMetadataEntryData {
    name: *const c_char,
    value: *const SchemaNetworkValue,
}

pub struct SchemaMetadataSetData {
    static_entries: SchemaMetadataEntryData,
}

#[repr(C)]
#[derive(C)]
pub struct SchemaClassField<'a> {
    name: *const c_char,
    ty: *const SchemaType<'a>,
    pub single_inheritance_offset: i32,
    metadata_size: i32,
    metadata: *const SchemaMetadataEntryData,
}

#[derive(C)]
pub struct SchemaStaticField<'a> {
    name: *const c_char,
    ty: *const SchemaType<'a>,
    instance: *const c_void,
    __pad: [u8; 0x10],
}

#[repr(C)]
#[derive(C)]
pub struct SchemaBaseClassInfo<'a> {
    offset: u32,
    class: *const SchemaClassInfo<'a>,
}

#[vmt]
#[derive(C)]
pub struct SchemaClassInfo<'a> {
    name: *const c_char,
    module: *const c_char,

    pub size: i32,
    pub alignment: i16,

    pub static_size: i16,
    pub metadata_size: i16,

    __unknown1: i16,
    __unknown2: i16,
    __unknown3: i16,

    _fields: *const SchemaClassField<'a>,
    static_fields: *const SchemaStaticField<'a>,
    schema_parent: *const SchemaBaseClassInfo<'a>,
    __pad: [u8; 0x10],

    metadata: *const SchemaMetadataSetData,
    type_scope: *const SchemaSystemTypeScope<'a>,
    schema_type: *const SchemaType<'a>,
    class_flags: SchemaClassFlags,
}

impl SchemaClassInfo<'_> {
    pub fn fields(&self) -> SchemaClassFields {
        SchemaClassFields {
            fields: self._fields,
            size: self.alignment,
            index: 0,
        }
    }
}

pub struct SchemaClassFields<'a> {
    fields: *const SchemaClassField<'a>,
    size: i16,
    index: i16,
}

impl<'a> Iterator for SchemaClassFields<'a> {
    type Item = &'a SchemaClassField<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.size {
            self.index += 1;

            Some(unsafe { self.fields.offset(self.index as isize - 1).as_ref()? })
        } else {
            None
        }
    }
}

#[repr(C)]
union SchemaEnumVariantData {
    char: u8,
    short: u16,
    int: u32,
    value: u64,
}

#[repr(C)]
#[derive(C)]
pub struct SchemaEnumVariant {
    name: *const c_char,
    value: SchemaEnumVariantData,
    __pad: [u8; 0x10],
}

#[repr(C)]
#[derive(C)]
pub struct SchemaEnumInfo {
    field: SchemaEnumVariant,
}

#[vmt]
#[derive(C, Clone, Copy)]
pub struct SchemaEnum<'a> {
    name: *const c_char,
    module_name: *const c_char,

    pub alignment: i8,
    __pad_0x0: [u8; 0x3],
    pub size: i16,
    pub flags: i16,

    _variants: *const SchemaEnumVariant,

    __pad_0x1: [u8; 0x8],

    type_scope: *const SchemaSystemTypeScope<'a>,

    __pad_0x2: [u8; 0x8],
    __unk: i32,
}

impl SchemaEnum<'_> {
    pub fn variants(&self) -> SchemaEnumVariants {
        SchemaEnumVariants {
            variants: self._variants,
            size: self.size,
            index: 0,
            _phantom: Default::default(),
        }
    }
}

pub struct SchemaEnumVariants<'a> {
    variants: *const SchemaEnumVariant,
    size: i16,
    index: i16,

    _phantom: PhantomData<&'a SchemaEnumVariant>,
}

impl<'a> Iterator for SchemaEnumVariants<'a> {
    type Item = &'a SchemaEnumVariant;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.size {
            self.index += 1;

            Some(unsafe { self.variants.offset(self.index as isize - 1).as_ref()? })
        } else {
            None
        }
    }
}

#[repr(u8)]
enum SchemaClassFlags {
    HasVirtualMembers = 1,
    IsAbstract = 2,
    HasTrivialConstructor = 4,
    HasTrivialDestructor = 8,
    HasNoSchemaMembers = 16,
    HasConstructorLikeMethods = 32,
    HasDestructorLikeMethods = 64,
    IsNoSchemaClass = 128,
}

#[repr(C)]
#[derive(C, Clone, Copy)]
pub struct SchemaClass<'a> {
    parent: *const SchemaClass<'a>,
    name: *const c_char,
    module_name: *const c_char,

    unknown: *const c_char,
    class_info_old_synthesized: usize,
    class_info: usize,

    this_module_binding_pointer: usize,
    schema_type: *const SchemaType<'a>,
}

#[repr(C)]
pub union SchemaNetworkValue {
    string: *const c_char,
    number: i32,
    float: f32,
    pointer: *const usize,
}
