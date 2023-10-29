use crate::tier1::utlvector::UtlVector;

use crate::schema::SCHEMA_SYSTEM;
use schema::Schema;

// scope rendersystemvulkan.dll
// 2023-10-29 23:17:00.725798400 UTC

pub enum RenderSlotType {
    RenderSlotInvalid,
    RenderSlotPerVertex,
    RenderSlotPerInstance,
}

pub enum RenderBufferFlags {
    RenderBufferUsageVertexBuffer,
    RenderBufferUsageIndexBuffer,
    RenderBufferUsageShaderResource,
    RenderBufferUsageUnorderedAccess,
    RenderBufferByteaddressBuffer,
    RenderBufferStructuredBuffer,
    RenderBufferAppendConsumeBuffer,
    RenderBufferUavCounter,
    RenderBufferUavDrawIndirectArgs,
}

pub enum RenderPrimitiveType {
    RenderPrimPoints,
    RenderPrimLines,
    RenderPrimLinesWithAdjacency,
    RenderPrimLineStrip,
    RenderPrimLineStripWithAdjacency,
    RenderPrimTriangles,
    RenderPrimTrianglesWithAdjacency,
    RenderPrimTriangleStrip,
    RenderPrimTriangleStripWithAdjacency,
    RenderPrimInstancedQuads,
    RenderPrimHeterogenous,
    RenderPrimComputeShader,
    RenderPrimTypeCount,
}

pub enum InputLayoutVariation {
    InputLayoutVariationDefault,
    InputLayoutVariationStream1Instanceid,
    InputLayoutVariationStream1InstanceidMorphVertId,
    InputLayoutVariationMax,
}

pub enum RenderMultisampleType {
    RenderMultisampleInvalid,
    RenderMultisampleNone,
    RenderMultisample2X,
    RenderMultisample4X,
    RenderMultisample6X,
    RenderMultisample8X,
    RenderMultisample16X,
    RenderMultisampleTypeCount,
}

#[derive(Schema)]
#[scope("rendersystemvulkan.dll")]
pub struct RenderInputLayoutField {
    #[field("RenderInputLayoutField_t", "m_pSemanticName")]
    semantic_name: [u8; 32],
    #[field("RenderInputLayoutField_t", "m_nSemanticIndex")]
    semantic_index: i32,
    #[field("RenderInputLayoutField_t", "m_Format")]
    format: u32,
    #[field("RenderInputLayoutField_t", "m_nOffset")]
    offset: i32,
    #[field("RenderInputLayoutField_t", "m_nSlot")]
    slot: i32,
    #[field("RenderInputLayoutField_t", "m_nSlotType")]
    slot_type: RenderSlotType,
    #[field("RenderInputLayoutField_t", "m_nInstanceStepRate")]
    instance_step_rate: i32,
}

#[derive(Schema)]
#[scope("rendersystemvulkan.dll")]
pub struct VsInputSignatureElement {
    #[field("VsInputSignatureElement_t", "m_pName")]
    name: [std::ffi::c_char; 64],
    #[field("VsInputSignatureElement_t", "m_pSemantic")]
    semantic: [std::ffi::c_char; 64],
    #[field("VsInputSignatureElement_t", "m_pD3DSemanticName")]
    d_3_d_semantic_name: [std::ffi::c_char; 64],
    #[field("VsInputSignatureElement_t", "m_nD3DSemanticIndex")]
    d_3_d_semantic_index: i32,
}

#[derive(Schema)]
#[scope("rendersystemvulkan.dll")]
pub struct VsInputSignature {
    #[field("VsInputSignature_t", "m_elems")]
    elems: UtlVector<VsInputSignatureElement>,
}
