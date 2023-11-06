use crate::tier1::utlsymbollarge::UtlSymbolLarge;
use crate::types::{SplitScreenSlot, VariantBase, WorldGroupId};

use crate::interfaces::schema_system::schema_system::SCHEMA_SYSTEM;
use schema::Schema;

use super::networksystem::ChangeAccessorFieldPathIndex;

// scope engine2.dll
// 2023-10-29 23:17:00.901780300 UTC

pub enum SpawnDebugOverrideState {
    SpawnDebugOverrideNone,
    SpawnDebugOverrideForceEnabled,
    SpawnDebugOverrideForceDisabled,
}

pub enum SpawnDebugRestrictionOverrideState {
    SpawnDebugRestrictNone,
    SpawnDebugRestrictIgnoreManagerDistanceReqs,
    SpawnDebugRestrictIgnoreTemplateDistanceLosReqs,
    SpawnDebugRestrictIgnoreTemplateCooldownLimits,
    SpawnDebugRestrictIgnoreTargetCooldownLimits,
}

pub enum EntityDormancyType {
    EntityNotDormant,
    EntityDormant,
    EntitySuspended,
}

pub enum EntityIoTargetType {
    EntityIoTargetInvalid,
    EntityIoTargetEntityname,
    EntityIoTargetEhandle,
    EntityIoTargetEntitynameOrClassname,
}

#[derive(Schema)]
#[scope("engine2.dll")]
pub struct EngineLoopState {
    #[field("EngineLoopState_t", "m_nPlatWindowWidth")]
    plat_window_width: i32,
    #[field("EngineLoopState_t", "m_nPlatWindowHeight")]
    plat_window_height: i32,
    #[field("EngineLoopState_t", "m_nRenderWidth")]
    render_width: i32,
    #[field("EngineLoopState_t", "m_nRenderHeight")]
    render_height: i32,
}

#[derive(Schema)]
#[scope("engine2.dll")]
pub struct EventModInitialized {}

#[derive(Schema)]
#[scope("engine2.dll")]
pub struct EventFrameBoundary {
    #[field("EventFrameBoundary_t", "m_flFrameTime")]
    frame_time: f32,
}

#[derive(Schema)]
#[scope("engine2.dll")]
pub struct EventProfileStorageAvailable {
    #[field("EventProfileStorageAvailable_t", "m_nSplitScreenSlot")]
    split_screen_slot: SplitScreenSlot,
}

#[derive(Schema)]
#[scope("engine2.dll")]
pub struct EventSplitScreenStateChanged {}

#[derive(Schema)]
#[scope("engine2.dll")]
pub struct EventSetTime {
    #[field("EventSetTime_t", "m_LoopState")]
    loop_state: EngineLoopState,
    #[field("EventSetTime_t", "m_nClientOutputFrames")]
    client_output_frames: i32,
    #[field("EventSetTime_t", "m_flRealTime")]
    real_time: f64,
    #[field("EventSetTime_t", "m_flRenderTime")]
    render_time: f64,
    #[field("EventSetTime_t", "m_flRenderFrameTime")]
    render_frame_time: f64,
    #[field("EventSetTime_t", "m_flRenderFrameTimeUnbounded")]
    render_frame_time_unbounded: f64,
    #[field("EventSetTime_t", "m_flRenderFrameTimeUnscaled")]
    render_frame_time_unscaled: f64,
    #[field("EventSetTime_t", "m_flTickRemainder")]
    tick_remainder: f64,
}

#[derive(Schema)]
#[scope("engine2.dll")]
pub struct EventClientPollInput {
    #[field("EventClientPollInput_t", "m_LoopState")]
    loop_state: EngineLoopState,
    #[field("EventClientPollInput_t", "m_flRealTime")]
    real_time: f32,
}

#[derive(Schema)]
#[scope("engine2.dll")]
pub struct EventClientProcessInput {
    #[field("EventClientProcessInput_t", "m_LoopState")]
    loop_state: EngineLoopState,
    #[field("EventClientProcessInput_t", "m_flRealTime")]
    real_time: f32,
}

#[derive(Schema)]
#[scope("engine2.dll")]
pub struct EventClientProcessGameInput {
    #[field("EventClientProcessGameInput_t", "m_LoopState")]
    loop_state: EngineLoopState,
    #[field("EventClientProcessGameInput_t", "m_flRealTime")]
    real_time: f32,
    #[field("EventClientProcessGameInput_t", "m_flFrameTime")]
    frame_time: f32,
}

#[derive(Schema)]
#[scope("engine2.dll")]
pub struct EventClientPreOutput {
    #[field("EventClientPreOutput_t", "m_LoopState")]
    loop_state: EngineLoopState,
    #[field("EventClientPreOutput_t", "m_flRenderTime")]
    render_time: f64,
    #[field("EventClientPreOutput_t", "m_flRenderFrameTime")]
    render_frame_time: f64,
    #[field("EventClientPreOutput_t", "m_flRenderFrameTimeUnbounded")]
    render_frame_time_unbounded: f64,
    #[field("EventClientPreOutput_t", "m_flRealTime")]
    real_time: f32,
    #[field("EventClientPreOutput_t", "m_bRenderOnly")]
    render_only: bool,
}

#[derive(Schema)]
#[scope("engine2.dll")]
pub struct EventClientSceneSystemThreadStateChange {
    #[field("EventClientSceneSystemThreadStateChange_t", "m_bThreadsActive")]
    threads_active: bool,
}

#[derive(Schema)]
#[scope("engine2.dll")]
pub struct EventClientOutput {
    #[field("EventClientOutput_t", "m_LoopState")]
    loop_state: EngineLoopState,
    #[field("EventClientOutput_t", "m_flRenderTime")]
    render_time: f32,
    #[field("EventClientOutput_t", "m_flRealTime")]
    real_time: f32,
    #[field("EventClientOutput_t", "m_flRenderFrameTimeUnbounded")]
    render_frame_time_unbounded: f32,
    #[field("EventClientOutput_t", "m_bRenderOnly")]
    render_only: bool,
}

#[derive(Schema)]
#[scope("engine2.dll")]
pub struct EventClientPostOutput {
    #[field("EventClientPostOutput_t", "m_LoopState")]
    loop_state: EngineLoopState,
    #[field("EventClientPostOutput_t", "m_flRenderTime")]
    render_time: f64,
    #[field("EventClientPostOutput_t", "m_flRenderFrameTime")]
    render_frame_time: f32,
    #[field("EventClientPostOutput_t", "m_flRenderFrameTimeUnbounded")]
    render_frame_time_unbounded: f32,
    #[field("EventClientPostOutput_t", "m_bRenderOnly")]
    render_only: bool,
}

#[derive(Schema)]
#[scope("engine2.dll")]
pub struct EventClientFrameSimulate {
    #[field("EventClientFrameSimulate_t", "m_LoopState")]
    loop_state: EngineLoopState,
    #[field("EventClientFrameSimulate_t", "m_flRealTime")]
    real_time: f32,
    #[field("EventClientFrameSimulate_t", "m_flFrameTime")]
    frame_time: f32,
}

#[derive(Schema)]
#[scope("engine2.dll")]
pub struct EventSimpleLoopFrameUpdate {
    #[field("EventSimpleLoopFrameUpdate_t", "m_LoopState")]
    loop_state: EngineLoopState,
    #[field("EventSimpleLoopFrameUpdate_t", "m_flRealTime")]
    real_time: f32,
    #[field("EventSimpleLoopFrameUpdate_t", "m_flFrameTime")]
    frame_time: f32,
}

#[derive(Schema)]
#[scope("engine2.dll")]
pub struct EventSimulate {
    #[field("EventSimulate_t", "m_LoopState")]
    loop_state: EngineLoopState,
    #[field("EventSimulate_t", "m_bFirstTick")]
    first_tick: bool,
    #[field("EventSimulate_t", "m_bLastTick")]
    last_tick: bool,
}

#[derive(Schema)]
#[scope("engine2.dll")]
pub struct EventAdvanceTick {
    #[field("EventSimulate_t", "m_LoopState")]
    loop_state: EngineLoopState,
    #[field("EventSimulate_t", "m_bFirstTick")]
    first_tick: bool,
    #[field("EventSimulate_t", "m_bLastTick")]
    last_tick: bool,
    #[field("EventAdvanceTick_t", "m_nCurrentTick")]
    current_tick: i32,
    #[field("EventAdvanceTick_t", "m_nCurrentTickThisFrame")]
    current_tick_this_frame: i32,
    #[field("EventAdvanceTick_t", "m_nTotalTicksThisFrame")]
    total_ticks_this_frame: i32,
    #[field("EventAdvanceTick_t", "m_nTotalTicks")]
    total_ticks: i32,
}

#[derive(Schema)]
#[scope("engine2.dll")]
pub struct EventPostAdvanceTick {
    #[field("EventSimulate_t", "m_LoopState")]
    loop_state: EngineLoopState,
    #[field("EventSimulate_t", "m_bFirstTick")]
    first_tick: bool,
    #[field("EventSimulate_t", "m_bLastTick")]
    last_tick: bool,
    #[field("EventPostAdvanceTick_t", "m_nCurrentTick")]
    current_tick: i32,
    #[field("EventPostAdvanceTick_t", "m_nCurrentTickThisFrame")]
    current_tick_this_frame: i32,
    #[field("EventPostAdvanceTick_t", "m_nTotalTicksThisFrame")]
    total_ticks_this_frame: i32,
    #[field("EventPostAdvanceTick_t", "m_nTotalTicks")]
    total_ticks: i32,
}

#[derive(Schema)]
#[scope("engine2.dll")]
pub struct EventServerAdvanceTick {
    #[field("EventSimulate_t", "m_LoopState")]
    loop_state: EngineLoopState,
    #[field("EventSimulate_t", "m_bFirstTick")]
    first_tick: bool,
    #[field("EventSimulate_t", "m_bLastTick")]
    last_tick: bool,
    #[field("EventAdvanceTick_t", "m_nCurrentTick")]
    current_tick: i32,
    #[field("EventAdvanceTick_t", "m_nCurrentTickThisFrame")]
    current_tick_this_frame: i32,
    #[field("EventAdvanceTick_t", "m_nTotalTicksThisFrame")]
    total_ticks_this_frame: i32,
    #[field("EventAdvanceTick_t", "m_nTotalTicks")]
    total_ticks: i32,
}

#[derive(Schema)]
#[scope("engine2.dll")]
pub struct EventServerPostAdvanceTick {
    #[field("EventSimulate_t", "m_LoopState")]
    loop_state: EngineLoopState,
    #[field("EventSimulate_t", "m_bFirstTick")]
    first_tick: bool,
    #[field("EventSimulate_t", "m_bLastTick")]
    last_tick: bool,
    #[field("EventPostAdvanceTick_t", "m_nCurrentTick")]
    current_tick: i32,
    #[field("EventPostAdvanceTick_t", "m_nCurrentTickThisFrame")]
    current_tick_this_frame: i32,
    #[field("EventPostAdvanceTick_t", "m_nTotalTicksThisFrame")]
    total_ticks_this_frame: i32,
    #[field("EventPostAdvanceTick_t", "m_nTotalTicks")]
    total_ticks: i32,
}

#[derive(Schema)]
#[scope("engine2.dll")]
pub struct EventClientAdvanceTick {
    #[field("EventSimulate_t", "m_LoopState")]
    loop_state: EngineLoopState,
    #[field("EventSimulate_t", "m_bFirstTick")]
    first_tick: bool,
    #[field("EventSimulate_t", "m_bLastTick")]
    last_tick: bool,
    #[field("EventAdvanceTick_t", "m_nCurrentTick")]
    current_tick: i32,
    #[field("EventAdvanceTick_t", "m_nCurrentTickThisFrame")]
    current_tick_this_frame: i32,
    #[field("EventAdvanceTick_t", "m_nTotalTicksThisFrame")]
    total_ticks_this_frame: i32,
    #[field("EventAdvanceTick_t", "m_nTotalTicks")]
    total_ticks: i32,
}

#[derive(Schema)]
#[scope("engine2.dll")]
pub struct EventClientPostAdvanceTick {
    #[field("EventSimulate_t", "m_LoopState")]
    loop_state: EngineLoopState,
    #[field("EventSimulate_t", "m_bFirstTick")]
    first_tick: bool,
    #[field("EventSimulate_t", "m_bLastTick")]
    last_tick: bool,
    #[field("EventPostAdvanceTick_t", "m_nCurrentTick")]
    current_tick: i32,
    #[field("EventPostAdvanceTick_t", "m_nCurrentTickThisFrame")]
    current_tick_this_frame: i32,
    #[field("EventPostAdvanceTick_t", "m_nTotalTicksThisFrame")]
    total_ticks_this_frame: i32,
    #[field("EventPostAdvanceTick_t", "m_nTotalTicks")]
    total_ticks: i32,
}

#[derive(Schema)]
#[scope("engine2.dll")]
pub struct EventClientSendInput {
    #[field("EventClientSendInput_t", "m_bFinalClientCommandTick")]
    final_client_command_tick: bool,
    #[field("EventClientSendInput_t", "m_nAdditionalClientCommandsToCreate")]
    additional_client_commands_to_create: i32,
}

#[derive(Schema)]
#[scope("engine2.dll")]
pub struct EventClientPredictionPostNetupdate {}

#[derive(Schema)]
#[scope("engine2.dll")]
pub struct EventClientPollNetworking {
    #[field("EventClientPollNetworking_t", "m_nTickCount")]
    tick_count: i32,
}

#[derive(Schema)]
#[scope("engine2.dll")]
pub struct EventClientProcessNetworking {}

#[derive(Schema)]
#[scope("engine2.dll")]
pub struct EventClientPreSimulate {
    #[field("EventSimulate_t", "m_LoopState")]
    loop_state: EngineLoopState,
    #[field("EventSimulate_t", "m_bFirstTick")]
    first_tick: bool,
    #[field("EventSimulate_t", "m_bLastTick")]
    last_tick: bool,
}

#[derive(Schema)]
#[scope("engine2.dll")]
pub struct EventClientSimulate {
    #[field("EventSimulate_t", "m_LoopState")]
    loop_state: EngineLoopState,
    #[field("EventSimulate_t", "m_bFirstTick")]
    first_tick: bool,
    #[field("EventSimulate_t", "m_bLastTick")]
    last_tick: bool,
}

#[derive(Schema)]
#[scope("engine2.dll")]
pub struct EventServerPollNetworking {
    #[field("EventSimulate_t", "m_LoopState")]
    loop_state: EngineLoopState,
    #[field("EventSimulate_t", "m_bFirstTick")]
    first_tick: bool,
    #[field("EventSimulate_t", "m_bLastTick")]
    last_tick: bool,
}

#[derive(Schema)]
#[scope("engine2.dll")]
pub struct EventServerProcessNetworking {
    #[field("EventSimulate_t", "m_LoopState")]
    loop_state: EngineLoopState,
    #[field("EventSimulate_t", "m_bFirstTick")]
    first_tick: bool,
    #[field("EventSimulate_t", "m_bLastTick")]
    last_tick: bool,
}

#[derive(Schema)]
#[scope("engine2.dll")]
pub struct EventServerSimulate {
    #[field("EventSimulate_t", "m_LoopState")]
    loop_state: EngineLoopState,
    #[field("EventSimulate_t", "m_bFirstTick")]
    first_tick: bool,
    #[field("EventSimulate_t", "m_bLastTick")]
    last_tick: bool,
}

#[derive(Schema)]
#[scope("engine2.dll")]
pub struct EventServerPostSimulate {
    #[field("EventSimulate_t", "m_LoopState")]
    loop_state: EngineLoopState,
    #[field("EventSimulate_t", "m_bFirstTick")]
    first_tick: bool,
    #[field("EventSimulate_t", "m_bLastTick")]
    last_tick: bool,
}

#[derive(Schema)]
#[scope("engine2.dll")]
pub struct EventClientPostSimulate {
    #[field("EventSimulate_t", "m_LoopState")]
    loop_state: EngineLoopState,
    #[field("EventSimulate_t", "m_bFirstTick")]
    first_tick: bool,
    #[field("EventSimulate_t", "m_bLastTick")]
    last_tick: bool,
}

#[derive(Schema)]
#[scope("engine2.dll")]
pub struct EventClientPauseSimulate {
    #[field("EventSimulate_t", "m_LoopState")]
    loop_state: EngineLoopState,
    #[field("EventSimulate_t", "m_bFirstTick")]
    first_tick: bool,
    #[field("EventSimulate_t", "m_bLastTick")]
    last_tick: bool,
}

#[derive(Schema)]
#[scope("engine2.dll")]
pub struct EventPostDataUpdate {
    #[field("EventPostDataUpdate_t", "m_nCount")]
    count: i32,
}

#[derive(Schema)]
#[scope("engine2.dll")]
pub struct EventPreDataUpdate {
    #[field("EventPreDataUpdate_t", "m_nCount")]
    count: i32,
}

#[derive(Schema)]
#[scope("engine2.dll")]
pub struct EventAppShutdown {
    #[field("EventAppShutdown_t", "m_nDummy0")]
    dummy_0: i32,
}

#[derive(Schema)]
#[scope("engine2.dll")]
pub struct NetworkVarChainer {
    #[field("CNetworkVarChainer", "m_PathIndex")]
    path_index: ChangeAccessorFieldPathIndex,
}

#[derive(Schema)]
#[scope("engine2.dll")]
pub struct VariantDefaultAllocator {}

#[derive(Schema)]
#[scope("engine2.dll")]
pub struct EntOutput {}

#[derive(Schema)]
#[scope("engine2.dll")]
pub struct EntComponentInfo {
    #[field("EntComponentInfo_t", "m_pName")]
    name: *const std::ffi::c_char,
    #[field("EntComponentInfo_t", "m_pCPPClassname")]
    cpp_classname: *const std::ffi::c_char,
    #[field("EntComponentInfo_t", "m_pNetworkDataReferencedDescription")]
    network_data_referenced_description: *const std::ffi::c_char,
    #[field("EntComponentInfo_t", "m_pNetworkDataReferencedPtrPropDescription")]
    network_data_referenced_ptr_prop_description: *const std::ffi::c_char,
    #[field("EntComponentInfo_t", "m_nRuntimeIndex")]
    runtime_index: i32,
    #[field("EntComponentInfo_t", "m_nFlags")]
    flags: u32,
    #[field("EntComponentInfo_t", "m_pBaseClassComponentHelper")]
    base_class_component_helper: *const EntityComponentHelper,
}

#[derive(Schema)]
#[scope("engine2.dll")]
pub struct EntityComponent {}

#[derive(Schema)]
#[scope("engine2.dll")]
pub struct EntInput {}

#[derive(Schema)]
#[scope("engine2.dll")]
pub struct EntityComponentHelper {
    #[field("CEntityComponentHelper", "m_flags")]
    flags: u32,
    #[field("CEntityComponentHelper", "m_pInfo")]
    info: *const EntComponentInfo,
    #[field("CEntityComponentHelper", "m_nPriority")]
    priority: i32,
    #[field("CEntityComponentHelper", "m_pNext")]
    next: *const EntityComponentHelper,
}

#[derive(Schema)]
#[scope("engine2.dll")]
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
#[scope("engine2.dll")]
pub struct EmptyEntityInstance {}

#[derive(Schema)]
#[scope("engine2.dll")]
pub struct EntityInstance {
    #[field("CEntityInstance", "m_iszPrivateVScripts")]
    private_v_scripts: UtlSymbolLarge,
    #[field("CEntityInstance", "m_pEntity")]
    entity: *const EntityIdentity,
    #[field("CEntityInstance", "m_CScriptComponent")]
    script_component: *const ScriptComponent,
}

#[derive(Schema)]
#[scope("engine2.dll")]
pub struct EntityIoOutput {
    #[field("CEntityIOOutput", "m_Value")]
    value: VariantBase<VariantDefaultAllocator>,
}

#[derive(Schema)]
#[scope("engine2.dll")]
pub struct ScriptComponent {
    #[field("CScriptComponent", "m_scriptClassName")]
    script_class_name: UtlSymbolLarge,
}
