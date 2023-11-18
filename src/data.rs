use crate::model::*;
use {CategoryId::*, ItemId::*, RarityId::*};

#[derive(Debug)]
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

#[derive(Debug)]
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

#[derive(Debug)]
pub struct CommandoLoadout {
    pub secondary: CommandoSecondaryId,
    pub utility: CommandoUtilityId,
    pub special: CommandoSpecialId,
}

#[derive(Debug)]
pub enum CommandoSecondaryId {
    PhaseRound,
    PhaseBlast,
}

#[derive(Debug)]
pub enum CommandoUtilityId {
    TacticalDive,
    TacticalSlide,
}

#[derive(Debug)]
pub enum CommandoSpecialId {
    SuppressiveFire,
    FragGrenade,
}

#[derive(Debug)]
pub struct HuntressLoadout {
    pub primary: HuntressPrimaryId,
    pub utility: HuntressUtilityId,
    pub special: HuntressSpecialId,
}

#[derive(Debug)]
pub enum HuntressPrimaryId {
    Strafe,
    Flurry,
}

#[derive(Debug)]
pub enum HuntressUtilityId {
    Blink,
    PhaseBlink,
}

#[derive(Debug)]
pub enum HuntressSpecialId {
    ArrowRain,
    Ballista,
}

#[derive(Debug)]
pub struct BanditLoadout {
    primary: BanditPrimaryId,
    secondary: BanditSecondaryId,
    special: BanditSpecialId,
}

#[derive(Debug)]
pub enum BanditPrimaryId {
    Burst,
    Blast,
}

#[derive(Debug)]
pub enum BanditSecondaryId {
    SerratedDagger,
    SerratedShiv,
}

#[derive(Debug)]
pub enum BanditSpecialId {
    LightsOut,
    Desperado,
}

#[derive(Debug)]
pub struct MulTLoadout {
    primary1: MulTPrimaryId,
    primary2: MulTPrimaryId,
    special: MulTSpecialId,
}

#[derive(Debug)]
pub enum MulTPrimaryId {
    AutoNailgun,
    RebarPuncher,
    ScrapLauncher,
    PowerSaw,
}

#[derive(Debug)]
pub enum MulTSpecialId {
    Retool,
    PowerMode,
}

#[derive(Debug)]
pub struct EngineerLoadout {
    secondary: EngineerSecondaryId,
    utility: EngineerUtilityId,
    special: EngineerSpecialId,
}

#[derive(Debug)]
pub enum EngineerSecondaryId {
    PressureMines,
    SpiderMines,
}

#[derive(Debug)]
pub enum EngineerUtilityId {
    BubbleShield,
    ThermalHarpoons,
}

#[derive(Debug)]
pub enum EngineerSpecialId {
    TR12GaussAutoTurret,
    TR58CarbonizerTurret,
}

#[derive(Debug)]
pub struct ArtificerLoadout {
    primary: ArtificerPrimaryId,
    secondary: ArtificerSecondaryId,
    special: ArtificerSpecialId,
}

#[derive(Debug)]
pub enum ArtificerPrimaryId {
    FlameBolt,
    PlasmaBolt,
}

#[derive(Debug)]
pub enum ArtificerSecondaryId {
    ChargedNanoBomb,
    CastNanoSpear,
}

#[derive(Debug)]
pub enum ArtificerSpecialId {
    Flamethrower,
    IonSurge,
}

#[derive(Debug)]
pub struct MercenaryLoadout {
    secondary: MercenarySecondaryId,
    utility: MercenaryUtilityId,
    special: MercenarySpecialId,
}

#[derive(Debug)]
pub enum MercenarySecondaryId {
    Whirlwind,
    RisingThunder,
}

#[derive(Debug)]
pub enum MercenaryUtilityId {
    BlindingAssault,
    FocusedAssault,
}

#[derive(Debug)]
pub enum MercenarySpecialId {
    Eviscerate,
    SlicingWinds,
}

#[derive(Debug)]
pub struct RexLoadout {
    secondary: RexSecondaryId,
    utility: RexUtilityId,
    special: RexSpecialId,
}

#[derive(Debug)]
pub enum RexSecondaryId {
    DirectiveDrill,
    SeedBarrage,
}

#[derive(Debug)]
pub enum RexUtilityId {
    DirectiveDisperse,
    BrambleVolley,
}

#[derive(Debug)]
pub enum RexSpecialId {
    DirectiveHarvest,
    TanglingGrowth,
}

#[derive(Debug)]
pub struct LoaderLoadout {
    secondary: LoaderSecondaryId,
    utility: LoaderUtilityId,
    special: LoaderSpecialId,
}

#[derive(Debug)]
pub enum LoaderSecondaryId {
    GrappleFist,
    SpikedFist,
}

#[derive(Debug)]
pub enum LoaderUtilityId {
    ChargedGauntlet,
    ThunderGauntlet,
}

#[derive(Debug)]
pub enum LoaderSpecialId {
    M551Pylon,
    Thunderslam,
}

#[derive(Debug)]
pub struct AcridLoadout {
    misc: AcridMiscId,
    secondary: AcridSecondaryId,
    utility: AcridUtilityId,
}

#[derive(Debug)]
pub enum AcridMiscId {
    Poison,
    Blight,
}

#[derive(Debug)]
pub enum AcridSecondaryId {
    Neurotoxin,
    RavenousBite,
}

#[derive(Debug)]
pub enum AcridUtilityId {
    CausticLeap,
    FrenziedLeap,
}

#[derive(Debug)]
pub struct CaptainLoadout {
    utility: CaptainUtilityId,
    special1: CaptainSpecialId,
    special2: CaptainSpecialId,
}

#[derive(Debug)]
pub enum CaptainUtilityId {
    OrbitalProbe,
    OGM72DiabloStrike,
}

#[derive(Debug)]
pub enum CaptainSpecialId {
    BeaconHealing,
    BeaconShocking,
    BeaconResupply,
    BeaconHacking,
}

#[derive(Debug)]
pub struct RailgunnerLoadout {
    secondary: RailgunnerSecondaryId,
    utility: RailgunnerUtilityId,
    special: RailgunnerSpecialId,
}

#[derive(Debug)]
pub enum RailgunnerSecondaryId {
    M99Sniper,
    HH44Marksman,
}

#[derive(Debug)]
pub enum RailgunnerUtilityId {
    ConcussionDevice,
    PolarFieldDevice,
}

#[derive(Debug)]
pub enum RailgunnerSpecialId {
    Supercharge,
    Cryocharge,
}

#[derive(Debug)]
pub struct VoidFiendLoadout {}

#[derive(Debug)]
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

#[derive(Debug)]
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

#[derive(Debug)]
pub enum CategoryId {
    Damage,
    Healing,
    Utility,
    BrotherBlacklist,
    OnKillEffect,
}

#[derive(Debug, PartialEq, Eq)]
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

#[derive(Debug)]
pub enum OutcomeId {
    Defeat,
    FateUnknown,
    Victory,
}

#[derive(Debug)]
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

#[derive(Debug)]
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
