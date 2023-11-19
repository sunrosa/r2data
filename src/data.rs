use crate::model::*;
use {CategoryId::*, ItemId::*, RarityId::*};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum SurvivorId {
    Commando,
    Huntress,
    Bandit,
    MulT,
    Engineer,
    Artificer,
    Mercenary,
    Rex,
    Loader,
    Acrid,
    Captain,
    Railgunner,
    VoidFiend,
    Heretic,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum SurvivorLoadout {
    Commando(CommandoLoadout),
    Huntress(HuntressLoadout),
    Bandit(BanditLoadout),
    MulT(MulTLoadout),
    Engineer(EngineerLoadout),
    Artificer(ArtificerLoadout),
    Mercenary(MercenaryLoadout),
    Rex(RexLoadout),
    Loader(LoaderLoadout),
    Acrid(AcridLoadout),
    Captain(CaptainLoadout),
    Railgunner(RailgunnerLoadout),
    VoidFiend(VoidFiendLoadout),
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct CommandoLoadout {
    pub secondary: CommandoSecondaryId,
    pub utility: CommandoUtilityId,
    pub special: CommandoSpecialId,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum CommandoSecondaryId {
    PhaseRound,
    PhaseBlast,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum CommandoUtilityId {
    TacticalDive,
    TacticalSlide,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum CommandoSpecialId {
    SuppressiveFire,
    FragGrenade,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct HuntressLoadout {
    pub primary: HuntressPrimaryId,
    pub utility: HuntressUtilityId,
    pub special: HuntressSpecialId,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum HuntressPrimaryId {
    Strafe,
    Flurry,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum HuntressUtilityId {
    Blink,
    PhaseBlink,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum HuntressSpecialId {
    ArrowRain,
    Ballista,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct BanditLoadout {
    pub primary: BanditPrimaryId,
    pub secondary: BanditSecondaryId,
    pub special: BanditSpecialId,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum BanditPrimaryId {
    Burst,
    Blast,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum BanditSecondaryId {
    SerratedDagger,
    SerratedShiv,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum BanditSpecialId {
    LightsOut,
    Desperado,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct MulTLoadout {
    pub primary1: MulTPrimaryId,
    pub primary2: MulTPrimaryId,
    pub special: MulTSpecialId,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum MulTPrimaryId {
    AutoNailgun,
    RebarPuncher,
    ScrapLauncher,
    PowerSaw,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum MulTSpecialId {
    Retool,
    PowerMode,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct EngineerLoadout {
    pub secondary: EngineerSecondaryId,
    pub utility: EngineerUtilityId,
    pub special: EngineerSpecialId,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum EngineerSecondaryId {
    PressureMines,
    SpiderMines,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum EngineerUtilityId {
    BubbleShield,
    ThermalHarpoons,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum EngineerSpecialId {
    TR12GaussAutoTurret,
    TR58CarbonizerTurret,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ArtificerLoadout {
    pub primary: ArtificerPrimaryId,
    pub secondary: ArtificerSecondaryId,
    pub special: ArtificerSpecialId,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ArtificerPrimaryId {
    FlameBolt,
    PlasmaBolt,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ArtificerSecondaryId {
    ChargedNanoBomb,
    CastNanoSpear,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ArtificerSpecialId {
    Flamethrower,
    IonSurge,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct MercenaryLoadout {
    pub secondary: MercenarySecondaryId,
    pub utility: MercenaryUtilityId,
    pub special: MercenarySpecialId,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum MercenarySecondaryId {
    Whirlwind,
    RisingThunder,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum MercenaryUtilityId {
    BlindingAssault,
    FocusedAssault,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum MercenarySpecialId {
    Eviscerate,
    SlicingWinds,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct RexLoadout {
    pub secondary: RexSecondaryId,
    pub utility: RexUtilityId,
    pub special: RexSpecialId,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum RexSecondaryId {
    DirectiveDrill,
    SeedBarrage,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum RexUtilityId {
    DirectiveDisperse,
    BrambleVolley,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum RexSpecialId {
    DirectiveHarvest,
    TanglingGrowth,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct LoaderLoadout {
    pub secondary: LoaderSecondaryId,
    pub utility: LoaderUtilityId,
    pub special: LoaderSpecialId,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum LoaderSecondaryId {
    GrappleFist,
    SpikedFist,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum LoaderUtilityId {
    ChargedGauntlet,
    ThunderGauntlet,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum LoaderSpecialId {
    M551Pylon,
    Thunderslam,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct AcridLoadout {
    pub misc: AcridMiscId,
    pub secondary: AcridSecondaryId,
    pub utility: AcridUtilityId,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum AcridMiscId {
    Poison,
    Blight,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum AcridSecondaryId {
    Neurotoxin,
    RavenousBite,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum AcridUtilityId {
    CausticLeap,
    FrenziedLeap,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct CaptainLoadout {
    pub utility: CaptainUtilityId,
    pub special1: CaptainSpecialId,
    pub special2: CaptainSpecialId,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum CaptainUtilityId {
    OrbitalProbe,
    OGM72DiabloStrike,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum CaptainSpecialId {
    BeaconHealing,
    BeaconShocking,
    BeaconResupply,
    BeaconHacking,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct RailgunnerLoadout {
    pub secondary: RailgunnerSecondaryId,
    pub utility: RailgunnerUtilityId,
    pub special: RailgunnerSpecialId,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum RailgunnerSecondaryId {
    M99Sniper,
    HH44Marksman,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum RailgunnerUtilityId {
    ConcussionDevice,
    PolarFieldDevice,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum RailgunnerSpecialId {
    Supercharge,
    Cryocharge,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct VoidFiendLoadout {}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum MonsterId {
    AlloyVulture,
    AlphaConstruct,
    Beetle,
    BeetleGuard,
    BighornBison,
    BlindPest,
    BlindVermin,
    BrassContraption,
    ClayApothecary,
    ClayTemplar,
    ElderLemurian,
    Geep,
    Gip,
    GreaterWisp,
    Gup,
    HermitCrab,
    Imp,
    Jellyfish,
    Larva,
    Lemurian,
    LesserWisp,
    LunarExploder,
    LunarGolem,
    LunarWisp,
    MiniMushrum,
    Parent,
    SolusProbe,
    StoneGolem,
    VoidBarnacle,
    VoidJailer,
    VoidReaver,
    BeetleQueen,
    ClayDunestrider,
    Grandparent,
    Grovetender,
    ImpOverlord,
    MagmaWorm,
    OverloadingWorm,
    Scavenger,
    SolusControlUnit,
    StoneTitan,
    VoidDevastator,
    WanderingVagrant,
    XiConstruct,
    HealingCore,
    MalachiteUrchin,
    Newt,
    VoidInfestor,
    AlloyWorshipUnit,
    ArtifactReliquary,
    Aurelionite,
    GuraguraTheLucky,
    KipkipTheGentle,
    Mithrix,
    TwiptwipTheDevotee,
    Voidling,
    WipwipTheWild,
    AlphaConstructAlly,
    AurelioniteAlly,
    BeetleGuardAlly,
    VoidDevastatorAlly,
    VoidJailerAlly,
    VoidReaverAlly,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum RarityId {
    /// Common
    White,
    /// Uncommon
    Green,
    /// Legendary
    Red,
    /// Boss
    Yellow,
    /// Lunar
    Blue,
    /// Void
    Purple,
    /// Equipment
    Orange,
    /// Things like Delicate Watch (Broken) and Empty Bottle
    Untiered,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum CategoryId {
    Damage,
    Healing,
    Utility,
    BrotherBlacklist,
    OnKillEffect,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ItemId {
    SoldiersSyringe,
    TougherTimes,
    MonsterTooth,
    LensMakersGlasses,
    PaulsGoatHoof,
    BustlingFungus,
    Crowbar,
    TriTipDagger,
    Warbanner,
    CautiousSlug,
    PersonalShieldGenerator,
    Medkit,
    Gasoline,
    StunGrenade,
    BundleOfFireworks,
    EnergyDrink,
    BackupMagazine,
    StickyBomb,
    RustedKey,
    ArmorPiercingRounds,
    TopazBrooch,
    FocusCrystal,
    BisonSteak,
    RepulsionArmorPlate,
    Mocha,
    PowerElixir,
    DelicateWatch,
    OddlyShapedOpal,
    RollOfPennies,
    ItemScrapWhite,
    ATGMissileMk1,
    WillOTheWisp,
    HopooFeather,
    Ukulele,
    LeechingSeed,
    PredatoryInstincts,
    RedWhip,
    OldWarStealthkit,
    HarvestersScythe,
    FuelCell,
    Infusion,
    Bandolier,
    BerzerkersPauldron,
    RoseBuckler,
    RunaldsBand,
    KjarosBand,
    Chronobauble,
    WaxQuail,
    OldGuillotine,
    WarHorn,
    LeptonDaisy,
    Razorwire,
    GhorsTome,
    SquidPolyp,
    DeathMark,
    HuntersHarpoon,
    IgnitionTank,
    Shuriken,
    RegeneratingScrap,
    ShippingRequestForm,
    ItemScrapGreen,
    BrilliantBehemoth,
    CeremonialDagger,
    FrostRelic,
    HappiestMask,
    H3AD5Tv2,
    NkuhanasOpinion,
    UnstableTeslaCoil,
    FiftySevenLeafClover,
    SentientMeatHook,
    AlienHead,
    SoulboundCatalyst,
    DiosBestFriend,
    HardlightAfterburner,
    WakeOfVultures,
    Brainstalks,
    RejuvenationRack,
    Aegis,
    ShatteringJustice,
    ResonanceDisc,
    InterstellarDeskPlant,
    DefensiveMicrobots,
    LaserScope,
    PocketICBM,
    SpareDroneParts,
    SymbioticScorpion,
    BensRaincoat,
    BottledChaos,
    ItemScrapRed,
    TitanicKnurl,
    QueensGland,
    HalcyonSeed,
    LittleDisciple,
    Pearl,
    IrradiantPearl,
    GenesisLoop,
    ArtifactKey,
    MoltenPerforator,
    Shatterspleen,
    MiredUrn,
    ChargedPerforator,
    EmpathyCores,
    Planula,
    DefenseNucleus,
    ItemScrapYellow,
    BenthicBloom,
    EncrustedKey,
    LostSeersLenses,
    LysateCell,
    Needletick,
    NewlyHatchedZoea,
    PlasmaShrimp,
    PluripotentLarva,
    Polylute,
    SaferSpaces,
    SingularityBand,
    Tentabauble,
    VoidsentFlame,
    WeepingFungus,
    ShapedGlass,
    BrittleCrown,
    Transcendence,
    Corpsebloom,
    GestureOfTheDrowned,
    StridesOfHeresy,
    VisionsOfHeresy,
    BeadsOfFealty,
    FocusedConvergence,
    DefiantGouge,
    MercurialRachis,
    Purity,
    HooksOfHeresy,
    Egocentrism,
    EulogyZero,
    StoneFluxPauldron,
    LightFluxPauldron,
    EssenceOfHeresy,
    GlowingMeteorite,
    HelfireTincture,
    EffigyOfGrief,
    SpinelTonic,
    DisposableMissileLauncher,
    ForeignFruit,
    PrimordialCube,
    TrophyHuntersTricorn,
    OcularHUD,
    TheBackUp,
    PreonAccumulator,
    GooboJr,
    MilkyChrysalis,
    RoyalCapacitor,
    Molotov6Pack,
    ExecutiveCard,
    TheCrowdfunder,
    GnarledWoodsprite,
    RadarScanner,
    EccentricVase,
    BlastShower,
    VolcanicEgg,
    JadeElephant,
    Sawmerang,
    Recycler,
    SuperMassiveLeech,
    GoragsOpus,
    ForgiveMePlease,
    RemoteCaffeinator,
    IfritsDistinction,
    HisReassurance,
    SilenceBetweenTwoStrikes,
    HerBitingEmbrace,
    NkuhanasRetort,
    SpectralCirclet,
    SharedDesign,
    FuelArray,
    DelicateWatchBroken,
    EmptyBottle,
    DiosBestFriendConsumed,
    RegeneratingScrapConsumed,
}

impl From<ItemId> for Item {
    fn from(id: ItemId) -> Self {
        match id {
            SoldiersSyringe => Item {
                id: SoldiersSyringe,
                name: "Soldier's Syringe".into(),
                rarity: White,
                category: vec![Damage],
            },
            TougherTimes => Item {
                id: TougherTimes,
                name: "Tougher Times".into(),
                rarity: White,
                category: vec![Damage, BrotherBlacklist],
            },
            MonsterTooth => Item {
                id: MonsterTooth,
                name: "Monster Tooth".into(),
                rarity: White,
                category: vec![Healing, OnKillEffect],
            },
            _ => todo!(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum OutcomeId {
    Defeat,
    FateUnknown,
    Victory,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum DifficultyId {
    Drizzle,
    Rainstorm,
    Monsoon,
    Eclipse1,
    Eclipse2,
    Eclipse3,
    Eclipse4,
    Eclipse5,
    Eclipse6,
    Eclipse7,
    Eclipse8,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum EnvironmentId {
    DistantRoost,
    TitanicPlains,
    SiphonedForest,
    AbandonedAqueduct,
    WetlandAspect,
    AphelianSanctuary,
    RallypointDelta,
    ScorchedAcres,
    SulfurPools,
    AbyssalDepths,
    SirensCall,
    SunderedGrove,
    SkyMeadow,
    Commencement,
    AMomentFractured,
    AMomentWhole,
    BazaarBetweenTime,
    BulwarksAmbry,
    GildedCoast,
    VoidFields,
    VoidLocus,
    ThePlanetarium,
}
