use crate::math::qangle::QAngle;
use crate::math::vector::{Vector, Vector2D};

use crate::schema::SCHEMA_SYSTEM;
use crate::tier1::utlstringtoken::UtlStringToken;
use crate::types::{HScript, NetworkedQuantizedFloat, VariantBase, WorldGroupId};
use schema::Schema;

use super::engine2::VariantDefaultAllocator;
use super::networksystem::ChangeAccessorFieldPathIndex;

// scope !GlobalTypes
// 2023-10-29 23:16:59.332075800 UTC

pub enum ThreeState {
    TrsFalse,
    TrsTrue,
    TrsNone,
}

pub enum Fieldtype {
    FieldVoid,
    FieldFloat32,
    FieldString,
    FieldVector,
    FieldQuaternion,
    FieldInt32,
    FieldBoolean,
    FieldInt16,
    FieldCharacter,
    FieldColor32,
    FieldEmbedded,
    FieldCustom,
    FieldClassptr,
    FieldEhandle,
    FieldPositionVector,
    FieldTime,
    FieldTick,
    FieldSoundname,
    FieldInput,
    FieldFunction,
    FieldVmatrix,
    FieldVmatrixWorldspace,
    FieldMatrix3X4Worldspace,
    FieldInterval,
    FieldUnused,
    FieldVector2D,
    FieldInt64,
    FieldVector4D,
    FieldResource,
    FieldTypeunknown,
    FieldCstring,
    FieldHscript,
    FieldVariant,
    FieldUint64,
    FieldFloat64,
    FieldPositiveintegerOrNull,
    FieldHscriptNewInstance,
    FieldUint32,
    FieldUtlstringtoken,
    FieldQangle,
    FieldNetworkOriginCellQuantizedVector,
    FieldHmaterial,
    FieldHmodel,
    FieldNetworkQuantizedVector,
    FieldNetworkQuantizedFloat,
    FieldDirectionVectorWorldspace,
    FieldQangleWorldspace,
    FieldQuaternionWorldspace,
    FieldHscriptLightbinding,
    FieldV8Value,
    FieldV8Object,
    FieldV8Array,
    FieldV8CallbackInfo,
    FieldUtlstring,
    FieldNetworkOriginCellQuantizedPositionVector,
    FieldHrendertexture,
    FieldHparticlesystemdefinition,
    FieldUint8,
    FieldUint16,
    FieldCtransform,
    FieldCtransformWorldspace,
    FieldHpostprocessing,
    FieldMatrix3X4,
    FieldShim,
    FieldCmotiontransform,
    FieldCmotiontransformWorldspace,
    FieldAttachmentHandle,
    FieldAmmoIndex,
    FieldConditionId,
    FieldAiScheduleBits,
    FieldModifierHandle,
    FieldRotationVector,
    FieldRotationVectorWorldspace,
    FieldHvdata,
    FieldScale32,
    FieldStringAndToken,
    FieldEngineTime,
    FieldEngineTick,
    FieldWorldGroupId,
    FieldTypecount,
}

pub enum FuseVariableAccess {
    Writable,
    ReadOnly,
}

pub enum FuseVariableType {
    Invalid,
    Bool,
    Int8,
    Int16,
    Int32,
    Uint8,
    Uint16,
    Uint32,
    Float32,
}

pub enum RenderSlotType {
    RenderSlotInvalid,
    RenderSlotPerVertex,
    RenderSlotPerInstance,
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

pub enum SpawnDebugRestrictionOverrideState {
    SpawnDebugRestrictNone,
    SpawnDebugRestrictIgnoreManagerDistanceReqs,
    SpawnDebugRestrictIgnoreTemplateDistanceLosReqs,
    SpawnDebugRestrictIgnoreTemplateCooldownLimits,
    SpawnDebugRestrictIgnoreTargetCooldownLimits,
}

pub enum SpawnDebugOverrideState {
    SpawnDebugOverrideNone,
    SpawnDebugOverrideForceEnabled,
    SpawnDebugOverrideForceDisabled,
}

pub enum EntityIoTargetType {
    EntityIoTargetInvalid,
    EntityIoTargetEntityname,
    EntityIoTargetEhandle,
    EntityIoTargetEntitynameOrClassname,
}

pub enum EntityDormancyType {
    EntityNotDormant,
    EntityDormant,
    EntitySuspended,
}

pub enum HorizJustificationE {
    HorizJustificationLeft,
    HorizJustificationCenter,
    HorizJustificationRight,
    HorizJustificationNone,
}

pub enum BloomBlendMode {
    BloomBlendAdd,
    BloomBlendScreen,
    BloomBlendBlur,
}

pub enum ViewFadeMode {
    ViewFadeConstantColor,
    ViewFadeModulate,
    ViewFadeMod2X,
}

pub enum LayoutPositionTypeE {
    LayoutpositiontypeViewportRelative,
    LayoutpositiontypeFractional,
    LayoutpositiontypeNone,
}

pub enum VertJustificationE {
    VertJustificationTop,
    VertJustificationCenter,
    VertJustificationBottom,
    VertJustificationNone,
}

pub enum MoodType {
    EMoodTypeHead,
    EMoodTypeBody,
}

pub enum JumpCorrectionMethod {
    ScaleMotion,
    AddCorrectionDelta,
}

pub enum BinaryNodeChildOption {
    Child1,
    Child2,
}

pub enum ModelConfigAttachmentType {
    ModelConfigAttachmentInvalid,
    ModelConfigAttachmentBoneOrAttachment,
    ModelConfigAttachmentRootRelative,
    ModelConfigAttachmentBonemerge,
    ModelConfigAttachmentCount,
}

pub enum BoneMaskBlendSpace {
    BlendSpaceParent,
    BlendSpaceModel,
    BlendSpaceModelRotationOnly,
    BlendSpaceModelTranslationOnly,
}

pub enum IkTargetType {
    IkTargetAttachment,
    IkTargetBone,
    IkTargetParameterModelSpace,
    IkTargetParameterWorldSpace,
}

pub enum AnimVectorSource {
    MoveDirection,
    FacingDirection,
    LookDirection,
    VectorParameter,
    WayPointDirection,
    Acceleration,
    SlopeNormal,
    SlopeNormalWorldSpace,
    LookTarget,
    LookTargetWorldSpace,
    WayPointPosition,
    GoalPosition,
    RootMotionVelocity,
}

pub enum ChoiceMethod {
    WeightedRandom,
    WeightedRandomNoRepeat,
    Iterate,
    IterateRandom,
}

pub enum FootFallTagFoot {
    Foot1,
    Foot2,
    Foot3,
    Foot4,
    Foot5,
    Foot6,
    Foot7,
    Foot8,
}

pub enum AnimationSnapshotType {
    AnimationSnapshotServerSimulation,
    AnimationSnapshotClientSimulation,
    AnimationSnapshotClientPrediction,
    AnimationSnapshotClientInterpolation,
    AnimationSnapshotClientRender,
    AnimationSnapshotFinalComposite,
    AnimationSnapshotMax,
}

pub enum AimMatrixBlendMode {
    AimMatrixBlendModeNone,
    AimMatrixBlendModeAdditive,
    AimMatrixBlendModeModelSpaceAdditive,
    AimMatrixBlendModeBoneMask,
}

pub enum FacingMode {
    FacingModeManual,
    FacingModePath,
    FacingModeLookTarget,
}

pub enum VelocityMetricMode {
    DirectionOnly,
    MagnitudeOnly,
    DirectionAndMagnitude,
}

pub enum Flags {
    JointFlagsNone,
    JointFlagsBody1Fixed,
    JointFlagsUseBlockSolver,
}

pub enum AnimVrHandMotionRange {
    MotionRangeWithController,
    MotionRangeWithoutController,
}

pub enum FieldNetworkOption {
    Auto,
    ForceEnable,
    ForceDisable,
}

pub enum ParticleAttachment {
    PattachInvalid,
    PattachAbsorigin,
    PattachAbsoriginFollow,
    PattachCustomorigin,
    PattachCustomoriginFollow,
    PattachPoint,
    PattachPointFollow,
    PattachEyesFollow,
    PattachOverheadFollow,
    PattachWorldorigin,
    PattachRootboneFollow,
    PattachRenderoriginFollow,
    PattachMainView,
    PattachWaterwake,
    PattachCenterFollow,
    PattachCustomGameState1,
    PattachHealthbar,
    MaxPattachTypes,
}

pub enum SelectorTagBehavior {
    SelectorTagBehaviorOnWhileCurrent,
    SelectorTagBehaviorOffWhenFinished,
    SelectorTagBehaviorOffBeforeFinished,
}

pub enum AnimParamButton {
    AnimparamButtonNone,
    AnimparamButtonDpadUp,
    AnimparamButtonDpadRight,
    AnimparamButtonDpadDown,
    AnimparamButtonDpadLeft,
    AnimparamButtonA,
    AnimparamButtonB,
    AnimparamButtonX,
    AnimparamButtonY,
    AnimparamButtonLeftShoulder,
    AnimparamButtonRightShoulder,
    AnimparamButtonLtrigger,
    AnimparamButtonRtrigger,
}

pub enum SeqPoseSetting {
    SeqPoseSettingConstant,
    SeqPoseSettingRotation,
    SeqPoseSettingPosition,
    SeqPoseSettingVelocity,
}

pub enum AnimScriptType {
    AnimscriptTypeInvalid,
    AnimscriptFuseGeneral,
    AnimscriptFuseStatemachine,
}

pub enum IkEndEffectorType {
    IkEndEffectorAttachment,
    IkEndEffectorBone,
}

pub enum StanceOverrideMode {
    Sequence,
    Node,
}

pub enum JiggleBoneSimSpace {
    SimSpaceLocal,
    SimSpaceModel,
    SimSpaceWorld,
}

pub enum AnimationProcessingType {
    AnimationProcessingServerSimulation,
    AnimationProcessingClientSimulation,
    AnimationProcessingClientPrediction,
    AnimationProcessingClientInterpolation,
    AnimationProcessingClientRender,
    AnimationProcessingMax,
}

pub enum DampingSpeedFunction {
    NoDamping,
    Constant,
    Spring,
}

pub enum FootPinningTimingSource {
    FootMotion,
    Tag,
    Parameter,
}

pub enum FootLockSubVisualization {
    FootlocksubvisualizationReachabilityAnalysis,
    FootlocksubvisualizationIkSolve,
}

pub enum FootstepLandedFootSoundType {
    FootsoundLeft,
    FootsoundRight,
    FootsoundUseOverrideSound,
}

pub enum IkChannelMode {
    TwoBone,
    TwoBoneTranslate,
    OneBone,
    OneBoneTranslate,
}

pub enum AnimVrFingerSplay {
    AnimVrFingerSplayThumbIndex,
    AnimVrFingerSplayIndexMiddle,
    AnimVrFingerSplayMiddleRing,
    AnimVrFingerSplayRingPinky,
}

pub enum AnimVrBoneTransformSource {
    AnimVrBoneTransformSourceLiveStream,
    AnimVrBoneTransformSourceGripLimit,
}

pub enum ResetCycleOption {
    Beginning,
    SameCycleAsSource,
    InverseSourceCycle,
    FixedValue,
    SameTimeAsSource,
}

pub enum MorphBundleType {
    MorphBundleTypeNone,
    MorphBundleTypePositionSpeed,
    MorphBundleTypeNormalWrinkle,
    MorphBundleTypeCount,
}

pub enum BoneFlags {
    FlagNoBoneFlags,
    FlagBoneflexdriver,
    FlagCloth,
    FlagPhysics,
    FlagAttachment,
    FlagAnimation,
    FlagMesh,
    FlagHitbox,
    FlagBoneUsedByVertexLod0,
    FlagBoneUsedByVertexLod1,
    FlagBoneUsedByVertexLod2,
    FlagBoneUsedByVertexLod3,
    FlagBoneUsedByVertexLod4,
    FlagBoneUsedByVertexLod5,
    FlagBoneUsedByVertexLod6,
    FlagBoneUsedByVertexLod7,
    FlagBoneMergeRead,
    FlagBoneMergeWrite,
    FlagAllBoneFlags,
    BlendPrealigned,
    FlagRigidlength,
    FlagProcedural,
}

pub enum StateActionBehavior {
    StatetagbehaviorActiveWhileCurrent,
    StatetagbehaviorFireOnEnter,
    StatetagbehaviorFireOnExit,
    StatetagbehaviorFireOnEnterAndExit,
}

pub enum BlendKeyType {
    BlendKeyUserValue,
    BlendKeyVelocity,
    BlendKeyDistance,
    BlendKeyRemainingDistance,
}

pub enum EnumFlags0 {
    Flag0ShiftInterpenetrate,
    Flag0ShiftConstrain,
    Flag0ShiftBreakableForce,
    Flag0ShiftBreakableTorque,
}

pub enum ChoiceBlendMethod {
    SingleBlendTime,
    PerChoiceBlendTimes,
}

pub enum ChoiceChangeMethod {
    OnReset,
    OnCycleEnd,
    OnResetOrCycleEnd,
}

pub enum Blend2DMode {
    Blend2DModeGeneral,
    Blend2DModeDirectional,
}

pub enum AnimVrHand {
    AnimVrHandLeft,
    AnimVrHandRight,
}

pub enum IkSolverType {
    IksolverPerlin,
    IksolverTwoBone,
    IksolverFabrik,
    IksolverDogLeg3Bone,
    IksolverCcd,
    IksolverCount,
}

pub enum RagdollPoseControl {
    Absolute,
    Relative,
}

pub enum AnimValueSource {
    MoveHeading,
    MoveSpeed,
    ForwardSpeed,
    StrafeSpeed,
    FacingHeading,
    ManualFacingHeading,
    LookHeading,
    LookPitch,
    LookDistance,
    Parameter,
    WayPointHeading,
    WayPointDistance,
    BoundaryRadius,
    TargetMoveHeading,
    TargetMoveSpeed,
    AccelerationHeading,
    AccelerationSpeed,
    SlopeHeading,
    SlopeAngle,
    SlopePitch,
    SlopeYaw,
    GoalDistance,
    AccelerationLeftRight,
    AccelerationFrontBack,
    RootMotionSpeed,
    RootMotionTurnSpeed,
    MoveHeadingRelativeToLookHeading,
    MaxMoveSpeed,
    FingerCurlThumb,
    FingerCurlIndex,
    FingerCurlMiddle,
    FingerCurlRing,
    FingerCurlPinky,
    FingerSplayThumbIndex,
    FingerSplayIndexMiddle,
    FingerSplayMiddleRing,
    FingerSplayRingPinky,
}

pub enum PoseType {
    PosetypeStatic,
    PosetypeDynamic,
    PosetypeInvalid,
}

pub enum ModelBoneFlexComponent {
    ModelBoneFlexInvalid,
    ModelBoneFlexTx,
    ModelBoneFlexTy,
    ModelBoneFlexTz,
}

pub enum MeshDrawPrimitiveFlags {
    MeshDrawFlagsNone,
    MeshDrawFlagsUseShadowFastPath,
    MeshDrawFlagsUseCompressedNormalTangent,
    MeshDrawInputLayoutIsNotMatchedToMaterial,
    MeshDrawFlagsUseCompressedPerVertexLighting,
    MeshDrawFlagsUseUncompressedPerVertexLighting,
    MeshDrawFlagsCanBatchWithDynamicShaderConstants,
    MeshDrawFlagsDrawLast,
}

pub enum MorphFlexControllerRemapType {
    MorphFlexcontrollerRemapPassthru,
    MorphFlexcontrollerRemap2Way,
    MorphFlexcontrollerRemapNway,
    MorphFlexcontrollerRemapEyelid,
}

pub enum AnimParamNetworkSetting {
    Auto,
    AlwaysNetwork,
    NeverNetwork,
}

pub enum FlagEnum {
    FlagTranslucent,
    FlagTranslucentTwoPass,
    FlagModelIsRuntimeCombined,
    FlagSource1Import,
    FlagModelPartChild,
    FlagNavGenNone,
    FlagNavGenHull,
    FlagNoForcedFade,
    FlagHasSkinnedMeshes,
    FlagDoNotCastShadows,
    FlagForcePhonemeCrossfade,
    FlagNoAnimEvents,
    FlagAnimationDrivenFlexes,
    FlagImplicitBindPoseSequence,
    FlagModelDoc,
}

pub enum IkTargetSource {
    IktargetsourceBone,
    IktargetsourceAnimgraphParameter,
    IktargetsourceCount,
}

pub enum AnimParamType {
    AnimparamUnknown,
    AnimparamBool,
    AnimparamEnum,
    AnimparamInt,
    AnimparamFloat,
    AnimparamVector,
    AnimparamQuaternion,
    AnimparamStringtoken,
    AnimparamCount,
}

pub enum MatterialAttributeTagType {
    MaterialAttributeTagValue,
    MaterialAttributeTagColor,
}

pub enum AnimNodeNetworkMode {
    ServerAuthoritative,
    ClientSimulate,
}

pub enum FlexOpCode {
    FlexOpConst,
    FlexOpFetch1,
    FlexOpFetch2,
    FlexOpAdd,
    FlexOpSub,
    FlexOpMul,
    FlexOpDiv,
    FlexOpNeg,
    FlexOpExp,
    FlexOpOpen,
    FlexOpClose,
    FlexOpComma,
    FlexOpMax,
    FlexOpMin,
    FlexOp2Way0,
    FlexOp2Way1,
    FlexOpNway,
    FlexOpCombo,
    FlexOpDominate,
    FlexOpDmeLowerEyelid,
    FlexOpDmeUpperEyelid,
    FlexOpSqrt,
    FlexOpRemapvalclamped,
    FlexOpSin,
    FlexOpCos,
    FlexOpAbs,
}

pub enum StepPhase {
    StepPhaseOnGround,
    StepPhaseInAir,
}

pub enum EDemoBoneSelectionMode {
    CaptureAllBones,
    CaptureSelectedBones,
}

pub enum AnimPoseControl {
    NoPoseControl,
    AbsolutePoseControl,
    RelativePoseControl,
}

pub enum VPhysXFlagEnum {
    FlagIsPolysoupGeometry,
    FlagLevelCollision,
    FlagIgnoreScaleObsoleteDoNotUse,
}

pub enum IkTargetCoordinateSystem {
    IktargetcoordinatesystemWorldSpace,
    IktargetcoordinatesystemModelSpace,
    IktargetcoordinatesystemCount,
}

pub enum AnimVrFinger {
    AnimVrFingerThumb,
    AnimVrFingerIndex,
    AnimVrFingerMiddle,
    AnimVrFingerRing,
    AnimVrFingerPinky,
}

pub enum SolveIkChainAnimNodeDebugSetting {
    SolveikchainanimnodedebugsettingNone,
    SolveikchainanimnodedebugsettingXAxisCircle,
    SolveikchainanimnodedebugsettingYAxisCircle,
    SolveikchainanimnodedebugsettingZAxisCircle,
    SolveikchainanimnodedebugsettingForward,
    SolveikchainanimnodedebugsettingUp,
    SolveikchainanimnodedebugsettingLeft,
}

pub enum BinaryNodeTiming {
    UseChild1,
    UseChild2,
    SyncChildren,
}

pub enum CAnimationGraphVisualizerPrimitiveType {
    AnimationgraphvisualizerprimitivetypeText,
    AnimationgraphvisualizerprimitivetypeSphere,
    AnimationgraphvisualizerprimitivetypeLine,
    AnimationgraphvisualizerprimitivetypePie,
    AnimationgraphvisualizerprimitivetypeAxis,
}

pub enum BoneTransformSpace {
    BoneTransformSpaceInvalid,
    BoneTransformSpaceParent,
    BoneTransformSpaceModel,
    BoneTransformSpaceWorld,
}

pub enum SeqCmd {
    SeqCmdNop,
    SeqCmdLinearDelta,
    SeqCmdFetchFrameRange,
    SeqCmdSlerp,
    SeqCmdAdd,
    SeqCmdSubtract,
    SeqCmdScale,
    SeqCmdCopy,
    SeqCmdBlend,
    SeqCmdWorldspace,
    SeqCmdSequence,
    SeqCmdFetchCycle,
    SeqCmdFetchFrame,
    SeqCmdIkLockInPlace,
    SeqCmdIkRestoreAll,
    SeqCmdReverseSequence,
    SeqCmdTransform,
}

pub enum JointAxis {
    JointAxisX,
    JointAxisY,
    JointAxisZ,
    JointAxisCount,
}

pub enum JointMotion {
    JointMotionFree,
    JointMotionLocked,
    JointMotionCount,
}

pub enum SosGroupType {
    SosGrouptypeDynamic,
    SosGrouptypeStatic,
}

pub enum VMixSubgraphSwitchInterpolationType {
    SubgraphInterpolationTemporalCrossfade,
    SubgraphInterpolationTemporalFadeOut,
    SubgraphInterpolationKeepLastSubgraphRunning,
}

pub enum ActionType {
    SosActionNone,
    SosActionLimiter,
    SosActionTimeLimit,
    SosActionSetSoundeventParam,
}

pub enum VMixFilterSlope {
    FilterSlope1Pole6DB,
    FilterSlope1Pole12DB,
    FilterSlope1Pole18DB,
    FilterSlope1Pole24DB,
    FilterSlope12DB,
    FilterSlope24DB,
    FilterSlope36DB,
    FilterSlope48DB,
    FilterSlopeMax,
}

pub enum VMixChannelOperation {
    VmixChanStereo,
    VmixChanLeft,
    VmixChanRight,
    VmixChanSwap,
    VmixChanMono,
    VmixChanMidSide,
}

pub enum VMixPannerType {
    PannerTypeLinear,
    PannerTypeEqualPower,
}

pub enum Soundlevel {
    SndlvlNone,
    Sndlvl20DB,
    Sndlvl25DB,
    Sndlvl30DB,
    Sndlvl35DB,
    Sndlvl40DB,
    Sndlvl45DB,
    Sndlvl50DB,
    Sndlvl55DB,
    SndlvlIdle,
    Sndlvl60DB,
    Sndlvl65DB,
    SndlvlStatic,
    Sndlvl70DB,
    SndlvlNorm,
    Sndlvl75DB,
    Sndlvl80DB,
    SndlvlTalking,
    Sndlvl85DB,
    Sndlvl90DB,
    Sndlvl95DB,
    Sndlvl100DB,
    Sndlvl105DB,
    Sndlvl110DB,
    Sndlvl120DB,
    Sndlvl130DB,
    SndlvlGunfire,
    Sndlvl140DB,
    Sndlvl150DB,
    Sndlvl180DB,
}

pub enum VMixProcessorType {
    VprocessorUnknown,
    VprocessorSteamaudioReverb,
    VprocessorRtPitch,
    VprocessorSteamaudioHrtf,
    VprocessorDynamics,
    VprocessorPresetdsp,
    VprocessorDelay,
    VprocessorModDelay,
    VprocessorDiffusor,
    VprocessorBoxverb,
    VprocessorFreeverb,
    VprocessorPlateverb,
    VprocessorFullwaveIntegrator,
    VprocessorFilter,
    VprocessorSteamaudioPathing,
    VprocessorEq8,
    VprocessorEnvelope,
    VprocessorVocoder,
    VprocessorConvolution,
    VprocessorDynamics3Band,
    VprocessorDynamicsCompressor,
    VprocessorShaper,
    VprocessorPanner,
    VprocessorUtility,
    VprocessorAutofilter,
    VprocessorOsc,
    VprocessorStereodelay,
    VprocessorEffectChain,
    VprocessorSubgraphSwitch,
    VprocessorSteamaudioDirect,
}

pub enum SosActionSortType {
    SosSorttypeHighest,
    SosSorttypeLowest,
}

pub enum SosEditItemType {
    SosEditItemTypeSoundevents,
    SosEditItemTypeSoundevent,
    SosEditItemTypeLibrarystacks,
    SosEditItemTypeStack,
    SosEditItemTypeOperator,
    SosEditItemTypeField,
}

pub enum SosActionStopType {
    SosStoptypeNone,
    SosStoptypeTime,
    SosStoptypeOpvar,
}

pub enum VMixFilterType {
    FilterUnknown,
    FilterLowpass,
    FilterHighpass,
    FilterBandpass,
    FilterNotch,
    FilterPeakingEq,
    FilterLowShelf,
    FilterHighShelf,
    FilterAllpass,
    FilterPassthrough,
}

pub enum VMixLfoShape {
    LfoShapeSine,
    LfoShapeSquare,
    LfoShapeTri,
    LfoShapeSaw,
    LfoShapeNoise,
}

pub enum DisableShadows {
    KDisableShadowsNone,
    KDisableShadowsAll,
    KDisableShadowsBaked,
    KDisableShadowsRealtime,
}

pub enum ObjectTypeFlags {
    ObjectTypeNone,
    ObjectTypeImageLod,
    ObjectTypeGeometryLod,
    ObjectTypeDecal,
    ObjectTypeModel,
    ObjectTypeBlockLight,
    ObjectTypeNoShadows,
    ObjectTypeWorldspaceTexureBlend,
    ObjectTypeDisabledInLowQuality,
    ObjectTypeNoSunShadows,
    ObjectTypeRenderWithDynamic,
    ObjectTypeRenderToCubemaps,
    ObjectTypeModelHasLods,
    ObjectTypeOverlay,
    ObjectTypePrecomputedVismembers,
    ObjectTypeStaticCubeMap,
}

pub enum PulseInstructionCode {
    Invalid,
    ImmediateHalt,
    ReturnVoid,
    ReturnValue,
    Nop,
    Jump,
    JumpCond,
    ChunkLeap,
    ChunkLeapCond,
    PulseCallSync,
    PulseCallAsyncFire,
    CellInvoke,
    LibraryInvoke,
    TargetInvoke,
    SetVar,
    GetVar,
    SetRegisterLitBool,
    SetRegisterLitInt,
    SetRegisterLitFloat,
    SetRegisterLitStr,
    SetRegisterLitInvalEhandle,
    SetRegisterLitInvalSndevtGuid,
    SetRegisterLitVec3,
    SetRegisterDomainValue,
    Copy,
    Not,
    Negate,
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Lt,
    Lte,
    Eq,
    Ne,
    And,
    Or,
    ConvertValue,
    LastSerializedCode,
    NegateInt,
    NegateFloat,
    AddInt,
    AddFloat,
    AddString,
    SubInt,
    SubFloat,
    MulInt,
    MulFloat,
    DivInt,
    DivFloat,
    ModInt,
    ModFloat,
    LtInt,
    LtFloat,
    LteInt,
    LteFloat,
    EqBool,
    EqInt,
    EqFloat,
    EqString,
    NeBool,
    NeInt,
    NeFloat,
    NeString,
}

pub enum PulseValueType {
    PvalInvalid,
    PvalBool,
    PvalInt,
    PvalFloat,
    PvalString,
    PvalVec3,
    PvalTransform,
    PvalEhandle,
    PvalResource,
    PvalSndevtGuid,
    PvalSchemaPtr,
    PvalCursorFlow,
    PvalAny,
    PvalCount,
}

pub enum PulseMethodCallMode {
    SyncWaitForCompletion,
    AsyncFireAndForget,
}

pub enum TextureRepetitionMode {
    TextureRepetitionParticle,
    TextureRepetitionPath,
}

pub enum ParticleOrientationSetMode {
    ParticleOrientationSetFromVelocity,
    ParticleOrientationSetFromRotations,
}

pub enum ParticleLightTypeChoiceList {
    ParticleLightTypePoint,
    ParticleLightTypeSpot,
    ParticleLightTypeFx,
    ParticleLightTypeCapsule,
}

pub enum ParticleLightFogLightingMode {
    ParticleLightFogLightingModeNone,
    ParticleLightFogLightingModeDynamic,
    ParticleLightFogLightingModeDynamicNoshadows,
}

pub enum ParticleOmni2LightTypeChoiceList {
    ParticleOmni2LightTypePoint,
    ParticleOmni2LightTypeSphere,
}

pub enum SpriteCardShaderType {
    SpritecardShaderBase,
    SpritecardShaderCustom,
}

pub enum ParticleImpulseType {
    ImpulseTypeNone,
    ImpulseTypeGeneric,
    ImpulseTypeRope,
    ImpulseTypeExplosion,
    ImpulseTypeExplosionUnderwater,
    ImpulseTypeParticleSystem,
}

pub enum ClosestPointTestType {
    ParticleClosestTypeBox,
    ParticleClosestTypeCapsule,
    ParticleClosestTypeHybrid,
}

pub enum ParticleEndcapMode {
    ParticleEndcapAlwaysOn,
    ParticleEndcapEndcapOff,
    ParticleEndcapEndcapOn,
}

pub enum ParticleSortingChoiceList {
    ParticleSortingNearest,
    ParticleSortingCreationTime,
}

pub enum ParticleCollisionMode {
    CollisionModePerParticleTrace,
    CollisionModeUseNearestTrace,
    CollisionModePerFramePlaneset,
    CollisionModeInitialTraceDown,
    CollisionModeDisabled,
}

pub enum ParticleOrientationChoiceList {
    ParticleOrientationScreenAligned,
    ParticleOrientationScreenZAligned,
    ParticleOrientationWorldZAligned,
    ParticleOrientationAlignToParticleNormal,
    ParticleOrientationScreenalignToParticleNormal,
    ParticleOrientationFull3AxisRotation,
}

pub enum ParticleHitboxDataSelection {
    ParticleHitboxAverageSpeed,
    ParticleHitboxCount,
}

pub enum ParticleTransformType {
    PtTypeInvalid,
    PtTypeNamedValue,
    PtTypeControlPoint,
    PtTypeControlPointRange,
    PtTypeCount,
}

pub enum SpriteCardTextureChannel {
    SpritecardTextureChannelMixRgb,
    SpritecardTextureChannelMixRgba,
    SpritecardTextureChannelMixA,
    SpritecardTextureChannelMixRgbA,
    SpritecardTextureChannelMixRgbAlphamask,
    SpritecardTextureChannelMixRgbRgbmask,
    SpritecardTextureChannelMixRgbaRgbalpha,
    SpritecardTextureChannelMixARgbalpha,
    SpritecardTextureChannelMixRgbARgbalpha,
    SpritecardTextureChannelMixR,
    SpritecardTextureChannelMixG,
    SpritecardTextureChannelMixB,
    SpritecardTextureChannelMixRalpha,
    SpritecardTextureChannelMixGalpha,
    SpritecardTextureChannelMixBalpha,
}

pub enum ParticleAlphaReferenceType {
    ParticleAlphaReferenceAlphaAlpha,
    ParticleAlphaReferenceOpaqueAlpha,
    ParticleAlphaReferenceAlphaOpaque,
    ParticleAlphaReferenceOpaqueOpaque,
}

pub enum SpriteCardTextureType {
    SpritecardTextureDiffuse,
    SpritecardTextureZoom,
    SpritecardTexture1DColorLookup,
    SpritecardTextureUvdistortion,
    SpritecardTextureUvdistortionZoom,
    SpritecardTextureNormalmap,
    SpritecardTextureAnimmotionvec,
    SpritecardTextureSphericalHarmonicsA,
    SpritecardTextureSphericalHarmonicsB,
    SpritecardTextureSphericalHarmonicsC,
}

pub enum BBoxVolumeType {
    BboxVolume,
    BboxDimensions,
    BboxMinsMaxs,
}

pub enum ParticleDetailLevel {
    ParticledetailLow,
    ParticledetailMedium,
    ParticledetailHigh,
    ParticledetailUltra,
}

pub enum ParticleSequenceCropOverride {
    ParticleSequenceCropOverrideDefault,
    ParticleSequenceCropOverrideForceOff,
    ParticleSequenceCropOverrideForceOn,
}

pub enum ParticleFalloffFunction {
    ParticleFalloffConstant,
    ParticleFalloffLinear,
    ParticleFalloffExponential,
}

pub enum ParticleLightUnitChoiceList {
    ParticleLightUnitCandelas,
    ParticleLightUnitLumens,
}

pub enum ParticleVecType {
    PvecTypeInvalid,
    PvecTypeLiteral,
    PvecTypeLiteralColor,
    PvecTypeNamedValue,
    PvecTypeParticleVector,
    PvecTypeParticleVelocity,
    PvecTypeCpValue,
    PvecTypeCpRelativePosition,
    PvecTypeCpRelativeDir,
    PvecTypeCpRelativeRandomDir,
    PvecTypeFloatComponents,
    PvecTypeFloatInterpClamped,
    PvecTypeFloatInterpOpen,
    PvecTypeFloatInterpGradient,
    PvecTypeRandomUniform,
    PvecTypeRandomUniformOffset,
    PvecTypeCpDelta,
    PvecTypeClosestCameraPosition,
    PvecTypeCount,
}

pub enum PfNoiseModifier {
    PfNoiseModifierNone,
    PfNoiseModifierLines,
    PfNoiseModifierClumps,
    PfNoiseModifierRings,
}

pub enum ParticleFloatRandomMode {
    PfRandomModeInvalid,
    PfRandomModeConstant,
    PfRandomModeVarying,
    PfRandomModeCount,
}

pub enum ParticleDepthFeatheringMode {
    ParticleDepthFeatheringOff,
    ParticleDepthFeatheringOnOptional,
    ParticleDepthFeatheringOnRequired,
}

pub enum MaterialProxyType {
    MaterialProxyStatusEffect,
    MaterialProxyTint,
}

pub enum ParticleLightnintBranchBehavior {
    ParticleLightningBranchCurrentDir,
    ParticleLightningBranchEndpointDir,
}

pub enum ParticleOutputBlendMode {
    ParticleOutputBlendModeAlpha,
    ParticleOutputBlendModeAdd,
    ParticleOutputBlendModeBlendAdd,
    ParticleOutputBlendModeHalfBlendAdd,
    ParticleOutputBlendModeNegHalfBlendAdd,
    ParticleOutputBlendModeMod2X,
    ParticleOutputBlendModeLighten,
}

pub enum ParticleFloatInputMode {
    PfInputModeInvalid,
    PfInputModeClamped,
    PfInputModeLooped,
    PfInputModeCount,
}

pub enum ParticleSelection {
    ParticleSelectionFirst,
    ParticleSelectionLast,
    ParticleSelectionNumber,
}

pub enum ParticleTextureLayerBlendType {
    SpritecardTextureBlendMultiply,
    SpritecardTextureBlendMod2X,
    SpritecardTextureBlendReplace,
    SpritecardTextureBlendAdd,
    SpritecardTextureBlendSubtract,
    SpritecardTextureBlendAverage,
    SpritecardTextureBlendLuminance,
}

pub enum ParticleTraceSet {
    ParticleTraceSetAll,
    ParticleTraceSetStatic,
    ParticleTraceSetStaticAndKeyframed,
    ParticleTraceSetDynamic,
}

pub enum ParticleFloatMapType {
    PfMapTypeInvalid,
    PfMapTypeDirect,
    PfMapTypeMult,
    PfMapTypeRemap,
    PfMapTypeRemapBiased,
    PfMapTypeCurve,
    PfMapTypeNotched,
    PfMapTypeCount,
}

pub enum ParticleLightBehaviorChoiceList {
    ParticleLightBehaviorFollowDirection,
    ParticleLightBehaviorRope,
    ParticleLightBehaviorTrails,
}

pub enum ParticleTopology {
    ParticleTopologyPoints,
    ParticleTopologyLines,
    ParticleTopologyTris,
    ParticleTopologyQuads,
    ParticleTopologyCubes,
}

pub enum AnimationType {
    AnimationTypeFixedRate,
    AnimationTypeFitLifetime,
    AnimationTypeManualFrames,
}

pub enum HitboxLerpType {
    HitboxLerpLifetime,
    HitboxLerpConstant,
}

pub enum ParticleRotationLockType {
    ParticleRotationLockNone,
    ParticleRotationLockRotations,
    ParticleRotationLockNormal,
}

pub enum VectorExpressionType {
    VectorExpressionUninitialized,
    VectorExpressionAdd,
    VectorExpressionSubtract,
    VectorExpressionMul,
    VectorExpressionDivide,
    VectorExpressionInput1,
    VectorExpressionMin,
    VectorExpressionMax,
    VectorExpressionCrossproduct,
}

pub enum ParticleFogType {
    ParticleFogGameDefault,
    ParticleFogEnabled,
    ParticleFogDisabled,
}

pub enum VectorFloatExpressionType {
    VectorFloatExpressionUninitialized,
    VectorFloatExpressionDotproduct,
    VectorFloatExpressionDistance,
    VectorFloatExpressionDistancesqr,
    VectorFloatExpressionInput1Length,
    VectorFloatExpressionInput1Lengthsqr,
    VectorFloatExpressionInput1Noise,
}

pub enum ParticlePinDistance {
    ParticlePinDistanceNone,
    ParticlePinDistanceNeighbor,
    ParticlePinDistanceFarthest,
    ParticlePinDistanceFirst,
    ParticlePinDistanceLast,
    ParticlePinDistanceCenter,
    ParticlePinDistanceCp,
    ParticlePinDistanceCpPairEither,
    ParticlePinDistanceCpPairBoth,
    ParticlePinSpeed,
    ParticlePinCollectionAge,
    ParticlePinFloatValue,
}

pub enum ParticleControlPointAxis {
    ParticleCpAxisX,
    ParticleCpAxisY,
    ParticleCpAxisZ,
    ParticleCpAxisNegativeX,
    ParticleCpAxisNegativeY,
    ParticleCpAxisNegativeZ,
}

pub enum ParticleHitboxBiasType {
    ParticleHitboxBiasEntity,
    ParticleHitboxBiasHitbox,
}

pub enum ParticleDirectionNoiseType {
    ParticleDirNoisePerlin,
    ParticleDirNoiseCurl,
    ParticleDirNoiseWorleyBasic,
}

pub enum PfNoiseType {
    PfNoiseTypePerlin,
    PfNoiseTypeSimplex,
    PfNoiseTypeWorley,
    PfNoiseTypeCurl,
}

pub enum ParticleLightingQuality {
    ParticleLightingPerParticle,
    ParticleLightingPerVertex,
    ParticleLightingPerPixel,
}

pub enum ParticleModelType {
    PmTypeInvalid,
    PmTypeNamedValueModel,
    PmTypeNamedValueEhandle,
    PmTypeControlPoint,
    PmTypeCount,
}

pub enum ParticleParentSetMode {
    ParticleSetParentNo,
    ParticleSetParentImmediate,
    ParticleSetParentRoot,
}

pub enum StandardLightingAttenuationStyle {
    LightStyleOld,
    LightStyleNew,
}

pub enum BlurFilterType {
    BlurfilterGaussian,
    BlurfilterBox,
}

pub enum SpriteCardPerParticleScale {
    SpritecardTexturePpScaleNone,
    SpritecardTexturePpScaleParticleAge,
    SpritecardTexturePpScaleAnimationFrame,
    SpritecardTexturePpScaleShaderExtraData1,
    SpritecardTexturePpScaleShaderExtraData2,
    SpritecardTexturePpScaleParticleAlpha,
    SpritecardTexturePpScaleShaderRadius,
    SpritecardTexturePpScaleRoll,
    SpritecardTexturePpScaleYaw,
    SpritecardTexturePpScalePitch,
    SpritecardTexturePpScaleRandom,
    SpritecardTexturePpScaleNegRandom,
    SpritecardTexturePpScaleRandomTime,
    SpritecardTexturePpScaleNegRandomTime,
}

pub enum ScalarExpressionType {
    ScalarExpressionUninitialized,
    ScalarExpressionAdd,
    ScalarExpressionSubtract,
    ScalarExpressionMul,
    ScalarExpressionDivide,
    ScalarExpressionInput1,
    ScalarExpressionMin,
    ScalarExpressionMax,
    ScalarExpressionMod,
}

pub enum DetailCombo {
    DetailComboOff,
    DetailComboAdd,
    DetailComboAddSelfIllum,
    DetailComboMod2X,
}

pub enum ParticleColorBlendType {
    ParticleColorBlendMultiply,
    ParticleColorBlendMultiply2X,
    ParticleColorBlendDivide,
    ParticleColorBlendAdd,
    ParticleColorBlendSubtract,
    ParticleColorBlendMod2X,
    ParticleColorBlendScreen,
    ParticleColorBlendMax,
    ParticleColorBlendMin,
    ParticleColorBlendReplace,
    ParticleColorBlendAverage,
    ParticleColorBlendNegate,
    ParticleColorBlendLuminance,
}

pub enum ParticleColorBlendMode {
    ParticleblendDefault,
    ParticleblendOverlay,
    ParticleblendDarken,
    ParticleblendLighten,
    ParticleblendMultiply,
}

pub enum PfNoiseTurbulence {
    PfNoiseTurbNone,
    PfNoiseTurbHighlight,
    PfNoiseTurbFeedback,
    PfNoiseTurbLoopy,
    PfNoiseTurbContrast,
    PfNoiseTurbAlternate,
}

pub enum ParticlePostProcessPriorityGroup {
    ParticlePostProcessPriorityLevelVolume,
    ParticlePostProcessPriorityLevelOverride,
    ParticlePostProcessPriorityGameplayEffect,
    ParticlePostProcessPriorityGameplayStateLow,
    ParticlePostProcessPriorityGameplayStateHigh,
    ParticlePostProcessPriorityGlobalUi,
}

pub enum ParticleFloatType {
    PfTypeInvalid,
    PfTypeLiteral,
    PfTypeNamedValue,
    PfTypeRandomUniform,
    PfTypeRandomBiased,
    PfTypeCollectionAge,
    PfTypeEndcapAge,
    PfTypeControlPointComponent,
    PfTypeControlPointChangeAge,
    PfTypeControlPointSpeed,
    PfTypeParticleDetailLevel,
    PfTypeConcurrentDefCount,
    PfTypeClosestCameraDistance,
    PfTypeRendererCameraDistance,
    PfTypeRendererCameraDotProduct,
    PfTypeParticleNoise,
    PfTypeParticleAge,
    PfTypeParticleAgeNormalized,
    PfTypeParticleFloat,
    PfTypeParticleVectorComponent,
    PfTypeParticleSpeed,
    PfTypeParticleNumber,
    PfTypeParticleNumberNormalized,
    PfTypeCount,
}

pub enum InheritableBoolType {
    InheritableBoolInherit,
    InheritableBoolFalse,
    InheritableBoolTrue,
}

pub enum PetGroundType {
    PetGroundNone,
    PetGroundGrid,
    PetGroundPlane,
}

pub enum ParticleVrHandChoiceList {
    ParticleVrhandLeft,
    ParticleVrhandRight,
    ParticleVrhandCp,
    ParticleVrhandCpObject,
}

pub enum PFuncVisualizationType {
    PfuncVisualizationSphereWireframe,
    PfuncVisualizationSphereSolid,
    PfuncVisualizationBox,
    PfuncVisualizationRing,
    PfuncVisualizationPlane,
    PfuncVisualizationLine,
    PfuncVisualizationCylinder,
}

pub enum ParticleTraceMissBehavior {
    ParticleTraceMissBehaviorNone,
    ParticleTraceMissBehaviorKill,
    ParticleTraceMissBehaviorTraceEnd,
}

pub enum ParticleFloatBiasType {
    PfBiasTypeInvalid,
    PfBiasTypeStandard,
    PfBiasTypeGain,
    PfBiasTypeExponential,
    PfBiasTypeCount,
}

pub enum MissingParentInheritBehavior {
    MissingParentDoNothing,
    MissingParentKill,
    MissingParentFindNew,
    MissingParentSameIndex,
}

pub enum Detail2Combo {
    Detail2ComboUninitialized,
    Detail2ComboOff,
    Detail2ComboAdd,
    Detail2ComboAddSelfIllum,
    Detail2ComboMod2X,
    Detail2ComboMul,
    Detail2ComboCrossfade,
}

pub enum ParticleSetMethod {
    ParticleSetReplaceValue,
    ParticleSetScaleInitialValue,
    ParticleSetAddToInitialValue,
    ParticleSetRampCurrentValue,
    ParticleSetScaleCurrentValue,
    ParticleSetAddToCurrentValue,
}

pub enum EStyleNodeType {
    Root,
    Expression,
    Property,
    Define,
    Import,
    Keyframes,
    KeyframeSelector,
    StyleSelector,
    Whitespace,
    ExpressionText,
    ExpressionUrl,
    ExpressionConcat,
    ReferenceContent,
    ReferenceCompiled,
    ReferencePassthrough,
}

pub enum ELayoutNodeType {
    Root,
    Styles,
    ScriptBody,
    Scripts,
    Snippets,
    Include,
    Snippet,
    Panel,
    PanelAttribute,
    PanelAttributeValue,
    ReferenceContent,
    ReferenceCompiled,
    ReferencePassthrough,
}

pub enum MoveType {
    MovetypeNone,
    MovetypeObsolete,
    MovetypeWalk,
    MovetypeStep,
    MovetypeFly,
    MovetypeFlygravity,
    MovetypeVphysics,
    MovetypePush,
    MovetypeNoclip,
    MovetypeObserver,
    MovetypeLadder,
    MovetypeCustom,
    MovetypeLast,
    MovetypeMaxBits,
}

pub enum EOverrideBlockLos {
    BlockLosDefault,
    BlockLosForceFalse,
    BlockLosForceTrue,
}

pub enum FuncDoorSpawnPos {
    FuncDoorSpawnClosed,
    FuncDoorSpawnOpen,
}

pub enum ObserverMode {
    ObsModeNone,
    ObsModeFixed,
    ObsModeInEye,
    ObsModeChase,
    ObsModeRoaming,
    ObsModeDirected,
    NumObserverModes,
}

pub enum MedalRank {
    MedalRankNone,
    MedalRankBronze,
    MedalRankSilver,
    MedalRankGold,
    MedalRankCount,
}

pub enum CanPlaySequence {
    CannotPlay,
    CanPlayNow,
    CanPlayEnqueued,
}

pub enum Disposition {
    DEr,
    DHt,
    DFr,
    DLi,
    DNu,
    DError,
    DHate,
    DFear,
    DLike,
    DNeutral,
}

pub enum ValueRemapperHapticsType {
    HaticsTypeDefault,
    HaticsTypeNone,
}

pub enum TakeDamageFlags {
    DflagNone,
    DflagSuppressHealthChanges,
    DflagSuppressPhysicsForce,
    DflagSuppressEffects,
    DflagPreventDeath,
    DflagForceDeath,
    DflagAlwaysGib,
    DflagNeverGib,
    DflagRemoveNoRagdoll,
    DflagSuppressDamageModification,
    DflagAlwaysFireDamageEvents,
    DflagRadiusDmg,
    DmgLastdflag,
    DflagIgnoreArmor,
}

pub enum WeaponSound {
    WeaponSoundEmpty,
    WeaponSoundSecondaryEmpty,
    WeaponSoundSingle,
    WeaponSoundSecondaryAttack,
    WeaponSoundReload,
    WeaponSoundMeleeMiss,
    WeaponSoundMeleeHit,
    WeaponSoundMeleeHitWorld,
    WeaponSoundMeleeHitPlayer,
    WeaponSoundMeleeHitNpc,
    WeaponSoundSpecial1,
    WeaponSoundSpecial2,
    WeaponSoundSpecial3,
    WeaponSoundNearlyempty,
    WeaponSoundImpact,
    WeaponSoundReflect,
    WeaponSoundSecondaryImpact,
    WeaponSoundSecondaryReflect,
    WeaponSoundSingleAccurate,
    WeaponSoundZoomIn,
    WeaponSoundZoomOut,
    WeaponSoundMousePressed,
    WeaponSoundDrop,
    WeaponSoundRadioUse,
    WeaponSoundNumTypes,
}

pub enum KillTypes {
    KillNone,
    KillDefault,
    KillHeadshot,
    KillBlast,
    KillBurn,
    KillSlash,
    KillShock,
    KilltypeCount,
}

pub enum ValueRemapperInputType {
    InputTypePlayerShootPosition,
    InputTypePlayerShootPositionAroundAxis,
}

pub enum GrenadeType {
    GrenadeTypeExplosive,
    GrenadeTypeFlash,
    GrenadeTypeFire,
    GrenadeTypeDecoy,
    GrenadeTypeSmoke,
    GrenadeTypeSensor,
    GrenadeTypeSnowball,
    GrenadeTypeTotal,
}

pub enum ShadowType {
    ShadowsNone,
    ShadowsSimple,
}

pub enum ModifyDamageReturn {
    ContinueToApplyDamage,
    AbortDoNotApplyDamage,
}

pub enum QuestProgressReason {
    QuestNoninitialized,
    QuestOk,
    QuestNotEnoughPlayers,
    QuestWarmup,
    QuestNotConnectedToSteam,
    QuestNonofficialServer,
    QuestNoEntitlement,
    QuestNoQuest,
    QuestPlayerIsBot,
    QuestWrongMap,
    QuestWrongMode,
    QuestNotSyncedWithServer,
    QuestReasonMax,
}

pub enum BrushSoliditiesE {
    BrushsolidToggle,
    BrushsolidNever,
    BrushsolidAlways,
}

pub enum TrainCode {
    TrainSafe,
    TrainBlocking,
    TrainFollowing,
}

pub enum ShakeCommand {
    ShakeStart,
    ShakeStop,
    ShakeAmplitude,
    ShakeFrequency,
    ShakeStartRumbleonly,
    ShakeStartNorumble,
}

pub enum PointWorldTextJustifyHorizontal {
    PointWorldTextJustifyHorizontalLeft,
    PointWorldTextJustifyHorizontalCenter,
    PointWorldTextJustifyHorizontalRight,
}

pub enum PointTemplateOwnerSpawnGroupType {
    InsertIntoPointTemplateSpawnGroup,
    InsertIntoCurrentlyActiveSpawnGroup,
    InsertIntoNewlyCreatedSpawnGroup,
}

pub enum ValueRemapperOutputType {
    OutputTypeAnimationCycle,
    OutputTypeRotationX,
    OutputTypeRotationY,
    OutputTypeRotationZ,
}

pub enum ChoreoState {
    StatePreScript,
    StateWaitForScript,
    StateWalkToMark,
    StateSynchronizeScript,
    StatePlayScript,
    StatePlayScriptPostIdle,
    StatePlayScriptPostIdleDone,
}

pub enum MoveCollide {
    MovecollideDefault,
    MovecollideFlyBounce,
    MovecollideFlyCustom,
    MovecollideFlySlide,
    MovecollideCount,
    MovecollideMaxBits,
}

pub enum AttributeProviderTypes {
    ProviderGeneric,
    ProviderWeapon,
}

pub enum PointWorldTextJustifyVertical {
    PointWorldTextJustifyVerticalBottom,
    PointWorldTextJustifyVerticalCenter,
    PointWorldTextJustifyVerticalTop,
}

pub enum DamageTypes {
    DmgGeneric,
    DmgCrush,
    DmgBullet,
    DmgSlash,
    DmgBurn,
    DmgVehicle,
    DmgFall,
    DmgBlast,
    DmgClub,
    DmgShock,
    DmgSonic,
    DmgEnergybeam,
    DmgDrown,
    DmgPoison,
    DmgRadiation,
    DmgDrownrecover,
    DmgAcid,
    DmgPhysgun,
    DmgDissolve,
    DmgBlastSurface,
    DmgBuckshot,
    DmgLastgenericflag,
    DmgHeadshot,
    DmgDangerzone,
}

pub enum SolidType {
    SolidNone,
    SolidBsp,
    SolidBbox,
    SolidObb,
    SolidSphere,
    SolidPoint,
    SolidVphysics,
    SolidCapsule,
    SolidLast,
}

pub enum Navproperties {
    NavIgnore,
}

pub enum SimpleConstraintsSoundProfileKeypoints {
    KMinThreshold,
    KMinFull,
    KHighwater,
}

pub enum PlayerConnectedState {
    PlayerNeverConnected,
    PlayerConnected,
    PlayerConnecting,
    PlayerReconnecting,
    PlayerDisconnecting,
    PlayerDisconnected,
    PlayerReserved,
}

pub enum ChatIgnoreType {
    ChatIgnoreNone,
    ChatIgnoreAll,
    ChatIgnoreTeam,
}

pub enum DoorState {
    DoorStateClosed,
    DoorStateOpening,
    DoorStateOpen,
    DoorStateClosing,
    DoorStateAjar,
}

pub enum LatchDirtyPermission {
    LatchDirtyDisallow,
    LatchDirtyServerControlled,
    LatchDirtyClientSimulated,
    LatchDirtyPrediction,
    LatchDirtyFramesimulate,
    LatchDirtyParticleSimulate,
}

pub enum RumbleEffect {
    RumbleInvalid,
    RumbleStopAll,
    RumblePistol,
    Rumble357,
    RumbleSmg1,
    RumbleAr2,
    RumbleShotgunSingle,
    RumbleShotgunDouble,
    RumbleAr2AltFire,
    RumbleRpgMissile,
    RumbleCrowbarSwing,
    RumbleAirboatGun,
    RumbleJeepEngineLoop,
    RumbleFlatLeft,
    RumbleFlatRight,
    RumbleFlatBoth,
    RumbleDmgLow,
    RumbleDmgMed,
    RumbleDmgHigh,
    RumbleFallLong,
    RumbleFallShort,
    RumblePhyscannonOpen,
    RumblePhyscannonPunt,
    RumblePhyscannonLow,
    RumblePhyscannonMedium,
    RumblePhyscannonHigh,
    NumRumbleEffects,
}

pub enum VoteCreateFailed {
    VoteFailedGeneric,
    VoteFailedTransitioningPlayers,
    VoteFailedRateExceeded,
    VoteFailedYesMustExceedNo,
    VoteFailedQuorumFailure,
    VoteFailedIssueDisabled,
    VoteFailedMapNotFound,
    VoteFailedMapNameRequired,
    VoteFailedFailedRecently,
    VoteFailedTeamCantCall,
    VoteFailedWaitingforplayers,
    VoteFailedPlayernotfound,
    VoteFailedCannotKickAdmin,
    VoteFailedScrambleInProgress,
    VoteFailedSpectator,
    VoteFailedFailedRecentKick,
    VoteFailedFailedRecentChangemap,
    VoteFailedFailedRecentSwapteams,
    VoteFailedFailedRecentScrambleteams,
    VoteFailedFailedRecentRestart,
    VoteFailedSwapInProgress,
    VoteFailedDisabled,
    VoteFailedNextlevelSet,
    VoteFailedTooEarlySurrender,
    VoteFailedMatchPaused,
    VoteFailedMatchNotPaused,
    VoteFailedNotInWarmup,
    VoteFailedNot10Players,
    VoteFailedTimeoutActive,
    VoteFailedTimeoutInactive,
    VoteFailedTimeoutExhausted,
    VoteFailedCantRoundEnd,
    VoteFailedRematch,
    VoteFailedContinue,
    VoteFailedMax,
}

pub enum RenderFx {
    KRenderFxNone,
    KRenderFxPulseSlow,
    KRenderFxPulseFast,
    KRenderFxPulseSlowWide,
    KRenderFxPulseFastWide,
    KRenderFxFadeSlow,
    KRenderFxFadeFast,
    KRenderFxSolidSlow,
    KRenderFxSolidFast,
    KRenderFxStrobeSlow,
    KRenderFxStrobeFast,
    KRenderFxStrobeFaster,
    KRenderFxFlickerSlow,
    KRenderFxFlickerFast,
    KRenderFxNoDissipation,
    KRenderFxFadeOut,
    KRenderFxFadeIn,
    KRenderFxPulseFastWider,
    KRenderFxGlowShell,
    KRenderFxMax,
}

pub enum MoveMountingAmount {
    MoveMountNone,
    MoveMountLow,
    MoveMountHigh,
    MoveMountMaxcount,
}

pub enum ResponseEnum {
    MaxResponseName,
    MaxRuleName,
}

pub enum NavDirType {
    North,
    East,
    South,
    West,
    NumNavDirTypeDirections,
}

pub enum ValueRemapperRatchetType {
    RatchetTypeAbsolute,
    RatchetTypeEachEngage,
}

pub enum TrackOrientationType {
    TrackOrientationFixed,
    TrackOrientationFacePath,
    TrackOrientationFacePathAngles,
}

pub enum ShatterGlassStressType {
    ShatterglassBlunt,
    ShatterglassBallistic,
    ShatterglassPulse,
    ShatterdrywallChunks,
    ShatterglassExplosive,
}

pub enum PointTemplateClientOnlyEntityBehavior {
    CreateForCurrentlyConnectedClientsOnly,
    CreateForClientsWhoConnectLater,
}

pub enum EntitySubclassScope {
    SubclassScopeNone,
    SubclassScopePrecipitation,
    SubclassScopePlayerWeapons,
    SubclassScopeCount,
}

pub enum BeamType {
    BeamInvalid,
    BeamPoints,
    BeamEntpoint,
    BeamEnts,
    BeamHose,
    BeamSpline,
    BeamLaser,
}

pub enum DoorCheckE {
    DoorCheckForward,
    DoorCheckBackward,
    DoorCheckFull,
}

pub enum HierarchyType {
    HierarchyNone,
    HierarchyBoneMerge,
    HierarchyAttachment,
    HierarchyAbsorigin,
    HierarchyBone,
    HierarchyTypeCount,
}

pub enum AmmoFlags {
    AmmoForceDropIfCarried,
    AmmoReserveStaysWithWeapon,
    AmmoFlagMax,
}

pub enum DebugOverlayBits {
    OverlayTextBit,
    OverlayNameBit,
    OverlayBboxBit,
    OverlayPivotBit,
    OverlayMessageBit,
    OverlayAbsboxBit,
    OverlayRboxBit,
    OverlayShowBlockslos,
    OverlayAttachmentsBit,
    OverlayInterpolatedAttachmentsBit,
    OverlayInterpolatedPivotBit,
    OverlaySkeletonBit,
    OverlayInterpolatedSkeletonBit,
    OverlayTriggerBoundsBit,
    OverlayHitboxBit,
    OverlayInterpolatedHitboxBit,
    OverlayAutoaimBit,
    OverlayNpcSelectedBit,
    OverlayJointInfoBit,
    OverlayNpcRouteBit,
    OverlayNpcTriangulateBit,
    OverlayNpcZapBit,
    OverlayNpcEnemiesBit,
    OverlayNpcConditionsBit,
    OverlayNpcCombatBit,
    OverlayNpcTaskBit,
    OverlayNpcBodylocations,
    OverlayNpcViewconeBit,
    OverlayNpcKillBit,
    OverlayWcChangeEntity,
    OverlayBuddhaMode,
    OverlayNpcSteeringRegulations,
    OverlayNpcTaskTextBit,
    OverlayPropDebug,
    OverlayNpcRelationBit,
    OverlayViewoffset,
    OverlayVcollideWireframeBit,
    OverlayNpcNearestNodeBit,
    OverlayActornameBit,
    OverlayNpcConditionsTextBit,
}

pub enum PointWorldTextReorientMode {
    PointWorldTextReorientNone,
    PointWorldTextReorientAroundUp,
}

pub enum ChickenActivity {
    Idle,
    Walk,
    Run,
    Hop,
    Jump,
    Glide,
    Land,
}

pub enum HitGroup {
    HitgroupInvalid,
    HitgroupGeneric,
    HitgroupHead,
    HitgroupChest,
    HitgroupStomach,
    HitgroupLeftarm,
    HitgroupRightarm,
    HitgroupLeftleg,
    HitgroupRightleg,
    HitgroupNeck,
    HitgroupUnused,
    HitgroupGear,
    HitgroupSpecial,
    HitgroupCount,
}

pub enum InputBitMask {
    InNone,
    InAll,
    InAttack,
    InJump,
    InDuck,
    InForward,
    InBack,
    InUse,
    InTurnleft,
    InTurnright,
    InMoveleft,
    InMoveright,
    InAttack2,
    InReload,
    InSpeed,
    InJoyautosprint,
    InFirstModSpecificBit,
    InUseorreload,
    InScore,
    InZoom,
    InLookAtWeapon,
}

pub enum EntityDisolveType {
    EntityDissolveInvalid,
    EntityDissolveNormal,
    EntityDissolveElectrical,
    EntityDissolveElectricalLight,
    EntityDissolveCore,
}

pub enum AmmoPosition {
    AmmoPositionInvalid,
    AmmoPositionPrimary,
    AmmoPositionSecondary,
    AmmoPositionCount,
}

pub enum WorldTextPanelVerticalAlign {
    WorldtextVerticalAlignTop,
    WorldtextVerticalAlignCenter,
    WorldtextVerticalAlignBottom,
}

pub enum WaterLevel {
    WlNotInWater,
    WlFeet,
    WlKnees,
    WlWaist,
    WlChest,
    WlFullyUnderwater,
    WlCount,
}

pub enum ScriptedConflictResponse {
    SsConflictEnqueue,
    SsConflictInterrupt,
}

pub enum CsPlayerState {
    StateActive,
    StateWelcome,
    StatePickingteam,
    StatePickingclass,
    StateDeathAnim,
    StateDeathWaitForKey,
    StateObserverMode,
    StateGungameRespawn,
    StateDormant,
    NumPlayerStates,
}

pub enum GearSlot {
    GearSlotInvalid,
    GearSlotRifle,
    GearSlotPistol,
    GearSlotKnife,
    GearSlotGrenades,
    GearSlotC4,
    GearSlotReservedSlot6,
    GearSlotReservedSlot7,
    GearSlotReservedSlot8,
    GearSlotReservedSlot9,
    GearSlotReservedSlot10,
    GearSlotReservedSlot11,
    GearSlotBoosts,
    GearSlotUtility,
    GearSlotCount,
    GearSlotFirst,
    GearSlotLast,
}

pub enum ScriptedMoveType {
    ScriptedMovetypeNone,
    ScriptedMovetypeToWithDuration,
    ScriptedMovetypeToWithMovespeed,
    ScriptedMovetypeSweepToAtMovementSpeed,
}

pub enum ScriptedOnDeath {
    SsOndeathNotApplicable,
    SsOndeathUndefined,
    SsOndeathRagdoll,
    SsOndeathAnimatedDeath,
}

pub enum ShatterDamageCause {
    ShatterdamageBullet,
    ShatterdamageMelee,
    ShatterdamageThrown,
    ShatterdamageScript,
    ShatterdamageExplosive,
}

pub enum CommandExecMode {
    ExecManual,
    ExecLevelstart,
    ExecPeriodic,
    ExecModesCount,
}

pub enum CsWeaponState {
    WeaponNotCarried,
    WeaponIsCarriedByPlayer,
    WeaponIsActive,
}

pub enum CsPlayerBlockingUseAction {
    KCsPlayerBlockingUseActionNone,
    KCsPlayerBlockingUseActionDefusingDefault,
    KCsPlayerBlockingUseActionDefusingWithKit,
    KCsPlayerBlockingUseActionHostageGrabbing,
    KCsPlayerBlockingUseActionHostageDropping,
    KCsPlayerBlockingUseActionOpeningSafe,
    KCsPlayerBlockingUseActionEquippingParachute,
    KCsPlayerBlockingUseActionEquippingHeavyArmor,
    KCsPlayerBlockingUseActionEquippingContract,
    KCsPlayerBlockingUseActionEquippingTabletUpgrade,
    KCsPlayerBlockingUseActionTakingOffHeavyArmor,
    KCsPlayerBlockingUseActionPayingToOpenDoor,
    KCsPlayerBlockingUseActionCancelingSpawnRappelling,
    KCsPlayerBlockingUseActionEquippingExoJump,
    KCsPlayerBlockingUseActionPickingUpBumpMine,
    KCsPlayerBlockingUseActionMapLongUseEntityPickup,
    KCsPlayerBlockingUseActionMapLongUseEntityPlace,
    KCsPlayerBlockingUseActionMaxCount,
}

pub enum WeaponAttackType {
    EInvalid,
    EPrimary,
    ESecondary,
    ECount,
}

pub enum BeamClipStyle {
    KNoclip,
    KGeoclip,
    KModelclip,
    KBeamclipstyleNumbits,
}

pub enum EInButtonState {
    InButtonUp,
    InButtonDown,
    InButtonDownUp,
    InButtonUpDown,
    InButtonUpDownUp,
    InButtonDownUpDown,
    InButtonDownUpDownUp,
    InButtonUpDownUpDown,
    InButtonStateCount,
}

pub enum Hull {
    HullHuman,
    HullSmallCentered,
    HullWideHuman,
    HullTiny,
    HullMedium,
    HullTinyCentered,
    HullLarge,
    HullLargeCentered,
    HullMediumTall,
    HullSmall,
    NumHulls,
    HullNone,
}

pub enum ValueRemapperMomentumType {
    MomentumTypeNone,
    MomentumTypeFriction,
    MomentumTypeSpringTowardSnapValue,
    MomentumTypeSpringAwayFromSnapValue,
}

pub enum MoveLinearAuthoredPos {
    MovelinearAuthoredAtStartPosition,
    MovelinearAuthoredAtOpenPosition,
    MovelinearAuthoredAtClosedPosition,
}

pub enum NavAttributeEnum {
    NavMeshAvoid,
    NavMeshStairs,
    NavMeshNonZup,
    NavMeshShortHeight,
    NavMeshCrouch,
    NavMeshJump,
    NavMeshPrecise,
    NavMeshNoJump,
    NavMeshStop,
    NavMeshRun,
    NavMeshWalk,
    NavMeshTransient,
    NavMeshDontHide,
    NavMeshStand,
    NavMeshNoHostages,
    NavMeshNoMerge,
    NavMeshObstacleTop,
    NavAttrFirstGameIndex,
    NavAttrLastIndex,
}

pub enum LoadoutSlot {
    LoadoutSlotInvalid,
    LoadoutSlotMelee,
    LoadoutSlotC4,
    LoadoutSlotFirstAutoBuyWeapon,
    LoadoutSlotLastAutoBuyWeapon,
    LoadoutSlotSecondary0,
    LoadoutSlotSecondary1,
    LoadoutSlotSecondary2,
    LoadoutSlotSecondary3,
    LoadoutSlotSecondary4,
    LoadoutSlotSecondary5,
    LoadoutSlotSmg0,
    LoadoutSlotSmg1,
    LoadoutSlotSmg2,
    LoadoutSlotSmg3,
    LoadoutSlotSmg4,
    LoadoutSlotSmg5,
    LoadoutSlotRifle0,
    LoadoutSlotRifle1,
    LoadoutSlotRifle2,
    LoadoutSlotRifle3,
    LoadoutSlotRifle4,
    LoadoutSlotRifle5,
    LoadoutSlotHeavy0,
    LoadoutSlotHeavy1,
    LoadoutSlotHeavy2,
    LoadoutSlotHeavy3,
    LoadoutSlotHeavy4,
    LoadoutSlotHeavy5,
    LoadoutSlotFirstWheelWeapon,
    LoadoutSlotLastWheelWeapon,
    LoadoutSlotFirstPrimaryWeapon,
    LoadoutSlotLastPrimaryWeapon,
    LoadoutSlotFirstWheelGrenade,
    LoadoutSlotGrenade0,
    LoadoutSlotGrenade1,
    LoadoutSlotGrenade2,
    LoadoutSlotGrenade3,
    LoadoutSlotGrenade4,
    LoadoutSlotGrenade5,
    LoadoutSlotLastWheelGrenade,
    LoadoutSlotEquipment0,
    LoadoutSlotEquipment1,
    LoadoutSlotEquipment2,
    LoadoutSlotEquipment3,
    LoadoutSlotEquipment4,
    LoadoutSlotEquipment5,
    LoadoutSlotFirstWheelEquipment,
    LoadoutSlotLastWheelEquipment,
    LoadoutSlotClothingCustomplayer,
    LoadoutSlotPet,
    LoadoutSlotClothingFacemask,
    LoadoutSlotClothingHands,
    LoadoutSlotFirstCosmetic,
    LoadoutSlotLastCosmetic,
    LoadoutSlotClothingEyewear,
    LoadoutSlotClothingHat,
    LoadoutSlotClothingLowerbody,
    LoadoutSlotClothingTorso,
    LoadoutSlotClothingAppearance,
    LoadoutSlotMisc0,
    LoadoutSlotMisc1,
    LoadoutSlotMisc2,
    LoadoutSlotMisc3,
    LoadoutSlotMisc4,
    LoadoutSlotMisc5,
    LoadoutSlotMisc6,
    LoadoutSlotMusickit,
    LoadoutSlotFlair0,
    LoadoutSlotSpray0,
    LoadoutSlotFirstAllCharacter,
    LoadoutSlotLastAllCharacter,
    LoadoutSlotCount,
}

pub enum ToggleState {
    TsAtTop,
    TsAtBottom,
    TsGoingUp,
    TsGoingDown,
    DoorOpen,
    DoorClosed,
    DoorOpening,
    DoorClosing,
}

pub enum PerformanceMode {
    PmNormal,
    PmNoGibs,
    PmFullGibs,
    PmReducedGibs,
}

pub enum ForcedCrouchState {
    ForcedcrouchNone,
    ForcedcrouchCrouched,
    ForcedcrouchUncrouched,
}

pub enum RenderMode {
    KRenderNormal,
    KRenderTransColor,
    KRenderTransTexture,
    KRenderGlow,
    KRenderTransAlpha,
    KRenderTransAdd,
    KRenderEnvironmental,
    KRenderTransAddFrameBlend,
    KRenderTransAlphaAdd,
    KRenderWorldGlow,
    KRenderNone,
    KRenderDevVisualizer,
    KRenderModeCount,
}

pub enum ShardSolid {
    ShardSolid,
    ShardDebris,
}

pub enum PropDoorRotatingSpawnPos {
    DoorSpawnClosed,
    DoorSpawnOpenForward,
    DoorSpawnOpenBack,
    DoorSpawnAjar,
}

pub enum EntFinderMethod {
    EntFindMethodNearest,
    EntFindMethodFarthest,
    EntFindMethodRandom,
}

pub enum CsWeaponType {
    WeapontypeKnife,
    WeapontypePistol,
    WeapontypeSubmachinegun,
    WeapontypeRifle,
    WeapontypeShotgun,
    WeapontypeSniperRifle,
    WeapontypeMachinegun,
    WeapontypeC4,
    WeapontypeTaser,
    WeapontypeGrenade,
    WeapontypeEquipment,
    WeapontypeStackableitem,
    WeapontypeFists,
    WeapontypeBreachcharge,
    WeapontypeBumpmine,
    WeapontypeTablet,
    WeapontypeMelee,
    WeapontypeShield,
    WeapontypeZoneRepulsor,
    WeapontypeUnknown,
}

pub enum TrainVelocityType {
    TrainVelocityInstantaneous,
    TrainVelocityLinearBlend,
    TrainVelocityEaseInEaseOut,
}

pub enum Touch {
    TouchNone,
    TouchPlayerOnly,
    TouchNpcOnly,
    TouchPlayerOrNpc,
    TouchPlayerOrNpcOrPhysicsprop,
}

pub enum CommandEntitySpecType {
    SpecSearch,
    SpecTypesCount,
}

pub enum ScriptState {
    ScriptPlaying,
    ScriptWait,
    ScriptPostIdle,
    ScriptCleanup,
    ScriptWalkToMark,
    ScriptRunToMark,
    ScriptCustomMoveToMark,
}

pub enum FixAngleSet {
    None,
    Absolute,
    Relative,
}

pub enum GameAnimEventIndex {
    AeEmpty,
    AeClPlaysound,
    AeClPlaysoundAttachment,
    AeClPlaysoundPosition,
    AeSvPlaysound,
    AeClStopsound,
    AeClPlaysoundLooping,
    AeClCreateParticleEffect,
    AeClStopParticleEffect,
    AeClCreateParticleEffectCfg,
    AeSvCreateParticleEffectCfg,
    AeSvStopParticleEffect,
    AeFootstep,
    AeRagdoll,
    AeClStopRagdollControl,
    AeClEnableBodygroup,
    AeClDisableBodygroup,
    AeClBodygroupSetValue,
    AeSvBodygroupSetValue,
    AeClBodygroupSetValueCmodelWpn,
    AeWpnPrimaryattack,
    AeWpnSecondaryattack,
    AeFireInput,
    AeClClothAttr,
    AeClClothGroundOffset,
    AeClClothStiffen,
    AeClClothEffect,
    AeClCreateAnimScopeProp,
    AeClWeaponTransitionIntoHand,
    AeClBodygroupSetToClip,
    AeClBodygroupSetToNextclip,
    AeSvShowSilencer,
    AeSvAttachSilencerComplete,
    AeSvHideSilencer,
    AeSvDetachSilencerComplete,
    AeClEjectMag,
    AeWpnCompleteReload,
    AeWpnHealthshotInject,
    AeClC4ScreenText,
}

pub enum BaseExplosionTypes {
    ExplosionTypeDefault,
    ExplosionTypeGrenade,
    ExplosionTypeMolotov,
    ExplosionTypeFireworks,
    ExplosionTypeGascan,
    ExplosionTypeGascylinder,
    ExplosionTypeExplosivebarrel,
    ExplosionTypeElectrical,
    ExplosionTypeEmp,
    ExplosionTypeShrapnel,
    ExplosionTypeSmokegrenade,
    ExplosionTypeFlashbang,
    ExplosionTypeTripmine,
    ExplosionTypeIce,
    ExplosionTypeNone,
    ExplosionTypeCustom,
    ExplosionTypeCount,
}

pub enum CsWeaponSilencerType {
    WeaponsilencerNone,
    WeaponsilencerDetachable,
    WeaponsilencerIntegrated,
}

pub enum Filter {
    FilterAnd,
    FilterOr,
}

pub enum ClassT {
    ClassNone,
    ClassPlayer,
    ClassPlayerAlly,
    ClassBomb,
    ClassFootContactShadow,
    ClassWeapon,
    ClassWaterSplasher,
    ClassWeaponViewmodel,
    ClassDoor,
    NumClassifyClasses,
}

pub enum SoundFlags {
    SoundNone,
    SoundCombat,
    SoundWorld,
    SoundPlayer,
    SoundDanger,
    SoundBulletImpact,
    SoundThumper,
    SoundPhysicsDanger,
    SoundMoveAway,
    SoundPlayerVehicle,
    SoundGlassBreak,
    SoundPhysicsObject,
    SoundContextGunfire,
    SoundContextCombineOnly,
    SoundContextReactToSource,
    SoundContextExplosion,
    SoundContextExcludeCombine,
    SoundContextDangerApproach,
    SoundContextAlliesOnly,
    SoundContextPanicNpcs,
    AllContexts,
    AllScents,
    AllSounds,
}

pub enum AnimLoopMode {
    AnimLoopModeInvalid,
    AnimLoopModeNotLooping,
    AnimLoopModeLooping,
    AnimLoopModeUseSequenceSettings,
    AnimLoopModeCount,
}

pub enum ScriptedMoveTo {
    CineMovetoWait,
    CineMovetoWalk,
    CineMovetoRun,
    CineMovetoCustom,
    CineMovetoTeleport,
    CineMovetoWaitFacing,
}

pub enum Materials {
    MatGlass,
    MatWood,
    MatMetal,
    MatFlesh,
    MatCinderBlock,
    MatCeilingTile,
    MatComputer,
    MatUnbreakableGlass,
    MatRocks,
    MatWeb,
    MatNone,
    MatLastMaterial,
}

pub enum OnFrame {
    OnframeUnknown,
    OnframeTrue,
    OnframeFalse,
}

pub enum CsWeaponMode {
    PrimaryMode,
    SecondaryMode,
    WeaponModeMax,
}

pub enum LifeState {
    LifeAlive,
    LifeDying,
    LifeDead,
    LifeRespawnable,
    LifeRespawning,
}

pub enum SurroundingBoundsType {
    UseObbCollisionBounds,
    UseBestCollisionBounds,
    UseHitboxes,
    UseSpecifiedBounds,
    UseGameCode,
    UseRotationExpandedBounds,
    UseCollisionBoundsNeverVphysics,
    UseRotationExpandedSequenceBounds,
    SurroundingTypeBitCount,
}

pub enum ShatterPanelMode {
    ShatterGlass,
    ShatterDrywall,
}

pub enum ItemFlagTypes {
    ItemFlagNone,
    ItemFlagCanSelectWithoutAmmo,
    ItemFlagNoautoreload,
    ItemFlagNoautoswitchempty,
    ItemFlagLimitinworld,
    ItemFlagExhaustible,
    ItemFlagDohitlocationdmg,
    ItemFlagNoammopickups,
    ItemFlagNoitempickup,
}

pub enum PlayerAnimEvent {
    PlayeranimeventFireGunPrimary,
    PlayeranimeventFireGunSecondary,
    PlayeranimeventGrenadePullPin,
    PlayeranimeventThrowGrenade,
    PlayeranimeventJump,
    PlayeranimeventReload,
    PlayeranimeventClearFiring,
    PlayeranimeventDeploy,
    PlayeranimeventSilencerState,
    PlayeranimeventSilencerToggle,
    PlayeranimeventThrowGrenadeUnderhand,
    PlayeranimeventCatchWeapon,
    PlayeranimeventLookatweaponRequest,
    PlayeranimeventReloadCancelLookatweapon,
    PlayeranimeventHaulback,
    PlayeranimeventIdle,
    PlayeranimeventStrikeHit,
    PlayeranimeventStrikeMiss,
    PlayeranimeventBackstab,
    PlayeranimeventDryfire,
    PlayeranimeventFidget,
    PlayeranimeventRelease,
    PlayeranimeventTaunt,
    PlayeranimeventCount,
}

pub enum BeginDeathLifeStateTransition {
    NoChangeInLifestate,
    TransitionToLifestateDying,
    TransitionToLifestateDead,
}

pub enum CsWeaponCategory {
    WeaponcategoryOther,
    WeaponcategoryMelee,
    WeaponcategorySecondary,
    WeaponcategorySmg,
    WeaponcategoryRifle,
    WeaponcategoryHeavy,
    WeaponcategoryCount,
}

pub enum TrainOrientationType {
    TrainOrientationFixed,
    TrainOrientationAtPathTracks,
    TrainOrientationLinearBlend,
    TrainOrientationEaseInEaseOut,
}

pub enum SoundEventStartType {
    SoundeventStartPlayer,
    SoundeventStartWorld,
    SoundeventStartEntity,
}

pub enum WorldTextPanelHorizontalAlign {
    WorldtextHorizontalAlignLeft,
    WorldtextHorizontalAlignCenter,
    WorldtextHorizontalAlignRight,
}

pub enum WorldTextPanelOrientation {
    WorldtextOrientationDefault,
    WorldtextOrientationFaceuser,
    WorldtextOrientationFaceuserUpright,
}

pub enum ObserverInterpState {
    ObserverInterpNone,
    ObserverInterpTraveling,
    ObserverInterpSettling,
}

pub enum Explosions {
    ExpRandom,
    ExpDirected,
    ExpUsePrecise,
}

pub enum StanceType {
    StanceCurrent,
    StanceDefault,
    StanceCrouching,
    StanceProne,
    NumStances,
}

pub enum SubclassVDataChangeType {
    SubclassVdataCreated,
    SubclassVdataSubclassChanged,
    SubclassVdataReloaded,
}

pub enum TimelineCompression {
    TimelineCompressionSum,
    TimelineCompressionCountPerInterval,
    TimelineCompressionAverage,
    TimelineCompressionAverageBlend,
    TimelineCompressionTotal,
}

pub enum LessonPanelLayoutFileTypes {
    LayoutHandDefault,
    LayoutWorldDefault,
    LayoutCustom,
}

pub enum EGrenadeThrowState {
    NotThrowing,
    Throwing,
    ThrowComplete,
}

pub enum SceneOnPlayerDeath {
    SceneOnplayerdeathDoNothing,
    SceneOnplayerdeathCancel,
}

pub enum PropDoorRotatingOpenDirectionE {
    DoorRotatingOpenBothWays,
    DoorRotatingOpenForward,
    DoorRotatingOpenBackward,
}

pub enum CompMatPropertyMutatorConditionType {
    CompMatMutatorConditionInputContainerExists,
    CompMatMutatorConditionInputContainerValueExists,
    CompMatMutatorConditionInputContainerValueEquals,
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

pub enum CompositeMaterialInputContainerSourceType {
    ContainerSourceTypeTargetMaterial,
    ContainerSourceTypeMaterialFromTargetAttr,
    ContainerSourceTypeSpecificMaterial,
    ContainerSourceTypeLooseVariables,
    ContainerSourceTypeVariableFromTargetAttr,
    ContainerSourceTypeTargetInstanceMaterial,
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

#[derive(Schema)]
#[scope("!GlobalTypes")]
pub struct GameTick {
    #[field("GameTick_t", "m_Value")]
    value: i32,
}

#[derive(Schema)]
#[scope("!GlobalTypes")]
pub struct GameTime {
    #[field("GameTime_t", "m_Value")]
    value: f32,
}

#[derive(Schema)]
#[scope("!GlobalTypes")]
pub struct NetworkVarChainer {
    #[field("CNetworkVarChainer", "m_PathIndex")]
    path_index: ChangeAccessorFieldPathIndex,
}

#[derive(Schema)]
#[scope("!GlobalTypes")]
pub struct NetworkTransmitComponent {
    #[field("CNetworkTransmitComponent", "m_nTransmitStateOwnedCounter")]
    transmit_state_owned_counter: u8,
}

#[derive(Schema)]
#[scope("!GlobalTypes")]
pub struct Thinkfunc {
    #[field("thinkfunc_t", "m_hFn")]
    func: HScript,
    #[field("thinkfunc_t", "m_nContext")]
    context: UtlStringToken,
    #[field("thinkfunc_t", "m_nNextThinkTick")]
    next_think_tick: GameTick,
    #[field("thinkfunc_t", "m_nLastThinkTick")]
    last_think_tick: GameTick,
}

#[derive(Schema)]
#[scope("!GlobalTypes")]
pub struct NetworkVelocityVector {
    #[field("CNetworkVelocityVector", "m_vecX")]
    x: NetworkedQuantizedFloat,
    #[field("CNetworkVelocityVector", "m_vecY")]
    y: NetworkedQuantizedFloat,
    #[field("CNetworkVelocityVector", "m_vecZ")]
    z: NetworkedQuantizedFloat,
}

#[derive(Schema)]
#[scope("!GlobalTypes")]
pub struct ParticleProperty {}

#[derive(Schema)]
#[scope("!GlobalTypes")]
pub struct NetworkViewOffsetVector {
    #[field("CNetworkViewOffsetVector", "m_vecX")]
    x: NetworkedQuantizedFloat,
    #[field("CNetworkViewOffsetVector", "m_vecY")]
    y: NetworkedQuantizedFloat,
    #[field("CNetworkViewOffsetVector", "m_vecZ")]
    z: NetworkedQuantizedFloat,
}

#[derive(Schema)]
#[scope("!GlobalTypes")]
pub struct ClientAlphaProperty {
    #[field("CClientAlphaProperty", "m_nRenderFX")]
    render_fx: u8,
    #[field("CClientAlphaProperty", "m_nRenderMode")]
    render_mode: u8,
    #[field("CClientAlphaProperty", "m_bAlphaOverride")]
    alpha_override: u8,
    #[field("CClientAlphaProperty", "m_bShadowAlphaOverride")]
    shadow_alpha_override: u8,
    #[field("CClientAlphaProperty", "m_nReserved")]
    reserved: u8,
    #[field("CClientAlphaProperty", "m_nAlpha")]
    alpha: u8,
    #[field("CClientAlphaProperty", "m_nDesyncOffset")]
    desync_offset: u16,
    #[field("CClientAlphaProperty", "m_nReserved2")]
    reserved_2: u16,
    #[field("CClientAlphaProperty", "m_nDistFadeStart")]
    dist_fade_start: u16,
    #[field("CClientAlphaProperty", "m_nDistFadeEnd")]
    dist_fade_end: u16,
    #[field("CClientAlphaProperty", "m_flFadeScale")]
    fade_scale: f32,
    #[field("CClientAlphaProperty", "m_flRenderFxStartTime")]
    render_fx_start_time: GameTime,
    #[field("CClientAlphaProperty", "m_flRenderFxDuration")]
    render_fx_duration: f32,
}

#[derive(Schema)]
#[scope("!GlobalTypes")]
pub struct AttachmentHandle {
    #[field("AttachmentHandle_t", "m_Value")]
    value: u8,
}

#[derive(Schema)]
#[scope("!GlobalTypes")]
pub struct HSequence {
    #[field("HSequence", "m_Value")]
    value: i32,
}

#[derive(Schema)]
#[scope("!GlobalTypes")]
pub struct EntityIoOutput {
    #[field("CEntityIOOutput", "m_Value")]
    value: VariantBase<VariantDefaultAllocator>,
}

#[derive(Schema)]
#[scope("!GlobalTypes")]
pub struct IronSightController {
    #[field("C_IronSightController", "m_bIronSightAvailable")]
    iron_sight_available: bool,
    #[field("C_IronSightController", "m_flIronSightAmount")]
    iron_sight_amount: f32,
    #[field("C_IronSightController", "m_flIronSightAmountGained")]
    iron_sight_amount_gained: f32,
    #[field("C_IronSightController", "m_flIronSightAmountBiased")]
    iron_sight_amount_biased: f32,
    #[field("C_IronSightController", "m_flIronSightAmount_Interpolated")]
    iron_sight_amount_interpolated: f32,
    #[field("C_IronSightController", "m_flIronSightAmountGained_Interpolated")]
    iron_sight_amount_gained_interpolated: f32,
    #[field("C_IronSightController", "m_flIronSightAmountBiased_Interpolated")]
    iron_sight_amount_biased_interpolated: f32,
    #[field("C_IronSightController", "m_flInterpolationLastUpdated")]
    interpolation_last_updated: f32,
    #[field("C_IronSightController", "m_angDeltaAverage")]
    delta_average: [QAngle; 8],
    #[field("C_IronSightController", "m_angViewLast")]
    view_last: QAngle,
    #[field("C_IronSightController", "m_vecDotCoords")]
    dot_coords: Vector2D,
    #[field("C_IronSightController", "m_flDotBlur")]
    dot_blur: f32,
    #[field("C_IronSightController", "m_flSpeedRatio")]
    speed_ratio: f32,
}

#[derive(Schema)]
#[scope("client.dll")]
pub struct CountdownTimer {
    #[field("CountdownTimer", "m_duration")]
    duration: f32,
    #[field("CountdownTimer", "m_timestamp")]
    timestamp: GameTime,
    #[field("CountdownTimer", "m_timescale")]
    timescale: f32,
    #[field("CountdownTimer", "m_nWorldGroupId")]
    world_group_id: WorldGroupId,
}

#[derive(Schema)]
#[scope("!GlobalTypes")]
pub struct ParticleIndex {
    #[field("ParticleIndex_t", "m_Value")]
    value: i32,
}
