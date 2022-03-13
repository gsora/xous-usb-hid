//! USB HID usage pages
//!
//! See Universal Serial Bus (USB) HID Usage Tables Version 1.12
//! <https://www.usb.org/sites/default/files/documents/hut1_12v2.pdf>

use hash32::{Hash, Hasher};
use num_enum::FromPrimitive;
use num_enum::IntoPrimitive;
use packed_struct::prelude::*;

// Notes for converting .upg files to rust enum
// * Trim header
// * Flip and format: ([0-9A-F]*)\t(.*) - $2=0x$1,
// * Fix casing: (\b[a-z]) - \u$1
// * Squash spaces and punctuation: [^\w=,]
// * Unmangle reserved: (.*)(reserved)=(.*) - //0x$1-$3 $2

/// LEDs usage page
///
/// See [Universal Serial Bus (USB) HID Usage Tables Version 1.12](<https://www.usb.org/sites/default/files/documents/hut1_12v2.pdf>):
/// Section 11 LED Page (0x08)
#[derive(
    Debug,
    Copy,
    Clone,
    Eq,
    PartialEq,
    Ord,
    PartialOrd,
    PrimitiveEnum,
    Hash,
    IntoPrimitive,
    FromPrimitive,
)]
#[repr(u8)]
pub enum Leds {
    #[num_enum(default)]
    Undefined = 0x00,
    NumLock = 0x01,
    CapsLock = 0x02,
    ScrollLock = 0x03,
    Compose = 0x04,
    Kana = 0x05,
    Power = 0x06,
    Shift = 0x07,
    DoNotDisturb = 0x08,
    Mute = 0x09,
    ToneEnable = 0x0A,
    HighCutFilter = 0x0B,
    LowCutFilter = 0x0C,
    EqualizerEnable = 0x0D,
    SoundFieldOn = 0x0E,
    SurroundFieldOn = 0x0F,
    Repeat = 0x10,
    Stereo = 0x11,
    SamplingRateDetect = 0x12,
    Spinning = 0x13,
    CAV = 0x14,
    CLV = 0x15,
    RecordingFormatDetect = 0x16,
    OffHook = 0x17,
    Ring = 0x18,
    MessageWaiting = 0x19,
    DataMode = 0x1A,
    BatteryOperation = 0x1B,
    BatteryOK = 0x1C,
    BatteryLow = 0x1D,
    Speaker = 0x1E,
    HeadSet = 0x1F,
    Hold = 0x20,
    Microphone = 0x21,
    Coverage = 0x22,
    NightMode = 0x23,
    SendCalls = 0x24,
    CallPickup = 0x25,
    Conference = 0x26,
    StandBy = 0x27,
    CameraOn = 0x28,
    CameraOff = 0x29,
    OnLine = 0x2A,
    OffLine = 0x2B,
    Busy = 0x2C,
    Ready = 0x2D,
    PaperOut = 0x2E,
    PaperJam = 0x2F,
    Remote = 0x30,
    Forward = 0x31,
    Reverse = 0x32,
    Stop = 0x33,
    Rewind = 0x34,
    FastForward = 0x35,
    Play = 0x36,
    Pause = 0x37,
    Record = 0x38,
    Error = 0x39,
    UsageSelectedIndicator = 0x3A,
    UsageInUseIndicator = 0x3B,
    UsageMultiModeIndicator = 0x3C,
    IndicatorOn = 0x3D,
    IndicatorFlash = 0x3E,
    IndicatorSlowBlink = 0x3F,
    IndicatorFastBlink = 0x40,
    IndicatorOff = 0x41,
    FlashOnTime = 0x42,
    SlowBlinkOnTime = 0x43,
    SlowBlinkOffTime = 0x44,
    FastBlinkOnTime = 0x45,
    FastBlinkOffTime = 0x46,
    UsageIndicatorColor = 0x47,
    Red = 0x48,
    Green = 0x49,
    Amber = 0x4A,
    GenericIndicator = 0x4B,
    //0x4C-0xFFFF Reserved
}

impl Default for Leds {
    fn default() -> Self {
        Self::Undefined
    }
}

impl Hash for Leds {
    fn hash<H>(&self, state: &mut H)
    where
        H: Hasher,
    {
        let x: u8 = (*self).into();
        state.write(&x.to_le_bytes());
    }
}

/// Consumer usage page
///
/// See [Universal Serial Bus (USB) HID Usage Tables Version 1.12](<https://www.usb.org/sites/default/files/documents/hut1_12v2.pdf>):
/// Section 15 Consumer Page (0x0C)
#[derive(
    Debug,
    Copy,
    Clone,
    Eq,
    PartialEq,
    Ord,
    PartialOrd,
    PrimitiveEnum,
    Hash,
    IntoPrimitive,
    FromPrimitive,
)]
#[repr(u16)]
pub enum Consumer {
    #[num_enum(default)]
    Unassigned = 0x00,
    ConsumerControl = 0x01,
    NumericKeyPad = 0x02,
    ProgrammableButtons = 0x03,
    Microphone = 0x04,
    Headphone = 0x05,
    GraphicEqualizer = 0x06,
    //0x07-0x1F Reserved
    Plus10 = 0x20,
    Plus100 = 0x21,
    AmPm = 0x22,
    //0x23-0x3F Reserved
    Power = 0x30,
    Reset = 0x31,
    Sleep = 0x32,
    SleepAfter = 0x33,
    SleepMode = 0x34,
    Illumination = 0x35,
    FunctionButtons = 0x36,
    //0x37-0x3F Reserved
    Menu = 0x40,
    MenuPick = 0x41,
    MenuUp = 0x42,
    MenuDown = 0x43,
    MenuLeft = 0x44,
    MenuRight = 0x45,
    MenuEscape = 0x46,
    MenuValueIncrease = 0x47,
    MenuValueDecrease = 0x48,
    //0x49-0x5F Reserved
    DataOnScreen = 0x60,
    ClosedCaption = 0x61,
    ClosedCaptionSelect = 0x62,
    VcrTv = 0x63,
    BroadcastMode = 0x64,
    Snapshot = 0x65,
    Still = 0x66,
    //0x67-0x7F Reserved
    Selection = 0x80,
    AssignSelection = 0x81,
    ModeStep = 0x82,
    RecallLast = 0x83,
    EnterChannel = 0x84,
    OrderMovie = 0x85,
    Channel = 0x86,
    MediaSelection = 0x87,
    MediaSelectComputer = 0x88,
    MediaSelectTV = 0x89,
    MediaSelectWWW = 0x8A,
    MediaSelectDVD = 0x8B,
    MediaSelectTelephone = 0x8C,
    MediaSelectProgramGuide = 0x8D,
    MediaSelectVideoPhone = 0x8E,
    MediaSelectGames = 0x8F,
    MediaSelectMessages = 0x90,
    MediaSelectCD = 0x91,
    MediaSelectVCR = 0x92,
    MediaSelectTuner = 0x93,
    Quit = 0x94,
    Help = 0x95,
    MediaSelectTape = 0x96,
    MediaSelectCable = 0x97,
    MediaSelectSatellite = 0x98,
    MediaSelectSecurity = 0x99,
    MediaSelectHome = 0x9A,
    MediaSelectCall = 0x9B,
    ChannelIncrement = 0x9C,
    ChannelDecrement = 0x9D,
    MediaSelectSAP = 0x9E,
    //0x9F Reserved
    VCRPlus = 0xA0,
    Once = 0xA1,
    Daily = 0xA2,
    Weekly = 0xA3,
    Monthly = 0xA4,
    //0xA5-0xAF Reserved
    Play = 0xB0,
    Pause = 0xB1,
    Record = 0xB2,
    FastForward = 0xB3,
    Rewind = 0xB4,
    ScanNextTrack = 0xB5,
    ScanPreviousTrack = 0xB6,
    Stop = 0xB7,
    Eject = 0xB8,
    RandomPlay = 0xB9,
    SelectDisc = 0xBA,
    EnterDisc = 0xBB,
    Repeat = 0xBC,
    Tracking = 0xBD,
    TrackNormal = 0xBE,
    SlowTracking = 0xBF,
    FrameForward = 0xC0,
    FrameBack = 0xC1,
    Mark = 0xC2,
    ClearMark = 0xC3,
    RepeatFromMark = 0xC4,
    ReturnToMark = 0xC5,
    SearchMarkForward = 0xC6,
    SearchMarkBackwards = 0xC7,
    CounterReset = 0xC8,
    ShowCounter = 0xC9,
    TrackingIncrement = 0xCA,
    TrackingDecrement = 0xCB,
    StopEject = 0xCC,
    PlayPause = 0xCD,
    PlaySkip = 0xCE,
    //0xCF-0xDF Reserved
    Volume = 0xE0,
    Balance = 0xE1,
    Mute = 0xE2,
    Bass = 0xE3,
    Treble = 0xE4,
    BassBoost = 0xE5,
    SurroundMode = 0xE6,
    Loudness = 0xE7,
    MPX = 0xE8,
    VolumeIncrement = 0xE9,
    VolumeDecrement = 0xEA,
    //0xEB-0xEF Reserved
    SpeedSelect = 0xF0,
    PlaybackSpeed = 0xF1,
    StandardPlay = 0xF2,
    LongPlay = 0xF3,
    ExtendedPlay = 0xF4,
    Slow = 0xF5,
    //0xF6-0xFF Reserved
    FanEnable = 0x100,
    FanSpeed = 0x101,
    LightEnable = 0x102,
    LightIlluminationLevel = 0x103,
    ClimateControlEnable = 0x104,
    RoomTemperature = 0x105,
    SecurityEnable = 0x106,
    FireAlarm = 0x107,
    PoliceAlarm = 0x108,
    Proximity = 0x109,
    Motion = 0x10A,
    DuressAlarm = 0x10B,
    HoldupAlarm = 0x10C,
    MedicalAlarm = 0x10D,
    //0x10E-0x14F Reserved
    BalanceRight = 0x150,
    BalanceLeft = 0x151,
    BassIncrement = 0x152,
    BassDecrement = 0x153,
    TrebleIncrement = 0x154,
    TrebleDecrement = 0x155,
    //0x156-0x15F Reserved
    SpeakerSystem = 0x160,
    ChannelLeft = 0x161,
    ChannelRight = 0x162,
    ChannelCenter = 0x163,
    ChannelFront = 0x164,
    ChannelCenterFront = 0x165,
    ChannelSide = 0x166,
    ChannelSurround = 0x167,
    ChannelLowFrequencyEnhancement = 0x168,
    ChannelTop = 0x169,
    ChannelUnknown = 0x16A,
    //0x16B-0x16F Reserved
    SubChannel = 0x170,
    SubChannelIncrement = 0x171,
    SubChannelDecrement = 0x172,
    AlternateAudioIncrement = 0x173,
    AlternateAudioDecrement = 0x174,
    //0x175-0x17F Reserved
    ApplicationLaunchButtons = 0x180,
    ALLaunchButtonConfigurationTool = 0x181,
    ALProgrammableButtonConfiguration = 0x182,
    ALConsumerControlConfiguration = 0x183,
    ALWordProcessor = 0x184,
    ALTextEditor = 0x185,
    ALSpreadsheet = 0x186,
    ALGraphicsEditor = 0x187,
    ALPresentationApp = 0x188,
    ALDatabaseApp = 0x189,
    ALEmailReader = 0x18A,
    ALNewsreader = 0x18B,
    ALVoicemail = 0x18C,
    ALContactsAddressBook = 0x18D,
    ALCalendarSchedule = 0x18E,
    ALTaskProjectManager = 0x18F,
    ALLogJournalTimecard = 0x190,
    ALCheckbookFinance = 0x191,
    ALCalculator = 0x192,
    ALAvCapturePlayback = 0x193,
    ALLocalMachineBrowser = 0x194,
    ALLanWanBrowser = 0x195,
    ALInternetBrowser = 0x196,
    ALRemoteNetworkingISPConnect = 0x197,
    ALNetworkConference = 0x198,
    ALNetworkChat = 0x199,
    ALTelephonyDialer = 0x19A,
    ALLogon = 0x19B,
    ALLogoff = 0x19C,
    ALLogonLogoff = 0x19D,
    ALTerminalLockScreensaver = 0x19E,
    ALControlPanel = 0x19F,
    ALCommandLineProcessorRun = 0x1A0,
    ALProcessTaskManager = 0x1A1,
    ALSelectTaskApplication = 0x1A2,
    ALNextTaskApplication = 0x1A3,
    ALPreviousTaskApplication = 0x1A4,
    ALPreemptiveHaltTaskApplication = 0x1A5,
    ALIntegratedHelpCenter = 0x1A6,
    ALDocuments = 0x1A7,
    ALThesaurus = 0x1A8,
    ALDictionary = 0x1A9,
    ALDesktop = 0x1AA,
    ALSpellCheck = 0x1AB,
    ALGrammarCheck = 0x1AC,
    ALWirelessStatus = 0x1AD,
    ALKeyboardLayout = 0x1AE,
    ALVirusProtection = 0x1AF,
    ALEncryption = 0x1B0,
    ALScreenSaver = 0x1B1,
    ALAlarms = 0x1B2,
    ALClock = 0x1B3,
    ALFileBrowser = 0x1B4,
    ALPowerStatus = 0x1B5,
    ALImageBrowser = 0x1B6,
    ALAudioBrowser = 0x1B7,
    ALMovieBrowser = 0x1B8,
    ALDigitalRightsManager = 0x1B9,
    ALDigitalWallet = 0x1BA,
    //0x-0x1BB Reserved
    ALInstantMessaging = 0x1BC,
    ALOemFeaturesTipsTutorialBrowser = 0x1BD,
    ALOemHelp = 0x1BE,
    ALOnlineCommunity = 0x1BF,
    ALEntertainmentContentBrowser = 0x1C0,
    ALOnlineShoppingBrowser = 0x1C1,
    ALSmartCardInformationHelp = 0x1C2,
    ALMarketMonitorFinanceBrowser = 0x1C3,
    ALCustomizedCorporateNewsBrowser = 0x1C4,
    ALOnlineActivityBrowser = 0x1C5,
    ALResearchSearchBrowser = 0x1C6,
    ALAudioPlayer = 0x1C7,
    //0x1C8-0x1FF Reserved
    GenericGUIApplicationControls = 0x200,
    ACNew = 0x201,
    ACOpen = 0x202,
    ACClose = 0x203,
    ACExit = 0x204,
    ACMaximize = 0x205,
    ACMinimize = 0x206,
    ACSave = 0x207,
    ACPrint = 0x208,
    ACProperties = 0x209,
    ACUndo = 0x21A,
    ACCopy = 0x21B,
    ACCut = 0x21C,
    ACPaste = 0x21D,
    ACSelectAll = 0x21E,
    ACFind = 0x21F,
    ACFindAndReplace = 0x220,
    ACSearch = 0x221,
    ACGoTo = 0x222,
    ACHome = 0x223,
    ACBack = 0x224,
    ACForward = 0x225,
    ACStop = 0x226,
    ACRefresh = 0x227,
    ACPreviousLink = 0x228,
    ACNextLink = 0x229,
    ACBookmarks = 0x22A,
    ACHistory = 0x22B,
    ACSubscriptions = 0x22C,
    ACZoomIn = 0x22D,
    ACZoomOut = 0x22E,
    ACZoom = 0x22F,
    ACFullScreenView = 0x230,
    ACNormalView = 0x231,
    ACViewToggle = 0x232,
    ACScrollUp = 0x233,
    ACScrollDown = 0x234,
    ACScroll = 0x235,
    ACPanLeft = 0x236,
    ACPanRight = 0x237,
    ACPan = 0x238,
    ACNewWindow = 0x239,
    ACTileHorizontally = 0x23A,
    ACTileVertically = 0x23B,
    ACFormat = 0x23C,
    ACEdit = 0x23D,
    ACBold = 0x23E,
    ACItalics = 0x23F,
    ACUnderline = 0x240,
    ACStrikethrough = 0x241,
    ACSubscript = 0x242,
    ACSuperscript = 0x243,
    ACAllCaps = 0x244,
    ACRotate = 0x245,
    ACResize = 0x246,
    ACFlipHorizontal = 0x247,
    ACFlipVertical = 0x248,
    ACMirrorHorizontal = 0x249,
    ACMirrorVertical = 0x24A,
    ACFontSelect = 0x24B,
    ACFontColor = 0x24C,
    ACFontSize = 0x24D,
    ACJustifyLeft = 0x24E,
    ACJustifyCenterH = 0x24F,
    ACJustifyRight = 0x250,
    ACJustifyBlockH = 0x251,
    ACJustifyTop = 0x252,
    ACJustifyCenterV = 0x253,
    ACJustifyBottom = 0x254,
    ACJustifyBlockV = 0x255,
    ACIndentDecrease = 0x256,
    ACIndentIncrease = 0x257,
    ACNumberedList = 0x258,
    ACRestartNumbering = 0x259,
    ACBulletedList = 0x25A,
    ACPromote = 0x25B,
    ACDemote = 0x25C,
    ACYes = 0x25D,
    ACNo = 0x25E,
    ACCancel = 0x25F,
    ACCatalog = 0x260,
    ACBuyCheckout = 0x261,
    ACAddToCart = 0x262,
    ACExpand = 0x263,
    ACExpandAll = 0x264,
    ACCollapse = 0x265,
    ACCollapseAll = 0x266,
    ACPrintPreview = 0x267,
    ACPasteSpecial = 0x268,
    ACInsertMode = 0x269,
    ACDelete = 0x26A,
    ACLock = 0x26B,
    ACUnlock = 0x26C,
    ACProtect = 0x26D,
    ACUnprotect = 0x26E,
    ACAttachComment = 0x26F,
    ACDeleteComment = 0x270,
    ACViewComment = 0x271,
    ACSelectWord = 0x272,
    ACSelectSentence = 0x273,
    ACSelectParagraph = 0x274,
    ACSelectColumn = 0x275,
    ACSelectRow = 0x276,
    ACSelectTable = 0x277,
    ACSelectObject = 0x278,
    ACRedoRepeat = 0x279,
    ACSort = 0x27A,
    ACSortAscending = 0x27B,
    ACSortDescending = 0x27C,
    ACFilter = 0x27D,
    ACSetClock = 0x27E,
    ACViewClock = 0x27F,
    ACSelectTimeZone = 0x280,
    ACEditTimeZones = 0x281,
    ACSetAlarm = 0x282,
    ACClearAlarm = 0x283,
    ACSnoozeAlarm = 0x284,
    ACResetAlarm = 0x285,
    ACSynchronize = 0x286,
    ACSendReceive = 0x287,
    ACSendTo = 0x288,
    ACReply = 0x289,
    ACReplyAll = 0x28A,
    ACForwardMsg = 0x28B,
    ACSend = 0x28C,
    ACAttachFile = 0x28D,
    ACUpload = 0x28E,
    ACDownloadSaveTargetAs = 0x28F,
    ACSetBorders = 0x290,
    ACInsertRow = 0x291,
    ACInsertColumn = 0x292,
    ACInsertFile = 0x293,
    ACInsertPicture = 0x294,
    ACInsertObject = 0x295,
    ACInsertSymbol = 0x296,
    ACSaveAndClose = 0x297,
    ACRename = 0x298,
    ACMerge = 0x299,
    ACSplit = 0x29A,
    ACDistributeHorizontally = 0x29B,
    ACDistributeVertically = 0x29C,
    //0x29D-0xFFFF Reserved
}

impl Default for Consumer {
    fn default() -> Self {
        Self::Unassigned
    }
}

impl Hash for Consumer {
    fn hash<H>(&self, state: &mut H)
    where
        H: Hasher,
    {
        let x: u16 = (*self).into();
        state.write(&x.to_le_bytes());
    }
}

/// Generic Desktop usage page
///
/// See [Universal Serial Bus (USB) HID Usage Tables Version 1.12](<https://www.usb.org/sites/default/files/documents/hut1_12v2.pdf>):
/// Section 4 Desktop Page (0x01)
#[derive(
    Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, PrimitiveEnum, IntoPrimitive, FromPrimitive,
)]
#[repr(u8)]
pub enum Desktop {
    #[num_enum(default)]
    Undefined = 0x00,
    Pointer = 0x01,
    Mouse = 0x02,
    //0x03 Reserved
    Joystick = 0x04,
    GamePad = 0x05,
    Keyboard = 0x06,
    Keypad = 0x07,
    MultiAxisController = 0x08,
    TabletPcSystemControls = 0x09,
    //0x0A-0x2F Reserved
    X = 0x30,
    Y = 0x31,
    Z = 0x32,
    Rx = 0x33,
    Ry = 0x34,
    Rz = 0x35,
    Slider = 0x36,
    Dial = 0x37,
    Wheel = 0x38,
    HatSwitch = 0x39,
    CountedBuffer = 0x3A,
    ByteCount = 0x3B,
    MotionWakeup = 0x3C,
    Start = 0x3D,
    Select = 0x3E,
    //0x3F-0x3F Reserved
    Vx = 0x40,
    Vy = 0x41,
    Vz = 0x42,
    Vbrx = 0x43,
    Vbry = 0x44,
    Vbrz = 0x45,
    Vno = 0x46,
    FeatureNotification = 0x47,
    ResolutionMultiplier = 0x48,
    //0x49-0x7F Reserved
    SystemControl = 0x80,
    SystemPowerDown = 0x81,
    SystemSleep = 0x82,
    SystemWakeUp = 0x83,
    SystemContextMenu = 0x84,
    SystemMainMenu = 0x85,
    SystemAppMenu = 0x86,
    SystemHelpMenu = 0x87,
    SystemMenuExit = 0x88,
    SystemMenuSelect = 0x89,
    SystemMenuRight = 0x8A,
    SystemMenuLeft = 0x8B,
    SystemMenuUp = 0x8C,
    SystemMenuDown = 0x8D,
    SystemColdRestart = 0x8E,
    SystemWarmRestart = 0x8F,
    DPadUp = 0x90,
    DPadDown = 0x91,
    DPadRight = 0x92,
    DPadLeft = 0x93,
    //0x94-0xFFFF Reserved
}

impl Default for Desktop {
    fn default() -> Self {
        Self::Undefined
    }
}

impl Hash for Desktop {
    fn hash<H>(&self, state: &mut H)
    where
        H: Hasher,
    {
        let x: u8 = (*self).into();
        state.write(&x.to_le_bytes());
    }
}

/// Game Controls usage page
///
/// See [Universal Serial Bus (USB) HID Usage Tables Version 1.12](<https://www.usb.org/sites/default/files/documents/hut1_12v2.pdf>):
/// Section 4 Game Controls Page (0x05)
#[derive(
    Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, PrimitiveEnum, IntoPrimitive, FromPrimitive,
)]
#[repr(u8)]
pub enum Game {
    #[num_enum(default)]
    Undefined = 0x00,
    Game3DController = 0x01,
    PinballDevice = 0x02,
    GunDevice = 0x03,
    //0x04-0x1F Reserved
    PointOfView = 0x20,
    TurnRightLeft = 0x21,
    PitchRightLeft = 0x22,
    RollForwardBackward = 0x23,
    MoveRightLeft = 0x24,
    MoveForwardBackward = 0x25,
    MoveUpDown = 0x26,
    LeanRightLeft = 0x27,
    LeanForwardBackward = 0x28,
    HeightOfPOV = 0x29,
    Flipper = 0x2A,
    SecondaryFlipper = 0x2B,
    Bump = 0x2C,
    NewGame = 0x2D,
    ShootBall = 0x2E,
    Player = 0x2F,
    GunBolt = 0x30,
    GunClip = 0x31,
    GunSelector = 0x32,
    GunSingleShot = 0x33,
    GunBurst = 0x34,
    GunAutomatic = 0x35,
    GunSafety = 0x36,
    GamePadFireJump = 0x37,
    GamePadTrigger = 0x39,
    //0x3A-0xFFFF Reserved
}

impl Default for Game {
    fn default() -> Self {
        Self::Undefined
    }
}

impl Hash for Game {
    fn hash<H>(&self, state: &mut H)
    where
        H: Hasher,
    {
        let x: u8 = (*self).into();
        state.write(&x.to_le_bytes());
    }
}

/// Keyboard usage page
///
/// See [Universal Serial Bus (USB) HID Usage Tables Version 1.12](<https://www.usb.org/sites/default/files/documents/hut1_12v2.pdf>):
/// Section 10 Keyboard/Keypad Page (0x04)
///
/// Naming from the specification has been preserved where possible but some names
/// have been shortened or transliterated to be valid rust identifiers
#[derive(
    Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, PrimitiveEnum, IntoPrimitive, FromPrimitive,
)]
#[repr(u8)]
pub enum Keyboard {
    #[num_enum(default)]
    NoEventIndicated = 0x00,
    ErrorRollOver = 0x01,
    POSTFail = 0x02,
    ErrorUndefine = 0x03,
    A = 0x04,
    B = 0x05,
    C = 0x06,
    D = 0x07,
    E = 0x08,
    F = 0x09,
    G = 0x0A,
    H = 0x0B,
    I = 0x0C,
    J = 0x0D,
    K = 0x0E,
    L = 0x0F,
    M = 0x10,
    N = 0x11,
    O = 0x12,
    P = 0x13,
    Q = 0x14,
    R = 0x15,
    S = 0x16,
    T = 0x17,
    U = 0x18,
    V = 0x19,
    W = 0x1A,
    X = 0x1B,
    Y = 0x1C,
    Z = 0x1D,
    Keyboard1 = 0x1E,
    Keyboard2 = 0x1F,
    Keyboard3 = 0x20,
    Keyboard4 = 0x21,
    Keyboard5 = 0x22,
    Keyboard6 = 0x23,
    Keyboard7 = 0x24,
    Keyboard8 = 0x25,
    Keyboard9 = 0x26,
    Keyboard0 = 0x27,
    ReturnEnter = 0x28,
    Escape = 0x29,
    DeleteBackspace = 0x2A,
    Tab = 0x2B,
    Space = 0x2C,
    Minus = 0x2D,
    Equal = 0x2E,
    LeftBrace = 0x2F,
    RightBrace = 0x30,
    Backslash = 0x31,
    NonUSHash = 0x32,
    Semicolon = 0x33,
    Apostrophe = 0x34,
    Grave = 0x35,
    Comma = 0x36,
    Dot = 0x37,
    ForwardSlash = 0x38,
    CapsLock = 0x39,
    F1 = 0x3A,
    F2 = 0x3B,
    F3 = 0x3C,
    F4 = 0x3D,
    F5 = 0x3E,
    F6 = 0x3F,
    F7 = 0x40,
    F8 = 0x41,
    F9 = 0x42,
    F10 = 0x43,
    F11 = 0x44,
    F12 = 0x45,
    PrintScreen = 0x46,
    ScrollLock = 0x47,
    Pause = 0x48,
    Insert = 0x49,
    Home = 0x4A,
    PageUp = 0x4B,
    DeleteForward = 0x4C,
    End = 0x4D,
    PageDown = 0x4E,
    RightArrow = 0x4F,
    LeftArrow = 0x50,
    DownArrow = 0x51,
    UpArrow = 0x52,
    KeypadNumLockAndClear = 0x53,
    KeypadDivide = 0x54,
    KeypadMultiply = 0x55,
    KeypadSubtract = 0x56,
    KeypadAdd = 0x57,
    KeypadEnter = 0x58,
    Keypad1 = 0x59,
    Keypad2 = 0x5A,
    Keypad3 = 0x5B,
    Keypad4 = 0x5C,
    Keypad5 = 0x5D,
    Keypad6 = 0x5E,
    Keypad7 = 0x5F,
    Keypad8 = 0x60,
    Keypad9 = 0x61,
    Keypad0 = 0x62,
    KeypadDot = 0x63,
    NonUSBackslash = 0x64,
    Application = 0x65,
    Power = 0x66,
    KeypadEqual = 0x67,
    F13 = 0x68,
    F14 = 0x69,
    F15 = 0x6A,
    F16 = 0x6B,
    F17 = 0x6C,
    F18 = 0x6D,
    F19 = 0x6E,
    F20 = 0x6F,
    F21 = 0x70,
    F22 = 0x71,
    F23 = 0x72,
    F24 = 0x73,
    Execute = 0x74,
    Help = 0x75,
    Menu = 0x76,
    Select = 0x77,
    Stop = 0x78,
    Again = 0x79,
    Undo = 0x7A,
    Cut = 0x7B,
    Copy = 0x7C,
    Paste = 0x7D,
    Find = 0x7E,
    Mute = 0x7F,
    VolumeUp = 0x80,
    VolumeDown = 0x81,
    LockingCapsLock = 0x82,
    LockingNumLock = 0x83,
    LockingScrollLock = 0x84,
    KeypadComma = 0x85,
    KeypadEqualSign = 0x86,
    Kanji1 = 0x87,
    Kanji2 = 0x88,
    Kanji3 = 0x89,
    Kanji4 = 0x8A,
    Kanji5 = 0x8B,
    Kanji6 = 0x8C,
    Kanji7 = 0x8D,
    Kanji8 = 0x8E,
    Kanji9 = 0x8F,
    LANG1 = 0x90,
    LANG2 = 0x91,
    LANG3 = 0x92,
    LANG4 = 0x93,
    LANG5 = 0x94,
    LANG6 = 0x95,
    LANG7 = 0x96,
    LANG8 = 0x97,
    LANG9 = 0x98,
    AlternateErase = 0x99,
    SysReqAttention = 0x9A,
    Cancel = 0x9B,
    Clear = 0x9C,
    Prior = 0x9D,
    Return = 0x9E,
    Separator = 0x9F,
    Out = 0xA0,
    Oper = 0xA1,
    ClearAgain = 0xA2,
    CrSelProps = 0xA3,
    ExSel = 0xA4,
    //0xA5-0xDF Reserved
    LeftControl = 0xE0,
    LeftShift = 0xE1,
    LeftAlt = 0xE2,
    LeftGUI = 0xE3,
    RightControl = 0xE4,
    RightShift = 0xE5,
    RightAlt = 0xE6,
    RightGUI = 0xE7,
    //0xE8-0xFFFF Reserved
}

impl Default for Keyboard {
    fn default() -> Self {
        Self::NoEventIndicated
    }
}

impl Hash for Keyboard {
    fn hash<H>(&self, state: &mut H)
    where
        H: Hasher,
    {
        let x: u8 = (*self).into();
        state.write(&x.to_le_bytes());
    }
}

/// Simulation Controls usage page
///
/// See [Universal Serial Bus (USB) HID Usage Tables Version 1.12](<https://www.usb.org/sites/default/files/documents/hut1_12v2.pdf>):
/// Section 5 Simulation Controls Page (0x02)
#[derive(
    Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, PrimitiveEnum, IntoPrimitive, FromPrimitive,
)]
#[repr(u8)]
pub enum Simulation {
    #[num_enum(default)]
    Undefined = 0x00,
    FlightSimulationDevice = 0x01,
    AutomobileSimulationDevice = 0x02,
    TankSimulationDevice = 0x03,
    SpaceshipSimulationDevice = 0x04,
    SubmarineSimulationDevice = 0x05,
    SailingSimulationDevice = 0x06,
    MotorcycleSimulationDevice = 0x07,
    SportsSimulationDevice = 0x08,
    AirplaneSimulationDevice = 0x09,
    HelicopterSimulationDevice = 0x0A,
    MagicCarpetSimulationDevice = 0x0B,
    Bicycle = 0x0C,
    //0x0D-0x1F Reserved
    FlightControlStick = 0x20,
    FlightStick = 0x21,
    CyclicControl = 0x22,
    CyclicTrim = 0x23,
    FlightYoke = 0x24,
    TrackControl = 0x25,
    DrivingControl = 0x26,
    //0x27-0xCF Reserved
    Aileron = 0xB0,
    AileronTrim = 0xB1,
    AntiTorqueControl = 0xB2,
    AutoPilotEnable = 0xB3,
    ChaffRelease = 0xB4,
    CollectiveControl = 0xB5,
    DiveBrake = 0xB6,
    ElectronicCounterMeasures = 0xB7,
    Elevator = 0xB8,
    ElevatorTrim = 0xB9,
    Rudder = 0xBA,
    Throttle = 0xBB,
    FlightCommunication = 0xBC,
    FlareRelease = 0xBD,
    LandingGear = 0xBE,
    ToeBrake = 0xBF,
    Trigger = 0xC0,
    WeaponsArm = 0xC1,
    WeaponsSelect = 0xC2,
    WingFlaps = 0xC3,
    Accelerator = 0xC4,
    Brake = 0xC5,
    Clutch = 0xC6,
    Shifter = 0xC7,
    Steering = 0xC8,
    TurretDirection = 0xC9,
    BarrelElevation = 0xCA,
    DivePlane = 0xCB,
    Ballast = 0xCC,
    BicycleCrank = 0xCD,
    HandleBars = 0xCE,
    FrontBrake = 0xCF,
    RearBrake = 0xD0,
    //0xD1-0xFFFF Reserved
}

impl Default for Simulation {
    fn default() -> Self {
        Self::Undefined
    }
}

impl Hash for Simulation {
    fn hash<H>(&self, state: &mut H)
    where
        H: Hasher,
    {
        let x: u8 = (*self).into();
        state.write(&x.to_le_bytes());
    }
}

/// Telephony Device usage page
///
/// See [Universal Serial Bus (USB) HID Usage Tables Version 1.12](<https://www.usb.org/sites/default/files/documents/hut1_12v2.pdf>):
/// Section 14 Telephony Device  Page (0x0B)
#[derive(
    Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, PrimitiveEnum, IntoPrimitive, FromPrimitive,
)]
#[repr(u8)]
pub enum Telephony {
    #[num_enum(default)]
    Unassigned = 0x00,
    Phone = 0x01,
    AnsweringMachine = 0x02,
    MessageControls = 0x03,
    Handset = 0x04,
    Headset = 0x05,
    TelephonyKeyPad = 0x06,
    ProgrammableButton = 0x07,
    //0x08-0x1F Reserved
    HookSwitch = 0x20,
    Flash = 0x21,
    Feature = 0x22,
    Hold = 0x23,
    Redial = 0x24,
    Transfer = 0x25,
    Drop = 0x26,
    Park = 0x27,
    ForwardCalls = 0x28,
    AlternateFunction = 0x29,
    Line = 0x2A,
    SpeakerPhone = 0x2B,
    Conference = 0x2C,
    RingEnable = 0x2D,
    RingSelect = 0x2E,
    PhoneMute = 0x2F,
    CallerID = 0x30,
    Send = 0x31,
    //0x32-0x4F Reserved
    SpeedDial = 0x50,
    StoreNumber = 0x51,
    RecallNumber = 0x52,
    PhoneDirectory = 0x53,
    //0x54-0x6F Reserved
    VoiceMail = 0x70,
    ScreenCalls = 0x71,
    DoNotDisturb = 0x72,
    Message = 0x73,
    AnswerOnOff = 0x74,
    //0x75-0x8F Reserved
    InsideDialTone = 0x90,
    OutsideDialTone = 0x91,
    InsideRingTone = 0x92,
    OutsideRingTone = 0x93,
    PriorityRingTone = 0x94,
    InsideRingback = 0x95,
    PriorityRingback = 0x96,
    LineBusyTone = 0x97,
    ReorderTone = 0x98,
    CallWaitingTone = 0x99,
    ConfirmationTone1 = 0x9A,
    ConfirmationTone2 = 0x9B,
    TonesOff = 0x9C,
    OutsideRingback = 0x9D,
    Ringer = 0x9E,
    //0x9F-0xAF Reserved
    PhoneKey0 = 0xB0,
    PhoneKey1 = 0xB1,
    PhoneKey2 = 0xB2,
    PhoneKey3 = 0xB3,
    PhoneKey4 = 0xB4,
    PhoneKey5 = 0xB5,
    PhoneKey6 = 0xB6,
    PhoneKey7 = 0xB7,
    PhoneKey8 = 0xB8,
    PhoneKey9 = 0xB9,
    PhoneKeyStar = 0xBA,
    PhoneKeyPound = 0xBB,
    PhoneKeyA = 0xBC,
    PhoneKeyB = 0xBD,
    PhoneKeyC = 0xBE,
    PhoneKeyD = 0xBF,
    //0xC0-0xFFFF Reserved
}
impl Default for Telephony {
    fn default() -> Self {
        Self::Unassigned
    }
}

impl Hash for Telephony {
    fn hash<H>(&self, state: &mut H)
    where
        H: Hasher,
    {
        let x: u8 = (*self).into();
        state.write(&x.to_le_bytes());
    }
}
