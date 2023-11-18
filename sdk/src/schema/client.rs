use crate::interfaces::client::entity_system::ENTITY_SYSTEM;
use crate::{
    color::Color,
    ehandle::{EntityHandle, Handle},
    interfaces::schema_system::schema_system::SCHEMA_SYSTEM,
    math::{
        matrix::Matrix3X4,
        qangle::QAngle,
        vector::{Vector, Vector4D},
    },
    tier1::{
        utlstring::UtlString, utlstringtoken::UtlStringToken, utlsymbollarge::UtlSymbolLarge,
        utlvector::UtlVector,
    },
    types::{EntityIndex, SplitScreenSlot, Transform, WorldGroupId},
};
use glam::Vec3;
use schema::Schema;

use super::global::{
    CommandContext, InButtonState, LifeState, NetworkOriginCellCoordQuantizedVector,
};
use super::{
    global::{
        AttachmentHandle, AttributeProviderTypes, ClientAlphaProperty, CountdownTimer,
        CsPlayerBlockingUseAction, CsPlayerState, CsWeaponMode, CsWeaponState, EntityIoOutput,
        FixAngleSet, GameTick, GameTime, HSequence, IronSightController, KillTypes, LoadoutSlot,
        MedalRank, MoveCollide, MoveType, NetworkTransmitComponent, NetworkVarChainer,
        NetworkVelocityVector, NetworkViewOffsetVector, ObserverMode, ParticleIndex,
        ParticleProperty, PlayerConnectedState, QuestProgressReason, RenderFx, RenderMode,
        SolidType, SurroundingBoundsType, TakeDamageFlags, Thinkfunc,
    },
    networksystem::ChangeAccessorFieldPathIndex,
};

// scope client.dll
// 2023-10-29 23:18:03.788621600 UTC

pub enum WaterWakeMode {
    WaterWakeNone,
    WaterWakeIdle,
    WaterWakeWalking,
    WaterWakeRunning,
    WaterWakeWaterOverhead,
}

pub enum CompositeMaterialMatchFilterType {
    MatchFilterMaterialAttributeExists,
    MatchFilterMaterialShader,
    MatchFilterMaterialNameSubstr,
    MatchFilterMaterialAttributeEquals,
    MatchFilterMaterialPropertyExists,
    MatchFilterMaterialPropertyEquals,
}

pub enum CompositeMaterialVarSystemVar {
    CompmatsysvarCompositetime,
    CompmatsysvarEmptyResourceSpacer,
}

pub enum CompositeMaterialInputLooseVariableType {
    LooseVariableTypeBoolean,
    LooseVariableTypeInteger1,
    LooseVariableTypeInteger2,
    LooseVariableTypeInteger3,
    LooseVariableTypeInteger4,
    LooseVariableTypeFloat1,
    LooseVariableTypeFloat2,
    LooseVariableTypeFloat3,
    LooseVariableTypeFloat4,
    LooseVariableTypeColor4,
    LooseVariableTypeString,
    LooseVariableTypeSystemvar,
    LooseVariableTypeResourceMaterial,
    LooseVariableTypeResourceTexture,
}

pub enum CompositeMaterialInputTextureType {
    InputTextureTypeDefault,
    InputTextureTypeNormalmap,
    InputTextureTypeColor,
    InputTextureTypeMasks,
    InputTextureTypeRoughness,
    InputTextureTypePearlescenceMask,
    InputTextureTypeAo,
}

pub enum CompMatPropertyMutatorType {
    CompMatPropertyMutatorInit,
    CompMatPropertyMutatorCopyMatchingKeys,
    CompMatPropertyMutatorCopyKeysWithSuffix,
    CompMatPropertyMutatorCopyProperty,
    CompMatPropertyMutatorSetValue,
    CompMatPropertyMutatorGenerateTexture,
    CompMatPropertyMutatorConditionalMutators,
    CompMatPropertyMutatorPopInputQueue,
    CompMatPropertyMutatorDrawText,
    CompMatPropertyMutatorRandomRollInputVariables,
}

pub enum CompMatPropertyMutatorConditionType {
    CompMatMutatorConditionInputContainerExists,
    CompMatMutatorConditionInputContainerValueExists,
    CompMatMutatorConditionInputContainerValueEquals,
}

pub enum CompositeMaterialInputContainerSourceType {
    ContainerSourceTypeTargetMaterial,
    ContainerSourceTypeMaterialFromTargetAttr,
    ContainerSourceTypeSpecificMaterial,
    ContainerSourceTypeLooseVariables,
    ContainerSourceTypeVariableFromTargetAttr,
    ContainerSourceTypeTargetInstanceMaterial,
}

#[derive(Schema)]
#[scope("client.dll")]
pub struct ScriptComponent {
    #[field("CScriptComponent", "m_scriptClassName")]
    script_class_name: UtlSymbolLarge,
}

#[derive(Schema)]
#[scope("client.dll")]
pub struct EntityIdentity {
    #[field("CEntityIdentity", "m_nameStringableIndex")]
    name_stringable_index: i32,
    #[field("CEntityIdentity", "m_name")]
    name: UtlSymbolLarge,
    #[field("CEntityIdentity", "m_designerName")]
    designer_name: UtlSymbolLarge,
    #[field("CEntityIdentity", "m_flags")]
    flags: u32,
    #[field("CEntityIdentity", "m_worldGroupId")]
    world_group_id: WorldGroupId,
    #[field("CEntityIdentity", "m_fDataObjectTypes")]
    data_object_types: u32,
    #[field("CEntityIdentity", "m_PathIndex")]
    path_index: ChangeAccessorFieldPathIndex,
    #[field("CEntityIdentity", "m_pPrev")]
    prev: *const EntityIdentity,
    #[field("CEntityIdentity", "m_pNext")]
    next: *const EntityIdentity,
    #[field("CEntityIdentity", "m_pPrevByClass")]
    prev_by_class: *const EntityIdentity,
    #[field("CEntityIdentity", "m_pNextByClass")]
    next_by_class: *const EntityIdentity,
}

#[derive(Schema)]
#[scope("client.dll")]
pub struct EntityInstance {
    #[field("CEntityInstance", "m_pEntity")]
    entity: *const EntityIdentity,
    #[field("CEntityInstance", "m_CScriptComponent")]
    script_component: *const ScriptComponent,
}

#[derive(Schema)]
#[scope("client.dll")]
pub struct GameSceneNodeHandle {
    #[field("CGameSceneNodeHandle", "m_hOwner")]
    owner: EntityHandle,
    #[field("CGameSceneNodeHandle", "m_name")]
    name: UtlStringToken,
}

#[derive(Schema)]
#[scope("client.dll")]
pub struct GameSceneNode {
    #[field("CGameSceneNode", "m_nodeToWorld")]
    node_to_world: Transform,
    #[field("CGameSceneNode", "m_pOwner")]
    owner: *const EntityInstance,
    #[field("CGameSceneNode", "m_pParent")]
    p_parent: *const GameSceneNode,
    #[field("CGameSceneNode", "m_pChild")]
    child: *const GameSceneNode,
    #[field("CGameSceneNode", "m_pNextSibling")]
    next_sibling: *const GameSceneNode,
    #[field("CGameSceneNode", "m_hParent")]
    h_parent: GameSceneNodeHandle,
    #[field("CGameSceneNode", "m_vecOrigin")]
    origin: NetworkOriginCellCoordQuantizedVector,
    #[field("CGameSceneNode", "m_angRotation")]
    rotation: QAngle,
    #[field("CGameSceneNode", "m_flScale")]
    scale: f32,
    #[field("CGameSceneNode", "m_vecAbsOrigin")]
    abs_origin: Vec3,
    #[field("CGameSceneNode", "m_angAbsRotation")]
    abs_rotation: QAngle,
    #[field("CGameSceneNode", "m_flAbsScale")]
    abs_scale: f32,
    #[field("CGameSceneNode", "m_nParentAttachmentOrBone")]
    parent_attachment_or_bone: i16,
    #[field("CGameSceneNode", "m_bDebugAbsOriginChanges")]
    debug_abs_origin_changes: bool,
    #[field("CGameSceneNode", "m_bDormant")]
    dormant: bool,
    #[field("CGameSceneNode", "m_bForceParentToBeNetworked")]
    force_parent_to_be_networked: bool,
    #[field("CGameSceneNode", "m_bDirtyHierarchy")]
    dirty_hierarchy: u8,
    #[field("CGameSceneNode", "m_bDirtyBoneMergeInfo")]
    dirty_bone_merge_info: u8,
    #[field("CGameSceneNode", "m_bNetworkedPositionChanged")]
    networked_position_changed: u8,
    #[field("CGameSceneNode", "m_bNetworkedAnglesChanged")]
    networked_angles_changed: u8,
    #[field("CGameSceneNode", "m_bNetworkedScaleChanged")]
    networked_scale_changed: u8,
    #[field("CGameSceneNode", "m_bWillBeCallingPostDataUpdate")]
    will_be_calling_post_data_update: u8,
    #[field("CGameSceneNode", "m_bNotifyBoneTransformsChanged")]
    notify_bone_transforms_changed: u8,
    #[field("CGameSceneNode", "m_bBoneMergeFlex")]
    bone_merge_flex: u8,
    #[field("CGameSceneNode", "m_nLatchAbsOrigin")]
    latch_abs_origin: u8,
    #[field("CGameSceneNode", "m_bDirtyBoneMergeBoneToRoot")]
    dirty_bone_merge_bone_to_root: u8,
    #[field("CGameSceneNode", "m_nHierarchicalDepth")]
    hierarchical_depth: u8,
    #[field("CGameSceneNode", "m_nHierarchyType")]
    hierarchy_type: u8,
    #[field("CGameSceneNode", "m_nDoNotSetAnimTimeInInvalidatePhysicsCount")]
    do_not_set_anim_time_in_invalidate_physics_count: u8,
    #[field("CGameSceneNode", "m_name")]
    name: UtlStringToken,
    #[field("CGameSceneNode", "m_hierarchyAttachName")]
    hierarchy_attach_name: UtlStringToken,
    #[field("CGameSceneNode", "m_flZOffset")]
    z_offset: f32,
    #[field("CGameSceneNode", "m_vRenderOrigin")]
    render_origin: Vector,
}

#[derive(Schema)]
#[scope("client.dll")]
pub struct BodyComponent {
    #[field("CBodyComponent", "m_pSceneNode")]
    scene_node: *const GameSceneNode,
    #[field("CBodyComponent", "__m_pChainEntity")]
    chain_entity: NetworkVarChainer,
}

#[derive(Schema)]
#[scope("client.dll")]
pub struct RenderComponent {
    #[field("CRenderComponent", "__m_pChainEntity")]
    chain_entity: NetworkVarChainer,
    #[field("CRenderComponent", "m_bIsRenderingWithViewModels")]
    is_rendering_with_view_models: bool,
    #[field("CRenderComponent", "m_nSplitscreenFlags")]
    splitscreen_flags: u32,
    #[field("CRenderComponent", "m_bEnableRendering")]
    enable_rendering: bool,
    #[field("CRenderComponent", "m_bInterpolationReadyToDraw")]
    interpolation_ready_to_draw: bool,
}

#[derive(Schema)]
#[scope("client.dll")]
pub struct VPhysicsCollisionAttribute {
    #[field("VPhysicsCollisionAttribute_t", "m_nInteractsAs")]
    interacts_as: u64,
    #[field("VPhysicsCollisionAttribute_t", "m_nInteractsWith")]
    interacts_with: u64,
    #[field("VPhysicsCollisionAttribute_t", "m_nInteractsExclude")]
    interacts_exclude: u64,
    #[field("VPhysicsCollisionAttribute_t", "m_nEntityId")]
    entity_id: u32,
    #[field("VPhysicsCollisionAttribute_t", "m_nOwnerId")]
    owner_id: u32,
    #[field("VPhysicsCollisionAttribute_t", "m_nHierarchyId")]
    hierarchy_id: u16,
    #[field("VPhysicsCollisionAttribute_t", "m_nCollisionGroup")]
    collision_group: u8,
    #[field("VPhysicsCollisionAttribute_t", "m_nCollisionFunctionMask")]
    collision_function_mask: u8,
}

#[derive(Schema)]
#[scope("client.dll")]
pub struct CollisionProperty {
    #[field("CCollisionProperty", "m_collisionAttribute")]
    collision_attribute: VPhysicsCollisionAttribute,
    #[field("CCollisionProperty", "m_vecMins")]
    mins: Vec3,
    #[field("CCollisionProperty", "m_vecMaxs")]
    maxs: Vec3,
    #[field("CCollisionProperty", "m_usSolidFlags")]
    us_solid_flags: u8,
    #[field("CCollisionProperty", "m_nSolidType")]
    solid_type: SolidType,
    #[field("CCollisionProperty", "m_triggerBloat")]
    trigger_bloat: u8,
    #[field("CCollisionProperty", "m_nSurroundType")]
    surround_type: SurroundingBoundsType,
    #[field("CCollisionProperty", "m_CollisionGroup")]
    collision_group: u8,
    #[field("CCollisionProperty", "m_nEnablePhysics")]
    enable_physics: u8,
    #[field("CCollisionProperty", "m_flBoundingRadius")]
    bounding_radius: f32,
    #[field("CCollisionProperty", "m_vecSpecifiedSurroundingMins")]
    specified_surrounding_mins: Vector,
    #[field("CCollisionProperty", "m_vecSpecifiedSurroundingMaxs")]
    specified_surrounding_maxs: Vector,
    #[field("CCollisionProperty", "m_vecSurroundingMaxs")]
    surrounding_maxs: Vector,
    #[field("CCollisionProperty", "m_vecSurroundingMins")]
    surrounding_mins: Vector,
    #[field("CCollisionProperty", "m_vCapsuleCenter1")]
    capsule_center_1: Vector,
    #[field("CCollisionProperty", "m_vCapsuleCenter2")]
    capsule_center_2: Vector,
    #[field("CCollisionProperty", "m_flCapsuleRadius")]
    capsule_radius: f32,
}

#[derive(Schema)]
#[scope("client.dll")]
pub struct BaseEntity {
    #[field("CEntityInstance", "m_iszPrivateVScripts")]
    private_v_scripts: UtlSymbolLarge,
    #[field("CEntityInstance", "m_pEntity")]
    entity: *const EntityIdentity,
    #[field("CEntityInstance", "m_CScriptComponent")]
    script_component: *const ScriptComponent,
    #[field("C_BaseEntity", "m_CBodyComponent")]
    body_component: *const BodyComponent,
    #[field("C_BaseEntity", "m_NetworkTransmitComponent")]
    network_transmit_component: NetworkTransmitComponent,
    #[field("C_BaseEntity", "m_nLastThinkTick")]
    last_think_tick: GameTick,
    #[field("C_BaseEntity", "m_pGameSceneNode")]
    game_scene_node: *const GameSceneNode,
    #[field("C_BaseEntity", "m_pRenderComponent")]
    render_component: *const RenderComponent,
    #[field("C_BaseEntity", "m_pCollision")]
    collision: *const CollisionProperty,
    #[field("C_BaseEntity", "m_iMaxHealth")]
    max_health: i32,
    #[field("C_BaseEntity", "m_iHealth")]
    health: i32,
    #[field("C_BaseEntity", "m_lifeState")]
    life_state: u8,
    #[field("C_BaseEntity", "m_bTakesDamage")]
    takes_damage: bool,
    #[field("C_BaseEntity", "m_nTakeDamageFlags")]
    take_damage_flags: TakeDamageFlags,
    #[field("C_BaseEntity", "m_ubInterpolationFrame")]
    interpolation_frame: u8,
    #[field("C_BaseEntity", "m_hSceneObjectController")]
    scene_object_controller: Handle<BaseEntity>,
    #[field("C_BaseEntity", "m_nNoInterpolationTick")]
    no_interpolation_tick: i32,
    #[field("C_BaseEntity", "m_nVisibilityNoInterpolationTick")]
    visibility_no_interpolation_tick: i32,
    #[field("C_BaseEntity", "m_flProxyRandomValue")]
    proxy_random_value: f32,
    #[field("C_BaseEntity", "m_iEFlags")]
    e_flags: i32,
    #[field("C_BaseEntity", "m_nWaterType")]
    water_type: u8,
    #[field("C_BaseEntity", "m_bInterpolateEvenWithNoModel")]
    interpolate_even_with_no_model: bool,
    #[field("C_BaseEntity", "m_bPredictionEligible")]
    prediction_eligible: bool,
    #[field("C_BaseEntity", "m_bApplyLayerMatchIDToModel")]
    apply_layer_match_id_to_model: bool,
    #[field("C_BaseEntity", "m_tokLayerMatchID")]
    tok_layer_match_id: UtlStringToken,
    #[field("C_BaseEntity", "m_nSubclassID")]
    subclass_id: UtlStringToken,
    #[field("C_BaseEntity", "m_nSimulationTick")]
    simulation_tick: i32,
    #[field("C_BaseEntity", "m_iCurrentThinkContext")]
    current_think_context: i32,
    #[field("C_BaseEntity", "m_aThinkFunctions")]
    think_functions: UtlVector<Thinkfunc>,
    #[field("C_BaseEntity", "m_flAnimTime")]
    anim_time: f32,
    #[field("C_BaseEntity", "m_flSimulationTime")]
    simulation_time: f32,
    #[field("C_BaseEntity", "m_nSceneObjectOverrideFlags")]
    scene_object_override_flags: u8,
    #[field("C_BaseEntity", "m_bHasSuccessfullyInterpolated")]
    has_successfully_interpolated: bool,
    #[field("C_BaseEntity", "m_bHasAddedVarsToInterpolation")]
    has_added_vars_to_interpolation: bool,
    #[field("C_BaseEntity", "m_bRenderEvenWhenNotSuccessfullyInterpolated")]
    render_even_when_not_successfully_interpolated: bool,
    #[field("C_BaseEntity", "m_nInterpolationLatchDirtyFlags")]
    interpolation_latch_dirty_flags: [i32; 2],
    #[field("C_BaseEntity", "m_ListEntry")]
    list_entry: [u16; 11],
    #[field("C_BaseEntity", "m_flCreateTime")]
    create_time: GameTime,
    #[field("C_BaseEntity", "m_flSpeed")]
    speed: f32,
    #[field("C_BaseEntity", "m_EntClientFlags")]
    ent_client_flags: u16,
    #[field("C_BaseEntity", "m_bClientSideRagdoll")]
    client_side_ragdoll: bool,
    #[field("C_BaseEntity", "m_iTeamNum")]
    team_num: u8,
    #[field("C_BaseEntity", "m_spawnflags")]
    spawnflags: u32,
    #[field("C_BaseEntity", "m_nNextThinkTick")]
    next_think_tick: GameTick,
    #[field("C_BaseEntity", "m_fFlags")]
    flags: u32,
    #[field("C_BaseEntity", "m_vecAbsVelocity")]
    abs_velocity: Vector,
    #[field("C_BaseEntity", "m_vecVelocity")]
    velocity: NetworkVelocityVector,
    #[field("C_BaseEntity", "m_vecBaseVelocity")]
    base_velocity: Vector,
    #[field("C_BaseEntity", "m_hEffectEntity")]
    effect_entity: Handle<BaseEntity>,
    #[field("C_BaseEntity", "m_hOwnerEntity")]
    owner_entity: Handle<BaseEntity>,
    #[field("C_BaseEntity", "m_MoveCollide")]
    move_collide: MoveCollide,
    #[field("C_BaseEntity", "m_MoveType")]
    move_type: MoveType,
    #[field("C_BaseEntity", "m_flWaterLevel")]
    water_level: f32,
    #[field("C_BaseEntity", "m_fEffects")]
    effects: u32,
    #[field("C_BaseEntity", "m_hGroundEntity")]
    ground_entity: Handle<BaseEntity>,
    #[field("C_BaseEntity", "m_flFriction")]
    friction: f32,
    #[field("C_BaseEntity", "m_flElasticity")]
    elasticity: f32,
    #[field("C_BaseEntity", "m_flGravityScale")]
    gravity_scale: f32,
    #[field("C_BaseEntity", "m_flTimeScale")]
    time_scale: f32,
    #[field("C_BaseEntity", "m_bSimulatedEveryTick")]
    simulated_every_tick: bool,
    #[field("C_BaseEntity", "m_bAnimatedEveryTick")]
    animated_every_tick: bool,
    #[field("C_BaseEntity", "m_flNavIgnoreUntilTime")]
    nav_ignore_until_time: GameTime,
    #[field("C_BaseEntity", "m_hThink")]
    think: u16,
    #[field("C_BaseEntity", "m_fBBoxVisFlags")]
    b_box_vis_flags: u8,
    #[field("C_BaseEntity", "m_bPredictable")]
    predictable: bool,
    #[field("C_BaseEntity", "m_bRenderWithViewModels")]
    render_with_view_models: bool,
    #[field("C_BaseEntity", "m_nSplitUserPlayerPredictionSlot")]
    split_user_player_prediction_slot: SplitScreenSlot,
    #[field("C_BaseEntity", "m_nFirstPredictableCommand")]
    first_predictable_command: i32,
    #[field("C_BaseEntity", "m_nLastPredictableCommand")]
    last_predictable_command: i32,
    #[field("C_BaseEntity", "m_hOldMoveParent")]
    old_move_parent: Handle<BaseEntity>,
    #[field("C_BaseEntity", "m_Particles")]
    particles: ParticleProperty,
    #[field("C_BaseEntity", "m_vecPredictedScriptFloats")]
    predicted_script_floats: UtlVector<f32>,
    #[field("C_BaseEntity", "m_vecPredictedScriptFloatIDs")]
    predicted_script_float_i_ds: UtlVector<i32>,
    #[field("C_BaseEntity", "m_nNextScriptVarRecordID")]
    next_script_var_record_id: i32,
    #[field("C_BaseEntity", "m_vecAngVelocity")]
    ang_velocity: QAngle,
    #[field("C_BaseEntity", "m_DataChangeEventRef")]
    data_change_event_ref: i32,
    #[field("C_BaseEntity", "m_dependencies")]
    dependencies: UtlVector<EntityHandle>,
    #[field("C_BaseEntity", "m_nCreationTick")]
    creation_tick: i32,
    #[field("C_BaseEntity", "m_bAnimTimeChanged")]
    anim_time_changed: bool,
    #[field("C_BaseEntity", "m_bSimulationTimeChanged")]
    simulation_time_changed: bool,
    #[field("C_BaseEntity", "m_sUniqueHammerID")]
    unique_hammer_id: UtlString,
}

#[derive(Schema)]
#[scope("client.dll")]
pub struct HitboxComponent {
    #[field("CHitboxComponent", "m_bvDisabledHitGroups")]
    disabled_hit_groups: [u32; 1],
}

#[derive(Schema)]
#[scope("client.dll")]
pub struct EntityRenderAttribute {
    #[field("EntityRenderAttribute_t", "m_ID")]
    id: UtlStringToken,
    #[field("EntityRenderAttribute_t", "m_Values")]
    values: Vector4D,
}

#[derive(Schema)]
#[scope("client.dll")]
pub struct GlowProperty {
    #[field("CGlowProperty", "m_fGlowColor")]
    glow_color: Vector,
    #[field("CGlowProperty", "m_iGlowType")]
    glow_type: i32,
    #[field("CGlowProperty", "m_iGlowTeam")]
    glow_team: i32,
    #[field("CGlowProperty", "m_nGlowRange")]
    glow_range: i32,
    #[field("CGlowProperty", "m_nGlowRangeMin")]
    glow_range_min: i32,
    #[field("CGlowProperty", "m_glowColorOverride")]
    glow_color_override: Color,
    #[field("CGlowProperty", "m_bFlashing")]
    flashing: bool,
    #[field("CGlowProperty", "m_flGlowTime")]
    glow_time: f32,
    #[field("CGlowProperty", "m_flGlowStartTime")]
    glow_start_time: f32,
    #[field("CGlowProperty", "m_bEligibleForScreenHighlight")]
    eligible_for_screen_highlight: bool,
    #[field("CGlowProperty", "m_bGlowing")]
    glowing: bool,
}

#[derive(Schema)]
#[scope("client.dll")]
pub struct BaseModelEntity {}

#[derive(Schema)]
#[scope("client.dll")]
pub struct BaseAnimGraph {}

#[derive(Schema)]
#[scope("client.dll")]
pub struct PhysicsRagdollPose {
    #[field("PhysicsRagdollPose_t", "__m_pChainEntity")]
    chain_entity: NetworkVarChainer,
    #[field("PhysicsRagdollPose_t", "m_Transforms")]
    transforms: UtlVector<Transform>,
    #[field("PhysicsRagdollPose_t", "m_hOwner")]
    owner: Handle<BaseEntity>,
    #[field("PhysicsRagdollPose_t", "m_bDirty")]
    dirty: bool,
}

#[derive(Schema)]
#[scope("client.dll")]
pub struct BaseFlexEmphasizedPhoneme {
    #[field("C_BaseFlex::Emphasized_Phoneme", "m_sClassName")]
    class_name: UtlString,
    #[field("C_BaseFlex::Emphasized_Phoneme", "m_flAmount")]
    amount: f32,
    #[field("C_BaseFlex::Emphasized_Phoneme", "m_bRequired")]
    required: bool,
    #[field("C_BaseFlex::Emphasized_Phoneme", "m_bBasechecked")]
    basechecked: bool,
    #[field("C_BaseFlex::Emphasized_Phoneme", "m_bValid")]
    valid: bool,
}

#[derive(Schema)]
#[scope("client.dll")]
pub struct AttributeManagercachedAttributeFloat {
    #[field("CAttributeManager::cached_attribute_float_t", "flIn")]
    fl_in: f32,
    #[field("CAttributeManager::cached_attribute_float_t", "iAttribHook")]
    i_attrib_hook: UtlSymbolLarge,
    #[field("CAttributeManager::cached_attribute_float_t", "flOut")]
    fl_out: f32,
}

#[derive(Schema)]
#[scope("client.dll")]
pub struct EconItemAttribute {
    #[field("CEconItemAttribute", "m_iAttributeDefinitionIndex")]
    attribute_definition_index: u16,
    #[field("CEconItemAttribute", "m_flValue")]
    value: f32,
    #[field("CEconItemAttribute", "m_flInitialValue")]
    initial_value: f32,
    #[field("CEconItemAttribute", "m_nRefundableCurrency")]
    refundable_currency: i32,
    #[field("CEconItemAttribute", "m_bSetBonus")]
    set_bonus: bool,
}

#[derive(Schema)]
#[scope("client.dll")]
pub struct AttributeManager {
    #[field("CAttributeManager", "m_Providers")]
    providers: UtlVector<Handle<BaseEntity>>,
    #[field("CAttributeManager", "m_iReapplyProvisionParity")]
    reapply_provision_parity: i32,
    #[field("CAttributeManager", "m_hOuter")]
    outer: Handle<BaseEntity>,
    #[field("CAttributeManager", "m_bPreventLoopback")]
    prevent_loopback: bool,
    #[field("CAttributeManager", "m_ProviderType")]
    provider_type: AttributeProviderTypes,
    #[field("CAttributeManager", "m_CachedResults")]
    cached_results: UtlVector<AttributeManagercachedAttributeFloat>,
}

#[derive(Schema)]
#[scope("client.dll")]
pub struct AttributeList {
    #[field("CAttributeList", "m_Attributes")]
    attributes: UtlVector<EconItemAttribute>,
    #[field("CAttributeList", "m_pManager")]
    manager: *const AttributeManager,
}

#[derive(Schema)]
#[scope("client.dll")]
pub struct EconItemView {
    #[field("C_EconItemView", "m_bInventoryImageRgbaRequested")]
    inventory_image_rgba_requested: bool,
    #[field("C_EconItemView", "m_bInventoryImageTriedCache")]
    inventory_image_tried_cache: bool,
    #[field("C_EconItemView", "m_nInventoryImageRgbaWidth")]
    inventory_image_rgba_width: i32,
    #[field("C_EconItemView", "m_nInventoryImageRgbaHeight")]
    inventory_image_rgba_height: i32,
    #[field("C_EconItemView", "m_szCurrentLoadCachedFileName")]
    current_load_cached_file_name: [std::ffi::c_char; 260],
    #[field("C_EconItemView", "m_bRestoreCustomMaterialAfterPrecache")]
    restore_custom_material_after_precache: bool,
    #[field("C_EconItemView", "m_iItemDefinitionIndex")]
    item_definition_index: u16,
    #[field("C_EconItemView", "m_iEntityQuality")]
    entity_quality: i32,
    #[field("C_EconItemView", "m_iEntityLevel")]
    entity_level: u32,
    #[field("C_EconItemView", "m_iItemID")]
    item_id: u64,
    #[field("C_EconItemView", "m_iItemIDHigh")]
    item_id_high: u32,
    #[field("C_EconItemView", "m_iItemIDLow")]
    item_id_low: u32,
    #[field("C_EconItemView", "m_iAccountID")]
    account_id: u32,
    #[field("C_EconItemView", "m_iInventoryPosition")]
    inventory_position: u32,
    #[field("C_EconItemView", "m_bInitialized")]
    initialized: bool,
    #[field("C_EconItemView", "m_bIsStoreItem")]
    is_store_item: bool,
    #[field("C_EconItemView", "m_bIsTradeItem")]
    is_trade_item: bool,
    #[field("C_EconItemView", "m_iEntityQuantity")]
    entity_quantity: i32,
    #[field("C_EconItemView", "m_iRarityOverride")]
    rarity_override: i32,
    #[field("C_EconItemView", "m_iQualityOverride")]
    quality_override: i32,
    #[field("C_EconItemView", "m_unClientFlags")]
    client_flags: u8,
    #[field("C_EconItemView", "m_unOverrideStyle")]
    override_style: u8,
    #[field("C_EconItemView", "m_AttributeList")]
    attribute_list: AttributeList,
    #[field("C_EconItemView", "m_NetworkedDynamicAttributes")]
    networked_dynamic_attributes: AttributeList,
    #[field("C_EconItemView", "m_szCustomName")]
    custom_name: [std::ffi::c_char; 161],
    #[field("C_EconItemView", "m_szCustomNameOverride")]
    custom_name_override: [std::ffi::c_char; 161],
    #[field("C_EconItemView", "m_bInitializedTags")]
    initialized_tags: bool,
}

#[derive(Schema)]
#[scope("client.dll")]
pub struct AttributeContainer {
    #[field("CAttributeManager", "m_Providers")]
    providers: UtlVector<Handle<BaseEntity>>,
    #[field("CAttributeManager", "m_iReapplyProvisionParity")]
    reapply_provision_parity: i32,
    #[field("CAttributeManager", "m_hOuter")]
    outer: Handle<BaseEntity>,
    #[field("CAttributeManager", "m_bPreventLoopback")]
    prevent_loopback: bool,
    #[field("CAttributeManager", "m_ProviderType")]
    provider_type: AttributeProviderTypes,
    #[field("CAttributeManager", "m_CachedResults")]
    cached_results: UtlVector<AttributeManagercachedAttributeFloat>,
    #[field("C_AttributeContainer", "m_Item")]
    item: EconItemView,
    #[field("C_AttributeContainer", "m_iExternalItemProviderRegisteredToken")]
    external_item_provider_registered_token: i32,
    #[field("C_AttributeContainer", "m_ullRegisteredAsItemID")]
    ull_registered_as_item_id: u64,
}

#[derive(Schema)]
#[scope("client.dll")]
pub struct EconEntityAttachedModelData {
    #[field("C_EconEntity::AttachedModelData_t", "m_iModelDisplayFlags")]
    model_display_flags: i32,
}

#[derive(Schema)]
#[scope("client.dll")]
pub struct EconWearable {
    #[field("CEntityInstance", "m_iszPrivateVScripts")]
    private_v_scripts: UtlSymbolLarge,
    #[field("CEntityInstance", "m_pEntity")]
    entity: *const EntityIdentity,
    #[field("CEntityInstance", "m_CScriptComponent")]
    script_component: *const ScriptComponent,
    #[field("C_BaseEntity", "m_CBodyComponent")]
    body_component: *const BodyComponent,
    #[field("C_BaseEntity", "m_NetworkTransmitComponent")]
    network_transmit_component: NetworkTransmitComponent,
    #[field("C_BaseEntity", "m_nLastThinkTick")]
    last_think_tick: GameTick,
    #[field("C_BaseEntity", "m_pGameSceneNode")]
    game_scene_node: *const GameSceneNode,
    #[field("C_BaseEntity", "m_pRenderComponent")]
    p_render_component: *const RenderComponent,
    #[field("C_BaseEntity", "m_pCollision")]
    p_collision: *const CollisionProperty,
    #[field("C_BaseEntity", "m_iMaxHealth")]
    max_health: i32,
    #[field("C_BaseEntity", "m_iHealth")]
    health: i32,
    #[field("C_BaseEntity", "m_lifeState")]
    life_state: u8,
    #[field("C_BaseEntity", "m_bTakesDamage")]
    takes_damage: bool,
    #[field("C_BaseEntity", "m_nTakeDamageFlags")]
    take_damage_flags: TakeDamageFlags,
    #[field("C_BaseEntity", "m_ubInterpolationFrame")]
    interpolation_frame: u8,
    #[field("C_BaseEntity", "m_hSceneObjectController")]
    scene_object_controller: Handle<BaseEntity>,
    #[field("C_BaseEntity", "m_nNoInterpolationTick")]
    no_interpolation_tick: i32,
    #[field("C_BaseEntity", "m_nVisibilityNoInterpolationTick")]
    visibility_no_interpolation_tick: i32,
    #[field("C_BaseEntity", "m_flProxyRandomValue")]
    proxy_random_value: f32,
    #[field("C_BaseEntity", "m_iEFlags")]
    e_flags: i32,
    #[field("C_BaseEntity", "m_nWaterType")]
    water_type: u8,
    #[field("C_BaseEntity", "m_bInterpolateEvenWithNoModel")]
    interpolate_even_with_no_model: bool,
    #[field("C_BaseEntity", "m_bPredictionEligible")]
    prediction_eligible: bool,
    #[field("C_BaseEntity", "m_bApplyLayerMatchIDToModel")]
    apply_layer_match_id_to_model: bool,
    #[field("C_BaseEntity", "m_tokLayerMatchID")]
    tok_layer_match_id: UtlStringToken,
    #[field("C_BaseEntity", "m_nSubclassID")]
    subclass_id: UtlStringToken,
    #[field("C_BaseEntity", "m_nSimulationTick")]
    simulation_tick: i32,
    #[field("C_BaseEntity", "m_iCurrentThinkContext")]
    current_think_context: i32,
    #[field("C_BaseEntity", "m_aThinkFunctions")]
    think_functions: UtlVector<Thinkfunc>,
    #[field("C_BaseEntity", "m_flAnimTime")]
    anim_time: f32,
    #[field("C_BaseEntity", "m_flSimulationTime")]
    simulation_time: f32,
    #[field("C_BaseEntity", "m_nSceneObjectOverrideFlags")]
    scene_object_override_flags: u8,
    #[field("C_BaseEntity", "m_bHasSuccessfullyInterpolated")]
    has_successfully_interpolated: bool,
    #[field("C_BaseEntity", "m_bHasAddedVarsToInterpolation")]
    has_added_vars_to_interpolation: bool,
    #[field("C_BaseEntity", "m_bRenderEvenWhenNotSuccessfullyInterpolated")]
    render_even_when_not_successfully_interpolated: bool,
    #[field("C_BaseEntity", "m_nInterpolationLatchDirtyFlags")]
    interpolation_latch_dirty_flags: [i32; 2],
    #[field("C_BaseEntity", "m_ListEntry")]
    list_entry: [u16; 11],
    #[field("C_BaseEntity", "m_flCreateTime")]
    create_time: GameTime,
    #[field("C_BaseEntity", "m_flSpeed")]
    speed: f32,
    #[field("C_BaseEntity", "m_EntClientFlags")]
    ent_client_flags: u16,
    #[field("C_BaseEntity", "m_bClientSideRagdoll")]
    client_side_ragdoll: bool,
    #[field("C_BaseEntity", "m_iTeamNum")]
    team_num: u8,
    #[field("C_BaseEntity", "m_spawnflags")]
    spawnflags: u32,
    #[field("C_BaseEntity", "m_nNextThinkTick")]
    next_think_tick: GameTick,
    #[field("C_BaseEntity", "m_fFlags")]
    flags: u32,
    #[field("C_BaseEntity", "m_vecAbsVelocity")]
    abs_velocity: Vector,
    #[field("C_BaseEntity", "m_vecVelocity")]
    velocity: NetworkVelocityVector,
    #[field("C_BaseEntity", "m_vecBaseVelocity")]
    base_velocity: Vector,
    #[field("C_BaseEntity", "m_hEffectEntity")]
    effect_entity: Handle<BaseEntity>,
    #[field("C_BaseEntity", "m_hOwnerEntity")]
    owner_entity: Handle<BaseEntity>,
    #[field("C_BaseEntity", "m_MoveCollide")]
    move_collide: MoveCollide,
    #[field("C_BaseEntity", "m_MoveType")]
    move_type: MoveType,
    #[field("C_BaseEntity", "m_flWaterLevel")]
    water_level: f32,
    #[field("C_BaseEntity", "m_fEffects")]
    effects: u32,
    #[field("C_BaseEntity", "m_hGroundEntity")]
    ground_entity: Handle<BaseEntity>,
    #[field("C_BaseEntity", "m_flFriction")]
    friction: f32,
    #[field("C_BaseEntity", "m_flElasticity")]
    elasticity: f32,
    #[field("C_BaseEntity", "m_flGravityScale")]
    gravity_scale: f32,
    #[field("C_BaseEntity", "m_flTimeScale")]
    time_scale: f32,
    #[field("C_BaseEntity", "m_bSimulatedEveryTick")]
    simulated_every_tick: bool,
    #[field("C_BaseEntity", "m_bAnimatedEveryTick")]
    animated_every_tick: bool,
    #[field("C_BaseEntity", "m_flNavIgnoreUntilTime")]
    nav_ignore_until_time: GameTime,
    #[field("C_BaseEntity", "m_hThink")]
    think: u16,
    #[field("C_BaseEntity", "m_fBBoxVisFlags")]
    b_box_vis_flags: u8,
    #[field("C_BaseEntity", "m_bPredictable")]
    predictable: bool,
    #[field("C_BaseEntity", "m_bRenderWithViewModels")]
    render_with_view_models: bool,
    #[field("C_BaseEntity", "m_nSplitUserPlayerPredictionSlot")]
    split_user_player_prediction_slot: SplitScreenSlot,
    #[field("C_BaseEntity", "m_nFirstPredictableCommand")]
    first_predictable_command: i32,
    #[field("C_BaseEntity", "m_nLastPredictableCommand")]
    last_predictable_command: i32,
    #[field("C_BaseEntity", "m_hOldMoveParent")]
    old_move_parent: Handle<BaseEntity>,
    #[field("C_BaseEntity", "m_Particles")]
    particles: ParticleProperty,
    #[field("C_BaseEntity", "m_vecPredictedScriptFloats")]
    predicted_script_floats: UtlVector<f32>,
    #[field("C_BaseEntity", "m_vecPredictedScriptFloatIDs")]
    predicted_script_float_i_ds: UtlVector<i32>,
    #[field("C_BaseEntity", "m_nNextScriptVarRecordID")]
    next_script_var_record_id: i32,
    #[field("C_BaseEntity", "m_vecAngVelocity")]
    ang_velocity: QAngle,
    #[field("C_BaseEntity", "m_DataChangeEventRef")]
    data_change_event_ref: i32,
    #[field("C_BaseEntity", "m_dependencies")]
    dependencies: UtlVector<EntityHandle>,
    #[field("C_BaseEntity", "m_nCreationTick")]
    creation_tick: i32,
    #[field("C_BaseEntity", "m_bAnimTimeChanged")]
    anim_time_changed: bool,
    #[field("C_BaseEntity", "m_bSimulationTimeChanged")]
    simulation_time_changed: bool,
    #[field("C_BaseEntity", "m_sUniqueHammerID")]
    unique_hammer_id: UtlString,
    #[field("C_BaseModelEntity", "m_CRenderComponent")]
    c_render_component: *const RenderComponent,
    #[field("C_BaseModelEntity", "m_CHitboxComponent")]
    hitbox_component: HitboxComponent,
    #[field("C_BaseModelEntity", "m_bInitModelEffects")]
    init_model_effects: bool,
    #[field("C_BaseModelEntity", "m_bIsStaticProp")]
    is_static_prop: bool,
    #[field("C_BaseModelEntity", "m_nLastAddDecal")]
    last_add_decal: i32,
    #[field("C_BaseModelEntity", "m_nDecalsAdded")]
    decals_added: i32,
    #[field("C_BaseModelEntity", "m_iOldHealth")]
    old_health: i32,
    #[field("C_BaseModelEntity", "m_nRenderMode")]
    render_mode: RenderMode,
    #[field("C_BaseModelEntity", "m_nRenderFX")]
    render_fx: RenderFx,
    #[field("C_BaseModelEntity", "m_bAllowFadeInView")]
    allow_fade_in_view: bool,
    #[field("C_BaseModelEntity", "m_clrRender")]
    render: Color,
    #[field("C_BaseModelEntity", "m_vecRenderAttributes")]
    render_attributes: UtlVector<EntityRenderAttribute>,
    #[field("C_BaseModelEntity", "m_LightGroup")]
    light_group: UtlStringToken,
    #[field("C_BaseModelEntity", "m_bRenderToCubemaps")]
    render_to_cubemaps: bool,
    #[field("C_BaseModelEntity", "m_Collision")]
    collision: CollisionProperty,
    #[field("C_BaseModelEntity", "m_Glow")]
    glow: GlowProperty,
    #[field("C_BaseModelEntity", "m_flGlowBackfaceMult")]
    glow_backface_mult: f32,
    #[field("C_BaseModelEntity", "m_fadeMinDist")]
    fade_min_dist: f32,
    #[field("C_BaseModelEntity", "m_fadeMaxDist")]
    fade_max_dist: f32,
    #[field("C_BaseModelEntity", "m_flFadeScale")]
    fade_scale: f32,
    #[field("C_BaseModelEntity", "m_flShadowStrength")]
    shadow_strength: f32,
    #[field("C_BaseModelEntity", "m_nObjectCulling")]
    object_culling: u8,
    #[field("C_BaseModelEntity", "m_nAddDecal")]
    add_decal: i32,
    #[field("C_BaseModelEntity", "m_vDecalPosition")]
    decal_position: Vector,
    #[field("C_BaseModelEntity", "m_vDecalForwardAxis")]
    decal_forward_axis: Vector,
    #[field("C_BaseModelEntity", "m_flDecalHealBloodRate")]
    decal_heal_blood_rate: f32,
    #[field("C_BaseModelEntity", "m_flDecalHealHeightRate")]
    decal_heal_height_rate: f32,
    #[field("C_BaseModelEntity", "m_ConfigEntitiesToPropagateMaterialDecalsTo")]
    config_entities_to_propagate_material_decals_to: UtlVector<Handle<BaseModelEntity>>,
    #[field("C_BaseModelEntity", "m_vecViewOffset")]
    view_offset: NetworkViewOffsetVector,
    #[field("C_BaseModelEntity", "m_pClientAlphaProperty")]
    client_alpha_property: *const ClientAlphaProperty,
    #[field("C_BaseModelEntity", "m_ClientOverrideTint")]
    client_override_tint: Color,
    #[field("C_BaseModelEntity", "m_bUseClientOverrideTint")]
    use_client_override_tint: bool,
    #[field("CBaseAnimGraph", "m_bInitiallyPopulateInterpHistory")]
    initially_populate_interp_history: bool,
    #[field("CBaseAnimGraph", "m_bShouldAnimateDuringGameplayPause")]
    should_animate_during_gameplay_pause: bool,
    #[field("CBaseAnimGraph", "m_bSuppressAnimEventSounds")]
    suppress_anim_event_sounds: bool,
    #[field("CBaseAnimGraph", "m_bAnimGraphUpdateEnabled")]
    anim_graph_update_enabled: bool,
    #[field("CBaseAnimGraph", "m_flMaxSlopeDistance")]
    max_slope_distance: f32,
    #[field("CBaseAnimGraph", "m_vLastSlopeCheckPos")]
    last_slope_check_pos: Vector,
    #[field("CBaseAnimGraph", "m_vecForce")]
    force: Vector,
    #[field("CBaseAnimGraph", "m_nForceBone")]
    force_bone: i32,
    #[field("CBaseAnimGraph", "m_pClientsideRagdoll")]
    clientside_ragdoll: *const BaseAnimGraph,
    #[field("CBaseAnimGraph", "m_bBuiltRagdoll")]
    built_ragdoll: bool,
    #[field("CBaseAnimGraph", "m_pRagdollPose")]
    ragdoll_pose: *const PhysicsRagdollPose,
    #[field("CBaseAnimGraph", "m_bClientRagdoll")]
    client_ragdoll: bool,
    #[field("CBaseAnimGraph", "m_bHasAnimatedMaterialAttributes")]
    has_animated_material_attributes: bool,
    #[field("C_BaseFlex", "m_flexWeight")]
    flex_weight: UtlVector<f32>,
    #[field("C_BaseFlex", "m_vLookTargetPosition")]
    look_target_position: Vector,
    #[field("C_BaseFlex", "m_blinktoggle")]
    blinktoggle: bool,
    #[field("C_BaseFlex", "m_nLastFlexUpdateFrameCount")]
    last_flex_update_frame_count: i32,
    #[field("C_BaseFlex", "m_CachedViewTarget")]
    cached_view_target: Vector,
    #[field("C_BaseFlex", "m_nNextSceneEventId")]
    next_scene_event_id: u32,
    #[field("C_BaseFlex", "m_iBlink")]
    blink: i32,
    #[field("C_BaseFlex", "m_blinktime")]
    blinktime: f32,
    #[field("C_BaseFlex", "m_prevblinktoggle")]
    prevblinktoggle: bool,
    #[field("C_BaseFlex", "m_iJawOpen")]
    jaw_open: i32,
    #[field("C_BaseFlex", "m_flJawOpenAmount")]
    jaw_open_amount: f32,
    #[field("C_BaseFlex", "m_flBlinkAmount")]
    blink_amount: f32,
    #[field("C_BaseFlex", "m_iMouthAttachment")]
    mouth_attachment: AttachmentHandle,
    #[field("C_BaseFlex", "m_iEyeAttachment")]
    eye_attachment: AttachmentHandle,
    #[field("C_BaseFlex", "m_bResetFlexWeightsOnModelChange")]
    reset_flex_weights_on_model_change: bool,
    #[field("C_BaseFlex", "m_nEyeOcclusionRendererBone")]
    eye_occlusion_renderer_bone: i32,
    #[field("C_BaseFlex", "m_mEyeOcclusionRendererCameraToBoneTransform")]
    m_eye_occlusion_renderer_camera_to_bone_transform: Matrix3X4,
    #[field("C_BaseFlex", "m_vEyeOcclusionRendererHalfExtent")]
    eye_occlusion_renderer_half_extent: Vector,
    #[field("C_BaseFlex", "m_PhonemeClasses")]
    phoneme_classes: [BaseFlexEmphasizedPhoneme; 3],
    #[field("C_EconEntity", "m_flFlexDelayTime")]
    flex_delay_time: f32,
    #[field("C_EconEntity", "m_flFlexDelayedWeight")]
    flex_delayed_weight: *const f32,
    #[field("C_EconEntity", "m_bAttributesInitialized")]
    attributes_initialized: bool,
    #[field("C_EconEntity", "m_AttributeManager")]
    attribute_manager: AttributeContainer,
    #[field("C_EconEntity", "m_OriginalOwnerXuidLow")]
    original_owner_xuid_low: u32,
    #[field("C_EconEntity", "m_OriginalOwnerXuidHigh")]
    original_owner_xuid_high: u32,
    #[field("C_EconEntity", "m_nFallbackPaintKit")]
    fallback_paint_kit: i32,
    #[field("C_EconEntity", "m_nFallbackSeed")]
    fallback_seed: i32,
    #[field("C_EconEntity", "m_flFallbackWear")]
    fallback_wear: f32,
    #[field("C_EconEntity", "m_nFallbackStatTrak")]
    fallback_stat_trak: i32,
    #[field("C_EconEntity", "m_bClientside")]
    clientside: bool,
    #[field("C_EconEntity", "m_bParticleSystemsCreated")]
    particle_systems_created: bool,
    #[field("C_EconEntity", "m_vecAttachedParticles")]
    attached_particles: UtlVector<i32>,
    #[field("C_EconEntity", "m_hViewmodelAttachment")]
    viewmodel_attachment: Handle<BaseAnimGraph>,
    #[field("C_EconEntity", "m_iOldTeam")]
    old_team: i32,
    #[field("C_EconEntity", "m_bAttachmentDirty")]
    attachment_dirty: bool,
    #[field("C_EconEntity", "m_nUnloadedModelIndex")]
    unloaded_model_index: i32,
    #[field("C_EconEntity", "m_iNumOwnerValidationRetries")]
    num_owner_validation_retries: i32,
    #[field("C_EconEntity", "m_hOldProvidee")]
    old_providee: Handle<BaseEntity>,
    #[field("C_EconEntity", "m_vecAttachedModels")]
    attached_models: UtlVector<EconEntityAttachedModelData>,
    #[field("C_EconWearable", "m_nForceSkin")]
    force_skin: i32,
    #[field("C_EconWearable", "m_bAlwaysAllow")]
    always_allow: bool,
}

#[derive(Schema)]
#[scope("client.dll")]
pub struct BasePlayerWeapon {
    #[field("CEntityInstance", "m_iszPrivateVScripts")]
    private_v_scripts: UtlSymbolLarge,
    #[field("CEntityInstance", "m_pEntity")]
    entity: *const EntityIdentity,
    #[field("CEntityInstance", "m_CScriptComponent")]
    script_component: *const ScriptComponent,
    #[field("C_BaseEntity", "m_CBodyComponent")]
    body_component: *const BodyComponent,
    #[field("C_BaseEntity", "m_NetworkTransmitComponent")]
    network_transmit_component: NetworkTransmitComponent,
    #[field("C_BaseEntity", "m_nLastThinkTick")]
    last_think_tick: GameTick,
    #[field("C_BaseEntity", "m_pGameSceneNode")]
    game_scene_node: *const GameSceneNode,
    #[field("C_BaseEntity", "m_pRenderComponent")]
    p_render_component: *const RenderComponent,
    #[field("C_BaseEntity", "m_pCollision")]
    p_collision: *const CollisionProperty,
    #[field("C_BaseEntity", "m_iMaxHealth")]
    max_health: i32,
    #[field("C_BaseEntity", "m_iHealth")]
    health: i32,
    #[field("C_BaseEntity", "m_lifeState")]
    life_state: u8,
    #[field("C_BaseEntity", "m_bTakesDamage")]
    takes_damage: bool,
    #[field("C_BaseEntity", "m_nTakeDamageFlags")]
    take_damage_flags: TakeDamageFlags,
    #[field("C_BaseEntity", "m_ubInterpolationFrame")]
    interpolation_frame: u8,
    #[field("C_BaseEntity", "m_hSceneObjectController")]
    scene_object_controller: Handle<BaseEntity>,
    #[field("C_BaseEntity", "m_nNoInterpolationTick")]
    no_interpolation_tick: i32,
    #[field("C_BaseEntity", "m_nVisibilityNoInterpolationTick")]
    visibility_no_interpolation_tick: i32,
    #[field("C_BaseEntity", "m_flProxyRandomValue")]
    proxy_random_value: f32,
    #[field("C_BaseEntity", "m_iEFlags")]
    e_flags: i32,
    #[field("C_BaseEntity", "m_nWaterType")]
    water_type: u8,
    #[field("C_BaseEntity", "m_bInterpolateEvenWithNoModel")]
    interpolate_even_with_no_model: bool,
    #[field("C_BaseEntity", "m_bPredictionEligible")]
    prediction_eligible: bool,
    #[field("C_BaseEntity", "m_bApplyLayerMatchIDToModel")]
    apply_layer_match_id_to_model: bool,
    #[field("C_BaseEntity", "m_tokLayerMatchID")]
    tok_layer_match_id: UtlStringToken,
    #[field("C_BaseEntity", "m_nSubclassID")]
    subclass_id: UtlStringToken,
    #[field("C_BaseEntity", "m_nSimulationTick")]
    simulation_tick: i32,
    #[field("C_BaseEntity", "m_iCurrentThinkContext")]
    current_think_context: i32,
    #[field("C_BaseEntity", "m_aThinkFunctions")]
    think_functions: UtlVector<Thinkfunc>,
    #[field("C_BaseEntity", "m_flAnimTime")]
    anim_time: f32,
    #[field("C_BaseEntity", "m_flSimulationTime")]
    simulation_time: f32,
    #[field("C_BaseEntity", "m_nSceneObjectOverrideFlags")]
    scene_object_override_flags: u8,
    #[field("C_BaseEntity", "m_bHasSuccessfullyInterpolated")]
    has_successfully_interpolated: bool,
    #[field("C_BaseEntity", "m_bHasAddedVarsToInterpolation")]
    has_added_vars_to_interpolation: bool,
    #[field("C_BaseEntity", "m_bRenderEvenWhenNotSuccessfullyInterpolated")]
    render_even_when_not_successfully_interpolated: bool,
    #[field("C_BaseEntity", "m_nInterpolationLatchDirtyFlags")]
    interpolation_latch_dirty_flags: [i32; 2],
    #[field("C_BaseEntity", "m_ListEntry")]
    list_entry: [u16; 11],
    #[field("C_BaseEntity", "m_flCreateTime")]
    create_time: GameTime,
    #[field("C_BaseEntity", "m_flSpeed")]
    speed: f32,
    #[field("C_BaseEntity", "m_EntClientFlags")]
    ent_client_flags: u16,
    #[field("C_BaseEntity", "m_bClientSideRagdoll")]
    client_side_ragdoll: bool,
    #[field("C_BaseEntity", "m_iTeamNum")]
    team_num: u8,
    #[field("C_BaseEntity", "m_spawnflags")]
    spawnflags: u32,
    #[field("C_BaseEntity", "m_nNextThinkTick")]
    next_think_tick: GameTick,
    #[field("C_BaseEntity", "m_fFlags")]
    flags: u32,
    #[field("C_BaseEntity", "m_vecAbsVelocity")]
    abs_velocity: Vector,
    #[field("C_BaseEntity", "m_vecVelocity")]
    velocity: NetworkVelocityVector,
    #[field("C_BaseEntity", "m_vecBaseVelocity")]
    base_velocity: Vector,
    #[field("C_BaseEntity", "m_hEffectEntity")]
    effect_entity: Handle<BaseEntity>,
    #[field("C_BaseEntity", "m_hOwnerEntity")]
    owner_entity: Handle<BaseEntity>,
    #[field("C_BaseEntity", "m_MoveCollide")]
    move_collide: MoveCollide,
    #[field("C_BaseEntity", "m_MoveType")]
    move_type: MoveType,
    #[field("C_BaseEntity", "m_flWaterLevel")]
    water_level: f32,
    #[field("C_BaseEntity", "m_fEffects")]
    effects: u32,
    #[field("C_BaseEntity", "m_hGroundEntity")]
    ground_entity: Handle<BaseEntity>,
    #[field("C_BaseEntity", "m_flFriction")]
    friction: f32,
    #[field("C_BaseEntity", "m_flElasticity")]
    elasticity: f32,
    #[field("C_BaseEntity", "m_flGravityScale")]
    gravity_scale: f32,
    #[field("C_BaseEntity", "m_flTimeScale")]
    time_scale: f32,
    #[field("C_BaseEntity", "m_bSimulatedEveryTick")]
    simulated_every_tick: bool,
    #[field("C_BaseEntity", "m_bAnimatedEveryTick")]
    animated_every_tick: bool,
    #[field("C_BaseEntity", "m_flNavIgnoreUntilTime")]
    nav_ignore_until_time: GameTime,
    #[field("C_BaseEntity", "m_hThink")]
    think: u16,
    #[field("C_BaseEntity", "m_fBBoxVisFlags")]
    b_box_vis_flags: u8,
    #[field("C_BaseEntity", "m_bPredictable")]
    predictable: bool,
    #[field("C_BaseEntity", "m_bRenderWithViewModels")]
    render_with_view_models: bool,
    #[field("C_BaseEntity", "m_nSplitUserPlayerPredictionSlot")]
    split_user_player_prediction_slot: SplitScreenSlot,
    #[field("C_BaseEntity", "m_nFirstPredictableCommand")]
    first_predictable_command: i32,
    #[field("C_BaseEntity", "m_nLastPredictableCommand")]
    last_predictable_command: i32,
    #[field("C_BaseEntity", "m_hOldMoveParent")]
    old_move_parent: Handle<BaseEntity>,
    #[field("C_BaseEntity", "m_Particles")]
    particles: ParticleProperty,
    #[field("C_BaseEntity", "m_vecPredictedScriptFloats")]
    predicted_script_floats: UtlVector<f32>,
    #[field("C_BaseEntity", "m_vecPredictedScriptFloatIDs")]
    predicted_script_float_i_ds: UtlVector<i32>,
    #[field("C_BaseEntity", "m_nNextScriptVarRecordID")]
    next_script_var_record_id: i32,
    #[field("C_BaseEntity", "m_vecAngVelocity")]
    ang_velocity: QAngle,
    #[field("C_BaseEntity", "m_DataChangeEventRef")]
    data_change_event_ref: i32,
    #[field("C_BaseEntity", "m_dependencies")]
    dependencies: UtlVector<EntityHandle>,
    #[field("C_BaseEntity", "m_nCreationTick")]
    creation_tick: i32,
    #[field("C_BaseEntity", "m_bAnimTimeChanged")]
    anim_time_changed: bool,
    #[field("C_BaseEntity", "m_bSimulationTimeChanged")]
    simulation_time_changed: bool,
    #[field("C_BaseEntity", "m_sUniqueHammerID")]
    unique_hammer_id: UtlString,
    #[field("C_BaseModelEntity", "m_CRenderComponent")]
    c_render_component: *const RenderComponent,
    #[field("C_BaseModelEntity", "m_CHitboxComponent")]
    hitbox_component: HitboxComponent,
    #[field("C_BaseModelEntity", "m_bInitModelEffects")]
    init_model_effects: bool,
    #[field("C_BaseModelEntity", "m_bIsStaticProp")]
    is_static_prop: bool,
    #[field("C_BaseModelEntity", "m_nLastAddDecal")]
    last_add_decal: i32,
    #[field("C_BaseModelEntity", "m_nDecalsAdded")]
    decals_added: i32,
    #[field("C_BaseModelEntity", "m_iOldHealth")]
    old_health: i32,
    #[field("C_BaseModelEntity", "m_nRenderMode")]
    render_mode: RenderMode,
    #[field("C_BaseModelEntity", "m_nRenderFX")]
    render_fx: RenderFx,
    #[field("C_BaseModelEntity", "m_bAllowFadeInView")]
    allow_fade_in_view: bool,
    #[field("C_BaseModelEntity", "m_clrRender")]
    render: Color,
    #[field("C_BaseModelEntity", "m_vecRenderAttributes")]
    render_attributes: UtlVector<EntityRenderAttribute>,
    #[field("C_BaseModelEntity", "m_LightGroup")]
    light_group: UtlStringToken,
    #[field("C_BaseModelEntity", "m_bRenderToCubemaps")]
    render_to_cubemaps: bool,
    #[field("C_BaseModelEntity", "m_Collision")]
    collision: CollisionProperty,
    #[field("C_BaseModelEntity", "m_Glow")]
    glow: GlowProperty,
    #[field("C_BaseModelEntity", "m_flGlowBackfaceMult")]
    glow_backface_mult: f32,
    #[field("C_BaseModelEntity", "m_fadeMinDist")]
    fade_min_dist: f32,
    #[field("C_BaseModelEntity", "m_fadeMaxDist")]
    fade_max_dist: f32,
    #[field("C_BaseModelEntity", "m_flFadeScale")]
    fade_scale: f32,
    #[field("C_BaseModelEntity", "m_flShadowStrength")]
    shadow_strength: f32,
    #[field("C_BaseModelEntity", "m_nObjectCulling")]
    object_culling: u8,
    #[field("C_BaseModelEntity", "m_nAddDecal")]
    add_decal: i32,
    #[field("C_BaseModelEntity", "m_vDecalPosition")]
    decal_position: Vector,
    #[field("C_BaseModelEntity", "m_vDecalForwardAxis")]
    decal_forward_axis: Vector,
    #[field("C_BaseModelEntity", "m_flDecalHealBloodRate")]
    decal_heal_blood_rate: f32,
    #[field("C_BaseModelEntity", "m_flDecalHealHeightRate")]
    decal_heal_height_rate: f32,
    #[field("C_BaseModelEntity", "m_ConfigEntitiesToPropagateMaterialDecalsTo")]
    config_entities_to_propagate_material_decals_to: UtlVector<Handle<BaseModelEntity>>,
    #[field("C_BaseModelEntity", "m_vecViewOffset")]
    view_offset: NetworkViewOffsetVector,
    #[field("C_BaseModelEntity", "m_pClientAlphaProperty")]
    client_alpha_property: *const ClientAlphaProperty,
    #[field("C_BaseModelEntity", "m_ClientOverrideTint")]
    client_override_tint: Color,
    #[field("C_BaseModelEntity", "m_bUseClientOverrideTint")]
    use_client_override_tint: bool,
    #[field("CBaseAnimGraph", "m_bInitiallyPopulateInterpHistory")]
    initially_populate_interp_history: bool,
    #[field("CBaseAnimGraph", "m_bShouldAnimateDuringGameplayPause")]
    should_animate_during_gameplay_pause: bool,
    #[field("CBaseAnimGraph", "m_bSuppressAnimEventSounds")]
    suppress_anim_event_sounds: bool,
    #[field("CBaseAnimGraph", "m_bAnimGraphUpdateEnabled")]
    anim_graph_update_enabled: bool,
    #[field("CBaseAnimGraph", "m_flMaxSlopeDistance")]
    max_slope_distance: f32,
    #[field("CBaseAnimGraph", "m_vLastSlopeCheckPos")]
    last_slope_check_pos: Vector,
    #[field("CBaseAnimGraph", "m_vecForce")]
    force: Vector,
    #[field("CBaseAnimGraph", "m_nForceBone")]
    force_bone: i32,
    #[field("CBaseAnimGraph", "m_pClientsideRagdoll")]
    clientside_ragdoll: *const BaseAnimGraph,
    #[field("CBaseAnimGraph", "m_bBuiltRagdoll")]
    built_ragdoll: bool,
    #[field("CBaseAnimGraph", "m_pRagdollPose")]
    ragdoll_pose: *const PhysicsRagdollPose,
    #[field("CBaseAnimGraph", "m_bClientRagdoll")]
    client_ragdoll: bool,
    #[field("CBaseAnimGraph", "m_bHasAnimatedMaterialAttributes")]
    has_animated_material_attributes: bool,
    #[field("C_BaseFlex", "m_flexWeight")]
    flex_weight: UtlVector<f32>,
    #[field("C_BaseFlex", "m_vLookTargetPosition")]
    look_target_position: Vector,
    #[field("C_BaseFlex", "m_blinktoggle")]
    blinktoggle: bool,
    #[field("C_BaseFlex", "m_nLastFlexUpdateFrameCount")]
    last_flex_update_frame_count: i32,
    #[field("C_BaseFlex", "m_CachedViewTarget")]
    cached_view_target: Vector,
    #[field("C_BaseFlex", "m_nNextSceneEventId")]
    next_scene_event_id: u32,
    #[field("C_BaseFlex", "m_iBlink")]
    blink: i32,
    #[field("C_BaseFlex", "m_blinktime")]
    blinktime: f32,
    #[field("C_BaseFlex", "m_prevblinktoggle")]
    prevblinktoggle: bool,
    #[field("C_BaseFlex", "m_iJawOpen")]
    jaw_open: i32,
    #[field("C_BaseFlex", "m_flJawOpenAmount")]
    jaw_open_amount: f32,
    #[field("C_BaseFlex", "m_flBlinkAmount")]
    blink_amount: f32,
    #[field("C_BaseFlex", "m_iMouthAttachment")]
    mouth_attachment: AttachmentHandle,
    #[field("C_BaseFlex", "m_iEyeAttachment")]
    eye_attachment: AttachmentHandle,
    #[field("C_BaseFlex", "m_bResetFlexWeightsOnModelChange")]
    reset_flex_weights_on_model_change: bool,
    #[field("C_BaseFlex", "m_nEyeOcclusionRendererBone")]
    eye_occlusion_renderer_bone: i32,
    #[field("C_BaseFlex", "m_mEyeOcclusionRendererCameraToBoneTransform")]
    m_eye_occlusion_renderer_camera_to_bone_transform: Matrix3X4,
    #[field("C_BaseFlex", "m_vEyeOcclusionRendererHalfExtent")]
    eye_occlusion_renderer_half_extent: Vector,
    #[field("C_BaseFlex", "m_PhonemeClasses")]
    phoneme_classes: [BaseFlexEmphasizedPhoneme; 3],
    #[field("C_EconEntity", "m_flFlexDelayTime")]
    flex_delay_time: f32,
    #[field("C_EconEntity", "m_flFlexDelayedWeight")]
    flex_delayed_weight: *const f32,
    #[field("C_EconEntity", "m_bAttributesInitialized")]
    attributes_initialized: bool,
    #[field("C_EconEntity", "m_AttributeManager")]
    attribute_manager: AttributeContainer,
    #[field("C_EconEntity", "m_OriginalOwnerXuidLow")]
    original_owner_xuid_low: u32,
    #[field("C_EconEntity", "m_OriginalOwnerXuidHigh")]
    original_owner_xuid_high: u32,
    #[field("C_EconEntity", "m_nFallbackPaintKit")]
    fallback_paint_kit: i32,
    #[field("C_EconEntity", "m_nFallbackSeed")]
    fallback_seed: i32,
    #[field("C_EconEntity", "m_flFallbackWear")]
    fallback_wear: f32,
    #[field("C_EconEntity", "m_nFallbackStatTrak")]
    fallback_stat_trak: i32,
    #[field("C_EconEntity", "m_bClientside")]
    clientside: bool,
    #[field("C_EconEntity", "m_bParticleSystemsCreated")]
    particle_systems_created: bool,
    #[field("C_EconEntity", "m_vecAttachedParticles")]
    attached_particles: UtlVector<i32>,
    #[field("C_EconEntity", "m_hViewmodelAttachment")]
    viewmodel_attachment: Handle<BaseAnimGraph>,
    #[field("C_EconEntity", "m_iOldTeam")]
    old_team: i32,
    #[field("C_EconEntity", "m_bAttachmentDirty")]
    attachment_dirty: bool,
    #[field("C_EconEntity", "m_nUnloadedModelIndex")]
    unloaded_model_index: i32,
    #[field("C_EconEntity", "m_iNumOwnerValidationRetries")]
    num_owner_validation_retries: i32,
    #[field("C_EconEntity", "m_hOldProvidee")]
    old_providee: Handle<BaseEntity>,
    #[field("C_EconEntity", "m_vecAttachedModels")]
    attached_models: UtlVector<EconEntityAttachedModelData>,
    #[field("C_BasePlayerWeapon", "m_nNextPrimaryAttackTick")]
    next_primary_attack_tick: GameTick,
    #[field("C_BasePlayerWeapon", "m_flNextPrimaryAttackTickRatio")]
    next_primary_attack_tick_ratio: f32,
    #[field("C_BasePlayerWeapon", "m_nNextSecondaryAttackTick")]
    next_secondary_attack_tick: GameTick,
    #[field("C_BasePlayerWeapon", "m_flNextSecondaryAttackTickRatio")]
    next_secondary_attack_tick_ratio: f32,
    #[field("C_BasePlayerWeapon", "m_iClip1")]
    clip_1: i32,
    #[field("C_BasePlayerWeapon", "m_iClip2")]
    clip_2: i32,
    #[field("C_BasePlayerWeapon", "m_pReserveAmmo")]
    reserve_ammo: [i32; 2],
}

#[derive(Schema)]
#[scope("client.dll")]
pub struct PlayerWeaponServices {
    #[field("CPlayerPawnComponent", "__m_pChainEntity")]
    chain_entity: NetworkVarChainer,
    #[field("CPlayer_WeaponServices", "m_bAllowSwitchToNoWeapon")]
    allow_switch_to_no_weapon: bool,
    #[field("CPlayer_WeaponServices", "m_hMyWeapons")]
    my_weapons: UtlVector<Handle<BasePlayerWeapon>>,
    #[field("CPlayer_WeaponServices", "m_hActiveWeapon")]
    active_weapon: Handle<BasePlayerWeapon>,
    #[field("CPlayer_WeaponServices", "m_hLastWeapon")]
    last_weapon: Handle<BasePlayerWeapon>,
    #[field("CPlayer_WeaponServices", "m_iAmmo")]
    ammo: [u16; 32],
}

#[derive(Schema)]
#[scope("client.dll")]
pub struct PlayerItemServices {
    #[field("CPlayerPawnComponent", "__m_pChainEntity")]
    chain_entity: NetworkVarChainer,
}

#[derive(Schema)]
#[scope("client.dll")]
pub struct PlayerAutoaimServices {
    #[field("CPlayerPawnComponent", "__m_pChainEntity")]
    chain_entity: NetworkVarChainer,
}

#[derive(Schema)]
#[scope("client.dll")]
pub struct PlayerObserverServices {
    #[field("CPlayerPawnComponent", "__m_pChainEntity")]
    chain_entity: NetworkVarChainer,
    #[field("CPlayer_ObserverServices", "m_iObserverMode")]
    observer_mode: u8,
    #[field("CPlayer_ObserverServices", "m_hObserverTarget")]
    observer_target: Handle<BaseEntity>,
    #[field("CPlayer_ObserverServices", "m_iObserverLastMode")]
    observer_last_mode: ObserverMode,
    #[field("CPlayer_ObserverServices", "m_bForcedObserverMode")]
    forced_observer_mode: bool,
    #[field("CPlayer_ObserverServices", "m_flObserverChaseDistance")]
    observer_chase_distance: f32,
    #[field("CPlayer_ObserverServices", "m_flObserverChaseDistanceCalcTime")]
    observer_chase_distance_calc_time: GameTime,
}

#[derive(Schema)]
#[scope("client.dll")]
pub struct PlayerWaterServices {
    #[field("CPlayerPawnComponent", "__m_pChainEntity")]
    chain_entity: NetworkVarChainer,
}

#[derive(Schema)]
#[scope("client.dll")]
pub struct PlayerUseServices {
    #[field("CPlayerPawnComponent", "__m_pChainEntity")]
    chain_entity: NetworkVarChainer,
}

#[derive(Schema)]
#[scope("client.dll")]
pub struct PlayerFlashlightServices {
    #[field("CPlayerPawnComponent", "__m_pChainEntity")]
    chain_entity: NetworkVarChainer,
}

#[derive(Schema)]
#[scope("client.dll")]
pub struct FogController {}

#[derive(Schema)]
#[scope("client.dll")]
pub struct FogPlayerParams {
    #[field("C_fogplayerparams_t", "m_hCtrl")]
    ctrl: Handle<FogController>,
    #[field("C_fogplayerparams_t", "m_flTransitionTime")]
    transition_time: f32,
    #[field("C_fogplayerparams_t", "m_OldColor")]
    old_color: Color,
    #[field("C_fogplayerparams_t", "m_flOldStart")]
    old_start: f32,
    #[field("C_fogplayerparams_t", "m_flOldEnd")]
    old_end: f32,
    #[field("C_fogplayerparams_t", "m_flOldMaxDensity")]
    old_max_density: f32,
    #[field("C_fogplayerparams_t", "m_flOldHDRColorScale")]
    old_hdr_color_scale: f32,
    #[field("C_fogplayerparams_t", "m_flOldFarZ")]
    old_far_z: f32,
    #[field("C_fogplayerparams_t", "m_NewColor")]
    new_color: Color,
    #[field("C_fogplayerparams_t", "m_flNewStart")]
    new_start: f32,
    #[field("C_fogplayerparams_t", "m_flNewEnd")]
    new_end: f32,
    #[field("C_fogplayerparams_t", "m_flNewMaxDensity")]
    new_max_density: f32,
    #[field("C_fogplayerparams_t", "m_flNewHDRColorScale")]
    new_hdr_color_scale: f32,
    #[field("C_fogplayerparams_t", "m_flNewFarZ")]
    new_far_z: f32,
}

#[derive(Schema)]
#[scope("client.dll")]
pub struct ColorCorrection {}

#[derive(Schema)]
#[scope("client.dll")]
pub struct TonemapController2 {}

#[derive(Schema)]
#[scope("client.dll")]
pub struct AudioParams {
    #[field("audioparams_t", "localSound")]
    local_sound: [Vector; 8],
    #[field("audioparams_t", "soundscapeIndex")]
    soundscape_index: i32,
    #[field("audioparams_t", "localBits")]
    local_bits: u8,
    #[field("audioparams_t", "soundscapeEntityListIndex")]
    soundscape_entity_list_index: i32,
    #[field("audioparams_t", "soundEventHash")]
    sound_event_hash: u32,
}

#[derive(Schema)]
#[scope("client.dll")]
pub struct PostProcessingVolume {}

#[derive(Schema)]
#[scope("client.dll")]
pub struct FogParams {
    #[field("fogparams_t", "dirPrimary")]
    dir_primary: Vector,
    #[field("fogparams_t", "colorPrimary")]
    color_primary: Color,
    #[field("fogparams_t", "colorSecondary")]
    color_secondary: Color,
    #[field("fogparams_t", "colorPrimaryLerpTo")]
    color_primary_lerp_to: Color,
    #[field("fogparams_t", "colorSecondaryLerpTo")]
    color_secondary_lerp_to: Color,
    #[field("fogparams_t", "start")]
    start: f32,
    #[field("fogparams_t", "end")]
    end: f32,
    #[field("fogparams_t", "farz")]
    farz: f32,
    #[field("fogparams_t", "maxdensity")]
    maxdensity: f32,
    #[field("fogparams_t", "exponent")]
    exponent: f32,
    #[field("fogparams_t", "HDRColorScale")]
    hdr_color_scale: f32,
    #[field("fogparams_t", "skyboxFogFactor")]
    skybox_fog_factor: f32,
    #[field("fogparams_t", "skyboxFogFactorLerpTo")]
    skybox_fog_factor_lerp_to: f32,
    #[field("fogparams_t", "startLerpTo")]
    start_lerp_to: f32,
    #[field("fogparams_t", "endLerpTo")]
    end_lerp_to: f32,
    #[field("fogparams_t", "maxdensityLerpTo")]
    maxdensity_lerp_to: f32,
    #[field("fogparams_t", "lerptime")]
    lerptime: GameTime,
    #[field("fogparams_t", "duration")]
    duration: f32,
    #[field("fogparams_t", "blendtobackground")]
    blendtobackground: f32,
    #[field("fogparams_t", "scattering")]
    scattering: f32,
    #[field("fogparams_t", "locallightscale")]
    locallightscale: f32,
    #[field("fogparams_t", "enable")]
    enable: bool,
    #[field("fogparams_t", "blend")]
    blend: bool,
    #[field("fogparams_t", "m_bNoReflectionFog")]
    no_reflection_fog: bool,
    #[field("fogparams_t", "m_bPadding")]
    padding: bool,
}

#[derive(Schema)]
#[scope("client.dll")]
pub struct PlayerCameraServices {
    #[field("CPlayerPawnComponent", "__m_pChainEntity")]
    chain_entity: NetworkVarChainer,
    #[field("CPlayer_CameraServices", "m_vecCsViewPunchAngle")]
    view_punch_angle: QAngle,
    #[field("CPlayer_CameraServices", "m_nCsViewPunchAngleTick")]
    view_punch_angle_tick: GameTick,
    #[field("CPlayer_CameraServices", "m_flCsViewPunchAngleTickRatio")]
    view_punch_angle_tick_ratio: f32,
    #[field("CPlayer_CameraServices", "m_PlayerFog")]
    player_fog: FogPlayerParams,
    #[field("CPlayer_CameraServices", "m_hColorCorrectionCtrl")]
    color_correction_ctrl: Handle<ColorCorrection>,
    #[field("CPlayer_CameraServices", "m_hViewEntity")]
    view_entity: Handle<BaseEntity>,
    #[field("CPlayer_CameraServices", "m_hTonemapController")]
    tonemap_controller: Handle<TonemapController2>,
    #[field("CPlayer_CameraServices", "m_audio")]
    audio: AudioParams,
    #[field("CPlayer_CameraServices", "m_PostProcessingVolumes")]
    post_processing_volumes: UtlVector<Handle<PostProcessingVolume>>,
    #[field("CPlayer_CameraServices", "m_flOldPlayerZ")]
    old_player_z: f32,
    #[field("CPlayer_CameraServices", "m_flOldPlayerViewOffsetZ")]
    old_player_view_offset_z: f32,
    #[field("CPlayer_CameraServices", "m_CurrentFog")]
    current_fog: FogParams,
    #[field("CPlayer_CameraServices", "m_hOldFogController")]
    old_fog_controller: Handle<FogController>,
    #[field("CPlayer_CameraServices", "m_bOverrideFogColor")]
    b_override_fog_color: [bool; 5],
    #[field("CPlayer_CameraServices", "m_OverrideFogColor")]
    override_fog_color: [Color; 5],
    #[field("CPlayer_CameraServices", "m_bOverrideFogStartEnd")]
    override_fog_start_end: [bool; 5],
    #[field("CPlayer_CameraServices", "m_fOverrideFogStart")]
    override_fog_start: [f32; 5],
    #[field("CPlayer_CameraServices", "m_fOverrideFogEnd")]
    override_fog_end: [f32; 5],
    #[field("CPlayer_CameraServices", "m_hActivePostProcessingVolume")]
    active_post_processing_volume: Handle<PostProcessingVolume>,
    #[field("CPlayer_CameraServices", "m_angDemoViewAngles")]
    demo_view_angles: QAngle,
}

#[derive(Schema)]
#[scope("client.dll")]
pub struct PlayerMovementServices {
    #[field("CPlayerPawnComponent", "__m_pChainEntity")]
    chain_entity: NetworkVarChainer,
    #[field("CPlayer_MovementServices", "m_nImpulse")]
    impulse: i32,
    #[field("CPlayer_MovementServices", "m_nButtons")]
    buttons: InButtonState,
    #[field("CPlayer_MovementServices", "m_nQueuedButtonDownMask")]
    queued_button_down_mask: u64,
    #[field("CPlayer_MovementServices", "m_nQueuedButtonChangeMask")]
    queued_button_change_mask: u64,
    #[field("CPlayer_MovementServices", "m_nButtonDoublePressed")]
    button_double_pressed: u64,
    #[field("CPlayer_MovementServices", "m_pButtonPressedCmdNumber")]
    button_pressed_cmd_number: [u32; 64],
    #[field("CPlayer_MovementServices", "m_nLastCommandNumberProcessed")]
    last_command_number_processed: u32,
    #[field("CPlayer_MovementServices", "m_nToggleButtonDownMask")]
    toggle_button_down_mask: u64,
    #[field("CPlayer_MovementServices", "m_flMaxspeed")]
    maxspeed: f32,
    #[field("CPlayer_MovementServices", "m_arrForceSubtickMoveWhen")]
    force_subtick_move_when: [f32; 4],
    #[field("CPlayer_MovementServices", "m_flForwardMove")]
    forward_move: f32,
    #[field("CPlayer_MovementServices", "m_flLeftMove")]
    left_move: f32,
    #[field("CPlayer_MovementServices", "m_flUpMove")]
    up_move: f32,
    #[field("CPlayer_MovementServices", "m_vecLastMovementImpulses")]
    last_movement_impulses: Vector,
    #[field("CPlayer_MovementServices", "m_vecOldViewAngles")]
    old_view_angles: QAngle,
}

#[derive(Schema)]
#[scope("client.dll")]
pub struct ViewAngleServerChange {
    #[field("ViewAngleServerChange_t", "nType")]
    ty: FixAngleSet,
    #[field("ViewAngleServerChange_t", "qAngle")]
    angle: QAngle,
    #[field("ViewAngleServerChange_t", "nIndex")]
    index: u32,
}

#[derive(Schema)]
#[scope("client.dll")]
pub struct Sky3Dparams {
    #[field("sky3dparams_t", "scale")]
    scale: i16,
    #[field("sky3dparams_t", "origin")]
    origin: Vector,
    #[field("sky3dparams_t", "bClip3DSkyBoxNearToWorldFar")]
    b_clip_3_d_sky_box_near_to_world_far: bool,
    #[field("sky3dparams_t", "flClip3DSkyBoxNearToWorldFarOffset")]
    fl_clip_3_d_sky_box_near_to_world_far_offset: f32,
    #[field("sky3dparams_t", "fog")]
    fog: FogParams,
    #[field("sky3dparams_t", "m_nWorldGroupID")]
    world_group_id: WorldGroupId,
}

#[derive(Schema)]
#[scope("client.dll")]
pub struct BasePlayerPawn {
    #[field("CEntityInstance", "m_iszPrivateVScripts")]
    private_v_scripts: UtlSymbolLarge,
    #[field("CEntityInstance", "m_pEntity")]
    entity: *const EntityIdentity,
    #[field("CEntityInstance", "m_CScriptComponent")]
    script_component: *const ScriptComponent,
    #[field("C_BaseEntity", "m_CBodyComponent")]
    body_component: *const BodyComponent,
    #[field("C_BaseEntity", "m_NetworkTransmitComponent")]
    network_transmit_component: NetworkTransmitComponent,
    #[field("C_BaseEntity", "m_nLastThinkTick")]
    last_think_tick: GameTick,
    #[field("C_BaseEntity", "m_pGameSceneNode")]
    game_scene_node: *const GameSceneNode,
    #[field("C_BaseEntity", "m_pRenderComponent")]
    p_render_component: *const RenderComponent,
    #[field("C_BaseEntity", "m_pCollision")]
    p_collision: *const CollisionProperty,
    #[field("C_BaseEntity", "m_iMaxHealth")]
    max_health: i32,
    #[field("C_BaseEntity", "m_iHealth")]
    health: i32,
    #[field("C_BaseEntity", "m_lifeState")]
    life_state: LifeState,
    #[field("C_BaseEntity", "m_bTakesDamage")]
    takes_damage: bool,
    #[field("C_BaseEntity", "m_nTakeDamageFlags")]
    take_damage_flags: TakeDamageFlags,
    #[field("C_BaseEntity", "m_ubInterpolationFrame")]
    interpolation_frame: u8,
    #[field("C_BaseEntity", "m_hSceneObjectController")]
    scene_object_controller: Handle<BaseEntity>,
    #[field("C_BaseEntity", "m_nNoInterpolationTick")]
    no_interpolation_tick: i32,
    #[field("C_BaseEntity", "m_nVisibilityNoInterpolationTick")]
    visibility_no_interpolation_tick: i32,
    #[field("C_BaseEntity", "m_flProxyRandomValue")]
    proxy_random_value: f32,
    #[field("C_BaseEntity", "m_iEFlags")]
    e_flags: i32,
    #[field("C_BaseEntity", "m_nWaterType")]
    water_type: u8,
    #[field("C_BaseEntity", "m_bInterpolateEvenWithNoModel")]
    interpolate_even_with_no_model: bool,
    #[field("C_BaseEntity", "m_bPredictionEligible")]
    prediction_eligible: bool,
    #[field("C_BaseEntity", "m_bApplyLayerMatchIDToModel")]
    apply_layer_match_id_to_model: bool,
    #[field("C_BaseEntity", "m_tokLayerMatchID")]
    tok_layer_match_id: UtlStringToken,
    #[field("C_BaseEntity", "m_nSubclassID")]
    subclass_id: UtlStringToken,
    #[field("C_BaseEntity", "m_nSimulationTick")]
    simulation_tick: i32,
    #[field("C_BaseEntity", "m_iCurrentThinkContext")]
    current_think_context: i32,
    #[field("C_BaseEntity", "m_aThinkFunctions")]
    think_functions: UtlVector<Thinkfunc>,
    #[field("C_BaseEntity", "m_flAnimTime")]
    anim_time: f32,
    #[field("C_BaseEntity", "m_flSimulationTime")]
    simulation_time: f32,
    #[field("C_BaseEntity", "m_nSceneObjectOverrideFlags")]
    scene_object_override_flags: u8,
    #[field("C_BaseEntity", "m_bHasSuccessfullyInterpolated")]
    has_successfully_interpolated: bool,
    #[field("C_BaseEntity", "m_bHasAddedVarsToInterpolation")]
    has_added_vars_to_interpolation: bool,
    #[field("C_BaseEntity", "m_bRenderEvenWhenNotSuccessfullyInterpolated")]
    render_even_when_not_successfully_interpolated: bool,
    #[field("C_BaseEntity", "m_nInterpolationLatchDirtyFlags")]
    interpolation_latch_dirty_flags: [i32; 2],
    #[field("C_BaseEntity", "m_ListEntry")]
    list_entry: [u16; 11],
    #[field("C_BaseEntity", "m_flCreateTime")]
    create_time: GameTime,
    #[field("C_BaseEntity", "m_flSpeed")]
    speed: f32,
    #[field("C_BaseEntity", "m_EntClientFlags")]
    ent_client_flags: u16,
    #[field("C_BaseEntity", "m_bClientSideRagdoll")]
    client_side_ragdoll: bool,
    #[field("C_BaseEntity", "m_iTeamNum")]
    team_num: u8,
    #[field("C_BaseEntity", "m_spawnflags")]
    spawnflags: u32,
    #[field("C_BaseEntity", "m_nNextThinkTick")]
    next_think_tick: GameTick,
    #[field("C_BaseEntity", "m_fFlags")]
    flags: u32,
    #[field("C_BaseEntity", "m_vecAbsVelocity")]
    abs_velocity: Vector,
    #[field("C_BaseEntity", "m_vecVelocity")]
    velocity: NetworkVelocityVector,
    #[field("C_BaseEntity", "m_vecBaseVelocity")]
    base_velocity: Vector,
    #[field("C_BaseEntity", "m_hEffectEntity")]
    effect_entity: Handle<BaseEntity>,
    #[field("C_BaseEntity", "m_hOwnerEntity")]
    owner_entity: Handle<BaseEntity>,
    #[field("C_BaseEntity", "m_MoveCollide")]
    move_collide: MoveCollide,
    #[field("C_BaseEntity", "m_MoveType")]
    move_type: MoveType,
    #[field("C_BaseEntity", "m_flWaterLevel")]
    water_level: f32,
    #[field("C_BaseEntity", "m_fEffects")]
    effects: u32,
    #[field("C_BaseEntity", "m_hGroundEntity")]
    ground_entity: Handle<BaseEntity>,
    #[field("C_BaseEntity", "m_flFriction")]
    friction: f32,
    #[field("C_BaseEntity", "m_flElasticity")]
    elasticity: f32,
    #[field("C_BaseEntity", "m_flGravityScale")]
    gravity_scale: f32,
    #[field("C_BaseEntity", "m_flTimeScale")]
    time_scale: f32,
    #[field("C_BaseEntity", "m_bSimulatedEveryTick")]
    simulated_every_tick: bool,
    #[field("C_BaseEntity", "m_bAnimatedEveryTick")]
    animated_every_tick: bool,
    #[field("C_BaseEntity", "m_flNavIgnoreUntilTime")]
    nav_ignore_until_time: GameTime,
    #[field("C_BaseEntity", "m_hThink")]
    think: u16,
    #[field("C_BaseEntity", "m_fBBoxVisFlags")]
    b_box_vis_flags: u8,
    #[field("C_BaseEntity", "m_bPredictable")]
    predictable: bool,
    #[field("C_BaseEntity", "m_bRenderWithViewModels")]
    render_with_view_models: bool,
    #[field("C_BaseEntity", "m_nSplitUserPlayerPredictionSlot")]
    split_user_player_prediction_slot: SplitScreenSlot,
    #[field("C_BaseEntity", "m_nFirstPredictableCommand")]
    first_predictable_command: i32,
    #[field("C_BaseEntity", "m_nLastPredictableCommand")]
    last_predictable_command: i32,
    #[field("C_BaseEntity", "m_hOldMoveParent")]
    old_move_parent: Handle<BaseEntity>,
    #[field("C_BaseEntity", "m_Particles")]
    particles: ParticleProperty,
    #[field("C_BaseEntity", "m_vecPredictedScriptFloats")]
    predicted_script_floats: UtlVector<f32>,
    #[field("C_BaseEntity", "m_vecPredictedScriptFloatIDs")]
    predicted_script_float_i_ds: UtlVector<i32>,
    #[field("C_BaseEntity", "m_nNextScriptVarRecordID")]
    next_script_var_record_id: i32,
    #[field("C_BaseEntity", "m_vecAngVelocity")]
    ang_velocity: QAngle,
    #[field("C_BaseEntity", "m_DataChangeEventRef")]
    data_change_event_ref: i32,
    #[field("C_BaseEntity", "m_dependencies")]
    dependencies: UtlVector<EntityHandle>,
    #[field("C_BaseEntity", "m_nCreationTick")]
    creation_tick: i32,
    #[field("C_BaseEntity", "m_bAnimTimeChanged")]
    anim_time_changed: bool,
    #[field("C_BaseEntity", "m_bSimulationTimeChanged")]
    simulation_time_changed: bool,
    #[field("C_BaseEntity", "m_sUniqueHammerID")]
    unique_hammer_id: UtlString,
    #[field("C_BaseModelEntity", "m_CRenderComponent")]
    c_render_component: *const RenderComponent,
    #[field("C_BaseModelEntity", "m_CHitboxComponent")]
    hitbox_component: HitboxComponent,
    #[field("C_BaseModelEntity", "m_bInitModelEffects")]
    init_model_effects: bool,
    #[field("C_BaseModelEntity", "m_bIsStaticProp")]
    is_static_prop: bool,
    #[field("C_BaseModelEntity", "m_nLastAddDecal")]
    last_add_decal: i32,
    #[field("C_BaseModelEntity", "m_nDecalsAdded")]
    decals_added: i32,
    #[field("C_BaseModelEntity", "m_iOldHealth")]
    old_health: i32,
    #[field("C_BaseModelEntity", "m_nRenderMode")]
    render_mode: RenderMode,
    #[field("C_BaseModelEntity", "m_nRenderFX")]
    render_fx: RenderFx,
    #[field("C_BaseModelEntity", "m_bAllowFadeInView")]
    allow_fade_in_view: bool,
    #[field("C_BaseModelEntity", "m_clrRender")]
    render: Color,
    #[field("C_BaseModelEntity", "m_vecRenderAttributes")]
    render_attributes: UtlVector<EntityRenderAttribute>,
    #[field("C_BaseModelEntity", "m_LightGroup")]
    light_group: UtlStringToken,
    #[field("C_BaseModelEntity", "m_bRenderToCubemaps")]
    render_to_cubemaps: bool,
    #[field("C_BaseModelEntity", "m_Collision")]
    collision: CollisionProperty,
    #[field("C_BaseModelEntity", "m_Glow")]
    glow: GlowProperty,
    #[field("C_BaseModelEntity", "m_flGlowBackfaceMult")]
    glow_backface_mult: f32,
    #[field("C_BaseModelEntity", "m_fadeMinDist")]
    fade_min_dist: f32,
    #[field("C_BaseModelEntity", "m_fadeMaxDist")]
    fade_max_dist: f32,
    #[field("C_BaseModelEntity", "m_flFadeScale")]
    fade_scale: f32,
    #[field("C_BaseModelEntity", "m_flShadowStrength")]
    shadow_strength: f32,
    #[field("C_BaseModelEntity", "m_nObjectCulling")]
    object_culling: u8,
    #[field("C_BaseModelEntity", "m_nAddDecal")]
    add_decal: i32,
    #[field("C_BaseModelEntity", "m_vDecalPosition")]
    decal_position: Vector,
    #[field("C_BaseModelEntity", "m_vDecalForwardAxis")]
    decal_forward_axis: Vector,
    #[field("C_BaseModelEntity", "m_flDecalHealBloodRate")]
    decal_heal_blood_rate: f32,
    #[field("C_BaseModelEntity", "m_flDecalHealHeightRate")]
    decal_heal_height_rate: f32,
    #[field("C_BaseModelEntity", "m_ConfigEntitiesToPropagateMaterialDecalsTo")]
    config_entities_to_propagate_material_decals_to: UtlVector<Handle<BaseModelEntity>>,
    #[field("C_BaseModelEntity", "m_vecViewOffset")]
    view_offset: NetworkViewOffsetVector,
    #[field("C_BaseModelEntity", "m_pClientAlphaProperty")]
    client_alpha_property: *const ClientAlphaProperty,
    #[field("C_BaseModelEntity", "m_ClientOverrideTint")]
    client_override_tint: Color,
    #[field("C_BaseModelEntity", "m_bUseClientOverrideTint")]
    use_client_override_tint: bool,
    #[field("CBaseAnimGraph", "m_bInitiallyPopulateInterpHistory")]
    initially_populate_interp_history: bool,
    #[field("CBaseAnimGraph", "m_bShouldAnimateDuringGameplayPause")]
    should_animate_during_gameplay_pause: bool,
    #[field("CBaseAnimGraph", "m_bSuppressAnimEventSounds")]
    suppress_anim_event_sounds: bool,
    #[field("CBaseAnimGraph", "m_bAnimGraphUpdateEnabled")]
    anim_graph_update_enabled: bool,
    #[field("CBaseAnimGraph", "m_flMaxSlopeDistance")]
    max_slope_distance: f32,
    #[field("CBaseAnimGraph", "m_vLastSlopeCheckPos")]
    last_slope_check_pos: Vector,
    #[field("CBaseAnimGraph", "m_vecForce")]
    force: Vector,
    #[field("CBaseAnimGraph", "m_nForceBone")]
    force_bone: i32,
    #[field("CBaseAnimGraph", "m_pClientsideRagdoll")]
    clientside_ragdoll: *const BaseAnimGraph,
    #[field("CBaseAnimGraph", "m_bBuiltRagdoll")]
    built_ragdoll: bool,
    #[field("CBaseAnimGraph", "m_pRagdollPose")]
    ragdoll_pose: *const PhysicsRagdollPose,
    #[field("CBaseAnimGraph", "m_bClientRagdoll")]
    client_ragdoll: bool,
    #[field("CBaseAnimGraph", "m_bHasAnimatedMaterialAttributes")]
    has_animated_material_attributes: bool,
    #[field("C_BaseFlex", "m_flexWeight")]
    flex_weight: UtlVector<f32>,
    #[field("C_BaseFlex", "m_vLookTargetPosition")]
    look_target_position: Vector,
    #[field("C_BaseFlex", "m_blinktoggle")]
    blinktoggle: bool,
    #[field("C_BaseFlex", "m_nLastFlexUpdateFrameCount")]
    last_flex_update_frame_count: i32,
    #[field("C_BaseFlex", "m_CachedViewTarget")]
    cached_view_target: Vector,
    #[field("C_BaseFlex", "m_nNextSceneEventId")]
    next_scene_event_id: u32,
    #[field("C_BaseFlex", "m_iBlink")]
    blink: i32,
    #[field("C_BaseFlex", "m_blinktime")]
    blinktime: f32,
    #[field("C_BaseFlex", "m_prevblinktoggle")]
    prevblinktoggle: bool,
    #[field("C_BaseFlex", "m_iJawOpen")]
    jaw_open: i32,
    #[field("C_BaseFlex", "m_flJawOpenAmount")]
    jaw_open_amount: f32,
    #[field("C_BaseFlex", "m_flBlinkAmount")]
    blink_amount: f32,
    #[field("C_BaseFlex", "m_iMouthAttachment")]
    mouth_attachment: AttachmentHandle,
    #[field("C_BaseFlex", "m_iEyeAttachment")]
    eye_attachment: AttachmentHandle,
    #[field("C_BaseFlex", "m_bResetFlexWeightsOnModelChange")]
    reset_flex_weights_on_model_change: bool,
    #[field("C_BaseFlex", "m_nEyeOcclusionRendererBone")]
    eye_occlusion_renderer_bone: i32,
    #[field("C_BaseFlex", "m_mEyeOcclusionRendererCameraToBoneTransform")]
    m_eye_occlusion_renderer_camera_to_bone_transform: Matrix3X4,
    #[field("C_BaseFlex", "m_vEyeOcclusionRendererHalfExtent")]
    eye_occlusion_renderer_half_extent: Vector,
    #[field("C_BaseFlex", "m_PhonemeClasses")]
    phoneme_classes: [BaseFlexEmphasizedPhoneme; 3],
    #[field("C_BaseCombatCharacter", "m_hMyWearables")]
    my_wearables: UtlVector<Handle<EconWearable>>,
    #[field("C_BaseCombatCharacter", "m_bloodColor")]
    blood_color: i32,
    #[field("C_BaseCombatCharacter", "m_leftFootAttachment")]
    left_foot_attachment: AttachmentHandle,
    #[field("C_BaseCombatCharacter", "m_rightFootAttachment")]
    right_foot_attachment: AttachmentHandle,
    #[field("C_BaseCombatCharacter", "m_nWaterWakeMode")]
    water_wake_mode: WaterWakeMode,
    #[field("C_BaseCombatCharacter", "m_flWaterWorldZ")]
    water_world_z: f32,
    #[field("C_BaseCombatCharacter", "m_flWaterNextTraceTime")]
    water_next_trace_time: f32,
    #[field("C_BaseCombatCharacter", "m_flFieldOfView")]
    field_of_view: f32,
    #[field("C_BasePlayerPawn", "m_pWeaponServices")]
    weapon_services: *const PlayerWeaponServices,
    #[field("C_BasePlayerPawn", "m_pItemServices")]
    item_services: *const PlayerItemServices,
    #[field("C_BasePlayerPawn", "m_pAutoaimServices")]
    autoaim_services: *const PlayerAutoaimServices,
    #[field("C_BasePlayerPawn", "m_pObserverServices")]
    observer_services: *const PlayerObserverServices,
    #[field("C_BasePlayerPawn", "m_pWaterServices")]
    water_services: *const PlayerWaterServices,
    #[field("C_BasePlayerPawn", "m_pUseServices")]
    use_services: *const PlayerUseServices,
    #[field("C_BasePlayerPawn", "m_pFlashlightServices")]
    flashlight_services: *const PlayerFlashlightServices,
    #[field("C_BasePlayerPawn", "m_pCameraServices")]
    camera_services: *const PlayerCameraServices,
    #[field("C_BasePlayerPawn", "m_pMovementServices")]
    movement_services: *const PlayerMovementServices,
    #[field("C_BasePlayerPawn", "m_ServerViewAngleChanges")]
    server_view_angle_changes: UtlVector<ViewAngleServerChange>,
    #[field("C_BasePlayerPawn", "m_nHighestConsumedServerViewAngleChangeIndex")]
    highest_consumed_server_view_angle_change_index: u32,
    #[field("C_BasePlayerPawn", "v_angle")]
    v_angle: QAngle,
    #[field("C_BasePlayerPawn", "v_anglePrevious")]
    v_angle_previous: QAngle,
    #[field("C_BasePlayerPawn", "m_iHideHUD")]
    hide_hud: u32,
    #[field("C_BasePlayerPawn", "m_skybox3d")]
    skybox_3_d: Sky3Dparams,
    #[field("C_BasePlayerPawn", "m_flDeathTime")]
    death_time: GameTime,
    #[field("C_BasePlayerPawn", "m_vecPredictionError")]
    prediction_error: Vector,
    #[field("C_BasePlayerPawn", "m_flPredictionErrorTime")]
    prediction_error_time: GameTime,
    #[field("C_BasePlayerPawn", "m_flFOVSensitivityAdjust")]
    fov_sensitivity_adjust: f32,
    #[field("C_BasePlayerPawn", "m_flMouseSensitivity")]
    mouse_sensitivity: f32,
    #[field("C_BasePlayerPawn", "m_vOldOrigin")]
    old_origin: Vec3,
    #[field("C_BasePlayerPawn", "m_flOldSimulationTime")]
    old_simulation_time: f32,
    #[field("C_BasePlayerPawn", "m_nLastExecutedCommandNumber")]
    last_executed_command_number: i32,
    #[field("C_BasePlayerPawn", "m_nLastExecutedCommandTick")]
    last_executed_command_tick: i32,
    #[field("C_BasePlayerPawn", "m_hController")]
    controller: Handle<BasePlayerController>,
    #[field("C_BasePlayerPawn", "m_bIsSwappingToPredictableController")]
    is_swapping_to_predictable_controller: bool,
}

#[derive(Schema)]
#[scope("client.dll")]
pub struct BasePlayerController {
    #[field("CBasePlayerController", "m_nFinalPredictedTick")]
    final_predicted_tick: i32,
    #[field("CBasePlayerController", "m_CommandContext")]
    command_context: CommandContext,
    #[field("CBasePlayerController", "m_nInButtonsWhichAreToggles")]
    in_buttons_which_are_toggles: u64,
    #[field("CBasePlayerController", "m_nTickBase")]
    tick_base: u32,
    #[field("CBasePlayerController", "m_hPawn")]
    pawn: Handle<BasePlayerPawn>,
    #[field("CBasePlayerController", "m_hPredictedPawn")]
    predicted_pawn: Handle<BasePlayerPawn>,
    #[field("CBasePlayerController", "m_nSplitScreenSlot")]
    split_screen_slot: SplitScreenSlot,
    #[field("CBasePlayerController", "m_hSplitOwner")]
    split_owner: Handle<BasePlayerController>,
    #[field("CBasePlayerController", "m_hSplitScreenPlayers")]
    split_screen_players: UtlVector<Handle<BasePlayerController>>,
    #[field("CBasePlayerController", "m_bIsHLTV")]
    is_hltv: bool,
    #[field("CBasePlayerController", "m_iConnected")]
    connected: PlayerConnectedState,
    #[field("CBasePlayerController", "m_iszPlayerName")]
    player_name: [std::ffi::c_char; 128],
    #[field("CBasePlayerController", "m_steamID")]
    steam_id: u64,
    #[field("CBasePlayerController", "m_bIsLocalPlayerController")]
    is_local_player_controller: bool,
    #[field("CBasePlayerController", "m_iDesiredFOV")]
    desired_fov: u32,
}

#[derive(Schema)]
#[scope("client.dll")]
pub struct PlayerPingServices {
    #[field("CCSPlayer_PingServices", "m_hPlayerPing")]
    player_ping: Handle<BaseEntity>,
}

#[derive(Schema)]
#[scope("client.dll")]
pub struct PlayerViewModelServices {}

#[derive(Schema)]
#[scope("client.dll")]
pub struct WeaponBase {
    #[field("CEntityInstance", "m_iszPrivateVScripts")]
    private_v_scripts: UtlSymbolLarge,
    #[field("CEntityInstance", "m_pEntity")]
    entity: *const EntityIdentity,
    #[field("CEntityInstance", "m_CScriptComponent")]
    script_component: *const ScriptComponent,
    #[field("C_BaseEntity", "m_CBodyComponent")]
    body_component: *const BodyComponent,
    #[field("C_BaseEntity", "m_NetworkTransmitComponent")]
    network_transmit_component: NetworkTransmitComponent,
    #[field("C_BaseEntity", "m_nLastThinkTick")]
    last_think_tick: GameTick,
    #[field("C_BaseEntity", "m_pGameSceneNode")]
    game_scene_node: *const GameSceneNode,
    #[field("C_BaseEntity", "m_pRenderComponent")]
    p_render_component: *const RenderComponent,
    #[field("C_BaseEntity", "m_pCollision")]
    p_collision: *const CollisionProperty,
    #[field("C_BaseEntity", "m_iMaxHealth")]
    max_health: i32,
    #[field("C_BaseEntity", "m_iHealth")]
    health: i32,
    #[field("C_BaseEntity", "m_lifeState")]
    life_state: u8,
    #[field("C_BaseEntity", "m_bTakesDamage")]
    takes_damage: bool,
    #[field("C_BaseEntity", "m_nTakeDamageFlags")]
    take_damage_flags: TakeDamageFlags,
    #[field("C_BaseEntity", "m_ubInterpolationFrame")]
    interpolation_frame: u8,
    #[field("C_BaseEntity", "m_hSceneObjectController")]
    scene_object_controller: Handle<BaseEntity>,
    #[field("C_BaseEntity", "m_nNoInterpolationTick")]
    no_interpolation_tick: i32,
    #[field("C_BaseEntity", "m_nVisibilityNoInterpolationTick")]
    visibility_no_interpolation_tick: i32,
    #[field("C_BaseEntity", "m_flProxyRandomValue")]
    proxy_random_value: f32,
    #[field("C_BaseEntity", "m_iEFlags")]
    e_flags: i32,
    #[field("C_BaseEntity", "m_nWaterType")]
    water_type: u8,
    #[field("C_BaseEntity", "m_bInterpolateEvenWithNoModel")]
    interpolate_even_with_no_model: bool,
    #[field("C_BaseEntity", "m_bPredictionEligible")]
    prediction_eligible: bool,
    #[field("C_BaseEntity", "m_bApplyLayerMatchIDToModel")]
    apply_layer_match_id_to_model: bool,
    #[field("C_BaseEntity", "m_tokLayerMatchID")]
    tok_layer_match_id: UtlStringToken,
    #[field("C_BaseEntity", "m_nSubclassID")]
    subclass_id: UtlStringToken,
    #[field("C_BaseEntity", "m_nSimulationTick")]
    simulation_tick: i32,
    #[field("C_BaseEntity", "m_iCurrentThinkContext")]
    current_think_context: i32,
    #[field("C_BaseEntity", "m_aThinkFunctions")]
    think_functions: UtlVector<Thinkfunc>,
    #[field("C_BaseEntity", "m_flAnimTime")]
    anim_time: f32,
    #[field("C_BaseEntity", "m_flSimulationTime")]
    simulation_time: f32,
    #[field("C_BaseEntity", "m_nSceneObjectOverrideFlags")]
    scene_object_override_flags: u8,
    #[field("C_BaseEntity", "m_bHasSuccessfullyInterpolated")]
    has_successfully_interpolated: bool,
    #[field("C_BaseEntity", "m_bHasAddedVarsToInterpolation")]
    has_added_vars_to_interpolation: bool,
    #[field("C_BaseEntity", "m_bRenderEvenWhenNotSuccessfullyInterpolated")]
    render_even_when_not_successfully_interpolated: bool,
    #[field("C_BaseEntity", "m_nInterpolationLatchDirtyFlags")]
    interpolation_latch_dirty_flags: [i32; 2],
    #[field("C_BaseEntity", "m_ListEntry")]
    list_entry: [u16; 11],
    #[field("C_BaseEntity", "m_flCreateTime")]
    create_time: GameTime,
    #[field("C_BaseEntity", "m_flSpeed")]
    speed: f32,
    #[field("C_BaseEntity", "m_EntClientFlags")]
    ent_client_flags: u16,
    #[field("C_BaseEntity", "m_bClientSideRagdoll")]
    client_side_ragdoll: bool,
    #[field("C_BaseEntity", "m_iTeamNum")]
    team_num: u8,
    #[field("C_BaseEntity", "m_spawnflags")]
    spawnflags: u32,
    #[field("C_BaseEntity", "m_nNextThinkTick")]
    next_think_tick: GameTick,
    #[field("C_BaseEntity", "m_fFlags")]
    flags: u32,
    #[field("C_BaseEntity", "m_vecAbsVelocity")]
    abs_velocity: Vector,
    #[field("C_BaseEntity", "m_vecVelocity")]
    velocity: NetworkVelocityVector,
    #[field("C_BaseEntity", "m_vecBaseVelocity")]
    base_velocity: Vector,
    #[field("C_BaseEntity", "m_hEffectEntity")]
    effect_entity: Handle<BaseEntity>,
    #[field("C_BaseEntity", "m_hOwnerEntity")]
    owner_entity: Handle<BaseEntity>,
    #[field("C_BaseEntity", "m_MoveCollide")]
    move_collide: MoveCollide,
    #[field("C_BaseEntity", "m_MoveType")]
    move_type: MoveType,
    #[field("C_BaseEntity", "m_flWaterLevel")]
    water_level: f32,
    #[field("C_BaseEntity", "m_fEffects")]
    effects: u32,
    #[field("C_BaseEntity", "m_hGroundEntity")]
    ground_entity: Handle<BaseEntity>,
    #[field("C_BaseEntity", "m_flFriction")]
    friction: f32,
    #[field("C_BaseEntity", "m_flElasticity")]
    elasticity: f32,
    #[field("C_BaseEntity", "m_flGravityScale")]
    gravity_scale: f32,
    #[field("C_BaseEntity", "m_flTimeScale")]
    time_scale: f32,
    #[field("C_BaseEntity", "m_bSimulatedEveryTick")]
    simulated_every_tick: bool,
    #[field("C_BaseEntity", "m_bAnimatedEveryTick")]
    animated_every_tick: bool,
    #[field("C_BaseEntity", "m_flNavIgnoreUntilTime")]
    nav_ignore_until_time: GameTime,
    #[field("C_BaseEntity", "m_hThink")]
    think: u16,
    #[field("C_BaseEntity", "m_fBBoxVisFlags")]
    b_box_vis_flags: u8,
    #[field("C_BaseEntity", "m_bPredictable")]
    predictable: bool,
    #[field("C_BaseEntity", "m_bRenderWithViewModels")]
    render_with_view_models: bool,
    #[field("C_BaseEntity", "m_nSplitUserPlayerPredictionSlot")]
    split_user_player_prediction_slot: SplitScreenSlot,
    #[field("C_BaseEntity", "m_nFirstPredictableCommand")]
    first_predictable_command: i32,
    #[field("C_BaseEntity", "m_nLastPredictableCommand")]
    last_predictable_command: i32,
    #[field("C_BaseEntity", "m_hOldMoveParent")]
    old_move_parent: Handle<BaseEntity>,
    #[field("C_BaseEntity", "m_Particles")]
    particles: ParticleProperty,
    #[field("C_BaseEntity", "m_vecPredictedScriptFloats")]
    predicted_script_floats: UtlVector<f32>,
    #[field("C_BaseEntity", "m_vecPredictedScriptFloatIDs")]
    predicted_script_float_i_ds: UtlVector<i32>,
    #[field("C_BaseEntity", "m_nNextScriptVarRecordID")]
    next_script_var_record_id: i32,
    #[field("C_BaseEntity", "m_vecAngVelocity")]
    ang_velocity: QAngle,
    #[field("C_BaseEntity", "m_DataChangeEventRef")]
    data_change_event_ref: i32,
    #[field("C_BaseEntity", "m_dependencies")]
    dependencies: UtlVector<EntityHandle>,
    #[field("C_BaseEntity", "m_nCreationTick")]
    creation_tick: i32,
    #[field("C_BaseEntity", "m_bAnimTimeChanged")]
    anim_time_changed: bool,
    #[field("C_BaseEntity", "m_bSimulationTimeChanged")]
    simulation_time_changed: bool,
    #[field("C_BaseEntity", "m_sUniqueHammerID")]
    unique_hammer_id: UtlString,
    #[field("C_BaseModelEntity", "m_CRenderComponent")]
    c_render_component: *const RenderComponent,
    #[field("C_BaseModelEntity", "m_CHitboxComponent")]
    hitbox_component: HitboxComponent,
    #[field("C_BaseModelEntity", "m_bInitModelEffects")]
    init_model_effects: bool,
    #[field("C_BaseModelEntity", "m_bIsStaticProp")]
    is_static_prop: bool,
    #[field("C_BaseModelEntity", "m_nLastAddDecal")]
    last_add_decal: i32,
    #[field("C_BaseModelEntity", "m_nDecalsAdded")]
    decals_added: i32,
    #[field("C_BaseModelEntity", "m_iOldHealth")]
    old_health: i32,
    #[field("C_BaseModelEntity", "m_nRenderMode")]
    render_mode: RenderMode,
    #[field("C_BaseModelEntity", "m_nRenderFX")]
    render_fx: RenderFx,
    #[field("C_BaseModelEntity", "m_bAllowFadeInView")]
    allow_fade_in_view: bool,
    #[field("C_BaseModelEntity", "m_clrRender")]
    render: Color,
    #[field("C_BaseModelEntity", "m_vecRenderAttributes")]
    render_attributes: UtlVector<EntityRenderAttribute>,
    #[field("C_BaseModelEntity", "m_LightGroup")]
    light_group: UtlStringToken,
    #[field("C_BaseModelEntity", "m_bRenderToCubemaps")]
    render_to_cubemaps: bool,
    #[field("C_BaseModelEntity", "m_Collision")]
    collision: CollisionProperty,
    #[field("C_BaseModelEntity", "m_Glow")]
    glow: GlowProperty,
    #[field("C_BaseModelEntity", "m_flGlowBackfaceMult")]
    glow_backface_mult: f32,
    #[field("C_BaseModelEntity", "m_fadeMinDist")]
    fade_min_dist: f32,
    #[field("C_BaseModelEntity", "m_fadeMaxDist")]
    fade_max_dist: f32,
    #[field("C_BaseModelEntity", "m_flFadeScale")]
    fade_scale: f32,
    #[field("C_BaseModelEntity", "m_flShadowStrength")]
    shadow_strength: f32,
    #[field("C_BaseModelEntity", "m_nObjectCulling")]
    object_culling: u8,
    #[field("C_BaseModelEntity", "m_nAddDecal")]
    add_decal: i32,
    #[field("C_BaseModelEntity", "m_vDecalPosition")]
    decal_position: Vector,
    #[field("C_BaseModelEntity", "m_vDecalForwardAxis")]
    decal_forward_axis: Vector,
    #[field("C_BaseModelEntity", "m_flDecalHealBloodRate")]
    decal_heal_blood_rate: f32,
    #[field("C_BaseModelEntity", "m_flDecalHealHeightRate")]
    decal_heal_height_rate: f32,
    #[field("C_BaseModelEntity", "m_ConfigEntitiesToPropagateMaterialDecalsTo")]
    config_entities_to_propagate_material_decals_to: UtlVector<Handle<BaseModelEntity>>,
    #[field("C_BaseModelEntity", "m_vecViewOffset")]
    view_offset: NetworkViewOffsetVector,
    #[field("C_BaseModelEntity", "m_pClientAlphaProperty")]
    client_alpha_property: *const ClientAlphaProperty,
    #[field("C_BaseModelEntity", "m_ClientOverrideTint")]
    client_override_tint: Color,
    #[field("C_BaseModelEntity", "m_bUseClientOverrideTint")]
    use_client_override_tint: bool,
    #[field("CBaseAnimGraph", "m_bInitiallyPopulateInterpHistory")]
    initially_populate_interp_history: bool,
    #[field("CBaseAnimGraph", "m_bShouldAnimateDuringGameplayPause")]
    should_animate_during_gameplay_pause: bool,
    #[field("CBaseAnimGraph", "m_bSuppressAnimEventSounds")]
    suppress_anim_event_sounds: bool,
    #[field("CBaseAnimGraph", "m_bAnimGraphUpdateEnabled")]
    anim_graph_update_enabled: bool,
    #[field("CBaseAnimGraph", "m_flMaxSlopeDistance")]
    max_slope_distance: f32,
    #[field("CBaseAnimGraph", "m_vLastSlopeCheckPos")]
    last_slope_check_pos: Vector,
    #[field("CBaseAnimGraph", "m_vecForce")]
    force: Vector,
    #[field("CBaseAnimGraph", "m_nForceBone")]
    force_bone: i32,
    #[field("CBaseAnimGraph", "m_pClientsideRagdoll")]
    clientside_ragdoll: *const BaseAnimGraph,
    #[field("CBaseAnimGraph", "m_bBuiltRagdoll")]
    built_ragdoll: bool,
    #[field("CBaseAnimGraph", "m_pRagdollPose")]
    ragdoll_pose: *const PhysicsRagdollPose,
    #[field("CBaseAnimGraph", "m_bClientRagdoll")]
    client_ragdoll: bool,
    #[field("CBaseAnimGraph", "m_bHasAnimatedMaterialAttributes")]
    has_animated_material_attributes: bool,
    #[field("C_BaseFlex", "m_flexWeight")]
    flex_weight: UtlVector<f32>,
    #[field("C_BaseFlex", "m_vLookTargetPosition")]
    look_target_position: Vector,
    #[field("C_BaseFlex", "m_blinktoggle")]
    blinktoggle: bool,
    #[field("C_BaseFlex", "m_nLastFlexUpdateFrameCount")]
    last_flex_update_frame_count: i32,
    #[field("C_BaseFlex", "m_CachedViewTarget")]
    cached_view_target: Vector,
    #[field("C_BaseFlex", "m_nNextSceneEventId")]
    next_scene_event_id: u32,
    #[field("C_BaseFlex", "m_iBlink")]
    blink: i32,
    #[field("C_BaseFlex", "m_blinktime")]
    blinktime: f32,
    #[field("C_BaseFlex", "m_prevblinktoggle")]
    prevblinktoggle: bool,
    #[field("C_BaseFlex", "m_iJawOpen")]
    jaw_open: i32,
    #[field("C_BaseFlex", "m_flJawOpenAmount")]
    jaw_open_amount: f32,
    #[field("C_BaseFlex", "m_flBlinkAmount")]
    blink_amount: f32,
    #[field("C_BaseFlex", "m_iMouthAttachment")]
    mouth_attachment: AttachmentHandle,
    #[field("C_BaseFlex", "m_iEyeAttachment")]
    eye_attachment: AttachmentHandle,
    #[field("C_BaseFlex", "m_bResetFlexWeightsOnModelChange")]
    reset_flex_weights_on_model_change: bool,
    #[field("C_BaseFlex", "m_nEyeOcclusionRendererBone")]
    eye_occlusion_renderer_bone: i32,
    #[field("C_BaseFlex", "m_mEyeOcclusionRendererCameraToBoneTransform")]
    m_eye_occlusion_renderer_camera_to_bone_transform: Matrix3X4,
    #[field("C_BaseFlex", "m_vEyeOcclusionRendererHalfExtent")]
    eye_occlusion_renderer_half_extent: Vector,
    #[field("C_BaseFlex", "m_PhonemeClasses")]
    phoneme_classes: [BaseFlexEmphasizedPhoneme; 3],
    #[field("C_EconEntity", "m_flFlexDelayTime")]
    flex_delay_time: f32,
    #[field("C_EconEntity", "m_flFlexDelayedWeight")]
    flex_delayed_weight: *const f32,
    #[field("C_EconEntity", "m_bAttributesInitialized")]
    attributes_initialized: bool,
    #[field("C_EconEntity", "m_AttributeManager")]
    attribute_manager: AttributeContainer,
    #[field("C_EconEntity", "m_OriginalOwnerXuidLow")]
    original_owner_xuid_low: u32,
    #[field("C_EconEntity", "m_OriginalOwnerXuidHigh")]
    original_owner_xuid_high: u32,
    #[field("C_EconEntity", "m_nFallbackPaintKit")]
    fallback_paint_kit: i32,
    #[field("C_EconEntity", "m_nFallbackSeed")]
    fallback_seed: i32,
    #[field("C_EconEntity", "m_flFallbackWear")]
    fallback_wear: f32,
    #[field("C_EconEntity", "m_nFallbackStatTrak")]
    fallback_stat_trak: i32,
    #[field("C_EconEntity", "m_bClientside")]
    clientside: bool,
    #[field("C_EconEntity", "m_bParticleSystemsCreated")]
    particle_systems_created: bool,
    #[field("C_EconEntity", "m_vecAttachedParticles")]
    attached_particles: UtlVector<i32>,
    #[field("C_EconEntity", "m_hViewmodelAttachment")]
    viewmodel_attachment: Handle<BaseAnimGraph>,
    #[field("C_EconEntity", "m_iOldTeam")]
    old_team: i32,
    #[field("C_EconEntity", "m_bAttachmentDirty")]
    attachment_dirty: bool,
    #[field("C_EconEntity", "m_nUnloadedModelIndex")]
    unloaded_model_index: i32,
    #[field("C_EconEntity", "m_iNumOwnerValidationRetries")]
    num_owner_validation_retries: i32,
    #[field("C_EconEntity", "m_hOldProvidee")]
    old_providee: Handle<BaseEntity>,
    #[field("C_EconEntity", "m_vecAttachedModels")]
    attached_models: UtlVector<EconEntityAttachedModelData>,
    #[field("C_BasePlayerWeapon", "m_nNextPrimaryAttackTick")]
    next_primary_attack_tick: GameTick,
    #[field("C_BasePlayerWeapon", "m_flNextPrimaryAttackTickRatio")]
    next_primary_attack_tick_ratio: f32,
    #[field("C_BasePlayerWeapon", "m_nNextSecondaryAttackTick")]
    next_secondary_attack_tick: GameTick,
    #[field("C_BasePlayerWeapon", "m_flNextSecondaryAttackTickRatio")]
    next_secondary_attack_tick_ratio: f32,
    #[field("C_BasePlayerWeapon", "m_iClip1")]
    clip_1: i32,
    #[field("C_BasePlayerWeapon", "m_iClip2")]
    clip_2: i32,
    #[field("C_BasePlayerWeapon", "m_pReserveAmmo")]
    reserve_ammo: [i32; 2],
    #[field("C_CSWeaponBase", "m_flFireSequenceStartTime")]
    fire_sequence_start_time: f32,
    #[field("C_CSWeaponBase", "m_nFireSequenceStartTimeChange")]
    fire_sequence_start_time_change: i32,
    #[field("C_CSWeaponBase", "m_nFireSequenceStartTimeAck")]
    fire_sequence_start_time_ack: i32,
    #[field("C_CSWeaponBase", "m_bPlayerFireEventIsPrimary")]
    player_fire_event_is_primary: bool,
    #[field("C_CSWeaponBase", "m_seqIdle")]
    seq_idle: HSequence,
    #[field("C_CSWeaponBase", "m_seqFirePrimary")]
    seq_fire_primary: HSequence,
    #[field("C_CSWeaponBase", "m_seqFireSecondary")]
    seq_fire_secondary: HSequence,
    #[field("C_CSWeaponBase", "m_ClientPreviousWeaponState")]
    client_previous_weapon_state: CsWeaponState,
    #[field("C_CSWeaponBase", "m_iState")]
    state: CsWeaponState,
    #[field("C_CSWeaponBase", "m_flCrosshairDistance")]
    crosshair_distance: f32,
    #[field("C_CSWeaponBase", "m_iAmmoLastCheck")]
    ammo_last_check: i32,
    #[field("C_CSWeaponBase", "m_iAlpha")]
    alpha: i32,
    #[field("C_CSWeaponBase", "m_iScopeTextureID")]
    scope_texture_id: i32,
    #[field("C_CSWeaponBase", "m_iCrosshairTextureID")]
    crosshair_texture_id: i32,
    #[field("C_CSWeaponBase", "m_flGunAccuracyPosition")]
    gun_accuracy_position: f32,
    #[field("C_CSWeaponBase", "m_nViewModelIndex")]
    view_model_index: u32,
    #[field("C_CSWeaponBase", "m_bReloadsWithClips")]
    reloads_with_clips: bool,
    #[field("C_CSWeaponBase", "m_flTimeWeaponIdle")]
    time_weapon_idle: GameTime,
    #[field("C_CSWeaponBase", "m_bFireOnEmpty")]
    fire_on_empty: bool,
    #[field("C_CSWeaponBase", "m_OnPlayerPickup")]
    on_player_pickup: EntityIoOutput,
    #[field("C_CSWeaponBase", "m_weaponMode")]
    weapon_mode: CsWeaponMode,
    #[field("C_CSWeaponBase", "m_flTurningInaccuracyDelta")]
    turning_inaccuracy_delta: f32,
    #[field("C_CSWeaponBase", "m_vecTurningInaccuracyEyeDirLast")]
    turning_inaccuracy_eye_dir_last: Vector,
    #[field("C_CSWeaponBase", "m_flTurningInaccuracy")]
    turning_inaccuracy: f32,
    #[field("C_CSWeaponBase", "m_fAccuracyPenalty")]
    accuracy_penalty: f32,
    #[field("C_CSWeaponBase", "m_flLastAccuracyUpdateTime")]
    last_accuracy_update_time: GameTime,
    #[field("C_CSWeaponBase", "m_fAccuracySmoothedForZoom")]
    accuracy_smoothed_for_zoom: f32,
    #[field("C_CSWeaponBase", "m_fScopeZoomEndTime")]
    scope_zoom_end_time: GameTime,
    #[field("C_CSWeaponBase", "m_iRecoilIndex")]
    i_recoil_index: i32,
    #[field("C_CSWeaponBase", "m_flRecoilIndex")]
    fl_recoil_index: f32,
    #[field("C_CSWeaponBase", "m_bBurstMode")]
    burst_mode: bool,
    #[field("C_CSWeaponBase", "m_flPostponeFireReadyTime")]
    postpone_fire_ready_time: GameTime,
    #[field("C_CSWeaponBase", "m_bInReload")]
    in_reload: bool,
    #[field("C_CSWeaponBase", "m_bReloadVisuallyComplete")]
    reload_visually_complete: bool,
    #[field("C_CSWeaponBase", "m_flDroppedAtTime")]
    dropped_at_time: GameTime,
    #[field("C_CSWeaponBase", "m_bIsHauledBack")]
    is_hauled_back: bool,
    #[field("C_CSWeaponBase", "m_bSilencerOn")]
    silencer_on: bool,
    #[field("C_CSWeaponBase", "m_flTimeSilencerSwitchComplete")]
    time_silencer_switch_complete: GameTime,
    #[field("C_CSWeaponBase", "m_iOriginalTeamNumber")]
    original_team_number: i32,
    #[field("C_CSWeaponBase", "m_flNextAttackRenderTimeOffset")]
    next_attack_render_time_offset: f32,
    #[field("C_CSWeaponBase", "m_bVisualsDataSet")]
    visuals_data_set: bool,
    #[field("C_CSWeaponBase", "m_bOldFirstPersonSpectatedState")]
    old_first_person_spectated_state: bool,
    #[field("C_CSWeaponBase", "m_hOurPing")]
    our_ping: Handle<BaseEntity>,
    #[field("C_CSWeaponBase", "m_nOurPingIndex")]
    our_ping_index: EntityIndex,
    #[field("C_CSWeaponBase", "m_vecOurPingPos")]
    our_ping_pos: Vector,
    #[field("C_CSWeaponBase", "m_bGlowForPing")]
    glow_for_ping: bool,
    #[field("C_CSWeaponBase", "m_bUIWeapon")]
    ui_weapon: bool,
    #[field("C_CSWeaponBase", "m_hPrevOwner")]
    prev_owner: Handle<PlayerPawn>,
    #[field("C_CSWeaponBase", "m_nDropTick")]
    drop_tick: GameTick,
    #[field("C_CSWeaponBase", "m_donated")]
    donated: bool,
    #[field("C_CSWeaponBase", "m_fLastShotTime")]
    last_shot_time: GameTime,
    #[field("C_CSWeaponBase", "m_bWasOwnedByCT")]
    was_owned_by_ct: bool,
    #[field("C_CSWeaponBase", "m_bWasOwnedByTerrorist")]
    was_owned_by_terrorist: bool,
    #[field("C_CSWeaponBase", "m_gunHeat")]
    gun_heat: f32,
    #[field("C_CSWeaponBase", "m_smokeAttachments")]
    smoke_attachments: u32,
    #[field("C_CSWeaponBase", "m_lastSmokeTime")]
    last_smoke_time: GameTime,
    #[field("C_CSWeaponBase", "m_flLastClientFireBulletTime")]
    last_client_fire_bullet_time: f32,
    #[field("C_CSWeaponBase", "m_IronSightController")]
    iron_sight_controller: IronSightController,
    #[field("C_CSWeaponBase", "m_iIronSightMode")]
    iron_sight_mode: i32,
    #[field("C_CSWeaponBase", "m_flLastLOSTraceFailureTime")]
    last_los_trace_failure_time: GameTime,
    #[field("C_CSWeaponBase", "m_iNumEmptyAttacks")]
    num_empty_attacks: i32,
}

#[derive(Schema)]
#[scope("client.dll")]
pub struct BulletHitModel {
    #[field("C_BulletHitModel", "m_matLocal")]
    mat_local: Matrix3X4,
    #[field("C_BulletHitModel", "m_iBoneIndex")]
    bone_index: i32,
    #[field("C_BulletHitModel", "m_hPlayerParent")]
    player_parent: Handle<BaseEntity>,
    #[field("C_BulletHitModel", "m_bIsHit")]
    is_hit: bool,
    #[field("C_BulletHitModel", "m_flTimeCreated")]
    time_created: f32,
    #[field("C_BulletHitModel", "m_vecStartPos")]
    start_pos: Vector,
}

#[derive(Schema)]
#[scope("client.dll")]
pub struct PickUpModelSlerper {
    #[field("C_PickUpModelSlerper", "m_hPlayerParent")]
    player_parent: Handle<BaseEntity>,
    #[field("C_PickUpModelSlerper", "m_hItem")]
    item: Handle<BaseEntity>,
    #[field("C_PickUpModelSlerper", "m_flTimePickedUp")]
    time_picked_up: f32,
    #[field("C_PickUpModelSlerper", "m_angOriginal")]
    original: QAngle,
    #[field("C_PickUpModelSlerper", "m_vecPosOriginal")]
    pos_original: Vector,
    #[field("C_PickUpModelSlerper", "m_angRandom")]
    random: QAngle,
}

#[derive(Schema)]
#[scope("client.dll")]
pub struct EntitySpottedState {
    #[field("EntitySpottedState_t", "m_bSpotted")]
    spotted: bool,
    #[field("EntitySpottedState_t", "m_bSpottedByMask")]
    spotted_by_mask: [u32; 2],
}

#[derive(Schema)]
#[scope("client.dll")]
pub struct PlayerControllerInGameMoneyServices {
    #[field("CPlayerControllerComponent", "__m_pChainEntity")]
    chain_entity: NetworkVarChainer,
    #[field("CCSPlayerController_InGameMoneyServices", "m_iAccount")]
    account: i32,
    #[field("CCSPlayerController_InGameMoneyServices", "m_iStartAccount")]
    start_account: i32,
    #[field("CCSPlayerController_InGameMoneyServices", "m_iTotalCashSpent")]
    total_cash_spent: i32,
    #[field("CCSPlayerController_InGameMoneyServices", "m_iCashSpentThisRound")]
    cash_spent_this_round: i32,
    #[field("CCSPlayerController_InGameMoneyServices", "m_nPreviousAccount")]
    previous_account: i32,
}

#[derive(Schema)]
#[scope("client.dll")]
pub struct ServerAuthoritativeWeaponSlot {
    #[field("ServerAuthoritativeWeaponSlot_t", "unClass")]
    un_class: u16,
    #[field("ServerAuthoritativeWeaponSlot_t", "unSlot")]
    un_slot: u16,
    #[field("ServerAuthoritativeWeaponSlot_t", "unItemDefIdx")]
    un_item_def_idx: u16,
}

#[derive(Schema)]
#[scope("client.dll")]
pub struct PlayerControllerInventoryServices {
    #[field("CPlayerControllerComponent", "__m_pChainEntity")]
    chain_entity: NetworkVarChainer,
    #[field("CCSPlayerController_InventoryServices", "m_unMusicID")]
    music_id: u16,
    #[field("CCSPlayerController_InventoryServices", "m_rank")]
    rank: [MedalRank; 6],
    #[field("CCSPlayerController_InventoryServices", "m_nPersonaDataPublicLevel")]
    persona_data_public_level: i32,
    #[field(
        "CCSPlayerController_InventoryServices",
        "m_nPersonaDataPublicCommendsLeader"
    )]
    persona_data_public_commends_leader: i32,
    #[field(
        "CCSPlayerController_InventoryServices",
        "m_nPersonaDataPublicCommendsTeacher"
    )]
    persona_data_public_commends_teacher: i32,
    #[field(
        "CCSPlayerController_InventoryServices",
        "m_nPersonaDataPublicCommendsFriendly"
    )]
    persona_data_public_commends_friendly: i32,
    #[field(
        "CCSPlayerController_InventoryServices",
        "m_vecServerAuthoritativeWeaponSlots"
    )]
    server_authoritative_weapon_slots: UtlVector<ServerAuthoritativeWeaponSlot>,
}

#[derive(Schema)]
#[scope("client.dll")]
pub struct PerRoundStats {
    #[field("CSPerRoundStats_t", "m_iKills")]
    kills: i32,
    #[field("CSPerRoundStats_t", "m_iDeaths")]
    deaths: i32,
    #[field("CSPerRoundStats_t", "m_iAssists")]
    assists: i32,
    #[field("CSPerRoundStats_t", "m_iDamage")]
    damage: i32,
    #[field("CSPerRoundStats_t", "m_iEquipmentValue")]
    equipment_value: i32,
    #[field("CSPerRoundStats_t", "m_iMoneySaved")]
    money_saved: i32,
    #[field("CSPerRoundStats_t", "m_iKillReward")]
    kill_reward: i32,
    #[field("CSPerRoundStats_t", "m_iLiveTime")]
    live_time: i32,
    #[field("CSPerRoundStats_t", "m_iHeadShotKills")]
    head_shot_kills: i32,
    #[field("CSPerRoundStats_t", "m_iObjective")]
    objective: i32,
    #[field("CSPerRoundStats_t", "m_iCashEarned")]
    cash_earned: i32,
    #[field("CSPerRoundStats_t", "m_iUtilityDamage")]
    utility_damage: i32,
    #[field("CSPerRoundStats_t", "m_iEnemiesFlashed")]
    enemies_flashed: i32,
}

#[derive(Schema)]
#[scope("client.dll")]
pub struct MatchStats {
    #[field("CSPerRoundStats_t", "m_iKills")]
    kills: i32,
    #[field("CSPerRoundStats_t", "m_iDeaths")]
    deaths: i32,
    #[field("CSPerRoundStats_t", "m_iAssists")]
    assists: i32,
    #[field("CSPerRoundStats_t", "m_iDamage")]
    damage: i32,
    #[field("CSPerRoundStats_t", "m_iEquipmentValue")]
    equipment_value: i32,
    #[field("CSPerRoundStats_t", "m_iMoneySaved")]
    money_saved: i32,
    #[field("CSPerRoundStats_t", "m_iKillReward")]
    kill_reward: i32,
    #[field("CSPerRoundStats_t", "m_iLiveTime")]
    live_time: i32,
    #[field("CSPerRoundStats_t", "m_iHeadShotKills")]
    head_shot_kills: i32,
    #[field("CSPerRoundStats_t", "m_iObjective")]
    objective: i32,
    #[field("CSPerRoundStats_t", "m_iCashEarned")]
    cash_earned: i32,
    #[field("CSPerRoundStats_t", "m_iUtilityDamage")]
    utility_damage: i32,
    #[field("CSPerRoundStats_t", "m_iEnemiesFlashed")]
    enemies_flashed: i32,
    #[field("CSMatchStats_t", "m_iEnemy5Ks")]
    enemy_5_ks: i32,
    #[field("CSMatchStats_t", "m_iEnemy4Ks")]
    enemy_4_ks: i32,
    #[field("CSMatchStats_t", "m_iEnemy3Ks")]
    enemy_3_ks: i32,
}

#[derive(Schema)]
#[scope("client.dll")]
pub struct PlayerControllerActionTrackingServices {
    #[field("CPlayerControllerComponent", "__m_pChainEntity")]
    chain_entity: NetworkVarChainer,
    #[field("CCSPlayerController_ActionTrackingServices", "m_perRoundStats")]
    per_round_stats: UtlVector<PerRoundStats>,
    #[field("CCSPlayerController_ActionTrackingServices", "m_matchStats")]
    match_stats: MatchStats,
    #[field("CCSPlayerController_ActionTrackingServices", "m_iNumRoundKills")]
    num_round_kills: i32,
    #[field(
        "CCSPlayerController_ActionTrackingServices",
        "m_iNumRoundKillsHeadshots"
    )]
    num_round_kills_headshots: i32,
    #[field(
        "CCSPlayerController_ActionTrackingServices",
        "m_unTotalRoundDamageDealt"
    )]
    total_round_damage_dealt: u32,
}

#[derive(Schema)]
#[scope("client.dll")]
pub struct PlayerPawnBase {
    #[field("CEntityInstance", "m_iszPrivateVScripts")]
    private_v_scripts: UtlSymbolLarge,
    #[field("CEntityInstance", "m_pEntity")]
    entity: *const EntityIdentity,
    #[field("CEntityInstance", "m_CScriptComponent")]
    script_component: *const ScriptComponent,
    #[field("C_BaseEntity", "m_CBodyComponent")]
    body_component: *const BodyComponent,
    #[field("C_BaseEntity", "m_NetworkTransmitComponent")]
    network_transmit_component: NetworkTransmitComponent,
    #[field("C_BaseEntity", "m_nLastThinkTick")]
    last_think_tick: GameTick,
    #[field("C_BaseEntity", "m_pGameSceneNode")]
    game_scene_node: *const GameSceneNode,
    #[field("C_BaseEntity", "m_pRenderComponent")]
    p_render_component: *const RenderComponent,
    #[field("C_BaseEntity", "m_pCollision")]
    p_collision: *const CollisionProperty,
    #[field("C_BaseEntity", "m_iMaxHealth")]
    max_health: i32,
    #[field("C_BaseEntity", "m_iHealth")]
    health: i32,
    #[field("C_BaseEntity", "m_lifeState")]
    life_state: u8,
    #[field("C_BaseEntity", "m_bTakesDamage")]
    takes_damage: bool,
    #[field("C_BaseEntity", "m_nTakeDamageFlags")]
    take_damage_flags: TakeDamageFlags,
    #[field("C_BaseEntity", "m_ubInterpolationFrame")]
    interpolation_frame: u8,
    #[field("C_BaseEntity", "m_hSceneObjectController")]
    scene_object_controller: Handle<BaseEntity>,
    #[field("C_BaseEntity", "m_nNoInterpolationTick")]
    no_interpolation_tick: i32,
    #[field("C_BaseEntity", "m_nVisibilityNoInterpolationTick")]
    visibility_no_interpolation_tick: i32,
    #[field("C_BaseEntity", "m_flProxyRandomValue")]
    proxy_random_value: f32,
    #[field("C_BaseEntity", "m_iEFlags")]
    e_flags: i32,
    #[field("C_BaseEntity", "m_nWaterType")]
    water_type: u8,
    #[field("C_BaseEntity", "m_bInterpolateEvenWithNoModel")]
    interpolate_even_with_no_model: bool,
    #[field("C_BaseEntity", "m_bPredictionEligible")]
    prediction_eligible: bool,
    #[field("C_BaseEntity", "m_bApplyLayerMatchIDToModel")]
    apply_layer_match_id_to_model: bool,
    #[field("C_BaseEntity", "m_tokLayerMatchID")]
    tok_layer_match_id: UtlStringToken,
    #[field("C_BaseEntity", "m_nSubclassID")]
    subclass_id: UtlStringToken,
    #[field("C_BaseEntity", "m_nSimulationTick")]
    simulation_tick: i32,
    #[field("C_BaseEntity", "m_iCurrentThinkContext")]
    current_think_context: i32,
    #[field("C_BaseEntity", "m_aThinkFunctions")]
    think_functions: UtlVector<Thinkfunc>,
    #[field("C_BaseEntity", "m_flAnimTime")]
    anim_time: f32,
    #[field("C_BaseEntity", "m_flSimulationTime")]
    simulation_time: f32,
    #[field("C_BaseEntity", "m_nSceneObjectOverrideFlags")]
    scene_object_override_flags: u8,
    #[field("C_BaseEntity", "m_bHasSuccessfullyInterpolated")]
    has_successfully_interpolated: bool,
    #[field("C_BaseEntity", "m_bHasAddedVarsToInterpolation")]
    has_added_vars_to_interpolation: bool,
    #[field("C_BaseEntity", "m_bRenderEvenWhenNotSuccessfullyInterpolated")]
    render_even_when_not_successfully_interpolated: bool,
    #[field("C_BaseEntity", "m_nInterpolationLatchDirtyFlags")]
    interpolation_latch_dirty_flags: [i32; 2],
    #[field("C_BaseEntity", "m_ListEntry")]
    list_entry: [u16; 11],
    #[field("C_BaseEntity", "m_flCreateTime")]
    create_time: GameTime,
    #[field("C_BaseEntity", "m_flSpeed")]
    speed: f32,
    #[field("C_BaseEntity", "m_EntClientFlags")]
    ent_client_flags: u16,
    #[field("C_BaseEntity", "m_bClientSideRagdoll")]
    client_side_ragdoll: bool,
    #[field("C_BaseEntity", "m_iTeamNum")]
    team_num: u8,
    #[field("C_BaseEntity", "m_spawnflags")]
    spawnflags: u32,
    #[field("C_BaseEntity", "m_nNextThinkTick")]
    next_think_tick: GameTick,
    #[field("C_BaseEntity", "m_fFlags")]
    flags: u32,
    #[field("C_BaseEntity", "m_vecAbsVelocity")]
    abs_velocity: Vector,
    #[field("C_BaseEntity", "m_vecVelocity")]
    velocity: NetworkVelocityVector,
    #[field("C_BaseEntity", "m_vecBaseVelocity")]
    base_velocity: Vector,
    #[field("C_BaseEntity", "m_hEffectEntity")]
    effect_entity: Handle<BaseEntity>,
    #[field("C_BaseEntity", "m_hOwnerEntity")]
    owner_entity: Handle<BaseEntity>,
    #[field("C_BaseEntity", "m_MoveCollide")]
    move_collide: MoveCollide,
    #[field("C_BaseEntity", "m_MoveType")]
    move_type: MoveType,
    #[field("C_BaseEntity", "m_flWaterLevel")]
    water_level: f32,
    #[field("C_BaseEntity", "m_fEffects")]
    effects: u32,
    #[field("C_BaseEntity", "m_hGroundEntity")]
    ground_entity: Handle<BaseEntity>,
    #[field("C_BaseEntity", "m_flFriction")]
    friction: f32,
    #[field("C_BaseEntity", "m_flElasticity")]
    elasticity: f32,
    #[field("C_BaseEntity", "m_flGravityScale")]
    gravity_scale: f32,
    #[field("C_BaseEntity", "m_flTimeScale")]
    time_scale: f32,
    #[field("C_BaseEntity", "m_bSimulatedEveryTick")]
    simulated_every_tick: bool,
    #[field("C_BaseEntity", "m_bAnimatedEveryTick")]
    animated_every_tick: bool,
    #[field("C_BaseEntity", "m_flNavIgnoreUntilTime")]
    nav_ignore_until_time: GameTime,
    #[field("C_BaseEntity", "m_hThink")]
    think: u16,
    #[field("C_BaseEntity", "m_fBBoxVisFlags")]
    b_box_vis_flags: u8,
    #[field("C_BaseEntity", "m_bPredictable")]
    predictable: bool,
    #[field("C_BaseEntity", "m_bRenderWithViewModels")]
    render_with_view_models: bool,
    #[field("C_BaseEntity", "m_nSplitUserPlayerPredictionSlot")]
    split_user_player_prediction_slot: SplitScreenSlot,
    #[field("C_BaseEntity", "m_nFirstPredictableCommand")]
    first_predictable_command: i32,
    #[field("C_BaseEntity", "m_nLastPredictableCommand")]
    last_predictable_command: i32,
    #[field("C_BaseEntity", "m_hOldMoveParent")]
    old_move_parent: Handle<BaseEntity>,
    #[field("C_BaseEntity", "m_Particles")]
    particles: ParticleProperty,
    #[field("C_BaseEntity", "m_vecPredictedScriptFloats")]
    predicted_script_floats: UtlVector<f32>,
    #[field("C_BaseEntity", "m_vecPredictedScriptFloatIDs")]
    predicted_script_float_i_ds: UtlVector<i32>,
    #[field("C_BaseEntity", "m_nNextScriptVarRecordID")]
    next_script_var_record_id: i32,
    #[field("C_BaseEntity", "m_vecAngVelocity")]
    ang_velocity: QAngle,
    #[field("C_BaseEntity", "m_DataChangeEventRef")]
    data_change_event_ref: i32,
    #[field("C_BaseEntity", "m_dependencies")]
    dependencies: UtlVector<EntityHandle>,
    #[field("C_BaseEntity", "m_nCreationTick")]
    creation_tick: i32,
    #[field("C_BaseEntity", "m_bAnimTimeChanged")]
    anim_time_changed: bool,
    #[field("C_BaseEntity", "m_bSimulationTimeChanged")]
    simulation_time_changed: bool,
    #[field("C_BaseEntity", "m_sUniqueHammerID")]
    unique_hammer_id: UtlString,
    #[field("C_BaseModelEntity", "m_CRenderComponent")]
    c_render_component: *const RenderComponent,
    #[field("C_BaseModelEntity", "m_CHitboxComponent")]
    hitbox_component: HitboxComponent,
    #[field("C_BaseModelEntity", "m_bInitModelEffects")]
    init_model_effects: bool,
    #[field("C_BaseModelEntity", "m_bIsStaticProp")]
    is_static_prop: bool,
    #[field("C_BaseModelEntity", "m_nLastAddDecal")]
    last_add_decal: i32,
    #[field("C_BaseModelEntity", "m_nDecalsAdded")]
    decals_added: i32,
    #[field("C_BaseModelEntity", "m_iOldHealth")]
    old_health: i32,
    #[field("C_BaseModelEntity", "m_nRenderMode")]
    render_mode: RenderMode,
    #[field("C_BaseModelEntity", "m_nRenderFX")]
    render_fx: RenderFx,
    #[field("C_BaseModelEntity", "m_bAllowFadeInView")]
    allow_fade_in_view: bool,
    #[field("C_BaseModelEntity", "m_clrRender")]
    render: Color,
    #[field("C_BaseModelEntity", "m_vecRenderAttributes")]
    render_attributes: UtlVector<EntityRenderAttribute>,
    #[field("C_BaseModelEntity", "m_LightGroup")]
    light_group: UtlStringToken,
    #[field("C_BaseModelEntity", "m_bRenderToCubemaps")]
    render_to_cubemaps: bool,
    #[field("C_BaseModelEntity", "m_Collision")]
    collision: CollisionProperty,
    #[field("C_BaseModelEntity", "m_Glow")]
    glow: GlowProperty,
    #[field("C_BaseModelEntity", "m_flGlowBackfaceMult")]
    glow_backface_mult: f32,
    #[field("C_BaseModelEntity", "m_fadeMinDist")]
    fade_min_dist: f32,
    #[field("C_BaseModelEntity", "m_fadeMaxDist")]
    fade_max_dist: f32,
    #[field("C_BaseModelEntity", "m_flFadeScale")]
    fade_scale: f32,
    #[field("C_BaseModelEntity", "m_flShadowStrength")]
    shadow_strength: f32,
    #[field("C_BaseModelEntity", "m_nObjectCulling")]
    object_culling: u8,
    #[field("C_BaseModelEntity", "m_nAddDecal")]
    add_decal: i32,
    #[field("C_BaseModelEntity", "m_vDecalPosition")]
    decal_position: Vector,
    #[field("C_BaseModelEntity", "m_vDecalForwardAxis")]
    decal_forward_axis: Vector,
    #[field("C_BaseModelEntity", "m_flDecalHealBloodRate")]
    decal_heal_blood_rate: f32,
    #[field("C_BaseModelEntity", "m_flDecalHealHeightRate")]
    decal_heal_height_rate: f32,
    #[field("C_BaseModelEntity", "m_ConfigEntitiesToPropagateMaterialDecalsTo")]
    config_entities_to_propagate_material_decals_to: UtlVector<Handle<BaseModelEntity>>,
    #[field("C_BaseModelEntity", "m_vecViewOffset")]
    view_offset: NetworkViewOffsetVector,
    #[field("C_BaseModelEntity", "m_pClientAlphaProperty")]
    client_alpha_property: *const ClientAlphaProperty,
    #[field("C_BaseModelEntity", "m_ClientOverrideTint")]
    client_override_tint: Color,
    #[field("C_BaseModelEntity", "m_bUseClientOverrideTint")]
    use_client_override_tint: bool,
    #[field("CBaseAnimGraph", "m_bInitiallyPopulateInterpHistory")]
    initially_populate_interp_history: bool,
    #[field("CBaseAnimGraph", "m_bShouldAnimateDuringGameplayPause")]
    should_animate_during_gameplay_pause: bool,
    #[field("CBaseAnimGraph", "m_bSuppressAnimEventSounds")]
    suppress_anim_event_sounds: bool,
    #[field("CBaseAnimGraph", "m_bAnimGraphUpdateEnabled")]
    anim_graph_update_enabled: bool,
    #[field("CBaseAnimGraph", "m_flMaxSlopeDistance")]
    max_slope_distance: f32,
    #[field("CBaseAnimGraph", "m_vLastSlopeCheckPos")]
    last_slope_check_pos: Vector,
    #[field("CBaseAnimGraph", "m_vecForce")]
    force: Vector,
    #[field("CBaseAnimGraph", "m_nForceBone")]
    force_bone: i32,
    #[field("CBaseAnimGraph", "m_pClientsideRagdoll")]
    clientside_ragdoll: *const BaseAnimGraph,
    #[field("CBaseAnimGraph", "m_bBuiltRagdoll")]
    built_ragdoll: bool,
    #[field("CBaseAnimGraph", "m_pRagdollPose")]
    ragdoll_pose: *const PhysicsRagdollPose,
    #[field("CBaseAnimGraph", "m_bClientRagdoll")]
    client_ragdoll: bool,
    #[field("CBaseAnimGraph", "m_bHasAnimatedMaterialAttributes")]
    has_animated_material_attributes: bool,
    #[field("C_BaseFlex", "m_flexWeight")]
    flex_weight: UtlVector<f32>,
    #[field("C_BaseFlex", "m_vLookTargetPosition")]
    look_target_position: Vector,
    #[field("C_BaseFlex", "m_blinktoggle")]
    blinktoggle: bool,
    #[field("C_BaseFlex", "m_nLastFlexUpdateFrameCount")]
    last_flex_update_frame_count: i32,
    #[field("C_BaseFlex", "m_CachedViewTarget")]
    cached_view_target: Vector,
    #[field("C_BaseFlex", "m_nNextSceneEventId")]
    next_scene_event_id: u32,
    #[field("C_BaseFlex", "m_iBlink")]
    blink: i32,
    #[field("C_BaseFlex", "m_blinktime")]
    blinktime: f32,
    #[field("C_BaseFlex", "m_prevblinktoggle")]
    prevblinktoggle: bool,
    #[field("C_BaseFlex", "m_iJawOpen")]
    jaw_open: i32,
    #[field("C_BaseFlex", "m_flJawOpenAmount")]
    jaw_open_amount: f32,
    #[field("C_BaseFlex", "m_flBlinkAmount")]
    blink_amount: f32,
    #[field("C_BaseFlex", "m_iMouthAttachment")]
    mouth_attachment: AttachmentHandle,
    #[field("C_BaseFlex", "m_iEyeAttachment")]
    eye_attachment: AttachmentHandle,
    #[field("C_BaseFlex", "m_bResetFlexWeightsOnModelChange")]
    reset_flex_weights_on_model_change: bool,
    #[field("C_BaseFlex", "m_nEyeOcclusionRendererBone")]
    eye_occlusion_renderer_bone: i32,
    #[field("C_BaseFlex", "m_mEyeOcclusionRendererCameraToBoneTransform")]
    m_eye_occlusion_renderer_camera_to_bone_transform: Matrix3X4,
    #[field("C_BaseFlex", "m_vEyeOcclusionRendererHalfExtent")]
    eye_occlusion_renderer_half_extent: Vector,
    #[field("C_BaseFlex", "m_PhonemeClasses")]
    phoneme_classes: [BaseFlexEmphasizedPhoneme; 3],
    #[field("C_BaseCombatCharacter", "m_hMyWearables")]
    my_wearables: UtlVector<Handle<EconWearable>>,
    #[field("C_BaseCombatCharacter", "m_bloodColor")]
    blood_color: i32,
    #[field("C_BaseCombatCharacter", "m_leftFootAttachment")]
    left_foot_attachment: AttachmentHandle,
    #[field("C_BaseCombatCharacter", "m_rightFootAttachment")]
    right_foot_attachment: AttachmentHandle,
    #[field("C_BaseCombatCharacter", "m_nWaterWakeMode")]
    water_wake_mode: WaterWakeMode,
    #[field("C_BaseCombatCharacter", "m_flWaterWorldZ")]
    water_world_z: f32,
    #[field("C_BaseCombatCharacter", "m_flWaterNextTraceTime")]
    water_next_trace_time: f32,
    #[field("C_BaseCombatCharacter", "m_flFieldOfView")]
    field_of_view: f32,
    #[field("C_BasePlayerPawn", "m_pWeaponServices")]
    weapon_services: *const PlayerWeaponServices,
    #[field("C_BasePlayerPawn", "m_pItemServices")]
    item_services: *const PlayerItemServices,
    #[field("C_BasePlayerPawn", "m_pAutoaimServices")]
    autoaim_services: *const PlayerAutoaimServices,
    #[field("C_BasePlayerPawn", "m_pObserverServices")]
    observer_services: *const PlayerObserverServices,
    #[field("C_BasePlayerPawn", "m_pWaterServices")]
    water_services: *const PlayerWaterServices,
    #[field("C_BasePlayerPawn", "m_pUseServices")]
    use_services: *const PlayerUseServices,
    #[field("C_BasePlayerPawn", "m_pFlashlightServices")]
    flashlight_services: *const PlayerFlashlightServices,
    #[field("C_BasePlayerPawn", "m_pCameraServices")]
    camera_services: *const PlayerCameraServices,
    #[field("C_BasePlayerPawn", "m_pMovementServices")]
    movement_services: *const PlayerMovementServices,
    #[field("C_BasePlayerPawn", "m_ServerViewAngleChanges")]
    server_view_angle_changes: UtlVector<ViewAngleServerChange>,
    #[field("C_BasePlayerPawn", "m_nHighestConsumedServerViewAngleChangeIndex")]
    highest_consumed_server_view_angle_change_index: u32,
    #[field("C_BasePlayerPawn", "v_angle")]
    v_angle: QAngle,
    #[field("C_BasePlayerPawn", "v_anglePrevious")]
    v_angle_previous: QAngle,
    #[field("C_BasePlayerPawn", "m_iHideHUD")]
    hide_hud: u32,
    #[field("C_BasePlayerPawn", "m_skybox3d")]
    skybox_3_d: Sky3Dparams,
    #[field("C_BasePlayerPawn", "m_flDeathTime")]
    death_time: GameTime,
    #[field("C_BasePlayerPawn", "m_vecPredictionError")]
    prediction_error: Vector,
    #[field("C_BasePlayerPawn", "m_flPredictionErrorTime")]
    prediction_error_time: GameTime,
    #[field("C_BasePlayerPawn", "m_flFOVSensitivityAdjust")]
    fov_sensitivity_adjust: f32,
    #[field("C_BasePlayerPawn", "m_flMouseSensitivity")]
    mouse_sensitivity: f32,
    #[field("C_BasePlayerPawn", "m_vOldOrigin")]
    old_origin: Vector,
    #[field("C_BasePlayerPawn", "m_flOldSimulationTime")]
    old_simulation_time: f32,
    #[field("C_BasePlayerPawn", "m_nLastExecutedCommandNumber")]
    last_executed_command_number: i32,
    #[field("C_BasePlayerPawn", "m_nLastExecutedCommandTick")]
    last_executed_command_tick: i32,
    #[field("C_BasePlayerPawn", "m_hController")]
    controller: Handle<BasePlayerController>,
    #[field("C_BasePlayerPawn", "m_bIsSwappingToPredictableController")]
    is_swapping_to_predictable_controller: bool,
    #[field("C_CSPlayerPawnBase", "m_pPingServices")]
    ping_services: *const PlayerPingServices,
    #[field("C_CSPlayerPawnBase", "m_pViewModelServices")]
    view_model_services: *const PlayerViewModelServices,
    #[field("C_CSPlayerPawnBase", "m_fRenderingClipPlane")]
    rendering_clip_plane: [f32; 4],
    #[field("C_CSPlayerPawnBase", "m_nLastClipPlaneSetupFrame")]
    last_clip_plane_setup_frame: i32,
    #[field("C_CSPlayerPawnBase", "m_vecLastClipCameraPos")]
    last_clip_camera_pos: Vector,
    #[field("C_CSPlayerPawnBase", "m_vecLastClipCameraForward")]
    last_clip_camera_forward: Vector,
    #[field("C_CSPlayerPawnBase", "m_bClipHitStaticWorld")]
    clip_hit_static_world: bool,
    #[field("C_CSPlayerPawnBase", "m_bCachedPlaneIsValid")]
    cached_plane_is_valid: bool,
    #[field("C_CSPlayerPawnBase", "m_pClippingWeapon")]
    clipping_weapon: *const WeaponBase,
    #[field("C_CSPlayerPawnBase", "m_previousPlayerState")]
    previous_player_state: CsPlayerState,
    #[field("C_CSPlayerPawnBase", "m_flLastCollisionCeiling")]
    last_collision_ceiling: f32,
    #[field("C_CSPlayerPawnBase", "m_flLastCollisionCeilingChangeTime")]
    last_collision_ceiling_change_time: f32,
    #[field("C_CSPlayerPawnBase", "m_grenadeParameterStashTime")]
    grenade_parameter_stash_time: GameTime,
    #[field("C_CSPlayerPawnBase", "m_bGrenadeParametersStashed")]
    grenade_parameters_stashed: bool,
    #[field("C_CSPlayerPawnBase", "m_angStashedShootAngles")]
    stashed_shoot_angles: QAngle,
    #[field("C_CSPlayerPawnBase", "m_vecStashedGrenadeThrowPosition")]
    stashed_grenade_throw_position: Vector,
    #[field("C_CSPlayerPawnBase", "m_vecStashedVelocity")]
    stashed_velocity: Vector,
    #[field("C_CSPlayerPawnBase", "m_angShootAngleHistory")]
    shoot_angle_history: [QAngle; 2],
    #[field("C_CSPlayerPawnBase", "m_vecThrowPositionHistory")]
    throw_position_history: [Vector; 2],
    #[field("C_CSPlayerPawnBase", "m_vecVelocityHistory")]
    velocity_history: [Vector; 2],
    #[field("C_CSPlayerPawnBase", "m_thirdPersonHeading")]
    third_person_heading: QAngle,
    #[field("C_CSPlayerPawnBase", "m_flSlopeDropOffset")]
    slope_drop_offset: f32,
    #[field("C_CSPlayerPawnBase", "m_flSlopeDropHeight")]
    slope_drop_height: f32,
    #[field("C_CSPlayerPawnBase", "m_vHeadConstraintOffset")]
    head_constraint_offset: Vector,
    #[field("C_CSPlayerPawnBase", "m_bIsScoped")]
    is_scoped: bool,
    #[field("C_CSPlayerPawnBase", "m_bIsWalking")]
    is_walking: bool,
    #[field("C_CSPlayerPawnBase", "m_bResumeZoom")]
    resume_zoom: bool,
    #[field("C_CSPlayerPawnBase", "m_iPlayerState")]
    player_state: CsPlayerState,
    #[field("C_CSPlayerPawnBase", "m_bIsDefusing")]
    is_defusing: bool,
    #[field("C_CSPlayerPawnBase", "m_bIsGrabbingHostage")]
    is_grabbing_hostage: bool,
    #[field("C_CSPlayerPawnBase", "m_iBlockingUseActionInProgress")]
    blocking_use_action_in_progress: CsPlayerBlockingUseAction,
    #[field("C_CSPlayerPawnBase", "m_bIsRescuing")]
    is_rescuing: bool,
    #[field("C_CSPlayerPawnBase", "m_fImmuneToGunGameDamageTime")]
    immune_to_gun_game_damage_time: GameTime,
    #[field("C_CSPlayerPawnBase", "m_fImmuneToGunGameDamageTimeLast")]
    immune_to_gun_game_damage_time_last: GameTime,
    #[field("C_CSPlayerPawnBase", "m_bGunGameImmunity")]
    gun_game_immunity: bool,
    #[field("C_CSPlayerPawnBase", "m_bHasMovedSinceSpawn")]
    has_moved_since_spawn: bool,
    #[field("C_CSPlayerPawnBase", "m_fMolotovUseTime")]
    molotov_use_time: f32,
    #[field("C_CSPlayerPawnBase", "m_fMolotovDamageTime")]
    molotov_damage_time: f32,
    #[field("C_CSPlayerPawnBase", "m_nWhichBombZone")]
    which_bomb_zone: i32,
    #[field("C_CSPlayerPawnBase", "m_bInNoDefuseArea")]
    in_no_defuse_area: bool,
    #[field("C_CSPlayerPawnBase", "m_iThrowGrenadeCounter")]
    throw_grenade_counter: i32,
    #[field("C_CSPlayerPawnBase", "m_bWaitForNoAttack")]
    wait_for_no_attack: bool,
    #[field("C_CSPlayerPawnBase", "m_flGuardianTooFarDistFrac")]
    guardian_too_far_dist_frac: f32,
    #[field("C_CSPlayerPawnBase", "m_flDetectedByEnemySensorTime")]
    detected_by_enemy_sensor_time: GameTime,
    #[field("C_CSPlayerPawnBase", "m_flNextGuardianTooFarWarning")]
    next_guardian_too_far_warning: f32,
    #[field("C_CSPlayerPawnBase", "m_bSuppressGuardianTooFarWarningAudio")]
    suppress_guardian_too_far_warning_audio: bool,
    #[field("C_CSPlayerPawnBase", "m_bKilledByTaser")]
    killed_by_taser: bool,
    #[field("C_CSPlayerPawnBase", "m_iMoveState")]
    move_state: i32,
    #[field("C_CSPlayerPawnBase", "m_bCanMoveDuringFreezePeriod")]
    can_move_during_freeze_period: bool,
    #[field("C_CSPlayerPawnBase", "m_flLowerBodyYawTarget")]
    lower_body_yaw_target: f32,
    #[field("C_CSPlayerPawnBase", "m_bStrafing")]
    strafing: bool,
    #[field("C_CSPlayerPawnBase", "m_flLastSpawnTimeIndex")]
    last_spawn_time_index: GameTime,
    #[field("C_CSPlayerPawnBase", "m_flEmitSoundTime")]
    emit_sound_time: GameTime,
    #[field("C_CSPlayerPawnBase", "m_iAddonBits")]
    addon_bits: i32,
    #[field("C_CSPlayerPawnBase", "m_iPrimaryAddon")]
    primary_addon: i32,
    #[field("C_CSPlayerPawnBase", "m_iSecondaryAddon")]
    secondary_addon: i32,
    #[field("C_CSPlayerPawnBase", "m_iProgressBarDuration")]
    progress_bar_duration: i32,
    #[field("C_CSPlayerPawnBase", "m_flProgressBarStartTime")]
    progress_bar_start_time: f32,
    #[field("C_CSPlayerPawnBase", "m_iDirection")]
    direction: i32,
    #[field("C_CSPlayerPawnBase", "m_iShotsFired")]
    shots_fired: i32,
    #[field("C_CSPlayerPawnBase", "m_bNightVisionOn")]
    night_vision_on: bool,
    #[field("C_CSPlayerPawnBase", "m_bHasNightVision")]
    has_night_vision: bool,
    #[field("C_CSPlayerPawnBase", "m_flVelocityModifier")]
    velocity_modifier: f32,
    #[field("C_CSPlayerPawnBase", "m_flHitHeading")]
    hit_heading: f32,
    #[field("C_CSPlayerPawnBase", "m_nHitBodyPart")]
    hit_body_part: i32,
    #[field("C_CSPlayerPawnBase", "m_iStartAccount")]
    start_account: i32,
    #[field("C_CSPlayerPawnBase", "m_vecIntroStartEyePosition")]
    intro_start_eye_position: Vector,
    #[field("C_CSPlayerPawnBase", "m_vecIntroStartPlayerForward")]
    intro_start_player_forward: Vector,
    #[field("C_CSPlayerPawnBase", "m_flClientDeathTime")]
    client_death_time: GameTime,
    #[field("C_CSPlayerPawnBase", "m_flNightVisionAlpha")]
    night_vision_alpha: f32,
    #[field("C_CSPlayerPawnBase", "m_bScreenTearFrameCaptured")]
    screen_tear_frame_captured: bool,
    #[field("C_CSPlayerPawnBase", "m_flFlashBangTime")]
    flash_bang_time: f32,
    #[field("C_CSPlayerPawnBase", "m_flFlashScreenshotAlpha")]
    flash_screenshot_alpha: f32,
    #[field("C_CSPlayerPawnBase", "m_flFlashOverlayAlpha")]
    flash_overlay_alpha: f32,
    #[field("C_CSPlayerPawnBase", "m_bFlashBuildUp")]
    flash_build_up: bool,
    #[field("C_CSPlayerPawnBase", "m_bFlashDspHasBeenCleared")]
    flash_dsp_has_been_cleared: bool,
    #[field("C_CSPlayerPawnBase", "m_bFlashScreenshotHasBeenGrabbed")]
    flash_screenshot_has_been_grabbed: bool,
    #[field("C_CSPlayerPawnBase", "m_flFlashMaxAlpha")]
    flash_max_alpha: f32,
    #[field("C_CSPlayerPawnBase", "m_flFlashDuration")]
    flash_duration: f32,
    #[field("C_CSPlayerPawnBase", "m_lastStandingPos")]
    last_standing_pos: Vector,
    #[field("C_CSPlayerPawnBase", "m_vecLastMuzzleFlashPos")]
    last_muzzle_flash_pos: Vector,
    #[field("C_CSPlayerPawnBase", "m_angLastMuzzleFlashAngle")]
    last_muzzle_flash_angle: QAngle,
    #[field("C_CSPlayerPawnBase", "m_hMuzzleFlashShape")]
    muzzle_flash_shape: Handle<BaseEntity>,
    #[field("C_CSPlayerPawnBase", "m_iHealthBarRenderMaskIndex")]
    health_bar_render_mask_index: i32,
    #[field("C_CSPlayerPawnBase", "m_flHealthFadeValue")]
    health_fade_value: f32,
    #[field("C_CSPlayerPawnBase", "m_flHealthFadeAlpha")]
    health_fade_alpha: f32,
    #[field("C_CSPlayerPawnBase", "m_nMyCollisionGroup")]
    my_collision_group: i32,
    #[field("C_CSPlayerPawnBase", "m_ignoreLadderJumpTime")]
    ignore_ladder_jump_time: f32,
    #[field("C_CSPlayerPawnBase", "m_ladderSurpressionTimer")]
    ladder_surpression_timer: CountdownTimer,
    #[field("C_CSPlayerPawnBase", "m_lastLadderNormal")]
    last_ladder_normal: Vector,
    #[field("C_CSPlayerPawnBase", "m_lastLadderPos")]
    last_ladder_pos: Vector,
    #[field("C_CSPlayerPawnBase", "m_flDeathCCWeight")]
    death_cc_weight: f32,
    #[field("C_CSPlayerPawnBase", "m_bOldIsScoped")]
    old_is_scoped: bool,
    #[field("C_CSPlayerPawnBase", "m_flPrevRoundEndTime")]
    prev_round_end_time: f32,
    #[field("C_CSPlayerPawnBase", "m_flPrevMatchEndTime")]
    prev_match_end_time: f32,
    #[field("C_CSPlayerPawnBase", "m_unCurrentEquipmentValue")]
    current_equipment_value: u16,
    #[field("C_CSPlayerPawnBase", "m_unRoundStartEquipmentValue")]
    round_start_equipment_value: u16,
    #[field("C_CSPlayerPawnBase", "m_unFreezetimeEndEquipmentValue")]
    freezetime_end_equipment_value: u16,
    #[field("C_CSPlayerPawnBase", "m_vecThirdPersonViewPositionOverride")]
    third_person_view_position_override: Vector,
    #[field("C_CSPlayerPawnBase", "m_nHeavyAssaultSuitCooldownRemaining")]
    heavy_assault_suit_cooldown_remaining: i32,
    #[field("C_CSPlayerPawnBase", "m_ArmorValue")]
    armor_value: i32,
    #[field("C_CSPlayerPawnBase", "m_angEyeAngles")]
    eye_angles: QAngle,
    #[field("C_CSPlayerPawnBase", "m_fNextThinkPushAway")]
    next_think_push_away: f32,
    #[field("C_CSPlayerPawnBase", "m_bShouldAutobuyDMWeapons")]
    should_autobuy_dm_weapons: bool,
    #[field("C_CSPlayerPawnBase", "m_bShouldAutobuyNow")]
    should_autobuy_now: bool,
    #[field("C_CSPlayerPawnBase", "m_bHud_MiniScoreHidden")]
    hud_mini_score_hidden: bool,
    #[field("C_CSPlayerPawnBase", "m_bHud_RadarHidden")]
    hud_radar_hidden: bool,
    #[field("C_CSPlayerPawnBase", "m_nLastKillerIndex")]
    last_killer_index: EntityIndex,
    #[field("C_CSPlayerPawnBase", "m_nLastConcurrentKilled")]
    last_concurrent_killed: i32,
    #[field("C_CSPlayerPawnBase", "m_nDeathCamMusic")]
    death_cam_music: i32,
    #[field("C_CSPlayerPawnBase", "m_iIDEntIndex")]
    id_ent_index: EntityIndex,
    #[field("C_CSPlayerPawnBase", "m_delayTargetIDTimer")]
    delay_target_id_timer: CountdownTimer,
    #[field("C_CSPlayerPawnBase", "m_iTargetedWeaponEntIndex")]
    targeted_weapon_ent_index: EntityIndex,
    #[field("C_CSPlayerPawnBase", "m_iOldIDEntIndex")]
    old_id_ent_index: EntityIndex,
    #[field("C_CSPlayerPawnBase", "m_holdTargetIDTimer")]
    hold_target_id_timer: CountdownTimer,
    #[field("C_CSPlayerPawnBase", "m_flCurrentMusicStartTime")]
    current_music_start_time: f32,
    #[field("C_CSPlayerPawnBase", "m_flMusicRoundStartTime")]
    music_round_start_time: f32,
    #[field("C_CSPlayerPawnBase", "m_bDeferStartMusicOnWarmup")]
    defer_start_music_on_warmup: bool,
    #[field("C_CSPlayerPawnBase", "m_cycleLatch")]
    cycle_latch: i32,
    #[field("C_CSPlayerPawnBase", "m_serverIntendedCycle")]
    server_intended_cycle: f32,
    #[field("C_CSPlayerPawnBase", "m_vecPlayerPatchEconIndices")]
    player_patch_econ_indices: [u32; 5],
    #[field("C_CSPlayerPawnBase", "m_bHideTargetID")]
    hide_target_id: bool,
    #[field("C_CSPlayerPawnBase", "m_nextTaserShakeTime")]
    next_taser_shake_time: f32,
    #[field("C_CSPlayerPawnBase", "m_firstTaserShakeTime")]
    first_taser_shake_time: f32,
    #[field("C_CSPlayerPawnBase", "m_flLastSmokeOverlayAlpha")]
    last_smoke_overlay_alpha: f32,
    #[field("C_CSPlayerPawnBase", "m_vLastSmokeOverlayColor")]
    last_smoke_overlay_color: Vector,
    #[field("C_CSPlayerPawnBase", "m_nPlayerSmokedFx")]
    player_smoked_fx: ParticleIndex,
    #[field("C_CSPlayerPawnBase", "m_flNextMagDropTime")]
    next_mag_drop_time: f32,
    #[field("C_CSPlayerPawnBase", "m_nLastMagDropAttachmentIndex")]
    last_mag_drop_attachment_index: i32,
    #[field("C_CSPlayerPawnBase", "m_vecBulletHitModels")]
    bullet_hit_models: UtlVector<*const BulletHitModel>,
    #[field("C_CSPlayerPawnBase", "m_vecPickupModelSlerpers")]
    pickup_model_slerpers: UtlVector<*const PickUpModelSlerper>,
    #[field("C_CSPlayerPawnBase", "m_vecLastAliveLocalVelocity")]
    last_alive_local_velocity: Vector,
    #[field("C_CSPlayerPawnBase", "m_entitySpottedState")]
    entity_spotted_state: EntitySpottedState,
    #[field("C_CSPlayerPawnBase", "m_nSurvivalTeamNumber")]
    survival_team_number: i32,
    #[field("C_CSPlayerPawnBase", "m_bGuardianShouldSprayCustomXMark")]
    guardian_should_spray_custom_x_mark: bool,
    #[field("C_CSPlayerPawnBase", "m_bHasDeathInfo")]
    has_death_info: bool,
    #[field("C_CSPlayerPawnBase", "m_flDeathInfoTime")]
    death_info_time: f32,
    #[field("C_CSPlayerPawnBase", "m_vecDeathInfoOrigin")]
    death_info_origin: Vector,
    #[field("C_CSPlayerPawnBase", "m_bKilledByHeadshot")]
    killed_by_headshot: bool,
    #[field("C_CSPlayerPawnBase", "m_hOriginalController")]
    original_controller: Handle<PlayerController>,
}

#[derive(Schema)]
#[scope("client.dll")]
pub struct DamageRecord {
    #[field("CDamageRecord", "m_PlayerDamager")]
    player_damager: Handle<PlayerPawnBase>,
    #[field("CDamageRecord", "m_PlayerRecipient")]
    player_recipient: Handle<PlayerPawnBase>,
    #[field("CDamageRecord", "m_hPlayerControllerDamager")]
    player_controller_damager: Handle<PlayerController>,
    #[field("CDamageRecord", "m_hPlayerControllerRecipient")]
    player_controller_recipient: Handle<PlayerController>,
    #[field("CDamageRecord", "m_szPlayerDamagerName")]
    player_damager_name: UtlString,
    #[field("CDamageRecord", "m_szPlayerRecipientName")]
    player_recipient_name: UtlString,
    #[field("CDamageRecord", "m_DamagerXuid")]
    damager_xuid: u64,
    #[field("CDamageRecord", "m_RecipientXuid")]
    recipient_xuid: u64,
    #[field("CDamageRecord", "m_iDamage")]
    damage: i32,
    #[field("CDamageRecord", "m_iActualHealthRemoved")]
    actual_health_removed: i32,
    #[field("CDamageRecord", "m_iNumHits")]
    num_hits: i32,
    #[field("CDamageRecord", "m_iLastBulletUpdate")]
    last_bullet_update: i32,
    #[field("CDamageRecord", "m_bIsOtherEnemy")]
    is_other_enemy: bool,
    #[field("CDamageRecord", "m_killType")]
    kill_type: KillTypes,
}

#[derive(Schema)]
#[scope("client.dll")]
pub struct PlayerControllerDamageServices {
    #[field("CPlayerControllerComponent", "__m_pChainEntity")]
    chain_entity: NetworkVarChainer,
    #[field("CCSPlayerController_DamageServices", "m_nSendUpdate")]
    send_update: i32,
    #[field("CCSPlayerController_DamageServices", "m_DamageList")]
    damage_list: UtlVector<DamageRecord>,
}

#[derive(Schema)]
#[scope("client.dll")]
pub struct ObserverPawn {
    #[field("C_CSObserverPawn", "m_hDetectParentChange")]
    detect_parent_change: EntityHandle,
}

#[derive(Schema)]
#[scope("client.dll")]
pub struct PlayerController {
    #[field("CBasePlayerController", "m_nFinalPredictedTick")]
    final_predicted_tick: i32,
    #[field("CBasePlayerController", "m_CommandContext")]
    command_context: CommandContext,
    #[field("CBasePlayerController", "m_nInButtonsWhichAreToggles")]
    in_buttons_which_are_toggles: u64,
    #[field("CBasePlayerController", "m_nTickBase")]
    tick_base: u32,
    #[field("CBasePlayerController", "m_hPawn")]
    pawn: Handle<BasePlayerPawn>,
    #[field("CBasePlayerController", "m_hPredictedPawn")]
    predicted_pawn: Handle<BasePlayerPawn>,
    #[field("CBasePlayerController", "m_nSplitScreenSlot")]
    split_screen_slot: SplitScreenSlot,
    #[field("CBasePlayerController", "m_hSplitOwner")]
    split_owner: Handle<BasePlayerController>,
    #[field("CBasePlayerController", "m_hSplitScreenPlayers")]
    split_screen_players: UtlVector<Handle<BasePlayerController>>,
    #[field("CBasePlayerController", "m_bIsHLTV")]
    is_hltv: bool,
    #[field("CBasePlayerController", "m_iConnected")]
    connected: PlayerConnectedState,
    #[field("CBasePlayerController", "m_iszPlayerName")]
    player_name: [std::ffi::c_char; 128],
    #[field("CBasePlayerController", "m_steamID")]
    steam_id: u64,
    #[field("CBasePlayerController", "m_bIsLocalPlayerController")]
    is_local_player_controller: bool,
    #[field("CBasePlayerController", "m_iDesiredFOV")]
    desired_fov: u32,
    #[field("CCSPlayerController", "m_pInGameMoneyServices")]
    in_game_money_services: *const PlayerControllerInGameMoneyServices,
    #[field("CCSPlayerController", "m_pInventoryServices")]
    inventory_services: *const PlayerControllerInventoryServices,
    #[field("CCSPlayerController", "m_pActionTrackingServices")]
    action_tracking_services: *const PlayerControllerActionTrackingServices,
    #[field("CCSPlayerController", "m_pDamageServices")]
    damage_services: *const PlayerControllerDamageServices,
    #[field("CCSPlayerController", "m_iPing")]
    ping: u32,
    #[field("CCSPlayerController", "m_bHasCommunicationAbuseMute")]
    has_communication_abuse_mute: bool,
    #[field("CCSPlayerController", "m_szCrosshairCodes")]
    crosshair_codes: UtlSymbolLarge,
    #[field("CCSPlayerController", "m_iPendingTeamNum")]
    pending_team_num: u8,
    #[field("CCSPlayerController", "m_flForceTeamTime")]
    force_team_time: GameTime,
    #[field("CCSPlayerController", "m_iCompTeammateColor")]
    comp_teammate_color: i32,
    #[field("CCSPlayerController", "m_bEverPlayedOnTeam")]
    ever_played_on_team: bool,
    #[field("CCSPlayerController", "m_flPreviousForceJoinTeamTime")]
    previous_force_join_team_time: GameTime,
    #[field("CCSPlayerController", "m_szClan")]
    clan: UtlSymbolLarge,
    #[field("CCSPlayerController", "m_sSanitizedPlayerName")]
    sanitized_player_name: UtlString,
    #[field("CCSPlayerController", "m_iCoachingTeam")]
    coaching_team: i32,
    #[field("CCSPlayerController", "m_nPlayerDominated")]
    player_dominated: u64,
    #[field("CCSPlayerController", "m_nPlayerDominatingMe")]
    player_dominating_me: u64,
    #[field("CCSPlayerController", "m_iCompetitiveRanking")]
    competitive_ranking: i32,
    #[field("CCSPlayerController", "m_iCompetitiveWins")]
    competitive_wins: i32,
    #[field("CCSPlayerController", "m_iCompetitiveRankType")]
    competitive_rank_type: i8,
    #[field("CCSPlayerController", "m_iCompetitiveRankingPredicted_Win")]
    competitive_ranking_predicted_win: i32,
    #[field("CCSPlayerController", "m_iCompetitiveRankingPredicted_Loss")]
    competitive_ranking_predicted_loss: i32,
    #[field("CCSPlayerController", "m_iCompetitiveRankingPredicted_Tie")]
    competitive_ranking_predicted_tie: i32,
    #[field("CCSPlayerController", "m_nEndMatchNextMapVote")]
    end_match_next_map_vote: i32,
    #[field("CCSPlayerController", "m_unActiveQuestId")]
    active_quest_id: u16,
    #[field("CCSPlayerController", "m_nQuestProgressReason")]
    quest_progress_reason: QuestProgressReason,
    #[field("CCSPlayerController", "m_unPlayerTvControlFlags")]
    player_tv_control_flags: u32,
    #[field("CCSPlayerController", "m_iDraftIndex")]
    draft_index: i32,
    #[field("CCSPlayerController", "m_msQueuedModeDisconnectionTimestamp")]
    ms_queued_mode_disconnection_timestamp: u32,
    #[field("CCSPlayerController", "m_uiAbandonRecordedReason")]
    ui_abandon_recorded_reason: u32,
    #[field("CCSPlayerController", "m_bEverFullyConnected")]
    ever_fully_connected: bool,
    #[field("CCSPlayerController", "m_bAbandonAllowsSurrender")]
    abandon_allows_surrender: bool,
    #[field("CCSPlayerController", "m_bAbandonOffersInstantSurrender")]
    abandon_offers_instant_surrender: bool,
    #[field("CCSPlayerController", "m_bDisconnection1MinWarningPrinted")]
    disconnection_1_min_warning_printed: bool,
    #[field("CCSPlayerController", "m_bScoreReported")]
    score_reported: bool,
    #[field("CCSPlayerController", "m_nDisconnectionTick")]
    disconnection_tick: i32,
    #[field("CCSPlayerController", "m_bControllingBot")]
    controlling_bot: bool,
    #[field("CCSPlayerController", "m_bHasControlledBotThisRound")]
    has_controlled_bot_this_round: bool,
    #[field("CCSPlayerController", "m_bHasBeenControlledByPlayerThisRound")]
    has_been_controlled_by_player_this_round: bool,
    #[field("CCSPlayerController", "m_nBotsControlledThisRound")]
    bots_controlled_this_round: i32,
    #[field("CCSPlayerController", "m_bCanControlObservedBot")]
    can_control_observed_bot: bool,
    #[field("CCSPlayerController", "m_hPlayerPawn")]
    player_pawn: Handle<PlayerPawn>,
    #[field("CCSPlayerController", "m_hObserverPawn")]
    observer_pawn: Handle<ObserverPawn>,
    #[field("CCSPlayerController", "m_bPawnIsAlive")]
    pawn_is_alive: bool,
    #[field("CCSPlayerController", "m_iPawnHealth")]
    pawn_health: u32,
    #[field("CCSPlayerController", "m_iPawnArmor")]
    pawn_armor: i32,
    #[field("CCSPlayerController", "m_bPawnHasDefuser")]
    pawn_has_defuser: bool,
    #[field("CCSPlayerController", "m_bPawnHasHelmet")]
    pawn_has_helmet: bool,
    #[field("CCSPlayerController", "m_nPawnCharacterDefIndex")]
    pawn_character_def_index: u16,
    #[field("CCSPlayerController", "m_iPawnLifetimeStart")]
    pawn_lifetime_start: i32,
    #[field("CCSPlayerController", "m_iPawnLifetimeEnd")]
    pawn_lifetime_end: i32,
    #[field("CCSPlayerController", "m_iPawnBotDifficulty")]
    pawn_bot_difficulty: i32,
    #[field("CCSPlayerController", "m_hOriginalControllerOfCurrentPawn")]
    original_controller_of_current_pawn: Handle<PlayerController>,
    #[field("CCSPlayerController", "m_iScore")]
    score: i32,
    #[field("CCSPlayerController", "m_vecKills")]
    kills: UtlVector<KillTypes>,
    #[field("CCSPlayerController", "m_iMVPs")]
    mv_ps: i32,
    #[field("CCSPlayerController", "m_bIsPlayerNameDirty")]
    is_player_name_dirty: bool,
}

#[derive(Schema)]
#[scope("client.dll")]
pub struct PlayerBulletServices {
    #[field("CCSPlayer_BulletServices", "m_totalHitsOnServer")]
    total_hits_on_server: i32,
}

#[derive(Schema)]
#[scope("client.dll")]
pub struct SellbackPurchaseEntry {
    #[field("SellbackPurchaseEntry_t", "m_unDefIdx")]
    def_idx: u16,
    #[field("SellbackPurchaseEntry_t", "m_nCost")]
    cost: i32,
    #[field("SellbackPurchaseEntry_t", "m_nPrevArmor")]
    prev_armor: i32,
    #[field("SellbackPurchaseEntry_t", "m_bPrevHelmet")]
    prev_helmet: bool,
    #[field("SellbackPurchaseEntry_t", "m_hItem")]
    item: EntityHandle,
}

#[derive(Schema)]
#[scope("client.dll")]
pub struct PlayerBuyServices {
    #[field("CCSPlayer_BuyServices", "m_vecSellbackPurchaseEntries")]
    sellback_purchase_entries: UtlVector<SellbackPurchaseEntry>,
}

#[derive(Schema)]
#[scope("client.dll")]
pub struct PlayerHostageServices {
    #[field("CCSPlayer_HostageServices", "m_hCarriedHostage")]
    carried_hostage: Handle<BaseEntity>,
    #[field("CCSPlayer_HostageServices", "m_hCarriedHostageProp")]
    carried_hostage_prop: Handle<BaseEntity>,
}

#[derive(Schema)]
#[scope("client.dll")]
pub struct WeaponPurchaseCount {
    #[field("WeaponPurchaseCount_t", "m_nItemDefIndex")]
    item_def_index: u16,
    #[field("WeaponPurchaseCount_t", "m_nCount")]
    count: u16,
}

#[derive(Schema)]
#[scope("client.dll")]
pub struct WeaponPurchaseTracker {
    #[field("WeaponPurchaseTracker_t", "m_weaponPurchases")]
    weapon_purchases: UtlVector<WeaponPurchaseCount>,
}

#[derive(Schema)]
#[scope("client.dll")]
pub struct PlayerActionTrackingServices {
    #[field("CCSPlayer_ActionTrackingServices", "m_hLastWeaponBeforeC4AutoSwitch")]
    last_weapon_before_c_4_auto_switch: Handle<BasePlayerWeapon>,
    #[field("CCSPlayer_ActionTrackingServices", "m_bIsRescuing")]
    is_rescuing: bool,
    #[field("CCSPlayer_ActionTrackingServices", "m_weaponPurchasesThisMatch")]
    weapon_purchases_this_match: WeaponPurchaseTracker,
    #[field("CCSPlayer_ActionTrackingServices", "m_weaponPurchasesThisRound")]
    weapon_purchases_this_round: WeaponPurchaseTracker,
}

#[derive(Schema)]
#[scope("client.dll")]
pub struct PlayerGlowServices {}

#[derive(Schema)]
#[scope("client.dll")]
pub struct PlayerPawn {
    #[field("CEntityInstance", "m_iszPrivateVScripts")]
    private_v_scripts: UtlSymbolLarge,
    #[field("CEntityInstance", "m_pEntity")]
    entity: *const EntityIdentity,
    #[field("CEntityInstance", "m_CScriptComponent")]
    script_component: *const ScriptComponent,
    #[field("C_BaseEntity", "m_CBodyComponent")]
    body_component: *const BodyComponent,
    #[field("C_BaseEntity", "m_NetworkTransmitComponent")]
    network_transmit_component: NetworkTransmitComponent,
    #[field("C_BaseEntity", "m_nLastThinkTick")]
    last_think_tick: GameTick,
    #[field("C_BaseEntity", "m_pGameSceneNode")]
    game_scene_node: *const GameSceneNode,
    #[field("C_BaseEntity", "m_pRenderComponent")]
    p_render_component: *const RenderComponent,
    #[field("C_BaseEntity", "m_pCollision")]
    p_collision: *const CollisionProperty,
    #[field("C_BaseEntity", "m_iMaxHealth")]
    max_health: i32,
    #[field("C_BaseEntity", "m_iHealth")]
    health: i32,
    #[field("C_BaseEntity", "m_lifeState")]
    life_state: u8,
    #[field("C_BaseEntity", "m_bTakesDamage")]
    takes_damage: bool,
    #[field("C_BaseEntity", "m_nTakeDamageFlags")]
    take_damage_flags: TakeDamageFlags,
    #[field("C_BaseEntity", "m_ubInterpolationFrame")]
    interpolation_frame: u8,
    #[field("C_BaseEntity", "m_hSceneObjectController")]
    scene_object_controller: Handle<BaseEntity>,
    #[field("C_BaseEntity", "m_nNoInterpolationTick")]
    no_interpolation_tick: i32,
    #[field("C_BaseEntity", "m_nVisibilityNoInterpolationTick")]
    visibility_no_interpolation_tick: i32,
    #[field("C_BaseEntity", "m_flProxyRandomValue")]
    proxy_random_value: f32,
    #[field("C_BaseEntity", "m_iEFlags")]
    e_flags: i32,
    #[field("C_BaseEntity", "m_nWaterType")]
    water_type: u8,
    #[field("C_BaseEntity", "m_bInterpolateEvenWithNoModel")]
    interpolate_even_with_no_model: bool,
    #[field("C_BaseEntity", "m_bPredictionEligible")]
    prediction_eligible: bool,
    #[field("C_BaseEntity", "m_bApplyLayerMatchIDToModel")]
    apply_layer_match_id_to_model: bool,
    #[field("C_BaseEntity", "m_tokLayerMatchID")]
    tok_layer_match_id: UtlStringToken,
    #[field("C_BaseEntity", "m_nSubclassID")]
    subclass_id: UtlStringToken,
    #[field("C_BaseEntity", "m_nSimulationTick")]
    simulation_tick: i32,
    #[field("C_BaseEntity", "m_iCurrentThinkContext")]
    current_think_context: i32,
    #[field("C_BaseEntity", "m_aThinkFunctions")]
    think_functions: UtlVector<Thinkfunc>,
    #[field("C_BaseEntity", "m_flAnimTime")]
    anim_time: f32,
    #[field("C_BaseEntity", "m_flSimulationTime")]
    simulation_time: f32,
    #[field("C_BaseEntity", "m_nSceneObjectOverrideFlags")]
    scene_object_override_flags: u8,
    #[field("C_BaseEntity", "m_bHasSuccessfullyInterpolated")]
    has_successfully_interpolated: bool,
    #[field("C_BaseEntity", "m_bHasAddedVarsToInterpolation")]
    has_added_vars_to_interpolation: bool,
    #[field("C_BaseEntity", "m_bRenderEvenWhenNotSuccessfullyInterpolated")]
    render_even_when_not_successfully_interpolated: bool,
    #[field("C_BaseEntity", "m_nInterpolationLatchDirtyFlags")]
    interpolation_latch_dirty_flags: [i32; 2],
    #[field("C_BaseEntity", "m_ListEntry")]
    list_entry: [u16; 11],
    #[field("C_BaseEntity", "m_flCreateTime")]
    create_time: GameTime,
    #[field("C_BaseEntity", "m_flSpeed")]
    speed: f32,
    #[field("C_BaseEntity", "m_EntClientFlags")]
    ent_client_flags: u16,
    #[field("C_BaseEntity", "m_bClientSideRagdoll")]
    client_side_ragdoll: bool,
    #[field("C_BaseEntity", "m_iTeamNum")]
    team_num: u8,
    #[field("C_BaseEntity", "m_spawnflags")]
    spawnflags: u32,
    #[field("C_BaseEntity", "m_nNextThinkTick")]
    next_think_tick: GameTick,
    #[field("C_BaseEntity", "m_fFlags")]
    flags: u32,
    #[field("C_BaseEntity", "m_vecAbsVelocity")]
    abs_velocity: Vector,
    #[field("C_BaseEntity", "m_vecVelocity")]
    velocity: NetworkVelocityVector,
    #[field("C_BaseEntity", "m_vecBaseVelocity")]
    base_velocity: Vector,
    #[field("C_BaseEntity", "m_hEffectEntity")]
    effect_entity: Handle<BaseEntity>,
    #[field("C_BaseEntity", "m_hOwnerEntity")]
    owner_entity: Handle<BaseEntity>,
    #[field("C_BaseEntity", "m_MoveCollide")]
    move_collide: MoveCollide,
    #[field("C_BaseEntity", "m_MoveType")]
    move_type: MoveType,
    #[field("C_BaseEntity", "m_flWaterLevel")]
    water_level: f32,
    #[field("C_BaseEntity", "m_fEffects")]
    effects: u32,
    #[field("C_BaseEntity", "m_hGroundEntity")]
    ground_entity: Handle<BaseEntity>,
    #[field("C_BaseEntity", "m_flFriction")]
    friction: f32,
    #[field("C_BaseEntity", "m_flElasticity")]
    elasticity: f32,
    #[field("C_BaseEntity", "m_flGravityScale")]
    gravity_scale: f32,
    #[field("C_BaseEntity", "m_flTimeScale")]
    time_scale: f32,
    #[field("C_BaseEntity", "m_bSimulatedEveryTick")]
    simulated_every_tick: bool,
    #[field("C_BaseEntity", "m_bAnimatedEveryTick")]
    animated_every_tick: bool,
    #[field("C_BaseEntity", "m_flNavIgnoreUntilTime")]
    nav_ignore_until_time: GameTime,
    #[field("C_BaseEntity", "m_hThink")]
    think: u16,
    #[field("C_BaseEntity", "m_fBBoxVisFlags")]
    b_box_vis_flags: u8,
    #[field("C_BaseEntity", "m_bPredictable")]
    predictable: bool,
    #[field("C_BaseEntity", "m_bRenderWithViewModels")]
    render_with_view_models: bool,
    #[field("C_BaseEntity", "m_nSplitUserPlayerPredictionSlot")]
    split_user_player_prediction_slot: SplitScreenSlot,
    #[field("C_BaseEntity", "m_nFirstPredictableCommand")]
    first_predictable_command: i32,
    #[field("C_BaseEntity", "m_nLastPredictableCommand")]
    last_predictable_command: i32,
    #[field("C_BaseEntity", "m_hOldMoveParent")]
    old_move_parent: Handle<BaseEntity>,
    #[field("C_BaseEntity", "m_Particles")]
    particles: ParticleProperty,
    #[field("C_BaseEntity", "m_vecPredictedScriptFloats")]
    predicted_script_floats: UtlVector<f32>,
    #[field("C_BaseEntity", "m_vecPredictedScriptFloatIDs")]
    predicted_script_float_i_ds: UtlVector<i32>,
    #[field("C_BaseEntity", "m_nNextScriptVarRecordID")]
    next_script_var_record_id: i32,
    #[field("C_BaseEntity", "m_vecAngVelocity")]
    ang_velocity: QAngle,
    #[field("C_BaseEntity", "m_DataChangeEventRef")]
    data_change_event_ref: i32,
    #[field("C_BaseEntity", "m_dependencies")]
    dependencies: UtlVector<EntityHandle>,
    #[field("C_BaseEntity", "m_nCreationTick")]
    creation_tick: i32,
    #[field("C_BaseEntity", "m_bAnimTimeChanged")]
    anim_time_changed: bool,
    #[field("C_BaseEntity", "m_bSimulationTimeChanged")]
    simulation_time_changed: bool,
    #[field("C_BaseEntity", "m_sUniqueHammerID")]
    unique_hammer_id: UtlString,
    #[field("C_BaseModelEntity", "m_CRenderComponent")]
    c_render_component: *const RenderComponent,
    #[field("C_BaseModelEntity", "m_CHitboxComponent")]
    hitbox_component: HitboxComponent,
    #[field("C_BaseModelEntity", "m_bInitModelEffects")]
    init_model_effects: bool,
    #[field("C_BaseModelEntity", "m_bIsStaticProp")]
    is_static_prop: bool,
    #[field("C_BaseModelEntity", "m_nLastAddDecal")]
    last_add_decal: i32,
    #[field("C_BaseModelEntity", "m_nDecalsAdded")]
    decals_added: i32,
    #[field("C_BaseModelEntity", "m_iOldHealth")]
    old_health: i32,
    #[field("C_BaseModelEntity", "m_nRenderMode")]
    render_mode: RenderMode,
    #[field("C_BaseModelEntity", "m_nRenderFX")]
    render_fx: RenderFx,
    #[field("C_BaseModelEntity", "m_bAllowFadeInView")]
    allow_fade_in_view: bool,
    #[field("C_BaseModelEntity", "m_clrRender")]
    render: Color,
    #[field("C_BaseModelEntity", "m_vecRenderAttributes")]
    render_attributes: UtlVector<EntityRenderAttribute>,
    #[field("C_BaseModelEntity", "m_LightGroup")]
    light_group: UtlStringToken,
    #[field("C_BaseModelEntity", "m_bRenderToCubemaps")]
    render_to_cubemaps: bool,
    #[field("C_BaseModelEntity", "m_Collision")]
    collision: CollisionProperty,
    #[field("C_BaseModelEntity", "m_Glow")]
    glow: GlowProperty,
    #[field("C_BaseModelEntity", "m_flGlowBackfaceMult")]
    glow_backface_mult: f32,
    #[field("C_BaseModelEntity", "m_fadeMinDist")]
    fade_min_dist: f32,
    #[field("C_BaseModelEntity", "m_fadeMaxDist")]
    fade_max_dist: f32,
    #[field("C_BaseModelEntity", "m_flFadeScale")]
    fade_scale: f32,
    #[field("C_BaseModelEntity", "m_flShadowStrength")]
    shadow_strength: f32,
    #[field("C_BaseModelEntity", "m_nObjectCulling")]
    object_culling: u8,
    #[field("C_BaseModelEntity", "m_nAddDecal")]
    add_decal: i32,
    #[field("C_BaseModelEntity", "m_vDecalPosition")]
    decal_position: Vector,
    #[field("C_BaseModelEntity", "m_vDecalForwardAxis")]
    decal_forward_axis: Vector,
    #[field("C_BaseModelEntity", "m_flDecalHealBloodRate")]
    decal_heal_blood_rate: f32,
    #[field("C_BaseModelEntity", "m_flDecalHealHeightRate")]
    decal_heal_height_rate: f32,
    #[field("C_BaseModelEntity", "m_ConfigEntitiesToPropagateMaterialDecalsTo")]
    config_entities_to_propagate_material_decals_to: UtlVector<Handle<BaseModelEntity>>,
    #[field("C_BaseModelEntity", "m_vecViewOffset")]
    view_offset: NetworkViewOffsetVector,
    #[field("C_BaseModelEntity", "m_pClientAlphaProperty")]
    client_alpha_property: *const ClientAlphaProperty,
    #[field("C_BaseModelEntity", "m_ClientOverrideTint")]
    client_override_tint: Color,
    #[field("C_BaseModelEntity", "m_bUseClientOverrideTint")]
    use_client_override_tint: bool,
    #[field("CBaseAnimGraph", "m_bInitiallyPopulateInterpHistory")]
    initially_populate_interp_history: bool,
    #[field("CBaseAnimGraph", "m_bShouldAnimateDuringGameplayPause")]
    should_animate_during_gameplay_pause: bool,
    #[field("CBaseAnimGraph", "m_bSuppressAnimEventSounds")]
    suppress_anim_event_sounds: bool,
    #[field("CBaseAnimGraph", "m_bAnimGraphUpdateEnabled")]
    anim_graph_update_enabled: bool,
    #[field("CBaseAnimGraph", "m_flMaxSlopeDistance")]
    max_slope_distance: f32,
    #[field("CBaseAnimGraph", "m_vLastSlopeCheckPos")]
    last_slope_check_pos: Vector,
    #[field("CBaseAnimGraph", "m_vecForce")]
    force: Vector,
    #[field("CBaseAnimGraph", "m_nForceBone")]
    force_bone: i32,
    #[field("CBaseAnimGraph", "m_pClientsideRagdoll")]
    clientside_ragdoll: *const BaseAnimGraph,
    #[field("CBaseAnimGraph", "m_bBuiltRagdoll")]
    built_ragdoll: bool,
    #[field("CBaseAnimGraph", "m_pRagdollPose")]
    ragdoll_pose: *const PhysicsRagdollPose,
    #[field("CBaseAnimGraph", "m_bClientRagdoll")]
    client_ragdoll: bool,
    #[field("CBaseAnimGraph", "m_bHasAnimatedMaterialAttributes")]
    has_animated_material_attributes: bool,
    #[field("C_BaseFlex", "m_flexWeight")]
    flex_weight: UtlVector<f32>,
    #[field("C_BaseFlex", "m_vLookTargetPosition")]
    look_target_position: Vector,
    #[field("C_BaseFlex", "m_blinktoggle")]
    blinktoggle: bool,
    #[field("C_BaseFlex", "m_nLastFlexUpdateFrameCount")]
    last_flex_update_frame_count: i32,
    #[field("C_BaseFlex", "m_CachedViewTarget")]
    cached_view_target: Vector,
    #[field("C_BaseFlex", "m_nNextSceneEventId")]
    next_scene_event_id: u32,
    #[field("C_BaseFlex", "m_iBlink")]
    blink: i32,
    #[field("C_BaseFlex", "m_blinktime")]
    blinktime: f32,
    #[field("C_BaseFlex", "m_prevblinktoggle")]
    prevblinktoggle: bool,
    #[field("C_BaseFlex", "m_iJawOpen")]
    jaw_open: i32,
    #[field("C_BaseFlex", "m_flJawOpenAmount")]
    jaw_open_amount: f32,
    #[field("C_BaseFlex", "m_flBlinkAmount")]
    blink_amount: f32,
    #[field("C_BaseFlex", "m_iMouthAttachment")]
    mouth_attachment: AttachmentHandle,
    #[field("C_BaseFlex", "m_iEyeAttachment")]
    eye_attachment: AttachmentHandle,
    #[field("C_BaseFlex", "m_bResetFlexWeightsOnModelChange")]
    reset_flex_weights_on_model_change: bool,
    #[field("C_BaseFlex", "m_nEyeOcclusionRendererBone")]
    eye_occlusion_renderer_bone: i32,
    #[field("C_BaseFlex", "m_mEyeOcclusionRendererCameraToBoneTransform")]
    m_eye_occlusion_renderer_camera_to_bone_transform: Matrix3X4,
    #[field("C_BaseFlex", "m_vEyeOcclusionRendererHalfExtent")]
    eye_occlusion_renderer_half_extent: Vector,
    #[field("C_BaseFlex", "m_PhonemeClasses")]
    phoneme_classes: [BaseFlexEmphasizedPhoneme; 3],
    #[field("C_BaseCombatCharacter", "m_hMyWearables")]
    my_wearables: UtlVector<Handle<EconWearable>>,
    #[field("C_BaseCombatCharacter", "m_bloodColor")]
    blood_color: i32,
    #[field("C_BaseCombatCharacter", "m_leftFootAttachment")]
    left_foot_attachment: AttachmentHandle,
    #[field("C_BaseCombatCharacter", "m_rightFootAttachment")]
    right_foot_attachment: AttachmentHandle,
    #[field("C_BaseCombatCharacter", "m_nWaterWakeMode")]
    water_wake_mode: WaterWakeMode,
    #[field("C_BaseCombatCharacter", "m_flWaterWorldZ")]
    water_world_z: f32,
    #[field("C_BaseCombatCharacter", "m_flWaterNextTraceTime")]
    water_next_trace_time: f32,
    #[field("C_BaseCombatCharacter", "m_flFieldOfView")]
    field_of_view: f32,
    #[field("C_BasePlayerPawn", "m_pWeaponServices")]
    weapon_services: *const PlayerWeaponServices,
    #[field("C_BasePlayerPawn", "m_pItemServices")]
    item_services: *const PlayerItemServices,
    #[field("C_BasePlayerPawn", "m_pAutoaimServices")]
    autoaim_services: *const PlayerAutoaimServices,
    #[field("C_BasePlayerPawn", "m_pObserverServices")]
    observer_services: *const PlayerObserverServices,
    #[field("C_BasePlayerPawn", "m_pWaterServices")]
    water_services: *const PlayerWaterServices,
    #[field("C_BasePlayerPawn", "m_pUseServices")]
    use_services: *const PlayerUseServices,
    #[field("C_BasePlayerPawn", "m_pFlashlightServices")]
    flashlight_services: *const PlayerFlashlightServices,
    #[field("C_BasePlayerPawn", "m_pCameraServices")]
    camera_services: *const PlayerCameraServices,
    #[field("C_BasePlayerPawn", "m_pMovementServices")]
    movement_services: *const PlayerMovementServices,
    #[field("C_BasePlayerPawn", "m_ServerViewAngleChanges")]
    server_view_angle_changes: UtlVector<ViewAngleServerChange>,
    #[field("C_BasePlayerPawn", "m_nHighestConsumedServerViewAngleChangeIndex")]
    highest_consumed_server_view_angle_change_index: u32,
    #[field("C_BasePlayerPawn", "v_angle")]
    v_angle: QAngle,
    #[field("C_BasePlayerPawn", "v_anglePrevious")]
    v_angle_previous: QAngle,
    #[field("C_BasePlayerPawn", "m_iHideHUD")]
    hide_hud: u32,
    #[field("C_BasePlayerPawn", "m_skybox3d")]
    skybox_3_d: Sky3Dparams,
    #[field("C_BasePlayerPawn", "m_flDeathTime")]
    death_time: GameTime,
    #[field("C_BasePlayerPawn", "m_vecPredictionError")]
    prediction_error: Vector,
    #[field("C_BasePlayerPawn", "m_flPredictionErrorTime")]
    prediction_error_time: GameTime,
    #[field("C_BasePlayerPawn", "m_flFOVSensitivityAdjust")]
    fov_sensitivity_adjust: f32,
    #[field("C_BasePlayerPawn", "m_flMouseSensitivity")]
    mouse_sensitivity: f32,
    #[field("C_BasePlayerPawn", "m_vOldOrigin")]
    old_origin: Vector,
    #[field("C_BasePlayerPawn", "m_flOldSimulationTime")]
    old_simulation_time: f32,
    #[field("C_BasePlayerPawn", "m_nLastExecutedCommandNumber")]
    last_executed_command_number: i32,
    #[field("C_BasePlayerPawn", "m_nLastExecutedCommandTick")]
    last_executed_command_tick: i32,
    #[field("C_BasePlayerPawn", "m_hController")]
    controller: Handle<BasePlayerController>,
    #[field("C_BasePlayerPawn", "m_bIsSwappingToPredictableController")]
    is_swapping_to_predictable_controller: bool,
    #[field("C_CSPlayerPawnBase", "m_pPingServices")]
    ping_services: *const PlayerPingServices,
    #[field("C_CSPlayerPawnBase", "m_pViewModelServices")]
    view_model_services: *const PlayerViewModelServices,
    #[field("C_CSPlayerPawnBase", "m_fRenderingClipPlane")]
    rendering_clip_plane: [f32; 4],
    #[field("C_CSPlayerPawnBase", "m_nLastClipPlaneSetupFrame")]
    last_clip_plane_setup_frame: i32,
    #[field("C_CSPlayerPawnBase", "m_vecLastClipCameraPos")]
    last_clip_camera_pos: Vector,
    #[field("C_CSPlayerPawnBase", "m_vecLastClipCameraForward")]
    last_clip_camera_forward: Vector,
    #[field("C_CSPlayerPawnBase", "m_bClipHitStaticWorld")]
    clip_hit_static_world: bool,
    #[field("C_CSPlayerPawnBase", "m_bCachedPlaneIsValid")]
    cached_plane_is_valid: bool,
    #[field("C_CSPlayerPawnBase", "m_pClippingWeapon")]
    clipping_weapon: *const WeaponBase,
    #[field("C_CSPlayerPawnBase", "m_previousPlayerState")]
    previous_player_state: CsPlayerState,
    #[field("C_CSPlayerPawnBase", "m_flLastCollisionCeiling")]
    last_collision_ceiling: f32,
    #[field("C_CSPlayerPawnBase", "m_flLastCollisionCeilingChangeTime")]
    last_collision_ceiling_change_time: f32,
    #[field("C_CSPlayerPawnBase", "m_grenadeParameterStashTime")]
    grenade_parameter_stash_time: GameTime,
    #[field("C_CSPlayerPawnBase", "m_bGrenadeParametersStashed")]
    grenade_parameters_stashed: bool,
    #[field("C_CSPlayerPawnBase", "m_angStashedShootAngles")]
    stashed_shoot_angles: QAngle,
    #[field("C_CSPlayerPawnBase", "m_vecStashedGrenadeThrowPosition")]
    stashed_grenade_throw_position: Vector,
    #[field("C_CSPlayerPawnBase", "m_vecStashedVelocity")]
    stashed_velocity: Vector,
    #[field("C_CSPlayerPawnBase", "m_angShootAngleHistory")]
    shoot_angle_history: [QAngle; 2],
    #[field("C_CSPlayerPawnBase", "m_vecThrowPositionHistory")]
    throw_position_history: [Vector; 2],
    #[field("C_CSPlayerPawnBase", "m_vecVelocityHistory")]
    velocity_history: [Vector; 2],
    #[field("C_CSPlayerPawnBase", "m_thirdPersonHeading")]
    third_person_heading: QAngle,
    #[field("C_CSPlayerPawnBase", "m_flSlopeDropOffset")]
    slope_drop_offset: f32,
    #[field("C_CSPlayerPawnBase", "m_flSlopeDropHeight")]
    slope_drop_height: f32,
    #[field("C_CSPlayerPawnBase", "m_vHeadConstraintOffset")]
    head_constraint_offset: Vector,
    #[field("C_CSPlayerPawnBase", "m_bIsScoped")]
    is_scoped: bool,
    #[field("C_CSPlayerPawnBase", "m_bIsWalking")]
    is_walking: bool,
    #[field("C_CSPlayerPawnBase", "m_bResumeZoom")]
    resume_zoom: bool,
    #[field("C_CSPlayerPawnBase", "m_iPlayerState")]
    player_state: CsPlayerState,
    #[field("C_CSPlayerPawnBase", "m_bIsDefusing")]
    is_defusing: bool,
    #[field("C_CSPlayerPawnBase", "m_bIsGrabbingHostage")]
    is_grabbing_hostage: bool,
    #[field("C_CSPlayerPawnBase", "m_iBlockingUseActionInProgress")]
    blocking_use_action_in_progress: CsPlayerBlockingUseAction,
    #[field("C_CSPlayerPawnBase", "m_bIsRescuing")]
    is_rescuing: bool,
    #[field("C_CSPlayerPawnBase", "m_fImmuneToGunGameDamageTime")]
    immune_to_gun_game_damage_time: GameTime,
    #[field("C_CSPlayerPawnBase", "m_fImmuneToGunGameDamageTimeLast")]
    immune_to_gun_game_damage_time_last: GameTime,
    #[field("C_CSPlayerPawnBase", "m_bGunGameImmunity")]
    gun_game_immunity: bool,
    #[field("C_CSPlayerPawnBase", "m_bHasMovedSinceSpawn")]
    has_moved_since_spawn: bool,
    #[field("C_CSPlayerPawnBase", "m_fMolotovUseTime")]
    molotov_use_time: f32,
    #[field("C_CSPlayerPawnBase", "m_fMolotovDamageTime")]
    molotov_damage_time: f32,
    #[field("C_CSPlayerPawnBase", "m_nWhichBombZone")]
    which_bomb_zone: i32,
    #[field("C_CSPlayerPawnBase", "m_bInNoDefuseArea")]
    in_no_defuse_area: bool,
    #[field("C_CSPlayerPawnBase", "m_iThrowGrenadeCounter")]
    throw_grenade_counter: i32,
    #[field("C_CSPlayerPawnBase", "m_bWaitForNoAttack")]
    wait_for_no_attack: bool,
    #[field("C_CSPlayerPawnBase", "m_flGuardianTooFarDistFrac")]
    guardian_too_far_dist_frac: f32,
    #[field("C_CSPlayerPawnBase", "m_flDetectedByEnemySensorTime")]
    detected_by_enemy_sensor_time: GameTime,
    #[field("C_CSPlayerPawnBase", "m_flNextGuardianTooFarWarning")]
    next_guardian_too_far_warning: f32,
    #[field("C_CSPlayerPawnBase", "m_bSuppressGuardianTooFarWarningAudio")]
    suppress_guardian_too_far_warning_audio: bool,
    #[field("C_CSPlayerPawnBase", "m_bKilledByTaser")]
    killed_by_taser: bool,
    #[field("C_CSPlayerPawnBase", "m_iMoveState")]
    move_state: i32,
    #[field("C_CSPlayerPawnBase", "m_bCanMoveDuringFreezePeriod")]
    can_move_during_freeze_period: bool,
    #[field("C_CSPlayerPawnBase", "m_flLowerBodyYawTarget")]
    lower_body_yaw_target: f32,
    #[field("C_CSPlayerPawnBase", "m_bStrafing")]
    strafing: bool,
    #[field("C_CSPlayerPawnBase", "m_flLastSpawnTimeIndex")]
    last_spawn_time_index: GameTime,
    #[field("C_CSPlayerPawnBase", "m_flEmitSoundTime")]
    emit_sound_time: GameTime,
    #[field("C_CSPlayerPawnBase", "m_iAddonBits")]
    addon_bits: i32,
    #[field("C_CSPlayerPawnBase", "m_iPrimaryAddon")]
    primary_addon: i32,
    #[field("C_CSPlayerPawnBase", "m_iSecondaryAddon")]
    secondary_addon: i32,
    #[field("C_CSPlayerPawnBase", "m_iProgressBarDuration")]
    progress_bar_duration: i32,
    #[field("C_CSPlayerPawnBase", "m_flProgressBarStartTime")]
    progress_bar_start_time: f32,
    #[field("C_CSPlayerPawnBase", "m_iDirection")]
    direction: i32,
    #[field("C_CSPlayerPawnBase", "m_iShotsFired")]
    shots_fired: i32,
    #[field("C_CSPlayerPawnBase", "m_bNightVisionOn")]
    night_vision_on: bool,
    #[field("C_CSPlayerPawnBase", "m_bHasNightVision")]
    has_night_vision: bool,
    #[field("C_CSPlayerPawnBase", "m_flVelocityModifier")]
    velocity_modifier: f32,
    #[field("C_CSPlayerPawnBase", "m_flHitHeading")]
    hit_heading: f32,
    #[field("C_CSPlayerPawnBase", "m_nHitBodyPart")]
    hit_body_part: i32,
    #[field("C_CSPlayerPawnBase", "m_iStartAccount")]
    start_account: i32,
    #[field("C_CSPlayerPawnBase", "m_vecIntroStartEyePosition")]
    intro_start_eye_position: Vector,
    #[field("C_CSPlayerPawnBase", "m_vecIntroStartPlayerForward")]
    intro_start_player_forward: Vector,
    #[field("C_CSPlayerPawnBase", "m_flClientDeathTime")]
    client_death_time: GameTime,
    #[field("C_CSPlayerPawnBase", "m_flNightVisionAlpha")]
    night_vision_alpha: f32,
    #[field("C_CSPlayerPawnBase", "m_bScreenTearFrameCaptured")]
    screen_tear_frame_captured: bool,
    #[field("C_CSPlayerPawnBase", "m_flFlashBangTime")]
    flash_bang_time: f32,
    #[field("C_CSPlayerPawnBase", "m_flFlashScreenshotAlpha")]
    flash_screenshot_alpha: f32,
    #[field("C_CSPlayerPawnBase", "m_flFlashOverlayAlpha")]
    flash_overlay_alpha: f32,
    #[field("C_CSPlayerPawnBase", "m_bFlashBuildUp")]
    flash_build_up: bool,
    #[field("C_CSPlayerPawnBase", "m_bFlashDspHasBeenCleared")]
    flash_dsp_has_been_cleared: bool,
    #[field("C_CSPlayerPawnBase", "m_bFlashScreenshotHasBeenGrabbed")]
    flash_screenshot_has_been_grabbed: bool,
    #[field("C_CSPlayerPawnBase", "m_flFlashMaxAlpha")]
    flash_max_alpha: f32,
    #[field("C_CSPlayerPawnBase", "m_flFlashDuration")]
    flash_duration: f32,
    #[field("C_CSPlayerPawnBase", "m_lastStandingPos")]
    last_standing_pos: Vector,
    #[field("C_CSPlayerPawnBase", "m_vecLastMuzzleFlashPos")]
    last_muzzle_flash_pos: Vector,
    #[field("C_CSPlayerPawnBase", "m_angLastMuzzleFlashAngle")]
    last_muzzle_flash_angle: QAngle,
    #[field("C_CSPlayerPawnBase", "m_hMuzzleFlashShape")]
    muzzle_flash_shape: Handle<BaseEntity>,
    #[field("C_CSPlayerPawnBase", "m_iHealthBarRenderMaskIndex")]
    health_bar_render_mask_index: i32,
    #[field("C_CSPlayerPawnBase", "m_flHealthFadeValue")]
    health_fade_value: f32,
    #[field("C_CSPlayerPawnBase", "m_flHealthFadeAlpha")]
    health_fade_alpha: f32,
    #[field("C_CSPlayerPawnBase", "m_nMyCollisionGroup")]
    my_collision_group: i32,
    #[field("C_CSPlayerPawnBase", "m_ignoreLadderJumpTime")]
    ignore_ladder_jump_time: f32,
    #[field("C_CSPlayerPawnBase", "m_ladderSurpressionTimer")]
    ladder_surpression_timer: CountdownTimer,
    #[field("C_CSPlayerPawnBase", "m_lastLadderNormal")]
    last_ladder_normal: Vector,
    #[field("C_CSPlayerPawnBase", "m_lastLadderPos")]
    last_ladder_pos: Vector,
    #[field("C_CSPlayerPawnBase", "m_flDeathCCWeight")]
    death_cc_weight: f32,
    #[field("C_CSPlayerPawnBase", "m_bOldIsScoped")]
    old_is_scoped: bool,
    #[field("C_CSPlayerPawnBase", "m_flPrevRoundEndTime")]
    prev_round_end_time: f32,
    #[field("C_CSPlayerPawnBase", "m_flPrevMatchEndTime")]
    prev_match_end_time: f32,
    #[field("C_CSPlayerPawnBase", "m_unCurrentEquipmentValue")]
    current_equipment_value: u16,
    #[field("C_CSPlayerPawnBase", "m_unRoundStartEquipmentValue")]
    round_start_equipment_value: u16,
    #[field("C_CSPlayerPawnBase", "m_unFreezetimeEndEquipmentValue")]
    freezetime_end_equipment_value: u16,
    #[field("C_CSPlayerPawnBase", "m_vecThirdPersonViewPositionOverride")]
    third_person_view_position_override: Vector,
    #[field("C_CSPlayerPawnBase", "m_nHeavyAssaultSuitCooldownRemaining")]
    heavy_assault_suit_cooldown_remaining: i32,
    #[field("C_CSPlayerPawnBase", "m_ArmorValue")]
    armor_value: i32,
    #[field("C_CSPlayerPawnBase", "m_angEyeAngles")]
    eye_angles: QAngle,
    #[field("C_CSPlayerPawnBase", "m_fNextThinkPushAway")]
    next_think_push_away: f32,
    #[field("C_CSPlayerPawnBase", "m_bShouldAutobuyDMWeapons")]
    should_autobuy_dm_weapons: bool,
    #[field("C_CSPlayerPawnBase", "m_bShouldAutobuyNow")]
    should_autobuy_now: bool,
    #[field("C_CSPlayerPawnBase", "m_bHud_MiniScoreHidden")]
    hud_mini_score_hidden: bool,
    #[field("C_CSPlayerPawnBase", "m_bHud_RadarHidden")]
    hud_radar_hidden: bool,
    #[field("C_CSPlayerPawnBase", "m_nLastKillerIndex")]
    last_killer_index: EntityIndex,
    #[field("C_CSPlayerPawnBase", "m_nLastConcurrentKilled")]
    last_concurrent_killed: i32,
    #[field("C_CSPlayerPawnBase", "m_nDeathCamMusic")]
    death_cam_music: i32,
    #[field("C_CSPlayerPawnBase", "m_iIDEntIndex")]
    id_ent_index: EntityIndex,
    #[field("C_CSPlayerPawnBase", "m_delayTargetIDTimer")]
    delay_target_id_timer: CountdownTimer,
    #[field("C_CSPlayerPawnBase", "m_iTargetedWeaponEntIndex")]
    targeted_weapon_ent_index: EntityIndex,
    #[field("C_CSPlayerPawnBase", "m_iOldIDEntIndex")]
    old_id_ent_index: EntityIndex,
    #[field("C_CSPlayerPawnBase", "m_holdTargetIDTimer")]
    hold_target_id_timer: CountdownTimer,
    #[field("C_CSPlayerPawnBase", "m_flCurrentMusicStartTime")]
    current_music_start_time: f32,
    #[field("C_CSPlayerPawnBase", "m_flMusicRoundStartTime")]
    music_round_start_time: f32,
    #[field("C_CSPlayerPawnBase", "m_bDeferStartMusicOnWarmup")]
    defer_start_music_on_warmup: bool,
    #[field("C_CSPlayerPawnBase", "m_cycleLatch")]
    cycle_latch: i32,
    #[field("C_CSPlayerPawnBase", "m_serverIntendedCycle")]
    server_intended_cycle: f32,
    #[field("C_CSPlayerPawnBase", "m_vecPlayerPatchEconIndices")]
    player_patch_econ_indices: [u32; 5],
    #[field("C_CSPlayerPawnBase", "m_bHideTargetID")]
    hide_target_id: bool,
    #[field("C_CSPlayerPawnBase", "m_nextTaserShakeTime")]
    next_taser_shake_time: f32,
    #[field("C_CSPlayerPawnBase", "m_firstTaserShakeTime")]
    first_taser_shake_time: f32,
    #[field("C_CSPlayerPawnBase", "m_flLastSmokeOverlayAlpha")]
    last_smoke_overlay_alpha: f32,
    #[field("C_CSPlayerPawnBase", "m_vLastSmokeOverlayColor")]
    last_smoke_overlay_color: Vector,
    #[field("C_CSPlayerPawnBase", "m_nPlayerSmokedFx")]
    player_smoked_fx: ParticleIndex,
    #[field("C_CSPlayerPawnBase", "m_flNextMagDropTime")]
    next_mag_drop_time: f32,
    #[field("C_CSPlayerPawnBase", "m_nLastMagDropAttachmentIndex")]
    last_mag_drop_attachment_index: i32,
    #[field("C_CSPlayerPawnBase", "m_vecBulletHitModels")]
    bullet_hit_models: UtlVector<*const BulletHitModel>,
    #[field("C_CSPlayerPawnBase", "m_vecPickupModelSlerpers")]
    pickup_model_slerpers: UtlVector<*const PickUpModelSlerper>,
    #[field("C_CSPlayerPawnBase", "m_vecLastAliveLocalVelocity")]
    last_alive_local_velocity: Vector,
    #[field("C_CSPlayerPawnBase", "m_entitySpottedState")]
    entity_spotted_state: EntitySpottedState,
    #[field("C_CSPlayerPawnBase", "m_nSurvivalTeamNumber")]
    survival_team_number: i32,
    #[field("C_CSPlayerPawnBase", "m_bGuardianShouldSprayCustomXMark")]
    guardian_should_spray_custom_x_mark: bool,
    #[field("C_CSPlayerPawnBase", "m_bHasDeathInfo")]
    has_death_info: bool,
    #[field("C_CSPlayerPawnBase", "m_flDeathInfoTime")]
    death_info_time: f32,
    #[field("C_CSPlayerPawnBase", "m_vecDeathInfoOrigin")]
    death_info_origin: Vector,
    #[field("C_CSPlayerPawnBase", "m_bKilledByHeadshot")]
    killed_by_headshot: bool,
    #[field("C_CSPlayerPawnBase", "m_hOriginalController")]
    original_controller: Handle<PlayerController>,
    #[field("C_CSPlayerPawn", "m_pBulletServices")]
    bullet_services: *const PlayerBulletServices,
    #[field("C_CSPlayerPawn", "m_pHostageServices")]
    hostage_services: *const PlayerHostageServices,
    #[field("C_CSPlayerPawn", "m_pBuyServices")]
    buy_services: *const PlayerBuyServices,
    #[field("C_CSPlayerPawn", "m_pGlowServices")]
    glow_services: *const PlayerGlowServices,
    #[field("C_CSPlayerPawn", "m_pActionTrackingServices")]
    action_tracking_services: *const PlayerActionTrackingServices,
    #[field("C_CSPlayerPawn", "m_flHealthShotBoostExpirationTime")]
    health_shot_boost_expiration_time: GameTime,
    #[field("C_CSPlayerPawn", "m_flLastFiredWeaponTime")]
    last_fired_weapon_time: GameTime,
    #[field("C_CSPlayerPawn", "m_bHasFemaleVoice")]
    has_female_voice: bool,
    #[field("C_CSPlayerPawn", "m_flLandseconds")]
    landseconds: f32,
    #[field("C_CSPlayerPawn", "m_flOldFallVelocity")]
    old_fall_velocity: f32,
    #[field("C_CSPlayerPawn", "m_szLastPlaceName")]
    last_place_name: [std::ffi::c_char; 18],
    #[field("C_CSPlayerPawn", "m_bPrevDefuser")]
    prev_defuser: bool,
    #[field("C_CSPlayerPawn", "m_bPrevHelmet")]
    prev_helmet: bool,
    #[field("C_CSPlayerPawn", "m_nPrevArmorVal")]
    prev_armor_val: i32,
    #[field("C_CSPlayerPawn", "m_nPrevGrenadeAmmoCount")]
    prev_grenade_ammo_count: i32,
    #[field("C_CSPlayerPawn", "m_unPreviousWeaponHash")]
    previous_weapon_hash: u32,
    #[field("C_CSPlayerPawn", "m_unWeaponHash")]
    weapon_hash: u32,
    #[field("C_CSPlayerPawn", "m_bInBuyZone")]
    in_buy_zone: bool,
    #[field("C_CSPlayerPawn", "m_bPreviouslyInBuyZone")]
    previously_in_buy_zone: bool,
    #[field("C_CSPlayerPawn", "m_aimPunchAngle")]
    punch_angle: QAngle,
    #[field("C_CSPlayerPawn", "m_aimPunchAngleVel")]
    punch_angle_vel: QAngle,
    #[field("C_CSPlayerPawn", "m_aimPunchTickBase")]
    punch_tick_base: i32,
    #[field("C_CSPlayerPawn", "m_aimPunchTickFraction")]
    punch_tick_fraction: f32,
    #[field("C_CSPlayerPawn", "m_aimPunchCache")]
    punch_cache: UtlVector<QAngle>,
    #[field("C_CSPlayerPawn", "m_bInLanding")]
    in_landing: bool,
    #[field("C_CSPlayerPawn", "m_flLandingTime")]
    landing_time: f32,
    #[field("C_CSPlayerPawn", "m_bInHostageRescueZone")]
    in_hostage_rescue_zone: bool,
    #[field("C_CSPlayerPawn", "m_bInBombZone")]
    in_bomb_zone: bool,
    #[field("C_CSPlayerPawn", "m_bIsBuyMenuOpen")]
    is_buy_menu_open: bool,
    #[field("C_CSPlayerPawn", "m_flTimeOfLastInjury")]
    time_of_last_injury: GameTime,
    #[field("C_CSPlayerPawn", "m_flNextSprayDecalTime")]
    next_spray_decal_time: GameTime,
    #[field("C_CSPlayerPawn", "m_iRetakesOffering")]
    retakes_offering: i32,
    #[field("C_CSPlayerPawn", "m_iRetakesOfferingCard")]
    retakes_offering_card: i32,
    #[field("C_CSPlayerPawn", "m_bRetakesHasDefuseKit")]
    retakes_has_defuse_kit: bool,
    #[field("C_CSPlayerPawn", "m_bRetakesMVPLastRound")]
    retakes_mvp_last_round: bool,
    #[field("C_CSPlayerPawn", "m_iRetakesMVPBoostItem")]
    retakes_mvp_boost_item: i32,
    #[field("C_CSPlayerPawn", "m_RetakesMVPBoostExtraUtility")]
    retakes_mvp_boost_extra_utility: LoadoutSlot,
    #[field("C_CSPlayerPawn", "m_bNeedToReApplyGloves")]
    need_to_re_apply_gloves: bool,
    #[field("C_CSPlayerPawn", "m_EconGloves")]
    econ_gloves: EconItemView,
    #[field("C_CSPlayerPawn", "m_bMustSyncRagdollState")]
    must_sync_ragdoll_state: bool,
    #[field("C_CSPlayerPawn", "m_nRagdollDamageBone")]
    ragdoll_damage_bone: i32,
    #[field("C_CSPlayerPawn", "m_vRagdollDamageForce")]
    ragdoll_damage_force: Vector,
    #[field("C_CSPlayerPawn", "m_vRagdollDamagePosition")]
    ragdoll_damage_position: Vector,
    #[field("C_CSPlayerPawn", "m_szRagdollDamageWeaponName")]
    ragdoll_damage_weapon_name: [std::ffi::c_char; 64],
    #[field("C_CSPlayerPawn", "m_bRagdollDamageHeadshot")]
    ragdoll_damage_headshot: bool,
    #[field("C_CSPlayerPawn", "m_bLastHeadBoneTransformIsValid")]
    last_head_bone_transform_is_valid: bool,
    #[field("C_CSPlayerPawn", "m_lastLandTime")]
    last_land_time: GameTime,
    #[field("C_CSPlayerPawn", "m_bOnGroundLastTick")]
    on_ground_last_tick: bool,
    #[field("C_CSPlayerPawn", "m_qDeathEyeAngles")]
    q_death_eye_angles: QAngle,
    #[field("C_CSPlayerPawn", "m_bSkipOneHeadConstraintUpdate")]
    skip_one_head_constraint_update: bool,
}
