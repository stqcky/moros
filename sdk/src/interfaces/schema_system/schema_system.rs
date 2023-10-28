use crate::sdk::tier1::{utltshash::CUtlTSHash, utlvector::CUtlVector};
use cstruct::{vfunc, vmt, C};
use std::{
    borrow::Cow,
    ffi::{c_char, c_void},
    marker::PhantomData, mem::ManuallyDrop,
};

#[vmt]
pub struct SchemaSystem<'a> {
    __pad: [u8; 0x188],
    pub type_scopes: CUtlVector<SchemaSystemTypeScope<'a>>,
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
}

#[vmt]
#[derive(C, Clone, Copy)]
pub struct SchemaSystemTypeScope<'a> {
    #[str]
    module_name: [c_char; 256],

    __pad1: [u8; 0x47e],
    pub classes: CUtlTSHash<'a, *const SchemaClass<'a>>,
    __pad2: [u8; 0x2808],
    pub enums: CUtlTSHash<'a, *const SchemaEnum<'a>>,
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

        Some(unsafe { class_info.as_ref() }?)
    }
}

#[repr(u8)]
#[derive(Debug)]
pub enum TypeCategory {
    Builtin,
    Pointer,
    Butfield,
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
}

#[repr(C)]
struct SchemaTypeArrayT<'a> {
    size: u32,
    unknown: u32,
    element_type: *const SchemaType<'a>,
}

#[repr(C)]
struct SchemaTypeAtomicT<'a> {
    gap: [u64; 2],
    template_typename: *const SchemaType<'a>,
}

#[repr(C)]
struct SchemaTypeAtomicTT<'a> {
    gap: [u64; 2],
    templates: [*const SchemaType<'a>; 2],
}

#[repr(C)]
struct SchemaTypeAtomicI {
    gap: [u64; 2],
    integer: u64,
}

#[repr(C)]
pub union SchemaTypeUnion<'a> {
    schema_type: *const SchemaType<'a>,
    class_info: *const SchemaClassInfo<'a>,
    enum_info: *const SchemaEnum<'a>,
    array: ManuallyDrop<SchemaTypeArrayT<'a>>,
    atomic_t: ManuallyDrop<SchemaTypeAtomicT<'a>>,
    atomic_tt: ManuallyDrop<SchemaTypeAtomicTT<'a>>,
    atomic_i: ManuallyDrop<SchemaTypeAtomicI>,
}

#[vmt]
#[derive(C)]
pub struct SchemaType<'a> {
    #[str]
    name: *const c_char,

    #[ptr]
    type_scope: *const SchemaSystemTypeScope<'a>,

    pub type_category: TypeCategory,
    pub atomic_category: u8,

    value: SchemaTypeUnion<'a>
}

#[repr(C)]
#[derive(C)]
pub struct SchemaMetadataEntryData {
    #[str]
    name: *const c_char,

    #[ptr]
    value: *const SchemaNetworkValue,
}

pub struct SchemaMetadataSetData {
    static_entries: SchemaMetadataEntryData,
}

#[repr(C)]
#[derive(C)]
pub struct SchemaClassField<'a> {
    #[str]
    name: *const c_char,

    #[ptr]
    ty: *const SchemaType<'a>,

    pub single_inheritance_offset: i32,
    metadata_size: i32,

    #[ptr]
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

    #[ptr]
    class: *const SchemaClassInfo<'a>,
}

#[vmt]
#[derive(C)]
pub struct SchemaClassInfo<'a> {
    #[str]
    name: *const c_char,

    #[str]
    module: *const c_char,

    pub size: i32,
    pub alignment: i16,

    pub static_size: i16,
    pub metadata_size: i16,

    __unknown1: i16,
    __unknown2: i16,
    __unknown3: i16,

    #[ptr]
    fields: *const SchemaClassField<'a>,

    #[ptr]
    static_fields: *const SchemaStaticField<'a>,

    #[ptr]
    schema_parent: *const SchemaBaseClassInfo<'a>,

    __pad: [u8; 0x10],

    #[ptr]
    metadata: *const SchemaMetadataSetData,

    #[ptr]
    type_scope: *const SchemaSystemTypeScope<'a>,

    #[ptr]
    schema_type: *const SchemaType<'a>,

    class_flags: SchemaClassFlags,
}

impl SchemaClassInfo<'_> {
    pub fn fields(&self) -> SchemaClassFields {
        SchemaClassFields {
            fields: self.fields,
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
    #[str]
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
    #[str]
    name: *const c_char,

    #[str]
    module_name: *const c_char,

    pub alignment: i8,
    __pad_0x0: [u8; 0x3],
    pub size: i16,
    pub flags: i16,

    variants: *const SchemaEnumVariant,

    __pad_0x1: [u8; 0x8],

    #[ptr]
    type_scope: *const SchemaSystemTypeScope<'a>,

    __pad_0x2: [u8; 0x8],
    __unk: i32,
}

impl SchemaEnum<'_> {
    pub fn variants(&self) -> SchemaEnumVariants {
        SchemaEnumVariants {
            variants: self.variants,
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
    #[ptr]
    parent: *const SchemaClass<'a>,

    #[str]
    name: *const c_char,

    #[str]
    module_name: *const c_char,

    unknown: *const c_char,
    class_info_old_synthesized: usize,
    class_info: usize,

    this_module_binding_pointer: usize,

    #[ptr]
    schema_type: *const SchemaType<'a>,
}

#[repr(C)]
pub union SchemaNetworkValue {
    string: *const c_char,
    number: i32,
    float: f32,
    pointer: *const usize,
}
