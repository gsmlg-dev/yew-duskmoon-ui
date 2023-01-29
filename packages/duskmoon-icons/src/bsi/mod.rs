use yew::prelude::*;

pub const NAMES: [&str; 1953] = [
  "Check2Circle",
  "HouseUp",
  "ArrowDownLeftSquareFill",
  "FiletypeExe",
  "Tornado",
  "ViewStacked",
  "DropletHalf",
  "TrashFill",
  "NodePlusFill",
  "ArrowUpLeftCircle",
  "PentagonHalf",
  "DatabaseFill",
  "FileEarmarkArrowUp",
  "Buildings",
  "5Circle",
  "FiletypeGif",
  "BorderMiddle",
  "ExplicitFill",
  "Scooter",
  "Compass",
  "CalendarDate",
  "Youtube",
  "ChatLeftQuoteFill",
  "CloudFog2Fill",
  "7CircleFill",
  "PlusCircle",
  "CaretUpSquare",
  "Explicit",
  "Check2Square",
  "CSquare",
  "PatchCheck",
  "CaretUp",
  "BarChartSteps",
  "CardChecklist",
  "Badge4k",
  "CCircleFill",
  "FileEarmarkDiffFill",
  "FileImageFill",
  "HouseSlashFill",
  "TicketPerforatedFill",
  "PCircleFill",
  "StarHalf",
  "CloudHailFill",
  "HouseCheck",
  "CalendarWeekFill",
  "JournalMedical",
  "CloudFog2",
  "FiletypeWoff",
  "BookmarkPlus",
  "VolumeUpFill",
  "FileEarmarkEasel",
  "Dribbble",
  "SunsetFill",
  "PersonFillLock",
  "Keyboard",
  "FileEarmarkImageFill",
  "FileEarmarkPdf",
  "SortAlphaDownAlt",
  "ChatSquareTextFill",
  "Easel2",
  "Inbox",
  "ArrowUpRightCircle",
  "FiletypeTxt",
  "FiletypeSql",
  "Toggles",
  "SortAlphaUpAlt",
  "Trash3Fill",
  "BalloonHeart",
  "ClipboardHeartFill",
  "FileMedical",
  "BuildingFillGear",
  "TrainFront",
  "HousesFill",
  "Thermometer",
  "0Circle",
  "FileEarmarkBinary",
  "ChatLeftTextFill",
  "Bicycle",
  "Translate",
  "FileEarmarkPlus",
  "BookmarkFill",
  "ChevronLeft",
  "EraserFill",
  "XOctagon",
  "TagFill",
  "FileEarmarkPlay",
  "Trash2",
  "Hourglass",
  "Question",
  "LifePreserver",
  "BookmarkStar",
  "ConeStriped",
  "QuestionCircleFill",
  "ExclamationOctagonFill",
  "DashLg",
  "FiletypeJpg",
  "Dice2",
  "SignMergeRightFill",
  "Telephone",
  "Hammer",
  "Twitch",
  "FileSlides",
  "Eraser",
  "VolumeMuteFill",
  "HouseCheckFill",
  "FiletypeOtf",
  "CreditCard2FrontFill",
  "Tv",
  "FileEarmarkExcelFill",
  "EmojiSmileUpsideDown",
  "TypeH2",
  "PSquareFill",
  "LayerForward",
  "Cursor",
  "FolderMinus",
  "Eyedropper",
  "TicketFill",
  "StopwatchFill",
  "Cup",
  "Nvidia",
  "BookmarkHeartFill",
  "TicketDetailed",
  "Paypal",
  "Grid3x2",
  "CCircle",
  "Mortarboard",
  "ChatSquareHeart",
  "Terminal",
  "FolderSymlink",
  "PersonHeart",
  "ArrowRightShort",
  "JournalPlus",
  "FileX",
  "Pass",
  "EnvelopePlus",
  "Wifi1",
  "FilesAlt",
  "TruckFlatbed",
  "ChatLeftQuote",
  "HouseGear",
  "FolderSymlinkFill",
  "Diagram2",
  "FiletypePsd",
  "PersonPlus",
  "EmojiSmileFill",
  "RewindBtn",
  "ChevronExpand",
  "WindowDash",
  "CloudRainHeavy",
  "SkipEndBtn",
  "FileFontFill",
  "Prescription",
  "Handbag",
  "Ear",
  "Xbox",
  "Plugin",
  "BrightnessAltLowFill",
  "ChatRightHeartFill",
  "GooglePlay",
  "BalloonFill",
  "FileEarmarkArrowDownFill",
  "Arrow90degUp",
  "Mic",
  "1CircleFill",
  "BrightnessLow",
  "Clipboard2Minus",
  "AspectRatioFill",
  "SignTurnSlightRightFill",
  "WindowX",
  "ChatLeftDotsFill",
  "SignDeadEnd",
  "ChevronUp",
  "Bookmark",
  "FileEarmarkCodeFill",
  "Subscript",
  "Fan",
  "Dice1",
  "SignStopLightsFill",
  "ArrowDownLeftSquare",
  "ArrowUpLeftCircleFill",
  "DatabaseFillExclamation",
  "Speedometer",
  "ChatHeartFill",
  "HeartHalf",
  "ShieldFillCheck",
  "CalendarDay",
  "HourglassTop",
  "BackspaceReverseFill",
  "Brush",
  "FileEarmarkPpt",
  "Yelp",
  "Stop",
  "EmojiExpressionless",
  "SkipForwardBtn",
  "Columns",
  "InfoCircleFill",
  "6CircleFill",
  "QrCode",
  "DiamondFill",
  "CloudRainHeavyFill",
  "CursorText",
  "ArchiveFill",
  "ChevronBarUp",
  "CaretRightFill",
  "FastForwardCircle",
  "Map",
  "CartFill",
  "CartCheckFill",
  "FilePlayFill",
  "LayoutSidebarInset",
  "CcSquare",
  "SdCard",
  "EmojiDizzy",
  "Wifi",
  "ThermometerLow",
  "FileLock2Fill",
  "BagPlus",
  "CameraVideo",
  "BoxArrowLeft",
  "BootstrapFill",
  "Sunset",
  "FilterCircle",
  "CapsulePill",
  "SendSlashFill",
  "BuildingFill",
  "Files",
  "BagCheck",
  "JournalBookmarkFill",
  "CloudSnowFill",
  "OctagonHalf",
  "TerminalSplit",
  "CalendarMinus",
  "ArrowLeftSquareFill",
  "ChevronContract",
  "Router",
  "Calendar2Day",
  "PersonPlusFill",
  "SignpostFill",
  "Disc",
  "Grid3x2GapFill",
  "3Square",
  "Basket3Fill",
  "ArrowUpCircle",
  "GlobeEuropeAfrica",
  "FileExcelFill",
  "Discord",
  "Prescription2",
  "ArrowReturnLeft",
  "Cart",
  "CalendarDayFill",
  "3CircleFill",
  "MusicNoteBeamed",
  "ArrowDownCircle",
  "CloudyFill",
  "DisplayFill",
  "FilePersonFill",
  "ClipboardData",
  "RocketTakeoff",
  "PersonFillCheck",
  "HeadsetVr",
  "BookmarksFill",
  "DatabaseSlash",
  "DistributeVertical",
  "BarChartFill",
  "SuitHeart",
  "HCircleFill",
  "Server",
  "DistributeHorizontal",
  "SignIntersectionY",
  "BuildingCheck",
  "BackspaceReverse",
  "ListUl",
  "SortDown",
  "FileMedicalFill",
  "FileEarmarkMinusFill",
  "ShieldFillX",
  "ChatRightHeart",
  "ArrowDownSquareFill",
  "Paragraph",
  "TerminalX",
  "ChatLeftFill",
  "Soundwave",
  "EnvelopeAtFill",
  "Arrow90degDown",
  "FileRuledFill",
  "ExclamationTriangle",
  "UniversalAccess",
  "SendCheck",
  "Hypnotize",
  "PatchPlusFill",
  "CartDash",
  "StackOverflow",
  "BellSlashFill",
  "FileEarmarkArrowDown",
  "LayoutTextSidebar",
  "HandIndexThumb",
  "StoplightsFill",
  "Minecart",
  "BorderTop",
  "Fullscreen",
  "FileLock",
  "ShieldCheck",
  "ChatLeftHeartFill",
  "PersonDown",
  "Dash",
  "Reception4",
  "Speaker",
  "ClipboardX",
  "BookmarkCheckFill",
  "BinocularsFill",
  "TrainLightrailFront",
  "TreeFill",
  "ArrowThroughHeart",
  "FilePerson",
  "HouseLockFill",
  "FileEarmarkPptFill",
  "Tree",
  "Pinterest",
  "Gem",
  "Clipboard2PulseFill",
  "Dice2Fill",
  "CartCheck",
  "FuelPumpDieselFill",
  "ThreeDotsVertical",
  "FuelPumpDiesel",
  "XSquare",
  "PersonRolodex",
  "Calendar2MinusFill",
  "GlobeAmericas",
  "DoorClosed",
  "VinylFill",
  "ListStars",
  "PlayCircleFill",
  "SendExclamation",
  "Bezier",
  "PauseCircleFill",
  "LayoutTextWindow",
  "PinAngleFill",
  "SpeakerFill",
  "Slack",
  "Basket",
  "BookmarkX",
  "BatteryHalf",
  "CalendarEventFill",
  "EnvelopeSlashFill",
  "PuzzleFill",
  "CalendarCheck",
  "ImageAlt",
  "FiletypePy",
  "FilePostFill",
  "Braces",
  "SkipEndCircleFill",
  "LightbulbOffFill",
  "BrowserChrome",
  "Plus",
  "GenderAmbiguous",
  "Sliders",
  "Whatsapp",
  "BrightnessLowFill",
  "SkipBackwardFill",
  "PersonX",
  "VolumeUp",
  "Laptop",
  "Apple",
  "PersonDashFill",
  "Unindent",
  "MoonFill",
  "8SquareFill",
  "HouseHeartFill",
  "FileArrowUpFill",
  "Snow2",
  "At",
  "CheckSquare",
  "FileTextFill",
  "FileEarmarkLock",
  "FiletypeM4p",
  "SendExclamationFill",
  "TelephoneMinus",
  "Google",
  "JournalRichtext",
  "0CircleFill",
  "ArrowLeftCircleFill",
  "StopBtnFill",
  "Moon",
  "TabletLandscapeFill",
  "Reply",
  "QuestionSquare",
  "FiletypeAi",
  "TruckFront",
  "SignIntersectionT",
  "ClockHistory",
  "TriangleHalf",
  "Sunglasses",
  "TypeH3",
  "6Circle",
  "Pentagon",
  "FileEarmarkX",
  "StickiesFill",
  "NintendoSwitch",
  "CollectionPlayFill",
  "JournalBookmark",
  "StickyFill",
  "CurrencyBitcoin",
  "BackspaceFill",
  "TextLeft",
  "Headphones",
  "GeoFill",
  "CloudMinusFill",
  "FileWord",
  "FileDiffFill",
  "LightbulbOff",
  "Box2Heart",
  "EnvelopeOpenFill",
  "ArrowLeftShort",
  "PostcardHeart",
  "Journals",
  "Link",
  "TextIndentLeft",
  "BookHalf",
  "ArrowUpRightSquareFill",
  "FileSpreadsheetFill",
  "UiRadiosGrid",
  "Bank",
  "FiletypeSh",
  "BoundingBox",
  "CameraFill",
  "Memory",
  "Quote",
  "FilePost",
  "Calendar3RangeFill",
  "HeartPulse",
  "SkipStartCircle",
  "Amd",
  "TextCenter",
  "ListOl",
  "SignDoNotEnterFill",
  "LayoutTextSidebarReverse",
  "PersonVcardFill",
  "DiamondHalf",
  "Reception3",
  "SortDownAlt",
  "Wifi2",
  "Eye",
  "CalendarXFill",
  "Incognito",
  "ArrowUpCircleFill",
  "HddStack",
  "Check",
  "InfoSquare",
  "FileEarmarkBreak",
  "Infinity",
  "BuildingUp",
  "JournalX",
  "CalendarPlus",
  "ExclamationCircle",
  "DatabaseFillLock",
  "Calendar2DateFill",
  "Palette",
  "Window",
  "BookmarkHeart",
  "Asterisk",
  "Basket2",
  "MicMuteFill",
  "BrowserSafari",
  "ChatRightDotsFill",
  "ClipboardMinusFill",
  "Circle",
  "Easel3Fill",
  "TrainFreightFrontFill",
  "GearWide",
  "ChevronRight",
  "FileEarmarkBinaryFill",
  "Wikipedia",
  "9CircleFill",
  "Droplet",
  "SignpostSplit",
  "StopCircle",
  "DatabaseFillGear",
  "CaretLeft",
  "InfoCircle",
  "DatabaseFillX",
  "Triangle",
  "HandThumbsUpFill",
  "JournalAlbum",
  "Cart2",
  "ShieldX",
  "Record2",
  "SuitDiamondFill",
  "JournalText",
  "SignYield",
  "Puzzle",
  "TelephoneMinusFill",
  "FilePdf",
  "MusicNote",
  "ArrowsAngleContract",
  "BoxArrowInLeft",
  "Recycle",
  "ChevronDoubleDown",
  "FiletypePptx",
  "XCircle",
  "TypeBold",
  "PhoneLandscape",
  "DatabaseDown",
  "DatabaseFillSlash",
  "PersonBadgeFill",
  "PieChart",
  "Exclamation",
  "FileEarmarkLock2",
  "DatabaseGear",
  "BookmarkStarFill",
  "SkipStartBtn",
  "SignNoRightTurnFill",
  "BagX",
  "ZoomIn",
  "ArrowRightCircleFill",
  "ClipboardCheck",
  "CameraVideoOffFill",
  "Database",
  "GearWideConnected",
  "TelephonePlus",
  "FileEarmarkWordFill",
  "EyeSlashFill",
  "Search",
  "Calendar2Range",
  "ChatSquareDots",
  "Reception0",
  "Calendar2WeekFill",
  "FileZip",
  "GlobeCentralSouthAsia",
  "PinMapFill",
  "LayoutSplit",
  "CloudCheckFill",
  "MortarboardFill",
  "0SquareFill",
  "FileLock2",
  "HouseDown",
  "CloudHaze2Fill",
  "ArrowDownShort",
  "BookmarkXFill",
  "EvStation",
  "FilterRight",
  "HeartFill",
  "UnlockFill",
  "FileEarmarkMusicFill",
  "Mouse3",
  "HourglassBottom",
  "Joystick",
  "FileEarmarkMinus",
  "Lightbulb",
  "FileEarmarkSpreadsheet",
  "FiletypeWav",
  "Bag",
  "LightningChargeFill",
  "Calendar3",
  "OpticalAudio",
  "DatabaseCheck",
  "ChevronDoubleRight",
  "Pc",
  "EnvelopePaperFill",
  "FileBreak",
  "Tsunami",
  "CheckSquareFill",
  "6SquareFill",
  "TextRight",
  "SkipStartBtnFill",
  "SuitHeartFill",
  "EarFill",
  "Robot",
  "UsbDriveFill",
  "Mailbox2",
  "FiletypeHeic",
  "BoxSeamFill",
  "FileEarmarkRichtext",
  "Signpost2Fill",
  "Nut",
  "Alexa",
  "FileEarmarkFontFill",
  "CloudDrizzle",
  "Key",
  "DisplayportFill",
  "ClipboardHeart",
  "Sim",
  "Pencil",
  "Save",
  "UmbrellaFill",
  "TabletLandscape",
  "SlashSquareFill",
  "Ethernet",
  "BadgeAdFill",
  "ChevronDown",
  "BadgeWcFill",
  "Eject",
  "Arrow90degRight",
  "ThermometerSun",
  "ChatRightTextFill",
  "FastForwardFill",
  "SignNoParkingFill",
  "FileEarmarkFill",
  "VolumeDownFill",
  "MusicNoteList",
  "EnvelopeExclamationFill",
  "TelephoneX",
  "Virus",
  "Basket3",
  "Box2HeartFill",
  "SignIntersectionSideFill",
  "Hearts",
  "SignMergeLeft",
  "CartPlus",
  "Heart",
  "Intersect",
  "CloudSunFill",
  "ArrowRightCircle",
  "Textarea",
  "SymmetryVertical",
  "PersonAdd",
  "Share",
  "CalendarRangeFill",
  "EnvelopeHeart",
  "ChatSquareDotsFill",
  "Bookshelf",
  "LayerBackward",
  "Calendar4Range",
  "Calendar3Fill",
  "DashCircle",
  "BadgeCc",
  "Spellcheck",
  "FiletypeDoc",
  "Calendar2HeartFill",
  "BadgeHdFill",
  "ReceiptCutoff",
  "BuildingFillLock",
  "GraphDown",
  "CaretDownSquareFill",
  "PersonLinesFill",
  "Sticky",
  "CarFront",
  "1Circle",
  "SinaWeibo",
  "WrenchAdjustableCircleFill",
  "SignTurnSlightLeft",
  "SuitSpadeFill",
  "BookmarkCheck",
  "FuelPumpFill",
  "SortUp",
  "FiletypeMd",
  "SkipEndFill",
  "EnvelopeDashFill",
  "FileEarmarkRuled",
  "Vimeo",
  "Info",
  "SortNumericDown",
  "MenuUp",
  "4Square",
  "CloudLightningFill",
  "TrophyFill",
  "PenFill",
  "FileMinusFill",
  "ArrowDownRightCircle",
  "EmojiAngryFill",
  "AwardFill",
  "Wrench",
  "FileEarmarkText",
  "CloudArrowUp",
  "Star",
  "FileEarmarkRuledFill",
  "Stripe",
  "EnvelopeX",
  "FiletypeAac",
  "EvStationFill",
  "Postage",
  "SkipForward",
  "CartX",
  "CardImage",
  "HeartArrow",
  "Arrow90degLeft",
  "UsbPlugFill",
  "FileEarmarkPlusFill",
  "BagXFill",
  "BagHeart",
  "PersonFillSlash",
  "ChatHeart",
  "DatabaseFillCheck",
  "SignStopFill",
  "UsbC",
  "EmojiLaughing",
  "Heartbreak",
  "Mouse",
  "Lamp",
  "Dice6Fill",
  "CurrencyExchange",
  "EmojiSunglassesFill",
  "Reception1",
  "ClipboardPlusFill",
  "Scissors",
  "Linkedin",
  "ArrowReturnRight",
  "Repeat1",
  "BagHeartFill",
  "TerminalFill",
  "FilePlus",
  "TelephoneOutbound",
  "DatabaseExclamation",
  "FileXFill",
  "FileRichtext",
  "BuildingAdd",
  "Markdown",
  "BookmarkPlusFill",
  "DatabaseFillDash",
  "BlockquoteRight",
  "PersonSlash",
  "FiletypePpt",
  "PatchMinusFill",
  "EnvelopeOpenHeart",
  "Flower1",
  "HouseDoor",
  "CircleSquare",
  "GeoAlt",
  "BuildingFillExclamation",
  "TextIndentRight",
  "PersonCheck",
  "Calendar2Check",
  "PostcardFill",
  "PersonHearts",
  "Check2",
  "Globe",
  "PrinterFill",
  "Valentine2",
  "YinYang",
  "Cash",
  "GraphUpArrow",
  "Calendar3WeekFill",
  "2SquareFill",
  "XLg",
  "Receipt",
  "WindowSplit",
  "PatchQuestion",
  "FileFont",
  "GenderTrans",
  "Tablet",
  "Upc",
  "Clipboard2HeartFill",
  "PlusSquareDotted",
  "CloudHazeFill",
  "Display",
  "FileCode",
  "Medium",
  "SignIntersectionTFill",
  "CalendarMonthFill",
  "TropicalStorm",
  "CloudFogFill",
  "Easel",
  "Hospital",
  "Rss",
  "ChatRightText",
  "PcDisplay",
  "TelephoneInboundFill",
  "FiletypeRaw",
  "ChatSquareQuote",
  "PersonGear",
  "FileWordFill",
  "Calendar3Event",
  "BugFill",
  "EmojiAngry",
  "EggFried",
  "LayersFill",
  "HourglassSplit",
  "Border",
  "Stopwatch",
  "BorderBottom",
  "BadgeCcFill",
  "Rocket",
  "Calendar2Event",
  "PersonFillDash",
  "EnvelopePaper",
  "ChatQuote",
  "InboxFill",
  "FileEarmarkBarGraphFill",
  "HddStackFill",
  "Snapchat",
  "QuestionDiamondFill",
  "Person",
  "Box2",
  "Calendar2RangeFill",
  "Code",
  "PersonDash",
  "CloudSnow",
  "VectorPen",
  "TextParagraph",
  "BorderOuter",
  "LayoutTextWindowReverse",
  "BoxArrowUp",
  "HouseXFill",
  "FullscreenExit",
  "ArrowsAngleExpand",
  "ImageFill",
  "EyeFill",
  "PlusCircleDotted",
  "Boxes",
  "Houses",
  "HouseExclamationFill",
  "Save2Fill",
  "EaselFill",
  "PersonLock",
  "PersonVideo2",
  "EvFrontFill",
  "ListNested",
  "RewindFill",
  "DropletFill",
  "Power",
  "Screwdriver",
  "SegmentedNav",
  "PhoneVibrateFill",
  "CheckAll",
  "Dice4Fill",
  "MenuAppFill",
  "EnvelopePlusFill",
  "BorderLeft",
  "Badge3dFill",
  "Calendar3Week",
  "ChatText",
  "CaretLeftSquare",
  "SlashCircleFill",
  "WrenchAdjustableCircle",
  "ChatDots",
  "BadgeVr",
  "FileEarmarkBreakFill",
  "CloudHail",
  "FolderX",
  "EmojiWink",
  "4SquareFill",
  "ArrowDownLeft",
  "FileEarmarkPostFill",
  "People",
  "HSquare",
  "ChatRightDots",
  "UsbDrive",
  "Signal",
  "GenderFemale",
  "PersonSquare",
  "Truck",
  "SkipBackward",
  "PlusSquareFill",
  "ArrowDownCircleFill",
  "File",
  "App",
  "Grid3x2Gap",
  "Windows",
  "FileZipFill",
  "VolumeMute",
  "4Circle",
  "SignRailroad",
  "FiletypeMp4",
  "HeartbreakFill",
  "CaretDown",
  "MicrosoftTeams",
  "ShieldPlus",
  "SymmetryHorizontal",
  "Clipboard2XFill",
  "FileRichtextFill",
  "ArrowDownRightCircleFill",
  "MapFill",
  "CompassFill",
  "EmojiWinkFill",
  "FilePlay",
  "GraphUp",
  "SignTurnRightFill",
  "5SquareFill",
  "SkipBackwardCircle",
  "SortAlphaUp",
  "HexagonFill",
  "ChatSquareText",
  "SignNoLeftTurn",
  "TvFill",
  "MenuButtonFill",
  "JournalMinus",
  "CartPlusFill",
  "EnvelopeAt",
  "Journal",
  "Percent",
  "Rulers",
  "Toggle2Off",
  "BuildingsFill",
  "Mouse3Fill",
  "FiletypeTiff",
  "HouseFill",
  "BoomboxFill",
  "BuildingFillDash",
  "Clipboard2DataFill",
  "Filter",
  "Sliders2",
  "SignTurnLeft",
  "Radioactive",
  "CaretLeftFill",
  "7SquareFill",
  "CloudHaze2",
  "ArrowRight",
  "Facebook",
  "Magnet",
  "FileSlidesFill",
  "FileExcel",
  "ArrowRightSquareFill",
  "CodeSquare",
  "Calendar2XFill",
  "HouseHeart",
  "FileEarmarkImage",
  "Bucket",
  "CupStraw",
  "FileEarmarkLock2Fill",
  "MoonStarsFill",
  "FilePdfFill",
  "Gear",
  "BoxArrowInRight",
  "FiletypeHtml",
  "Table",
  "ArrowDownRightSquare",
  "Dice3Fill",
  "DatabaseDash",
  "2Circle",
  "Clipboard2Fill",
  "Rewind",
  "CreditCardFill",
  "Dot",
  "NutFill",
  "FiletypeTsx",
  "BookmarkDash",
  "CloudFog",
  "RocketFill",
  "FileEarmarkCheckFill",
  "Bluetooth",
  "Shuffle",
  "BadgeAr",
  "PipFill",
  "Bootstrap",
  "Film",
  "BatteryCharging",
  "Clipboard2Plus",
  "CloudSleetFill",
  "SkipForwardFill",
  "ChevronCompactLeft",
  "MicFill",
  "Safe2Fill",
  "KanbanFill",
  "RewindBtnFill",
  "ReplyFill",
  "Twitter",
  "ToggleOn",
  "CurrencyRupee",
  "BadgeSdFill",
  "PersonVcard",
  "Cpu",
  "CaretDownSquare",
  "NodeMinus",
  "HandbagFill",
  "DoorOpenFill",
  "EvFront",
  "ArrowBarUp",
  "PlayBtn",
  "SendPlus",
  "ChevronBarDown",
  "CaretDownFill",
  "MicMute",
  "PersonFillDown",
  "TypeH1",
  "PlayBtnFill",
  "9Circle",
  "DatabaseFillDown",
  "Unlock",
  "BrightnessAltHigh",
  "ChatDotsFill",
  "Eyeglasses",
  "ArrowsMove",
  "MouseFill",
  "FileEarmarkZip",
  "SlashCircle",
  "Alt",
  "SignIntersectionYFill",
  "BrightnessHighFill",
  "Lock",
  "CloudMoon",
  "HandIndexFill",
  "BorderStyle",
  "CloudRainFill",
  "Telegram",
  "Tag",
  "Trello",
  "CalendarMonth",
  "Safe2",
  "BuildingDash",
  "FiletypeJs",
  "BoxArrowRight",
  "Messenger",
  "ArrowLeft",
  "Palette2",
  "FileEarmarkXFill",
  "FastForwardBtn",
  "FileDiff",
  "BadgeVrFill",
  "Broadcast",
  "PauseCircle",
  "PauseFill",
  "MotherboardFill",
  "ArrowRightSquare",
  "DatabaseFillAdd",
  "FilterSquareFill",
  "Virus2",
  "ChatTextFill",
  "SkipForwardCircleFill",
  "ChatRightQuoteFill",
  "PeaceFill",
  "PinFill",
  "Box2Fill",
  "ChatSquareHeartFill",
  "BarChartLine",
  "CalendarHeart",
  "PhoneFill",
  "SendSlash",
  "TagsFill",
  "Crop",
  "FiletypeTtf",
  "MenuButtonWide",
  "Award",
  "3SquareFill",
  "GripHorizontal",
  "9SquareFill",
  "FileMusic",
  "CursorFill",
  "PersonVideo3",
  "HddFill",
  "SuitClub",
  "SuitClubFill",
  "Collection",
  "Option",
  "ChevronDoubleLeft",
  "SafeFill",
  "BrushFill",
  "6Square",
  "ShopWindow",
  "ShieldExclamation",
  "RecordCircleFill",
  "FilePptFill",
  "Dice1Fill",
  "ArrowBarLeft",
  "HddNetwork",
  "ClockFill",
  "TabletFill",
  "HeartPulseFill",
  "FiletypePdf",
  "BoxArrowInDown",
  "Repeat",
  "BasketFill",
  "Globe2",
  "SunFill",
  "Gift",
  "LayoutSidebarInsetReverse",
  "PlugFill",
  "Phone",
  "AspectRatio",
  "TruckFrontFill",
  "Behance",
  "UniversalAccessCircle",
  "GraphDownArrow",
  "FileEarmarkZipFill",
  "BandaidFill",
  "ArrowDownSquare",
  "CalendarCheckFill",
  "BookFill",
  "UsbMicroFill",
  "Controller",
  "Fonts",
  "EnvelopeExclamation",
  "Pause",
  "BookmarkDashFill",
  "BoxArrowDownLeft",
  "Front",
  "SortNumericDownAlt",
  "Dice6",
  "RocketTakeoffFill",
  "Folder2",
  "WindowDesktop",
  "FileMinus",
  "Mailbox",
  "ThermometerHalf",
  "PersonCheckFill",
  "PlusCircleFill",
  "Flower2",
  "ArrowsExpand",
  "ShieldFillPlus",
  "Flower3",
  "FiletypeMov",
  "PentagonFill",
  "CloudDownloadFill",
  "Shift",
  "UsbMini",
  "FileEarmarkCheck",
  "4CircleFill",
  "NodeMinusFill",
  "Check2All",
  "SignStop",
  "Type",
  "LayoutSidebar",
  "FileArrowDown",
  "BagDash",
  "CpuFill",
  "PCircle",
  "RouterFill",
  "BalloonHeartFill",
  "FileEarmarkEaselFill",
  "Clipboard2Data",
  "PersonBoundingBox",
  "Kanban",
  "BoxArrowInDownLeft",
  "TelephoneFill",
  "PersonExclamation",
  "FileCheck",
  "HouseAdd",
  "FileEaselFill",
  "Shop",
  "ArrowCounterclockwise",
  "Bullseye",
  "GlobeAsiaAustralia",
  "PlusSlashMinus",
  "PassFill",
  "EnvelopeCheck",
  "Wind",
  "5CircleFill",
  "Geo",
  "PciCard",
  "Calendar4Event",
  "MusicPlayerFill",
  "FiletypeJava",
  "FiletypeCss",
  "FileEarmarkDiff",
  "BuildingFillCheck",
  "EnvelopeOpen",
  "PersonFillUp",
  "LaptopFill",
  "DeviceSsdFill",
  "PersonVideo",
  "SkipStartFill",
  "Camera",
  "Ubuntu",
  "CloudUploadFill",
  "DatabaseUp",
  "Fire",
  "BorderInner",
  "BadgeWc",
  "ExclamationSquareFill",
  "BrightnessAltHighFill",
  "Bandaid",
  "Vr",
  "PlusLg",
  "Webcam",
  "PersonFill",
  "PiggyBankFill",
  "SignTurnRight",
  "Calendar2CheckFill",
  "BoxArrowInUpRight",
  "BoundingBoxCircles",
  "SendX",
  "ChevronBarLeft",
  "BarChart",
  "DashSquare",
  "9Square",
  "PhoneVibrate",
  "Outlet",
  "LightningCharge",
  "ArrowUpRight",
  "HandThumbsDownFill",
  "Dice3",
  "GripVertical",
  "Strava",
  "EnvelopeOpenHeartFill",
  "FiletypePhp",
  "WebcamFill",
  "SunriseFill",
  "EmojiNeutralFill",
  "Subtract",
  "Briefcase",
  "BrowserFirefox",
  "Building",
  "SignYieldFill",
  "CaretUpSquareFill",
  "FolderPlus",
  "Dpad",
  "CardText",
  "Calendar4Week",
  "EmojiDizzyFill",
  "PcDisplayHorizontal",
  "BoxArrowInUp",
  "MenuDown",
  "Badge8k",
  "ExclamationDiamondFill",
  "Reddit",
  "PatchExclamationFill",
  "EnvelopeHeartFill",
  "Earbuds",
  "Camera2",
  "Grid3x3",
  "Toggle2On",
  "SignIntersectionSide",
  "FiletypeMp3",
  "MegaphoneFill",
  "PinMap",
  "HouseUpFill",
  "CardList",
  "Motherboard",
  "CreditCard",
  "123",
  "FiletypeXls",
  "StopFill",
  "Regex",
  "Trash3",
  "ArrowClockwise",
  "Trophy",
  "MagnetFill",
  "Cone",
  "ViewList",
  "TypeUnderline",
  "InfoLg",
  "CheckCircleFill",
  "SendFill",
  "FileEarmarkTextFill",
  "Diagram2Fill",
  "Alipay",
  "ArrowLeftCircle",
  "TicketDetailedFill",
  "FiletypeDocx",
  "PhoneLandscapeFill",
  "ChevronCompactUp",
  "Shield",
  "CaretLeftSquareFill",
  "Wechat",
  "ArrowThroughHeartFill",
  "QrCodeScan",
  "SendCheckFill",
  "ChatRightFill",
  "Link45deg",
  "FileImage",
  "CheckCircle",
  "FiletypeScss",
  "HouseExclamation",
  "EmojiKissFill",
  "Stars",
  "ShieldFillMinus",
  "CameraReelsFill",
  "MenuApp",
  "ArrowUpSquareFill",
  "Clipboard2PlusFill",
  "ChatRightQuote",
  "BrightnessHigh",
  "Signpost",
  "FiletypeXml",
  "ExclamationLg",
  "PersonFillExclamation",
  "VolumeOffFill",
  "Send",
  "FileBinary",
  "HouseDownFill",
  "SkipBackwardCircleFill",
  "SkipForwardBtnFill",
  "HouseAddFill",
  "CloudFill",
  "8CircleFill",
  "Clipboard2Pulse",
  "Calendar3EventFill",
  "FileText",
  "Tools",
  "CloudHaze",
  "FiletypeXlsx",
  "CodeSlash",
  "Grid1x2Fill",
  "Upload",
  "FileArrowDownFill",
  "PSquare",
  "ZoomOut",
  "ArrowBarRight",
  "7Square",
  "QuestionOctagonFill",
  "BuildingDown",
  "BlockquoteLeft",
  "WindowSidebar",
  "SignDoNotEnter",
  "FileEarmarkPerson",
  "Displayport",
  "EnvelopePaperHeartFill",
  "Trash2Fill",
  "HddRackFill",
  "ExclamationTriangleFill",
  "JustifyRight",
  "Airplane",
  "Dice4",
  "UsbSymbol",
  "FileBarGraph",
  "TriangleFill",
  "FileMusicFill",
  "Instagram",
  "DeviceHdd",
  "FunnelFill",
  "FiletypeSass",
  "Escape",
  "CurrencyDollar",
  "RSquareFill",
  "CloudSlashFill",
  "HandThumbsUp",
  "Bricks",
  "CardHeading",
  "ClipboardCheckFill",
  "Hdmi",
  "PatchPlus",
  "DashCircleDotted",
  "WrenchAdjustable",
  "Mouse2Fill",
  "Wordpress",
  "PaintBucket",
  "ChatRight",
  "ShiftFill",
  "TelephoneForward",
  "CassetteFill",
  "Slash",
  "FiletypeBmp",
  "BuildingGear",
  "CloudsFill",
  "SignRailroadFill",
  "Grid3x3GapFill",
  "KeyboardFill",
  "ArrowDownLeftCircleFill",
  "TerminalPlus",
  "UiRadios",
  "ArrowUp",
  "ArrowUpSquare",
  "Android",
  "Water",
  "InputCursorText",
  "FileEarmarkLockFill",
  "HospitalFill",
  "SortUpAlt",
  "SimFill",
  "CloudSun",
  "Fingerprint",
  "WifiOff",
  "ArrowDown",
  "FileRuled",
  "Save2",
  "VolumeOff",
  "ClipboardPulse",
  "QuestionLg",
  "FileEarmarkMusic",
  "1Square",
  "AlignBottom",
  "LungsFill",
  "Cassette",
  "ArrowUpRightCircleFill",
  "FiletypeJsx",
  "TelephonePlusFill",
  "MoonStars",
  "Diamond",
  "Bank2",
  "ArrowDownRight",
  "PostcardHeartFill",
  "EggFill",
  "RSquare",
  "RssFill",
  "ShieldLockFill",
  "BadgeVo",
  "Mastodon",
  "TextareaResize",
  "Battery",
  "UpcScan",
  "CreditCard2Back",
  "EmojiKiss",
  "Cloudy",
  "VolumeDown",
  "Easel3",
  "FileEarmarkPdfFill",
  "EnvelopePaperHeart",
  "HandIndex",
  "FiletypeMdx",
  "FuelPump",
  "LampFill",
  "LightbulbFill",
  "SendXFill",
  "AirplaneFill",
  "Bell",
  "BoxArrowUpLeft",
  "SignMergeLeftFill",
  "ArrowDownRightSquareFill",
  "ShieldFill",
  "BadgeVoFill",
  "ShieldShaded",
  "Plug",
  "SuitSpade",
  "TypeStrikethrough",
  "CameraReels",
  "SaveFill",
  "CalendarHeartFill",
  "PencilFill",
  "CloudSleet",
  "TaxiFront",
  "CashCoin",
  "FolderFill",
  "CollectionPlay",
  "Dropbox",
  "SignNoParking",
  "BadgeTmFill",
  "Record2Fill",
  "CircleFill",
  "BagDashFill",
  "ArrowLeftSquare",
  "Calculator",
  "FileEarmark",
  "CalendarPlusFill",
  "EmojiHeartEyesFill",
  "1SquareFill",
  "PlayCircle",
  "Capsule",
  "DatabaseAdd",
  "LayoutWtf",
  "PostageHeartFill",
  "DashSquareFill",
  "AlignStart",
  "EmojiSmileUpsideDownFill",
  "ChevronBarExpand",
  "GeoAltFill",
  "HouseDashFill",
  "SendDashFill",
  "Exclude",
  "EmojiExpressionlessFill",
  "Calendar2Plus",
  "Newspaper",
  "CalendarWeek",
  "UsbPlug",
  "ExclamationOctagon",
  "CashStack",
  "BoxArrowInUpLeft",
  "ReplyAllFill",
  "PauseBtnFill",
  "FileEarmarkSlides",
  "InfoSquareFill",
  "Calendar2MonthFill",
  "House",
  "JournalArrowUp",
  "Reception2",
  "FiletypeCs",
  "BuildingFillX",
  "Octagon",
  "CloudArrowDown",
  "Cloud",
  "ShieldSlashFill",
  "GiftFill",
  "DiscFill",
  "ColumnsGap",
  "SignMergeRight",
  "EmojiFrown",
  "HouseDoorFill",
  "BellSlash",
  "DatabaseLock",
  "OpticalAudioFill",
  "CloudLightningRain",
  "LayoutSidebarReverse",
  "WindowDock",
  "FileEarmarkMedical",
  "CSquareFill",
  "ExclamationCircleFill",
  "LayoutThreeColumns",
  "FileEarmarkExcel",
  "Quora",
  "FiletypeRb",
  "Balloon",
  "BatteryFull",
  "Paperclip",
  "Clipboard",
  "ArrowDownUp",
  "ChatLeftText",
  "CapslockFill",
  "Cart3",
  "Modem",
  "MusicPlayer",
  "SignTurnSlightLeftFill",
  "AirplaneEnginesFill",
  "BuildingSlash",
  "EmojiLaughingFill",
  "Projector",
  "FlagFill",
  "BodyText",
  "Magic",
  "Box",
  "Indent",
  "PeopleFill",
  "FileEarmarkSlidesFill",
  "FileBarGraphFill",
  "Funnel",
  "HouseDash",
  "Stoplights",
  "StopCircleFill",
  "KeyFill",
  "TrainLightrailFrontFill",
  "Steam",
  "Bezier2",
  "CameraVideoFill",
  "GearFill",
  "CupHotFill",
  "ListColumns",
  "XDiamondFill",
  "SortAlphaDown",
  "FileLockFill",
  "Justify",
  "Pip",
  "Layers",
  "BuildingFillAdd",
  "BoxArrowDownRight",
  "BorderCenter",
  "Diagram3",
  "DeviceHddFill",
  "BrightnessAltLow",
  "Chat",
  "TelephoneOutboundFill",
  "CaretRightSquareFill",
  "BroadcastPin",
  "XCircleFill",
  "SkipEndBtnFill",
  "Lungs",
  "Calendar2PlusFill",
  "EmojiSunglasses",
  "Clipboard2CheckFill",
  "Voicemail",
  "ArrowUpLeftSquareFill",
  "PatchExclamation",
  "Easel2Fill",
  "FiletypeKey",
  "FiletypeSvg",
  "SearchHeartFill",
  "FileEarmarkArrowUpFill",
  "CircleHalf",
  "AlignTop",
  "ArrowsCollapse",
  "BuildingFillSlash",
  "SlashLg",
  "EnvelopeCheckFill",
  "FileEarmarkPlayFill",
  "BadgeAd",
  "FileEarmarkPersonFill",
  "HouseGearFill",
  "Git",
  "BellFill",
  "Activity",
  "ChatLeftDots",
  "ShieldLock",
  "Capslock",
  "ArrowUpRightSquare",
  "NodePlus",
  "2Square",
  "CaretRight",
  "BuildingLock",
  "UsbFill",
  "PostageHeart",
  "Coin",
  "ChevronDoubleUp",
  "Grid3x3Gap",
  "Clouds",
  "Folder2Open",
  "FilterSquare",
  "Hexagon",
  "HeptagonHalf",
  "DashCircleFill",
  "FileEasel",
  "Grid1x2",
  "Hdd",
  "QuestionOctagon",
  "SendDash",
  "BagPlusFill",
  "FileEarmarkBarGraph",
  "UiChecks",
  "FileSpreadsheet",
  "Bug",
  "ArrowRepeat",
  "Dice5",
  "3Circle",
  "PauseBtn",
  "Images",
  "SignNoLeftTurnFill",
  "CalendarX",
  "FiletypePng",
  "Clock",
  "AirplaneEngines",
  "EmojiHeartEyes",
  "Book",
  "Sliders2Vertical",
  "CreditCard2Front",
  "FileEarmarkWord",
  "FiletypeCsv",
  "BarChartLineFill",
  "Hurricane",
  "Envelope",
  "OctagonFill",
  "EnvelopeSlash",
  "FileEarmarkSpreadsheetFill",
  "BoxArrowDown",
  "BuildingFillUp",
  "CloudSlash",
  "ClipboardXFill",
  "ArrowLeftRight",
  "Square",
  "FilterLeft",
  "Ladder",
  "FileCodeFill",
  "Badge4kFill",
  "Umbrella",
  "ArrowDownLeftCircle",
  "BracesAsterisk",
  "BrowserEdge",
  "Pen",
  "BootstrapReboot",
  "Smartwatch",
  "Peace",
  "PersonBadge",
  "XOctagonFill",
  "CreditCard2BackFill",
  "BorderWidth",
  "ToggleOff",
  "Basket2Fill",
  "PersonXFill",
  "CloudDrizzleFill",
  "Wallet2",
  "Forward",
  "Android2",
  "HCircle",
  "CupFill",
  "CalendarRange",
  "SignpostSplitFill",
  "EmojiNeutral",
  "DashSquareDotted",
  "PersonCircle",
  "SkipEnd",
  "EjectFill",
  "FolderCheck",
  "FileBreakFill",
  "SignTurnSlightRight",
  "Vinyl",
  "Calendar2X",
  "CollectionFill",
  "Moisture",
  "SlashSquare",
  "Image",
  "Clipboard2Heart",
  "Rainbow",
  "Valentine",
  "HouseLock",
  "ExclamationDiamond",
  "MenuButton",
  "2CircleFill",
  "EnvelopeXFill",
  "Backspace",
  "BadgeArFill",
  "ProjectorFill",
  "ArrowUpLeftSquare",
  "SkipBackwardBtn",
  "FilePlusFill",
  "AlignMiddle",
  "PinAngle",
  "Calendar2Fill",
  "Sunrise",
  "Badge3d",
  "BagFill",
  "Safe",
  "7Circle",
  "Clipboard2MinusFill",
  "FileEarmarkCode",
  "FilePpt",
  "FiletypeJson",
  "Calendar2Week",
  "Unity",
  "WindowFullscreen",
  "TrainFreightFront",
  "FileEarmarkRichtextFill",
  "BadgeTm",
  "Back",
  "SkipStart",
  "ChatQuoteFill",
  "FastForwardCircleFill",
  "BucketFill",
  "SdCardFill",
  "Playstation",
  "FileEarmarkMedicalFill",
  "LockFill",
  "Calendar2Heart",
  "XSquareFill",
  "Pin",
  "Folder",
  "ArrowUpShort",
  "Calendar2Minus",
  "FileArrowUp",
  "Line",
  "HdmiFill",
  "Tiktok",
  "SquareFill",
  "Heptagon",
  "QuestionSquareFill",
  "DpadFill",
  "BoxFill",
  "UsbMiniFill",
  "CartDashFill",
  "PersonFillX",
  "Trash",
  "QuestionDiamond",
  "Egg",
  "Headset",
  "CurrencyEuro",
  "JournalCheck",
  "Calendar2Month",
  "BoxArrowUpRight",
  "SignTurnLeftFill",
  "FileEarmarkFont",
  "ShieldSlash",
  "SkipForwardCircle",
  "Watch",
  "ArrowUpLeft",
  "Calendar2",
  "EnvelopeFill",
  "Skype",
  "RCircle",
  "Calendar2EventFill",
  "PostageFill",
  "UsbCFill",
  "SkipBackwardBtnFill",
  "RecordCircle",
  "BuildingFillDown",
  "GenderMale",
  "Hr",
  "CloudArrowUpFill",
  "Meta",
  "PieChartFill",
  "RCircleFill",
  "TaxiFrontFill",
  "StarFill",
  "Calendar4",
  "SignIntersection",
  "CaretRightSquare",
  "PatchQuestionFill",
  "X",
  "Record",
  "CloudMoonFill",
  "HddNetworkFill",
  "TrainFrontFill",
  "ListCheck",
  "CloudRain",
  "BuildingExclamation",
  "PlayFill",
  "SignDeadEndFill",
  "ShieldMinus",
  "HddRack",
  "CloudArrowDownFill",
  "DatabaseX",
  "CloudPlusFill",
  "SortNumericUp",
  "Archive",
  "BorderAll",
  "CalendarDateFill",
  "PhoneFlip",
  "PersonUp",
  "CartXFill",
  "PaletteFill",
  "Snow",
  "SuitDiamond",
  "CalendarEvent",
  "Grid",
  "PcHorizontal",
  "XDiamond",
  "Speedometer2",
  "SignIntersectionFill",
  "Hash",
  "ExclamationSquare",
  "EmojiSmile",
  "SkipStartCircleFill",
  "DoorClosedFill",
  "ChevronCompactRight",
  "CurrencyPound",
  "FileFill",
  "Boombox",
  "TypeItalic",
  "Toggles2",
  "BriefcaseFill",
  "ModemFill",
  "Calendar",
  "LayersHalf",
  "WalletFill",
  "SendPlusFill",
  "BorderRight",
  "PersonFillAdd",
  "GridFill",
  "PencilSquare",
  "BadgeHd",
  "ChatLeft",
  "8Circle",
  "FileCheckFill",
  "Tags",
  "RecordFill",
  "Cart4",
  "CheckLg",
  "ChatFill",
  "CalendarMinusFill",
  "ChevronBarContract",
  "AlarmFill",
  "CloudDownload",
  "Usb",
  "Diagram3Fill",
  "CcCircle",
  "TextareaT",
  "CalculatorFill",
  "BusFront",
  "Sun",
  "QuestionCircle",
  "CloudPlus",
  "SearchHeart",
  "HandIndexThumbFill",
  "Inboxes",
  "SquareHalf",
  "MenuButtonWideFill",
  "DoorOpen",
  "Ticket",
  "CurrencyYen",
  "Stack",
  "Download",
  "Clipboard2X",
  "MarkdownFill",
  "ForwardFill",
  "Binoculars",
  "Play",
  "Clipboard2Check",
  "StopBtn",
  "PlusSquare",
  "GpuCard",
  "LightningFill",
  "ReplyAll",
  "UiChecksGrid",
  "ClipboardDataFill",
  "RecordBtn",
  "8Square",
  "ShareFill",
  "SortNumericUpAlt",
  "ChatSquareQuoteFill",
  "BusFrontFill",
  "Mouse2",
  "TicketPerforated",
  "ChatLeftHeart",
  "RewindCircleFill",
  "Calendar3Range",
  "AppIndicator",
  "Alarm",
  "CalendarFill",
  "WindowPlus",
  "ChatSquare",
  "Github",
  "HandThumbsDown",
  "Bookmarks",
  "Flag",
  "ShieldFillExclamation",
  "PiggyBank",
  "RecordBtnFill",
  "DeviceSsd",
  "PersonFillGear",
  "SignStopLights",
  "0Square",
  "EnvelopeDash",
  "List",
  "ListColumnsReverse",
  "Command",
  "EmojiFrownFill",
  "ChevronBarRight",
  "HouseSlash",
  "ClipboardFill",
  "InputCursor",
  "ArrowsFullscreen",
  "SkipEndCircle",
  "Cast",
  "TencentQq",
  "Calendar2Date",
  "CcSquareFill",
  "TelephoneInbound",
  "HexagonHalf",
  "Megaphone",
  "ChatSquareFill",
  "FiletypeYml",
  "Dice5Fill",
  "PatchCheckFill",
  "BoxSeam",
  "AlignEnd",
  "BadgeSd",
  "InboxesFill",
  "CameraVideoOff",
  "ClipboardMinus",
  "CloudCheck",
  "FastForward",
  "JournalCode",
  "CcCircleFill",
  "TelephoneXFill",
  "Wallet",
  "ChevronCompactDown",
  "Printer",
  "PersonWorkspace",
  "Calendar2DayFill",
  "CarFrontFill",
  "Stickies",
  "TextWrap",
  "ThunderboltFill",
  "BoxArrowInDownRight",
  "HouseX",
  "FilterCircleFill",
  "JustifyLeft",
  "WindowStack",
  "FastForwardBtnFill",
  "FileBinaryFill",
  "SignNoRightTurn",
  "Badge8kFill",
  "Postcard",
  "5Square",
  "ListTask",
  "ClipboardPlus",
  "JournalArrowDown",
  "EyeSlash",
  "CloudLightning",
  "FileEarmarkPost",
  "BagCheckFill",
  "Spotify",
  "Superscript",
  "HeptagonFill",
  "HSquareFill",
  "AlignCenter",
  "PatchMinus",
  "Mask",
  "CloudUpload",
  "Snow3",
  "MinecartLoaded",
  "CloudMinus",
  "ThermometerHigh",
  "TerminalDash",
  "ThreeDots",
  "ArrowBarDown",
  "Thunderbolt",
  "ThermometerSnow",
  "Microsoft",
  "TelephoneForwardFill",
  "CaretUpFill",
  "Lightning",
  "RewindCircle",
  "Clipboard2",
  "DatabaseFillUp",
  "CupHot",
  "UsbMicro",
  "BuildingX",
  "CloudLightningRainFill",
  "Union",
  "Signpost2",

];

pub mod props;
pub use self::props::IconProps;

pub mod icon_check2_circle;
pub use self::icon_check2_circle::BS_Check2Circle;
pub mod icon_house_up;
pub use self::icon_house_up::BS_HouseUp;
pub mod icon_arrow_down_left_square_fill;
pub use self::icon_arrow_down_left_square_fill::BS_ArrowDownLeftSquareFill;
pub mod icon_filetype_exe;
pub use self::icon_filetype_exe::BS_FiletypeExe;
pub mod icon_tornado;
pub use self::icon_tornado::BS_Tornado;
pub mod icon_view_stacked;
pub use self::icon_view_stacked::BS_ViewStacked;
pub mod icon_droplet_half;
pub use self::icon_droplet_half::BS_DropletHalf;
pub mod icon_trash_fill;
pub use self::icon_trash_fill::BS_TrashFill;
pub mod icon_node_plus_fill;
pub use self::icon_node_plus_fill::BS_NodePlusFill;
pub mod icon_arrow_up_left_circle;
pub use self::icon_arrow_up_left_circle::BS_ArrowUpLeftCircle;
pub mod icon_pentagon_half;
pub use self::icon_pentagon_half::BS_PentagonHalf;
pub mod icon_database_fill;
pub use self::icon_database_fill::BS_DatabaseFill;
pub mod icon_file_earmark_arrow_up;
pub use self::icon_file_earmark_arrow_up::BS_FileEarmarkArrowUp;
pub mod icon_buildings;
pub use self::icon_buildings::BS_Buildings;
pub mod icon_5_circle;
pub use self::icon_5_circle::BS_5Circle;
pub mod icon_filetype_gif;
pub use self::icon_filetype_gif::BS_FiletypeGif;
pub mod icon_border_middle;
pub use self::icon_border_middle::BS_BorderMiddle;
pub mod icon_explicit_fill;
pub use self::icon_explicit_fill::BS_ExplicitFill;
pub mod icon_scooter;
pub use self::icon_scooter::BS_Scooter;
pub mod icon_compass;
pub use self::icon_compass::BS_Compass;
pub mod icon_calendar_date;
pub use self::icon_calendar_date::BS_CalendarDate;
pub mod icon_youtube;
pub use self::icon_youtube::BS_Youtube;
pub mod icon_chat_left_quote_fill;
pub use self::icon_chat_left_quote_fill::BS_ChatLeftQuoteFill;
pub mod icon_cloud_fog2_fill;
pub use self::icon_cloud_fog2_fill::BS_CloudFog2Fill;
pub mod icon_7_circle_fill;
pub use self::icon_7_circle_fill::BS_7CircleFill;
pub mod icon_plus_circle;
pub use self::icon_plus_circle::BS_PlusCircle;
pub mod icon_caret_up_square;
pub use self::icon_caret_up_square::BS_CaretUpSquare;
pub mod icon_explicit;
pub use self::icon_explicit::BS_Explicit;
pub mod icon_check2_square;
pub use self::icon_check2_square::BS_Check2Square;
pub mod icon_c_square;
pub use self::icon_c_square::BS_CSquare;
pub mod icon_patch_check;
pub use self::icon_patch_check::BS_PatchCheck;
pub mod icon_caret_up;
pub use self::icon_caret_up::BS_CaretUp;
pub mod icon_bar_chart_steps;
pub use self::icon_bar_chart_steps::BS_BarChartSteps;
pub mod icon_card_checklist;
pub use self::icon_card_checklist::BS_CardChecklist;
pub mod icon_badge_4k;
pub use self::icon_badge_4k::BS_Badge4k;
pub mod icon_c_circle_fill;
pub use self::icon_c_circle_fill::BS_CCircleFill;
pub mod icon_file_earmark_diff_fill;
pub use self::icon_file_earmark_diff_fill::BS_FileEarmarkDiffFill;
pub mod icon_file_image_fill;
pub use self::icon_file_image_fill::BS_FileImageFill;
pub mod icon_house_slash_fill;
pub use self::icon_house_slash_fill::BS_HouseSlashFill;
pub mod icon_ticket_perforated_fill;
pub use self::icon_ticket_perforated_fill::BS_TicketPerforatedFill;
pub mod icon_p_circle_fill;
pub use self::icon_p_circle_fill::BS_PCircleFill;
pub mod icon_star_half;
pub use self::icon_star_half::BS_StarHalf;
pub mod icon_cloud_hail_fill;
pub use self::icon_cloud_hail_fill::BS_CloudHailFill;
pub mod icon_house_check;
pub use self::icon_house_check::BS_HouseCheck;
pub mod icon_calendar_week_fill;
pub use self::icon_calendar_week_fill::BS_CalendarWeekFill;
pub mod icon_journal_medical;
pub use self::icon_journal_medical::BS_JournalMedical;
pub mod icon_cloud_fog2;
pub use self::icon_cloud_fog2::BS_CloudFog2;
pub mod icon_filetype_woff;
pub use self::icon_filetype_woff::BS_FiletypeWoff;
pub mod icon_bookmark_plus;
pub use self::icon_bookmark_plus::BS_BookmarkPlus;
pub mod icon_volume_up_fill;
pub use self::icon_volume_up_fill::BS_VolumeUpFill;
pub mod icon_file_earmark_easel;
pub use self::icon_file_earmark_easel::BS_FileEarmarkEasel;
pub mod icon_dribbble;
pub use self::icon_dribbble::BS_Dribbble;
pub mod icon_sunset_fill;
pub use self::icon_sunset_fill::BS_SunsetFill;
pub mod icon_person_fill_lock;
pub use self::icon_person_fill_lock::BS_PersonFillLock;
pub mod icon_keyboard;
pub use self::icon_keyboard::BS_Keyboard;
pub mod icon_file_earmark_image_fill;
pub use self::icon_file_earmark_image_fill::BS_FileEarmarkImageFill;
pub mod icon_file_earmark_pdf;
pub use self::icon_file_earmark_pdf::BS_FileEarmarkPdf;
pub mod icon_sort_alpha_down_alt;
pub use self::icon_sort_alpha_down_alt::BS_SortAlphaDownAlt;
pub mod icon_chat_square_text_fill;
pub use self::icon_chat_square_text_fill::BS_ChatSquareTextFill;
pub mod icon_easel2;
pub use self::icon_easel2::BS_Easel2;
pub mod icon_inbox;
pub use self::icon_inbox::BS_Inbox;
pub mod icon_arrow_up_right_circle;
pub use self::icon_arrow_up_right_circle::BS_ArrowUpRightCircle;
pub mod icon_filetype_txt;
pub use self::icon_filetype_txt::BS_FiletypeTxt;
pub mod icon_filetype_sql;
pub use self::icon_filetype_sql::BS_FiletypeSql;
pub mod icon_toggles;
pub use self::icon_toggles::BS_Toggles;
pub mod icon_sort_alpha_up_alt;
pub use self::icon_sort_alpha_up_alt::BS_SortAlphaUpAlt;
pub mod icon_trash3_fill;
pub use self::icon_trash3_fill::BS_Trash3Fill;
pub mod icon_balloon_heart;
pub use self::icon_balloon_heart::BS_BalloonHeart;
pub mod icon_clipboard_heart_fill;
pub use self::icon_clipboard_heart_fill::BS_ClipboardHeartFill;
pub mod icon_file_medical;
pub use self::icon_file_medical::BS_FileMedical;
pub mod icon_building_fill_gear;
pub use self::icon_building_fill_gear::BS_BuildingFillGear;
pub mod icon_train_front;
pub use self::icon_train_front::BS_TrainFront;
pub mod icon_houses_fill;
pub use self::icon_houses_fill::BS_HousesFill;
pub mod icon_thermometer;
pub use self::icon_thermometer::BS_Thermometer;
pub mod icon_0_circle;
pub use self::icon_0_circle::BS_0Circle;
pub mod icon_file_earmark_binary;
pub use self::icon_file_earmark_binary::BS_FileEarmarkBinary;
pub mod icon_chat_left_text_fill;
pub use self::icon_chat_left_text_fill::BS_ChatLeftTextFill;
pub mod icon_bicycle;
pub use self::icon_bicycle::BS_Bicycle;
pub mod icon_translate;
pub use self::icon_translate::BS_Translate;
pub mod icon_file_earmark_plus;
pub use self::icon_file_earmark_plus::BS_FileEarmarkPlus;
pub mod icon_bookmark_fill;
pub use self::icon_bookmark_fill::BS_BookmarkFill;
pub mod icon_chevron_left;
pub use self::icon_chevron_left::BS_ChevronLeft;
pub mod icon_eraser_fill;
pub use self::icon_eraser_fill::BS_EraserFill;
pub mod icon_x_octagon;
pub use self::icon_x_octagon::BS_XOctagon;
pub mod icon_tag_fill;
pub use self::icon_tag_fill::BS_TagFill;
pub mod icon_file_earmark_play;
pub use self::icon_file_earmark_play::BS_FileEarmarkPlay;
pub mod icon_trash2;
pub use self::icon_trash2::BS_Trash2;
pub mod icon_hourglass;
pub use self::icon_hourglass::BS_Hourglass;
pub mod icon_question;
pub use self::icon_question::BS_Question;
pub mod icon_life_preserver;
pub use self::icon_life_preserver::BS_LifePreserver;
pub mod icon_bookmark_star;
pub use self::icon_bookmark_star::BS_BookmarkStar;
pub mod icon_cone_striped;
pub use self::icon_cone_striped::BS_ConeStriped;
pub mod icon_question_circle_fill;
pub use self::icon_question_circle_fill::BS_QuestionCircleFill;
pub mod icon_exclamation_octagon_fill;
pub use self::icon_exclamation_octagon_fill::BS_ExclamationOctagonFill;
pub mod icon_dash_lg;
pub use self::icon_dash_lg::BS_DashLg;
pub mod icon_filetype_jpg;
pub use self::icon_filetype_jpg::BS_FiletypeJpg;
pub mod icon_dice_2;
pub use self::icon_dice_2::BS_Dice2;
pub mod icon_sign_merge_right_fill;
pub use self::icon_sign_merge_right_fill::BS_SignMergeRightFill;
pub mod icon_telephone;
pub use self::icon_telephone::BS_Telephone;
pub mod icon_hammer;
pub use self::icon_hammer::BS_Hammer;
pub mod icon_twitch;
pub use self::icon_twitch::BS_Twitch;
pub mod icon_file_slides;
pub use self::icon_file_slides::BS_FileSlides;
pub mod icon_eraser;
pub use self::icon_eraser::BS_Eraser;
pub mod icon_volume_mute_fill;
pub use self::icon_volume_mute_fill::BS_VolumeMuteFill;
pub mod icon_house_check_fill;
pub use self::icon_house_check_fill::BS_HouseCheckFill;
pub mod icon_filetype_otf;
pub use self::icon_filetype_otf::BS_FiletypeOtf;
pub mod icon_credit_card_2_front_fill;
pub use self::icon_credit_card_2_front_fill::BS_CreditCard2FrontFill;
pub mod icon_tv;
pub use self::icon_tv::BS_Tv;
pub mod icon_file_earmark_excel_fill;
pub use self::icon_file_earmark_excel_fill::BS_FileEarmarkExcelFill;
pub mod icon_emoji_smile_upside_down;
pub use self::icon_emoji_smile_upside_down::BS_EmojiSmileUpsideDown;
pub mod icon_type_h2;
pub use self::icon_type_h2::BS_TypeH2;
pub mod icon_p_square_fill;
pub use self::icon_p_square_fill::BS_PSquareFill;
pub mod icon_layer_forward;
pub use self::icon_layer_forward::BS_LayerForward;
pub mod icon_cursor;
pub use self::icon_cursor::BS_Cursor;
pub mod icon_folder_minus;
pub use self::icon_folder_minus::BS_FolderMinus;
pub mod icon_eyedropper;
pub use self::icon_eyedropper::BS_Eyedropper;
pub mod icon_ticket_fill;
pub use self::icon_ticket_fill::BS_TicketFill;
pub mod icon_stopwatch_fill;
pub use self::icon_stopwatch_fill::BS_StopwatchFill;
pub mod icon_cup;
pub use self::icon_cup::BS_Cup;
pub mod icon_nvidia;
pub use self::icon_nvidia::BS_Nvidia;
pub mod icon_bookmark_heart_fill;
pub use self::icon_bookmark_heart_fill::BS_BookmarkHeartFill;
pub mod icon_ticket_detailed;
pub use self::icon_ticket_detailed::BS_TicketDetailed;
pub mod icon_paypal;
pub use self::icon_paypal::BS_Paypal;
pub mod icon_grid_3x2;
pub use self::icon_grid_3x2::BS_Grid3x2;
pub mod icon_c_circle;
pub use self::icon_c_circle::BS_CCircle;
pub mod icon_mortarboard;
pub use self::icon_mortarboard::BS_Mortarboard;
pub mod icon_chat_square_heart;
pub use self::icon_chat_square_heart::BS_ChatSquareHeart;
pub mod icon_terminal;
pub use self::icon_terminal::BS_Terminal;
pub mod icon_folder_symlink;
pub use self::icon_folder_symlink::BS_FolderSymlink;
pub mod icon_person_heart;
pub use self::icon_person_heart::BS_PersonHeart;
pub mod icon_arrow_right_short;
pub use self::icon_arrow_right_short::BS_ArrowRightShort;
pub mod icon_journal_plus;
pub use self::icon_journal_plus::BS_JournalPlus;
pub mod icon_file_x;
pub use self::icon_file_x::BS_FileX;
pub mod icon_pass;
pub use self::icon_pass::BS_Pass;
pub mod icon_envelope_plus;
pub use self::icon_envelope_plus::BS_EnvelopePlus;
pub mod icon_wifi_1;
pub use self::icon_wifi_1::BS_Wifi1;
pub mod icon_files_alt;
pub use self::icon_files_alt::BS_FilesAlt;
pub mod icon_truck_flatbed;
pub use self::icon_truck_flatbed::BS_TruckFlatbed;
pub mod icon_chat_left_quote;
pub use self::icon_chat_left_quote::BS_ChatLeftQuote;
pub mod icon_house_gear;
pub use self::icon_house_gear::BS_HouseGear;
pub mod icon_folder_symlink_fill;
pub use self::icon_folder_symlink_fill::BS_FolderSymlinkFill;
pub mod icon_diagram_2;
pub use self::icon_diagram_2::BS_Diagram2;
pub mod icon_filetype_psd;
pub use self::icon_filetype_psd::BS_FiletypePsd;
pub mod icon_person_plus;
pub use self::icon_person_plus::BS_PersonPlus;
pub mod icon_emoji_smile_fill;
pub use self::icon_emoji_smile_fill::BS_EmojiSmileFill;
pub mod icon_rewind_btn;
pub use self::icon_rewind_btn::BS_RewindBtn;
pub mod icon_chevron_expand;
pub use self::icon_chevron_expand::BS_ChevronExpand;
pub mod icon_window_dash;
pub use self::icon_window_dash::BS_WindowDash;
pub mod icon_cloud_rain_heavy;
pub use self::icon_cloud_rain_heavy::BS_CloudRainHeavy;
pub mod icon_skip_end_btn;
pub use self::icon_skip_end_btn::BS_SkipEndBtn;
pub mod icon_file_font_fill;
pub use self::icon_file_font_fill::BS_FileFontFill;
pub mod icon_prescription;
pub use self::icon_prescription::BS_Prescription;
pub mod icon_handbag;
pub use self::icon_handbag::BS_Handbag;
pub mod icon_ear;
pub use self::icon_ear::BS_Ear;
pub mod icon_xbox;
pub use self::icon_xbox::BS_Xbox;
pub mod icon_plugin;
pub use self::icon_plugin::BS_Plugin;
pub mod icon_brightness_alt_low_fill;
pub use self::icon_brightness_alt_low_fill::BS_BrightnessAltLowFill;
pub mod icon_chat_right_heart_fill;
pub use self::icon_chat_right_heart_fill::BS_ChatRightHeartFill;
pub mod icon_google_play;
pub use self::icon_google_play::BS_GooglePlay;
pub mod icon_balloon_fill;
pub use self::icon_balloon_fill::BS_BalloonFill;
pub mod icon_file_earmark_arrow_down_fill;
pub use self::icon_file_earmark_arrow_down_fill::BS_FileEarmarkArrowDownFill;
pub mod icon_arrow_90deg_up;
pub use self::icon_arrow_90deg_up::BS_Arrow90degUp;
pub mod icon_mic;
pub use self::icon_mic::BS_Mic;
pub mod icon_1_circle_fill;
pub use self::icon_1_circle_fill::BS_1CircleFill;
pub mod icon_brightness_low;
pub use self::icon_brightness_low::BS_BrightnessLow;
pub mod icon_clipboard2_minus;
pub use self::icon_clipboard2_minus::BS_Clipboard2Minus;
pub mod icon_aspect_ratio_fill;
pub use self::icon_aspect_ratio_fill::BS_AspectRatioFill;
pub mod icon_sign_turn_slight_right_fill;
pub use self::icon_sign_turn_slight_right_fill::BS_SignTurnSlightRightFill;
pub mod icon_window_x;
pub use self::icon_window_x::BS_WindowX;
pub mod icon_chat_left_dots_fill;
pub use self::icon_chat_left_dots_fill::BS_ChatLeftDotsFill;
pub mod icon_sign_dead_end;
pub use self::icon_sign_dead_end::BS_SignDeadEnd;
pub mod icon_chevron_up;
pub use self::icon_chevron_up::BS_ChevronUp;
pub mod icon_bookmark;
pub use self::icon_bookmark::BS_Bookmark;
pub mod icon_file_earmark_code_fill;
pub use self::icon_file_earmark_code_fill::BS_FileEarmarkCodeFill;
pub mod icon_subscript;
pub use self::icon_subscript::BS_Subscript;
pub mod icon_fan;
pub use self::icon_fan::BS_Fan;
pub mod icon_dice_1;
pub use self::icon_dice_1::BS_Dice1;
pub mod icon_sign_stop_lights_fill;
pub use self::icon_sign_stop_lights_fill::BS_SignStopLightsFill;
pub mod icon_arrow_down_left_square;
pub use self::icon_arrow_down_left_square::BS_ArrowDownLeftSquare;
pub mod icon_arrow_up_left_circle_fill;
pub use self::icon_arrow_up_left_circle_fill::BS_ArrowUpLeftCircleFill;
pub mod icon_database_fill_exclamation;
pub use self::icon_database_fill_exclamation::BS_DatabaseFillExclamation;
pub mod icon_speedometer;
pub use self::icon_speedometer::BS_Speedometer;
pub mod icon_chat_heart_fill;
pub use self::icon_chat_heart_fill::BS_ChatHeartFill;
pub mod icon_heart_half;
pub use self::icon_heart_half::BS_HeartHalf;
pub mod icon_shield_fill_check;
pub use self::icon_shield_fill_check::BS_ShieldFillCheck;
pub mod icon_calendar_day;
pub use self::icon_calendar_day::BS_CalendarDay;
pub mod icon_hourglass_top;
pub use self::icon_hourglass_top::BS_HourglassTop;
pub mod icon_backspace_reverse_fill;
pub use self::icon_backspace_reverse_fill::BS_BackspaceReverseFill;
pub mod icon_brush;
pub use self::icon_brush::BS_Brush;
pub mod icon_file_earmark_ppt;
pub use self::icon_file_earmark_ppt::BS_FileEarmarkPpt;
pub mod icon_yelp;
pub use self::icon_yelp::BS_Yelp;
pub mod icon_stop;
pub use self::icon_stop::BS_Stop;
pub mod icon_emoji_expressionless;
pub use self::icon_emoji_expressionless::BS_EmojiExpressionless;
pub mod icon_skip_forward_btn;
pub use self::icon_skip_forward_btn::BS_SkipForwardBtn;
pub mod icon_columns;
pub use self::icon_columns::BS_Columns;
pub mod icon_info_circle_fill;
pub use self::icon_info_circle_fill::BS_InfoCircleFill;
pub mod icon_6_circle_fill;
pub use self::icon_6_circle_fill::BS_6CircleFill;
pub mod icon_qr_code;
pub use self::icon_qr_code::BS_QrCode;
pub mod icon_diamond_fill;
pub use self::icon_diamond_fill::BS_DiamondFill;
pub mod icon_cloud_rain_heavy_fill;
pub use self::icon_cloud_rain_heavy_fill::BS_CloudRainHeavyFill;
pub mod icon_cursor_text;
pub use self::icon_cursor_text::BS_CursorText;
pub mod icon_archive_fill;
pub use self::icon_archive_fill::BS_ArchiveFill;
pub mod icon_chevron_bar_up;
pub use self::icon_chevron_bar_up::BS_ChevronBarUp;
pub mod icon_caret_right_fill;
pub use self::icon_caret_right_fill::BS_CaretRightFill;
pub mod icon_fast_forward_circle;
pub use self::icon_fast_forward_circle::BS_FastForwardCircle;
pub mod icon_map;
pub use self::icon_map::BS_Map;
pub mod icon_cart_fill;
pub use self::icon_cart_fill::BS_CartFill;
pub mod icon_cart_check_fill;
pub use self::icon_cart_check_fill::BS_CartCheckFill;
pub mod icon_file_play_fill;
pub use self::icon_file_play_fill::BS_FilePlayFill;
pub mod icon_layout_sidebar_inset;
pub use self::icon_layout_sidebar_inset::BS_LayoutSidebarInset;
pub mod icon_cc_square;
pub use self::icon_cc_square::BS_CcSquare;
pub mod icon_sd_card;
pub use self::icon_sd_card::BS_SdCard;
pub mod icon_emoji_dizzy;
pub use self::icon_emoji_dizzy::BS_EmojiDizzy;
pub mod icon_wifi;
pub use self::icon_wifi::BS_Wifi;
pub mod icon_thermometer_low;
pub use self::icon_thermometer_low::BS_ThermometerLow;
pub mod icon_file_lock2_fill;
pub use self::icon_file_lock2_fill::BS_FileLock2Fill;
pub mod icon_bag_plus;
pub use self::icon_bag_plus::BS_BagPlus;
pub mod icon_camera_video;
pub use self::icon_camera_video::BS_CameraVideo;
pub mod icon_box_arrow_left;
pub use self::icon_box_arrow_left::BS_BoxArrowLeft;
pub mod icon_bootstrap_fill;
pub use self::icon_bootstrap_fill::BS_BootstrapFill;
pub mod icon_sunset;
pub use self::icon_sunset::BS_Sunset;
pub mod icon_filter_circle;
pub use self::icon_filter_circle::BS_FilterCircle;
pub mod icon_capsule_pill;
pub use self::icon_capsule_pill::BS_CapsulePill;
pub mod icon_send_slash_fill;
pub use self::icon_send_slash_fill::BS_SendSlashFill;
pub mod icon_building_fill;
pub use self::icon_building_fill::BS_BuildingFill;
pub mod icon_files;
pub use self::icon_files::BS_Files;
pub mod icon_bag_check;
pub use self::icon_bag_check::BS_BagCheck;
pub mod icon_journal_bookmark_fill;
pub use self::icon_journal_bookmark_fill::BS_JournalBookmarkFill;
pub mod icon_cloud_snow_fill;
pub use self::icon_cloud_snow_fill::BS_CloudSnowFill;
pub mod icon_octagon_half;
pub use self::icon_octagon_half::BS_OctagonHalf;
pub mod icon_terminal_split;
pub use self::icon_terminal_split::BS_TerminalSplit;
pub mod icon_calendar_minus;
pub use self::icon_calendar_minus::BS_CalendarMinus;
pub mod icon_arrow_left_square_fill;
pub use self::icon_arrow_left_square_fill::BS_ArrowLeftSquareFill;
pub mod icon_chevron_contract;
pub use self::icon_chevron_contract::BS_ChevronContract;
pub mod icon_router;
pub use self::icon_router::BS_Router;
pub mod icon_calendar2_day;
pub use self::icon_calendar2_day::BS_Calendar2Day;
pub mod icon_person_plus_fill;
pub use self::icon_person_plus_fill::BS_PersonPlusFill;
pub mod icon_signpost_fill;
pub use self::icon_signpost_fill::BS_SignpostFill;
pub mod icon_disc;
pub use self::icon_disc::BS_Disc;
pub mod icon_grid_3x2_gap_fill;
pub use self::icon_grid_3x2_gap_fill::BS_Grid3x2GapFill;
pub mod icon_3_square;
pub use self::icon_3_square::BS_3Square;
pub mod icon_basket3_fill;
pub use self::icon_basket3_fill::BS_Basket3Fill;
pub mod icon_arrow_up_circle;
pub use self::icon_arrow_up_circle::BS_ArrowUpCircle;
pub mod icon_globe_europe_africa;
pub use self::icon_globe_europe_africa::BS_GlobeEuropeAfrica;
pub mod icon_file_excel_fill;
pub use self::icon_file_excel_fill::BS_FileExcelFill;
pub mod icon_discord;
pub use self::icon_discord::BS_Discord;
pub mod icon_prescription2;
pub use self::icon_prescription2::BS_Prescription2;
pub mod icon_arrow_return_left;
pub use self::icon_arrow_return_left::BS_ArrowReturnLeft;
pub mod icon_cart;
pub use self::icon_cart::BS_Cart;
pub mod icon_calendar_day_fill;
pub use self::icon_calendar_day_fill::BS_CalendarDayFill;
pub mod icon_3_circle_fill;
pub use self::icon_3_circle_fill::BS_3CircleFill;
pub mod icon_music_note_beamed;
pub use self::icon_music_note_beamed::BS_MusicNoteBeamed;
pub mod icon_arrow_down_circle;
pub use self::icon_arrow_down_circle::BS_ArrowDownCircle;
pub mod icon_cloudy_fill;
pub use self::icon_cloudy_fill::BS_CloudyFill;
pub mod icon_display_fill;
pub use self::icon_display_fill::BS_DisplayFill;
pub mod icon_file_person_fill;
pub use self::icon_file_person_fill::BS_FilePersonFill;
pub mod icon_clipboard_data;
pub use self::icon_clipboard_data::BS_ClipboardData;
pub mod icon_rocket_takeoff;
pub use self::icon_rocket_takeoff::BS_RocketTakeoff;
pub mod icon_person_fill_check;
pub use self::icon_person_fill_check::BS_PersonFillCheck;
pub mod icon_headset_vr;
pub use self::icon_headset_vr::BS_HeadsetVr;
pub mod icon_bookmarks_fill;
pub use self::icon_bookmarks_fill::BS_BookmarksFill;
pub mod icon_database_slash;
pub use self::icon_database_slash::BS_DatabaseSlash;
pub mod icon_distribute_vertical;
pub use self::icon_distribute_vertical::BS_DistributeVertical;
pub mod icon_bar_chart_fill;
pub use self::icon_bar_chart_fill::BS_BarChartFill;
pub mod icon_suit_heart;
pub use self::icon_suit_heart::BS_SuitHeart;
pub mod icon_h_circle_fill;
pub use self::icon_h_circle_fill::BS_HCircleFill;
pub mod icon_server;
pub use self::icon_server::BS_Server;
pub mod icon_distribute_horizontal;
pub use self::icon_distribute_horizontal::BS_DistributeHorizontal;
pub mod icon_sign_intersection_y;
pub use self::icon_sign_intersection_y::BS_SignIntersectionY;
pub mod icon_building_check;
pub use self::icon_building_check::BS_BuildingCheck;
pub mod icon_backspace_reverse;
pub use self::icon_backspace_reverse::BS_BackspaceReverse;
pub mod icon_list_ul;
pub use self::icon_list_ul::BS_ListUl;
pub mod icon_sort_down;
pub use self::icon_sort_down::BS_SortDown;
pub mod icon_file_medical_fill;
pub use self::icon_file_medical_fill::BS_FileMedicalFill;
pub mod icon_file_earmark_minus_fill;
pub use self::icon_file_earmark_minus_fill::BS_FileEarmarkMinusFill;
pub mod icon_shield_fill_x;
pub use self::icon_shield_fill_x::BS_ShieldFillX;
pub mod icon_chat_right_heart;
pub use self::icon_chat_right_heart::BS_ChatRightHeart;
pub mod icon_arrow_down_square_fill;
pub use self::icon_arrow_down_square_fill::BS_ArrowDownSquareFill;
pub mod icon_paragraph;
pub use self::icon_paragraph::BS_Paragraph;
pub mod icon_terminal_x;
pub use self::icon_terminal_x::BS_TerminalX;
pub mod icon_chat_left_fill;
pub use self::icon_chat_left_fill::BS_ChatLeftFill;
pub mod icon_soundwave;
pub use self::icon_soundwave::BS_Soundwave;
pub mod icon_envelope_at_fill;
pub use self::icon_envelope_at_fill::BS_EnvelopeAtFill;
pub mod icon_arrow_90deg_down;
pub use self::icon_arrow_90deg_down::BS_Arrow90degDown;
pub mod icon_file_ruled_fill;
pub use self::icon_file_ruled_fill::BS_FileRuledFill;
pub mod icon_exclamation_triangle;
pub use self::icon_exclamation_triangle::BS_ExclamationTriangle;
pub mod icon_universal_access;
pub use self::icon_universal_access::BS_UniversalAccess;
pub mod icon_send_check;
pub use self::icon_send_check::BS_SendCheck;
pub mod icon_hypnotize;
pub use self::icon_hypnotize::BS_Hypnotize;
pub mod icon_patch_plus_fill;
pub use self::icon_patch_plus_fill::BS_PatchPlusFill;
pub mod icon_cart_dash;
pub use self::icon_cart_dash::BS_CartDash;
pub mod icon_stack_overflow;
pub use self::icon_stack_overflow::BS_StackOverflow;
pub mod icon_bell_slash_fill;
pub use self::icon_bell_slash_fill::BS_BellSlashFill;
pub mod icon_file_earmark_arrow_down;
pub use self::icon_file_earmark_arrow_down::BS_FileEarmarkArrowDown;
pub mod icon_layout_text_sidebar;
pub use self::icon_layout_text_sidebar::BS_LayoutTextSidebar;
pub mod icon_hand_index_thumb;
pub use self::icon_hand_index_thumb::BS_HandIndexThumb;
pub mod icon_stoplights_fill;
pub use self::icon_stoplights_fill::BS_StoplightsFill;
pub mod icon_minecart;
pub use self::icon_minecart::BS_Minecart;
pub mod icon_border_top;
pub use self::icon_border_top::BS_BorderTop;
pub mod icon_fullscreen;
pub use self::icon_fullscreen::BS_Fullscreen;
pub mod icon_file_lock;
pub use self::icon_file_lock::BS_FileLock;
pub mod icon_shield_check;
pub use self::icon_shield_check::BS_ShieldCheck;
pub mod icon_chat_left_heart_fill;
pub use self::icon_chat_left_heart_fill::BS_ChatLeftHeartFill;
pub mod icon_person_down;
pub use self::icon_person_down::BS_PersonDown;
pub mod icon_dash;
pub use self::icon_dash::BS_Dash;
pub mod icon_reception_4;
pub use self::icon_reception_4::BS_Reception4;
pub mod icon_speaker;
pub use self::icon_speaker::BS_Speaker;
pub mod icon_clipboard_x;
pub use self::icon_clipboard_x::BS_ClipboardX;
pub mod icon_bookmark_check_fill;
pub use self::icon_bookmark_check_fill::BS_BookmarkCheckFill;
pub mod icon_binoculars_fill;
pub use self::icon_binoculars_fill::BS_BinocularsFill;
pub mod icon_train_lightrail_front;
pub use self::icon_train_lightrail_front::BS_TrainLightrailFront;
pub mod icon_tree_fill;
pub use self::icon_tree_fill::BS_TreeFill;
pub mod icon_arrow_through_heart;
pub use self::icon_arrow_through_heart::BS_ArrowThroughHeart;
pub mod icon_file_person;
pub use self::icon_file_person::BS_FilePerson;
pub mod icon_house_lock_fill;
pub use self::icon_house_lock_fill::BS_HouseLockFill;
pub mod icon_file_earmark_ppt_fill;
pub use self::icon_file_earmark_ppt_fill::BS_FileEarmarkPptFill;
pub mod icon_tree;
pub use self::icon_tree::BS_Tree;
pub mod icon_pinterest;
pub use self::icon_pinterest::BS_Pinterest;
pub mod icon_gem;
pub use self::icon_gem::BS_Gem;
pub mod icon_clipboard2_pulse_fill;
pub use self::icon_clipboard2_pulse_fill::BS_Clipboard2PulseFill;
pub mod icon_dice_2_fill;
pub use self::icon_dice_2_fill::BS_Dice2Fill;
pub mod icon_cart_check;
pub use self::icon_cart_check::BS_CartCheck;
pub mod icon_fuel_pump_diesel_fill;
pub use self::icon_fuel_pump_diesel_fill::BS_FuelPumpDieselFill;
pub mod icon_three_dots_vertical;
pub use self::icon_three_dots_vertical::BS_ThreeDotsVertical;
pub mod icon_fuel_pump_diesel;
pub use self::icon_fuel_pump_diesel::BS_FuelPumpDiesel;
pub mod icon_x_square;
pub use self::icon_x_square::BS_XSquare;
pub mod icon_person_rolodex;
pub use self::icon_person_rolodex::BS_PersonRolodex;
pub mod icon_calendar2_minus_fill;
pub use self::icon_calendar2_minus_fill::BS_Calendar2MinusFill;
pub mod icon_globe_americas;
pub use self::icon_globe_americas::BS_GlobeAmericas;
pub mod icon_door_closed;
pub use self::icon_door_closed::BS_DoorClosed;
pub mod icon_vinyl_fill;
pub use self::icon_vinyl_fill::BS_VinylFill;
pub mod icon_list_stars;
pub use self::icon_list_stars::BS_ListStars;
pub mod icon_play_circle_fill;
pub use self::icon_play_circle_fill::BS_PlayCircleFill;
pub mod icon_send_exclamation;
pub use self::icon_send_exclamation::BS_SendExclamation;
pub mod icon_bezier;
pub use self::icon_bezier::BS_Bezier;
pub mod icon_pause_circle_fill;
pub use self::icon_pause_circle_fill::BS_PauseCircleFill;
pub mod icon_layout_text_window;
pub use self::icon_layout_text_window::BS_LayoutTextWindow;
pub mod icon_pin_angle_fill;
pub use self::icon_pin_angle_fill::BS_PinAngleFill;
pub mod icon_speaker_fill;
pub use self::icon_speaker_fill::BS_SpeakerFill;
pub mod icon_slack;
pub use self::icon_slack::BS_Slack;
pub mod icon_basket;
pub use self::icon_basket::BS_Basket;
pub mod icon_bookmark_x;
pub use self::icon_bookmark_x::BS_BookmarkX;
pub mod icon_battery_half;
pub use self::icon_battery_half::BS_BatteryHalf;
pub mod icon_calendar_event_fill;
pub use self::icon_calendar_event_fill::BS_CalendarEventFill;
pub mod icon_envelope_slash_fill;
pub use self::icon_envelope_slash_fill::BS_EnvelopeSlashFill;
pub mod icon_puzzle_fill;
pub use self::icon_puzzle_fill::BS_PuzzleFill;
pub mod icon_calendar_check;
pub use self::icon_calendar_check::BS_CalendarCheck;
pub mod icon_image_alt;
pub use self::icon_image_alt::BS_ImageAlt;
pub mod icon_filetype_py;
pub use self::icon_filetype_py::BS_FiletypePy;
pub mod icon_file_post_fill;
pub use self::icon_file_post_fill::BS_FilePostFill;
pub mod icon_braces;
pub use self::icon_braces::BS_Braces;
pub mod icon_skip_end_circle_fill;
pub use self::icon_skip_end_circle_fill::BS_SkipEndCircleFill;
pub mod icon_lightbulb_off_fill;
pub use self::icon_lightbulb_off_fill::BS_LightbulbOffFill;
pub mod icon_browser_chrome;
pub use self::icon_browser_chrome::BS_BrowserChrome;
pub mod icon_plus;
pub use self::icon_plus::BS_Plus;
pub mod icon_gender_ambiguous;
pub use self::icon_gender_ambiguous::BS_GenderAmbiguous;
pub mod icon_sliders;
pub use self::icon_sliders::BS_Sliders;
pub mod icon_whatsapp;
pub use self::icon_whatsapp::BS_Whatsapp;
pub mod icon_brightness_low_fill;
pub use self::icon_brightness_low_fill::BS_BrightnessLowFill;
pub mod icon_skip_backward_fill;
pub use self::icon_skip_backward_fill::BS_SkipBackwardFill;
pub mod icon_person_x;
pub use self::icon_person_x::BS_PersonX;
pub mod icon_volume_up;
pub use self::icon_volume_up::BS_VolumeUp;
pub mod icon_laptop;
pub use self::icon_laptop::BS_Laptop;
pub mod icon_apple;
pub use self::icon_apple::BS_Apple;
pub mod icon_person_dash_fill;
pub use self::icon_person_dash_fill::BS_PersonDashFill;
pub mod icon_unindent;
pub use self::icon_unindent::BS_Unindent;
pub mod icon_moon_fill;
pub use self::icon_moon_fill::BS_MoonFill;
pub mod icon_8_square_fill;
pub use self::icon_8_square_fill::BS_8SquareFill;
pub mod icon_house_heart_fill;
pub use self::icon_house_heart_fill::BS_HouseHeartFill;
pub mod icon_file_arrow_up_fill;
pub use self::icon_file_arrow_up_fill::BS_FileArrowUpFill;
pub mod icon_snow2;
pub use self::icon_snow2::BS_Snow2;
pub mod icon_at;
pub use self::icon_at::BS_At;
pub mod icon_check_square;
pub use self::icon_check_square::BS_CheckSquare;
pub mod icon_file_text_fill;
pub use self::icon_file_text_fill::BS_FileTextFill;
pub mod icon_file_earmark_lock;
pub use self::icon_file_earmark_lock::BS_FileEarmarkLock;
pub mod icon_filetype_m4p;
pub use self::icon_filetype_m4p::BS_FiletypeM4p;
pub mod icon_send_exclamation_fill;
pub use self::icon_send_exclamation_fill::BS_SendExclamationFill;
pub mod icon_telephone_minus;
pub use self::icon_telephone_minus::BS_TelephoneMinus;
pub mod icon_google;
pub use self::icon_google::BS_Google;
pub mod icon_journal_richtext;
pub use self::icon_journal_richtext::BS_JournalRichtext;
pub mod icon_0_circle_fill;
pub use self::icon_0_circle_fill::BS_0CircleFill;
pub mod icon_arrow_left_circle_fill;
pub use self::icon_arrow_left_circle_fill::BS_ArrowLeftCircleFill;
pub mod icon_stop_btn_fill;
pub use self::icon_stop_btn_fill::BS_StopBtnFill;
pub mod icon_moon;
pub use self::icon_moon::BS_Moon;
pub mod icon_tablet_landscape_fill;
pub use self::icon_tablet_landscape_fill::BS_TabletLandscapeFill;
pub mod icon_reply;
pub use self::icon_reply::BS_Reply;
pub mod icon_question_square;
pub use self::icon_question_square::BS_QuestionSquare;
pub mod icon_filetype_ai;
pub use self::icon_filetype_ai::BS_FiletypeAi;
pub mod icon_truck_front;
pub use self::icon_truck_front::BS_TruckFront;
pub mod icon_sign_intersection_t;
pub use self::icon_sign_intersection_t::BS_SignIntersectionT;
pub mod icon_clock_history;
pub use self::icon_clock_history::BS_ClockHistory;
pub mod icon_triangle_half;
pub use self::icon_triangle_half::BS_TriangleHalf;
pub mod icon_sunglasses;
pub use self::icon_sunglasses::BS_Sunglasses;
pub mod icon_type_h3;
pub use self::icon_type_h3::BS_TypeH3;
pub mod icon_6_circle;
pub use self::icon_6_circle::BS_6Circle;
pub mod icon_pentagon;
pub use self::icon_pentagon::BS_Pentagon;
pub mod icon_file_earmark_x;
pub use self::icon_file_earmark_x::BS_FileEarmarkX;
pub mod icon_stickies_fill;
pub use self::icon_stickies_fill::BS_StickiesFill;
pub mod icon_nintendo_switch;
pub use self::icon_nintendo_switch::BS_NintendoSwitch;
pub mod icon_collection_play_fill;
pub use self::icon_collection_play_fill::BS_CollectionPlayFill;
pub mod icon_journal_bookmark;
pub use self::icon_journal_bookmark::BS_JournalBookmark;
pub mod icon_sticky_fill;
pub use self::icon_sticky_fill::BS_StickyFill;
pub mod icon_currency_bitcoin;
pub use self::icon_currency_bitcoin::BS_CurrencyBitcoin;
pub mod icon_backspace_fill;
pub use self::icon_backspace_fill::BS_BackspaceFill;
pub mod icon_text_left;
pub use self::icon_text_left::BS_TextLeft;
pub mod icon_headphones;
pub use self::icon_headphones::BS_Headphones;
pub mod icon_geo_fill;
pub use self::icon_geo_fill::BS_GeoFill;
pub mod icon_cloud_minus_fill;
pub use self::icon_cloud_minus_fill::BS_CloudMinusFill;
pub mod icon_file_word;
pub use self::icon_file_word::BS_FileWord;
pub mod icon_file_diff_fill;
pub use self::icon_file_diff_fill::BS_FileDiffFill;
pub mod icon_lightbulb_off;
pub use self::icon_lightbulb_off::BS_LightbulbOff;
pub mod icon_box2_heart;
pub use self::icon_box2_heart::BS_Box2Heart;
pub mod icon_envelope_open_fill;
pub use self::icon_envelope_open_fill::BS_EnvelopeOpenFill;
pub mod icon_arrow_left_short;
pub use self::icon_arrow_left_short::BS_ArrowLeftShort;
pub mod icon_postcard_heart;
pub use self::icon_postcard_heart::BS_PostcardHeart;
pub mod icon_journals;
pub use self::icon_journals::BS_Journals;
pub mod icon_link;
pub use self::icon_link::BS_Link;
pub mod icon_text_indent_left;
pub use self::icon_text_indent_left::BS_TextIndentLeft;
pub mod icon_book_half;
pub use self::icon_book_half::BS_BookHalf;
pub mod icon_arrow_up_right_square_fill;
pub use self::icon_arrow_up_right_square_fill::BS_ArrowUpRightSquareFill;
pub mod icon_file_spreadsheet_fill;
pub use self::icon_file_spreadsheet_fill::BS_FileSpreadsheetFill;
pub mod icon_ui_radios_grid;
pub use self::icon_ui_radios_grid::BS_UiRadiosGrid;
pub mod icon_bank;
pub use self::icon_bank::BS_Bank;
pub mod icon_filetype_sh;
pub use self::icon_filetype_sh::BS_FiletypeSh;
pub mod icon_bounding_box;
pub use self::icon_bounding_box::BS_BoundingBox;
pub mod icon_camera_fill;
pub use self::icon_camera_fill::BS_CameraFill;
pub mod icon_memory;
pub use self::icon_memory::BS_Memory;
pub mod icon_quote;
pub use self::icon_quote::BS_Quote;
pub mod icon_file_post;
pub use self::icon_file_post::BS_FilePost;
pub mod icon_calendar3_range_fill;
pub use self::icon_calendar3_range_fill::BS_Calendar3RangeFill;
pub mod icon_heart_pulse;
pub use self::icon_heart_pulse::BS_HeartPulse;
pub mod icon_skip_start_circle;
pub use self::icon_skip_start_circle::BS_SkipStartCircle;
pub mod icon_amd;
pub use self::icon_amd::BS_Amd;
pub mod icon_text_center;
pub use self::icon_text_center::BS_TextCenter;
pub mod icon_list_ol;
pub use self::icon_list_ol::BS_ListOl;
pub mod icon_sign_do_not_enter_fill;
pub use self::icon_sign_do_not_enter_fill::BS_SignDoNotEnterFill;
pub mod icon_layout_text_sidebar_reverse;
pub use self::icon_layout_text_sidebar_reverse::BS_LayoutTextSidebarReverse;
pub mod icon_person_vcard_fill;
pub use self::icon_person_vcard_fill::BS_PersonVcardFill;
pub mod icon_diamond_half;
pub use self::icon_diamond_half::BS_DiamondHalf;
pub mod icon_reception_3;
pub use self::icon_reception_3::BS_Reception3;
pub mod icon_sort_down_alt;
pub use self::icon_sort_down_alt::BS_SortDownAlt;
pub mod icon_wifi_2;
pub use self::icon_wifi_2::BS_Wifi2;
pub mod icon_eye;
pub use self::icon_eye::BS_Eye;
pub mod icon_calendar_x_fill;
pub use self::icon_calendar_x_fill::BS_CalendarXFill;
pub mod icon_incognito;
pub use self::icon_incognito::BS_Incognito;
pub mod icon_arrow_up_circle_fill;
pub use self::icon_arrow_up_circle_fill::BS_ArrowUpCircleFill;
pub mod icon_hdd_stack;
pub use self::icon_hdd_stack::BS_HddStack;
pub mod icon_check;
pub use self::icon_check::BS_Check;
pub mod icon_info_square;
pub use self::icon_info_square::BS_InfoSquare;
pub mod icon_file_earmark_break;
pub use self::icon_file_earmark_break::BS_FileEarmarkBreak;
pub mod icon_infinity;
pub use self::icon_infinity::BS_Infinity;
pub mod icon_building_up;
pub use self::icon_building_up::BS_BuildingUp;
pub mod icon_journal_x;
pub use self::icon_journal_x::BS_JournalX;
pub mod icon_calendar_plus;
pub use self::icon_calendar_plus::BS_CalendarPlus;
pub mod icon_exclamation_circle;
pub use self::icon_exclamation_circle::BS_ExclamationCircle;
pub mod icon_database_fill_lock;
pub use self::icon_database_fill_lock::BS_DatabaseFillLock;
pub mod icon_calendar2_date_fill;
pub use self::icon_calendar2_date_fill::BS_Calendar2DateFill;
pub mod icon_palette;
pub use self::icon_palette::BS_Palette;
pub mod icon_window;
pub use self::icon_window::BS_Window;
pub mod icon_bookmark_heart;
pub use self::icon_bookmark_heart::BS_BookmarkHeart;
pub mod icon_asterisk;
pub use self::icon_asterisk::BS_Asterisk;
pub mod icon_basket2;
pub use self::icon_basket2::BS_Basket2;
pub mod icon_mic_mute_fill;
pub use self::icon_mic_mute_fill::BS_MicMuteFill;
pub mod icon_browser_safari;
pub use self::icon_browser_safari::BS_BrowserSafari;
pub mod icon_chat_right_dots_fill;
pub use self::icon_chat_right_dots_fill::BS_ChatRightDotsFill;
pub mod icon_clipboard_minus_fill;
pub use self::icon_clipboard_minus_fill::BS_ClipboardMinusFill;
pub mod icon_circle;
pub use self::icon_circle::BS_Circle;
pub mod icon_easel3_fill;
pub use self::icon_easel3_fill::BS_Easel3Fill;
pub mod icon_train_freight_front_fill;
pub use self::icon_train_freight_front_fill::BS_TrainFreightFrontFill;
pub mod icon_gear_wide;
pub use self::icon_gear_wide::BS_GearWide;
pub mod icon_chevron_right;
pub use self::icon_chevron_right::BS_ChevronRight;
pub mod icon_file_earmark_binary_fill;
pub use self::icon_file_earmark_binary_fill::BS_FileEarmarkBinaryFill;
pub mod icon_wikipedia;
pub use self::icon_wikipedia::BS_Wikipedia;
pub mod icon_9_circle_fill;
pub use self::icon_9_circle_fill::BS_9CircleFill;
pub mod icon_droplet;
pub use self::icon_droplet::BS_Droplet;
pub mod icon_signpost_split;
pub use self::icon_signpost_split::BS_SignpostSplit;
pub mod icon_stop_circle;
pub use self::icon_stop_circle::BS_StopCircle;
pub mod icon_database_fill_gear;
pub use self::icon_database_fill_gear::BS_DatabaseFillGear;
pub mod icon_caret_left;
pub use self::icon_caret_left::BS_CaretLeft;
pub mod icon_info_circle;
pub use self::icon_info_circle::BS_InfoCircle;
pub mod icon_database_fill_x;
pub use self::icon_database_fill_x::BS_DatabaseFillX;
pub mod icon_triangle;
pub use self::icon_triangle::BS_Triangle;
pub mod icon_hand_thumbs_up_fill;
pub use self::icon_hand_thumbs_up_fill::BS_HandThumbsUpFill;
pub mod icon_journal_album;
pub use self::icon_journal_album::BS_JournalAlbum;
pub mod icon_cart2;
pub use self::icon_cart2::BS_Cart2;
pub mod icon_shield_x;
pub use self::icon_shield_x::BS_ShieldX;
pub mod icon_record2;
pub use self::icon_record2::BS_Record2;
pub mod icon_suit_diamond_fill;
pub use self::icon_suit_diamond_fill::BS_SuitDiamondFill;
pub mod icon_journal_text;
pub use self::icon_journal_text::BS_JournalText;
pub mod icon_sign_yield;
pub use self::icon_sign_yield::BS_SignYield;
pub mod icon_puzzle;
pub use self::icon_puzzle::BS_Puzzle;
pub mod icon_telephone_minus_fill;
pub use self::icon_telephone_minus_fill::BS_TelephoneMinusFill;
pub mod icon_file_pdf;
pub use self::icon_file_pdf::BS_FilePdf;
pub mod icon_music_note;
pub use self::icon_music_note::BS_MusicNote;
pub mod icon_arrows_angle_contract;
pub use self::icon_arrows_angle_contract::BS_ArrowsAngleContract;
pub mod icon_box_arrow_in_left;
pub use self::icon_box_arrow_in_left::BS_BoxArrowInLeft;
pub mod icon_recycle;
pub use self::icon_recycle::BS_Recycle;
pub mod icon_chevron_double_down;
pub use self::icon_chevron_double_down::BS_ChevronDoubleDown;
pub mod icon_filetype_pptx;
pub use self::icon_filetype_pptx::BS_FiletypePptx;
pub mod icon_x_circle;
pub use self::icon_x_circle::BS_XCircle;
pub mod icon_type_bold;
pub use self::icon_type_bold::BS_TypeBold;
pub mod icon_phone_landscape;
pub use self::icon_phone_landscape::BS_PhoneLandscape;
pub mod icon_database_down;
pub use self::icon_database_down::BS_DatabaseDown;
pub mod icon_database_fill_slash;
pub use self::icon_database_fill_slash::BS_DatabaseFillSlash;
pub mod icon_person_badge_fill;
pub use self::icon_person_badge_fill::BS_PersonBadgeFill;
pub mod icon_pie_chart;
pub use self::icon_pie_chart::BS_PieChart;
pub mod icon_exclamation;
pub use self::icon_exclamation::BS_Exclamation;
pub mod icon_file_earmark_lock2;
pub use self::icon_file_earmark_lock2::BS_FileEarmarkLock2;
pub mod icon_database_gear;
pub use self::icon_database_gear::BS_DatabaseGear;
pub mod icon_bookmark_star_fill;
pub use self::icon_bookmark_star_fill::BS_BookmarkStarFill;
pub mod icon_skip_start_btn;
pub use self::icon_skip_start_btn::BS_SkipStartBtn;
pub mod icon_sign_no_right_turn_fill;
pub use self::icon_sign_no_right_turn_fill::BS_SignNoRightTurnFill;
pub mod icon_bag_x;
pub use self::icon_bag_x::BS_BagX;
pub mod icon_zoom_in;
pub use self::icon_zoom_in::BS_ZoomIn;
pub mod icon_arrow_right_circle_fill;
pub use self::icon_arrow_right_circle_fill::BS_ArrowRightCircleFill;
pub mod icon_clipboard_check;
pub use self::icon_clipboard_check::BS_ClipboardCheck;
pub mod icon_camera_video_off_fill;
pub use self::icon_camera_video_off_fill::BS_CameraVideoOffFill;
pub mod icon_database;
pub use self::icon_database::BS_Database;
pub mod icon_gear_wide_connected;
pub use self::icon_gear_wide_connected::BS_GearWideConnected;
pub mod icon_telephone_plus;
pub use self::icon_telephone_plus::BS_TelephonePlus;
pub mod icon_file_earmark_word_fill;
pub use self::icon_file_earmark_word_fill::BS_FileEarmarkWordFill;
pub mod icon_eye_slash_fill;
pub use self::icon_eye_slash_fill::BS_EyeSlashFill;
pub mod icon_search;
pub use self::icon_search::BS_Search;
pub mod icon_calendar2_range;
pub use self::icon_calendar2_range::BS_Calendar2Range;
pub mod icon_chat_square_dots;
pub use self::icon_chat_square_dots::BS_ChatSquareDots;
pub mod icon_reception_0;
pub use self::icon_reception_0::BS_Reception0;
pub mod icon_calendar2_week_fill;
pub use self::icon_calendar2_week_fill::BS_Calendar2WeekFill;
pub mod icon_file_zip;
pub use self::icon_file_zip::BS_FileZip;
pub mod icon_globe_central_south_asia;
pub use self::icon_globe_central_south_asia::BS_GlobeCentralSouthAsia;
pub mod icon_pin_map_fill;
pub use self::icon_pin_map_fill::BS_PinMapFill;
pub mod icon_layout_split;
pub use self::icon_layout_split::BS_LayoutSplit;
pub mod icon_cloud_check_fill;
pub use self::icon_cloud_check_fill::BS_CloudCheckFill;
pub mod icon_mortarboard_fill;
pub use self::icon_mortarboard_fill::BS_MortarboardFill;
pub mod icon_0_square_fill;
pub use self::icon_0_square_fill::BS_0SquareFill;
pub mod icon_file_lock2;
pub use self::icon_file_lock2::BS_FileLock2;
pub mod icon_house_down;
pub use self::icon_house_down::BS_HouseDown;
pub mod icon_cloud_haze2_fill;
pub use self::icon_cloud_haze2_fill::BS_CloudHaze2Fill;
pub mod icon_arrow_down_short;
pub use self::icon_arrow_down_short::BS_ArrowDownShort;
pub mod icon_bookmark_x_fill;
pub use self::icon_bookmark_x_fill::BS_BookmarkXFill;
pub mod icon_ev_station;
pub use self::icon_ev_station::BS_EvStation;
pub mod icon_filter_right;
pub use self::icon_filter_right::BS_FilterRight;
pub mod icon_heart_fill;
pub use self::icon_heart_fill::BS_HeartFill;
pub mod icon_unlock_fill;
pub use self::icon_unlock_fill::BS_UnlockFill;
pub mod icon_file_earmark_music_fill;
pub use self::icon_file_earmark_music_fill::BS_FileEarmarkMusicFill;
pub mod icon_mouse3;
pub use self::icon_mouse3::BS_Mouse3;
pub mod icon_hourglass_bottom;
pub use self::icon_hourglass_bottom::BS_HourglassBottom;
pub mod icon_joystick;
pub use self::icon_joystick::BS_Joystick;
pub mod icon_file_earmark_minus;
pub use self::icon_file_earmark_minus::BS_FileEarmarkMinus;
pub mod icon_lightbulb;
pub use self::icon_lightbulb::BS_Lightbulb;
pub mod icon_file_earmark_spreadsheet;
pub use self::icon_file_earmark_spreadsheet::BS_FileEarmarkSpreadsheet;
pub mod icon_filetype_wav;
pub use self::icon_filetype_wav::BS_FiletypeWav;
pub mod icon_bag;
pub use self::icon_bag::BS_Bag;
pub mod icon_lightning_charge_fill;
pub use self::icon_lightning_charge_fill::BS_LightningChargeFill;
pub mod icon_calendar3;
pub use self::icon_calendar3::BS_Calendar3;
pub mod icon_optical_audio;
pub use self::icon_optical_audio::BS_OpticalAudio;
pub mod icon_database_check;
pub use self::icon_database_check::BS_DatabaseCheck;
pub mod icon_chevron_double_right;
pub use self::icon_chevron_double_right::BS_ChevronDoubleRight;
pub mod icon_pc;
pub use self::icon_pc::BS_Pc;
pub mod icon_envelope_paper_fill;
pub use self::icon_envelope_paper_fill::BS_EnvelopePaperFill;
pub mod icon_file_break;
pub use self::icon_file_break::BS_FileBreak;
pub mod icon_tsunami;
pub use self::icon_tsunami::BS_Tsunami;
pub mod icon_check_square_fill;
pub use self::icon_check_square_fill::BS_CheckSquareFill;
pub mod icon_6_square_fill;
pub use self::icon_6_square_fill::BS_6SquareFill;
pub mod icon_text_right;
pub use self::icon_text_right::BS_TextRight;
pub mod icon_skip_start_btn_fill;
pub use self::icon_skip_start_btn_fill::BS_SkipStartBtnFill;
pub mod icon_suit_heart_fill;
pub use self::icon_suit_heart_fill::BS_SuitHeartFill;
pub mod icon_ear_fill;
pub use self::icon_ear_fill::BS_EarFill;
pub mod icon_robot;
pub use self::icon_robot::BS_Robot;
pub mod icon_usb_drive_fill;
pub use self::icon_usb_drive_fill::BS_UsbDriveFill;
pub mod icon_mailbox2;
pub use self::icon_mailbox2::BS_Mailbox2;
pub mod icon_filetype_heic;
pub use self::icon_filetype_heic::BS_FiletypeHeic;
pub mod icon_box_seam_fill;
pub use self::icon_box_seam_fill::BS_BoxSeamFill;
pub mod icon_file_earmark_richtext;
pub use self::icon_file_earmark_richtext::BS_FileEarmarkRichtext;
pub mod icon_signpost_2_fill;
pub use self::icon_signpost_2_fill::BS_Signpost2Fill;
pub mod icon_nut;
pub use self::icon_nut::BS_Nut;
pub mod icon_alexa;
pub use self::icon_alexa::BS_Alexa;
pub mod icon_file_earmark_font_fill;
pub use self::icon_file_earmark_font_fill::BS_FileEarmarkFontFill;
pub mod icon_cloud_drizzle;
pub use self::icon_cloud_drizzle::BS_CloudDrizzle;
pub mod icon_key;
pub use self::icon_key::BS_Key;
pub mod icon_displayport_fill;
pub use self::icon_displayport_fill::BS_DisplayportFill;
pub mod icon_clipboard_heart;
pub use self::icon_clipboard_heart::BS_ClipboardHeart;
pub mod icon_sim;
pub use self::icon_sim::BS_Sim;
pub mod icon_pencil;
pub use self::icon_pencil::BS_Pencil;
pub mod icon_save;
pub use self::icon_save::BS_Save;
pub mod icon_umbrella_fill;
pub use self::icon_umbrella_fill::BS_UmbrellaFill;
pub mod icon_tablet_landscape;
pub use self::icon_tablet_landscape::BS_TabletLandscape;
pub mod icon_slash_square_fill;
pub use self::icon_slash_square_fill::BS_SlashSquareFill;
pub mod icon_ethernet;
pub use self::icon_ethernet::BS_Ethernet;
pub mod icon_badge_ad_fill;
pub use self::icon_badge_ad_fill::BS_BadgeAdFill;
pub mod icon_chevron_down;
pub use self::icon_chevron_down::BS_ChevronDown;
pub mod icon_badge_wc_fill;
pub use self::icon_badge_wc_fill::BS_BadgeWcFill;
pub mod icon_eject;
pub use self::icon_eject::BS_Eject;
pub mod icon_arrow_90deg_right;
pub use self::icon_arrow_90deg_right::BS_Arrow90degRight;
pub mod icon_thermometer_sun;
pub use self::icon_thermometer_sun::BS_ThermometerSun;
pub mod icon_chat_right_text_fill;
pub use self::icon_chat_right_text_fill::BS_ChatRightTextFill;
pub mod icon_fast_forward_fill;
pub use self::icon_fast_forward_fill::BS_FastForwardFill;
pub mod icon_sign_no_parking_fill;
pub use self::icon_sign_no_parking_fill::BS_SignNoParkingFill;
pub mod icon_file_earmark_fill;
pub use self::icon_file_earmark_fill::BS_FileEarmarkFill;
pub mod icon_volume_down_fill;
pub use self::icon_volume_down_fill::BS_VolumeDownFill;
pub mod icon_music_note_list;
pub use self::icon_music_note_list::BS_MusicNoteList;
pub mod icon_envelope_exclamation_fill;
pub use self::icon_envelope_exclamation_fill::BS_EnvelopeExclamationFill;
pub mod icon_telephone_x;
pub use self::icon_telephone_x::BS_TelephoneX;
pub mod icon_virus;
pub use self::icon_virus::BS_Virus;
pub mod icon_basket3;
pub use self::icon_basket3::BS_Basket3;
pub mod icon_box2_heart_fill;
pub use self::icon_box2_heart_fill::BS_Box2HeartFill;
pub mod icon_sign_intersection_side_fill;
pub use self::icon_sign_intersection_side_fill::BS_SignIntersectionSideFill;
pub mod icon_hearts;
pub use self::icon_hearts::BS_Hearts;
pub mod icon_sign_merge_left;
pub use self::icon_sign_merge_left::BS_SignMergeLeft;
pub mod icon_cart_plus;
pub use self::icon_cart_plus::BS_CartPlus;
pub mod icon_heart;
pub use self::icon_heart::BS_Heart;
pub mod icon_intersect;
pub use self::icon_intersect::BS_Intersect;
pub mod icon_cloud_sun_fill;
pub use self::icon_cloud_sun_fill::BS_CloudSunFill;
pub mod icon_arrow_right_circle;
pub use self::icon_arrow_right_circle::BS_ArrowRightCircle;
pub mod icon_textarea;
pub use self::icon_textarea::BS_Textarea;
pub mod icon_symmetry_vertical;
pub use self::icon_symmetry_vertical::BS_SymmetryVertical;
pub mod icon_person_add;
pub use self::icon_person_add::BS_PersonAdd;
pub mod icon_share;
pub use self::icon_share::BS_Share;
pub mod icon_calendar_range_fill;
pub use self::icon_calendar_range_fill::BS_CalendarRangeFill;
pub mod icon_envelope_heart;
pub use self::icon_envelope_heart::BS_EnvelopeHeart;
pub mod icon_chat_square_dots_fill;
pub use self::icon_chat_square_dots_fill::BS_ChatSquareDotsFill;
pub mod icon_bookshelf;
pub use self::icon_bookshelf::BS_Bookshelf;
pub mod icon_layer_backward;
pub use self::icon_layer_backward::BS_LayerBackward;
pub mod icon_calendar4_range;
pub use self::icon_calendar4_range::BS_Calendar4Range;
pub mod icon_calendar3_fill;
pub use self::icon_calendar3_fill::BS_Calendar3Fill;
pub mod icon_dash_circle;
pub use self::icon_dash_circle::BS_DashCircle;
pub mod icon_badge_cc;
pub use self::icon_badge_cc::BS_BadgeCc;
pub mod icon_spellcheck;
pub use self::icon_spellcheck::BS_Spellcheck;
pub mod icon_filetype_doc;
pub use self::icon_filetype_doc::BS_FiletypeDoc;
pub mod icon_calendar2_heart_fill;
pub use self::icon_calendar2_heart_fill::BS_Calendar2HeartFill;
pub mod icon_badge_hd_fill;
pub use self::icon_badge_hd_fill::BS_BadgeHdFill;
pub mod icon_receipt_cutoff;
pub use self::icon_receipt_cutoff::BS_ReceiptCutoff;
pub mod icon_building_fill_lock;
pub use self::icon_building_fill_lock::BS_BuildingFillLock;
pub mod icon_graph_down;
pub use self::icon_graph_down::BS_GraphDown;
pub mod icon_caret_down_square_fill;
pub use self::icon_caret_down_square_fill::BS_CaretDownSquareFill;
pub mod icon_person_lines_fill;
pub use self::icon_person_lines_fill::BS_PersonLinesFill;
pub mod icon_sticky;
pub use self::icon_sticky::BS_Sticky;
pub mod icon_car_front;
pub use self::icon_car_front::BS_CarFront;
pub mod icon_1_circle;
pub use self::icon_1_circle::BS_1Circle;
pub mod icon_sina_weibo;
pub use self::icon_sina_weibo::BS_SinaWeibo;
pub mod icon_wrench_adjustable_circle_fill;
pub use self::icon_wrench_adjustable_circle_fill::BS_WrenchAdjustableCircleFill;
pub mod icon_sign_turn_slight_left;
pub use self::icon_sign_turn_slight_left::BS_SignTurnSlightLeft;
pub mod icon_suit_spade_fill;
pub use self::icon_suit_spade_fill::BS_SuitSpadeFill;
pub mod icon_bookmark_check;
pub use self::icon_bookmark_check::BS_BookmarkCheck;
pub mod icon_fuel_pump_fill;
pub use self::icon_fuel_pump_fill::BS_FuelPumpFill;
pub mod icon_sort_up;
pub use self::icon_sort_up::BS_SortUp;
pub mod icon_filetype_md;
pub use self::icon_filetype_md::BS_FiletypeMd;
pub mod icon_skip_end_fill;
pub use self::icon_skip_end_fill::BS_SkipEndFill;
pub mod icon_envelope_dash_fill;
pub use self::icon_envelope_dash_fill::BS_EnvelopeDashFill;
pub mod icon_file_earmark_ruled;
pub use self::icon_file_earmark_ruled::BS_FileEarmarkRuled;
pub mod icon_vimeo;
pub use self::icon_vimeo::BS_Vimeo;
pub mod icon_info;
pub use self::icon_info::BS_Info;
pub mod icon_sort_numeric_down;
pub use self::icon_sort_numeric_down::BS_SortNumericDown;
pub mod icon_menu_up;
pub use self::icon_menu_up::BS_MenuUp;
pub mod icon_4_square;
pub use self::icon_4_square::BS_4Square;
pub mod icon_cloud_lightning_fill;
pub use self::icon_cloud_lightning_fill::BS_CloudLightningFill;
pub mod icon_trophy_fill;
pub use self::icon_trophy_fill::BS_TrophyFill;
pub mod icon_pen_fill;
pub use self::icon_pen_fill::BS_PenFill;
pub mod icon_file_minus_fill;
pub use self::icon_file_minus_fill::BS_FileMinusFill;
pub mod icon_arrow_down_right_circle;
pub use self::icon_arrow_down_right_circle::BS_ArrowDownRightCircle;
pub mod icon_emoji_angry_fill;
pub use self::icon_emoji_angry_fill::BS_EmojiAngryFill;
pub mod icon_award_fill;
pub use self::icon_award_fill::BS_AwardFill;
pub mod icon_wrench;
pub use self::icon_wrench::BS_Wrench;
pub mod icon_file_earmark_text;
pub use self::icon_file_earmark_text::BS_FileEarmarkText;
pub mod icon_cloud_arrow_up;
pub use self::icon_cloud_arrow_up::BS_CloudArrowUp;
pub mod icon_star;
pub use self::icon_star::BS_Star;
pub mod icon_file_earmark_ruled_fill;
pub use self::icon_file_earmark_ruled_fill::BS_FileEarmarkRuledFill;
pub mod icon_stripe;
pub use self::icon_stripe::BS_Stripe;
pub mod icon_envelope_x;
pub use self::icon_envelope_x::BS_EnvelopeX;
pub mod icon_filetype_aac;
pub use self::icon_filetype_aac::BS_FiletypeAac;
pub mod icon_ev_station_fill;
pub use self::icon_ev_station_fill::BS_EvStationFill;
pub mod icon_postage;
pub use self::icon_postage::BS_Postage;
pub mod icon_skip_forward;
pub use self::icon_skip_forward::BS_SkipForward;
pub mod icon_cart_x;
pub use self::icon_cart_x::BS_CartX;
pub mod icon_card_image;
pub use self::icon_card_image::BS_CardImage;
pub mod icon_heart_arrow;
pub use self::icon_heart_arrow::BS_HeartArrow;
pub mod icon_arrow_90deg_left;
pub use self::icon_arrow_90deg_left::BS_Arrow90degLeft;
pub mod icon_usb_plug_fill;
pub use self::icon_usb_plug_fill::BS_UsbPlugFill;
pub mod icon_file_earmark_plus_fill;
pub use self::icon_file_earmark_plus_fill::BS_FileEarmarkPlusFill;
pub mod icon_bag_x_fill;
pub use self::icon_bag_x_fill::BS_BagXFill;
pub mod icon_bag_heart;
pub use self::icon_bag_heart::BS_BagHeart;
pub mod icon_person_fill_slash;
pub use self::icon_person_fill_slash::BS_PersonFillSlash;
pub mod icon_chat_heart;
pub use self::icon_chat_heart::BS_ChatHeart;
pub mod icon_database_fill_check;
pub use self::icon_database_fill_check::BS_DatabaseFillCheck;
pub mod icon_sign_stop_fill;
pub use self::icon_sign_stop_fill::BS_SignStopFill;
pub mod icon_usb_c;
pub use self::icon_usb_c::BS_UsbC;
pub mod icon_emoji_laughing;
pub use self::icon_emoji_laughing::BS_EmojiLaughing;
pub mod icon_heartbreak;
pub use self::icon_heartbreak::BS_Heartbreak;
pub mod icon_mouse;
pub use self::icon_mouse::BS_Mouse;
pub mod icon_lamp;
pub use self::icon_lamp::BS_Lamp;
pub mod icon_dice_6_fill;
pub use self::icon_dice_6_fill::BS_Dice6Fill;
pub mod icon_currency_exchange;
pub use self::icon_currency_exchange::BS_CurrencyExchange;
pub mod icon_emoji_sunglasses_fill;
pub use self::icon_emoji_sunglasses_fill::BS_EmojiSunglassesFill;
pub mod icon_reception_1;
pub use self::icon_reception_1::BS_Reception1;
pub mod icon_clipboard_plus_fill;
pub use self::icon_clipboard_plus_fill::BS_ClipboardPlusFill;
pub mod icon_scissors;
pub use self::icon_scissors::BS_Scissors;
pub mod icon_linkedin;
pub use self::icon_linkedin::BS_Linkedin;
pub mod icon_arrow_return_right;
pub use self::icon_arrow_return_right::BS_ArrowReturnRight;
pub mod icon_repeat_1;
pub use self::icon_repeat_1::BS_Repeat1;
pub mod icon_bag_heart_fill;
pub use self::icon_bag_heart_fill::BS_BagHeartFill;
pub mod icon_terminal_fill;
pub use self::icon_terminal_fill::BS_TerminalFill;
pub mod icon_file_plus;
pub use self::icon_file_plus::BS_FilePlus;
pub mod icon_telephone_outbound;
pub use self::icon_telephone_outbound::BS_TelephoneOutbound;
pub mod icon_database_exclamation;
pub use self::icon_database_exclamation::BS_DatabaseExclamation;
pub mod icon_file_x_fill;
pub use self::icon_file_x_fill::BS_FileXFill;
pub mod icon_file_richtext;
pub use self::icon_file_richtext::BS_FileRichtext;
pub mod icon_building_add;
pub use self::icon_building_add::BS_BuildingAdd;
pub mod icon_markdown;
pub use self::icon_markdown::BS_Markdown;
pub mod icon_bookmark_plus_fill;
pub use self::icon_bookmark_plus_fill::BS_BookmarkPlusFill;
pub mod icon_database_fill_dash;
pub use self::icon_database_fill_dash::BS_DatabaseFillDash;
pub mod icon_blockquote_right;
pub use self::icon_blockquote_right::BS_BlockquoteRight;
pub mod icon_person_slash;
pub use self::icon_person_slash::BS_PersonSlash;
pub mod icon_filetype_ppt;
pub use self::icon_filetype_ppt::BS_FiletypePpt;
pub mod icon_patch_minus_fill;
pub use self::icon_patch_minus_fill::BS_PatchMinusFill;
pub mod icon_envelope_open_heart;
pub use self::icon_envelope_open_heart::BS_EnvelopeOpenHeart;
pub mod icon_flower1;
pub use self::icon_flower1::BS_Flower1;
pub mod icon_house_door;
pub use self::icon_house_door::BS_HouseDoor;
pub mod icon_circle_square;
pub use self::icon_circle_square::BS_CircleSquare;
pub mod icon_geo_alt;
pub use self::icon_geo_alt::BS_GeoAlt;
pub mod icon_building_fill_exclamation;
pub use self::icon_building_fill_exclamation::BS_BuildingFillExclamation;
pub mod icon_text_indent_right;
pub use self::icon_text_indent_right::BS_TextIndentRight;
pub mod icon_person_check;
pub use self::icon_person_check::BS_PersonCheck;
pub mod icon_calendar2_check;
pub use self::icon_calendar2_check::BS_Calendar2Check;
pub mod icon_postcard_fill;
pub use self::icon_postcard_fill::BS_PostcardFill;
pub mod icon_person_hearts;
pub use self::icon_person_hearts::BS_PersonHearts;
pub mod icon_check2;
pub use self::icon_check2::BS_Check2;
pub mod icon_globe;
pub use self::icon_globe::BS_Globe;
pub mod icon_printer_fill;
pub use self::icon_printer_fill::BS_PrinterFill;
pub mod icon_valentine2;
pub use self::icon_valentine2::BS_Valentine2;
pub mod icon_yin_yang;
pub use self::icon_yin_yang::BS_YinYang;
pub mod icon_cash;
pub use self::icon_cash::BS_Cash;
pub mod icon_graph_up_arrow;
pub use self::icon_graph_up_arrow::BS_GraphUpArrow;
pub mod icon_calendar3_week_fill;
pub use self::icon_calendar3_week_fill::BS_Calendar3WeekFill;
pub mod icon_2_square_fill;
pub use self::icon_2_square_fill::BS_2SquareFill;
pub mod icon_x_lg;
pub use self::icon_x_lg::BS_XLg;
pub mod icon_receipt;
pub use self::icon_receipt::BS_Receipt;
pub mod icon_window_split;
pub use self::icon_window_split::BS_WindowSplit;
pub mod icon_patch_question;
pub use self::icon_patch_question::BS_PatchQuestion;
pub mod icon_file_font;
pub use self::icon_file_font::BS_FileFont;
pub mod icon_gender_trans;
pub use self::icon_gender_trans::BS_GenderTrans;
pub mod icon_tablet;
pub use self::icon_tablet::BS_Tablet;
pub mod icon_upc;
pub use self::icon_upc::BS_Upc;
pub mod icon_clipboard2_heart_fill;
pub use self::icon_clipboard2_heart_fill::BS_Clipboard2HeartFill;
pub mod icon_plus_square_dotted;
pub use self::icon_plus_square_dotted::BS_PlusSquareDotted;
pub mod icon_cloud_haze_fill;
pub use self::icon_cloud_haze_fill::BS_CloudHazeFill;
pub mod icon_display;
pub use self::icon_display::BS_Display;
pub mod icon_file_code;
pub use self::icon_file_code::BS_FileCode;
pub mod icon_medium;
pub use self::icon_medium::BS_Medium;
pub mod icon_sign_intersection_t_fill;
pub use self::icon_sign_intersection_t_fill::BS_SignIntersectionTFill;
pub mod icon_calendar_month_fill;
pub use self::icon_calendar_month_fill::BS_CalendarMonthFill;
pub mod icon_tropical_storm;
pub use self::icon_tropical_storm::BS_TropicalStorm;
pub mod icon_cloud_fog_fill;
pub use self::icon_cloud_fog_fill::BS_CloudFogFill;
pub mod icon_easel;
pub use self::icon_easel::BS_Easel;
pub mod icon_hospital;
pub use self::icon_hospital::BS_Hospital;
pub mod icon_rss;
pub use self::icon_rss::BS_Rss;
pub mod icon_chat_right_text;
pub use self::icon_chat_right_text::BS_ChatRightText;
pub mod icon_pc_display;
pub use self::icon_pc_display::BS_PcDisplay;
pub mod icon_telephone_inbound_fill;
pub use self::icon_telephone_inbound_fill::BS_TelephoneInboundFill;
pub mod icon_filetype_raw;
pub use self::icon_filetype_raw::BS_FiletypeRaw;
pub mod icon_chat_square_quote;
pub use self::icon_chat_square_quote::BS_ChatSquareQuote;
pub mod icon_person_gear;
pub use self::icon_person_gear::BS_PersonGear;
pub mod icon_file_word_fill;
pub use self::icon_file_word_fill::BS_FileWordFill;
pub mod icon_calendar3_event;
pub use self::icon_calendar3_event::BS_Calendar3Event;
pub mod icon_bug_fill;
pub use self::icon_bug_fill::BS_BugFill;
pub mod icon_emoji_angry;
pub use self::icon_emoji_angry::BS_EmojiAngry;
pub mod icon_egg_fried;
pub use self::icon_egg_fried::BS_EggFried;
pub mod icon_layers_fill;
pub use self::icon_layers_fill::BS_LayersFill;
pub mod icon_hourglass_split;
pub use self::icon_hourglass_split::BS_HourglassSplit;
pub mod icon_border;
pub use self::icon_border::BS_Border;
pub mod icon_stopwatch;
pub use self::icon_stopwatch::BS_Stopwatch;
pub mod icon_border_bottom;
pub use self::icon_border_bottom::BS_BorderBottom;
pub mod icon_badge_cc_fill;
pub use self::icon_badge_cc_fill::BS_BadgeCcFill;
pub mod icon_rocket;
pub use self::icon_rocket::BS_Rocket;
pub mod icon_calendar2_event;
pub use self::icon_calendar2_event::BS_Calendar2Event;
pub mod icon_person_fill_dash;
pub use self::icon_person_fill_dash::BS_PersonFillDash;
pub mod icon_envelope_paper;
pub use self::icon_envelope_paper::BS_EnvelopePaper;
pub mod icon_chat_quote;
pub use self::icon_chat_quote::BS_ChatQuote;
pub mod icon_inbox_fill;
pub use self::icon_inbox_fill::BS_InboxFill;
pub mod icon_file_earmark_bar_graph_fill;
pub use self::icon_file_earmark_bar_graph_fill::BS_FileEarmarkBarGraphFill;
pub mod icon_hdd_stack_fill;
pub use self::icon_hdd_stack_fill::BS_HddStackFill;
pub mod icon_snapchat;
pub use self::icon_snapchat::BS_Snapchat;
pub mod icon_question_diamond_fill;
pub use self::icon_question_diamond_fill::BS_QuestionDiamondFill;
pub mod icon_person;
pub use self::icon_person::BS_Person;
pub mod icon_box2;
pub use self::icon_box2::BS_Box2;
pub mod icon_calendar2_range_fill;
pub use self::icon_calendar2_range_fill::BS_Calendar2RangeFill;
pub mod icon_code;
pub use self::icon_code::BS_Code;
pub mod icon_person_dash;
pub use self::icon_person_dash::BS_PersonDash;
pub mod icon_cloud_snow;
pub use self::icon_cloud_snow::BS_CloudSnow;
pub mod icon_vector_pen;
pub use self::icon_vector_pen::BS_VectorPen;
pub mod icon_text_paragraph;
pub use self::icon_text_paragraph::BS_TextParagraph;
pub mod icon_border_outer;
pub use self::icon_border_outer::BS_BorderOuter;
pub mod icon_layout_text_window_reverse;
pub use self::icon_layout_text_window_reverse::BS_LayoutTextWindowReverse;
pub mod icon_box_arrow_up;
pub use self::icon_box_arrow_up::BS_BoxArrowUp;
pub mod icon_house_x_fill;
pub use self::icon_house_x_fill::BS_HouseXFill;
pub mod icon_fullscreen_exit;
pub use self::icon_fullscreen_exit::BS_FullscreenExit;
pub mod icon_arrows_angle_expand;
pub use self::icon_arrows_angle_expand::BS_ArrowsAngleExpand;
pub mod icon_image_fill;
pub use self::icon_image_fill::BS_ImageFill;
pub mod icon_eye_fill;
pub use self::icon_eye_fill::BS_EyeFill;
pub mod icon_plus_circle_dotted;
pub use self::icon_plus_circle_dotted::BS_PlusCircleDotted;
pub mod icon_boxes;
pub use self::icon_boxes::BS_Boxes;
pub mod icon_houses;
pub use self::icon_houses::BS_Houses;
pub mod icon_house_exclamation_fill;
pub use self::icon_house_exclamation_fill::BS_HouseExclamationFill;
pub mod icon_save2_fill;
pub use self::icon_save2_fill::BS_Save2Fill;
pub mod icon_easel_fill;
pub use self::icon_easel_fill::BS_EaselFill;
pub mod icon_person_lock;
pub use self::icon_person_lock::BS_PersonLock;
pub mod icon_person_video2;
pub use self::icon_person_video2::BS_PersonVideo2;
pub mod icon_ev_front_fill;
pub use self::icon_ev_front_fill::BS_EvFrontFill;
pub mod icon_list_nested;
pub use self::icon_list_nested::BS_ListNested;
pub mod icon_rewind_fill;
pub use self::icon_rewind_fill::BS_RewindFill;
pub mod icon_droplet_fill;
pub use self::icon_droplet_fill::BS_DropletFill;
pub mod icon_power;
pub use self::icon_power::BS_Power;
pub mod icon_screwdriver;
pub use self::icon_screwdriver::BS_Screwdriver;
pub mod icon_segmented_nav;
pub use self::icon_segmented_nav::BS_SegmentedNav;
pub mod icon_phone_vibrate_fill;
pub use self::icon_phone_vibrate_fill::BS_PhoneVibrateFill;
pub mod icon_check_all;
pub use self::icon_check_all::BS_CheckAll;
pub mod icon_dice_4_fill;
pub use self::icon_dice_4_fill::BS_Dice4Fill;
pub mod icon_menu_app_fill;
pub use self::icon_menu_app_fill::BS_MenuAppFill;
pub mod icon_envelope_plus_fill;
pub use self::icon_envelope_plus_fill::BS_EnvelopePlusFill;
pub mod icon_border_left;
pub use self::icon_border_left::BS_BorderLeft;
pub mod icon_badge_3d_fill;
pub use self::icon_badge_3d_fill::BS_Badge3dFill;
pub mod icon_calendar3_week;
pub use self::icon_calendar3_week::BS_Calendar3Week;
pub mod icon_chat_text;
pub use self::icon_chat_text::BS_ChatText;
pub mod icon_caret_left_square;
pub use self::icon_caret_left_square::BS_CaretLeftSquare;
pub mod icon_slash_circle_fill;
pub use self::icon_slash_circle_fill::BS_SlashCircleFill;
pub mod icon_wrench_adjustable_circle;
pub use self::icon_wrench_adjustable_circle::BS_WrenchAdjustableCircle;
pub mod icon_chat_dots;
pub use self::icon_chat_dots::BS_ChatDots;
pub mod icon_badge_vr;
pub use self::icon_badge_vr::BS_BadgeVr;
pub mod icon_file_earmark_break_fill;
pub use self::icon_file_earmark_break_fill::BS_FileEarmarkBreakFill;
pub mod icon_cloud_hail;
pub use self::icon_cloud_hail::BS_CloudHail;
pub mod icon_folder_x;
pub use self::icon_folder_x::BS_FolderX;
pub mod icon_emoji_wink;
pub use self::icon_emoji_wink::BS_EmojiWink;
pub mod icon_4_square_fill;
pub use self::icon_4_square_fill::BS_4SquareFill;
pub mod icon_arrow_down_left;
pub use self::icon_arrow_down_left::BS_ArrowDownLeft;
pub mod icon_file_earmark_post_fill;
pub use self::icon_file_earmark_post_fill::BS_FileEarmarkPostFill;
pub mod icon_people;
pub use self::icon_people::BS_People;
pub mod icon_h_square;
pub use self::icon_h_square::BS_HSquare;
pub mod icon_chat_right_dots;
pub use self::icon_chat_right_dots::BS_ChatRightDots;
pub mod icon_usb_drive;
pub use self::icon_usb_drive::BS_UsbDrive;
pub mod icon_signal;
pub use self::icon_signal::BS_Signal;
pub mod icon_gender_female;
pub use self::icon_gender_female::BS_GenderFemale;
pub mod icon_person_square;
pub use self::icon_person_square::BS_PersonSquare;
pub mod icon_truck;
pub use self::icon_truck::BS_Truck;
pub mod icon_skip_backward;
pub use self::icon_skip_backward::BS_SkipBackward;
pub mod icon_plus_square_fill;
pub use self::icon_plus_square_fill::BS_PlusSquareFill;
pub mod icon_arrow_down_circle_fill;
pub use self::icon_arrow_down_circle_fill::BS_ArrowDownCircleFill;
pub mod icon_file;
pub use self::icon_file::BS_File;
pub mod icon_app;
pub use self::icon_app::BS_App;
pub mod icon_grid_3x2_gap;
pub use self::icon_grid_3x2_gap::BS_Grid3x2Gap;
pub mod icon_windows;
pub use self::icon_windows::BS_Windows;
pub mod icon_file_zip_fill;
pub use self::icon_file_zip_fill::BS_FileZipFill;
pub mod icon_volume_mute;
pub use self::icon_volume_mute::BS_VolumeMute;
pub mod icon_4_circle;
pub use self::icon_4_circle::BS_4Circle;
pub mod icon_sign_railroad;
pub use self::icon_sign_railroad::BS_SignRailroad;
pub mod icon_filetype_mp4;
pub use self::icon_filetype_mp4::BS_FiletypeMp4;
pub mod icon_heartbreak_fill;
pub use self::icon_heartbreak_fill::BS_HeartbreakFill;
pub mod icon_caret_down;
pub use self::icon_caret_down::BS_CaretDown;
pub mod icon_microsoft_teams;
pub use self::icon_microsoft_teams::BS_MicrosoftTeams;
pub mod icon_shield_plus;
pub use self::icon_shield_plus::BS_ShieldPlus;
pub mod icon_symmetry_horizontal;
pub use self::icon_symmetry_horizontal::BS_SymmetryHorizontal;
pub mod icon_clipboard2_x_fill;
pub use self::icon_clipboard2_x_fill::BS_Clipboard2XFill;
pub mod icon_file_richtext_fill;
pub use self::icon_file_richtext_fill::BS_FileRichtextFill;
pub mod icon_arrow_down_right_circle_fill;
pub use self::icon_arrow_down_right_circle_fill::BS_ArrowDownRightCircleFill;
pub mod icon_map_fill;
pub use self::icon_map_fill::BS_MapFill;
pub mod icon_compass_fill;
pub use self::icon_compass_fill::BS_CompassFill;
pub mod icon_emoji_wink_fill;
pub use self::icon_emoji_wink_fill::BS_EmojiWinkFill;
pub mod icon_file_play;
pub use self::icon_file_play::BS_FilePlay;
pub mod icon_graph_up;
pub use self::icon_graph_up::BS_GraphUp;
pub mod icon_sign_turn_right_fill;
pub use self::icon_sign_turn_right_fill::BS_SignTurnRightFill;
pub mod icon_5_square_fill;
pub use self::icon_5_square_fill::BS_5SquareFill;
pub mod icon_skip_backward_circle;
pub use self::icon_skip_backward_circle::BS_SkipBackwardCircle;
pub mod icon_sort_alpha_up;
pub use self::icon_sort_alpha_up::BS_SortAlphaUp;
pub mod icon_hexagon_fill;
pub use self::icon_hexagon_fill::BS_HexagonFill;
pub mod icon_chat_square_text;
pub use self::icon_chat_square_text::BS_ChatSquareText;
pub mod icon_sign_no_left_turn;
pub use self::icon_sign_no_left_turn::BS_SignNoLeftTurn;
pub mod icon_tv_fill;
pub use self::icon_tv_fill::BS_TvFill;
pub mod icon_menu_button_fill;
pub use self::icon_menu_button_fill::BS_MenuButtonFill;
pub mod icon_journal_minus;
pub use self::icon_journal_minus::BS_JournalMinus;
pub mod icon_cart_plus_fill;
pub use self::icon_cart_plus_fill::BS_CartPlusFill;
pub mod icon_envelope_at;
pub use self::icon_envelope_at::BS_EnvelopeAt;
pub mod icon_journal;
pub use self::icon_journal::BS_Journal;
pub mod icon_percent;
pub use self::icon_percent::BS_Percent;
pub mod icon_rulers;
pub use self::icon_rulers::BS_Rulers;
pub mod icon_toggle2_off;
pub use self::icon_toggle2_off::BS_Toggle2Off;
pub mod icon_buildings_fill;
pub use self::icon_buildings_fill::BS_BuildingsFill;
pub mod icon_mouse3_fill;
pub use self::icon_mouse3_fill::BS_Mouse3Fill;
pub mod icon_filetype_tiff;
pub use self::icon_filetype_tiff::BS_FiletypeTiff;
pub mod icon_house_fill;
pub use self::icon_house_fill::BS_HouseFill;
pub mod icon_boombox_fill;
pub use self::icon_boombox_fill::BS_BoomboxFill;
pub mod icon_building_fill_dash;
pub use self::icon_building_fill_dash::BS_BuildingFillDash;
pub mod icon_clipboard2_data_fill;
pub use self::icon_clipboard2_data_fill::BS_Clipboard2DataFill;
pub mod icon_filter;
pub use self::icon_filter::BS_Filter;
pub mod icon_sliders2;
pub use self::icon_sliders2::BS_Sliders2;
pub mod icon_sign_turn_left;
pub use self::icon_sign_turn_left::BS_SignTurnLeft;
pub mod icon_radioactive;
pub use self::icon_radioactive::BS_Radioactive;
pub mod icon_caret_left_fill;
pub use self::icon_caret_left_fill::BS_CaretLeftFill;
pub mod icon_7_square_fill;
pub use self::icon_7_square_fill::BS_7SquareFill;
pub mod icon_cloud_haze2;
pub use self::icon_cloud_haze2::BS_CloudHaze2;
pub mod icon_arrow_right;
pub use self::icon_arrow_right::BS_ArrowRight;
pub mod icon_facebook;
pub use self::icon_facebook::BS_Facebook;
pub mod icon_magnet;
pub use self::icon_magnet::BS_Magnet;
pub mod icon_file_slides_fill;
pub use self::icon_file_slides_fill::BS_FileSlidesFill;
pub mod icon_file_excel;
pub use self::icon_file_excel::BS_FileExcel;
pub mod icon_arrow_right_square_fill;
pub use self::icon_arrow_right_square_fill::BS_ArrowRightSquareFill;
pub mod icon_code_square;
pub use self::icon_code_square::BS_CodeSquare;
pub mod icon_calendar2_x_fill;
pub use self::icon_calendar2_x_fill::BS_Calendar2XFill;
pub mod icon_house_heart;
pub use self::icon_house_heart::BS_HouseHeart;
pub mod icon_file_earmark_image;
pub use self::icon_file_earmark_image::BS_FileEarmarkImage;
pub mod icon_bucket;
pub use self::icon_bucket::BS_Bucket;
pub mod icon_cup_straw;
pub use self::icon_cup_straw::BS_CupStraw;
pub mod icon_file_earmark_lock2_fill;
pub use self::icon_file_earmark_lock2_fill::BS_FileEarmarkLock2Fill;
pub mod icon_moon_stars_fill;
pub use self::icon_moon_stars_fill::BS_MoonStarsFill;
pub mod icon_file_pdf_fill;
pub use self::icon_file_pdf_fill::BS_FilePdfFill;
pub mod icon_gear;
pub use self::icon_gear::BS_Gear;
pub mod icon_box_arrow_in_right;
pub use self::icon_box_arrow_in_right::BS_BoxArrowInRight;
pub mod icon_filetype_html;
pub use self::icon_filetype_html::BS_FiletypeHtml;
pub mod icon_table;
pub use self::icon_table::BS_Table;
pub mod icon_arrow_down_right_square;
pub use self::icon_arrow_down_right_square::BS_ArrowDownRightSquare;
pub mod icon_dice_3_fill;
pub use self::icon_dice_3_fill::BS_Dice3Fill;
pub mod icon_database_dash;
pub use self::icon_database_dash::BS_DatabaseDash;
pub mod icon_2_circle;
pub use self::icon_2_circle::BS_2Circle;
pub mod icon_clipboard2_fill;
pub use self::icon_clipboard2_fill::BS_Clipboard2Fill;
pub mod icon_rewind;
pub use self::icon_rewind::BS_Rewind;
pub mod icon_credit_card_fill;
pub use self::icon_credit_card_fill::BS_CreditCardFill;
pub mod icon_dot;
pub use self::icon_dot::BS_Dot;
pub mod icon_nut_fill;
pub use self::icon_nut_fill::BS_NutFill;
pub mod icon_filetype_tsx;
pub use self::icon_filetype_tsx::BS_FiletypeTsx;
pub mod icon_bookmark_dash;
pub use self::icon_bookmark_dash::BS_BookmarkDash;
pub mod icon_cloud_fog;
pub use self::icon_cloud_fog::BS_CloudFog;
pub mod icon_rocket_fill;
pub use self::icon_rocket_fill::BS_RocketFill;
pub mod icon_file_earmark_check_fill;
pub use self::icon_file_earmark_check_fill::BS_FileEarmarkCheckFill;
pub mod icon_bluetooth;
pub use self::icon_bluetooth::BS_Bluetooth;
pub mod icon_shuffle;
pub use self::icon_shuffle::BS_Shuffle;
pub mod icon_badge_ar;
pub use self::icon_badge_ar::BS_BadgeAr;
pub mod icon_pip_fill;
pub use self::icon_pip_fill::BS_PipFill;
pub mod icon_bootstrap;
pub use self::icon_bootstrap::BS_Bootstrap;
pub mod icon_film;
pub use self::icon_film::BS_Film;
pub mod icon_battery_charging;
pub use self::icon_battery_charging::BS_BatteryCharging;
pub mod icon_clipboard2_plus;
pub use self::icon_clipboard2_plus::BS_Clipboard2Plus;
pub mod icon_cloud_sleet_fill;
pub use self::icon_cloud_sleet_fill::BS_CloudSleetFill;
pub mod icon_skip_forward_fill;
pub use self::icon_skip_forward_fill::BS_SkipForwardFill;
pub mod icon_chevron_compact_left;
pub use self::icon_chevron_compact_left::BS_ChevronCompactLeft;
pub mod icon_mic_fill;
pub use self::icon_mic_fill::BS_MicFill;
pub mod icon_safe2_fill;
pub use self::icon_safe2_fill::BS_Safe2Fill;
pub mod icon_kanban_fill;
pub use self::icon_kanban_fill::BS_KanbanFill;
pub mod icon_rewind_btn_fill;
pub use self::icon_rewind_btn_fill::BS_RewindBtnFill;
pub mod icon_reply_fill;
pub use self::icon_reply_fill::BS_ReplyFill;
pub mod icon_twitter;
pub use self::icon_twitter::BS_Twitter;
pub mod icon_toggle_on;
pub use self::icon_toggle_on::BS_ToggleOn;
pub mod icon_currency_rupee;
pub use self::icon_currency_rupee::BS_CurrencyRupee;
pub mod icon_badge_sd_fill;
pub use self::icon_badge_sd_fill::BS_BadgeSdFill;
pub mod icon_person_vcard;
pub use self::icon_person_vcard::BS_PersonVcard;
pub mod icon_cpu;
pub use self::icon_cpu::BS_Cpu;
pub mod icon_caret_down_square;
pub use self::icon_caret_down_square::BS_CaretDownSquare;
pub mod icon_node_minus;
pub use self::icon_node_minus::BS_NodeMinus;
pub mod icon_handbag_fill;
pub use self::icon_handbag_fill::BS_HandbagFill;
pub mod icon_door_open_fill;
pub use self::icon_door_open_fill::BS_DoorOpenFill;
pub mod icon_ev_front;
pub use self::icon_ev_front::BS_EvFront;
pub mod icon_arrow_bar_up;
pub use self::icon_arrow_bar_up::BS_ArrowBarUp;
pub mod icon_play_btn;
pub use self::icon_play_btn::BS_PlayBtn;
pub mod icon_send_plus;
pub use self::icon_send_plus::BS_SendPlus;
pub mod icon_chevron_bar_down;
pub use self::icon_chevron_bar_down::BS_ChevronBarDown;
pub mod icon_caret_down_fill;
pub use self::icon_caret_down_fill::BS_CaretDownFill;
pub mod icon_mic_mute;
pub use self::icon_mic_mute::BS_MicMute;
pub mod icon_person_fill_down;
pub use self::icon_person_fill_down::BS_PersonFillDown;
pub mod icon_type_h1;
pub use self::icon_type_h1::BS_TypeH1;
pub mod icon_play_btn_fill;
pub use self::icon_play_btn_fill::BS_PlayBtnFill;
pub mod icon_9_circle;
pub use self::icon_9_circle::BS_9Circle;
pub mod icon_database_fill_down;
pub use self::icon_database_fill_down::BS_DatabaseFillDown;
pub mod icon_unlock;
pub use self::icon_unlock::BS_Unlock;
pub mod icon_brightness_alt_high;
pub use self::icon_brightness_alt_high::BS_BrightnessAltHigh;
pub mod icon_chat_dots_fill;
pub use self::icon_chat_dots_fill::BS_ChatDotsFill;
pub mod icon_eyeglasses;
pub use self::icon_eyeglasses::BS_Eyeglasses;
pub mod icon_arrows_move;
pub use self::icon_arrows_move::BS_ArrowsMove;
pub mod icon_mouse_fill;
pub use self::icon_mouse_fill::BS_MouseFill;
pub mod icon_file_earmark_zip;
pub use self::icon_file_earmark_zip::BS_FileEarmarkZip;
pub mod icon_slash_circle;
pub use self::icon_slash_circle::BS_SlashCircle;
pub mod icon_alt;
pub use self::icon_alt::BS_Alt;
pub mod icon_sign_intersection_y_fill;
pub use self::icon_sign_intersection_y_fill::BS_SignIntersectionYFill;
pub mod icon_brightness_high_fill;
pub use self::icon_brightness_high_fill::BS_BrightnessHighFill;
pub mod icon_lock;
pub use self::icon_lock::BS_Lock;
pub mod icon_cloud_moon;
pub use self::icon_cloud_moon::BS_CloudMoon;
pub mod icon_hand_index_fill;
pub use self::icon_hand_index_fill::BS_HandIndexFill;
pub mod icon_border_style;
pub use self::icon_border_style::BS_BorderStyle;
pub mod icon_cloud_rain_fill;
pub use self::icon_cloud_rain_fill::BS_CloudRainFill;
pub mod icon_telegram;
pub use self::icon_telegram::BS_Telegram;
pub mod icon_tag;
pub use self::icon_tag::BS_Tag;
pub mod icon_trello;
pub use self::icon_trello::BS_Trello;
pub mod icon_calendar_month;
pub use self::icon_calendar_month::BS_CalendarMonth;
pub mod icon_safe2;
pub use self::icon_safe2::BS_Safe2;
pub mod icon_building_dash;
pub use self::icon_building_dash::BS_BuildingDash;
pub mod icon_filetype_js;
pub use self::icon_filetype_js::BS_FiletypeJs;
pub mod icon_box_arrow_right;
pub use self::icon_box_arrow_right::BS_BoxArrowRight;
pub mod icon_messenger;
pub use self::icon_messenger::BS_Messenger;
pub mod icon_arrow_left;
pub use self::icon_arrow_left::BS_ArrowLeft;
pub mod icon_palette2;
pub use self::icon_palette2::BS_Palette2;
pub mod icon_file_earmark_x_fill;
pub use self::icon_file_earmark_x_fill::BS_FileEarmarkXFill;
pub mod icon_fast_forward_btn;
pub use self::icon_fast_forward_btn::BS_FastForwardBtn;
pub mod icon_file_diff;
pub use self::icon_file_diff::BS_FileDiff;
pub mod icon_badge_vr_fill;
pub use self::icon_badge_vr_fill::BS_BadgeVrFill;
pub mod icon_broadcast;
pub use self::icon_broadcast::BS_Broadcast;
pub mod icon_pause_circle;
pub use self::icon_pause_circle::BS_PauseCircle;
pub mod icon_pause_fill;
pub use self::icon_pause_fill::BS_PauseFill;
pub mod icon_motherboard_fill;
pub use self::icon_motherboard_fill::BS_MotherboardFill;
pub mod icon_arrow_right_square;
pub use self::icon_arrow_right_square::BS_ArrowRightSquare;
pub mod icon_database_fill_add;
pub use self::icon_database_fill_add::BS_DatabaseFillAdd;
pub mod icon_filter_square_fill;
pub use self::icon_filter_square_fill::BS_FilterSquareFill;
pub mod icon_virus2;
pub use self::icon_virus2::BS_Virus2;
pub mod icon_chat_text_fill;
pub use self::icon_chat_text_fill::BS_ChatTextFill;
pub mod icon_skip_forward_circle_fill;
pub use self::icon_skip_forward_circle_fill::BS_SkipForwardCircleFill;
pub mod icon_chat_right_quote_fill;
pub use self::icon_chat_right_quote_fill::BS_ChatRightQuoteFill;
pub mod icon_peace_fill;
pub use self::icon_peace_fill::BS_PeaceFill;
pub mod icon_pin_fill;
pub use self::icon_pin_fill::BS_PinFill;
pub mod icon_box2_fill;
pub use self::icon_box2_fill::BS_Box2Fill;
pub mod icon_chat_square_heart_fill;
pub use self::icon_chat_square_heart_fill::BS_ChatSquareHeartFill;
pub mod icon_bar_chart_line;
pub use self::icon_bar_chart_line::BS_BarChartLine;
pub mod icon_calendar_heart;
pub use self::icon_calendar_heart::BS_CalendarHeart;
pub mod icon_phone_fill;
pub use self::icon_phone_fill::BS_PhoneFill;
pub mod icon_send_slash;
pub use self::icon_send_slash::BS_SendSlash;
pub mod icon_tags_fill;
pub use self::icon_tags_fill::BS_TagsFill;
pub mod icon_crop;
pub use self::icon_crop::BS_Crop;
pub mod icon_filetype_ttf;
pub use self::icon_filetype_ttf::BS_FiletypeTtf;
pub mod icon_menu_button_wide;
pub use self::icon_menu_button_wide::BS_MenuButtonWide;
pub mod icon_award;
pub use self::icon_award::BS_Award;
pub mod icon_3_square_fill;
pub use self::icon_3_square_fill::BS_3SquareFill;
pub mod icon_grip_horizontal;
pub use self::icon_grip_horizontal::BS_GripHorizontal;
pub mod icon_9_square_fill;
pub use self::icon_9_square_fill::BS_9SquareFill;
pub mod icon_file_music;
pub use self::icon_file_music::BS_FileMusic;
pub mod icon_cursor_fill;
pub use self::icon_cursor_fill::BS_CursorFill;
pub mod icon_person_video3;
pub use self::icon_person_video3::BS_PersonVideo3;
pub mod icon_hdd_fill;
pub use self::icon_hdd_fill::BS_HddFill;
pub mod icon_suit_club;
pub use self::icon_suit_club::BS_SuitClub;
pub mod icon_suit_club_fill;
pub use self::icon_suit_club_fill::BS_SuitClubFill;
pub mod icon_collection;
pub use self::icon_collection::BS_Collection;
pub mod icon_option;
pub use self::icon_option::BS_Option;
pub mod icon_chevron_double_left;
pub use self::icon_chevron_double_left::BS_ChevronDoubleLeft;
pub mod icon_safe_fill;
pub use self::icon_safe_fill::BS_SafeFill;
pub mod icon_brush_fill;
pub use self::icon_brush_fill::BS_BrushFill;
pub mod icon_6_square;
pub use self::icon_6_square::BS_6Square;
pub mod icon_shop_window;
pub use self::icon_shop_window::BS_ShopWindow;
pub mod icon_shield_exclamation;
pub use self::icon_shield_exclamation::BS_ShieldExclamation;
pub mod icon_record_circle_fill;
pub use self::icon_record_circle_fill::BS_RecordCircleFill;
pub mod icon_file_ppt_fill;
pub use self::icon_file_ppt_fill::BS_FilePptFill;
pub mod icon_dice_1_fill;
pub use self::icon_dice_1_fill::BS_Dice1Fill;
pub mod icon_arrow_bar_left;
pub use self::icon_arrow_bar_left::BS_ArrowBarLeft;
pub mod icon_hdd_network;
pub use self::icon_hdd_network::BS_HddNetwork;
pub mod icon_clock_fill;
pub use self::icon_clock_fill::BS_ClockFill;
pub mod icon_tablet_fill;
pub use self::icon_tablet_fill::BS_TabletFill;
pub mod icon_heart_pulse_fill;
pub use self::icon_heart_pulse_fill::BS_HeartPulseFill;
pub mod icon_filetype_pdf;
pub use self::icon_filetype_pdf::BS_FiletypePdf;
pub mod icon_box_arrow_in_down;
pub use self::icon_box_arrow_in_down::BS_BoxArrowInDown;
pub mod icon_repeat;
pub use self::icon_repeat::BS_Repeat;
pub mod icon_basket_fill;
pub use self::icon_basket_fill::BS_BasketFill;
pub mod icon_globe2;
pub use self::icon_globe2::BS_Globe2;
pub mod icon_sun_fill;
pub use self::icon_sun_fill::BS_SunFill;
pub mod icon_gift;
pub use self::icon_gift::BS_Gift;
pub mod icon_layout_sidebar_inset_reverse;
pub use self::icon_layout_sidebar_inset_reverse::BS_LayoutSidebarInsetReverse;
pub mod icon_plug_fill;
pub use self::icon_plug_fill::BS_PlugFill;
pub mod icon_phone;
pub use self::icon_phone::BS_Phone;
pub mod icon_aspect_ratio;
pub use self::icon_aspect_ratio::BS_AspectRatio;
pub mod icon_truck_front_fill;
pub use self::icon_truck_front_fill::BS_TruckFrontFill;
pub mod icon_behance;
pub use self::icon_behance::BS_Behance;
pub mod icon_universal_access_circle;
pub use self::icon_universal_access_circle::BS_UniversalAccessCircle;
pub mod icon_graph_down_arrow;
pub use self::icon_graph_down_arrow::BS_GraphDownArrow;
pub mod icon_file_earmark_zip_fill;
pub use self::icon_file_earmark_zip_fill::BS_FileEarmarkZipFill;
pub mod icon_bandaid_fill;
pub use self::icon_bandaid_fill::BS_BandaidFill;
pub mod icon_arrow_down_square;
pub use self::icon_arrow_down_square::BS_ArrowDownSquare;
pub mod icon_calendar_check_fill;
pub use self::icon_calendar_check_fill::BS_CalendarCheckFill;
pub mod icon_book_fill;
pub use self::icon_book_fill::BS_BookFill;
pub mod icon_usb_micro_fill;
pub use self::icon_usb_micro_fill::BS_UsbMicroFill;
pub mod icon_controller;
pub use self::icon_controller::BS_Controller;
pub mod icon_fonts;
pub use self::icon_fonts::BS_Fonts;
pub mod icon_envelope_exclamation;
pub use self::icon_envelope_exclamation::BS_EnvelopeExclamation;
pub mod icon_pause;
pub use self::icon_pause::BS_Pause;
pub mod icon_bookmark_dash_fill;
pub use self::icon_bookmark_dash_fill::BS_BookmarkDashFill;
pub mod icon_box_arrow_down_left;
pub use self::icon_box_arrow_down_left::BS_BoxArrowDownLeft;
pub mod icon_front;
pub use self::icon_front::BS_Front;
pub mod icon_sort_numeric_down_alt;
pub use self::icon_sort_numeric_down_alt::BS_SortNumericDownAlt;
pub mod icon_dice_6;
pub use self::icon_dice_6::BS_Dice6;
pub mod icon_rocket_takeoff_fill;
pub use self::icon_rocket_takeoff_fill::BS_RocketTakeoffFill;
pub mod icon_folder2;
pub use self::icon_folder2::BS_Folder2;
pub mod icon_window_desktop;
pub use self::icon_window_desktop::BS_WindowDesktop;
pub mod icon_file_minus;
pub use self::icon_file_minus::BS_FileMinus;
pub mod icon_mailbox;
pub use self::icon_mailbox::BS_Mailbox;
pub mod icon_thermometer_half;
pub use self::icon_thermometer_half::BS_ThermometerHalf;
pub mod icon_person_check_fill;
pub use self::icon_person_check_fill::BS_PersonCheckFill;
pub mod icon_plus_circle_fill;
pub use self::icon_plus_circle_fill::BS_PlusCircleFill;
pub mod icon_flower2;
pub use self::icon_flower2::BS_Flower2;
pub mod icon_arrows_expand;
pub use self::icon_arrows_expand::BS_ArrowsExpand;
pub mod icon_shield_fill_plus;
pub use self::icon_shield_fill_plus::BS_ShieldFillPlus;
pub mod icon_flower3;
pub use self::icon_flower3::BS_Flower3;
pub mod icon_filetype_mov;
pub use self::icon_filetype_mov::BS_FiletypeMov;
pub mod icon_pentagon_fill;
pub use self::icon_pentagon_fill::BS_PentagonFill;
pub mod icon_cloud_download_fill;
pub use self::icon_cloud_download_fill::BS_CloudDownloadFill;
pub mod icon_shift;
pub use self::icon_shift::BS_Shift;
pub mod icon_usb_mini;
pub use self::icon_usb_mini::BS_UsbMini;
pub mod icon_file_earmark_check;
pub use self::icon_file_earmark_check::BS_FileEarmarkCheck;
pub mod icon_4_circle_fill;
pub use self::icon_4_circle_fill::BS_4CircleFill;
pub mod icon_node_minus_fill;
pub use self::icon_node_minus_fill::BS_NodeMinusFill;
pub mod icon_check2_all;
pub use self::icon_check2_all::BS_Check2All;
pub mod icon_sign_stop;
pub use self::icon_sign_stop::BS_SignStop;
pub mod icon_type;
pub use self::icon_type::BS_Type;
pub mod icon_layout_sidebar;
pub use self::icon_layout_sidebar::BS_LayoutSidebar;
pub mod icon_file_arrow_down;
pub use self::icon_file_arrow_down::BS_FileArrowDown;
pub mod icon_bag_dash;
pub use self::icon_bag_dash::BS_BagDash;
pub mod icon_cpu_fill;
pub use self::icon_cpu_fill::BS_CpuFill;
pub mod icon_p_circle;
pub use self::icon_p_circle::BS_PCircle;
pub mod icon_router_fill;
pub use self::icon_router_fill::BS_RouterFill;
pub mod icon_balloon_heart_fill;
pub use self::icon_balloon_heart_fill::BS_BalloonHeartFill;
pub mod icon_file_earmark_easel_fill;
pub use self::icon_file_earmark_easel_fill::BS_FileEarmarkEaselFill;
pub mod icon_clipboard2_data;
pub use self::icon_clipboard2_data::BS_Clipboard2Data;
pub mod icon_person_bounding_box;
pub use self::icon_person_bounding_box::BS_PersonBoundingBox;
pub mod icon_kanban;
pub use self::icon_kanban::BS_Kanban;
pub mod icon_box_arrow_in_down_left;
pub use self::icon_box_arrow_in_down_left::BS_BoxArrowInDownLeft;
pub mod icon_telephone_fill;
pub use self::icon_telephone_fill::BS_TelephoneFill;
pub mod icon_person_exclamation;
pub use self::icon_person_exclamation::BS_PersonExclamation;
pub mod icon_file_check;
pub use self::icon_file_check::BS_FileCheck;
pub mod icon_house_add;
pub use self::icon_house_add::BS_HouseAdd;
pub mod icon_file_easel_fill;
pub use self::icon_file_easel_fill::BS_FileEaselFill;
pub mod icon_shop;
pub use self::icon_shop::BS_Shop;
pub mod icon_arrow_counterclockwise;
pub use self::icon_arrow_counterclockwise::BS_ArrowCounterclockwise;
pub mod icon_bullseye;
pub use self::icon_bullseye::BS_Bullseye;
pub mod icon_globe_asia_australia;
pub use self::icon_globe_asia_australia::BS_GlobeAsiaAustralia;
pub mod icon_plus_slash_minus;
pub use self::icon_plus_slash_minus::BS_PlusSlashMinus;
pub mod icon_pass_fill;
pub use self::icon_pass_fill::BS_PassFill;
pub mod icon_envelope_check;
pub use self::icon_envelope_check::BS_EnvelopeCheck;
pub mod icon_wind;
pub use self::icon_wind::BS_Wind;
pub mod icon_5_circle_fill;
pub use self::icon_5_circle_fill::BS_5CircleFill;
pub mod icon_geo;
pub use self::icon_geo::BS_Geo;
pub mod icon_pci_card;
pub use self::icon_pci_card::BS_PciCard;
pub mod icon_calendar4_event;
pub use self::icon_calendar4_event::BS_Calendar4Event;
pub mod icon_music_player_fill;
pub use self::icon_music_player_fill::BS_MusicPlayerFill;
pub mod icon_filetype_java;
pub use self::icon_filetype_java::BS_FiletypeJava;
pub mod icon_filetype_css;
pub use self::icon_filetype_css::BS_FiletypeCss;
pub mod icon_file_earmark_diff;
pub use self::icon_file_earmark_diff::BS_FileEarmarkDiff;
pub mod icon_building_fill_check;
pub use self::icon_building_fill_check::BS_BuildingFillCheck;
pub mod icon_envelope_open;
pub use self::icon_envelope_open::BS_EnvelopeOpen;
pub mod icon_person_fill_up;
pub use self::icon_person_fill_up::BS_PersonFillUp;
pub mod icon_laptop_fill;
pub use self::icon_laptop_fill::BS_LaptopFill;
pub mod icon_device_ssd_fill;
pub use self::icon_device_ssd_fill::BS_DeviceSsdFill;
pub mod icon_person_video;
pub use self::icon_person_video::BS_PersonVideo;
pub mod icon_skip_start_fill;
pub use self::icon_skip_start_fill::BS_SkipStartFill;
pub mod icon_camera;
pub use self::icon_camera::BS_Camera;
pub mod icon_ubuntu;
pub use self::icon_ubuntu::BS_Ubuntu;
pub mod icon_cloud_upload_fill;
pub use self::icon_cloud_upload_fill::BS_CloudUploadFill;
pub mod icon_database_up;
pub use self::icon_database_up::BS_DatabaseUp;
pub mod icon_fire;
pub use self::icon_fire::BS_Fire;
pub mod icon_border_inner;
pub use self::icon_border_inner::BS_BorderInner;
pub mod icon_badge_wc;
pub use self::icon_badge_wc::BS_BadgeWc;
pub mod icon_exclamation_square_fill;
pub use self::icon_exclamation_square_fill::BS_ExclamationSquareFill;
pub mod icon_brightness_alt_high_fill;
pub use self::icon_brightness_alt_high_fill::BS_BrightnessAltHighFill;
pub mod icon_bandaid;
pub use self::icon_bandaid::BS_Bandaid;
pub mod icon_vr;
pub use self::icon_vr::BS_Vr;
pub mod icon_plus_lg;
pub use self::icon_plus_lg::BS_PlusLg;
pub mod icon_webcam;
pub use self::icon_webcam::BS_Webcam;
pub mod icon_person_fill;
pub use self::icon_person_fill::BS_PersonFill;
pub mod icon_piggy_bank_fill;
pub use self::icon_piggy_bank_fill::BS_PiggyBankFill;
pub mod icon_sign_turn_right;
pub use self::icon_sign_turn_right::BS_SignTurnRight;
pub mod icon_calendar2_check_fill;
pub use self::icon_calendar2_check_fill::BS_Calendar2CheckFill;
pub mod icon_box_arrow_in_up_right;
pub use self::icon_box_arrow_in_up_right::BS_BoxArrowInUpRight;
pub mod icon_bounding_box_circles;
pub use self::icon_bounding_box_circles::BS_BoundingBoxCircles;
pub mod icon_send_x;
pub use self::icon_send_x::BS_SendX;
pub mod icon_chevron_bar_left;
pub use self::icon_chevron_bar_left::BS_ChevronBarLeft;
pub mod icon_bar_chart;
pub use self::icon_bar_chart::BS_BarChart;
pub mod icon_dash_square;
pub use self::icon_dash_square::BS_DashSquare;
pub mod icon_9_square;
pub use self::icon_9_square::BS_9Square;
pub mod icon_phone_vibrate;
pub use self::icon_phone_vibrate::BS_PhoneVibrate;
pub mod icon_outlet;
pub use self::icon_outlet::BS_Outlet;
pub mod icon_lightning_charge;
pub use self::icon_lightning_charge::BS_LightningCharge;
pub mod icon_arrow_up_right;
pub use self::icon_arrow_up_right::BS_ArrowUpRight;
pub mod icon_hand_thumbs_down_fill;
pub use self::icon_hand_thumbs_down_fill::BS_HandThumbsDownFill;
pub mod icon_dice_3;
pub use self::icon_dice_3::BS_Dice3;
pub mod icon_grip_vertical;
pub use self::icon_grip_vertical::BS_GripVertical;
pub mod icon_strava;
pub use self::icon_strava::BS_Strava;
pub mod icon_envelope_open_heart_fill;
pub use self::icon_envelope_open_heart_fill::BS_EnvelopeOpenHeartFill;
pub mod icon_filetype_php;
pub use self::icon_filetype_php::BS_FiletypePhp;
pub mod icon_webcam_fill;
pub use self::icon_webcam_fill::BS_WebcamFill;
pub mod icon_sunrise_fill;
pub use self::icon_sunrise_fill::BS_SunriseFill;
pub mod icon_emoji_neutral_fill;
pub use self::icon_emoji_neutral_fill::BS_EmojiNeutralFill;
pub mod icon_subtract;
pub use self::icon_subtract::BS_Subtract;
pub mod icon_briefcase;
pub use self::icon_briefcase::BS_Briefcase;
pub mod icon_browser_firefox;
pub use self::icon_browser_firefox::BS_BrowserFirefox;
pub mod icon_building;
pub use self::icon_building::BS_Building;
pub mod icon_sign_yield_fill;
pub use self::icon_sign_yield_fill::BS_SignYieldFill;
pub mod icon_caret_up_square_fill;
pub use self::icon_caret_up_square_fill::BS_CaretUpSquareFill;
pub mod icon_folder_plus;
pub use self::icon_folder_plus::BS_FolderPlus;
pub mod icon_dpad;
pub use self::icon_dpad::BS_Dpad;
pub mod icon_card_text;
pub use self::icon_card_text::BS_CardText;
pub mod icon_calendar4_week;
pub use self::icon_calendar4_week::BS_Calendar4Week;
pub mod icon_emoji_dizzy_fill;
pub use self::icon_emoji_dizzy_fill::BS_EmojiDizzyFill;
pub mod icon_pc_display_horizontal;
pub use self::icon_pc_display_horizontal::BS_PcDisplayHorizontal;
pub mod icon_box_arrow_in_up;
pub use self::icon_box_arrow_in_up::BS_BoxArrowInUp;
pub mod icon_menu_down;
pub use self::icon_menu_down::BS_MenuDown;
pub mod icon_badge_8k;
pub use self::icon_badge_8k::BS_Badge8k;
pub mod icon_exclamation_diamond_fill;
pub use self::icon_exclamation_diamond_fill::BS_ExclamationDiamondFill;
pub mod icon_reddit;
pub use self::icon_reddit::BS_Reddit;
pub mod icon_patch_exclamation_fill;
pub use self::icon_patch_exclamation_fill::BS_PatchExclamationFill;
pub mod icon_envelope_heart_fill;
pub use self::icon_envelope_heart_fill::BS_EnvelopeHeartFill;
pub mod icon_earbuds;
pub use self::icon_earbuds::BS_Earbuds;
pub mod icon_camera2;
pub use self::icon_camera2::BS_Camera2;
pub mod icon_grid_3x3;
pub use self::icon_grid_3x3::BS_Grid3x3;
pub mod icon_toggle2_on;
pub use self::icon_toggle2_on::BS_Toggle2On;
pub mod icon_sign_intersection_side;
pub use self::icon_sign_intersection_side::BS_SignIntersectionSide;
pub mod icon_filetype_mp3;
pub use self::icon_filetype_mp3::BS_FiletypeMp3;
pub mod icon_megaphone_fill;
pub use self::icon_megaphone_fill::BS_MegaphoneFill;
pub mod icon_pin_map;
pub use self::icon_pin_map::BS_PinMap;
pub mod icon_house_up_fill;
pub use self::icon_house_up_fill::BS_HouseUpFill;
pub mod icon_card_list;
pub use self::icon_card_list::BS_CardList;
pub mod icon_motherboard;
pub use self::icon_motherboard::BS_Motherboard;
pub mod icon_credit_card;
pub use self::icon_credit_card::BS_CreditCard;
pub mod icon_123;
pub use self::icon_123::BS_123;
pub mod icon_filetype_xls;
pub use self::icon_filetype_xls::BS_FiletypeXls;
pub mod icon_stop_fill;
pub use self::icon_stop_fill::BS_StopFill;
pub mod icon_regex;
pub use self::icon_regex::BS_Regex;
pub mod icon_trash3;
pub use self::icon_trash3::BS_Trash3;
pub mod icon_arrow_clockwise;
pub use self::icon_arrow_clockwise::BS_ArrowClockwise;
pub mod icon_trophy;
pub use self::icon_trophy::BS_Trophy;
pub mod icon_magnet_fill;
pub use self::icon_magnet_fill::BS_MagnetFill;
pub mod icon_cone;
pub use self::icon_cone::BS_Cone;
pub mod icon_view_list;
pub use self::icon_view_list::BS_ViewList;
pub mod icon_type_underline;
pub use self::icon_type_underline::BS_TypeUnderline;
pub mod icon_info_lg;
pub use self::icon_info_lg::BS_InfoLg;
pub mod icon_check_circle_fill;
pub use self::icon_check_circle_fill::BS_CheckCircleFill;
pub mod icon_send_fill;
pub use self::icon_send_fill::BS_SendFill;
pub mod icon_file_earmark_text_fill;
pub use self::icon_file_earmark_text_fill::BS_FileEarmarkTextFill;
pub mod icon_diagram_2_fill;
pub use self::icon_diagram_2_fill::BS_Diagram2Fill;
pub mod icon_alipay;
pub use self::icon_alipay::BS_Alipay;
pub mod icon_arrow_left_circle;
pub use self::icon_arrow_left_circle::BS_ArrowLeftCircle;
pub mod icon_ticket_detailed_fill;
pub use self::icon_ticket_detailed_fill::BS_TicketDetailedFill;
pub mod icon_filetype_docx;
pub use self::icon_filetype_docx::BS_FiletypeDocx;
pub mod icon_phone_landscape_fill;
pub use self::icon_phone_landscape_fill::BS_PhoneLandscapeFill;
pub mod icon_chevron_compact_up;
pub use self::icon_chevron_compact_up::BS_ChevronCompactUp;
pub mod icon_shield;
pub use self::icon_shield::BS_Shield;
pub mod icon_caret_left_square_fill;
pub use self::icon_caret_left_square_fill::BS_CaretLeftSquareFill;
pub mod icon_wechat;
pub use self::icon_wechat::BS_Wechat;
pub mod icon_arrow_through_heart_fill;
pub use self::icon_arrow_through_heart_fill::BS_ArrowThroughHeartFill;
pub mod icon_qr_code_scan;
pub use self::icon_qr_code_scan::BS_QrCodeScan;
pub mod icon_send_check_fill;
pub use self::icon_send_check_fill::BS_SendCheckFill;
pub mod icon_chat_right_fill;
pub use self::icon_chat_right_fill::BS_ChatRightFill;
pub mod icon_link_45deg;
pub use self::icon_link_45deg::BS_Link45deg;
pub mod icon_file_image;
pub use self::icon_file_image::BS_FileImage;
pub mod icon_check_circle;
pub use self::icon_check_circle::BS_CheckCircle;
pub mod icon_filetype_scss;
pub use self::icon_filetype_scss::BS_FiletypeScss;
pub mod icon_house_exclamation;
pub use self::icon_house_exclamation::BS_HouseExclamation;
pub mod icon_emoji_kiss_fill;
pub use self::icon_emoji_kiss_fill::BS_EmojiKissFill;
pub mod icon_stars;
pub use self::icon_stars::BS_Stars;
pub mod icon_shield_fill_minus;
pub use self::icon_shield_fill_minus::BS_ShieldFillMinus;
pub mod icon_camera_reels_fill;
pub use self::icon_camera_reels_fill::BS_CameraReelsFill;
pub mod icon_menu_app;
pub use self::icon_menu_app::BS_MenuApp;
pub mod icon_arrow_up_square_fill;
pub use self::icon_arrow_up_square_fill::BS_ArrowUpSquareFill;
pub mod icon_clipboard2_plus_fill;
pub use self::icon_clipboard2_plus_fill::BS_Clipboard2PlusFill;
pub mod icon_chat_right_quote;
pub use self::icon_chat_right_quote::BS_ChatRightQuote;
pub mod icon_brightness_high;
pub use self::icon_brightness_high::BS_BrightnessHigh;
pub mod icon_signpost;
pub use self::icon_signpost::BS_Signpost;
pub mod icon_filetype_xml;
pub use self::icon_filetype_xml::BS_FiletypeXml;
pub mod icon_exclamation_lg;
pub use self::icon_exclamation_lg::BS_ExclamationLg;
pub mod icon_person_fill_exclamation;
pub use self::icon_person_fill_exclamation::BS_PersonFillExclamation;
pub mod icon_volume_off_fill;
pub use self::icon_volume_off_fill::BS_VolumeOffFill;
pub mod icon_send;
pub use self::icon_send::BS_Send;
pub mod icon_file_binary;
pub use self::icon_file_binary::BS_FileBinary;
pub mod icon_house_down_fill;
pub use self::icon_house_down_fill::BS_HouseDownFill;
pub mod icon_skip_backward_circle_fill;
pub use self::icon_skip_backward_circle_fill::BS_SkipBackwardCircleFill;
pub mod icon_skip_forward_btn_fill;
pub use self::icon_skip_forward_btn_fill::BS_SkipForwardBtnFill;
pub mod icon_house_add_fill;
pub use self::icon_house_add_fill::BS_HouseAddFill;
pub mod icon_cloud_fill;
pub use self::icon_cloud_fill::BS_CloudFill;
pub mod icon_8_circle_fill;
pub use self::icon_8_circle_fill::BS_8CircleFill;
pub mod icon_clipboard2_pulse;
pub use self::icon_clipboard2_pulse::BS_Clipboard2Pulse;
pub mod icon_calendar3_event_fill;
pub use self::icon_calendar3_event_fill::BS_Calendar3EventFill;
pub mod icon_file_text;
pub use self::icon_file_text::BS_FileText;
pub mod icon_tools;
pub use self::icon_tools::BS_Tools;
pub mod icon_cloud_haze;
pub use self::icon_cloud_haze::BS_CloudHaze;
pub mod icon_filetype_xlsx;
pub use self::icon_filetype_xlsx::BS_FiletypeXlsx;
pub mod icon_code_slash;
pub use self::icon_code_slash::BS_CodeSlash;
pub mod icon_grid_1x2_fill;
pub use self::icon_grid_1x2_fill::BS_Grid1x2Fill;
pub mod icon_upload;
pub use self::icon_upload::BS_Upload;
pub mod icon_file_arrow_down_fill;
pub use self::icon_file_arrow_down_fill::BS_FileArrowDownFill;
pub mod icon_p_square;
pub use self::icon_p_square::BS_PSquare;
pub mod icon_zoom_out;
pub use self::icon_zoom_out::BS_ZoomOut;
pub mod icon_arrow_bar_right;
pub use self::icon_arrow_bar_right::BS_ArrowBarRight;
pub mod icon_7_square;
pub use self::icon_7_square::BS_7Square;
pub mod icon_question_octagon_fill;
pub use self::icon_question_octagon_fill::BS_QuestionOctagonFill;
pub mod icon_building_down;
pub use self::icon_building_down::BS_BuildingDown;
pub mod icon_blockquote_left;
pub use self::icon_blockquote_left::BS_BlockquoteLeft;
pub mod icon_window_sidebar;
pub use self::icon_window_sidebar::BS_WindowSidebar;
pub mod icon_sign_do_not_enter;
pub use self::icon_sign_do_not_enter::BS_SignDoNotEnter;
pub mod icon_file_earmark_person;
pub use self::icon_file_earmark_person::BS_FileEarmarkPerson;
pub mod icon_displayport;
pub use self::icon_displayport::BS_Displayport;
pub mod icon_envelope_paper_heart_fill;
pub use self::icon_envelope_paper_heart_fill::BS_EnvelopePaperHeartFill;
pub mod icon_trash2_fill;
pub use self::icon_trash2_fill::BS_Trash2Fill;
pub mod icon_hdd_rack_fill;
pub use self::icon_hdd_rack_fill::BS_HddRackFill;
pub mod icon_exclamation_triangle_fill;
pub use self::icon_exclamation_triangle_fill::BS_ExclamationTriangleFill;
pub mod icon_justify_right;
pub use self::icon_justify_right::BS_JustifyRight;
pub mod icon_airplane;
pub use self::icon_airplane::BS_Airplane;
pub mod icon_dice_4;
pub use self::icon_dice_4::BS_Dice4;
pub mod icon_usb_symbol;
pub use self::icon_usb_symbol::BS_UsbSymbol;
pub mod icon_file_bar_graph;
pub use self::icon_file_bar_graph::BS_FileBarGraph;
pub mod icon_triangle_fill;
pub use self::icon_triangle_fill::BS_TriangleFill;
pub mod icon_file_music_fill;
pub use self::icon_file_music_fill::BS_FileMusicFill;
pub mod icon_instagram;
pub use self::icon_instagram::BS_Instagram;
pub mod icon_device_hdd;
pub use self::icon_device_hdd::BS_DeviceHdd;
pub mod icon_funnel_fill;
pub use self::icon_funnel_fill::BS_FunnelFill;
pub mod icon_filetype_sass;
pub use self::icon_filetype_sass::BS_FiletypeSass;
pub mod icon_escape;
pub use self::icon_escape::BS_Escape;
pub mod icon_currency_dollar;
pub use self::icon_currency_dollar::BS_CurrencyDollar;
pub mod icon_r_square_fill;
pub use self::icon_r_square_fill::BS_RSquareFill;
pub mod icon_cloud_slash_fill;
pub use self::icon_cloud_slash_fill::BS_CloudSlashFill;
pub mod icon_hand_thumbs_up;
pub use self::icon_hand_thumbs_up::BS_HandThumbsUp;
pub mod icon_bricks;
pub use self::icon_bricks::BS_Bricks;
pub mod icon_card_heading;
pub use self::icon_card_heading::BS_CardHeading;
pub mod icon_clipboard_check_fill;
pub use self::icon_clipboard_check_fill::BS_ClipboardCheckFill;
pub mod icon_hdmi;
pub use self::icon_hdmi::BS_Hdmi;
pub mod icon_patch_plus;
pub use self::icon_patch_plus::BS_PatchPlus;
pub mod icon_dash_circle_dotted;
pub use self::icon_dash_circle_dotted::BS_DashCircleDotted;
pub mod icon_wrench_adjustable;
pub use self::icon_wrench_adjustable::BS_WrenchAdjustable;
pub mod icon_mouse2_fill;
pub use self::icon_mouse2_fill::BS_Mouse2Fill;
pub mod icon_wordpress;
pub use self::icon_wordpress::BS_Wordpress;
pub mod icon_paint_bucket;
pub use self::icon_paint_bucket::BS_PaintBucket;
pub mod icon_chat_right;
pub use self::icon_chat_right::BS_ChatRight;
pub mod icon_shift_fill;
pub use self::icon_shift_fill::BS_ShiftFill;
pub mod icon_telephone_forward;
pub use self::icon_telephone_forward::BS_TelephoneForward;
pub mod icon_cassette_fill;
pub use self::icon_cassette_fill::BS_CassetteFill;
pub mod icon_slash;
pub use self::icon_slash::BS_Slash;
pub mod icon_filetype_bmp;
pub use self::icon_filetype_bmp::BS_FiletypeBmp;
pub mod icon_building_gear;
pub use self::icon_building_gear::BS_BuildingGear;
pub mod icon_clouds_fill;
pub use self::icon_clouds_fill::BS_CloudsFill;
pub mod icon_sign_railroad_fill;
pub use self::icon_sign_railroad_fill::BS_SignRailroadFill;
pub mod icon_grid_3x3_gap_fill;
pub use self::icon_grid_3x3_gap_fill::BS_Grid3x3GapFill;
pub mod icon_keyboard_fill;
pub use self::icon_keyboard_fill::BS_KeyboardFill;
pub mod icon_arrow_down_left_circle_fill;
pub use self::icon_arrow_down_left_circle_fill::BS_ArrowDownLeftCircleFill;
pub mod icon_terminal_plus;
pub use self::icon_terminal_plus::BS_TerminalPlus;
pub mod icon_ui_radios;
pub use self::icon_ui_radios::BS_UiRadios;
pub mod icon_arrow_up;
pub use self::icon_arrow_up::BS_ArrowUp;
pub mod icon_arrow_up_square;
pub use self::icon_arrow_up_square::BS_ArrowUpSquare;
pub mod icon_android;
pub use self::icon_android::BS_Android;
pub mod icon_water;
pub use self::icon_water::BS_Water;
pub mod icon_input_cursor_text;
pub use self::icon_input_cursor_text::BS_InputCursorText;
pub mod icon_file_earmark_lock_fill;
pub use self::icon_file_earmark_lock_fill::BS_FileEarmarkLockFill;
pub mod icon_hospital_fill;
pub use self::icon_hospital_fill::BS_HospitalFill;
pub mod icon_sort_up_alt;
pub use self::icon_sort_up_alt::BS_SortUpAlt;
pub mod icon_sim_fill;
pub use self::icon_sim_fill::BS_SimFill;
pub mod icon_cloud_sun;
pub use self::icon_cloud_sun::BS_CloudSun;
pub mod icon_fingerprint;
pub use self::icon_fingerprint::BS_Fingerprint;
pub mod icon_wifi_off;
pub use self::icon_wifi_off::BS_WifiOff;
pub mod icon_arrow_down;
pub use self::icon_arrow_down::BS_ArrowDown;
pub mod icon_file_ruled;
pub use self::icon_file_ruled::BS_FileRuled;
pub mod icon_save2;
pub use self::icon_save2::BS_Save2;
pub mod icon_volume_off;
pub use self::icon_volume_off::BS_VolumeOff;
pub mod icon_clipboard_pulse;
pub use self::icon_clipboard_pulse::BS_ClipboardPulse;
pub mod icon_question_lg;
pub use self::icon_question_lg::BS_QuestionLg;
pub mod icon_file_earmark_music;
pub use self::icon_file_earmark_music::BS_FileEarmarkMusic;
pub mod icon_1_square;
pub use self::icon_1_square::BS_1Square;
pub mod icon_align_bottom;
pub use self::icon_align_bottom::BS_AlignBottom;
pub mod icon_lungs_fill;
pub use self::icon_lungs_fill::BS_LungsFill;
pub mod icon_cassette;
pub use self::icon_cassette::BS_Cassette;
pub mod icon_arrow_up_right_circle_fill;
pub use self::icon_arrow_up_right_circle_fill::BS_ArrowUpRightCircleFill;
pub mod icon_filetype_jsx;
pub use self::icon_filetype_jsx::BS_FiletypeJsx;
pub mod icon_telephone_plus_fill;
pub use self::icon_telephone_plus_fill::BS_TelephonePlusFill;
pub mod icon_moon_stars;
pub use self::icon_moon_stars::BS_MoonStars;
pub mod icon_diamond;
pub use self::icon_diamond::BS_Diamond;
pub mod icon_bank2;
pub use self::icon_bank2::BS_Bank2;
pub mod icon_arrow_down_right;
pub use self::icon_arrow_down_right::BS_ArrowDownRight;
pub mod icon_postcard_heart_fill;
pub use self::icon_postcard_heart_fill::BS_PostcardHeartFill;
pub mod icon_egg_fill;
pub use self::icon_egg_fill::BS_EggFill;
pub mod icon_r_square;
pub use self::icon_r_square::BS_RSquare;
pub mod icon_rss_fill;
pub use self::icon_rss_fill::BS_RssFill;
pub mod icon_shield_lock_fill;
pub use self::icon_shield_lock_fill::BS_ShieldLockFill;
pub mod icon_badge_vo;
pub use self::icon_badge_vo::BS_BadgeVo;
pub mod icon_mastodon;
pub use self::icon_mastodon::BS_Mastodon;
pub mod icon_textarea_resize;
pub use self::icon_textarea_resize::BS_TextareaResize;
pub mod icon_battery;
pub use self::icon_battery::BS_Battery;
pub mod icon_upc_scan;
pub use self::icon_upc_scan::BS_UpcScan;
pub mod icon_credit_card_2_back;
pub use self::icon_credit_card_2_back::BS_CreditCard2Back;
pub mod icon_emoji_kiss;
pub use self::icon_emoji_kiss::BS_EmojiKiss;
pub mod icon_cloudy;
pub use self::icon_cloudy::BS_Cloudy;
pub mod icon_volume_down;
pub use self::icon_volume_down::BS_VolumeDown;
pub mod icon_easel3;
pub use self::icon_easel3::BS_Easel3;
pub mod icon_file_earmark_pdf_fill;
pub use self::icon_file_earmark_pdf_fill::BS_FileEarmarkPdfFill;
pub mod icon_envelope_paper_heart;
pub use self::icon_envelope_paper_heart::BS_EnvelopePaperHeart;
pub mod icon_hand_index;
pub use self::icon_hand_index::BS_HandIndex;
pub mod icon_filetype_mdx;
pub use self::icon_filetype_mdx::BS_FiletypeMdx;
pub mod icon_fuel_pump;
pub use self::icon_fuel_pump::BS_FuelPump;
pub mod icon_lamp_fill;
pub use self::icon_lamp_fill::BS_LampFill;
pub mod icon_lightbulb_fill;
pub use self::icon_lightbulb_fill::BS_LightbulbFill;
pub mod icon_send_x_fill;
pub use self::icon_send_x_fill::BS_SendXFill;
pub mod icon_airplane_fill;
pub use self::icon_airplane_fill::BS_AirplaneFill;
pub mod icon_bell;
pub use self::icon_bell::BS_Bell;
pub mod icon_box_arrow_up_left;
pub use self::icon_box_arrow_up_left::BS_BoxArrowUpLeft;
pub mod icon_sign_merge_left_fill;
pub use self::icon_sign_merge_left_fill::BS_SignMergeLeftFill;
pub mod icon_arrow_down_right_square_fill;
pub use self::icon_arrow_down_right_square_fill::BS_ArrowDownRightSquareFill;
pub mod icon_shield_fill;
pub use self::icon_shield_fill::BS_ShieldFill;
pub mod icon_badge_vo_fill;
pub use self::icon_badge_vo_fill::BS_BadgeVoFill;
pub mod icon_shield_shaded;
pub use self::icon_shield_shaded::BS_ShieldShaded;
pub mod icon_plug;
pub use self::icon_plug::BS_Plug;
pub mod icon_suit_spade;
pub use self::icon_suit_spade::BS_SuitSpade;
pub mod icon_type_strikethrough;
pub use self::icon_type_strikethrough::BS_TypeStrikethrough;
pub mod icon_camera_reels;
pub use self::icon_camera_reels::BS_CameraReels;
pub mod icon_save_fill;
pub use self::icon_save_fill::BS_SaveFill;
pub mod icon_calendar_heart_fill;
pub use self::icon_calendar_heart_fill::BS_CalendarHeartFill;
pub mod icon_pencil_fill;
pub use self::icon_pencil_fill::BS_PencilFill;
pub mod icon_cloud_sleet;
pub use self::icon_cloud_sleet::BS_CloudSleet;
pub mod icon_taxi_front;
pub use self::icon_taxi_front::BS_TaxiFront;
pub mod icon_cash_coin;
pub use self::icon_cash_coin::BS_CashCoin;
pub mod icon_folder_fill;
pub use self::icon_folder_fill::BS_FolderFill;
pub mod icon_collection_play;
pub use self::icon_collection_play::BS_CollectionPlay;
pub mod icon_dropbox;
pub use self::icon_dropbox::BS_Dropbox;
pub mod icon_sign_no_parking;
pub use self::icon_sign_no_parking::BS_SignNoParking;
pub mod icon_badge_tm_fill;
pub use self::icon_badge_tm_fill::BS_BadgeTmFill;
pub mod icon_record2_fill;
pub use self::icon_record2_fill::BS_Record2Fill;
pub mod icon_circle_fill;
pub use self::icon_circle_fill::BS_CircleFill;
pub mod icon_bag_dash_fill;
pub use self::icon_bag_dash_fill::BS_BagDashFill;
pub mod icon_arrow_left_square;
pub use self::icon_arrow_left_square::BS_ArrowLeftSquare;
pub mod icon_calculator;
pub use self::icon_calculator::BS_Calculator;
pub mod icon_file_earmark;
pub use self::icon_file_earmark::BS_FileEarmark;
pub mod icon_calendar_plus_fill;
pub use self::icon_calendar_plus_fill::BS_CalendarPlusFill;
pub mod icon_emoji_heart_eyes_fill;
pub use self::icon_emoji_heart_eyes_fill::BS_EmojiHeartEyesFill;
pub mod icon_1_square_fill;
pub use self::icon_1_square_fill::BS_1SquareFill;
pub mod icon_play_circle;
pub use self::icon_play_circle::BS_PlayCircle;
pub mod icon_capsule;
pub use self::icon_capsule::BS_Capsule;
pub mod icon_database_add;
pub use self::icon_database_add::BS_DatabaseAdd;
pub mod icon_layout_wtf;
pub use self::icon_layout_wtf::BS_LayoutWtf;
pub mod icon_postage_heart_fill;
pub use self::icon_postage_heart_fill::BS_PostageHeartFill;
pub mod icon_dash_square_fill;
pub use self::icon_dash_square_fill::BS_DashSquareFill;
pub mod icon_align_start;
pub use self::icon_align_start::BS_AlignStart;
pub mod icon_emoji_smile_upside_down_fill;
pub use self::icon_emoji_smile_upside_down_fill::BS_EmojiSmileUpsideDownFill;
pub mod icon_chevron_bar_expand;
pub use self::icon_chevron_bar_expand::BS_ChevronBarExpand;
pub mod icon_geo_alt_fill;
pub use self::icon_geo_alt_fill::BS_GeoAltFill;
pub mod icon_house_dash_fill;
pub use self::icon_house_dash_fill::BS_HouseDashFill;
pub mod icon_send_dash_fill;
pub use self::icon_send_dash_fill::BS_SendDashFill;
pub mod icon_exclude;
pub use self::icon_exclude::BS_Exclude;
pub mod icon_emoji_expressionless_fill;
pub use self::icon_emoji_expressionless_fill::BS_EmojiExpressionlessFill;
pub mod icon_calendar2_plus;
pub use self::icon_calendar2_plus::BS_Calendar2Plus;
pub mod icon_newspaper;
pub use self::icon_newspaper::BS_Newspaper;
pub mod icon_calendar_week;
pub use self::icon_calendar_week::BS_CalendarWeek;
pub mod icon_usb_plug;
pub use self::icon_usb_plug::BS_UsbPlug;
pub mod icon_exclamation_octagon;
pub use self::icon_exclamation_octagon::BS_ExclamationOctagon;
pub mod icon_cash_stack;
pub use self::icon_cash_stack::BS_CashStack;
pub mod icon_box_arrow_in_up_left;
pub use self::icon_box_arrow_in_up_left::BS_BoxArrowInUpLeft;
pub mod icon_reply_all_fill;
pub use self::icon_reply_all_fill::BS_ReplyAllFill;
pub mod icon_pause_btn_fill;
pub use self::icon_pause_btn_fill::BS_PauseBtnFill;
pub mod icon_file_earmark_slides;
pub use self::icon_file_earmark_slides::BS_FileEarmarkSlides;
pub mod icon_info_square_fill;
pub use self::icon_info_square_fill::BS_InfoSquareFill;
pub mod icon_calendar2_month_fill;
pub use self::icon_calendar2_month_fill::BS_Calendar2MonthFill;
pub mod icon_house;
pub use self::icon_house::BS_House;
pub mod icon_journal_arrow_up;
pub use self::icon_journal_arrow_up::BS_JournalArrowUp;
pub mod icon_reception_2;
pub use self::icon_reception_2::BS_Reception2;
pub mod icon_filetype_cs;
pub use self::icon_filetype_cs::BS_FiletypeCs;
pub mod icon_building_fill_x;
pub use self::icon_building_fill_x::BS_BuildingFillX;
pub mod icon_octagon;
pub use self::icon_octagon::BS_Octagon;
pub mod icon_cloud_arrow_down;
pub use self::icon_cloud_arrow_down::BS_CloudArrowDown;
pub mod icon_cloud;
pub use self::icon_cloud::BS_Cloud;
pub mod icon_shield_slash_fill;
pub use self::icon_shield_slash_fill::BS_ShieldSlashFill;
pub mod icon_gift_fill;
pub use self::icon_gift_fill::BS_GiftFill;
pub mod icon_disc_fill;
pub use self::icon_disc_fill::BS_DiscFill;
pub mod icon_columns_gap;
pub use self::icon_columns_gap::BS_ColumnsGap;
pub mod icon_sign_merge_right;
pub use self::icon_sign_merge_right::BS_SignMergeRight;
pub mod icon_emoji_frown;
pub use self::icon_emoji_frown::BS_EmojiFrown;
pub mod icon_house_door_fill;
pub use self::icon_house_door_fill::BS_HouseDoorFill;
pub mod icon_bell_slash;
pub use self::icon_bell_slash::BS_BellSlash;
pub mod icon_database_lock;
pub use self::icon_database_lock::BS_DatabaseLock;
pub mod icon_optical_audio_fill;
pub use self::icon_optical_audio_fill::BS_OpticalAudioFill;
pub mod icon_cloud_lightning_rain;
pub use self::icon_cloud_lightning_rain::BS_CloudLightningRain;
pub mod icon_layout_sidebar_reverse;
pub use self::icon_layout_sidebar_reverse::BS_LayoutSidebarReverse;
pub mod icon_window_dock;
pub use self::icon_window_dock::BS_WindowDock;
pub mod icon_file_earmark_medical;
pub use self::icon_file_earmark_medical::BS_FileEarmarkMedical;
pub mod icon_c_square_fill;
pub use self::icon_c_square_fill::BS_CSquareFill;
pub mod icon_exclamation_circle_fill;
pub use self::icon_exclamation_circle_fill::BS_ExclamationCircleFill;
pub mod icon_layout_three_columns;
pub use self::icon_layout_three_columns::BS_LayoutThreeColumns;
pub mod icon_file_earmark_excel;
pub use self::icon_file_earmark_excel::BS_FileEarmarkExcel;
pub mod icon_quora;
pub use self::icon_quora::BS_Quora;
pub mod icon_filetype_rb;
pub use self::icon_filetype_rb::BS_FiletypeRb;
pub mod icon_balloon;
pub use self::icon_balloon::BS_Balloon;
pub mod icon_battery_full;
pub use self::icon_battery_full::BS_BatteryFull;
pub mod icon_paperclip;
pub use self::icon_paperclip::BS_Paperclip;
pub mod icon_clipboard;
pub use self::icon_clipboard::BS_Clipboard;
pub mod icon_arrow_down_up;
pub use self::icon_arrow_down_up::BS_ArrowDownUp;
pub mod icon_chat_left_text;
pub use self::icon_chat_left_text::BS_ChatLeftText;
pub mod icon_capslock_fill;
pub use self::icon_capslock_fill::BS_CapslockFill;
pub mod icon_cart3;
pub use self::icon_cart3::BS_Cart3;
pub mod icon_modem;
pub use self::icon_modem::BS_Modem;
pub mod icon_music_player;
pub use self::icon_music_player::BS_MusicPlayer;
pub mod icon_sign_turn_slight_left_fill;
pub use self::icon_sign_turn_slight_left_fill::BS_SignTurnSlightLeftFill;
pub mod icon_airplane_engines_fill;
pub use self::icon_airplane_engines_fill::BS_AirplaneEnginesFill;
pub mod icon_building_slash;
pub use self::icon_building_slash::BS_BuildingSlash;
pub mod icon_emoji_laughing_fill;
pub use self::icon_emoji_laughing_fill::BS_EmojiLaughingFill;
pub mod icon_projector;
pub use self::icon_projector::BS_Projector;
pub mod icon_flag_fill;
pub use self::icon_flag_fill::BS_FlagFill;
pub mod icon_body_text;
pub use self::icon_body_text::BS_BodyText;
pub mod icon_magic;
pub use self::icon_magic::BS_Magic;
pub mod icon_box;
pub use self::icon_box::BS_Box;
pub mod icon_indent;
pub use self::icon_indent::BS_Indent;
pub mod icon_people_fill;
pub use self::icon_people_fill::BS_PeopleFill;
pub mod icon_file_earmark_slides_fill;
pub use self::icon_file_earmark_slides_fill::BS_FileEarmarkSlidesFill;
pub mod icon_file_bar_graph_fill;
pub use self::icon_file_bar_graph_fill::BS_FileBarGraphFill;
pub mod icon_funnel;
pub use self::icon_funnel::BS_Funnel;
pub mod icon_house_dash;
pub use self::icon_house_dash::BS_HouseDash;
pub mod icon_stoplights;
pub use self::icon_stoplights::BS_Stoplights;
pub mod icon_stop_circle_fill;
pub use self::icon_stop_circle_fill::BS_StopCircleFill;
pub mod icon_key_fill;
pub use self::icon_key_fill::BS_KeyFill;
pub mod icon_train_lightrail_front_fill;
pub use self::icon_train_lightrail_front_fill::BS_TrainLightrailFrontFill;
pub mod icon_steam;
pub use self::icon_steam::BS_Steam;
pub mod icon_bezier2;
pub use self::icon_bezier2::BS_Bezier2;
pub mod icon_camera_video_fill;
pub use self::icon_camera_video_fill::BS_CameraVideoFill;
pub mod icon_gear_fill;
pub use self::icon_gear_fill::BS_GearFill;
pub mod icon_cup_hot_fill;
pub use self::icon_cup_hot_fill::BS_CupHotFill;
pub mod icon_list_columns;
pub use self::icon_list_columns::BS_ListColumns;
pub mod icon_x_diamond_fill;
pub use self::icon_x_diamond_fill::BS_XDiamondFill;
pub mod icon_sort_alpha_down;
pub use self::icon_sort_alpha_down::BS_SortAlphaDown;
pub mod icon_file_lock_fill;
pub use self::icon_file_lock_fill::BS_FileLockFill;
pub mod icon_justify;
pub use self::icon_justify::BS_Justify;
pub mod icon_pip;
pub use self::icon_pip::BS_Pip;
pub mod icon_layers;
pub use self::icon_layers::BS_Layers;
pub mod icon_building_fill_add;
pub use self::icon_building_fill_add::BS_BuildingFillAdd;
pub mod icon_box_arrow_down_right;
pub use self::icon_box_arrow_down_right::BS_BoxArrowDownRight;
pub mod icon_border_center;
pub use self::icon_border_center::BS_BorderCenter;
pub mod icon_diagram_3;
pub use self::icon_diagram_3::BS_Diagram3;
pub mod icon_device_hdd_fill;
pub use self::icon_device_hdd_fill::BS_DeviceHddFill;
pub mod icon_brightness_alt_low;
pub use self::icon_brightness_alt_low::BS_BrightnessAltLow;
pub mod icon_chat;
pub use self::icon_chat::BS_Chat;
pub mod icon_telephone_outbound_fill;
pub use self::icon_telephone_outbound_fill::BS_TelephoneOutboundFill;
pub mod icon_caret_right_square_fill;
pub use self::icon_caret_right_square_fill::BS_CaretRightSquareFill;
pub mod icon_broadcast_pin;
pub use self::icon_broadcast_pin::BS_BroadcastPin;
pub mod icon_x_circle_fill;
pub use self::icon_x_circle_fill::BS_XCircleFill;
pub mod icon_skip_end_btn_fill;
pub use self::icon_skip_end_btn_fill::BS_SkipEndBtnFill;
pub mod icon_lungs;
pub use self::icon_lungs::BS_Lungs;
pub mod icon_calendar2_plus_fill;
pub use self::icon_calendar2_plus_fill::BS_Calendar2PlusFill;
pub mod icon_emoji_sunglasses;
pub use self::icon_emoji_sunglasses::BS_EmojiSunglasses;
pub mod icon_clipboard2_check_fill;
pub use self::icon_clipboard2_check_fill::BS_Clipboard2CheckFill;
pub mod icon_voicemail;
pub use self::icon_voicemail::BS_Voicemail;
pub mod icon_arrow_up_left_square_fill;
pub use self::icon_arrow_up_left_square_fill::BS_ArrowUpLeftSquareFill;
pub mod icon_patch_exclamation;
pub use self::icon_patch_exclamation::BS_PatchExclamation;
pub mod icon_easel2_fill;
pub use self::icon_easel2_fill::BS_Easel2Fill;
pub mod icon_filetype_key;
pub use self::icon_filetype_key::BS_FiletypeKey;
pub mod icon_filetype_svg;
pub use self::icon_filetype_svg::BS_FiletypeSvg;
pub mod icon_search_heart_fill;
pub use self::icon_search_heart_fill::BS_SearchHeartFill;
pub mod icon_file_earmark_arrow_up_fill;
pub use self::icon_file_earmark_arrow_up_fill::BS_FileEarmarkArrowUpFill;
pub mod icon_circle_half;
pub use self::icon_circle_half::BS_CircleHalf;
pub mod icon_align_top;
pub use self::icon_align_top::BS_AlignTop;
pub mod icon_arrows_collapse;
pub use self::icon_arrows_collapse::BS_ArrowsCollapse;
pub mod icon_building_fill_slash;
pub use self::icon_building_fill_slash::BS_BuildingFillSlash;
pub mod icon_slash_lg;
pub use self::icon_slash_lg::BS_SlashLg;
pub mod icon_envelope_check_fill;
pub use self::icon_envelope_check_fill::BS_EnvelopeCheckFill;
pub mod icon_file_earmark_play_fill;
pub use self::icon_file_earmark_play_fill::BS_FileEarmarkPlayFill;
pub mod icon_badge_ad;
pub use self::icon_badge_ad::BS_BadgeAd;
pub mod icon_file_earmark_person_fill;
pub use self::icon_file_earmark_person_fill::BS_FileEarmarkPersonFill;
pub mod icon_house_gear_fill;
pub use self::icon_house_gear_fill::BS_HouseGearFill;
pub mod icon_git;
pub use self::icon_git::BS_Git;
pub mod icon_bell_fill;
pub use self::icon_bell_fill::BS_BellFill;
pub mod icon_activity;
pub use self::icon_activity::BS_Activity;
pub mod icon_chat_left_dots;
pub use self::icon_chat_left_dots::BS_ChatLeftDots;
pub mod icon_shield_lock;
pub use self::icon_shield_lock::BS_ShieldLock;
pub mod icon_capslock;
pub use self::icon_capslock::BS_Capslock;
pub mod icon_arrow_up_right_square;
pub use self::icon_arrow_up_right_square::BS_ArrowUpRightSquare;
pub mod icon_node_plus;
pub use self::icon_node_plus::BS_NodePlus;
pub mod icon_2_square;
pub use self::icon_2_square::BS_2Square;
pub mod icon_caret_right;
pub use self::icon_caret_right::BS_CaretRight;
pub mod icon_building_lock;
pub use self::icon_building_lock::BS_BuildingLock;
pub mod icon_usb_fill;
pub use self::icon_usb_fill::BS_UsbFill;
pub mod icon_postage_heart;
pub use self::icon_postage_heart::BS_PostageHeart;
pub mod icon_coin;
pub use self::icon_coin::BS_Coin;
pub mod icon_chevron_double_up;
pub use self::icon_chevron_double_up::BS_ChevronDoubleUp;
pub mod icon_grid_3x3_gap;
pub use self::icon_grid_3x3_gap::BS_Grid3x3Gap;
pub mod icon_clouds;
pub use self::icon_clouds::BS_Clouds;
pub mod icon_folder2_open;
pub use self::icon_folder2_open::BS_Folder2Open;
pub mod icon_filter_square;
pub use self::icon_filter_square::BS_FilterSquare;
pub mod icon_hexagon;
pub use self::icon_hexagon::BS_Hexagon;
pub mod icon_heptagon_half;
pub use self::icon_heptagon_half::BS_HeptagonHalf;
pub mod icon_dash_circle_fill;
pub use self::icon_dash_circle_fill::BS_DashCircleFill;
pub mod icon_file_easel;
pub use self::icon_file_easel::BS_FileEasel;
pub mod icon_grid_1x2;
pub use self::icon_grid_1x2::BS_Grid1x2;
pub mod icon_hdd;
pub use self::icon_hdd::BS_Hdd;
pub mod icon_question_octagon;
pub use self::icon_question_octagon::BS_QuestionOctagon;
pub mod icon_send_dash;
pub use self::icon_send_dash::BS_SendDash;
pub mod icon_bag_plus_fill;
pub use self::icon_bag_plus_fill::BS_BagPlusFill;
pub mod icon_file_earmark_bar_graph;
pub use self::icon_file_earmark_bar_graph::BS_FileEarmarkBarGraph;
pub mod icon_ui_checks;
pub use self::icon_ui_checks::BS_UiChecks;
pub mod icon_file_spreadsheet;
pub use self::icon_file_spreadsheet::BS_FileSpreadsheet;
pub mod icon_bug;
pub use self::icon_bug::BS_Bug;
pub mod icon_arrow_repeat;
pub use self::icon_arrow_repeat::BS_ArrowRepeat;
pub mod icon_dice_5;
pub use self::icon_dice_5::BS_Dice5;
pub mod icon_3_circle;
pub use self::icon_3_circle::BS_3Circle;
pub mod icon_pause_btn;
pub use self::icon_pause_btn::BS_PauseBtn;
pub mod icon_images;
pub use self::icon_images::BS_Images;
pub mod icon_sign_no_left_turn_fill;
pub use self::icon_sign_no_left_turn_fill::BS_SignNoLeftTurnFill;
pub mod icon_calendar_x;
pub use self::icon_calendar_x::BS_CalendarX;
pub mod icon_filetype_png;
pub use self::icon_filetype_png::BS_FiletypePng;
pub mod icon_clock;
pub use self::icon_clock::BS_Clock;
pub mod icon_airplane_engines;
pub use self::icon_airplane_engines::BS_AirplaneEngines;
pub mod icon_emoji_heart_eyes;
pub use self::icon_emoji_heart_eyes::BS_EmojiHeartEyes;
pub mod icon_book;
pub use self::icon_book::BS_Book;
pub mod icon_sliders2_vertical;
pub use self::icon_sliders2_vertical::BS_Sliders2Vertical;
pub mod icon_credit_card_2_front;
pub use self::icon_credit_card_2_front::BS_CreditCard2Front;
pub mod icon_file_earmark_word;
pub use self::icon_file_earmark_word::BS_FileEarmarkWord;
pub mod icon_filetype_csv;
pub use self::icon_filetype_csv::BS_FiletypeCsv;
pub mod icon_bar_chart_line_fill;
pub use self::icon_bar_chart_line_fill::BS_BarChartLineFill;
pub mod icon_hurricane;
pub use self::icon_hurricane::BS_Hurricane;
pub mod icon_envelope;
pub use self::icon_envelope::BS_Envelope;
pub mod icon_octagon_fill;
pub use self::icon_octagon_fill::BS_OctagonFill;
pub mod icon_envelope_slash;
pub use self::icon_envelope_slash::BS_EnvelopeSlash;
pub mod icon_file_earmark_spreadsheet_fill;
pub use self::icon_file_earmark_spreadsheet_fill::BS_FileEarmarkSpreadsheetFill;
pub mod icon_box_arrow_down;
pub use self::icon_box_arrow_down::BS_BoxArrowDown;
pub mod icon_building_fill_up;
pub use self::icon_building_fill_up::BS_BuildingFillUp;
pub mod icon_cloud_slash;
pub use self::icon_cloud_slash::BS_CloudSlash;
pub mod icon_clipboard_x_fill;
pub use self::icon_clipboard_x_fill::BS_ClipboardXFill;
pub mod icon_arrow_left_right;
pub use self::icon_arrow_left_right::BS_ArrowLeftRight;
pub mod icon_square;
pub use self::icon_square::BS_Square;
pub mod icon_filter_left;
pub use self::icon_filter_left::BS_FilterLeft;
pub mod icon_ladder;
pub use self::icon_ladder::BS_Ladder;
pub mod icon_file_code_fill;
pub use self::icon_file_code_fill::BS_FileCodeFill;
pub mod icon_badge_4k_fill;
pub use self::icon_badge_4k_fill::BS_Badge4kFill;
pub mod icon_umbrella;
pub use self::icon_umbrella::BS_Umbrella;
pub mod icon_arrow_down_left_circle;
pub use self::icon_arrow_down_left_circle::BS_ArrowDownLeftCircle;
pub mod icon_braces_asterisk;
pub use self::icon_braces_asterisk::BS_BracesAsterisk;
pub mod icon_browser_edge;
pub use self::icon_browser_edge::BS_BrowserEdge;
pub mod icon_pen;
pub use self::icon_pen::BS_Pen;
pub mod icon_bootstrap_reboot;
pub use self::icon_bootstrap_reboot::BS_BootstrapReboot;
pub mod icon_smartwatch;
pub use self::icon_smartwatch::BS_Smartwatch;
pub mod icon_peace;
pub use self::icon_peace::BS_Peace;
pub mod icon_person_badge;
pub use self::icon_person_badge::BS_PersonBadge;
pub mod icon_x_octagon_fill;
pub use self::icon_x_octagon_fill::BS_XOctagonFill;
pub mod icon_credit_card_2_back_fill;
pub use self::icon_credit_card_2_back_fill::BS_CreditCard2BackFill;
pub mod icon_border_width;
pub use self::icon_border_width::BS_BorderWidth;
pub mod icon_toggle_off;
pub use self::icon_toggle_off::BS_ToggleOff;
pub mod icon_basket2_fill;
pub use self::icon_basket2_fill::BS_Basket2Fill;
pub mod icon_person_x_fill;
pub use self::icon_person_x_fill::BS_PersonXFill;
pub mod icon_cloud_drizzle_fill;
pub use self::icon_cloud_drizzle_fill::BS_CloudDrizzleFill;
pub mod icon_wallet2;
pub use self::icon_wallet2::BS_Wallet2;
pub mod icon_forward;
pub use self::icon_forward::BS_Forward;
pub mod icon_android2;
pub use self::icon_android2::BS_Android2;
pub mod icon_h_circle;
pub use self::icon_h_circle::BS_HCircle;
pub mod icon_cup_fill;
pub use self::icon_cup_fill::BS_CupFill;
pub mod icon_calendar_range;
pub use self::icon_calendar_range::BS_CalendarRange;
pub mod icon_signpost_split_fill;
pub use self::icon_signpost_split_fill::BS_SignpostSplitFill;
pub mod icon_emoji_neutral;
pub use self::icon_emoji_neutral::BS_EmojiNeutral;
pub mod icon_dash_square_dotted;
pub use self::icon_dash_square_dotted::BS_DashSquareDotted;
pub mod icon_person_circle;
pub use self::icon_person_circle::BS_PersonCircle;
pub mod icon_skip_end;
pub use self::icon_skip_end::BS_SkipEnd;
pub mod icon_eject_fill;
pub use self::icon_eject_fill::BS_EjectFill;
pub mod icon_folder_check;
pub use self::icon_folder_check::BS_FolderCheck;
pub mod icon_file_break_fill;
pub use self::icon_file_break_fill::BS_FileBreakFill;
pub mod icon_sign_turn_slight_right;
pub use self::icon_sign_turn_slight_right::BS_SignTurnSlightRight;
pub mod icon_vinyl;
pub use self::icon_vinyl::BS_Vinyl;
pub mod icon_calendar2_x;
pub use self::icon_calendar2_x::BS_Calendar2X;
pub mod icon_collection_fill;
pub use self::icon_collection_fill::BS_CollectionFill;
pub mod icon_moisture;
pub use self::icon_moisture::BS_Moisture;
pub mod icon_slash_square;
pub use self::icon_slash_square::BS_SlashSquare;
pub mod icon_image;
pub use self::icon_image::BS_Image;
pub mod icon_clipboard2_heart;
pub use self::icon_clipboard2_heart::BS_Clipboard2Heart;
pub mod icon_rainbow;
pub use self::icon_rainbow::BS_Rainbow;
pub mod icon_valentine;
pub use self::icon_valentine::BS_Valentine;
pub mod icon_house_lock;
pub use self::icon_house_lock::BS_HouseLock;
pub mod icon_exclamation_diamond;
pub use self::icon_exclamation_diamond::BS_ExclamationDiamond;
pub mod icon_menu_button;
pub use self::icon_menu_button::BS_MenuButton;
pub mod icon_2_circle_fill;
pub use self::icon_2_circle_fill::BS_2CircleFill;
pub mod icon_envelope_x_fill;
pub use self::icon_envelope_x_fill::BS_EnvelopeXFill;
pub mod icon_backspace;
pub use self::icon_backspace::BS_Backspace;
pub mod icon_badge_ar_fill;
pub use self::icon_badge_ar_fill::BS_BadgeArFill;
pub mod icon_projector_fill;
pub use self::icon_projector_fill::BS_ProjectorFill;
pub mod icon_arrow_up_left_square;
pub use self::icon_arrow_up_left_square::BS_ArrowUpLeftSquare;
pub mod icon_skip_backward_btn;
pub use self::icon_skip_backward_btn::BS_SkipBackwardBtn;
pub mod icon_file_plus_fill;
pub use self::icon_file_plus_fill::BS_FilePlusFill;
pub mod icon_align_middle;
pub use self::icon_align_middle::BS_AlignMiddle;
pub mod icon_pin_angle;
pub use self::icon_pin_angle::BS_PinAngle;
pub mod icon_calendar2_fill;
pub use self::icon_calendar2_fill::BS_Calendar2Fill;
pub mod icon_sunrise;
pub use self::icon_sunrise::BS_Sunrise;
pub mod icon_badge_3d;
pub use self::icon_badge_3d::BS_Badge3d;
pub mod icon_bag_fill;
pub use self::icon_bag_fill::BS_BagFill;
pub mod icon_safe;
pub use self::icon_safe::BS_Safe;
pub mod icon_7_circle;
pub use self::icon_7_circle::BS_7Circle;
pub mod icon_clipboard2_minus_fill;
pub use self::icon_clipboard2_minus_fill::BS_Clipboard2MinusFill;
pub mod icon_file_earmark_code;
pub use self::icon_file_earmark_code::BS_FileEarmarkCode;
pub mod icon_file_ppt;
pub use self::icon_file_ppt::BS_FilePpt;
pub mod icon_filetype_json;
pub use self::icon_filetype_json::BS_FiletypeJson;
pub mod icon_calendar2_week;
pub use self::icon_calendar2_week::BS_Calendar2Week;
pub mod icon_unity;
pub use self::icon_unity::BS_Unity;
pub mod icon_window_fullscreen;
pub use self::icon_window_fullscreen::BS_WindowFullscreen;
pub mod icon_train_freight_front;
pub use self::icon_train_freight_front::BS_TrainFreightFront;
pub mod icon_file_earmark_richtext_fill;
pub use self::icon_file_earmark_richtext_fill::BS_FileEarmarkRichtextFill;
pub mod icon_badge_tm;
pub use self::icon_badge_tm::BS_BadgeTm;
pub mod icon_back;
pub use self::icon_back::BS_Back;
pub mod icon_skip_start;
pub use self::icon_skip_start::BS_SkipStart;
pub mod icon_chat_quote_fill;
pub use self::icon_chat_quote_fill::BS_ChatQuoteFill;
pub mod icon_fast_forward_circle_fill;
pub use self::icon_fast_forward_circle_fill::BS_FastForwardCircleFill;
pub mod icon_bucket_fill;
pub use self::icon_bucket_fill::BS_BucketFill;
pub mod icon_sd_card_fill;
pub use self::icon_sd_card_fill::BS_SdCardFill;
pub mod icon_playstation;
pub use self::icon_playstation::BS_Playstation;
pub mod icon_file_earmark_medical_fill;
pub use self::icon_file_earmark_medical_fill::BS_FileEarmarkMedicalFill;
pub mod icon_lock_fill;
pub use self::icon_lock_fill::BS_LockFill;
pub mod icon_calendar2_heart;
pub use self::icon_calendar2_heart::BS_Calendar2Heart;
pub mod icon_x_square_fill;
pub use self::icon_x_square_fill::BS_XSquareFill;
pub mod icon_pin;
pub use self::icon_pin::BS_Pin;
pub mod icon_folder;
pub use self::icon_folder::BS_Folder;
pub mod icon_arrow_up_short;
pub use self::icon_arrow_up_short::BS_ArrowUpShort;
pub mod icon_calendar2_minus;
pub use self::icon_calendar2_minus::BS_Calendar2Minus;
pub mod icon_file_arrow_up;
pub use self::icon_file_arrow_up::BS_FileArrowUp;
pub mod icon_line;
pub use self::icon_line::BS_Line;
pub mod icon_hdmi_fill;
pub use self::icon_hdmi_fill::BS_HdmiFill;
pub mod icon_tiktok;
pub use self::icon_tiktok::BS_Tiktok;
pub mod icon_square_fill;
pub use self::icon_square_fill::BS_SquareFill;
pub mod icon_heptagon;
pub use self::icon_heptagon::BS_Heptagon;
pub mod icon_question_square_fill;
pub use self::icon_question_square_fill::BS_QuestionSquareFill;
pub mod icon_dpad_fill;
pub use self::icon_dpad_fill::BS_DpadFill;
pub mod icon_box_fill;
pub use self::icon_box_fill::BS_BoxFill;
pub mod icon_usb_mini_fill;
pub use self::icon_usb_mini_fill::BS_UsbMiniFill;
pub mod icon_cart_dash_fill;
pub use self::icon_cart_dash_fill::BS_CartDashFill;
pub mod icon_person_fill_x;
pub use self::icon_person_fill_x::BS_PersonFillX;
pub mod icon_trash;
pub use self::icon_trash::BS_Trash;
pub mod icon_question_diamond;
pub use self::icon_question_diamond::BS_QuestionDiamond;
pub mod icon_egg;
pub use self::icon_egg::BS_Egg;
pub mod icon_headset;
pub use self::icon_headset::BS_Headset;
pub mod icon_currency_euro;
pub use self::icon_currency_euro::BS_CurrencyEuro;
pub mod icon_journal_check;
pub use self::icon_journal_check::BS_JournalCheck;
pub mod icon_calendar2_month;
pub use self::icon_calendar2_month::BS_Calendar2Month;
pub mod icon_box_arrow_up_right;
pub use self::icon_box_arrow_up_right::BS_BoxArrowUpRight;
pub mod icon_sign_turn_left_fill;
pub use self::icon_sign_turn_left_fill::BS_SignTurnLeftFill;
pub mod icon_file_earmark_font;
pub use self::icon_file_earmark_font::BS_FileEarmarkFont;
pub mod icon_shield_slash;
pub use self::icon_shield_slash::BS_ShieldSlash;
pub mod icon_skip_forward_circle;
pub use self::icon_skip_forward_circle::BS_SkipForwardCircle;
pub mod icon_watch;
pub use self::icon_watch::BS_Watch;
pub mod icon_arrow_up_left;
pub use self::icon_arrow_up_left::BS_ArrowUpLeft;
pub mod icon_calendar2;
pub use self::icon_calendar2::BS_Calendar2;
pub mod icon_envelope_fill;
pub use self::icon_envelope_fill::BS_EnvelopeFill;
pub mod icon_skype;
pub use self::icon_skype::BS_Skype;
pub mod icon_r_circle;
pub use self::icon_r_circle::BS_RCircle;
pub mod icon_calendar2_event_fill;
pub use self::icon_calendar2_event_fill::BS_Calendar2EventFill;
pub mod icon_postage_fill;
pub use self::icon_postage_fill::BS_PostageFill;
pub mod icon_usb_c_fill;
pub use self::icon_usb_c_fill::BS_UsbCFill;
pub mod icon_skip_backward_btn_fill;
pub use self::icon_skip_backward_btn_fill::BS_SkipBackwardBtnFill;
pub mod icon_record_circle;
pub use self::icon_record_circle::BS_RecordCircle;
pub mod icon_building_fill_down;
pub use self::icon_building_fill_down::BS_BuildingFillDown;
pub mod icon_gender_male;
pub use self::icon_gender_male::BS_GenderMale;
pub mod icon_hr;
pub use self::icon_hr::BS_Hr;
pub mod icon_cloud_arrow_up_fill;
pub use self::icon_cloud_arrow_up_fill::BS_CloudArrowUpFill;
pub mod icon_meta;
pub use self::icon_meta::BS_Meta;
pub mod icon_pie_chart_fill;
pub use self::icon_pie_chart_fill::BS_PieChartFill;
pub mod icon_r_circle_fill;
pub use self::icon_r_circle_fill::BS_RCircleFill;
pub mod icon_taxi_front_fill;
pub use self::icon_taxi_front_fill::BS_TaxiFrontFill;
pub mod icon_star_fill;
pub use self::icon_star_fill::BS_StarFill;
pub mod icon_calendar4;
pub use self::icon_calendar4::BS_Calendar4;
pub mod icon_sign_intersection;
pub use self::icon_sign_intersection::BS_SignIntersection;
pub mod icon_caret_right_square;
pub use self::icon_caret_right_square::BS_CaretRightSquare;
pub mod icon_patch_question_fill;
pub use self::icon_patch_question_fill::BS_PatchQuestionFill;
pub mod icon_x;
pub use self::icon_x::BS_X;
pub mod icon_record;
pub use self::icon_record::BS_Record;
pub mod icon_cloud_moon_fill;
pub use self::icon_cloud_moon_fill::BS_CloudMoonFill;
pub mod icon_hdd_network_fill;
pub use self::icon_hdd_network_fill::BS_HddNetworkFill;
pub mod icon_train_front_fill;
pub use self::icon_train_front_fill::BS_TrainFrontFill;
pub mod icon_list_check;
pub use self::icon_list_check::BS_ListCheck;
pub mod icon_cloud_rain;
pub use self::icon_cloud_rain::BS_CloudRain;
pub mod icon_building_exclamation;
pub use self::icon_building_exclamation::BS_BuildingExclamation;
pub mod icon_play_fill;
pub use self::icon_play_fill::BS_PlayFill;
pub mod icon_sign_dead_end_fill;
pub use self::icon_sign_dead_end_fill::BS_SignDeadEndFill;
pub mod icon_shield_minus;
pub use self::icon_shield_minus::BS_ShieldMinus;
pub mod icon_hdd_rack;
pub use self::icon_hdd_rack::BS_HddRack;
pub mod icon_cloud_arrow_down_fill;
pub use self::icon_cloud_arrow_down_fill::BS_CloudArrowDownFill;
pub mod icon_database_x;
pub use self::icon_database_x::BS_DatabaseX;
pub mod icon_cloud_plus_fill;
pub use self::icon_cloud_plus_fill::BS_CloudPlusFill;
pub mod icon_sort_numeric_up;
pub use self::icon_sort_numeric_up::BS_SortNumericUp;
pub mod icon_archive;
pub use self::icon_archive::BS_Archive;
pub mod icon_border_all;
pub use self::icon_border_all::BS_BorderAll;
pub mod icon_calendar_date_fill;
pub use self::icon_calendar_date_fill::BS_CalendarDateFill;
pub mod icon_phone_flip;
pub use self::icon_phone_flip::BS_PhoneFlip;
pub mod icon_person_up;
pub use self::icon_person_up::BS_PersonUp;
pub mod icon_cart_x_fill;
pub use self::icon_cart_x_fill::BS_CartXFill;
pub mod icon_palette_fill;
pub use self::icon_palette_fill::BS_PaletteFill;
pub mod icon_snow;
pub use self::icon_snow::BS_Snow;
pub mod icon_suit_diamond;
pub use self::icon_suit_diamond::BS_SuitDiamond;
pub mod icon_calendar_event;
pub use self::icon_calendar_event::BS_CalendarEvent;
pub mod icon_grid;
pub use self::icon_grid::BS_Grid;
pub mod icon_pc_horizontal;
pub use self::icon_pc_horizontal::BS_PcHorizontal;
pub mod icon_x_diamond;
pub use self::icon_x_diamond::BS_XDiamond;
pub mod icon_speedometer2;
pub use self::icon_speedometer2::BS_Speedometer2;
pub mod icon_sign_intersection_fill;
pub use self::icon_sign_intersection_fill::BS_SignIntersectionFill;
pub mod icon_hash;
pub use self::icon_hash::BS_Hash;
pub mod icon_exclamation_square;
pub use self::icon_exclamation_square::BS_ExclamationSquare;
pub mod icon_emoji_smile;
pub use self::icon_emoji_smile::BS_EmojiSmile;
pub mod icon_skip_start_circle_fill;
pub use self::icon_skip_start_circle_fill::BS_SkipStartCircleFill;
pub mod icon_door_closed_fill;
pub use self::icon_door_closed_fill::BS_DoorClosedFill;
pub mod icon_chevron_compact_right;
pub use self::icon_chevron_compact_right::BS_ChevronCompactRight;
pub mod icon_currency_pound;
pub use self::icon_currency_pound::BS_CurrencyPound;
pub mod icon_file_fill;
pub use self::icon_file_fill::BS_FileFill;
pub mod icon_boombox;
pub use self::icon_boombox::BS_Boombox;
pub mod icon_type_italic;
pub use self::icon_type_italic::BS_TypeItalic;
pub mod icon_toggles2;
pub use self::icon_toggles2::BS_Toggles2;
pub mod icon_briefcase_fill;
pub use self::icon_briefcase_fill::BS_BriefcaseFill;
pub mod icon_modem_fill;
pub use self::icon_modem_fill::BS_ModemFill;
pub mod icon_calendar;
pub use self::icon_calendar::BS_Calendar;
pub mod icon_layers_half;
pub use self::icon_layers_half::BS_LayersHalf;
pub mod icon_wallet_fill;
pub use self::icon_wallet_fill::BS_WalletFill;
pub mod icon_send_plus_fill;
pub use self::icon_send_plus_fill::BS_SendPlusFill;
pub mod icon_border_right;
pub use self::icon_border_right::BS_BorderRight;
pub mod icon_person_fill_add;
pub use self::icon_person_fill_add::BS_PersonFillAdd;
pub mod icon_grid_fill;
pub use self::icon_grid_fill::BS_GridFill;
pub mod icon_pencil_square;
pub use self::icon_pencil_square::BS_PencilSquare;
pub mod icon_badge_hd;
pub use self::icon_badge_hd::BS_BadgeHd;
pub mod icon_chat_left;
pub use self::icon_chat_left::BS_ChatLeft;
pub mod icon_8_circle;
pub use self::icon_8_circle::BS_8Circle;
pub mod icon_file_check_fill;
pub use self::icon_file_check_fill::BS_FileCheckFill;
pub mod icon_tags;
pub use self::icon_tags::BS_Tags;
pub mod icon_record_fill;
pub use self::icon_record_fill::BS_RecordFill;
pub mod icon_cart4;
pub use self::icon_cart4::BS_Cart4;
pub mod icon_check_lg;
pub use self::icon_check_lg::BS_CheckLg;
pub mod icon_chat_fill;
pub use self::icon_chat_fill::BS_ChatFill;
pub mod icon_calendar_minus_fill;
pub use self::icon_calendar_minus_fill::BS_CalendarMinusFill;
pub mod icon_chevron_bar_contract;
pub use self::icon_chevron_bar_contract::BS_ChevronBarContract;
pub mod icon_alarm_fill;
pub use self::icon_alarm_fill::BS_AlarmFill;
pub mod icon_cloud_download;
pub use self::icon_cloud_download::BS_CloudDownload;
pub mod icon_usb;
pub use self::icon_usb::BS_Usb;
pub mod icon_diagram_3_fill;
pub use self::icon_diagram_3_fill::BS_Diagram3Fill;
pub mod icon_cc_circle;
pub use self::icon_cc_circle::BS_CcCircle;
pub mod icon_textarea_t;
pub use self::icon_textarea_t::BS_TextareaT;
pub mod icon_calculator_fill;
pub use self::icon_calculator_fill::BS_CalculatorFill;
pub mod icon_bus_front;
pub use self::icon_bus_front::BS_BusFront;
pub mod icon_sun;
pub use self::icon_sun::BS_Sun;
pub mod icon_question_circle;
pub use self::icon_question_circle::BS_QuestionCircle;
pub mod icon_cloud_plus;
pub use self::icon_cloud_plus::BS_CloudPlus;
pub mod icon_search_heart;
pub use self::icon_search_heart::BS_SearchHeart;
pub mod icon_hand_index_thumb_fill;
pub use self::icon_hand_index_thumb_fill::BS_HandIndexThumbFill;
pub mod icon_inboxes;
pub use self::icon_inboxes::BS_Inboxes;
pub mod icon_square_half;
pub use self::icon_square_half::BS_SquareHalf;
pub mod icon_menu_button_wide_fill;
pub use self::icon_menu_button_wide_fill::BS_MenuButtonWideFill;
pub mod icon_door_open;
pub use self::icon_door_open::BS_DoorOpen;
pub mod icon_ticket;
pub use self::icon_ticket::BS_Ticket;
pub mod icon_currency_yen;
pub use self::icon_currency_yen::BS_CurrencyYen;
pub mod icon_stack;
pub use self::icon_stack::BS_Stack;
pub mod icon_download;
pub use self::icon_download::BS_Download;
pub mod icon_clipboard2_x;
pub use self::icon_clipboard2_x::BS_Clipboard2X;
pub mod icon_markdown_fill;
pub use self::icon_markdown_fill::BS_MarkdownFill;
pub mod icon_forward_fill;
pub use self::icon_forward_fill::BS_ForwardFill;
pub mod icon_binoculars;
pub use self::icon_binoculars::BS_Binoculars;
pub mod icon_play;
pub use self::icon_play::BS_Play;
pub mod icon_clipboard2_check;
pub use self::icon_clipboard2_check::BS_Clipboard2Check;
pub mod icon_stop_btn;
pub use self::icon_stop_btn::BS_StopBtn;
pub mod icon_plus_square;
pub use self::icon_plus_square::BS_PlusSquare;
pub mod icon_gpu_card;
pub use self::icon_gpu_card::BS_GpuCard;
pub mod icon_lightning_fill;
pub use self::icon_lightning_fill::BS_LightningFill;
pub mod icon_reply_all;
pub use self::icon_reply_all::BS_ReplyAll;
pub mod icon_ui_checks_grid;
pub use self::icon_ui_checks_grid::BS_UiChecksGrid;
pub mod icon_clipboard_data_fill;
pub use self::icon_clipboard_data_fill::BS_ClipboardDataFill;
pub mod icon_record_btn;
pub use self::icon_record_btn::BS_RecordBtn;
pub mod icon_8_square;
pub use self::icon_8_square::BS_8Square;
pub mod icon_share_fill;
pub use self::icon_share_fill::BS_ShareFill;
pub mod icon_sort_numeric_up_alt;
pub use self::icon_sort_numeric_up_alt::BS_SortNumericUpAlt;
pub mod icon_chat_square_quote_fill;
pub use self::icon_chat_square_quote_fill::BS_ChatSquareQuoteFill;
pub mod icon_bus_front_fill;
pub use self::icon_bus_front_fill::BS_BusFrontFill;
pub mod icon_mouse2;
pub use self::icon_mouse2::BS_Mouse2;
pub mod icon_ticket_perforated;
pub use self::icon_ticket_perforated::BS_TicketPerforated;
pub mod icon_chat_left_heart;
pub use self::icon_chat_left_heart::BS_ChatLeftHeart;
pub mod icon_rewind_circle_fill;
pub use self::icon_rewind_circle_fill::BS_RewindCircleFill;
pub mod icon_calendar3_range;
pub use self::icon_calendar3_range::BS_Calendar3Range;
pub mod icon_app_indicator;
pub use self::icon_app_indicator::BS_AppIndicator;
pub mod icon_alarm;
pub use self::icon_alarm::BS_Alarm;
pub mod icon_calendar_fill;
pub use self::icon_calendar_fill::BS_CalendarFill;
pub mod icon_window_plus;
pub use self::icon_window_plus::BS_WindowPlus;
pub mod icon_chat_square;
pub use self::icon_chat_square::BS_ChatSquare;
pub mod icon_github;
pub use self::icon_github::BS_Github;
pub mod icon_hand_thumbs_down;
pub use self::icon_hand_thumbs_down::BS_HandThumbsDown;
pub mod icon_bookmarks;
pub use self::icon_bookmarks::BS_Bookmarks;
pub mod icon_flag;
pub use self::icon_flag::BS_Flag;
pub mod icon_shield_fill_exclamation;
pub use self::icon_shield_fill_exclamation::BS_ShieldFillExclamation;
pub mod icon_piggy_bank;
pub use self::icon_piggy_bank::BS_PiggyBank;
pub mod icon_record_btn_fill;
pub use self::icon_record_btn_fill::BS_RecordBtnFill;
pub mod icon_device_ssd;
pub use self::icon_device_ssd::BS_DeviceSsd;
pub mod icon_person_fill_gear;
pub use self::icon_person_fill_gear::BS_PersonFillGear;
pub mod icon_sign_stop_lights;
pub use self::icon_sign_stop_lights::BS_SignStopLights;
pub mod icon_0_square;
pub use self::icon_0_square::BS_0Square;
pub mod icon_envelope_dash;
pub use self::icon_envelope_dash::BS_EnvelopeDash;
pub mod icon_list;
pub use self::icon_list::BS_List;
pub mod icon_list_columns_reverse;
pub use self::icon_list_columns_reverse::BS_ListColumnsReverse;
pub mod icon_command;
pub use self::icon_command::BS_Command;
pub mod icon_emoji_frown_fill;
pub use self::icon_emoji_frown_fill::BS_EmojiFrownFill;
pub mod icon_chevron_bar_right;
pub use self::icon_chevron_bar_right::BS_ChevronBarRight;
pub mod icon_house_slash;
pub use self::icon_house_slash::BS_HouseSlash;
pub mod icon_clipboard_fill;
pub use self::icon_clipboard_fill::BS_ClipboardFill;
pub mod icon_input_cursor;
pub use self::icon_input_cursor::BS_InputCursor;
pub mod icon_arrows_fullscreen;
pub use self::icon_arrows_fullscreen::BS_ArrowsFullscreen;
pub mod icon_skip_end_circle;
pub use self::icon_skip_end_circle::BS_SkipEndCircle;
pub mod icon_cast;
pub use self::icon_cast::BS_Cast;
pub mod icon_tencent_qq;
pub use self::icon_tencent_qq::BS_TencentQq;
pub mod icon_calendar2_date;
pub use self::icon_calendar2_date::BS_Calendar2Date;
pub mod icon_cc_square_fill;
pub use self::icon_cc_square_fill::BS_CcSquareFill;
pub mod icon_telephone_inbound;
pub use self::icon_telephone_inbound::BS_TelephoneInbound;
pub mod icon_hexagon_half;
pub use self::icon_hexagon_half::BS_HexagonHalf;
pub mod icon_megaphone;
pub use self::icon_megaphone::BS_Megaphone;
pub mod icon_chat_square_fill;
pub use self::icon_chat_square_fill::BS_ChatSquareFill;
pub mod icon_filetype_yml;
pub use self::icon_filetype_yml::BS_FiletypeYml;
pub mod icon_dice_5_fill;
pub use self::icon_dice_5_fill::BS_Dice5Fill;
pub mod icon_patch_check_fill;
pub use self::icon_patch_check_fill::BS_PatchCheckFill;
pub mod icon_box_seam;
pub use self::icon_box_seam::BS_BoxSeam;
pub mod icon_align_end;
pub use self::icon_align_end::BS_AlignEnd;
pub mod icon_badge_sd;
pub use self::icon_badge_sd::BS_BadgeSd;
pub mod icon_inboxes_fill;
pub use self::icon_inboxes_fill::BS_InboxesFill;
pub mod icon_camera_video_off;
pub use self::icon_camera_video_off::BS_CameraVideoOff;
pub mod icon_clipboard_minus;
pub use self::icon_clipboard_minus::BS_ClipboardMinus;
pub mod icon_cloud_check;
pub use self::icon_cloud_check::BS_CloudCheck;
pub mod icon_fast_forward;
pub use self::icon_fast_forward::BS_FastForward;
pub mod icon_journal_code;
pub use self::icon_journal_code::BS_JournalCode;
pub mod icon_cc_circle_fill;
pub use self::icon_cc_circle_fill::BS_CcCircleFill;
pub mod icon_telephone_x_fill;
pub use self::icon_telephone_x_fill::BS_TelephoneXFill;
pub mod icon_wallet;
pub use self::icon_wallet::BS_Wallet;
pub mod icon_chevron_compact_down;
pub use self::icon_chevron_compact_down::BS_ChevronCompactDown;
pub mod icon_printer;
pub use self::icon_printer::BS_Printer;
pub mod icon_person_workspace;
pub use self::icon_person_workspace::BS_PersonWorkspace;
pub mod icon_calendar2_day_fill;
pub use self::icon_calendar2_day_fill::BS_Calendar2DayFill;
pub mod icon_car_front_fill;
pub use self::icon_car_front_fill::BS_CarFrontFill;
pub mod icon_stickies;
pub use self::icon_stickies::BS_Stickies;
pub mod icon_text_wrap;
pub use self::icon_text_wrap::BS_TextWrap;
pub mod icon_thunderbolt_fill;
pub use self::icon_thunderbolt_fill::BS_ThunderboltFill;
pub mod icon_box_arrow_in_down_right;
pub use self::icon_box_arrow_in_down_right::BS_BoxArrowInDownRight;
pub mod icon_house_x;
pub use self::icon_house_x::BS_HouseX;
pub mod icon_filter_circle_fill;
pub use self::icon_filter_circle_fill::BS_FilterCircleFill;
pub mod icon_justify_left;
pub use self::icon_justify_left::BS_JustifyLeft;
pub mod icon_window_stack;
pub use self::icon_window_stack::BS_WindowStack;
pub mod icon_fast_forward_btn_fill;
pub use self::icon_fast_forward_btn_fill::BS_FastForwardBtnFill;
pub mod icon_file_binary_fill;
pub use self::icon_file_binary_fill::BS_FileBinaryFill;
pub mod icon_sign_no_right_turn;
pub use self::icon_sign_no_right_turn::BS_SignNoRightTurn;
pub mod icon_badge_8k_fill;
pub use self::icon_badge_8k_fill::BS_Badge8kFill;
pub mod icon_postcard;
pub use self::icon_postcard::BS_Postcard;
pub mod icon_5_square;
pub use self::icon_5_square::BS_5Square;
pub mod icon_list_task;
pub use self::icon_list_task::BS_ListTask;
pub mod icon_clipboard_plus;
pub use self::icon_clipboard_plus::BS_ClipboardPlus;
pub mod icon_journal_arrow_down;
pub use self::icon_journal_arrow_down::BS_JournalArrowDown;
pub mod icon_eye_slash;
pub use self::icon_eye_slash::BS_EyeSlash;
pub mod icon_cloud_lightning;
pub use self::icon_cloud_lightning::BS_CloudLightning;
pub mod icon_file_earmark_post;
pub use self::icon_file_earmark_post::BS_FileEarmarkPost;
pub mod icon_bag_check_fill;
pub use self::icon_bag_check_fill::BS_BagCheckFill;
pub mod icon_spotify;
pub use self::icon_spotify::BS_Spotify;
pub mod icon_superscript;
pub use self::icon_superscript::BS_Superscript;
pub mod icon_heptagon_fill;
pub use self::icon_heptagon_fill::BS_HeptagonFill;
pub mod icon_h_square_fill;
pub use self::icon_h_square_fill::BS_HSquareFill;
pub mod icon_align_center;
pub use self::icon_align_center::BS_AlignCenter;
pub mod icon_patch_minus;
pub use self::icon_patch_minus::BS_PatchMinus;
pub mod icon_mask;
pub use self::icon_mask::BS_Mask;
pub mod icon_cloud_upload;
pub use self::icon_cloud_upload::BS_CloudUpload;
pub mod icon_snow3;
pub use self::icon_snow3::BS_Snow3;
pub mod icon_minecart_loaded;
pub use self::icon_minecart_loaded::BS_MinecartLoaded;
pub mod icon_cloud_minus;
pub use self::icon_cloud_minus::BS_CloudMinus;
pub mod icon_thermometer_high;
pub use self::icon_thermometer_high::BS_ThermometerHigh;
pub mod icon_terminal_dash;
pub use self::icon_terminal_dash::BS_TerminalDash;
pub mod icon_three_dots;
pub use self::icon_three_dots::BS_ThreeDots;
pub mod icon_arrow_bar_down;
pub use self::icon_arrow_bar_down::BS_ArrowBarDown;
pub mod icon_thunderbolt;
pub use self::icon_thunderbolt::BS_Thunderbolt;
pub mod icon_thermometer_snow;
pub use self::icon_thermometer_snow::BS_ThermometerSnow;
pub mod icon_microsoft;
pub use self::icon_microsoft::BS_Microsoft;
pub mod icon_telephone_forward_fill;
pub use self::icon_telephone_forward_fill::BS_TelephoneForwardFill;
pub mod icon_caret_up_fill;
pub use self::icon_caret_up_fill::BS_CaretUpFill;
pub mod icon_lightning;
pub use self::icon_lightning::BS_Lightning;
pub mod icon_rewind_circle;
pub use self::icon_rewind_circle::BS_RewindCircle;
pub mod icon_clipboard2;
pub use self::icon_clipboard2::BS_Clipboard2;
pub mod icon_database_fill_up;
pub use self::icon_database_fill_up::BS_DatabaseFillUp;
pub mod icon_cup_hot;
pub use self::icon_cup_hot::BS_CupHot;
pub mod icon_usb_micro;
pub use self::icon_usb_micro::BS_UsbMicro;
pub mod icon_building_x;
pub use self::icon_building_x::BS_BuildingX;
pub mod icon_cloud_lightning_rain_fill;
pub use self::icon_cloud_lightning_rain_fill::BS_CloudLightningRainFill;
pub mod icon_union;
pub use self::icon_union::BS_Union;
pub mod icon_signpost_2;
pub use self::icon_signpost_2::BS_Signpost2;


#[function_component(BSIcon)]
pub fn r#md_icon(props: &IconProps) -> Html {
  let owned_props = props.clone();
  match owned_props.name {
    implicit_clone::unsync::IString::Static("Check2Circle") => html! {
      <BS_Check2Circle ..owned_props />
    },
    implicit_clone::unsync::IString::Static("HouseUp") => html! {
      <BS_HouseUp ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ArrowDownLeftSquareFill") => html! {
      <BS_ArrowDownLeftSquareFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FiletypeExe") => html! {
      <BS_FiletypeExe ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Tornado") => html! {
      <BS_Tornado ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ViewStacked") => html! {
      <BS_ViewStacked ..owned_props />
    },
    implicit_clone::unsync::IString::Static("DropletHalf") => html! {
      <BS_DropletHalf ..owned_props />
    },
    implicit_clone::unsync::IString::Static("TrashFill") => html! {
      <BS_TrashFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("NodePlusFill") => html! {
      <BS_NodePlusFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ArrowUpLeftCircle") => html! {
      <BS_ArrowUpLeftCircle ..owned_props />
    },
    implicit_clone::unsync::IString::Static("PentagonHalf") => html! {
      <BS_PentagonHalf ..owned_props />
    },
    implicit_clone::unsync::IString::Static("DatabaseFill") => html! {
      <BS_DatabaseFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FileEarmarkArrowUp") => html! {
      <BS_FileEarmarkArrowUp ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Buildings") => html! {
      <BS_Buildings ..owned_props />
    },
    implicit_clone::unsync::IString::Static("5Circle") => html! {
      <BS_5Circle ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FiletypeGif") => html! {
      <BS_FiletypeGif ..owned_props />
    },
    implicit_clone::unsync::IString::Static("BorderMiddle") => html! {
      <BS_BorderMiddle ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ExplicitFill") => html! {
      <BS_ExplicitFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Scooter") => html! {
      <BS_Scooter ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Compass") => html! {
      <BS_Compass ..owned_props />
    },
    implicit_clone::unsync::IString::Static("CalendarDate") => html! {
      <BS_CalendarDate ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Youtube") => html! {
      <BS_Youtube ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ChatLeftQuoteFill") => html! {
      <BS_ChatLeftQuoteFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("CloudFog2Fill") => html! {
      <BS_CloudFog2Fill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("7CircleFill") => html! {
      <BS_7CircleFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("PlusCircle") => html! {
      <BS_PlusCircle ..owned_props />
    },
    implicit_clone::unsync::IString::Static("CaretUpSquare") => html! {
      <BS_CaretUpSquare ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Explicit") => html! {
      <BS_Explicit ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Check2Square") => html! {
      <BS_Check2Square ..owned_props />
    },
    implicit_clone::unsync::IString::Static("CSquare") => html! {
      <BS_CSquare ..owned_props />
    },
    implicit_clone::unsync::IString::Static("PatchCheck") => html! {
      <BS_PatchCheck ..owned_props />
    },
    implicit_clone::unsync::IString::Static("CaretUp") => html! {
      <BS_CaretUp ..owned_props />
    },
    implicit_clone::unsync::IString::Static("BarChartSteps") => html! {
      <BS_BarChartSteps ..owned_props />
    },
    implicit_clone::unsync::IString::Static("CardChecklist") => html! {
      <BS_CardChecklist ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Badge4k") => html! {
      <BS_Badge4k ..owned_props />
    },
    implicit_clone::unsync::IString::Static("CCircleFill") => html! {
      <BS_CCircleFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FileEarmarkDiffFill") => html! {
      <BS_FileEarmarkDiffFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FileImageFill") => html! {
      <BS_FileImageFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("HouseSlashFill") => html! {
      <BS_HouseSlashFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("TicketPerforatedFill") => html! {
      <BS_TicketPerforatedFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("PCircleFill") => html! {
      <BS_PCircleFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("StarHalf") => html! {
      <BS_StarHalf ..owned_props />
    },
    implicit_clone::unsync::IString::Static("CloudHailFill") => html! {
      <BS_CloudHailFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("HouseCheck") => html! {
      <BS_HouseCheck ..owned_props />
    },
    implicit_clone::unsync::IString::Static("CalendarWeekFill") => html! {
      <BS_CalendarWeekFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("JournalMedical") => html! {
      <BS_JournalMedical ..owned_props />
    },
    implicit_clone::unsync::IString::Static("CloudFog2") => html! {
      <BS_CloudFog2 ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FiletypeWoff") => html! {
      <BS_FiletypeWoff ..owned_props />
    },
    implicit_clone::unsync::IString::Static("BookmarkPlus") => html! {
      <BS_BookmarkPlus ..owned_props />
    },
    implicit_clone::unsync::IString::Static("VolumeUpFill") => html! {
      <BS_VolumeUpFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FileEarmarkEasel") => html! {
      <BS_FileEarmarkEasel ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Dribbble") => html! {
      <BS_Dribbble ..owned_props />
    },
    implicit_clone::unsync::IString::Static("SunsetFill") => html! {
      <BS_SunsetFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("PersonFillLock") => html! {
      <BS_PersonFillLock ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Keyboard") => html! {
      <BS_Keyboard ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FileEarmarkImageFill") => html! {
      <BS_FileEarmarkImageFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FileEarmarkPdf") => html! {
      <BS_FileEarmarkPdf ..owned_props />
    },
    implicit_clone::unsync::IString::Static("SortAlphaDownAlt") => html! {
      <BS_SortAlphaDownAlt ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ChatSquareTextFill") => html! {
      <BS_ChatSquareTextFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Easel2") => html! {
      <BS_Easel2 ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Inbox") => html! {
      <BS_Inbox ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ArrowUpRightCircle") => html! {
      <BS_ArrowUpRightCircle ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FiletypeTxt") => html! {
      <BS_FiletypeTxt ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FiletypeSql") => html! {
      <BS_FiletypeSql ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Toggles") => html! {
      <BS_Toggles ..owned_props />
    },
    implicit_clone::unsync::IString::Static("SortAlphaUpAlt") => html! {
      <BS_SortAlphaUpAlt ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Trash3Fill") => html! {
      <BS_Trash3Fill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("BalloonHeart") => html! {
      <BS_BalloonHeart ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ClipboardHeartFill") => html! {
      <BS_ClipboardHeartFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FileMedical") => html! {
      <BS_FileMedical ..owned_props />
    },
    implicit_clone::unsync::IString::Static("BuildingFillGear") => html! {
      <BS_BuildingFillGear ..owned_props />
    },
    implicit_clone::unsync::IString::Static("TrainFront") => html! {
      <BS_TrainFront ..owned_props />
    },
    implicit_clone::unsync::IString::Static("HousesFill") => html! {
      <BS_HousesFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Thermometer") => html! {
      <BS_Thermometer ..owned_props />
    },
    implicit_clone::unsync::IString::Static("0Circle") => html! {
      <BS_0Circle ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FileEarmarkBinary") => html! {
      <BS_FileEarmarkBinary ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ChatLeftTextFill") => html! {
      <BS_ChatLeftTextFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Bicycle") => html! {
      <BS_Bicycle ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Translate") => html! {
      <BS_Translate ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FileEarmarkPlus") => html! {
      <BS_FileEarmarkPlus ..owned_props />
    },
    implicit_clone::unsync::IString::Static("BookmarkFill") => html! {
      <BS_BookmarkFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ChevronLeft") => html! {
      <BS_ChevronLeft ..owned_props />
    },
    implicit_clone::unsync::IString::Static("EraserFill") => html! {
      <BS_EraserFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("XOctagon") => html! {
      <BS_XOctagon ..owned_props />
    },
    implicit_clone::unsync::IString::Static("TagFill") => html! {
      <BS_TagFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FileEarmarkPlay") => html! {
      <BS_FileEarmarkPlay ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Trash2") => html! {
      <BS_Trash2 ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Hourglass") => html! {
      <BS_Hourglass ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Question") => html! {
      <BS_Question ..owned_props />
    },
    implicit_clone::unsync::IString::Static("LifePreserver") => html! {
      <BS_LifePreserver ..owned_props />
    },
    implicit_clone::unsync::IString::Static("BookmarkStar") => html! {
      <BS_BookmarkStar ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ConeStriped") => html! {
      <BS_ConeStriped ..owned_props />
    },
    implicit_clone::unsync::IString::Static("QuestionCircleFill") => html! {
      <BS_QuestionCircleFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ExclamationOctagonFill") => html! {
      <BS_ExclamationOctagonFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("DashLg") => html! {
      <BS_DashLg ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FiletypeJpg") => html! {
      <BS_FiletypeJpg ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Dice2") => html! {
      <BS_Dice2 ..owned_props />
    },
    implicit_clone::unsync::IString::Static("SignMergeRightFill") => html! {
      <BS_SignMergeRightFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Telephone") => html! {
      <BS_Telephone ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Hammer") => html! {
      <BS_Hammer ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Twitch") => html! {
      <BS_Twitch ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FileSlides") => html! {
      <BS_FileSlides ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Eraser") => html! {
      <BS_Eraser ..owned_props />
    },
    implicit_clone::unsync::IString::Static("VolumeMuteFill") => html! {
      <BS_VolumeMuteFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("HouseCheckFill") => html! {
      <BS_HouseCheckFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FiletypeOtf") => html! {
      <BS_FiletypeOtf ..owned_props />
    },
    implicit_clone::unsync::IString::Static("CreditCard2FrontFill") => html! {
      <BS_CreditCard2FrontFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Tv") => html! {
      <BS_Tv ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FileEarmarkExcelFill") => html! {
      <BS_FileEarmarkExcelFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("EmojiSmileUpsideDown") => html! {
      <BS_EmojiSmileUpsideDown ..owned_props />
    },
    implicit_clone::unsync::IString::Static("TypeH2") => html! {
      <BS_TypeH2 ..owned_props />
    },
    implicit_clone::unsync::IString::Static("PSquareFill") => html! {
      <BS_PSquareFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("LayerForward") => html! {
      <BS_LayerForward ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Cursor") => html! {
      <BS_Cursor ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FolderMinus") => html! {
      <BS_FolderMinus ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Eyedropper") => html! {
      <BS_Eyedropper ..owned_props />
    },
    implicit_clone::unsync::IString::Static("TicketFill") => html! {
      <BS_TicketFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("StopwatchFill") => html! {
      <BS_StopwatchFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Cup") => html! {
      <BS_Cup ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Nvidia") => html! {
      <BS_Nvidia ..owned_props />
    },
    implicit_clone::unsync::IString::Static("BookmarkHeartFill") => html! {
      <BS_BookmarkHeartFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("TicketDetailed") => html! {
      <BS_TicketDetailed ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Paypal") => html! {
      <BS_Paypal ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Grid3x2") => html! {
      <BS_Grid3x2 ..owned_props />
    },
    implicit_clone::unsync::IString::Static("CCircle") => html! {
      <BS_CCircle ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Mortarboard") => html! {
      <BS_Mortarboard ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ChatSquareHeart") => html! {
      <BS_ChatSquareHeart ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Terminal") => html! {
      <BS_Terminal ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FolderSymlink") => html! {
      <BS_FolderSymlink ..owned_props />
    },
    implicit_clone::unsync::IString::Static("PersonHeart") => html! {
      <BS_PersonHeart ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ArrowRightShort") => html! {
      <BS_ArrowRightShort ..owned_props />
    },
    implicit_clone::unsync::IString::Static("JournalPlus") => html! {
      <BS_JournalPlus ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FileX") => html! {
      <BS_FileX ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Pass") => html! {
      <BS_Pass ..owned_props />
    },
    implicit_clone::unsync::IString::Static("EnvelopePlus") => html! {
      <BS_EnvelopePlus ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Wifi1") => html! {
      <BS_Wifi1 ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FilesAlt") => html! {
      <BS_FilesAlt ..owned_props />
    },
    implicit_clone::unsync::IString::Static("TruckFlatbed") => html! {
      <BS_TruckFlatbed ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ChatLeftQuote") => html! {
      <BS_ChatLeftQuote ..owned_props />
    },
    implicit_clone::unsync::IString::Static("HouseGear") => html! {
      <BS_HouseGear ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FolderSymlinkFill") => html! {
      <BS_FolderSymlinkFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Diagram2") => html! {
      <BS_Diagram2 ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FiletypePsd") => html! {
      <BS_FiletypePsd ..owned_props />
    },
    implicit_clone::unsync::IString::Static("PersonPlus") => html! {
      <BS_PersonPlus ..owned_props />
    },
    implicit_clone::unsync::IString::Static("EmojiSmileFill") => html! {
      <BS_EmojiSmileFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("RewindBtn") => html! {
      <BS_RewindBtn ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ChevronExpand") => html! {
      <BS_ChevronExpand ..owned_props />
    },
    implicit_clone::unsync::IString::Static("WindowDash") => html! {
      <BS_WindowDash ..owned_props />
    },
    implicit_clone::unsync::IString::Static("CloudRainHeavy") => html! {
      <BS_CloudRainHeavy ..owned_props />
    },
    implicit_clone::unsync::IString::Static("SkipEndBtn") => html! {
      <BS_SkipEndBtn ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FileFontFill") => html! {
      <BS_FileFontFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Prescription") => html! {
      <BS_Prescription ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Handbag") => html! {
      <BS_Handbag ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Ear") => html! {
      <BS_Ear ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Xbox") => html! {
      <BS_Xbox ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Plugin") => html! {
      <BS_Plugin ..owned_props />
    },
    implicit_clone::unsync::IString::Static("BrightnessAltLowFill") => html! {
      <BS_BrightnessAltLowFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ChatRightHeartFill") => html! {
      <BS_ChatRightHeartFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("GooglePlay") => html! {
      <BS_GooglePlay ..owned_props />
    },
    implicit_clone::unsync::IString::Static("BalloonFill") => html! {
      <BS_BalloonFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FileEarmarkArrowDownFill") => html! {
      <BS_FileEarmarkArrowDownFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Arrow90degUp") => html! {
      <BS_Arrow90degUp ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Mic") => html! {
      <BS_Mic ..owned_props />
    },
    implicit_clone::unsync::IString::Static("1CircleFill") => html! {
      <BS_1CircleFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("BrightnessLow") => html! {
      <BS_BrightnessLow ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Clipboard2Minus") => html! {
      <BS_Clipboard2Minus ..owned_props />
    },
    implicit_clone::unsync::IString::Static("AspectRatioFill") => html! {
      <BS_AspectRatioFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("SignTurnSlightRightFill") => html! {
      <BS_SignTurnSlightRightFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("WindowX") => html! {
      <BS_WindowX ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ChatLeftDotsFill") => html! {
      <BS_ChatLeftDotsFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("SignDeadEnd") => html! {
      <BS_SignDeadEnd ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ChevronUp") => html! {
      <BS_ChevronUp ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Bookmark") => html! {
      <BS_Bookmark ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FileEarmarkCodeFill") => html! {
      <BS_FileEarmarkCodeFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Subscript") => html! {
      <BS_Subscript ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Fan") => html! {
      <BS_Fan ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Dice1") => html! {
      <BS_Dice1 ..owned_props />
    },
    implicit_clone::unsync::IString::Static("SignStopLightsFill") => html! {
      <BS_SignStopLightsFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ArrowDownLeftSquare") => html! {
      <BS_ArrowDownLeftSquare ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ArrowUpLeftCircleFill") => html! {
      <BS_ArrowUpLeftCircleFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("DatabaseFillExclamation") => html! {
      <BS_DatabaseFillExclamation ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Speedometer") => html! {
      <BS_Speedometer ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ChatHeartFill") => html! {
      <BS_ChatHeartFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("HeartHalf") => html! {
      <BS_HeartHalf ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ShieldFillCheck") => html! {
      <BS_ShieldFillCheck ..owned_props />
    },
    implicit_clone::unsync::IString::Static("CalendarDay") => html! {
      <BS_CalendarDay ..owned_props />
    },
    implicit_clone::unsync::IString::Static("HourglassTop") => html! {
      <BS_HourglassTop ..owned_props />
    },
    implicit_clone::unsync::IString::Static("BackspaceReverseFill") => html! {
      <BS_BackspaceReverseFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Brush") => html! {
      <BS_Brush ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FileEarmarkPpt") => html! {
      <BS_FileEarmarkPpt ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Yelp") => html! {
      <BS_Yelp ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Stop") => html! {
      <BS_Stop ..owned_props />
    },
    implicit_clone::unsync::IString::Static("EmojiExpressionless") => html! {
      <BS_EmojiExpressionless ..owned_props />
    },
    implicit_clone::unsync::IString::Static("SkipForwardBtn") => html! {
      <BS_SkipForwardBtn ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Columns") => html! {
      <BS_Columns ..owned_props />
    },
    implicit_clone::unsync::IString::Static("InfoCircleFill") => html! {
      <BS_InfoCircleFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("6CircleFill") => html! {
      <BS_6CircleFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("QrCode") => html! {
      <BS_QrCode ..owned_props />
    },
    implicit_clone::unsync::IString::Static("DiamondFill") => html! {
      <BS_DiamondFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("CloudRainHeavyFill") => html! {
      <BS_CloudRainHeavyFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("CursorText") => html! {
      <BS_CursorText ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ArchiveFill") => html! {
      <BS_ArchiveFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ChevronBarUp") => html! {
      <BS_ChevronBarUp ..owned_props />
    },
    implicit_clone::unsync::IString::Static("CaretRightFill") => html! {
      <BS_CaretRightFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FastForwardCircle") => html! {
      <BS_FastForwardCircle ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Map") => html! {
      <BS_Map ..owned_props />
    },
    implicit_clone::unsync::IString::Static("CartFill") => html! {
      <BS_CartFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("CartCheckFill") => html! {
      <BS_CartCheckFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FilePlayFill") => html! {
      <BS_FilePlayFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("LayoutSidebarInset") => html! {
      <BS_LayoutSidebarInset ..owned_props />
    },
    implicit_clone::unsync::IString::Static("CcSquare") => html! {
      <BS_CcSquare ..owned_props />
    },
    implicit_clone::unsync::IString::Static("SdCard") => html! {
      <BS_SdCard ..owned_props />
    },
    implicit_clone::unsync::IString::Static("EmojiDizzy") => html! {
      <BS_EmojiDizzy ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Wifi") => html! {
      <BS_Wifi ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ThermometerLow") => html! {
      <BS_ThermometerLow ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FileLock2Fill") => html! {
      <BS_FileLock2Fill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("BagPlus") => html! {
      <BS_BagPlus ..owned_props />
    },
    implicit_clone::unsync::IString::Static("CameraVideo") => html! {
      <BS_CameraVideo ..owned_props />
    },
    implicit_clone::unsync::IString::Static("BoxArrowLeft") => html! {
      <BS_BoxArrowLeft ..owned_props />
    },
    implicit_clone::unsync::IString::Static("BootstrapFill") => html! {
      <BS_BootstrapFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Sunset") => html! {
      <BS_Sunset ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FilterCircle") => html! {
      <BS_FilterCircle ..owned_props />
    },
    implicit_clone::unsync::IString::Static("CapsulePill") => html! {
      <BS_CapsulePill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("SendSlashFill") => html! {
      <BS_SendSlashFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("BuildingFill") => html! {
      <BS_BuildingFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Files") => html! {
      <BS_Files ..owned_props />
    },
    implicit_clone::unsync::IString::Static("BagCheck") => html! {
      <BS_BagCheck ..owned_props />
    },
    implicit_clone::unsync::IString::Static("JournalBookmarkFill") => html! {
      <BS_JournalBookmarkFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("CloudSnowFill") => html! {
      <BS_CloudSnowFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("OctagonHalf") => html! {
      <BS_OctagonHalf ..owned_props />
    },
    implicit_clone::unsync::IString::Static("TerminalSplit") => html! {
      <BS_TerminalSplit ..owned_props />
    },
    implicit_clone::unsync::IString::Static("CalendarMinus") => html! {
      <BS_CalendarMinus ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ArrowLeftSquareFill") => html! {
      <BS_ArrowLeftSquareFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ChevronContract") => html! {
      <BS_ChevronContract ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Router") => html! {
      <BS_Router ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Calendar2Day") => html! {
      <BS_Calendar2Day ..owned_props />
    },
    implicit_clone::unsync::IString::Static("PersonPlusFill") => html! {
      <BS_PersonPlusFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("SignpostFill") => html! {
      <BS_SignpostFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Disc") => html! {
      <BS_Disc ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Grid3x2GapFill") => html! {
      <BS_Grid3x2GapFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("3Square") => html! {
      <BS_3Square ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Basket3Fill") => html! {
      <BS_Basket3Fill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ArrowUpCircle") => html! {
      <BS_ArrowUpCircle ..owned_props />
    },
    implicit_clone::unsync::IString::Static("GlobeEuropeAfrica") => html! {
      <BS_GlobeEuropeAfrica ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FileExcelFill") => html! {
      <BS_FileExcelFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Discord") => html! {
      <BS_Discord ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Prescription2") => html! {
      <BS_Prescription2 ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ArrowReturnLeft") => html! {
      <BS_ArrowReturnLeft ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Cart") => html! {
      <BS_Cart ..owned_props />
    },
    implicit_clone::unsync::IString::Static("CalendarDayFill") => html! {
      <BS_CalendarDayFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("3CircleFill") => html! {
      <BS_3CircleFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("MusicNoteBeamed") => html! {
      <BS_MusicNoteBeamed ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ArrowDownCircle") => html! {
      <BS_ArrowDownCircle ..owned_props />
    },
    implicit_clone::unsync::IString::Static("CloudyFill") => html! {
      <BS_CloudyFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("DisplayFill") => html! {
      <BS_DisplayFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FilePersonFill") => html! {
      <BS_FilePersonFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ClipboardData") => html! {
      <BS_ClipboardData ..owned_props />
    },
    implicit_clone::unsync::IString::Static("RocketTakeoff") => html! {
      <BS_RocketTakeoff ..owned_props />
    },
    implicit_clone::unsync::IString::Static("PersonFillCheck") => html! {
      <BS_PersonFillCheck ..owned_props />
    },
    implicit_clone::unsync::IString::Static("HeadsetVr") => html! {
      <BS_HeadsetVr ..owned_props />
    },
    implicit_clone::unsync::IString::Static("BookmarksFill") => html! {
      <BS_BookmarksFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("DatabaseSlash") => html! {
      <BS_DatabaseSlash ..owned_props />
    },
    implicit_clone::unsync::IString::Static("DistributeVertical") => html! {
      <BS_DistributeVertical ..owned_props />
    },
    implicit_clone::unsync::IString::Static("BarChartFill") => html! {
      <BS_BarChartFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("SuitHeart") => html! {
      <BS_SuitHeart ..owned_props />
    },
    implicit_clone::unsync::IString::Static("HCircleFill") => html! {
      <BS_HCircleFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Server") => html! {
      <BS_Server ..owned_props />
    },
    implicit_clone::unsync::IString::Static("DistributeHorizontal") => html! {
      <BS_DistributeHorizontal ..owned_props />
    },
    implicit_clone::unsync::IString::Static("SignIntersectionY") => html! {
      <BS_SignIntersectionY ..owned_props />
    },
    implicit_clone::unsync::IString::Static("BuildingCheck") => html! {
      <BS_BuildingCheck ..owned_props />
    },
    implicit_clone::unsync::IString::Static("BackspaceReverse") => html! {
      <BS_BackspaceReverse ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ListUl") => html! {
      <BS_ListUl ..owned_props />
    },
    implicit_clone::unsync::IString::Static("SortDown") => html! {
      <BS_SortDown ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FileMedicalFill") => html! {
      <BS_FileMedicalFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FileEarmarkMinusFill") => html! {
      <BS_FileEarmarkMinusFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ShieldFillX") => html! {
      <BS_ShieldFillX ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ChatRightHeart") => html! {
      <BS_ChatRightHeart ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ArrowDownSquareFill") => html! {
      <BS_ArrowDownSquareFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Paragraph") => html! {
      <BS_Paragraph ..owned_props />
    },
    implicit_clone::unsync::IString::Static("TerminalX") => html! {
      <BS_TerminalX ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ChatLeftFill") => html! {
      <BS_ChatLeftFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Soundwave") => html! {
      <BS_Soundwave ..owned_props />
    },
    implicit_clone::unsync::IString::Static("EnvelopeAtFill") => html! {
      <BS_EnvelopeAtFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Arrow90degDown") => html! {
      <BS_Arrow90degDown ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FileRuledFill") => html! {
      <BS_FileRuledFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ExclamationTriangle") => html! {
      <BS_ExclamationTriangle ..owned_props />
    },
    implicit_clone::unsync::IString::Static("UniversalAccess") => html! {
      <BS_UniversalAccess ..owned_props />
    },
    implicit_clone::unsync::IString::Static("SendCheck") => html! {
      <BS_SendCheck ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Hypnotize") => html! {
      <BS_Hypnotize ..owned_props />
    },
    implicit_clone::unsync::IString::Static("PatchPlusFill") => html! {
      <BS_PatchPlusFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("CartDash") => html! {
      <BS_CartDash ..owned_props />
    },
    implicit_clone::unsync::IString::Static("StackOverflow") => html! {
      <BS_StackOverflow ..owned_props />
    },
    implicit_clone::unsync::IString::Static("BellSlashFill") => html! {
      <BS_BellSlashFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FileEarmarkArrowDown") => html! {
      <BS_FileEarmarkArrowDown ..owned_props />
    },
    implicit_clone::unsync::IString::Static("LayoutTextSidebar") => html! {
      <BS_LayoutTextSidebar ..owned_props />
    },
    implicit_clone::unsync::IString::Static("HandIndexThumb") => html! {
      <BS_HandIndexThumb ..owned_props />
    },
    implicit_clone::unsync::IString::Static("StoplightsFill") => html! {
      <BS_StoplightsFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Minecart") => html! {
      <BS_Minecart ..owned_props />
    },
    implicit_clone::unsync::IString::Static("BorderTop") => html! {
      <BS_BorderTop ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Fullscreen") => html! {
      <BS_Fullscreen ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FileLock") => html! {
      <BS_FileLock ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ShieldCheck") => html! {
      <BS_ShieldCheck ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ChatLeftHeartFill") => html! {
      <BS_ChatLeftHeartFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("PersonDown") => html! {
      <BS_PersonDown ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Dash") => html! {
      <BS_Dash ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Reception4") => html! {
      <BS_Reception4 ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Speaker") => html! {
      <BS_Speaker ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ClipboardX") => html! {
      <BS_ClipboardX ..owned_props />
    },
    implicit_clone::unsync::IString::Static("BookmarkCheckFill") => html! {
      <BS_BookmarkCheckFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("BinocularsFill") => html! {
      <BS_BinocularsFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("TrainLightrailFront") => html! {
      <BS_TrainLightrailFront ..owned_props />
    },
    implicit_clone::unsync::IString::Static("TreeFill") => html! {
      <BS_TreeFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ArrowThroughHeart") => html! {
      <BS_ArrowThroughHeart ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FilePerson") => html! {
      <BS_FilePerson ..owned_props />
    },
    implicit_clone::unsync::IString::Static("HouseLockFill") => html! {
      <BS_HouseLockFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FileEarmarkPptFill") => html! {
      <BS_FileEarmarkPptFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Tree") => html! {
      <BS_Tree ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Pinterest") => html! {
      <BS_Pinterest ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Gem") => html! {
      <BS_Gem ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Clipboard2PulseFill") => html! {
      <BS_Clipboard2PulseFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Dice2Fill") => html! {
      <BS_Dice2Fill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("CartCheck") => html! {
      <BS_CartCheck ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FuelPumpDieselFill") => html! {
      <BS_FuelPumpDieselFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ThreeDotsVertical") => html! {
      <BS_ThreeDotsVertical ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FuelPumpDiesel") => html! {
      <BS_FuelPumpDiesel ..owned_props />
    },
    implicit_clone::unsync::IString::Static("XSquare") => html! {
      <BS_XSquare ..owned_props />
    },
    implicit_clone::unsync::IString::Static("PersonRolodex") => html! {
      <BS_PersonRolodex ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Calendar2MinusFill") => html! {
      <BS_Calendar2MinusFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("GlobeAmericas") => html! {
      <BS_GlobeAmericas ..owned_props />
    },
    implicit_clone::unsync::IString::Static("DoorClosed") => html! {
      <BS_DoorClosed ..owned_props />
    },
    implicit_clone::unsync::IString::Static("VinylFill") => html! {
      <BS_VinylFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ListStars") => html! {
      <BS_ListStars ..owned_props />
    },
    implicit_clone::unsync::IString::Static("PlayCircleFill") => html! {
      <BS_PlayCircleFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("SendExclamation") => html! {
      <BS_SendExclamation ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Bezier") => html! {
      <BS_Bezier ..owned_props />
    },
    implicit_clone::unsync::IString::Static("PauseCircleFill") => html! {
      <BS_PauseCircleFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("LayoutTextWindow") => html! {
      <BS_LayoutTextWindow ..owned_props />
    },
    implicit_clone::unsync::IString::Static("PinAngleFill") => html! {
      <BS_PinAngleFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("SpeakerFill") => html! {
      <BS_SpeakerFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Slack") => html! {
      <BS_Slack ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Basket") => html! {
      <BS_Basket ..owned_props />
    },
    implicit_clone::unsync::IString::Static("BookmarkX") => html! {
      <BS_BookmarkX ..owned_props />
    },
    implicit_clone::unsync::IString::Static("BatteryHalf") => html! {
      <BS_BatteryHalf ..owned_props />
    },
    implicit_clone::unsync::IString::Static("CalendarEventFill") => html! {
      <BS_CalendarEventFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("EnvelopeSlashFill") => html! {
      <BS_EnvelopeSlashFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("PuzzleFill") => html! {
      <BS_PuzzleFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("CalendarCheck") => html! {
      <BS_CalendarCheck ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ImageAlt") => html! {
      <BS_ImageAlt ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FiletypePy") => html! {
      <BS_FiletypePy ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FilePostFill") => html! {
      <BS_FilePostFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Braces") => html! {
      <BS_Braces ..owned_props />
    },
    implicit_clone::unsync::IString::Static("SkipEndCircleFill") => html! {
      <BS_SkipEndCircleFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("LightbulbOffFill") => html! {
      <BS_LightbulbOffFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("BrowserChrome") => html! {
      <BS_BrowserChrome ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Plus") => html! {
      <BS_Plus ..owned_props />
    },
    implicit_clone::unsync::IString::Static("GenderAmbiguous") => html! {
      <BS_GenderAmbiguous ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Sliders") => html! {
      <BS_Sliders ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Whatsapp") => html! {
      <BS_Whatsapp ..owned_props />
    },
    implicit_clone::unsync::IString::Static("BrightnessLowFill") => html! {
      <BS_BrightnessLowFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("SkipBackwardFill") => html! {
      <BS_SkipBackwardFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("PersonX") => html! {
      <BS_PersonX ..owned_props />
    },
    implicit_clone::unsync::IString::Static("VolumeUp") => html! {
      <BS_VolumeUp ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Laptop") => html! {
      <BS_Laptop ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Apple") => html! {
      <BS_Apple ..owned_props />
    },
    implicit_clone::unsync::IString::Static("PersonDashFill") => html! {
      <BS_PersonDashFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Unindent") => html! {
      <BS_Unindent ..owned_props />
    },
    implicit_clone::unsync::IString::Static("MoonFill") => html! {
      <BS_MoonFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("8SquareFill") => html! {
      <BS_8SquareFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("HouseHeartFill") => html! {
      <BS_HouseHeartFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FileArrowUpFill") => html! {
      <BS_FileArrowUpFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Snow2") => html! {
      <BS_Snow2 ..owned_props />
    },
    implicit_clone::unsync::IString::Static("At") => html! {
      <BS_At ..owned_props />
    },
    implicit_clone::unsync::IString::Static("CheckSquare") => html! {
      <BS_CheckSquare ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FileTextFill") => html! {
      <BS_FileTextFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FileEarmarkLock") => html! {
      <BS_FileEarmarkLock ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FiletypeM4p") => html! {
      <BS_FiletypeM4p ..owned_props />
    },
    implicit_clone::unsync::IString::Static("SendExclamationFill") => html! {
      <BS_SendExclamationFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("TelephoneMinus") => html! {
      <BS_TelephoneMinus ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Google") => html! {
      <BS_Google ..owned_props />
    },
    implicit_clone::unsync::IString::Static("JournalRichtext") => html! {
      <BS_JournalRichtext ..owned_props />
    },
    implicit_clone::unsync::IString::Static("0CircleFill") => html! {
      <BS_0CircleFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ArrowLeftCircleFill") => html! {
      <BS_ArrowLeftCircleFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("StopBtnFill") => html! {
      <BS_StopBtnFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Moon") => html! {
      <BS_Moon ..owned_props />
    },
    implicit_clone::unsync::IString::Static("TabletLandscapeFill") => html! {
      <BS_TabletLandscapeFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Reply") => html! {
      <BS_Reply ..owned_props />
    },
    implicit_clone::unsync::IString::Static("QuestionSquare") => html! {
      <BS_QuestionSquare ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FiletypeAi") => html! {
      <BS_FiletypeAi ..owned_props />
    },
    implicit_clone::unsync::IString::Static("TruckFront") => html! {
      <BS_TruckFront ..owned_props />
    },
    implicit_clone::unsync::IString::Static("SignIntersectionT") => html! {
      <BS_SignIntersectionT ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ClockHistory") => html! {
      <BS_ClockHistory ..owned_props />
    },
    implicit_clone::unsync::IString::Static("TriangleHalf") => html! {
      <BS_TriangleHalf ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Sunglasses") => html! {
      <BS_Sunglasses ..owned_props />
    },
    implicit_clone::unsync::IString::Static("TypeH3") => html! {
      <BS_TypeH3 ..owned_props />
    },
    implicit_clone::unsync::IString::Static("6Circle") => html! {
      <BS_6Circle ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Pentagon") => html! {
      <BS_Pentagon ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FileEarmarkX") => html! {
      <BS_FileEarmarkX ..owned_props />
    },
    implicit_clone::unsync::IString::Static("StickiesFill") => html! {
      <BS_StickiesFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("NintendoSwitch") => html! {
      <BS_NintendoSwitch ..owned_props />
    },
    implicit_clone::unsync::IString::Static("CollectionPlayFill") => html! {
      <BS_CollectionPlayFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("JournalBookmark") => html! {
      <BS_JournalBookmark ..owned_props />
    },
    implicit_clone::unsync::IString::Static("StickyFill") => html! {
      <BS_StickyFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("CurrencyBitcoin") => html! {
      <BS_CurrencyBitcoin ..owned_props />
    },
    implicit_clone::unsync::IString::Static("BackspaceFill") => html! {
      <BS_BackspaceFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("TextLeft") => html! {
      <BS_TextLeft ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Headphones") => html! {
      <BS_Headphones ..owned_props />
    },
    implicit_clone::unsync::IString::Static("GeoFill") => html! {
      <BS_GeoFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("CloudMinusFill") => html! {
      <BS_CloudMinusFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FileWord") => html! {
      <BS_FileWord ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FileDiffFill") => html! {
      <BS_FileDiffFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("LightbulbOff") => html! {
      <BS_LightbulbOff ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Box2Heart") => html! {
      <BS_Box2Heart ..owned_props />
    },
    implicit_clone::unsync::IString::Static("EnvelopeOpenFill") => html! {
      <BS_EnvelopeOpenFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ArrowLeftShort") => html! {
      <BS_ArrowLeftShort ..owned_props />
    },
    implicit_clone::unsync::IString::Static("PostcardHeart") => html! {
      <BS_PostcardHeart ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Journals") => html! {
      <BS_Journals ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Link") => html! {
      <BS_Link ..owned_props />
    },
    implicit_clone::unsync::IString::Static("TextIndentLeft") => html! {
      <BS_TextIndentLeft ..owned_props />
    },
    implicit_clone::unsync::IString::Static("BookHalf") => html! {
      <BS_BookHalf ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ArrowUpRightSquareFill") => html! {
      <BS_ArrowUpRightSquareFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FileSpreadsheetFill") => html! {
      <BS_FileSpreadsheetFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("UiRadiosGrid") => html! {
      <BS_UiRadiosGrid ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Bank") => html! {
      <BS_Bank ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FiletypeSh") => html! {
      <BS_FiletypeSh ..owned_props />
    },
    implicit_clone::unsync::IString::Static("BoundingBox") => html! {
      <BS_BoundingBox ..owned_props />
    },
    implicit_clone::unsync::IString::Static("CameraFill") => html! {
      <BS_CameraFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Memory") => html! {
      <BS_Memory ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Quote") => html! {
      <BS_Quote ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FilePost") => html! {
      <BS_FilePost ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Calendar3RangeFill") => html! {
      <BS_Calendar3RangeFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("HeartPulse") => html! {
      <BS_HeartPulse ..owned_props />
    },
    implicit_clone::unsync::IString::Static("SkipStartCircle") => html! {
      <BS_SkipStartCircle ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Amd") => html! {
      <BS_Amd ..owned_props />
    },
    implicit_clone::unsync::IString::Static("TextCenter") => html! {
      <BS_TextCenter ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ListOl") => html! {
      <BS_ListOl ..owned_props />
    },
    implicit_clone::unsync::IString::Static("SignDoNotEnterFill") => html! {
      <BS_SignDoNotEnterFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("LayoutTextSidebarReverse") => html! {
      <BS_LayoutTextSidebarReverse ..owned_props />
    },
    implicit_clone::unsync::IString::Static("PersonVcardFill") => html! {
      <BS_PersonVcardFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("DiamondHalf") => html! {
      <BS_DiamondHalf ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Reception3") => html! {
      <BS_Reception3 ..owned_props />
    },
    implicit_clone::unsync::IString::Static("SortDownAlt") => html! {
      <BS_SortDownAlt ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Wifi2") => html! {
      <BS_Wifi2 ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Eye") => html! {
      <BS_Eye ..owned_props />
    },
    implicit_clone::unsync::IString::Static("CalendarXFill") => html! {
      <BS_CalendarXFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Incognito") => html! {
      <BS_Incognito ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ArrowUpCircleFill") => html! {
      <BS_ArrowUpCircleFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("HddStack") => html! {
      <BS_HddStack ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Check") => html! {
      <BS_Check ..owned_props />
    },
    implicit_clone::unsync::IString::Static("InfoSquare") => html! {
      <BS_InfoSquare ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FileEarmarkBreak") => html! {
      <BS_FileEarmarkBreak ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Infinity") => html! {
      <BS_Infinity ..owned_props />
    },
    implicit_clone::unsync::IString::Static("BuildingUp") => html! {
      <BS_BuildingUp ..owned_props />
    },
    implicit_clone::unsync::IString::Static("JournalX") => html! {
      <BS_JournalX ..owned_props />
    },
    implicit_clone::unsync::IString::Static("CalendarPlus") => html! {
      <BS_CalendarPlus ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ExclamationCircle") => html! {
      <BS_ExclamationCircle ..owned_props />
    },
    implicit_clone::unsync::IString::Static("DatabaseFillLock") => html! {
      <BS_DatabaseFillLock ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Calendar2DateFill") => html! {
      <BS_Calendar2DateFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Palette") => html! {
      <BS_Palette ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Window") => html! {
      <BS_Window ..owned_props />
    },
    implicit_clone::unsync::IString::Static("BookmarkHeart") => html! {
      <BS_BookmarkHeart ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Asterisk") => html! {
      <BS_Asterisk ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Basket2") => html! {
      <BS_Basket2 ..owned_props />
    },
    implicit_clone::unsync::IString::Static("MicMuteFill") => html! {
      <BS_MicMuteFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("BrowserSafari") => html! {
      <BS_BrowserSafari ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ChatRightDotsFill") => html! {
      <BS_ChatRightDotsFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ClipboardMinusFill") => html! {
      <BS_ClipboardMinusFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Circle") => html! {
      <BS_Circle ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Easel3Fill") => html! {
      <BS_Easel3Fill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("TrainFreightFrontFill") => html! {
      <BS_TrainFreightFrontFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("GearWide") => html! {
      <BS_GearWide ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ChevronRight") => html! {
      <BS_ChevronRight ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FileEarmarkBinaryFill") => html! {
      <BS_FileEarmarkBinaryFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Wikipedia") => html! {
      <BS_Wikipedia ..owned_props />
    },
    implicit_clone::unsync::IString::Static("9CircleFill") => html! {
      <BS_9CircleFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Droplet") => html! {
      <BS_Droplet ..owned_props />
    },
    implicit_clone::unsync::IString::Static("SignpostSplit") => html! {
      <BS_SignpostSplit ..owned_props />
    },
    implicit_clone::unsync::IString::Static("StopCircle") => html! {
      <BS_StopCircle ..owned_props />
    },
    implicit_clone::unsync::IString::Static("DatabaseFillGear") => html! {
      <BS_DatabaseFillGear ..owned_props />
    },
    implicit_clone::unsync::IString::Static("CaretLeft") => html! {
      <BS_CaretLeft ..owned_props />
    },
    implicit_clone::unsync::IString::Static("InfoCircle") => html! {
      <BS_InfoCircle ..owned_props />
    },
    implicit_clone::unsync::IString::Static("DatabaseFillX") => html! {
      <BS_DatabaseFillX ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Triangle") => html! {
      <BS_Triangle ..owned_props />
    },
    implicit_clone::unsync::IString::Static("HandThumbsUpFill") => html! {
      <BS_HandThumbsUpFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("JournalAlbum") => html! {
      <BS_JournalAlbum ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Cart2") => html! {
      <BS_Cart2 ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ShieldX") => html! {
      <BS_ShieldX ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Record2") => html! {
      <BS_Record2 ..owned_props />
    },
    implicit_clone::unsync::IString::Static("SuitDiamondFill") => html! {
      <BS_SuitDiamondFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("JournalText") => html! {
      <BS_JournalText ..owned_props />
    },
    implicit_clone::unsync::IString::Static("SignYield") => html! {
      <BS_SignYield ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Puzzle") => html! {
      <BS_Puzzle ..owned_props />
    },
    implicit_clone::unsync::IString::Static("TelephoneMinusFill") => html! {
      <BS_TelephoneMinusFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FilePdf") => html! {
      <BS_FilePdf ..owned_props />
    },
    implicit_clone::unsync::IString::Static("MusicNote") => html! {
      <BS_MusicNote ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ArrowsAngleContract") => html! {
      <BS_ArrowsAngleContract ..owned_props />
    },
    implicit_clone::unsync::IString::Static("BoxArrowInLeft") => html! {
      <BS_BoxArrowInLeft ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Recycle") => html! {
      <BS_Recycle ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ChevronDoubleDown") => html! {
      <BS_ChevronDoubleDown ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FiletypePptx") => html! {
      <BS_FiletypePptx ..owned_props />
    },
    implicit_clone::unsync::IString::Static("XCircle") => html! {
      <BS_XCircle ..owned_props />
    },
    implicit_clone::unsync::IString::Static("TypeBold") => html! {
      <BS_TypeBold ..owned_props />
    },
    implicit_clone::unsync::IString::Static("PhoneLandscape") => html! {
      <BS_PhoneLandscape ..owned_props />
    },
    implicit_clone::unsync::IString::Static("DatabaseDown") => html! {
      <BS_DatabaseDown ..owned_props />
    },
    implicit_clone::unsync::IString::Static("DatabaseFillSlash") => html! {
      <BS_DatabaseFillSlash ..owned_props />
    },
    implicit_clone::unsync::IString::Static("PersonBadgeFill") => html! {
      <BS_PersonBadgeFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("PieChart") => html! {
      <BS_PieChart ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Exclamation") => html! {
      <BS_Exclamation ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FileEarmarkLock2") => html! {
      <BS_FileEarmarkLock2 ..owned_props />
    },
    implicit_clone::unsync::IString::Static("DatabaseGear") => html! {
      <BS_DatabaseGear ..owned_props />
    },
    implicit_clone::unsync::IString::Static("BookmarkStarFill") => html! {
      <BS_BookmarkStarFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("SkipStartBtn") => html! {
      <BS_SkipStartBtn ..owned_props />
    },
    implicit_clone::unsync::IString::Static("SignNoRightTurnFill") => html! {
      <BS_SignNoRightTurnFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("BagX") => html! {
      <BS_BagX ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ZoomIn") => html! {
      <BS_ZoomIn ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ArrowRightCircleFill") => html! {
      <BS_ArrowRightCircleFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ClipboardCheck") => html! {
      <BS_ClipboardCheck ..owned_props />
    },
    implicit_clone::unsync::IString::Static("CameraVideoOffFill") => html! {
      <BS_CameraVideoOffFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Database") => html! {
      <BS_Database ..owned_props />
    },
    implicit_clone::unsync::IString::Static("GearWideConnected") => html! {
      <BS_GearWideConnected ..owned_props />
    },
    implicit_clone::unsync::IString::Static("TelephonePlus") => html! {
      <BS_TelephonePlus ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FileEarmarkWordFill") => html! {
      <BS_FileEarmarkWordFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("EyeSlashFill") => html! {
      <BS_EyeSlashFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Search") => html! {
      <BS_Search ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Calendar2Range") => html! {
      <BS_Calendar2Range ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ChatSquareDots") => html! {
      <BS_ChatSquareDots ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Reception0") => html! {
      <BS_Reception0 ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Calendar2WeekFill") => html! {
      <BS_Calendar2WeekFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FileZip") => html! {
      <BS_FileZip ..owned_props />
    },
    implicit_clone::unsync::IString::Static("GlobeCentralSouthAsia") => html! {
      <BS_GlobeCentralSouthAsia ..owned_props />
    },
    implicit_clone::unsync::IString::Static("PinMapFill") => html! {
      <BS_PinMapFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("LayoutSplit") => html! {
      <BS_LayoutSplit ..owned_props />
    },
    implicit_clone::unsync::IString::Static("CloudCheckFill") => html! {
      <BS_CloudCheckFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("MortarboardFill") => html! {
      <BS_MortarboardFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("0SquareFill") => html! {
      <BS_0SquareFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FileLock2") => html! {
      <BS_FileLock2 ..owned_props />
    },
    implicit_clone::unsync::IString::Static("HouseDown") => html! {
      <BS_HouseDown ..owned_props />
    },
    implicit_clone::unsync::IString::Static("CloudHaze2Fill") => html! {
      <BS_CloudHaze2Fill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ArrowDownShort") => html! {
      <BS_ArrowDownShort ..owned_props />
    },
    implicit_clone::unsync::IString::Static("BookmarkXFill") => html! {
      <BS_BookmarkXFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("EvStation") => html! {
      <BS_EvStation ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FilterRight") => html! {
      <BS_FilterRight ..owned_props />
    },
    implicit_clone::unsync::IString::Static("HeartFill") => html! {
      <BS_HeartFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("UnlockFill") => html! {
      <BS_UnlockFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FileEarmarkMusicFill") => html! {
      <BS_FileEarmarkMusicFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Mouse3") => html! {
      <BS_Mouse3 ..owned_props />
    },
    implicit_clone::unsync::IString::Static("HourglassBottom") => html! {
      <BS_HourglassBottom ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Joystick") => html! {
      <BS_Joystick ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FileEarmarkMinus") => html! {
      <BS_FileEarmarkMinus ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Lightbulb") => html! {
      <BS_Lightbulb ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FileEarmarkSpreadsheet") => html! {
      <BS_FileEarmarkSpreadsheet ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FiletypeWav") => html! {
      <BS_FiletypeWav ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Bag") => html! {
      <BS_Bag ..owned_props />
    },
    implicit_clone::unsync::IString::Static("LightningChargeFill") => html! {
      <BS_LightningChargeFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Calendar3") => html! {
      <BS_Calendar3 ..owned_props />
    },
    implicit_clone::unsync::IString::Static("OpticalAudio") => html! {
      <BS_OpticalAudio ..owned_props />
    },
    implicit_clone::unsync::IString::Static("DatabaseCheck") => html! {
      <BS_DatabaseCheck ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ChevronDoubleRight") => html! {
      <BS_ChevronDoubleRight ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Pc") => html! {
      <BS_Pc ..owned_props />
    },
    implicit_clone::unsync::IString::Static("EnvelopePaperFill") => html! {
      <BS_EnvelopePaperFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FileBreak") => html! {
      <BS_FileBreak ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Tsunami") => html! {
      <BS_Tsunami ..owned_props />
    },
    implicit_clone::unsync::IString::Static("CheckSquareFill") => html! {
      <BS_CheckSquareFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("6SquareFill") => html! {
      <BS_6SquareFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("TextRight") => html! {
      <BS_TextRight ..owned_props />
    },
    implicit_clone::unsync::IString::Static("SkipStartBtnFill") => html! {
      <BS_SkipStartBtnFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("SuitHeartFill") => html! {
      <BS_SuitHeartFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("EarFill") => html! {
      <BS_EarFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Robot") => html! {
      <BS_Robot ..owned_props />
    },
    implicit_clone::unsync::IString::Static("UsbDriveFill") => html! {
      <BS_UsbDriveFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Mailbox2") => html! {
      <BS_Mailbox2 ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FiletypeHeic") => html! {
      <BS_FiletypeHeic ..owned_props />
    },
    implicit_clone::unsync::IString::Static("BoxSeamFill") => html! {
      <BS_BoxSeamFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FileEarmarkRichtext") => html! {
      <BS_FileEarmarkRichtext ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Signpost2Fill") => html! {
      <BS_Signpost2Fill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Nut") => html! {
      <BS_Nut ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Alexa") => html! {
      <BS_Alexa ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FileEarmarkFontFill") => html! {
      <BS_FileEarmarkFontFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("CloudDrizzle") => html! {
      <BS_CloudDrizzle ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Key") => html! {
      <BS_Key ..owned_props />
    },
    implicit_clone::unsync::IString::Static("DisplayportFill") => html! {
      <BS_DisplayportFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ClipboardHeart") => html! {
      <BS_ClipboardHeart ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Sim") => html! {
      <BS_Sim ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Pencil") => html! {
      <BS_Pencil ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Save") => html! {
      <BS_Save ..owned_props />
    },
    implicit_clone::unsync::IString::Static("UmbrellaFill") => html! {
      <BS_UmbrellaFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("TabletLandscape") => html! {
      <BS_TabletLandscape ..owned_props />
    },
    implicit_clone::unsync::IString::Static("SlashSquareFill") => html! {
      <BS_SlashSquareFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Ethernet") => html! {
      <BS_Ethernet ..owned_props />
    },
    implicit_clone::unsync::IString::Static("BadgeAdFill") => html! {
      <BS_BadgeAdFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ChevronDown") => html! {
      <BS_ChevronDown ..owned_props />
    },
    implicit_clone::unsync::IString::Static("BadgeWcFill") => html! {
      <BS_BadgeWcFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Eject") => html! {
      <BS_Eject ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Arrow90degRight") => html! {
      <BS_Arrow90degRight ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ThermometerSun") => html! {
      <BS_ThermometerSun ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ChatRightTextFill") => html! {
      <BS_ChatRightTextFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FastForwardFill") => html! {
      <BS_FastForwardFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("SignNoParkingFill") => html! {
      <BS_SignNoParkingFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FileEarmarkFill") => html! {
      <BS_FileEarmarkFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("VolumeDownFill") => html! {
      <BS_VolumeDownFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("MusicNoteList") => html! {
      <BS_MusicNoteList ..owned_props />
    },
    implicit_clone::unsync::IString::Static("EnvelopeExclamationFill") => html! {
      <BS_EnvelopeExclamationFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("TelephoneX") => html! {
      <BS_TelephoneX ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Virus") => html! {
      <BS_Virus ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Basket3") => html! {
      <BS_Basket3 ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Box2HeartFill") => html! {
      <BS_Box2HeartFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("SignIntersectionSideFill") => html! {
      <BS_SignIntersectionSideFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Hearts") => html! {
      <BS_Hearts ..owned_props />
    },
    implicit_clone::unsync::IString::Static("SignMergeLeft") => html! {
      <BS_SignMergeLeft ..owned_props />
    },
    implicit_clone::unsync::IString::Static("CartPlus") => html! {
      <BS_CartPlus ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Heart") => html! {
      <BS_Heart ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Intersect") => html! {
      <BS_Intersect ..owned_props />
    },
    implicit_clone::unsync::IString::Static("CloudSunFill") => html! {
      <BS_CloudSunFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ArrowRightCircle") => html! {
      <BS_ArrowRightCircle ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Textarea") => html! {
      <BS_Textarea ..owned_props />
    },
    implicit_clone::unsync::IString::Static("SymmetryVertical") => html! {
      <BS_SymmetryVertical ..owned_props />
    },
    implicit_clone::unsync::IString::Static("PersonAdd") => html! {
      <BS_PersonAdd ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Share") => html! {
      <BS_Share ..owned_props />
    },
    implicit_clone::unsync::IString::Static("CalendarRangeFill") => html! {
      <BS_CalendarRangeFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("EnvelopeHeart") => html! {
      <BS_EnvelopeHeart ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ChatSquareDotsFill") => html! {
      <BS_ChatSquareDotsFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Bookshelf") => html! {
      <BS_Bookshelf ..owned_props />
    },
    implicit_clone::unsync::IString::Static("LayerBackward") => html! {
      <BS_LayerBackward ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Calendar4Range") => html! {
      <BS_Calendar4Range ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Calendar3Fill") => html! {
      <BS_Calendar3Fill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("DashCircle") => html! {
      <BS_DashCircle ..owned_props />
    },
    implicit_clone::unsync::IString::Static("BadgeCc") => html! {
      <BS_BadgeCc ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Spellcheck") => html! {
      <BS_Spellcheck ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FiletypeDoc") => html! {
      <BS_FiletypeDoc ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Calendar2HeartFill") => html! {
      <BS_Calendar2HeartFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("BadgeHdFill") => html! {
      <BS_BadgeHdFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ReceiptCutoff") => html! {
      <BS_ReceiptCutoff ..owned_props />
    },
    implicit_clone::unsync::IString::Static("BuildingFillLock") => html! {
      <BS_BuildingFillLock ..owned_props />
    },
    implicit_clone::unsync::IString::Static("GraphDown") => html! {
      <BS_GraphDown ..owned_props />
    },
    implicit_clone::unsync::IString::Static("CaretDownSquareFill") => html! {
      <BS_CaretDownSquareFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("PersonLinesFill") => html! {
      <BS_PersonLinesFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Sticky") => html! {
      <BS_Sticky ..owned_props />
    },
    implicit_clone::unsync::IString::Static("CarFront") => html! {
      <BS_CarFront ..owned_props />
    },
    implicit_clone::unsync::IString::Static("1Circle") => html! {
      <BS_1Circle ..owned_props />
    },
    implicit_clone::unsync::IString::Static("SinaWeibo") => html! {
      <BS_SinaWeibo ..owned_props />
    },
    implicit_clone::unsync::IString::Static("WrenchAdjustableCircleFill") => html! {
      <BS_WrenchAdjustableCircleFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("SignTurnSlightLeft") => html! {
      <BS_SignTurnSlightLeft ..owned_props />
    },
    implicit_clone::unsync::IString::Static("SuitSpadeFill") => html! {
      <BS_SuitSpadeFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("BookmarkCheck") => html! {
      <BS_BookmarkCheck ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FuelPumpFill") => html! {
      <BS_FuelPumpFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("SortUp") => html! {
      <BS_SortUp ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FiletypeMd") => html! {
      <BS_FiletypeMd ..owned_props />
    },
    implicit_clone::unsync::IString::Static("SkipEndFill") => html! {
      <BS_SkipEndFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("EnvelopeDashFill") => html! {
      <BS_EnvelopeDashFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FileEarmarkRuled") => html! {
      <BS_FileEarmarkRuled ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Vimeo") => html! {
      <BS_Vimeo ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Info") => html! {
      <BS_Info ..owned_props />
    },
    implicit_clone::unsync::IString::Static("SortNumericDown") => html! {
      <BS_SortNumericDown ..owned_props />
    },
    implicit_clone::unsync::IString::Static("MenuUp") => html! {
      <BS_MenuUp ..owned_props />
    },
    implicit_clone::unsync::IString::Static("4Square") => html! {
      <BS_4Square ..owned_props />
    },
    implicit_clone::unsync::IString::Static("CloudLightningFill") => html! {
      <BS_CloudLightningFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("TrophyFill") => html! {
      <BS_TrophyFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("PenFill") => html! {
      <BS_PenFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FileMinusFill") => html! {
      <BS_FileMinusFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ArrowDownRightCircle") => html! {
      <BS_ArrowDownRightCircle ..owned_props />
    },
    implicit_clone::unsync::IString::Static("EmojiAngryFill") => html! {
      <BS_EmojiAngryFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("AwardFill") => html! {
      <BS_AwardFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Wrench") => html! {
      <BS_Wrench ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FileEarmarkText") => html! {
      <BS_FileEarmarkText ..owned_props />
    },
    implicit_clone::unsync::IString::Static("CloudArrowUp") => html! {
      <BS_CloudArrowUp ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Star") => html! {
      <BS_Star ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FileEarmarkRuledFill") => html! {
      <BS_FileEarmarkRuledFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Stripe") => html! {
      <BS_Stripe ..owned_props />
    },
    implicit_clone::unsync::IString::Static("EnvelopeX") => html! {
      <BS_EnvelopeX ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FiletypeAac") => html! {
      <BS_FiletypeAac ..owned_props />
    },
    implicit_clone::unsync::IString::Static("EvStationFill") => html! {
      <BS_EvStationFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Postage") => html! {
      <BS_Postage ..owned_props />
    },
    implicit_clone::unsync::IString::Static("SkipForward") => html! {
      <BS_SkipForward ..owned_props />
    },
    implicit_clone::unsync::IString::Static("CartX") => html! {
      <BS_CartX ..owned_props />
    },
    implicit_clone::unsync::IString::Static("CardImage") => html! {
      <BS_CardImage ..owned_props />
    },
    implicit_clone::unsync::IString::Static("HeartArrow") => html! {
      <BS_HeartArrow ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Arrow90degLeft") => html! {
      <BS_Arrow90degLeft ..owned_props />
    },
    implicit_clone::unsync::IString::Static("UsbPlugFill") => html! {
      <BS_UsbPlugFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FileEarmarkPlusFill") => html! {
      <BS_FileEarmarkPlusFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("BagXFill") => html! {
      <BS_BagXFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("BagHeart") => html! {
      <BS_BagHeart ..owned_props />
    },
    implicit_clone::unsync::IString::Static("PersonFillSlash") => html! {
      <BS_PersonFillSlash ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ChatHeart") => html! {
      <BS_ChatHeart ..owned_props />
    },
    implicit_clone::unsync::IString::Static("DatabaseFillCheck") => html! {
      <BS_DatabaseFillCheck ..owned_props />
    },
    implicit_clone::unsync::IString::Static("SignStopFill") => html! {
      <BS_SignStopFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("UsbC") => html! {
      <BS_UsbC ..owned_props />
    },
    implicit_clone::unsync::IString::Static("EmojiLaughing") => html! {
      <BS_EmojiLaughing ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Heartbreak") => html! {
      <BS_Heartbreak ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Mouse") => html! {
      <BS_Mouse ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Lamp") => html! {
      <BS_Lamp ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Dice6Fill") => html! {
      <BS_Dice6Fill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("CurrencyExchange") => html! {
      <BS_CurrencyExchange ..owned_props />
    },
    implicit_clone::unsync::IString::Static("EmojiSunglassesFill") => html! {
      <BS_EmojiSunglassesFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Reception1") => html! {
      <BS_Reception1 ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ClipboardPlusFill") => html! {
      <BS_ClipboardPlusFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Scissors") => html! {
      <BS_Scissors ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Linkedin") => html! {
      <BS_Linkedin ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ArrowReturnRight") => html! {
      <BS_ArrowReturnRight ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Repeat1") => html! {
      <BS_Repeat1 ..owned_props />
    },
    implicit_clone::unsync::IString::Static("BagHeartFill") => html! {
      <BS_BagHeartFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("TerminalFill") => html! {
      <BS_TerminalFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FilePlus") => html! {
      <BS_FilePlus ..owned_props />
    },
    implicit_clone::unsync::IString::Static("TelephoneOutbound") => html! {
      <BS_TelephoneOutbound ..owned_props />
    },
    implicit_clone::unsync::IString::Static("DatabaseExclamation") => html! {
      <BS_DatabaseExclamation ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FileXFill") => html! {
      <BS_FileXFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FileRichtext") => html! {
      <BS_FileRichtext ..owned_props />
    },
    implicit_clone::unsync::IString::Static("BuildingAdd") => html! {
      <BS_BuildingAdd ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Markdown") => html! {
      <BS_Markdown ..owned_props />
    },
    implicit_clone::unsync::IString::Static("BookmarkPlusFill") => html! {
      <BS_BookmarkPlusFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("DatabaseFillDash") => html! {
      <BS_DatabaseFillDash ..owned_props />
    },
    implicit_clone::unsync::IString::Static("BlockquoteRight") => html! {
      <BS_BlockquoteRight ..owned_props />
    },
    implicit_clone::unsync::IString::Static("PersonSlash") => html! {
      <BS_PersonSlash ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FiletypePpt") => html! {
      <BS_FiletypePpt ..owned_props />
    },
    implicit_clone::unsync::IString::Static("PatchMinusFill") => html! {
      <BS_PatchMinusFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("EnvelopeOpenHeart") => html! {
      <BS_EnvelopeOpenHeart ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Flower1") => html! {
      <BS_Flower1 ..owned_props />
    },
    implicit_clone::unsync::IString::Static("HouseDoor") => html! {
      <BS_HouseDoor ..owned_props />
    },
    implicit_clone::unsync::IString::Static("CircleSquare") => html! {
      <BS_CircleSquare ..owned_props />
    },
    implicit_clone::unsync::IString::Static("GeoAlt") => html! {
      <BS_GeoAlt ..owned_props />
    },
    implicit_clone::unsync::IString::Static("BuildingFillExclamation") => html! {
      <BS_BuildingFillExclamation ..owned_props />
    },
    implicit_clone::unsync::IString::Static("TextIndentRight") => html! {
      <BS_TextIndentRight ..owned_props />
    },
    implicit_clone::unsync::IString::Static("PersonCheck") => html! {
      <BS_PersonCheck ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Calendar2Check") => html! {
      <BS_Calendar2Check ..owned_props />
    },
    implicit_clone::unsync::IString::Static("PostcardFill") => html! {
      <BS_PostcardFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("PersonHearts") => html! {
      <BS_PersonHearts ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Check2") => html! {
      <BS_Check2 ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Globe") => html! {
      <BS_Globe ..owned_props />
    },
    implicit_clone::unsync::IString::Static("PrinterFill") => html! {
      <BS_PrinterFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Valentine2") => html! {
      <BS_Valentine2 ..owned_props />
    },
    implicit_clone::unsync::IString::Static("YinYang") => html! {
      <BS_YinYang ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Cash") => html! {
      <BS_Cash ..owned_props />
    },
    implicit_clone::unsync::IString::Static("GraphUpArrow") => html! {
      <BS_GraphUpArrow ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Calendar3WeekFill") => html! {
      <BS_Calendar3WeekFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("2SquareFill") => html! {
      <BS_2SquareFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("XLg") => html! {
      <BS_XLg ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Receipt") => html! {
      <BS_Receipt ..owned_props />
    },
    implicit_clone::unsync::IString::Static("WindowSplit") => html! {
      <BS_WindowSplit ..owned_props />
    },
    implicit_clone::unsync::IString::Static("PatchQuestion") => html! {
      <BS_PatchQuestion ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FileFont") => html! {
      <BS_FileFont ..owned_props />
    },
    implicit_clone::unsync::IString::Static("GenderTrans") => html! {
      <BS_GenderTrans ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Tablet") => html! {
      <BS_Tablet ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Upc") => html! {
      <BS_Upc ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Clipboard2HeartFill") => html! {
      <BS_Clipboard2HeartFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("PlusSquareDotted") => html! {
      <BS_PlusSquareDotted ..owned_props />
    },
    implicit_clone::unsync::IString::Static("CloudHazeFill") => html! {
      <BS_CloudHazeFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Display") => html! {
      <BS_Display ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FileCode") => html! {
      <BS_FileCode ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Medium") => html! {
      <BS_Medium ..owned_props />
    },
    implicit_clone::unsync::IString::Static("SignIntersectionTFill") => html! {
      <BS_SignIntersectionTFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("CalendarMonthFill") => html! {
      <BS_CalendarMonthFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("TropicalStorm") => html! {
      <BS_TropicalStorm ..owned_props />
    },
    implicit_clone::unsync::IString::Static("CloudFogFill") => html! {
      <BS_CloudFogFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Easel") => html! {
      <BS_Easel ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Hospital") => html! {
      <BS_Hospital ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Rss") => html! {
      <BS_Rss ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ChatRightText") => html! {
      <BS_ChatRightText ..owned_props />
    },
    implicit_clone::unsync::IString::Static("PcDisplay") => html! {
      <BS_PcDisplay ..owned_props />
    },
    implicit_clone::unsync::IString::Static("TelephoneInboundFill") => html! {
      <BS_TelephoneInboundFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FiletypeRaw") => html! {
      <BS_FiletypeRaw ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ChatSquareQuote") => html! {
      <BS_ChatSquareQuote ..owned_props />
    },
    implicit_clone::unsync::IString::Static("PersonGear") => html! {
      <BS_PersonGear ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FileWordFill") => html! {
      <BS_FileWordFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Calendar3Event") => html! {
      <BS_Calendar3Event ..owned_props />
    },
    implicit_clone::unsync::IString::Static("BugFill") => html! {
      <BS_BugFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("EmojiAngry") => html! {
      <BS_EmojiAngry ..owned_props />
    },
    implicit_clone::unsync::IString::Static("EggFried") => html! {
      <BS_EggFried ..owned_props />
    },
    implicit_clone::unsync::IString::Static("LayersFill") => html! {
      <BS_LayersFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("HourglassSplit") => html! {
      <BS_HourglassSplit ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Border") => html! {
      <BS_Border ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Stopwatch") => html! {
      <BS_Stopwatch ..owned_props />
    },
    implicit_clone::unsync::IString::Static("BorderBottom") => html! {
      <BS_BorderBottom ..owned_props />
    },
    implicit_clone::unsync::IString::Static("BadgeCcFill") => html! {
      <BS_BadgeCcFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Rocket") => html! {
      <BS_Rocket ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Calendar2Event") => html! {
      <BS_Calendar2Event ..owned_props />
    },
    implicit_clone::unsync::IString::Static("PersonFillDash") => html! {
      <BS_PersonFillDash ..owned_props />
    },
    implicit_clone::unsync::IString::Static("EnvelopePaper") => html! {
      <BS_EnvelopePaper ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ChatQuote") => html! {
      <BS_ChatQuote ..owned_props />
    },
    implicit_clone::unsync::IString::Static("InboxFill") => html! {
      <BS_InboxFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FileEarmarkBarGraphFill") => html! {
      <BS_FileEarmarkBarGraphFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("HddStackFill") => html! {
      <BS_HddStackFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Snapchat") => html! {
      <BS_Snapchat ..owned_props />
    },
    implicit_clone::unsync::IString::Static("QuestionDiamondFill") => html! {
      <BS_QuestionDiamondFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Person") => html! {
      <BS_Person ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Box2") => html! {
      <BS_Box2 ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Calendar2RangeFill") => html! {
      <BS_Calendar2RangeFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Code") => html! {
      <BS_Code ..owned_props />
    },
    implicit_clone::unsync::IString::Static("PersonDash") => html! {
      <BS_PersonDash ..owned_props />
    },
    implicit_clone::unsync::IString::Static("CloudSnow") => html! {
      <BS_CloudSnow ..owned_props />
    },
    implicit_clone::unsync::IString::Static("VectorPen") => html! {
      <BS_VectorPen ..owned_props />
    },
    implicit_clone::unsync::IString::Static("TextParagraph") => html! {
      <BS_TextParagraph ..owned_props />
    },
    implicit_clone::unsync::IString::Static("BorderOuter") => html! {
      <BS_BorderOuter ..owned_props />
    },
    implicit_clone::unsync::IString::Static("LayoutTextWindowReverse") => html! {
      <BS_LayoutTextWindowReverse ..owned_props />
    },
    implicit_clone::unsync::IString::Static("BoxArrowUp") => html! {
      <BS_BoxArrowUp ..owned_props />
    },
    implicit_clone::unsync::IString::Static("HouseXFill") => html! {
      <BS_HouseXFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FullscreenExit") => html! {
      <BS_FullscreenExit ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ArrowsAngleExpand") => html! {
      <BS_ArrowsAngleExpand ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ImageFill") => html! {
      <BS_ImageFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("EyeFill") => html! {
      <BS_EyeFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("PlusCircleDotted") => html! {
      <BS_PlusCircleDotted ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Boxes") => html! {
      <BS_Boxes ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Houses") => html! {
      <BS_Houses ..owned_props />
    },
    implicit_clone::unsync::IString::Static("HouseExclamationFill") => html! {
      <BS_HouseExclamationFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Save2Fill") => html! {
      <BS_Save2Fill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("EaselFill") => html! {
      <BS_EaselFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("PersonLock") => html! {
      <BS_PersonLock ..owned_props />
    },
    implicit_clone::unsync::IString::Static("PersonVideo2") => html! {
      <BS_PersonVideo2 ..owned_props />
    },
    implicit_clone::unsync::IString::Static("EvFrontFill") => html! {
      <BS_EvFrontFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ListNested") => html! {
      <BS_ListNested ..owned_props />
    },
    implicit_clone::unsync::IString::Static("RewindFill") => html! {
      <BS_RewindFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("DropletFill") => html! {
      <BS_DropletFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Power") => html! {
      <BS_Power ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Screwdriver") => html! {
      <BS_Screwdriver ..owned_props />
    },
    implicit_clone::unsync::IString::Static("SegmentedNav") => html! {
      <BS_SegmentedNav ..owned_props />
    },
    implicit_clone::unsync::IString::Static("PhoneVibrateFill") => html! {
      <BS_PhoneVibrateFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("CheckAll") => html! {
      <BS_CheckAll ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Dice4Fill") => html! {
      <BS_Dice4Fill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("MenuAppFill") => html! {
      <BS_MenuAppFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("EnvelopePlusFill") => html! {
      <BS_EnvelopePlusFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("BorderLeft") => html! {
      <BS_BorderLeft ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Badge3dFill") => html! {
      <BS_Badge3dFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Calendar3Week") => html! {
      <BS_Calendar3Week ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ChatText") => html! {
      <BS_ChatText ..owned_props />
    },
    implicit_clone::unsync::IString::Static("CaretLeftSquare") => html! {
      <BS_CaretLeftSquare ..owned_props />
    },
    implicit_clone::unsync::IString::Static("SlashCircleFill") => html! {
      <BS_SlashCircleFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("WrenchAdjustableCircle") => html! {
      <BS_WrenchAdjustableCircle ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ChatDots") => html! {
      <BS_ChatDots ..owned_props />
    },
    implicit_clone::unsync::IString::Static("BadgeVr") => html! {
      <BS_BadgeVr ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FileEarmarkBreakFill") => html! {
      <BS_FileEarmarkBreakFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("CloudHail") => html! {
      <BS_CloudHail ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FolderX") => html! {
      <BS_FolderX ..owned_props />
    },
    implicit_clone::unsync::IString::Static("EmojiWink") => html! {
      <BS_EmojiWink ..owned_props />
    },
    implicit_clone::unsync::IString::Static("4SquareFill") => html! {
      <BS_4SquareFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ArrowDownLeft") => html! {
      <BS_ArrowDownLeft ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FileEarmarkPostFill") => html! {
      <BS_FileEarmarkPostFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("People") => html! {
      <BS_People ..owned_props />
    },
    implicit_clone::unsync::IString::Static("HSquare") => html! {
      <BS_HSquare ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ChatRightDots") => html! {
      <BS_ChatRightDots ..owned_props />
    },
    implicit_clone::unsync::IString::Static("UsbDrive") => html! {
      <BS_UsbDrive ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Signal") => html! {
      <BS_Signal ..owned_props />
    },
    implicit_clone::unsync::IString::Static("GenderFemale") => html! {
      <BS_GenderFemale ..owned_props />
    },
    implicit_clone::unsync::IString::Static("PersonSquare") => html! {
      <BS_PersonSquare ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Truck") => html! {
      <BS_Truck ..owned_props />
    },
    implicit_clone::unsync::IString::Static("SkipBackward") => html! {
      <BS_SkipBackward ..owned_props />
    },
    implicit_clone::unsync::IString::Static("PlusSquareFill") => html! {
      <BS_PlusSquareFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ArrowDownCircleFill") => html! {
      <BS_ArrowDownCircleFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("File") => html! {
      <BS_File ..owned_props />
    },
    implicit_clone::unsync::IString::Static("App") => html! {
      <BS_App ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Grid3x2Gap") => html! {
      <BS_Grid3x2Gap ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Windows") => html! {
      <BS_Windows ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FileZipFill") => html! {
      <BS_FileZipFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("VolumeMute") => html! {
      <BS_VolumeMute ..owned_props />
    },
    implicit_clone::unsync::IString::Static("4Circle") => html! {
      <BS_4Circle ..owned_props />
    },
    implicit_clone::unsync::IString::Static("SignRailroad") => html! {
      <BS_SignRailroad ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FiletypeMp4") => html! {
      <BS_FiletypeMp4 ..owned_props />
    },
    implicit_clone::unsync::IString::Static("HeartbreakFill") => html! {
      <BS_HeartbreakFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("CaretDown") => html! {
      <BS_CaretDown ..owned_props />
    },
    implicit_clone::unsync::IString::Static("MicrosoftTeams") => html! {
      <BS_MicrosoftTeams ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ShieldPlus") => html! {
      <BS_ShieldPlus ..owned_props />
    },
    implicit_clone::unsync::IString::Static("SymmetryHorizontal") => html! {
      <BS_SymmetryHorizontal ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Clipboard2XFill") => html! {
      <BS_Clipboard2XFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FileRichtextFill") => html! {
      <BS_FileRichtextFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ArrowDownRightCircleFill") => html! {
      <BS_ArrowDownRightCircleFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("MapFill") => html! {
      <BS_MapFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("CompassFill") => html! {
      <BS_CompassFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("EmojiWinkFill") => html! {
      <BS_EmojiWinkFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FilePlay") => html! {
      <BS_FilePlay ..owned_props />
    },
    implicit_clone::unsync::IString::Static("GraphUp") => html! {
      <BS_GraphUp ..owned_props />
    },
    implicit_clone::unsync::IString::Static("SignTurnRightFill") => html! {
      <BS_SignTurnRightFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("5SquareFill") => html! {
      <BS_5SquareFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("SkipBackwardCircle") => html! {
      <BS_SkipBackwardCircle ..owned_props />
    },
    implicit_clone::unsync::IString::Static("SortAlphaUp") => html! {
      <BS_SortAlphaUp ..owned_props />
    },
    implicit_clone::unsync::IString::Static("HexagonFill") => html! {
      <BS_HexagonFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ChatSquareText") => html! {
      <BS_ChatSquareText ..owned_props />
    },
    implicit_clone::unsync::IString::Static("SignNoLeftTurn") => html! {
      <BS_SignNoLeftTurn ..owned_props />
    },
    implicit_clone::unsync::IString::Static("TvFill") => html! {
      <BS_TvFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("MenuButtonFill") => html! {
      <BS_MenuButtonFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("JournalMinus") => html! {
      <BS_JournalMinus ..owned_props />
    },
    implicit_clone::unsync::IString::Static("CartPlusFill") => html! {
      <BS_CartPlusFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("EnvelopeAt") => html! {
      <BS_EnvelopeAt ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Journal") => html! {
      <BS_Journal ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Percent") => html! {
      <BS_Percent ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Rulers") => html! {
      <BS_Rulers ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Toggle2Off") => html! {
      <BS_Toggle2Off ..owned_props />
    },
    implicit_clone::unsync::IString::Static("BuildingsFill") => html! {
      <BS_BuildingsFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Mouse3Fill") => html! {
      <BS_Mouse3Fill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FiletypeTiff") => html! {
      <BS_FiletypeTiff ..owned_props />
    },
    implicit_clone::unsync::IString::Static("HouseFill") => html! {
      <BS_HouseFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("BoomboxFill") => html! {
      <BS_BoomboxFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("BuildingFillDash") => html! {
      <BS_BuildingFillDash ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Clipboard2DataFill") => html! {
      <BS_Clipboard2DataFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Filter") => html! {
      <BS_Filter ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Sliders2") => html! {
      <BS_Sliders2 ..owned_props />
    },
    implicit_clone::unsync::IString::Static("SignTurnLeft") => html! {
      <BS_SignTurnLeft ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Radioactive") => html! {
      <BS_Radioactive ..owned_props />
    },
    implicit_clone::unsync::IString::Static("CaretLeftFill") => html! {
      <BS_CaretLeftFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("7SquareFill") => html! {
      <BS_7SquareFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("CloudHaze2") => html! {
      <BS_CloudHaze2 ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ArrowRight") => html! {
      <BS_ArrowRight ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Facebook") => html! {
      <BS_Facebook ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Magnet") => html! {
      <BS_Magnet ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FileSlidesFill") => html! {
      <BS_FileSlidesFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FileExcel") => html! {
      <BS_FileExcel ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ArrowRightSquareFill") => html! {
      <BS_ArrowRightSquareFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("CodeSquare") => html! {
      <BS_CodeSquare ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Calendar2XFill") => html! {
      <BS_Calendar2XFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("HouseHeart") => html! {
      <BS_HouseHeart ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FileEarmarkImage") => html! {
      <BS_FileEarmarkImage ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Bucket") => html! {
      <BS_Bucket ..owned_props />
    },
    implicit_clone::unsync::IString::Static("CupStraw") => html! {
      <BS_CupStraw ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FileEarmarkLock2Fill") => html! {
      <BS_FileEarmarkLock2Fill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("MoonStarsFill") => html! {
      <BS_MoonStarsFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FilePdfFill") => html! {
      <BS_FilePdfFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Gear") => html! {
      <BS_Gear ..owned_props />
    },
    implicit_clone::unsync::IString::Static("BoxArrowInRight") => html! {
      <BS_BoxArrowInRight ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FiletypeHtml") => html! {
      <BS_FiletypeHtml ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Table") => html! {
      <BS_Table ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ArrowDownRightSquare") => html! {
      <BS_ArrowDownRightSquare ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Dice3Fill") => html! {
      <BS_Dice3Fill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("DatabaseDash") => html! {
      <BS_DatabaseDash ..owned_props />
    },
    implicit_clone::unsync::IString::Static("2Circle") => html! {
      <BS_2Circle ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Clipboard2Fill") => html! {
      <BS_Clipboard2Fill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Rewind") => html! {
      <BS_Rewind ..owned_props />
    },
    implicit_clone::unsync::IString::Static("CreditCardFill") => html! {
      <BS_CreditCardFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Dot") => html! {
      <BS_Dot ..owned_props />
    },
    implicit_clone::unsync::IString::Static("NutFill") => html! {
      <BS_NutFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FiletypeTsx") => html! {
      <BS_FiletypeTsx ..owned_props />
    },
    implicit_clone::unsync::IString::Static("BookmarkDash") => html! {
      <BS_BookmarkDash ..owned_props />
    },
    implicit_clone::unsync::IString::Static("CloudFog") => html! {
      <BS_CloudFog ..owned_props />
    },
    implicit_clone::unsync::IString::Static("RocketFill") => html! {
      <BS_RocketFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FileEarmarkCheckFill") => html! {
      <BS_FileEarmarkCheckFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Bluetooth") => html! {
      <BS_Bluetooth ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Shuffle") => html! {
      <BS_Shuffle ..owned_props />
    },
    implicit_clone::unsync::IString::Static("BadgeAr") => html! {
      <BS_BadgeAr ..owned_props />
    },
    implicit_clone::unsync::IString::Static("PipFill") => html! {
      <BS_PipFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Bootstrap") => html! {
      <BS_Bootstrap ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Film") => html! {
      <BS_Film ..owned_props />
    },
    implicit_clone::unsync::IString::Static("BatteryCharging") => html! {
      <BS_BatteryCharging ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Clipboard2Plus") => html! {
      <BS_Clipboard2Plus ..owned_props />
    },
    implicit_clone::unsync::IString::Static("CloudSleetFill") => html! {
      <BS_CloudSleetFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("SkipForwardFill") => html! {
      <BS_SkipForwardFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ChevronCompactLeft") => html! {
      <BS_ChevronCompactLeft ..owned_props />
    },
    implicit_clone::unsync::IString::Static("MicFill") => html! {
      <BS_MicFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Safe2Fill") => html! {
      <BS_Safe2Fill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("KanbanFill") => html! {
      <BS_KanbanFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("RewindBtnFill") => html! {
      <BS_RewindBtnFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ReplyFill") => html! {
      <BS_ReplyFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Twitter") => html! {
      <BS_Twitter ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ToggleOn") => html! {
      <BS_ToggleOn ..owned_props />
    },
    implicit_clone::unsync::IString::Static("CurrencyRupee") => html! {
      <BS_CurrencyRupee ..owned_props />
    },
    implicit_clone::unsync::IString::Static("BadgeSdFill") => html! {
      <BS_BadgeSdFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("PersonVcard") => html! {
      <BS_PersonVcard ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Cpu") => html! {
      <BS_Cpu ..owned_props />
    },
    implicit_clone::unsync::IString::Static("CaretDownSquare") => html! {
      <BS_CaretDownSquare ..owned_props />
    },
    implicit_clone::unsync::IString::Static("NodeMinus") => html! {
      <BS_NodeMinus ..owned_props />
    },
    implicit_clone::unsync::IString::Static("HandbagFill") => html! {
      <BS_HandbagFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("DoorOpenFill") => html! {
      <BS_DoorOpenFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("EvFront") => html! {
      <BS_EvFront ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ArrowBarUp") => html! {
      <BS_ArrowBarUp ..owned_props />
    },
    implicit_clone::unsync::IString::Static("PlayBtn") => html! {
      <BS_PlayBtn ..owned_props />
    },
    implicit_clone::unsync::IString::Static("SendPlus") => html! {
      <BS_SendPlus ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ChevronBarDown") => html! {
      <BS_ChevronBarDown ..owned_props />
    },
    implicit_clone::unsync::IString::Static("CaretDownFill") => html! {
      <BS_CaretDownFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("MicMute") => html! {
      <BS_MicMute ..owned_props />
    },
    implicit_clone::unsync::IString::Static("PersonFillDown") => html! {
      <BS_PersonFillDown ..owned_props />
    },
    implicit_clone::unsync::IString::Static("TypeH1") => html! {
      <BS_TypeH1 ..owned_props />
    },
    implicit_clone::unsync::IString::Static("PlayBtnFill") => html! {
      <BS_PlayBtnFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("9Circle") => html! {
      <BS_9Circle ..owned_props />
    },
    implicit_clone::unsync::IString::Static("DatabaseFillDown") => html! {
      <BS_DatabaseFillDown ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Unlock") => html! {
      <BS_Unlock ..owned_props />
    },
    implicit_clone::unsync::IString::Static("BrightnessAltHigh") => html! {
      <BS_BrightnessAltHigh ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ChatDotsFill") => html! {
      <BS_ChatDotsFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Eyeglasses") => html! {
      <BS_Eyeglasses ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ArrowsMove") => html! {
      <BS_ArrowsMove ..owned_props />
    },
    implicit_clone::unsync::IString::Static("MouseFill") => html! {
      <BS_MouseFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FileEarmarkZip") => html! {
      <BS_FileEarmarkZip ..owned_props />
    },
    implicit_clone::unsync::IString::Static("SlashCircle") => html! {
      <BS_SlashCircle ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Alt") => html! {
      <BS_Alt ..owned_props />
    },
    implicit_clone::unsync::IString::Static("SignIntersectionYFill") => html! {
      <BS_SignIntersectionYFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("BrightnessHighFill") => html! {
      <BS_BrightnessHighFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Lock") => html! {
      <BS_Lock ..owned_props />
    },
    implicit_clone::unsync::IString::Static("CloudMoon") => html! {
      <BS_CloudMoon ..owned_props />
    },
    implicit_clone::unsync::IString::Static("HandIndexFill") => html! {
      <BS_HandIndexFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("BorderStyle") => html! {
      <BS_BorderStyle ..owned_props />
    },
    implicit_clone::unsync::IString::Static("CloudRainFill") => html! {
      <BS_CloudRainFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Telegram") => html! {
      <BS_Telegram ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Tag") => html! {
      <BS_Tag ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Trello") => html! {
      <BS_Trello ..owned_props />
    },
    implicit_clone::unsync::IString::Static("CalendarMonth") => html! {
      <BS_CalendarMonth ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Safe2") => html! {
      <BS_Safe2 ..owned_props />
    },
    implicit_clone::unsync::IString::Static("BuildingDash") => html! {
      <BS_BuildingDash ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FiletypeJs") => html! {
      <BS_FiletypeJs ..owned_props />
    },
    implicit_clone::unsync::IString::Static("BoxArrowRight") => html! {
      <BS_BoxArrowRight ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Messenger") => html! {
      <BS_Messenger ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ArrowLeft") => html! {
      <BS_ArrowLeft ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Palette2") => html! {
      <BS_Palette2 ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FileEarmarkXFill") => html! {
      <BS_FileEarmarkXFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FastForwardBtn") => html! {
      <BS_FastForwardBtn ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FileDiff") => html! {
      <BS_FileDiff ..owned_props />
    },
    implicit_clone::unsync::IString::Static("BadgeVrFill") => html! {
      <BS_BadgeVrFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Broadcast") => html! {
      <BS_Broadcast ..owned_props />
    },
    implicit_clone::unsync::IString::Static("PauseCircle") => html! {
      <BS_PauseCircle ..owned_props />
    },
    implicit_clone::unsync::IString::Static("PauseFill") => html! {
      <BS_PauseFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("MotherboardFill") => html! {
      <BS_MotherboardFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ArrowRightSquare") => html! {
      <BS_ArrowRightSquare ..owned_props />
    },
    implicit_clone::unsync::IString::Static("DatabaseFillAdd") => html! {
      <BS_DatabaseFillAdd ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FilterSquareFill") => html! {
      <BS_FilterSquareFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Virus2") => html! {
      <BS_Virus2 ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ChatTextFill") => html! {
      <BS_ChatTextFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("SkipForwardCircleFill") => html! {
      <BS_SkipForwardCircleFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ChatRightQuoteFill") => html! {
      <BS_ChatRightQuoteFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("PeaceFill") => html! {
      <BS_PeaceFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("PinFill") => html! {
      <BS_PinFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Box2Fill") => html! {
      <BS_Box2Fill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ChatSquareHeartFill") => html! {
      <BS_ChatSquareHeartFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("BarChartLine") => html! {
      <BS_BarChartLine ..owned_props />
    },
    implicit_clone::unsync::IString::Static("CalendarHeart") => html! {
      <BS_CalendarHeart ..owned_props />
    },
    implicit_clone::unsync::IString::Static("PhoneFill") => html! {
      <BS_PhoneFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("SendSlash") => html! {
      <BS_SendSlash ..owned_props />
    },
    implicit_clone::unsync::IString::Static("TagsFill") => html! {
      <BS_TagsFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Crop") => html! {
      <BS_Crop ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FiletypeTtf") => html! {
      <BS_FiletypeTtf ..owned_props />
    },
    implicit_clone::unsync::IString::Static("MenuButtonWide") => html! {
      <BS_MenuButtonWide ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Award") => html! {
      <BS_Award ..owned_props />
    },
    implicit_clone::unsync::IString::Static("3SquareFill") => html! {
      <BS_3SquareFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("GripHorizontal") => html! {
      <BS_GripHorizontal ..owned_props />
    },
    implicit_clone::unsync::IString::Static("9SquareFill") => html! {
      <BS_9SquareFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FileMusic") => html! {
      <BS_FileMusic ..owned_props />
    },
    implicit_clone::unsync::IString::Static("CursorFill") => html! {
      <BS_CursorFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("PersonVideo3") => html! {
      <BS_PersonVideo3 ..owned_props />
    },
    implicit_clone::unsync::IString::Static("HddFill") => html! {
      <BS_HddFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("SuitClub") => html! {
      <BS_SuitClub ..owned_props />
    },
    implicit_clone::unsync::IString::Static("SuitClubFill") => html! {
      <BS_SuitClubFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Collection") => html! {
      <BS_Collection ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Option") => html! {
      <BS_Option ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ChevronDoubleLeft") => html! {
      <BS_ChevronDoubleLeft ..owned_props />
    },
    implicit_clone::unsync::IString::Static("SafeFill") => html! {
      <BS_SafeFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("BrushFill") => html! {
      <BS_BrushFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("6Square") => html! {
      <BS_6Square ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ShopWindow") => html! {
      <BS_ShopWindow ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ShieldExclamation") => html! {
      <BS_ShieldExclamation ..owned_props />
    },
    implicit_clone::unsync::IString::Static("RecordCircleFill") => html! {
      <BS_RecordCircleFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FilePptFill") => html! {
      <BS_FilePptFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Dice1Fill") => html! {
      <BS_Dice1Fill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ArrowBarLeft") => html! {
      <BS_ArrowBarLeft ..owned_props />
    },
    implicit_clone::unsync::IString::Static("HddNetwork") => html! {
      <BS_HddNetwork ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ClockFill") => html! {
      <BS_ClockFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("TabletFill") => html! {
      <BS_TabletFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("HeartPulseFill") => html! {
      <BS_HeartPulseFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FiletypePdf") => html! {
      <BS_FiletypePdf ..owned_props />
    },
    implicit_clone::unsync::IString::Static("BoxArrowInDown") => html! {
      <BS_BoxArrowInDown ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Repeat") => html! {
      <BS_Repeat ..owned_props />
    },
    implicit_clone::unsync::IString::Static("BasketFill") => html! {
      <BS_BasketFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Globe2") => html! {
      <BS_Globe2 ..owned_props />
    },
    implicit_clone::unsync::IString::Static("SunFill") => html! {
      <BS_SunFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Gift") => html! {
      <BS_Gift ..owned_props />
    },
    implicit_clone::unsync::IString::Static("LayoutSidebarInsetReverse") => html! {
      <BS_LayoutSidebarInsetReverse ..owned_props />
    },
    implicit_clone::unsync::IString::Static("PlugFill") => html! {
      <BS_PlugFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Phone") => html! {
      <BS_Phone ..owned_props />
    },
    implicit_clone::unsync::IString::Static("AspectRatio") => html! {
      <BS_AspectRatio ..owned_props />
    },
    implicit_clone::unsync::IString::Static("TruckFrontFill") => html! {
      <BS_TruckFrontFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Behance") => html! {
      <BS_Behance ..owned_props />
    },
    implicit_clone::unsync::IString::Static("UniversalAccessCircle") => html! {
      <BS_UniversalAccessCircle ..owned_props />
    },
    implicit_clone::unsync::IString::Static("GraphDownArrow") => html! {
      <BS_GraphDownArrow ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FileEarmarkZipFill") => html! {
      <BS_FileEarmarkZipFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("BandaidFill") => html! {
      <BS_BandaidFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ArrowDownSquare") => html! {
      <BS_ArrowDownSquare ..owned_props />
    },
    implicit_clone::unsync::IString::Static("CalendarCheckFill") => html! {
      <BS_CalendarCheckFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("BookFill") => html! {
      <BS_BookFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("UsbMicroFill") => html! {
      <BS_UsbMicroFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Controller") => html! {
      <BS_Controller ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Fonts") => html! {
      <BS_Fonts ..owned_props />
    },
    implicit_clone::unsync::IString::Static("EnvelopeExclamation") => html! {
      <BS_EnvelopeExclamation ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Pause") => html! {
      <BS_Pause ..owned_props />
    },
    implicit_clone::unsync::IString::Static("BookmarkDashFill") => html! {
      <BS_BookmarkDashFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("BoxArrowDownLeft") => html! {
      <BS_BoxArrowDownLeft ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Front") => html! {
      <BS_Front ..owned_props />
    },
    implicit_clone::unsync::IString::Static("SortNumericDownAlt") => html! {
      <BS_SortNumericDownAlt ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Dice6") => html! {
      <BS_Dice6 ..owned_props />
    },
    implicit_clone::unsync::IString::Static("RocketTakeoffFill") => html! {
      <BS_RocketTakeoffFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Folder2") => html! {
      <BS_Folder2 ..owned_props />
    },
    implicit_clone::unsync::IString::Static("WindowDesktop") => html! {
      <BS_WindowDesktop ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FileMinus") => html! {
      <BS_FileMinus ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Mailbox") => html! {
      <BS_Mailbox ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ThermometerHalf") => html! {
      <BS_ThermometerHalf ..owned_props />
    },
    implicit_clone::unsync::IString::Static("PersonCheckFill") => html! {
      <BS_PersonCheckFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("PlusCircleFill") => html! {
      <BS_PlusCircleFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Flower2") => html! {
      <BS_Flower2 ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ArrowsExpand") => html! {
      <BS_ArrowsExpand ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ShieldFillPlus") => html! {
      <BS_ShieldFillPlus ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Flower3") => html! {
      <BS_Flower3 ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FiletypeMov") => html! {
      <BS_FiletypeMov ..owned_props />
    },
    implicit_clone::unsync::IString::Static("PentagonFill") => html! {
      <BS_PentagonFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("CloudDownloadFill") => html! {
      <BS_CloudDownloadFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Shift") => html! {
      <BS_Shift ..owned_props />
    },
    implicit_clone::unsync::IString::Static("UsbMini") => html! {
      <BS_UsbMini ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FileEarmarkCheck") => html! {
      <BS_FileEarmarkCheck ..owned_props />
    },
    implicit_clone::unsync::IString::Static("4CircleFill") => html! {
      <BS_4CircleFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("NodeMinusFill") => html! {
      <BS_NodeMinusFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Check2All") => html! {
      <BS_Check2All ..owned_props />
    },
    implicit_clone::unsync::IString::Static("SignStop") => html! {
      <BS_SignStop ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Type") => html! {
      <BS_Type ..owned_props />
    },
    implicit_clone::unsync::IString::Static("LayoutSidebar") => html! {
      <BS_LayoutSidebar ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FileArrowDown") => html! {
      <BS_FileArrowDown ..owned_props />
    },
    implicit_clone::unsync::IString::Static("BagDash") => html! {
      <BS_BagDash ..owned_props />
    },
    implicit_clone::unsync::IString::Static("CpuFill") => html! {
      <BS_CpuFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("PCircle") => html! {
      <BS_PCircle ..owned_props />
    },
    implicit_clone::unsync::IString::Static("RouterFill") => html! {
      <BS_RouterFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("BalloonHeartFill") => html! {
      <BS_BalloonHeartFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FileEarmarkEaselFill") => html! {
      <BS_FileEarmarkEaselFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Clipboard2Data") => html! {
      <BS_Clipboard2Data ..owned_props />
    },
    implicit_clone::unsync::IString::Static("PersonBoundingBox") => html! {
      <BS_PersonBoundingBox ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Kanban") => html! {
      <BS_Kanban ..owned_props />
    },
    implicit_clone::unsync::IString::Static("BoxArrowInDownLeft") => html! {
      <BS_BoxArrowInDownLeft ..owned_props />
    },
    implicit_clone::unsync::IString::Static("TelephoneFill") => html! {
      <BS_TelephoneFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("PersonExclamation") => html! {
      <BS_PersonExclamation ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FileCheck") => html! {
      <BS_FileCheck ..owned_props />
    },
    implicit_clone::unsync::IString::Static("HouseAdd") => html! {
      <BS_HouseAdd ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FileEaselFill") => html! {
      <BS_FileEaselFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Shop") => html! {
      <BS_Shop ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ArrowCounterclockwise") => html! {
      <BS_ArrowCounterclockwise ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Bullseye") => html! {
      <BS_Bullseye ..owned_props />
    },
    implicit_clone::unsync::IString::Static("GlobeAsiaAustralia") => html! {
      <BS_GlobeAsiaAustralia ..owned_props />
    },
    implicit_clone::unsync::IString::Static("PlusSlashMinus") => html! {
      <BS_PlusSlashMinus ..owned_props />
    },
    implicit_clone::unsync::IString::Static("PassFill") => html! {
      <BS_PassFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("EnvelopeCheck") => html! {
      <BS_EnvelopeCheck ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Wind") => html! {
      <BS_Wind ..owned_props />
    },
    implicit_clone::unsync::IString::Static("5CircleFill") => html! {
      <BS_5CircleFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Geo") => html! {
      <BS_Geo ..owned_props />
    },
    implicit_clone::unsync::IString::Static("PciCard") => html! {
      <BS_PciCard ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Calendar4Event") => html! {
      <BS_Calendar4Event ..owned_props />
    },
    implicit_clone::unsync::IString::Static("MusicPlayerFill") => html! {
      <BS_MusicPlayerFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FiletypeJava") => html! {
      <BS_FiletypeJava ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FiletypeCss") => html! {
      <BS_FiletypeCss ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FileEarmarkDiff") => html! {
      <BS_FileEarmarkDiff ..owned_props />
    },
    implicit_clone::unsync::IString::Static("BuildingFillCheck") => html! {
      <BS_BuildingFillCheck ..owned_props />
    },
    implicit_clone::unsync::IString::Static("EnvelopeOpen") => html! {
      <BS_EnvelopeOpen ..owned_props />
    },
    implicit_clone::unsync::IString::Static("PersonFillUp") => html! {
      <BS_PersonFillUp ..owned_props />
    },
    implicit_clone::unsync::IString::Static("LaptopFill") => html! {
      <BS_LaptopFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("DeviceSsdFill") => html! {
      <BS_DeviceSsdFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("PersonVideo") => html! {
      <BS_PersonVideo ..owned_props />
    },
    implicit_clone::unsync::IString::Static("SkipStartFill") => html! {
      <BS_SkipStartFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Camera") => html! {
      <BS_Camera ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Ubuntu") => html! {
      <BS_Ubuntu ..owned_props />
    },
    implicit_clone::unsync::IString::Static("CloudUploadFill") => html! {
      <BS_CloudUploadFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("DatabaseUp") => html! {
      <BS_DatabaseUp ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Fire") => html! {
      <BS_Fire ..owned_props />
    },
    implicit_clone::unsync::IString::Static("BorderInner") => html! {
      <BS_BorderInner ..owned_props />
    },
    implicit_clone::unsync::IString::Static("BadgeWc") => html! {
      <BS_BadgeWc ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ExclamationSquareFill") => html! {
      <BS_ExclamationSquareFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("BrightnessAltHighFill") => html! {
      <BS_BrightnessAltHighFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Bandaid") => html! {
      <BS_Bandaid ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Vr") => html! {
      <BS_Vr ..owned_props />
    },
    implicit_clone::unsync::IString::Static("PlusLg") => html! {
      <BS_PlusLg ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Webcam") => html! {
      <BS_Webcam ..owned_props />
    },
    implicit_clone::unsync::IString::Static("PersonFill") => html! {
      <BS_PersonFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("PiggyBankFill") => html! {
      <BS_PiggyBankFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("SignTurnRight") => html! {
      <BS_SignTurnRight ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Calendar2CheckFill") => html! {
      <BS_Calendar2CheckFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("BoxArrowInUpRight") => html! {
      <BS_BoxArrowInUpRight ..owned_props />
    },
    implicit_clone::unsync::IString::Static("BoundingBoxCircles") => html! {
      <BS_BoundingBoxCircles ..owned_props />
    },
    implicit_clone::unsync::IString::Static("SendX") => html! {
      <BS_SendX ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ChevronBarLeft") => html! {
      <BS_ChevronBarLeft ..owned_props />
    },
    implicit_clone::unsync::IString::Static("BarChart") => html! {
      <BS_BarChart ..owned_props />
    },
    implicit_clone::unsync::IString::Static("DashSquare") => html! {
      <BS_DashSquare ..owned_props />
    },
    implicit_clone::unsync::IString::Static("9Square") => html! {
      <BS_9Square ..owned_props />
    },
    implicit_clone::unsync::IString::Static("PhoneVibrate") => html! {
      <BS_PhoneVibrate ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Outlet") => html! {
      <BS_Outlet ..owned_props />
    },
    implicit_clone::unsync::IString::Static("LightningCharge") => html! {
      <BS_LightningCharge ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ArrowUpRight") => html! {
      <BS_ArrowUpRight ..owned_props />
    },
    implicit_clone::unsync::IString::Static("HandThumbsDownFill") => html! {
      <BS_HandThumbsDownFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Dice3") => html! {
      <BS_Dice3 ..owned_props />
    },
    implicit_clone::unsync::IString::Static("GripVertical") => html! {
      <BS_GripVertical ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Strava") => html! {
      <BS_Strava ..owned_props />
    },
    implicit_clone::unsync::IString::Static("EnvelopeOpenHeartFill") => html! {
      <BS_EnvelopeOpenHeartFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FiletypePhp") => html! {
      <BS_FiletypePhp ..owned_props />
    },
    implicit_clone::unsync::IString::Static("WebcamFill") => html! {
      <BS_WebcamFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("SunriseFill") => html! {
      <BS_SunriseFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("EmojiNeutralFill") => html! {
      <BS_EmojiNeutralFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Subtract") => html! {
      <BS_Subtract ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Briefcase") => html! {
      <BS_Briefcase ..owned_props />
    },
    implicit_clone::unsync::IString::Static("BrowserFirefox") => html! {
      <BS_BrowserFirefox ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Building") => html! {
      <BS_Building ..owned_props />
    },
    implicit_clone::unsync::IString::Static("SignYieldFill") => html! {
      <BS_SignYieldFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("CaretUpSquareFill") => html! {
      <BS_CaretUpSquareFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FolderPlus") => html! {
      <BS_FolderPlus ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Dpad") => html! {
      <BS_Dpad ..owned_props />
    },
    implicit_clone::unsync::IString::Static("CardText") => html! {
      <BS_CardText ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Calendar4Week") => html! {
      <BS_Calendar4Week ..owned_props />
    },
    implicit_clone::unsync::IString::Static("EmojiDizzyFill") => html! {
      <BS_EmojiDizzyFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("PcDisplayHorizontal") => html! {
      <BS_PcDisplayHorizontal ..owned_props />
    },
    implicit_clone::unsync::IString::Static("BoxArrowInUp") => html! {
      <BS_BoxArrowInUp ..owned_props />
    },
    implicit_clone::unsync::IString::Static("MenuDown") => html! {
      <BS_MenuDown ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Badge8k") => html! {
      <BS_Badge8k ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ExclamationDiamondFill") => html! {
      <BS_ExclamationDiamondFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Reddit") => html! {
      <BS_Reddit ..owned_props />
    },
    implicit_clone::unsync::IString::Static("PatchExclamationFill") => html! {
      <BS_PatchExclamationFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("EnvelopeHeartFill") => html! {
      <BS_EnvelopeHeartFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Earbuds") => html! {
      <BS_Earbuds ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Camera2") => html! {
      <BS_Camera2 ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Grid3x3") => html! {
      <BS_Grid3x3 ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Toggle2On") => html! {
      <BS_Toggle2On ..owned_props />
    },
    implicit_clone::unsync::IString::Static("SignIntersectionSide") => html! {
      <BS_SignIntersectionSide ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FiletypeMp3") => html! {
      <BS_FiletypeMp3 ..owned_props />
    },
    implicit_clone::unsync::IString::Static("MegaphoneFill") => html! {
      <BS_MegaphoneFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("PinMap") => html! {
      <BS_PinMap ..owned_props />
    },
    implicit_clone::unsync::IString::Static("HouseUpFill") => html! {
      <BS_HouseUpFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("CardList") => html! {
      <BS_CardList ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Motherboard") => html! {
      <BS_Motherboard ..owned_props />
    },
    implicit_clone::unsync::IString::Static("CreditCard") => html! {
      <BS_CreditCard ..owned_props />
    },
    implicit_clone::unsync::IString::Static("123") => html! {
      <BS_123 ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FiletypeXls") => html! {
      <BS_FiletypeXls ..owned_props />
    },
    implicit_clone::unsync::IString::Static("StopFill") => html! {
      <BS_StopFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Regex") => html! {
      <BS_Regex ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Trash3") => html! {
      <BS_Trash3 ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ArrowClockwise") => html! {
      <BS_ArrowClockwise ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Trophy") => html! {
      <BS_Trophy ..owned_props />
    },
    implicit_clone::unsync::IString::Static("MagnetFill") => html! {
      <BS_MagnetFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Cone") => html! {
      <BS_Cone ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ViewList") => html! {
      <BS_ViewList ..owned_props />
    },
    implicit_clone::unsync::IString::Static("TypeUnderline") => html! {
      <BS_TypeUnderline ..owned_props />
    },
    implicit_clone::unsync::IString::Static("InfoLg") => html! {
      <BS_InfoLg ..owned_props />
    },
    implicit_clone::unsync::IString::Static("CheckCircleFill") => html! {
      <BS_CheckCircleFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("SendFill") => html! {
      <BS_SendFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FileEarmarkTextFill") => html! {
      <BS_FileEarmarkTextFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Diagram2Fill") => html! {
      <BS_Diagram2Fill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Alipay") => html! {
      <BS_Alipay ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ArrowLeftCircle") => html! {
      <BS_ArrowLeftCircle ..owned_props />
    },
    implicit_clone::unsync::IString::Static("TicketDetailedFill") => html! {
      <BS_TicketDetailedFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FiletypeDocx") => html! {
      <BS_FiletypeDocx ..owned_props />
    },
    implicit_clone::unsync::IString::Static("PhoneLandscapeFill") => html! {
      <BS_PhoneLandscapeFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ChevronCompactUp") => html! {
      <BS_ChevronCompactUp ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Shield") => html! {
      <BS_Shield ..owned_props />
    },
    implicit_clone::unsync::IString::Static("CaretLeftSquareFill") => html! {
      <BS_CaretLeftSquareFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Wechat") => html! {
      <BS_Wechat ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ArrowThroughHeartFill") => html! {
      <BS_ArrowThroughHeartFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("QrCodeScan") => html! {
      <BS_QrCodeScan ..owned_props />
    },
    implicit_clone::unsync::IString::Static("SendCheckFill") => html! {
      <BS_SendCheckFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ChatRightFill") => html! {
      <BS_ChatRightFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Link45deg") => html! {
      <BS_Link45deg ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FileImage") => html! {
      <BS_FileImage ..owned_props />
    },
    implicit_clone::unsync::IString::Static("CheckCircle") => html! {
      <BS_CheckCircle ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FiletypeScss") => html! {
      <BS_FiletypeScss ..owned_props />
    },
    implicit_clone::unsync::IString::Static("HouseExclamation") => html! {
      <BS_HouseExclamation ..owned_props />
    },
    implicit_clone::unsync::IString::Static("EmojiKissFill") => html! {
      <BS_EmojiKissFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Stars") => html! {
      <BS_Stars ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ShieldFillMinus") => html! {
      <BS_ShieldFillMinus ..owned_props />
    },
    implicit_clone::unsync::IString::Static("CameraReelsFill") => html! {
      <BS_CameraReelsFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("MenuApp") => html! {
      <BS_MenuApp ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ArrowUpSquareFill") => html! {
      <BS_ArrowUpSquareFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Clipboard2PlusFill") => html! {
      <BS_Clipboard2PlusFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ChatRightQuote") => html! {
      <BS_ChatRightQuote ..owned_props />
    },
    implicit_clone::unsync::IString::Static("BrightnessHigh") => html! {
      <BS_BrightnessHigh ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Signpost") => html! {
      <BS_Signpost ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FiletypeXml") => html! {
      <BS_FiletypeXml ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ExclamationLg") => html! {
      <BS_ExclamationLg ..owned_props />
    },
    implicit_clone::unsync::IString::Static("PersonFillExclamation") => html! {
      <BS_PersonFillExclamation ..owned_props />
    },
    implicit_clone::unsync::IString::Static("VolumeOffFill") => html! {
      <BS_VolumeOffFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Send") => html! {
      <BS_Send ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FileBinary") => html! {
      <BS_FileBinary ..owned_props />
    },
    implicit_clone::unsync::IString::Static("HouseDownFill") => html! {
      <BS_HouseDownFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("SkipBackwardCircleFill") => html! {
      <BS_SkipBackwardCircleFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("SkipForwardBtnFill") => html! {
      <BS_SkipForwardBtnFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("HouseAddFill") => html! {
      <BS_HouseAddFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("CloudFill") => html! {
      <BS_CloudFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("8CircleFill") => html! {
      <BS_8CircleFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Clipboard2Pulse") => html! {
      <BS_Clipboard2Pulse ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Calendar3EventFill") => html! {
      <BS_Calendar3EventFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FileText") => html! {
      <BS_FileText ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Tools") => html! {
      <BS_Tools ..owned_props />
    },
    implicit_clone::unsync::IString::Static("CloudHaze") => html! {
      <BS_CloudHaze ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FiletypeXlsx") => html! {
      <BS_FiletypeXlsx ..owned_props />
    },
    implicit_clone::unsync::IString::Static("CodeSlash") => html! {
      <BS_CodeSlash ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Grid1x2Fill") => html! {
      <BS_Grid1x2Fill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Upload") => html! {
      <BS_Upload ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FileArrowDownFill") => html! {
      <BS_FileArrowDownFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("PSquare") => html! {
      <BS_PSquare ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ZoomOut") => html! {
      <BS_ZoomOut ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ArrowBarRight") => html! {
      <BS_ArrowBarRight ..owned_props />
    },
    implicit_clone::unsync::IString::Static("7Square") => html! {
      <BS_7Square ..owned_props />
    },
    implicit_clone::unsync::IString::Static("QuestionOctagonFill") => html! {
      <BS_QuestionOctagonFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("BuildingDown") => html! {
      <BS_BuildingDown ..owned_props />
    },
    implicit_clone::unsync::IString::Static("BlockquoteLeft") => html! {
      <BS_BlockquoteLeft ..owned_props />
    },
    implicit_clone::unsync::IString::Static("WindowSidebar") => html! {
      <BS_WindowSidebar ..owned_props />
    },
    implicit_clone::unsync::IString::Static("SignDoNotEnter") => html! {
      <BS_SignDoNotEnter ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FileEarmarkPerson") => html! {
      <BS_FileEarmarkPerson ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Displayport") => html! {
      <BS_Displayport ..owned_props />
    },
    implicit_clone::unsync::IString::Static("EnvelopePaperHeartFill") => html! {
      <BS_EnvelopePaperHeartFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Trash2Fill") => html! {
      <BS_Trash2Fill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("HddRackFill") => html! {
      <BS_HddRackFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ExclamationTriangleFill") => html! {
      <BS_ExclamationTriangleFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("JustifyRight") => html! {
      <BS_JustifyRight ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Airplane") => html! {
      <BS_Airplane ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Dice4") => html! {
      <BS_Dice4 ..owned_props />
    },
    implicit_clone::unsync::IString::Static("UsbSymbol") => html! {
      <BS_UsbSymbol ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FileBarGraph") => html! {
      <BS_FileBarGraph ..owned_props />
    },
    implicit_clone::unsync::IString::Static("TriangleFill") => html! {
      <BS_TriangleFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FileMusicFill") => html! {
      <BS_FileMusicFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Instagram") => html! {
      <BS_Instagram ..owned_props />
    },
    implicit_clone::unsync::IString::Static("DeviceHdd") => html! {
      <BS_DeviceHdd ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FunnelFill") => html! {
      <BS_FunnelFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FiletypeSass") => html! {
      <BS_FiletypeSass ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Escape") => html! {
      <BS_Escape ..owned_props />
    },
    implicit_clone::unsync::IString::Static("CurrencyDollar") => html! {
      <BS_CurrencyDollar ..owned_props />
    },
    implicit_clone::unsync::IString::Static("RSquareFill") => html! {
      <BS_RSquareFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("CloudSlashFill") => html! {
      <BS_CloudSlashFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("HandThumbsUp") => html! {
      <BS_HandThumbsUp ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Bricks") => html! {
      <BS_Bricks ..owned_props />
    },
    implicit_clone::unsync::IString::Static("CardHeading") => html! {
      <BS_CardHeading ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ClipboardCheckFill") => html! {
      <BS_ClipboardCheckFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Hdmi") => html! {
      <BS_Hdmi ..owned_props />
    },
    implicit_clone::unsync::IString::Static("PatchPlus") => html! {
      <BS_PatchPlus ..owned_props />
    },
    implicit_clone::unsync::IString::Static("DashCircleDotted") => html! {
      <BS_DashCircleDotted ..owned_props />
    },
    implicit_clone::unsync::IString::Static("WrenchAdjustable") => html! {
      <BS_WrenchAdjustable ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Mouse2Fill") => html! {
      <BS_Mouse2Fill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Wordpress") => html! {
      <BS_Wordpress ..owned_props />
    },
    implicit_clone::unsync::IString::Static("PaintBucket") => html! {
      <BS_PaintBucket ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ChatRight") => html! {
      <BS_ChatRight ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ShiftFill") => html! {
      <BS_ShiftFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("TelephoneForward") => html! {
      <BS_TelephoneForward ..owned_props />
    },
    implicit_clone::unsync::IString::Static("CassetteFill") => html! {
      <BS_CassetteFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Slash") => html! {
      <BS_Slash ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FiletypeBmp") => html! {
      <BS_FiletypeBmp ..owned_props />
    },
    implicit_clone::unsync::IString::Static("BuildingGear") => html! {
      <BS_BuildingGear ..owned_props />
    },
    implicit_clone::unsync::IString::Static("CloudsFill") => html! {
      <BS_CloudsFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("SignRailroadFill") => html! {
      <BS_SignRailroadFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Grid3x3GapFill") => html! {
      <BS_Grid3x3GapFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("KeyboardFill") => html! {
      <BS_KeyboardFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ArrowDownLeftCircleFill") => html! {
      <BS_ArrowDownLeftCircleFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("TerminalPlus") => html! {
      <BS_TerminalPlus ..owned_props />
    },
    implicit_clone::unsync::IString::Static("UiRadios") => html! {
      <BS_UiRadios ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ArrowUp") => html! {
      <BS_ArrowUp ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ArrowUpSquare") => html! {
      <BS_ArrowUpSquare ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Android") => html! {
      <BS_Android ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Water") => html! {
      <BS_Water ..owned_props />
    },
    implicit_clone::unsync::IString::Static("InputCursorText") => html! {
      <BS_InputCursorText ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FileEarmarkLockFill") => html! {
      <BS_FileEarmarkLockFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("HospitalFill") => html! {
      <BS_HospitalFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("SortUpAlt") => html! {
      <BS_SortUpAlt ..owned_props />
    },
    implicit_clone::unsync::IString::Static("SimFill") => html! {
      <BS_SimFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("CloudSun") => html! {
      <BS_CloudSun ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Fingerprint") => html! {
      <BS_Fingerprint ..owned_props />
    },
    implicit_clone::unsync::IString::Static("WifiOff") => html! {
      <BS_WifiOff ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ArrowDown") => html! {
      <BS_ArrowDown ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FileRuled") => html! {
      <BS_FileRuled ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Save2") => html! {
      <BS_Save2 ..owned_props />
    },
    implicit_clone::unsync::IString::Static("VolumeOff") => html! {
      <BS_VolumeOff ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ClipboardPulse") => html! {
      <BS_ClipboardPulse ..owned_props />
    },
    implicit_clone::unsync::IString::Static("QuestionLg") => html! {
      <BS_QuestionLg ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FileEarmarkMusic") => html! {
      <BS_FileEarmarkMusic ..owned_props />
    },
    implicit_clone::unsync::IString::Static("1Square") => html! {
      <BS_1Square ..owned_props />
    },
    implicit_clone::unsync::IString::Static("AlignBottom") => html! {
      <BS_AlignBottom ..owned_props />
    },
    implicit_clone::unsync::IString::Static("LungsFill") => html! {
      <BS_LungsFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Cassette") => html! {
      <BS_Cassette ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ArrowUpRightCircleFill") => html! {
      <BS_ArrowUpRightCircleFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FiletypeJsx") => html! {
      <BS_FiletypeJsx ..owned_props />
    },
    implicit_clone::unsync::IString::Static("TelephonePlusFill") => html! {
      <BS_TelephonePlusFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("MoonStars") => html! {
      <BS_MoonStars ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Diamond") => html! {
      <BS_Diamond ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Bank2") => html! {
      <BS_Bank2 ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ArrowDownRight") => html! {
      <BS_ArrowDownRight ..owned_props />
    },
    implicit_clone::unsync::IString::Static("PostcardHeartFill") => html! {
      <BS_PostcardHeartFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("EggFill") => html! {
      <BS_EggFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("RSquare") => html! {
      <BS_RSquare ..owned_props />
    },
    implicit_clone::unsync::IString::Static("RssFill") => html! {
      <BS_RssFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ShieldLockFill") => html! {
      <BS_ShieldLockFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("BadgeVo") => html! {
      <BS_BadgeVo ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Mastodon") => html! {
      <BS_Mastodon ..owned_props />
    },
    implicit_clone::unsync::IString::Static("TextareaResize") => html! {
      <BS_TextareaResize ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Battery") => html! {
      <BS_Battery ..owned_props />
    },
    implicit_clone::unsync::IString::Static("UpcScan") => html! {
      <BS_UpcScan ..owned_props />
    },
    implicit_clone::unsync::IString::Static("CreditCard2Back") => html! {
      <BS_CreditCard2Back ..owned_props />
    },
    implicit_clone::unsync::IString::Static("EmojiKiss") => html! {
      <BS_EmojiKiss ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Cloudy") => html! {
      <BS_Cloudy ..owned_props />
    },
    implicit_clone::unsync::IString::Static("VolumeDown") => html! {
      <BS_VolumeDown ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Easel3") => html! {
      <BS_Easel3 ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FileEarmarkPdfFill") => html! {
      <BS_FileEarmarkPdfFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("EnvelopePaperHeart") => html! {
      <BS_EnvelopePaperHeart ..owned_props />
    },
    implicit_clone::unsync::IString::Static("HandIndex") => html! {
      <BS_HandIndex ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FiletypeMdx") => html! {
      <BS_FiletypeMdx ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FuelPump") => html! {
      <BS_FuelPump ..owned_props />
    },
    implicit_clone::unsync::IString::Static("LampFill") => html! {
      <BS_LampFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("LightbulbFill") => html! {
      <BS_LightbulbFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("SendXFill") => html! {
      <BS_SendXFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("AirplaneFill") => html! {
      <BS_AirplaneFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Bell") => html! {
      <BS_Bell ..owned_props />
    },
    implicit_clone::unsync::IString::Static("BoxArrowUpLeft") => html! {
      <BS_BoxArrowUpLeft ..owned_props />
    },
    implicit_clone::unsync::IString::Static("SignMergeLeftFill") => html! {
      <BS_SignMergeLeftFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ArrowDownRightSquareFill") => html! {
      <BS_ArrowDownRightSquareFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ShieldFill") => html! {
      <BS_ShieldFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("BadgeVoFill") => html! {
      <BS_BadgeVoFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ShieldShaded") => html! {
      <BS_ShieldShaded ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Plug") => html! {
      <BS_Plug ..owned_props />
    },
    implicit_clone::unsync::IString::Static("SuitSpade") => html! {
      <BS_SuitSpade ..owned_props />
    },
    implicit_clone::unsync::IString::Static("TypeStrikethrough") => html! {
      <BS_TypeStrikethrough ..owned_props />
    },
    implicit_clone::unsync::IString::Static("CameraReels") => html! {
      <BS_CameraReels ..owned_props />
    },
    implicit_clone::unsync::IString::Static("SaveFill") => html! {
      <BS_SaveFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("CalendarHeartFill") => html! {
      <BS_CalendarHeartFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("PencilFill") => html! {
      <BS_PencilFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("CloudSleet") => html! {
      <BS_CloudSleet ..owned_props />
    },
    implicit_clone::unsync::IString::Static("TaxiFront") => html! {
      <BS_TaxiFront ..owned_props />
    },
    implicit_clone::unsync::IString::Static("CashCoin") => html! {
      <BS_CashCoin ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FolderFill") => html! {
      <BS_FolderFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("CollectionPlay") => html! {
      <BS_CollectionPlay ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Dropbox") => html! {
      <BS_Dropbox ..owned_props />
    },
    implicit_clone::unsync::IString::Static("SignNoParking") => html! {
      <BS_SignNoParking ..owned_props />
    },
    implicit_clone::unsync::IString::Static("BadgeTmFill") => html! {
      <BS_BadgeTmFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Record2Fill") => html! {
      <BS_Record2Fill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("CircleFill") => html! {
      <BS_CircleFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("BagDashFill") => html! {
      <BS_BagDashFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ArrowLeftSquare") => html! {
      <BS_ArrowLeftSquare ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Calculator") => html! {
      <BS_Calculator ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FileEarmark") => html! {
      <BS_FileEarmark ..owned_props />
    },
    implicit_clone::unsync::IString::Static("CalendarPlusFill") => html! {
      <BS_CalendarPlusFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("EmojiHeartEyesFill") => html! {
      <BS_EmojiHeartEyesFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("1SquareFill") => html! {
      <BS_1SquareFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("PlayCircle") => html! {
      <BS_PlayCircle ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Capsule") => html! {
      <BS_Capsule ..owned_props />
    },
    implicit_clone::unsync::IString::Static("DatabaseAdd") => html! {
      <BS_DatabaseAdd ..owned_props />
    },
    implicit_clone::unsync::IString::Static("LayoutWtf") => html! {
      <BS_LayoutWtf ..owned_props />
    },
    implicit_clone::unsync::IString::Static("PostageHeartFill") => html! {
      <BS_PostageHeartFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("DashSquareFill") => html! {
      <BS_DashSquareFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("AlignStart") => html! {
      <BS_AlignStart ..owned_props />
    },
    implicit_clone::unsync::IString::Static("EmojiSmileUpsideDownFill") => html! {
      <BS_EmojiSmileUpsideDownFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ChevronBarExpand") => html! {
      <BS_ChevronBarExpand ..owned_props />
    },
    implicit_clone::unsync::IString::Static("GeoAltFill") => html! {
      <BS_GeoAltFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("HouseDashFill") => html! {
      <BS_HouseDashFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("SendDashFill") => html! {
      <BS_SendDashFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Exclude") => html! {
      <BS_Exclude ..owned_props />
    },
    implicit_clone::unsync::IString::Static("EmojiExpressionlessFill") => html! {
      <BS_EmojiExpressionlessFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Calendar2Plus") => html! {
      <BS_Calendar2Plus ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Newspaper") => html! {
      <BS_Newspaper ..owned_props />
    },
    implicit_clone::unsync::IString::Static("CalendarWeek") => html! {
      <BS_CalendarWeek ..owned_props />
    },
    implicit_clone::unsync::IString::Static("UsbPlug") => html! {
      <BS_UsbPlug ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ExclamationOctagon") => html! {
      <BS_ExclamationOctagon ..owned_props />
    },
    implicit_clone::unsync::IString::Static("CashStack") => html! {
      <BS_CashStack ..owned_props />
    },
    implicit_clone::unsync::IString::Static("BoxArrowInUpLeft") => html! {
      <BS_BoxArrowInUpLeft ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ReplyAllFill") => html! {
      <BS_ReplyAllFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("PauseBtnFill") => html! {
      <BS_PauseBtnFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FileEarmarkSlides") => html! {
      <BS_FileEarmarkSlides ..owned_props />
    },
    implicit_clone::unsync::IString::Static("InfoSquareFill") => html! {
      <BS_InfoSquareFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Calendar2MonthFill") => html! {
      <BS_Calendar2MonthFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("House") => html! {
      <BS_House ..owned_props />
    },
    implicit_clone::unsync::IString::Static("JournalArrowUp") => html! {
      <BS_JournalArrowUp ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Reception2") => html! {
      <BS_Reception2 ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FiletypeCs") => html! {
      <BS_FiletypeCs ..owned_props />
    },
    implicit_clone::unsync::IString::Static("BuildingFillX") => html! {
      <BS_BuildingFillX ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Octagon") => html! {
      <BS_Octagon ..owned_props />
    },
    implicit_clone::unsync::IString::Static("CloudArrowDown") => html! {
      <BS_CloudArrowDown ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Cloud") => html! {
      <BS_Cloud ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ShieldSlashFill") => html! {
      <BS_ShieldSlashFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("GiftFill") => html! {
      <BS_GiftFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("DiscFill") => html! {
      <BS_DiscFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ColumnsGap") => html! {
      <BS_ColumnsGap ..owned_props />
    },
    implicit_clone::unsync::IString::Static("SignMergeRight") => html! {
      <BS_SignMergeRight ..owned_props />
    },
    implicit_clone::unsync::IString::Static("EmojiFrown") => html! {
      <BS_EmojiFrown ..owned_props />
    },
    implicit_clone::unsync::IString::Static("HouseDoorFill") => html! {
      <BS_HouseDoorFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("BellSlash") => html! {
      <BS_BellSlash ..owned_props />
    },
    implicit_clone::unsync::IString::Static("DatabaseLock") => html! {
      <BS_DatabaseLock ..owned_props />
    },
    implicit_clone::unsync::IString::Static("OpticalAudioFill") => html! {
      <BS_OpticalAudioFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("CloudLightningRain") => html! {
      <BS_CloudLightningRain ..owned_props />
    },
    implicit_clone::unsync::IString::Static("LayoutSidebarReverse") => html! {
      <BS_LayoutSidebarReverse ..owned_props />
    },
    implicit_clone::unsync::IString::Static("WindowDock") => html! {
      <BS_WindowDock ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FileEarmarkMedical") => html! {
      <BS_FileEarmarkMedical ..owned_props />
    },
    implicit_clone::unsync::IString::Static("CSquareFill") => html! {
      <BS_CSquareFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ExclamationCircleFill") => html! {
      <BS_ExclamationCircleFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("LayoutThreeColumns") => html! {
      <BS_LayoutThreeColumns ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FileEarmarkExcel") => html! {
      <BS_FileEarmarkExcel ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Quora") => html! {
      <BS_Quora ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FiletypeRb") => html! {
      <BS_FiletypeRb ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Balloon") => html! {
      <BS_Balloon ..owned_props />
    },
    implicit_clone::unsync::IString::Static("BatteryFull") => html! {
      <BS_BatteryFull ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Paperclip") => html! {
      <BS_Paperclip ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Clipboard") => html! {
      <BS_Clipboard ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ArrowDownUp") => html! {
      <BS_ArrowDownUp ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ChatLeftText") => html! {
      <BS_ChatLeftText ..owned_props />
    },
    implicit_clone::unsync::IString::Static("CapslockFill") => html! {
      <BS_CapslockFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Cart3") => html! {
      <BS_Cart3 ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Modem") => html! {
      <BS_Modem ..owned_props />
    },
    implicit_clone::unsync::IString::Static("MusicPlayer") => html! {
      <BS_MusicPlayer ..owned_props />
    },
    implicit_clone::unsync::IString::Static("SignTurnSlightLeftFill") => html! {
      <BS_SignTurnSlightLeftFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("AirplaneEnginesFill") => html! {
      <BS_AirplaneEnginesFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("BuildingSlash") => html! {
      <BS_BuildingSlash ..owned_props />
    },
    implicit_clone::unsync::IString::Static("EmojiLaughingFill") => html! {
      <BS_EmojiLaughingFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Projector") => html! {
      <BS_Projector ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FlagFill") => html! {
      <BS_FlagFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("BodyText") => html! {
      <BS_BodyText ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Magic") => html! {
      <BS_Magic ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Box") => html! {
      <BS_Box ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Indent") => html! {
      <BS_Indent ..owned_props />
    },
    implicit_clone::unsync::IString::Static("PeopleFill") => html! {
      <BS_PeopleFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FileEarmarkSlidesFill") => html! {
      <BS_FileEarmarkSlidesFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FileBarGraphFill") => html! {
      <BS_FileBarGraphFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Funnel") => html! {
      <BS_Funnel ..owned_props />
    },
    implicit_clone::unsync::IString::Static("HouseDash") => html! {
      <BS_HouseDash ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Stoplights") => html! {
      <BS_Stoplights ..owned_props />
    },
    implicit_clone::unsync::IString::Static("StopCircleFill") => html! {
      <BS_StopCircleFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("KeyFill") => html! {
      <BS_KeyFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("TrainLightrailFrontFill") => html! {
      <BS_TrainLightrailFrontFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Steam") => html! {
      <BS_Steam ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Bezier2") => html! {
      <BS_Bezier2 ..owned_props />
    },
    implicit_clone::unsync::IString::Static("CameraVideoFill") => html! {
      <BS_CameraVideoFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("GearFill") => html! {
      <BS_GearFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("CupHotFill") => html! {
      <BS_CupHotFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ListColumns") => html! {
      <BS_ListColumns ..owned_props />
    },
    implicit_clone::unsync::IString::Static("XDiamondFill") => html! {
      <BS_XDiamondFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("SortAlphaDown") => html! {
      <BS_SortAlphaDown ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FileLockFill") => html! {
      <BS_FileLockFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Justify") => html! {
      <BS_Justify ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Pip") => html! {
      <BS_Pip ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Layers") => html! {
      <BS_Layers ..owned_props />
    },
    implicit_clone::unsync::IString::Static("BuildingFillAdd") => html! {
      <BS_BuildingFillAdd ..owned_props />
    },
    implicit_clone::unsync::IString::Static("BoxArrowDownRight") => html! {
      <BS_BoxArrowDownRight ..owned_props />
    },
    implicit_clone::unsync::IString::Static("BorderCenter") => html! {
      <BS_BorderCenter ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Diagram3") => html! {
      <BS_Diagram3 ..owned_props />
    },
    implicit_clone::unsync::IString::Static("DeviceHddFill") => html! {
      <BS_DeviceHddFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("BrightnessAltLow") => html! {
      <BS_BrightnessAltLow ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Chat") => html! {
      <BS_Chat ..owned_props />
    },
    implicit_clone::unsync::IString::Static("TelephoneOutboundFill") => html! {
      <BS_TelephoneOutboundFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("CaretRightSquareFill") => html! {
      <BS_CaretRightSquareFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("BroadcastPin") => html! {
      <BS_BroadcastPin ..owned_props />
    },
    implicit_clone::unsync::IString::Static("XCircleFill") => html! {
      <BS_XCircleFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("SkipEndBtnFill") => html! {
      <BS_SkipEndBtnFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Lungs") => html! {
      <BS_Lungs ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Calendar2PlusFill") => html! {
      <BS_Calendar2PlusFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("EmojiSunglasses") => html! {
      <BS_EmojiSunglasses ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Clipboard2CheckFill") => html! {
      <BS_Clipboard2CheckFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Voicemail") => html! {
      <BS_Voicemail ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ArrowUpLeftSquareFill") => html! {
      <BS_ArrowUpLeftSquareFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("PatchExclamation") => html! {
      <BS_PatchExclamation ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Easel2Fill") => html! {
      <BS_Easel2Fill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FiletypeKey") => html! {
      <BS_FiletypeKey ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FiletypeSvg") => html! {
      <BS_FiletypeSvg ..owned_props />
    },
    implicit_clone::unsync::IString::Static("SearchHeartFill") => html! {
      <BS_SearchHeartFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FileEarmarkArrowUpFill") => html! {
      <BS_FileEarmarkArrowUpFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("CircleHalf") => html! {
      <BS_CircleHalf ..owned_props />
    },
    implicit_clone::unsync::IString::Static("AlignTop") => html! {
      <BS_AlignTop ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ArrowsCollapse") => html! {
      <BS_ArrowsCollapse ..owned_props />
    },
    implicit_clone::unsync::IString::Static("BuildingFillSlash") => html! {
      <BS_BuildingFillSlash ..owned_props />
    },
    implicit_clone::unsync::IString::Static("SlashLg") => html! {
      <BS_SlashLg ..owned_props />
    },
    implicit_clone::unsync::IString::Static("EnvelopeCheckFill") => html! {
      <BS_EnvelopeCheckFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FileEarmarkPlayFill") => html! {
      <BS_FileEarmarkPlayFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("BadgeAd") => html! {
      <BS_BadgeAd ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FileEarmarkPersonFill") => html! {
      <BS_FileEarmarkPersonFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("HouseGearFill") => html! {
      <BS_HouseGearFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Git") => html! {
      <BS_Git ..owned_props />
    },
    implicit_clone::unsync::IString::Static("BellFill") => html! {
      <BS_BellFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Activity") => html! {
      <BS_Activity ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ChatLeftDots") => html! {
      <BS_ChatLeftDots ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ShieldLock") => html! {
      <BS_ShieldLock ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Capslock") => html! {
      <BS_Capslock ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ArrowUpRightSquare") => html! {
      <BS_ArrowUpRightSquare ..owned_props />
    },
    implicit_clone::unsync::IString::Static("NodePlus") => html! {
      <BS_NodePlus ..owned_props />
    },
    implicit_clone::unsync::IString::Static("2Square") => html! {
      <BS_2Square ..owned_props />
    },
    implicit_clone::unsync::IString::Static("CaretRight") => html! {
      <BS_CaretRight ..owned_props />
    },
    implicit_clone::unsync::IString::Static("BuildingLock") => html! {
      <BS_BuildingLock ..owned_props />
    },
    implicit_clone::unsync::IString::Static("UsbFill") => html! {
      <BS_UsbFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("PostageHeart") => html! {
      <BS_PostageHeart ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Coin") => html! {
      <BS_Coin ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ChevronDoubleUp") => html! {
      <BS_ChevronDoubleUp ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Grid3x3Gap") => html! {
      <BS_Grid3x3Gap ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Clouds") => html! {
      <BS_Clouds ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Folder2Open") => html! {
      <BS_Folder2Open ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FilterSquare") => html! {
      <BS_FilterSquare ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Hexagon") => html! {
      <BS_Hexagon ..owned_props />
    },
    implicit_clone::unsync::IString::Static("HeptagonHalf") => html! {
      <BS_HeptagonHalf ..owned_props />
    },
    implicit_clone::unsync::IString::Static("DashCircleFill") => html! {
      <BS_DashCircleFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FileEasel") => html! {
      <BS_FileEasel ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Grid1x2") => html! {
      <BS_Grid1x2 ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Hdd") => html! {
      <BS_Hdd ..owned_props />
    },
    implicit_clone::unsync::IString::Static("QuestionOctagon") => html! {
      <BS_QuestionOctagon ..owned_props />
    },
    implicit_clone::unsync::IString::Static("SendDash") => html! {
      <BS_SendDash ..owned_props />
    },
    implicit_clone::unsync::IString::Static("BagPlusFill") => html! {
      <BS_BagPlusFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FileEarmarkBarGraph") => html! {
      <BS_FileEarmarkBarGraph ..owned_props />
    },
    implicit_clone::unsync::IString::Static("UiChecks") => html! {
      <BS_UiChecks ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FileSpreadsheet") => html! {
      <BS_FileSpreadsheet ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Bug") => html! {
      <BS_Bug ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ArrowRepeat") => html! {
      <BS_ArrowRepeat ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Dice5") => html! {
      <BS_Dice5 ..owned_props />
    },
    implicit_clone::unsync::IString::Static("3Circle") => html! {
      <BS_3Circle ..owned_props />
    },
    implicit_clone::unsync::IString::Static("PauseBtn") => html! {
      <BS_PauseBtn ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Images") => html! {
      <BS_Images ..owned_props />
    },
    implicit_clone::unsync::IString::Static("SignNoLeftTurnFill") => html! {
      <BS_SignNoLeftTurnFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("CalendarX") => html! {
      <BS_CalendarX ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FiletypePng") => html! {
      <BS_FiletypePng ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Clock") => html! {
      <BS_Clock ..owned_props />
    },
    implicit_clone::unsync::IString::Static("AirplaneEngines") => html! {
      <BS_AirplaneEngines ..owned_props />
    },
    implicit_clone::unsync::IString::Static("EmojiHeartEyes") => html! {
      <BS_EmojiHeartEyes ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Book") => html! {
      <BS_Book ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Sliders2Vertical") => html! {
      <BS_Sliders2Vertical ..owned_props />
    },
    implicit_clone::unsync::IString::Static("CreditCard2Front") => html! {
      <BS_CreditCard2Front ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FileEarmarkWord") => html! {
      <BS_FileEarmarkWord ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FiletypeCsv") => html! {
      <BS_FiletypeCsv ..owned_props />
    },
    implicit_clone::unsync::IString::Static("BarChartLineFill") => html! {
      <BS_BarChartLineFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Hurricane") => html! {
      <BS_Hurricane ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Envelope") => html! {
      <BS_Envelope ..owned_props />
    },
    implicit_clone::unsync::IString::Static("OctagonFill") => html! {
      <BS_OctagonFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("EnvelopeSlash") => html! {
      <BS_EnvelopeSlash ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FileEarmarkSpreadsheetFill") => html! {
      <BS_FileEarmarkSpreadsheetFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("BoxArrowDown") => html! {
      <BS_BoxArrowDown ..owned_props />
    },
    implicit_clone::unsync::IString::Static("BuildingFillUp") => html! {
      <BS_BuildingFillUp ..owned_props />
    },
    implicit_clone::unsync::IString::Static("CloudSlash") => html! {
      <BS_CloudSlash ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ClipboardXFill") => html! {
      <BS_ClipboardXFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ArrowLeftRight") => html! {
      <BS_ArrowLeftRight ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Square") => html! {
      <BS_Square ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FilterLeft") => html! {
      <BS_FilterLeft ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Ladder") => html! {
      <BS_Ladder ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FileCodeFill") => html! {
      <BS_FileCodeFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Badge4kFill") => html! {
      <BS_Badge4kFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Umbrella") => html! {
      <BS_Umbrella ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ArrowDownLeftCircle") => html! {
      <BS_ArrowDownLeftCircle ..owned_props />
    },
    implicit_clone::unsync::IString::Static("BracesAsterisk") => html! {
      <BS_BracesAsterisk ..owned_props />
    },
    implicit_clone::unsync::IString::Static("BrowserEdge") => html! {
      <BS_BrowserEdge ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Pen") => html! {
      <BS_Pen ..owned_props />
    },
    implicit_clone::unsync::IString::Static("BootstrapReboot") => html! {
      <BS_BootstrapReboot ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Smartwatch") => html! {
      <BS_Smartwatch ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Peace") => html! {
      <BS_Peace ..owned_props />
    },
    implicit_clone::unsync::IString::Static("PersonBadge") => html! {
      <BS_PersonBadge ..owned_props />
    },
    implicit_clone::unsync::IString::Static("XOctagonFill") => html! {
      <BS_XOctagonFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("CreditCard2BackFill") => html! {
      <BS_CreditCard2BackFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("BorderWidth") => html! {
      <BS_BorderWidth ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ToggleOff") => html! {
      <BS_ToggleOff ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Basket2Fill") => html! {
      <BS_Basket2Fill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("PersonXFill") => html! {
      <BS_PersonXFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("CloudDrizzleFill") => html! {
      <BS_CloudDrizzleFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Wallet2") => html! {
      <BS_Wallet2 ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Forward") => html! {
      <BS_Forward ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Android2") => html! {
      <BS_Android2 ..owned_props />
    },
    implicit_clone::unsync::IString::Static("HCircle") => html! {
      <BS_HCircle ..owned_props />
    },
    implicit_clone::unsync::IString::Static("CupFill") => html! {
      <BS_CupFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("CalendarRange") => html! {
      <BS_CalendarRange ..owned_props />
    },
    implicit_clone::unsync::IString::Static("SignpostSplitFill") => html! {
      <BS_SignpostSplitFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("EmojiNeutral") => html! {
      <BS_EmojiNeutral ..owned_props />
    },
    implicit_clone::unsync::IString::Static("DashSquareDotted") => html! {
      <BS_DashSquareDotted ..owned_props />
    },
    implicit_clone::unsync::IString::Static("PersonCircle") => html! {
      <BS_PersonCircle ..owned_props />
    },
    implicit_clone::unsync::IString::Static("SkipEnd") => html! {
      <BS_SkipEnd ..owned_props />
    },
    implicit_clone::unsync::IString::Static("EjectFill") => html! {
      <BS_EjectFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FolderCheck") => html! {
      <BS_FolderCheck ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FileBreakFill") => html! {
      <BS_FileBreakFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("SignTurnSlightRight") => html! {
      <BS_SignTurnSlightRight ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Vinyl") => html! {
      <BS_Vinyl ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Calendar2X") => html! {
      <BS_Calendar2X ..owned_props />
    },
    implicit_clone::unsync::IString::Static("CollectionFill") => html! {
      <BS_CollectionFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Moisture") => html! {
      <BS_Moisture ..owned_props />
    },
    implicit_clone::unsync::IString::Static("SlashSquare") => html! {
      <BS_SlashSquare ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Image") => html! {
      <BS_Image ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Clipboard2Heart") => html! {
      <BS_Clipboard2Heart ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Rainbow") => html! {
      <BS_Rainbow ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Valentine") => html! {
      <BS_Valentine ..owned_props />
    },
    implicit_clone::unsync::IString::Static("HouseLock") => html! {
      <BS_HouseLock ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ExclamationDiamond") => html! {
      <BS_ExclamationDiamond ..owned_props />
    },
    implicit_clone::unsync::IString::Static("MenuButton") => html! {
      <BS_MenuButton ..owned_props />
    },
    implicit_clone::unsync::IString::Static("2CircleFill") => html! {
      <BS_2CircleFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("EnvelopeXFill") => html! {
      <BS_EnvelopeXFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Backspace") => html! {
      <BS_Backspace ..owned_props />
    },
    implicit_clone::unsync::IString::Static("BadgeArFill") => html! {
      <BS_BadgeArFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ProjectorFill") => html! {
      <BS_ProjectorFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ArrowUpLeftSquare") => html! {
      <BS_ArrowUpLeftSquare ..owned_props />
    },
    implicit_clone::unsync::IString::Static("SkipBackwardBtn") => html! {
      <BS_SkipBackwardBtn ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FilePlusFill") => html! {
      <BS_FilePlusFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("AlignMiddle") => html! {
      <BS_AlignMiddle ..owned_props />
    },
    implicit_clone::unsync::IString::Static("PinAngle") => html! {
      <BS_PinAngle ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Calendar2Fill") => html! {
      <BS_Calendar2Fill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Sunrise") => html! {
      <BS_Sunrise ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Badge3d") => html! {
      <BS_Badge3d ..owned_props />
    },
    implicit_clone::unsync::IString::Static("BagFill") => html! {
      <BS_BagFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Safe") => html! {
      <BS_Safe ..owned_props />
    },
    implicit_clone::unsync::IString::Static("7Circle") => html! {
      <BS_7Circle ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Clipboard2MinusFill") => html! {
      <BS_Clipboard2MinusFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FileEarmarkCode") => html! {
      <BS_FileEarmarkCode ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FilePpt") => html! {
      <BS_FilePpt ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FiletypeJson") => html! {
      <BS_FiletypeJson ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Calendar2Week") => html! {
      <BS_Calendar2Week ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Unity") => html! {
      <BS_Unity ..owned_props />
    },
    implicit_clone::unsync::IString::Static("WindowFullscreen") => html! {
      <BS_WindowFullscreen ..owned_props />
    },
    implicit_clone::unsync::IString::Static("TrainFreightFront") => html! {
      <BS_TrainFreightFront ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FileEarmarkRichtextFill") => html! {
      <BS_FileEarmarkRichtextFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("BadgeTm") => html! {
      <BS_BadgeTm ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Back") => html! {
      <BS_Back ..owned_props />
    },
    implicit_clone::unsync::IString::Static("SkipStart") => html! {
      <BS_SkipStart ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ChatQuoteFill") => html! {
      <BS_ChatQuoteFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FastForwardCircleFill") => html! {
      <BS_FastForwardCircleFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("BucketFill") => html! {
      <BS_BucketFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("SdCardFill") => html! {
      <BS_SdCardFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Playstation") => html! {
      <BS_Playstation ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FileEarmarkMedicalFill") => html! {
      <BS_FileEarmarkMedicalFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("LockFill") => html! {
      <BS_LockFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Calendar2Heart") => html! {
      <BS_Calendar2Heart ..owned_props />
    },
    implicit_clone::unsync::IString::Static("XSquareFill") => html! {
      <BS_XSquareFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Pin") => html! {
      <BS_Pin ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Folder") => html! {
      <BS_Folder ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ArrowUpShort") => html! {
      <BS_ArrowUpShort ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Calendar2Minus") => html! {
      <BS_Calendar2Minus ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FileArrowUp") => html! {
      <BS_FileArrowUp ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Line") => html! {
      <BS_Line ..owned_props />
    },
    implicit_clone::unsync::IString::Static("HdmiFill") => html! {
      <BS_HdmiFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Tiktok") => html! {
      <BS_Tiktok ..owned_props />
    },
    implicit_clone::unsync::IString::Static("SquareFill") => html! {
      <BS_SquareFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Heptagon") => html! {
      <BS_Heptagon ..owned_props />
    },
    implicit_clone::unsync::IString::Static("QuestionSquareFill") => html! {
      <BS_QuestionSquareFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("DpadFill") => html! {
      <BS_DpadFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("BoxFill") => html! {
      <BS_BoxFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("UsbMiniFill") => html! {
      <BS_UsbMiniFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("CartDashFill") => html! {
      <BS_CartDashFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("PersonFillX") => html! {
      <BS_PersonFillX ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Trash") => html! {
      <BS_Trash ..owned_props />
    },
    implicit_clone::unsync::IString::Static("QuestionDiamond") => html! {
      <BS_QuestionDiamond ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Egg") => html! {
      <BS_Egg ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Headset") => html! {
      <BS_Headset ..owned_props />
    },
    implicit_clone::unsync::IString::Static("CurrencyEuro") => html! {
      <BS_CurrencyEuro ..owned_props />
    },
    implicit_clone::unsync::IString::Static("JournalCheck") => html! {
      <BS_JournalCheck ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Calendar2Month") => html! {
      <BS_Calendar2Month ..owned_props />
    },
    implicit_clone::unsync::IString::Static("BoxArrowUpRight") => html! {
      <BS_BoxArrowUpRight ..owned_props />
    },
    implicit_clone::unsync::IString::Static("SignTurnLeftFill") => html! {
      <BS_SignTurnLeftFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FileEarmarkFont") => html! {
      <BS_FileEarmarkFont ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ShieldSlash") => html! {
      <BS_ShieldSlash ..owned_props />
    },
    implicit_clone::unsync::IString::Static("SkipForwardCircle") => html! {
      <BS_SkipForwardCircle ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Watch") => html! {
      <BS_Watch ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ArrowUpLeft") => html! {
      <BS_ArrowUpLeft ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Calendar2") => html! {
      <BS_Calendar2 ..owned_props />
    },
    implicit_clone::unsync::IString::Static("EnvelopeFill") => html! {
      <BS_EnvelopeFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Skype") => html! {
      <BS_Skype ..owned_props />
    },
    implicit_clone::unsync::IString::Static("RCircle") => html! {
      <BS_RCircle ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Calendar2EventFill") => html! {
      <BS_Calendar2EventFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("PostageFill") => html! {
      <BS_PostageFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("UsbCFill") => html! {
      <BS_UsbCFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("SkipBackwardBtnFill") => html! {
      <BS_SkipBackwardBtnFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("RecordCircle") => html! {
      <BS_RecordCircle ..owned_props />
    },
    implicit_clone::unsync::IString::Static("BuildingFillDown") => html! {
      <BS_BuildingFillDown ..owned_props />
    },
    implicit_clone::unsync::IString::Static("GenderMale") => html! {
      <BS_GenderMale ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Hr") => html! {
      <BS_Hr ..owned_props />
    },
    implicit_clone::unsync::IString::Static("CloudArrowUpFill") => html! {
      <BS_CloudArrowUpFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Meta") => html! {
      <BS_Meta ..owned_props />
    },
    implicit_clone::unsync::IString::Static("PieChartFill") => html! {
      <BS_PieChartFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("RCircleFill") => html! {
      <BS_RCircleFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("TaxiFrontFill") => html! {
      <BS_TaxiFrontFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("StarFill") => html! {
      <BS_StarFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Calendar4") => html! {
      <BS_Calendar4 ..owned_props />
    },
    implicit_clone::unsync::IString::Static("SignIntersection") => html! {
      <BS_SignIntersection ..owned_props />
    },
    implicit_clone::unsync::IString::Static("CaretRightSquare") => html! {
      <BS_CaretRightSquare ..owned_props />
    },
    implicit_clone::unsync::IString::Static("PatchQuestionFill") => html! {
      <BS_PatchQuestionFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("X") => html! {
      <BS_X ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Record") => html! {
      <BS_Record ..owned_props />
    },
    implicit_clone::unsync::IString::Static("CloudMoonFill") => html! {
      <BS_CloudMoonFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("HddNetworkFill") => html! {
      <BS_HddNetworkFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("TrainFrontFill") => html! {
      <BS_TrainFrontFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ListCheck") => html! {
      <BS_ListCheck ..owned_props />
    },
    implicit_clone::unsync::IString::Static("CloudRain") => html! {
      <BS_CloudRain ..owned_props />
    },
    implicit_clone::unsync::IString::Static("BuildingExclamation") => html! {
      <BS_BuildingExclamation ..owned_props />
    },
    implicit_clone::unsync::IString::Static("PlayFill") => html! {
      <BS_PlayFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("SignDeadEndFill") => html! {
      <BS_SignDeadEndFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ShieldMinus") => html! {
      <BS_ShieldMinus ..owned_props />
    },
    implicit_clone::unsync::IString::Static("HddRack") => html! {
      <BS_HddRack ..owned_props />
    },
    implicit_clone::unsync::IString::Static("CloudArrowDownFill") => html! {
      <BS_CloudArrowDownFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("DatabaseX") => html! {
      <BS_DatabaseX ..owned_props />
    },
    implicit_clone::unsync::IString::Static("CloudPlusFill") => html! {
      <BS_CloudPlusFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("SortNumericUp") => html! {
      <BS_SortNumericUp ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Archive") => html! {
      <BS_Archive ..owned_props />
    },
    implicit_clone::unsync::IString::Static("BorderAll") => html! {
      <BS_BorderAll ..owned_props />
    },
    implicit_clone::unsync::IString::Static("CalendarDateFill") => html! {
      <BS_CalendarDateFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("PhoneFlip") => html! {
      <BS_PhoneFlip ..owned_props />
    },
    implicit_clone::unsync::IString::Static("PersonUp") => html! {
      <BS_PersonUp ..owned_props />
    },
    implicit_clone::unsync::IString::Static("CartXFill") => html! {
      <BS_CartXFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("PaletteFill") => html! {
      <BS_PaletteFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Snow") => html! {
      <BS_Snow ..owned_props />
    },
    implicit_clone::unsync::IString::Static("SuitDiamond") => html! {
      <BS_SuitDiamond ..owned_props />
    },
    implicit_clone::unsync::IString::Static("CalendarEvent") => html! {
      <BS_CalendarEvent ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Grid") => html! {
      <BS_Grid ..owned_props />
    },
    implicit_clone::unsync::IString::Static("PcHorizontal") => html! {
      <BS_PcHorizontal ..owned_props />
    },
    implicit_clone::unsync::IString::Static("XDiamond") => html! {
      <BS_XDiamond ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Speedometer2") => html! {
      <BS_Speedometer2 ..owned_props />
    },
    implicit_clone::unsync::IString::Static("SignIntersectionFill") => html! {
      <BS_SignIntersectionFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Hash") => html! {
      <BS_Hash ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ExclamationSquare") => html! {
      <BS_ExclamationSquare ..owned_props />
    },
    implicit_clone::unsync::IString::Static("EmojiSmile") => html! {
      <BS_EmojiSmile ..owned_props />
    },
    implicit_clone::unsync::IString::Static("SkipStartCircleFill") => html! {
      <BS_SkipStartCircleFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("DoorClosedFill") => html! {
      <BS_DoorClosedFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ChevronCompactRight") => html! {
      <BS_ChevronCompactRight ..owned_props />
    },
    implicit_clone::unsync::IString::Static("CurrencyPound") => html! {
      <BS_CurrencyPound ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FileFill") => html! {
      <BS_FileFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Boombox") => html! {
      <BS_Boombox ..owned_props />
    },
    implicit_clone::unsync::IString::Static("TypeItalic") => html! {
      <BS_TypeItalic ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Toggles2") => html! {
      <BS_Toggles2 ..owned_props />
    },
    implicit_clone::unsync::IString::Static("BriefcaseFill") => html! {
      <BS_BriefcaseFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ModemFill") => html! {
      <BS_ModemFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Calendar") => html! {
      <BS_Calendar ..owned_props />
    },
    implicit_clone::unsync::IString::Static("LayersHalf") => html! {
      <BS_LayersHalf ..owned_props />
    },
    implicit_clone::unsync::IString::Static("WalletFill") => html! {
      <BS_WalletFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("SendPlusFill") => html! {
      <BS_SendPlusFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("BorderRight") => html! {
      <BS_BorderRight ..owned_props />
    },
    implicit_clone::unsync::IString::Static("PersonFillAdd") => html! {
      <BS_PersonFillAdd ..owned_props />
    },
    implicit_clone::unsync::IString::Static("GridFill") => html! {
      <BS_GridFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("PencilSquare") => html! {
      <BS_PencilSquare ..owned_props />
    },
    implicit_clone::unsync::IString::Static("BadgeHd") => html! {
      <BS_BadgeHd ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ChatLeft") => html! {
      <BS_ChatLeft ..owned_props />
    },
    implicit_clone::unsync::IString::Static("8Circle") => html! {
      <BS_8Circle ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FileCheckFill") => html! {
      <BS_FileCheckFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Tags") => html! {
      <BS_Tags ..owned_props />
    },
    implicit_clone::unsync::IString::Static("RecordFill") => html! {
      <BS_RecordFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Cart4") => html! {
      <BS_Cart4 ..owned_props />
    },
    implicit_clone::unsync::IString::Static("CheckLg") => html! {
      <BS_CheckLg ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ChatFill") => html! {
      <BS_ChatFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("CalendarMinusFill") => html! {
      <BS_CalendarMinusFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ChevronBarContract") => html! {
      <BS_ChevronBarContract ..owned_props />
    },
    implicit_clone::unsync::IString::Static("AlarmFill") => html! {
      <BS_AlarmFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("CloudDownload") => html! {
      <BS_CloudDownload ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Usb") => html! {
      <BS_Usb ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Diagram3Fill") => html! {
      <BS_Diagram3Fill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("CcCircle") => html! {
      <BS_CcCircle ..owned_props />
    },
    implicit_clone::unsync::IString::Static("TextareaT") => html! {
      <BS_TextareaT ..owned_props />
    },
    implicit_clone::unsync::IString::Static("CalculatorFill") => html! {
      <BS_CalculatorFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("BusFront") => html! {
      <BS_BusFront ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Sun") => html! {
      <BS_Sun ..owned_props />
    },
    implicit_clone::unsync::IString::Static("QuestionCircle") => html! {
      <BS_QuestionCircle ..owned_props />
    },
    implicit_clone::unsync::IString::Static("CloudPlus") => html! {
      <BS_CloudPlus ..owned_props />
    },
    implicit_clone::unsync::IString::Static("SearchHeart") => html! {
      <BS_SearchHeart ..owned_props />
    },
    implicit_clone::unsync::IString::Static("HandIndexThumbFill") => html! {
      <BS_HandIndexThumbFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Inboxes") => html! {
      <BS_Inboxes ..owned_props />
    },
    implicit_clone::unsync::IString::Static("SquareHalf") => html! {
      <BS_SquareHalf ..owned_props />
    },
    implicit_clone::unsync::IString::Static("MenuButtonWideFill") => html! {
      <BS_MenuButtonWideFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("DoorOpen") => html! {
      <BS_DoorOpen ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Ticket") => html! {
      <BS_Ticket ..owned_props />
    },
    implicit_clone::unsync::IString::Static("CurrencyYen") => html! {
      <BS_CurrencyYen ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Stack") => html! {
      <BS_Stack ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Download") => html! {
      <BS_Download ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Clipboard2X") => html! {
      <BS_Clipboard2X ..owned_props />
    },
    implicit_clone::unsync::IString::Static("MarkdownFill") => html! {
      <BS_MarkdownFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ForwardFill") => html! {
      <BS_ForwardFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Binoculars") => html! {
      <BS_Binoculars ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Play") => html! {
      <BS_Play ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Clipboard2Check") => html! {
      <BS_Clipboard2Check ..owned_props />
    },
    implicit_clone::unsync::IString::Static("StopBtn") => html! {
      <BS_StopBtn ..owned_props />
    },
    implicit_clone::unsync::IString::Static("PlusSquare") => html! {
      <BS_PlusSquare ..owned_props />
    },
    implicit_clone::unsync::IString::Static("GpuCard") => html! {
      <BS_GpuCard ..owned_props />
    },
    implicit_clone::unsync::IString::Static("LightningFill") => html! {
      <BS_LightningFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ReplyAll") => html! {
      <BS_ReplyAll ..owned_props />
    },
    implicit_clone::unsync::IString::Static("UiChecksGrid") => html! {
      <BS_UiChecksGrid ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ClipboardDataFill") => html! {
      <BS_ClipboardDataFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("RecordBtn") => html! {
      <BS_RecordBtn ..owned_props />
    },
    implicit_clone::unsync::IString::Static("8Square") => html! {
      <BS_8Square ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ShareFill") => html! {
      <BS_ShareFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("SortNumericUpAlt") => html! {
      <BS_SortNumericUpAlt ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ChatSquareQuoteFill") => html! {
      <BS_ChatSquareQuoteFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("BusFrontFill") => html! {
      <BS_BusFrontFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Mouse2") => html! {
      <BS_Mouse2 ..owned_props />
    },
    implicit_clone::unsync::IString::Static("TicketPerforated") => html! {
      <BS_TicketPerforated ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ChatLeftHeart") => html! {
      <BS_ChatLeftHeart ..owned_props />
    },
    implicit_clone::unsync::IString::Static("RewindCircleFill") => html! {
      <BS_RewindCircleFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Calendar3Range") => html! {
      <BS_Calendar3Range ..owned_props />
    },
    implicit_clone::unsync::IString::Static("AppIndicator") => html! {
      <BS_AppIndicator ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Alarm") => html! {
      <BS_Alarm ..owned_props />
    },
    implicit_clone::unsync::IString::Static("CalendarFill") => html! {
      <BS_CalendarFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("WindowPlus") => html! {
      <BS_WindowPlus ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ChatSquare") => html! {
      <BS_ChatSquare ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Github") => html! {
      <BS_Github ..owned_props />
    },
    implicit_clone::unsync::IString::Static("HandThumbsDown") => html! {
      <BS_HandThumbsDown ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Bookmarks") => html! {
      <BS_Bookmarks ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Flag") => html! {
      <BS_Flag ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ShieldFillExclamation") => html! {
      <BS_ShieldFillExclamation ..owned_props />
    },
    implicit_clone::unsync::IString::Static("PiggyBank") => html! {
      <BS_PiggyBank ..owned_props />
    },
    implicit_clone::unsync::IString::Static("RecordBtnFill") => html! {
      <BS_RecordBtnFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("DeviceSsd") => html! {
      <BS_DeviceSsd ..owned_props />
    },
    implicit_clone::unsync::IString::Static("PersonFillGear") => html! {
      <BS_PersonFillGear ..owned_props />
    },
    implicit_clone::unsync::IString::Static("SignStopLights") => html! {
      <BS_SignStopLights ..owned_props />
    },
    implicit_clone::unsync::IString::Static("0Square") => html! {
      <BS_0Square ..owned_props />
    },
    implicit_clone::unsync::IString::Static("EnvelopeDash") => html! {
      <BS_EnvelopeDash ..owned_props />
    },
    implicit_clone::unsync::IString::Static("List") => html! {
      <BS_List ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ListColumnsReverse") => html! {
      <BS_ListColumnsReverse ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Command") => html! {
      <BS_Command ..owned_props />
    },
    implicit_clone::unsync::IString::Static("EmojiFrownFill") => html! {
      <BS_EmojiFrownFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ChevronBarRight") => html! {
      <BS_ChevronBarRight ..owned_props />
    },
    implicit_clone::unsync::IString::Static("HouseSlash") => html! {
      <BS_HouseSlash ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ClipboardFill") => html! {
      <BS_ClipboardFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("InputCursor") => html! {
      <BS_InputCursor ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ArrowsFullscreen") => html! {
      <BS_ArrowsFullscreen ..owned_props />
    },
    implicit_clone::unsync::IString::Static("SkipEndCircle") => html! {
      <BS_SkipEndCircle ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Cast") => html! {
      <BS_Cast ..owned_props />
    },
    implicit_clone::unsync::IString::Static("TencentQq") => html! {
      <BS_TencentQq ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Calendar2Date") => html! {
      <BS_Calendar2Date ..owned_props />
    },
    implicit_clone::unsync::IString::Static("CcSquareFill") => html! {
      <BS_CcSquareFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("TelephoneInbound") => html! {
      <BS_TelephoneInbound ..owned_props />
    },
    implicit_clone::unsync::IString::Static("HexagonHalf") => html! {
      <BS_HexagonHalf ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Megaphone") => html! {
      <BS_Megaphone ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ChatSquareFill") => html! {
      <BS_ChatSquareFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FiletypeYml") => html! {
      <BS_FiletypeYml ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Dice5Fill") => html! {
      <BS_Dice5Fill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("PatchCheckFill") => html! {
      <BS_PatchCheckFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("BoxSeam") => html! {
      <BS_BoxSeam ..owned_props />
    },
    implicit_clone::unsync::IString::Static("AlignEnd") => html! {
      <BS_AlignEnd ..owned_props />
    },
    implicit_clone::unsync::IString::Static("BadgeSd") => html! {
      <BS_BadgeSd ..owned_props />
    },
    implicit_clone::unsync::IString::Static("InboxesFill") => html! {
      <BS_InboxesFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("CameraVideoOff") => html! {
      <BS_CameraVideoOff ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ClipboardMinus") => html! {
      <BS_ClipboardMinus ..owned_props />
    },
    implicit_clone::unsync::IString::Static("CloudCheck") => html! {
      <BS_CloudCheck ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FastForward") => html! {
      <BS_FastForward ..owned_props />
    },
    implicit_clone::unsync::IString::Static("JournalCode") => html! {
      <BS_JournalCode ..owned_props />
    },
    implicit_clone::unsync::IString::Static("CcCircleFill") => html! {
      <BS_CcCircleFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("TelephoneXFill") => html! {
      <BS_TelephoneXFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Wallet") => html! {
      <BS_Wallet ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ChevronCompactDown") => html! {
      <BS_ChevronCompactDown ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Printer") => html! {
      <BS_Printer ..owned_props />
    },
    implicit_clone::unsync::IString::Static("PersonWorkspace") => html! {
      <BS_PersonWorkspace ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Calendar2DayFill") => html! {
      <BS_Calendar2DayFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("CarFrontFill") => html! {
      <BS_CarFrontFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Stickies") => html! {
      <BS_Stickies ..owned_props />
    },
    implicit_clone::unsync::IString::Static("TextWrap") => html! {
      <BS_TextWrap ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ThunderboltFill") => html! {
      <BS_ThunderboltFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("BoxArrowInDownRight") => html! {
      <BS_BoxArrowInDownRight ..owned_props />
    },
    implicit_clone::unsync::IString::Static("HouseX") => html! {
      <BS_HouseX ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FilterCircleFill") => html! {
      <BS_FilterCircleFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("JustifyLeft") => html! {
      <BS_JustifyLeft ..owned_props />
    },
    implicit_clone::unsync::IString::Static("WindowStack") => html! {
      <BS_WindowStack ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FastForwardBtnFill") => html! {
      <BS_FastForwardBtnFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FileBinaryFill") => html! {
      <BS_FileBinaryFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("SignNoRightTurn") => html! {
      <BS_SignNoRightTurn ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Badge8kFill") => html! {
      <BS_Badge8kFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Postcard") => html! {
      <BS_Postcard ..owned_props />
    },
    implicit_clone::unsync::IString::Static("5Square") => html! {
      <BS_5Square ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ListTask") => html! {
      <BS_ListTask ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ClipboardPlus") => html! {
      <BS_ClipboardPlus ..owned_props />
    },
    implicit_clone::unsync::IString::Static("JournalArrowDown") => html! {
      <BS_JournalArrowDown ..owned_props />
    },
    implicit_clone::unsync::IString::Static("EyeSlash") => html! {
      <BS_EyeSlash ..owned_props />
    },
    implicit_clone::unsync::IString::Static("CloudLightning") => html! {
      <BS_CloudLightning ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FileEarmarkPost") => html! {
      <BS_FileEarmarkPost ..owned_props />
    },
    implicit_clone::unsync::IString::Static("BagCheckFill") => html! {
      <BS_BagCheckFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Spotify") => html! {
      <BS_Spotify ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Superscript") => html! {
      <BS_Superscript ..owned_props />
    },
    implicit_clone::unsync::IString::Static("HeptagonFill") => html! {
      <BS_HeptagonFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("HSquareFill") => html! {
      <BS_HSquareFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("AlignCenter") => html! {
      <BS_AlignCenter ..owned_props />
    },
    implicit_clone::unsync::IString::Static("PatchMinus") => html! {
      <BS_PatchMinus ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Mask") => html! {
      <BS_Mask ..owned_props />
    },
    implicit_clone::unsync::IString::Static("CloudUpload") => html! {
      <BS_CloudUpload ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Snow3") => html! {
      <BS_Snow3 ..owned_props />
    },
    implicit_clone::unsync::IString::Static("MinecartLoaded") => html! {
      <BS_MinecartLoaded ..owned_props />
    },
    implicit_clone::unsync::IString::Static("CloudMinus") => html! {
      <BS_CloudMinus ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ThermometerHigh") => html! {
      <BS_ThermometerHigh ..owned_props />
    },
    implicit_clone::unsync::IString::Static("TerminalDash") => html! {
      <BS_TerminalDash ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ThreeDots") => html! {
      <BS_ThreeDots ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ArrowBarDown") => html! {
      <BS_ArrowBarDown ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Thunderbolt") => html! {
      <BS_Thunderbolt ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ThermometerSnow") => html! {
      <BS_ThermometerSnow ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Microsoft") => html! {
      <BS_Microsoft ..owned_props />
    },
    implicit_clone::unsync::IString::Static("TelephoneForwardFill") => html! {
      <BS_TelephoneForwardFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("CaretUpFill") => html! {
      <BS_CaretUpFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Lightning") => html! {
      <BS_Lightning ..owned_props />
    },
    implicit_clone::unsync::IString::Static("RewindCircle") => html! {
      <BS_RewindCircle ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Clipboard2") => html! {
      <BS_Clipboard2 ..owned_props />
    },
    implicit_clone::unsync::IString::Static("DatabaseFillUp") => html! {
      <BS_DatabaseFillUp ..owned_props />
    },
    implicit_clone::unsync::IString::Static("CupHot") => html! {
      <BS_CupHot ..owned_props />
    },
    implicit_clone::unsync::IString::Static("UsbMicro") => html! {
      <BS_UsbMicro ..owned_props />
    },
    implicit_clone::unsync::IString::Static("BuildingX") => html! {
      <BS_BuildingX ..owned_props />
    },
    implicit_clone::unsync::IString::Static("CloudLightningRainFill") => html! {
      <BS_CloudLightningRainFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Union") => html! {
      <BS_Union ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Signpost2") => html! {
      <BS_Signpost2 ..owned_props />
    },

    _ => html! {},
  }
}
