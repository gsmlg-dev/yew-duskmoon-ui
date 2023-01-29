use self::icon_props::IconProps;

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

pub use self::check2_circle::BS_Check2Circle;
pub use self::house_up::BS_HouseUp;
pub use self::arrow_down_left_square_fill::BS_ArrowDownLeftSquareFill;
pub use self::filetype_exe::BS_FiletypeExe;
pub use self::tornado::BS_Tornado;
pub use self::view_stacked::BS_ViewStacked;
pub use self::droplet_half::BS_DropletHalf;
pub use self::trash_fill::BS_TrashFill;
pub use self::node_plus_fill::BS_NodePlusFill;
pub use self::arrow_up_left_circle::BS_ArrowUpLeftCircle;
pub use self::pentagon_half::BS_PentagonHalf;
pub use self::database_fill::BS_DatabaseFill;
pub use self::file_earmark_arrow_up::BS_FileEarmarkArrowUp;
pub use self::buildings::BS_Buildings;
pub use self::5_circle::BS_5Circle;
pub use self::filetype_gif::BS_FiletypeGif;
pub use self::border_middle::BS_BorderMiddle;
pub use self::explicit_fill::BS_ExplicitFill;
pub use self::scooter::BS_Scooter;
pub use self::compass::BS_Compass;
pub use self::calendar_date::BS_CalendarDate;
pub use self::youtube::BS_Youtube;
pub use self::chat_left_quote_fill::BS_ChatLeftQuoteFill;
pub use self::cloud_fog2_fill::BS_CloudFog2Fill;
pub use self::7_circle_fill::BS_7CircleFill;
pub use self::plus_circle::BS_PlusCircle;
pub use self::caret_up_square::BS_CaretUpSquare;
pub use self::explicit::BS_Explicit;
pub use self::check2_square::BS_Check2Square;
pub use self::c_square::BS_CSquare;
pub use self::patch_check::BS_PatchCheck;
pub use self::caret_up::BS_CaretUp;
pub use self::bar_chart_steps::BS_BarChartSteps;
pub use self::card_checklist::BS_CardChecklist;
pub use self::badge_4k::BS_Badge4k;
pub use self::c_circle_fill::BS_CCircleFill;
pub use self::file_earmark_diff_fill::BS_FileEarmarkDiffFill;
pub use self::file_image_fill::BS_FileImageFill;
pub use self::house_slash_fill::BS_HouseSlashFill;
pub use self::ticket_perforated_fill::BS_TicketPerforatedFill;
pub use self::p_circle_fill::BS_PCircleFill;
pub use self::star_half::BS_StarHalf;
pub use self::cloud_hail_fill::BS_CloudHailFill;
pub use self::house_check::BS_HouseCheck;
pub use self::calendar_week_fill::BS_CalendarWeekFill;
pub use self::journal_medical::BS_JournalMedical;
pub use self::cloud_fog2::BS_CloudFog2;
pub use self::filetype_woff::BS_FiletypeWoff;
pub use self::bookmark_plus::BS_BookmarkPlus;
pub use self::volume_up_fill::BS_VolumeUpFill;
pub use self::file_earmark_easel::BS_FileEarmarkEasel;
pub use self::dribbble::BS_Dribbble;
pub use self::sunset_fill::BS_SunsetFill;
pub use self::person_fill_lock::BS_PersonFillLock;
pub use self::keyboard::BS_Keyboard;
pub use self::file_earmark_image_fill::BS_FileEarmarkImageFill;
pub use self::file_earmark_pdf::BS_FileEarmarkPdf;
pub use self::sort_alpha_down_alt::BS_SortAlphaDownAlt;
pub use self::chat_square_text_fill::BS_ChatSquareTextFill;
pub use self::easel2::BS_Easel2;
pub use self::inbox::BS_Inbox;
pub use self::arrow_up_right_circle::BS_ArrowUpRightCircle;
pub use self::filetype_txt::BS_FiletypeTxt;
pub use self::filetype_sql::BS_FiletypeSql;
pub use self::toggles::BS_Toggles;
pub use self::sort_alpha_up_alt::BS_SortAlphaUpAlt;
pub use self::trash3_fill::BS_Trash3Fill;
pub use self::balloon_heart::BS_BalloonHeart;
pub use self::clipboard_heart_fill::BS_ClipboardHeartFill;
pub use self::file_medical::BS_FileMedical;
pub use self::building_fill_gear::BS_BuildingFillGear;
pub use self::train_front::BS_TrainFront;
pub use self::houses_fill::BS_HousesFill;
pub use self::thermometer::BS_Thermometer;
pub use self::0_circle::BS_0Circle;
pub use self::file_earmark_binary::BS_FileEarmarkBinary;
pub use self::chat_left_text_fill::BS_ChatLeftTextFill;
pub use self::bicycle::BS_Bicycle;
pub use self::translate::BS_Translate;
pub use self::file_earmark_plus::BS_FileEarmarkPlus;
pub use self::bookmark_fill::BS_BookmarkFill;
pub use self::chevron_left::BS_ChevronLeft;
pub use self::eraser_fill::BS_EraserFill;
pub use self::x_octagon::BS_XOctagon;
pub use self::tag_fill::BS_TagFill;
pub use self::file_earmark_play::BS_FileEarmarkPlay;
pub use self::trash2::BS_Trash2;
pub use self::hourglass::BS_Hourglass;
pub use self::question::BS_Question;
pub use self::life_preserver::BS_LifePreserver;
pub use self::bookmark_star::BS_BookmarkStar;
pub use self::cone_striped::BS_ConeStriped;
pub use self::question_circle_fill::BS_QuestionCircleFill;
pub use self::exclamation_octagon_fill::BS_ExclamationOctagonFill;
pub use self::dash_lg::BS_DashLg;
pub use self::filetype_jpg::BS_FiletypeJpg;
pub use self::dice_2::BS_Dice2;
pub use self::sign_merge_right_fill::BS_SignMergeRightFill;
pub use self::telephone::BS_Telephone;
pub use self::hammer::BS_Hammer;
pub use self::twitch::BS_Twitch;
pub use self::file_slides::BS_FileSlides;
pub use self::eraser::BS_Eraser;
pub use self::volume_mute_fill::BS_VolumeMuteFill;
pub use self::house_check_fill::BS_HouseCheckFill;
pub use self::filetype_otf::BS_FiletypeOtf;
pub use self::credit_card_2_front_fill::BS_CreditCard2FrontFill;
pub use self::tv::BS_Tv;
pub use self::file_earmark_excel_fill::BS_FileEarmarkExcelFill;
pub use self::emoji_smile_upside_down::BS_EmojiSmileUpsideDown;
pub use self::type_h2::BS_TypeH2;
pub use self::p_square_fill::BS_PSquareFill;
pub use self::layer_forward::BS_LayerForward;
pub use self::cursor::BS_Cursor;
pub use self::folder_minus::BS_FolderMinus;
pub use self::eyedropper::BS_Eyedropper;
pub use self::ticket_fill::BS_TicketFill;
pub use self::stopwatch_fill::BS_StopwatchFill;
pub use self::cup::BS_Cup;
pub use self::nvidia::BS_Nvidia;
pub use self::bookmark_heart_fill::BS_BookmarkHeartFill;
pub use self::ticket_detailed::BS_TicketDetailed;
pub use self::paypal::BS_Paypal;
pub use self::grid_3x2::BS_Grid3x2;
pub use self::c_circle::BS_CCircle;
pub use self::mortarboard::BS_Mortarboard;
pub use self::chat_square_heart::BS_ChatSquareHeart;
pub use self::terminal::BS_Terminal;
pub use self::folder_symlink::BS_FolderSymlink;
pub use self::person_heart::BS_PersonHeart;
pub use self::arrow_right_short::BS_ArrowRightShort;
pub use self::journal_plus::BS_JournalPlus;
pub use self::file_x::BS_FileX;
pub use self::pass::BS_Pass;
pub use self::envelope_plus::BS_EnvelopePlus;
pub use self::wifi_1::BS_Wifi1;
pub use self::files_alt::BS_FilesAlt;
pub use self::truck_flatbed::BS_TruckFlatbed;
pub use self::chat_left_quote::BS_ChatLeftQuote;
pub use self::house_gear::BS_HouseGear;
pub use self::folder_symlink_fill::BS_FolderSymlinkFill;
pub use self::diagram_2::BS_Diagram2;
pub use self::filetype_psd::BS_FiletypePsd;
pub use self::person_plus::BS_PersonPlus;
pub use self::emoji_smile_fill::BS_EmojiSmileFill;
pub use self::rewind_btn::BS_RewindBtn;
pub use self::chevron_expand::BS_ChevronExpand;
pub use self::window_dash::BS_WindowDash;
pub use self::cloud_rain_heavy::BS_CloudRainHeavy;
pub use self::skip_end_btn::BS_SkipEndBtn;
pub use self::file_font_fill::BS_FileFontFill;
pub use self::prescription::BS_Prescription;
pub use self::handbag::BS_Handbag;
pub use self::ear::BS_Ear;
pub use self::xbox::BS_Xbox;
pub use self::plugin::BS_Plugin;
pub use self::brightness_alt_low_fill::BS_BrightnessAltLowFill;
pub use self::chat_right_heart_fill::BS_ChatRightHeartFill;
pub use self::google_play::BS_GooglePlay;
pub use self::balloon_fill::BS_BalloonFill;
pub use self::file_earmark_arrow_down_fill::BS_FileEarmarkArrowDownFill;
pub use self::arrow_90deg_up::BS_Arrow90degUp;
pub use self::mic::BS_Mic;
pub use self::1_circle_fill::BS_1CircleFill;
pub use self::brightness_low::BS_BrightnessLow;
pub use self::clipboard2_minus::BS_Clipboard2Minus;
pub use self::aspect_ratio_fill::BS_AspectRatioFill;
pub use self::sign_turn_slight_right_fill::BS_SignTurnSlightRightFill;
pub use self::window_x::BS_WindowX;
pub use self::chat_left_dots_fill::BS_ChatLeftDotsFill;
pub use self::sign_dead_end::BS_SignDeadEnd;
pub use self::chevron_up::BS_ChevronUp;
pub use self::bookmark::BS_Bookmark;
pub use self::file_earmark_code_fill::BS_FileEarmarkCodeFill;
pub use self::subscript::BS_Subscript;
pub use self::fan::BS_Fan;
pub use self::dice_1::BS_Dice1;
pub use self::sign_stop_lights_fill::BS_SignStopLightsFill;
pub use self::arrow_down_left_square::BS_ArrowDownLeftSquare;
pub use self::arrow_up_left_circle_fill::BS_ArrowUpLeftCircleFill;
pub use self::database_fill_exclamation::BS_DatabaseFillExclamation;
pub use self::speedometer::BS_Speedometer;
pub use self::chat_heart_fill::BS_ChatHeartFill;
pub use self::heart_half::BS_HeartHalf;
pub use self::shield_fill_check::BS_ShieldFillCheck;
pub use self::calendar_day::BS_CalendarDay;
pub use self::hourglass_top::BS_HourglassTop;
pub use self::backspace_reverse_fill::BS_BackspaceReverseFill;
pub use self::brush::BS_Brush;
pub use self::file_earmark_ppt::BS_FileEarmarkPpt;
pub use self::yelp::BS_Yelp;
pub use self::stop::BS_Stop;
pub use self::emoji_expressionless::BS_EmojiExpressionless;
pub use self::skip_forward_btn::BS_SkipForwardBtn;
pub use self::columns::BS_Columns;
pub use self::info_circle_fill::BS_InfoCircleFill;
pub use self::6_circle_fill::BS_6CircleFill;
pub use self::qr_code::BS_QrCode;
pub use self::diamond_fill::BS_DiamondFill;
pub use self::cloud_rain_heavy_fill::BS_CloudRainHeavyFill;
pub use self::cursor_text::BS_CursorText;
pub use self::archive_fill::BS_ArchiveFill;
pub use self::chevron_bar_up::BS_ChevronBarUp;
pub use self::caret_right_fill::BS_CaretRightFill;
pub use self::fast_forward_circle::BS_FastForwardCircle;
pub use self::map::BS_Map;
pub use self::cart_fill::BS_CartFill;
pub use self::cart_check_fill::BS_CartCheckFill;
pub use self::file_play_fill::BS_FilePlayFill;
pub use self::layout_sidebar_inset::BS_LayoutSidebarInset;
pub use self::cc_square::BS_CcSquare;
pub use self::sd_card::BS_SdCard;
pub use self::emoji_dizzy::BS_EmojiDizzy;
pub use self::wifi::BS_Wifi;
pub use self::thermometer_low::BS_ThermometerLow;
pub use self::file_lock2_fill::BS_FileLock2Fill;
pub use self::bag_plus::BS_BagPlus;
pub use self::camera_video::BS_CameraVideo;
pub use self::box_arrow_left::BS_BoxArrowLeft;
pub use self::bootstrap_fill::BS_BootstrapFill;
pub use self::sunset::BS_Sunset;
pub use self::filter_circle::BS_FilterCircle;
pub use self::capsule_pill::BS_CapsulePill;
pub use self::send_slash_fill::BS_SendSlashFill;
pub use self::building_fill::BS_BuildingFill;
pub use self::files::BS_Files;
pub use self::bag_check::BS_BagCheck;
pub use self::journal_bookmark_fill::BS_JournalBookmarkFill;
pub use self::cloud_snow_fill::BS_CloudSnowFill;
pub use self::octagon_half::BS_OctagonHalf;
pub use self::terminal_split::BS_TerminalSplit;
pub use self::calendar_minus::BS_CalendarMinus;
pub use self::arrow_left_square_fill::BS_ArrowLeftSquareFill;
pub use self::chevron_contract::BS_ChevronContract;
pub use self::router::BS_Router;
pub use self::calendar2_day::BS_Calendar2Day;
pub use self::person_plus_fill::BS_PersonPlusFill;
pub use self::signpost_fill::BS_SignpostFill;
pub use self::disc::BS_Disc;
pub use self::grid_3x2_gap_fill::BS_Grid3x2GapFill;
pub use self::3_square::BS_3Square;
pub use self::basket3_fill::BS_Basket3Fill;
pub use self::arrow_up_circle::BS_ArrowUpCircle;
pub use self::globe_europe_africa::BS_GlobeEuropeAfrica;
pub use self::file_excel_fill::BS_FileExcelFill;
pub use self::discord::BS_Discord;
pub use self::prescription2::BS_Prescription2;
pub use self::arrow_return_left::BS_ArrowReturnLeft;
pub use self::cart::BS_Cart;
pub use self::calendar_day_fill::BS_CalendarDayFill;
pub use self::3_circle_fill::BS_3CircleFill;
pub use self::music_note_beamed::BS_MusicNoteBeamed;
pub use self::arrow_down_circle::BS_ArrowDownCircle;
pub use self::cloudy_fill::BS_CloudyFill;
pub use self::display_fill::BS_DisplayFill;
pub use self::file_person_fill::BS_FilePersonFill;
pub use self::clipboard_data::BS_ClipboardData;
pub use self::rocket_takeoff::BS_RocketTakeoff;
pub use self::person_fill_check::BS_PersonFillCheck;
pub use self::headset_vr::BS_HeadsetVr;
pub use self::bookmarks_fill::BS_BookmarksFill;
pub use self::database_slash::BS_DatabaseSlash;
pub use self::distribute_vertical::BS_DistributeVertical;
pub use self::bar_chart_fill::BS_BarChartFill;
pub use self::suit_heart::BS_SuitHeart;
pub use self::h_circle_fill::BS_HCircleFill;
pub use self::server::BS_Server;
pub use self::distribute_horizontal::BS_DistributeHorizontal;
pub use self::sign_intersection_y::BS_SignIntersectionY;
pub use self::building_check::BS_BuildingCheck;
pub use self::backspace_reverse::BS_BackspaceReverse;
pub use self::list_ul::BS_ListUl;
pub use self::sort_down::BS_SortDown;
pub use self::file_medical_fill::BS_FileMedicalFill;
pub use self::file_earmark_minus_fill::BS_FileEarmarkMinusFill;
pub use self::shield_fill_x::BS_ShieldFillX;
pub use self::chat_right_heart::BS_ChatRightHeart;
pub use self::arrow_down_square_fill::BS_ArrowDownSquareFill;
pub use self::paragraph::BS_Paragraph;
pub use self::terminal_x::BS_TerminalX;
pub use self::chat_left_fill::BS_ChatLeftFill;
pub use self::soundwave::BS_Soundwave;
pub use self::envelope_at_fill::BS_EnvelopeAtFill;
pub use self::arrow_90deg_down::BS_Arrow90degDown;
pub use self::file_ruled_fill::BS_FileRuledFill;
pub use self::exclamation_triangle::BS_ExclamationTriangle;
pub use self::universal_access::BS_UniversalAccess;
pub use self::send_check::BS_SendCheck;
pub use self::hypnotize::BS_Hypnotize;
pub use self::patch_plus_fill::BS_PatchPlusFill;
pub use self::cart_dash::BS_CartDash;
pub use self::stack_overflow::BS_StackOverflow;
pub use self::bell_slash_fill::BS_BellSlashFill;
pub use self::file_earmark_arrow_down::BS_FileEarmarkArrowDown;
pub use self::layout_text_sidebar::BS_LayoutTextSidebar;
pub use self::hand_index_thumb::BS_HandIndexThumb;
pub use self::stoplights_fill::BS_StoplightsFill;
pub use self::minecart::BS_Minecart;
pub use self::border_top::BS_BorderTop;
pub use self::fullscreen::BS_Fullscreen;
pub use self::file_lock::BS_FileLock;
pub use self::shield_check::BS_ShieldCheck;
pub use self::chat_left_heart_fill::BS_ChatLeftHeartFill;
pub use self::person_down::BS_PersonDown;
pub use self::dash::BS_Dash;
pub use self::reception_4::BS_Reception4;
pub use self::speaker::BS_Speaker;
pub use self::clipboard_x::BS_ClipboardX;
pub use self::bookmark_check_fill::BS_BookmarkCheckFill;
pub use self::binoculars_fill::BS_BinocularsFill;
pub use self::train_lightrail_front::BS_TrainLightrailFront;
pub use self::tree_fill::BS_TreeFill;
pub use self::arrow_through_heart::BS_ArrowThroughHeart;
pub use self::file_person::BS_FilePerson;
pub use self::house_lock_fill::BS_HouseLockFill;
pub use self::file_earmark_ppt_fill::BS_FileEarmarkPptFill;
pub use self::tree::BS_Tree;
pub use self::pinterest::BS_Pinterest;
pub use self::gem::BS_Gem;
pub use self::clipboard2_pulse_fill::BS_Clipboard2PulseFill;
pub use self::dice_2_fill::BS_Dice2Fill;
pub use self::cart_check::BS_CartCheck;
pub use self::fuel_pump_diesel_fill::BS_FuelPumpDieselFill;
pub use self::three_dots_vertical::BS_ThreeDotsVertical;
pub use self::fuel_pump_diesel::BS_FuelPumpDiesel;
pub use self::x_square::BS_XSquare;
pub use self::person_rolodex::BS_PersonRolodex;
pub use self::calendar2_minus_fill::BS_Calendar2MinusFill;
pub use self::globe_americas::BS_GlobeAmericas;
pub use self::door_closed::BS_DoorClosed;
pub use self::vinyl_fill::BS_VinylFill;
pub use self::list_stars::BS_ListStars;
pub use self::play_circle_fill::BS_PlayCircleFill;
pub use self::send_exclamation::BS_SendExclamation;
pub use self::bezier::BS_Bezier;
pub use self::pause_circle_fill::BS_PauseCircleFill;
pub use self::layout_text_window::BS_LayoutTextWindow;
pub use self::pin_angle_fill::BS_PinAngleFill;
pub use self::speaker_fill::BS_SpeakerFill;
pub use self::slack::BS_Slack;
pub use self::basket::BS_Basket;
pub use self::bookmark_x::BS_BookmarkX;
pub use self::battery_half::BS_BatteryHalf;
pub use self::calendar_event_fill::BS_CalendarEventFill;
pub use self::envelope_slash_fill::BS_EnvelopeSlashFill;
pub use self::puzzle_fill::BS_PuzzleFill;
pub use self::calendar_check::BS_CalendarCheck;
pub use self::image_alt::BS_ImageAlt;
pub use self::filetype_py::BS_FiletypePy;
pub use self::file_post_fill::BS_FilePostFill;
pub use self::braces::BS_Braces;
pub use self::skip_end_circle_fill::BS_SkipEndCircleFill;
pub use self::lightbulb_off_fill::BS_LightbulbOffFill;
pub use self::browser_chrome::BS_BrowserChrome;
pub use self::plus::BS_Plus;
pub use self::gender_ambiguous::BS_GenderAmbiguous;
pub use self::sliders::BS_Sliders;
pub use self::whatsapp::BS_Whatsapp;
pub use self::brightness_low_fill::BS_BrightnessLowFill;
pub use self::skip_backward_fill::BS_SkipBackwardFill;
pub use self::person_x::BS_PersonX;
pub use self::volume_up::BS_VolumeUp;
pub use self::laptop::BS_Laptop;
pub use self::apple::BS_Apple;
pub use self::person_dash_fill::BS_PersonDashFill;
pub use self::unindent::BS_Unindent;
pub use self::moon_fill::BS_MoonFill;
pub use self::8_square_fill::BS_8SquareFill;
pub use self::house_heart_fill::BS_HouseHeartFill;
pub use self::file_arrow_up_fill::BS_FileArrowUpFill;
pub use self::snow2::BS_Snow2;
pub use self::at::BS_At;
pub use self::check_square::BS_CheckSquare;
pub use self::file_text_fill::BS_FileTextFill;
pub use self::file_earmark_lock::BS_FileEarmarkLock;
pub use self::filetype_m4p::BS_FiletypeM4p;
pub use self::send_exclamation_fill::BS_SendExclamationFill;
pub use self::telephone_minus::BS_TelephoneMinus;
pub use self::google::BS_Google;
pub use self::journal_richtext::BS_JournalRichtext;
pub use self::0_circle_fill::BS_0CircleFill;
pub use self::arrow_left_circle_fill::BS_ArrowLeftCircleFill;
pub use self::stop_btn_fill::BS_StopBtnFill;
pub use self::moon::BS_Moon;
pub use self::tablet_landscape_fill::BS_TabletLandscapeFill;
pub use self::reply::BS_Reply;
pub use self::question_square::BS_QuestionSquare;
pub use self::filetype_ai::BS_FiletypeAi;
pub use self::truck_front::BS_TruckFront;
pub use self::sign_intersection_t::BS_SignIntersectionT;
pub use self::clock_history::BS_ClockHistory;
pub use self::triangle_half::BS_TriangleHalf;
pub use self::sunglasses::BS_Sunglasses;
pub use self::type_h3::BS_TypeH3;
pub use self::6_circle::BS_6Circle;
pub use self::pentagon::BS_Pentagon;
pub use self::file_earmark_x::BS_FileEarmarkX;
pub use self::stickies_fill::BS_StickiesFill;
pub use self::nintendo_switch::BS_NintendoSwitch;
pub use self::collection_play_fill::BS_CollectionPlayFill;
pub use self::journal_bookmark::BS_JournalBookmark;
pub use self::sticky_fill::BS_StickyFill;
pub use self::currency_bitcoin::BS_CurrencyBitcoin;
pub use self::backspace_fill::BS_BackspaceFill;
pub use self::text_left::BS_TextLeft;
pub use self::headphones::BS_Headphones;
pub use self::geo_fill::BS_GeoFill;
pub use self::cloud_minus_fill::BS_CloudMinusFill;
pub use self::file_word::BS_FileWord;
pub use self::file_diff_fill::BS_FileDiffFill;
pub use self::lightbulb_off::BS_LightbulbOff;
pub use self::box2_heart::BS_Box2Heart;
pub use self::envelope_open_fill::BS_EnvelopeOpenFill;
pub use self::arrow_left_short::BS_ArrowLeftShort;
pub use self::postcard_heart::BS_PostcardHeart;
pub use self::journals::BS_Journals;
pub use self::link::BS_Link;
pub use self::text_indent_left::BS_TextIndentLeft;
pub use self::book_half::BS_BookHalf;
pub use self::arrow_up_right_square_fill::BS_ArrowUpRightSquareFill;
pub use self::file_spreadsheet_fill::BS_FileSpreadsheetFill;
pub use self::ui_radios_grid::BS_UiRadiosGrid;
pub use self::bank::BS_Bank;
pub use self::filetype_sh::BS_FiletypeSh;
pub use self::bounding_box::BS_BoundingBox;
pub use self::camera_fill::BS_CameraFill;
pub use self::memory::BS_Memory;
pub use self::quote::BS_Quote;
pub use self::file_post::BS_FilePost;
pub use self::calendar3_range_fill::BS_Calendar3RangeFill;
pub use self::heart_pulse::BS_HeartPulse;
pub use self::skip_start_circle::BS_SkipStartCircle;
pub use self::amd::BS_Amd;
pub use self::text_center::BS_TextCenter;
pub use self::list_ol::BS_ListOl;
pub use self::sign_do_not_enter_fill::BS_SignDoNotEnterFill;
pub use self::layout_text_sidebar_reverse::BS_LayoutTextSidebarReverse;
pub use self::person_vcard_fill::BS_PersonVcardFill;
pub use self::diamond_half::BS_DiamondHalf;
pub use self::reception_3::BS_Reception3;
pub use self::sort_down_alt::BS_SortDownAlt;
pub use self::wifi_2::BS_Wifi2;
pub use self::eye::BS_Eye;
pub use self::calendar_x_fill::BS_CalendarXFill;
pub use self::incognito::BS_Incognito;
pub use self::arrow_up_circle_fill::BS_ArrowUpCircleFill;
pub use self::hdd_stack::BS_HddStack;
pub use self::check::BS_Check;
pub use self::info_square::BS_InfoSquare;
pub use self::file_earmark_break::BS_FileEarmarkBreak;
pub use self::infinity::BS_Infinity;
pub use self::building_up::BS_BuildingUp;
pub use self::journal_x::BS_JournalX;
pub use self::calendar_plus::BS_CalendarPlus;
pub use self::exclamation_circle::BS_ExclamationCircle;
pub use self::database_fill_lock::BS_DatabaseFillLock;
pub use self::calendar2_date_fill::BS_Calendar2DateFill;
pub use self::palette::BS_Palette;
pub use self::window::BS_Window;
pub use self::bookmark_heart::BS_BookmarkHeart;
pub use self::asterisk::BS_Asterisk;
pub use self::basket2::BS_Basket2;
pub use self::mic_mute_fill::BS_MicMuteFill;
pub use self::browser_safari::BS_BrowserSafari;
pub use self::chat_right_dots_fill::BS_ChatRightDotsFill;
pub use self::clipboard_minus_fill::BS_ClipboardMinusFill;
pub use self::circle::BS_Circle;
pub use self::easel3_fill::BS_Easel3Fill;
pub use self::train_freight_front_fill::BS_TrainFreightFrontFill;
pub use self::gear_wide::BS_GearWide;
pub use self::chevron_right::BS_ChevronRight;
pub use self::file_earmark_binary_fill::BS_FileEarmarkBinaryFill;
pub use self::wikipedia::BS_Wikipedia;
pub use self::9_circle_fill::BS_9CircleFill;
pub use self::droplet::BS_Droplet;
pub use self::signpost_split::BS_SignpostSplit;
pub use self::stop_circle::BS_StopCircle;
pub use self::database_fill_gear::BS_DatabaseFillGear;
pub use self::caret_left::BS_CaretLeft;
pub use self::info_circle::BS_InfoCircle;
pub use self::database_fill_x::BS_DatabaseFillX;
pub use self::triangle::BS_Triangle;
pub use self::hand_thumbs_up_fill::BS_HandThumbsUpFill;
pub use self::journal_album::BS_JournalAlbum;
pub use self::cart2::BS_Cart2;
pub use self::shield_x::BS_ShieldX;
pub use self::record2::BS_Record2;
pub use self::suit_diamond_fill::BS_SuitDiamondFill;
pub use self::journal_text::BS_JournalText;
pub use self::sign_yield::BS_SignYield;
pub use self::puzzle::BS_Puzzle;
pub use self::telephone_minus_fill::BS_TelephoneMinusFill;
pub use self::file_pdf::BS_FilePdf;
pub use self::music_note::BS_MusicNote;
pub use self::arrows_angle_contract::BS_ArrowsAngleContract;
pub use self::box_arrow_in_left::BS_BoxArrowInLeft;
pub use self::recycle::BS_Recycle;
pub use self::chevron_double_down::BS_ChevronDoubleDown;
pub use self::filetype_pptx::BS_FiletypePptx;
pub use self::x_circle::BS_XCircle;
pub use self::type_bold::BS_TypeBold;
pub use self::phone_landscape::BS_PhoneLandscape;
pub use self::database_down::BS_DatabaseDown;
pub use self::database_fill_slash::BS_DatabaseFillSlash;
pub use self::person_badge_fill::BS_PersonBadgeFill;
pub use self::pie_chart::BS_PieChart;
pub use self::exclamation::BS_Exclamation;
pub use self::file_earmark_lock2::BS_FileEarmarkLock2;
pub use self::database_gear::BS_DatabaseGear;
pub use self::bookmark_star_fill::BS_BookmarkStarFill;
pub use self::skip_start_btn::BS_SkipStartBtn;
pub use self::sign_no_right_turn_fill::BS_SignNoRightTurnFill;
pub use self::bag_x::BS_BagX;
pub use self::zoom_in::BS_ZoomIn;
pub use self::arrow_right_circle_fill::BS_ArrowRightCircleFill;
pub use self::clipboard_check::BS_ClipboardCheck;
pub use self::camera_video_off_fill::BS_CameraVideoOffFill;
pub use self::database::BS_Database;
pub use self::gear_wide_connected::BS_GearWideConnected;
pub use self::telephone_plus::BS_TelephonePlus;
pub use self::file_earmark_word_fill::BS_FileEarmarkWordFill;
pub use self::eye_slash_fill::BS_EyeSlashFill;
pub use self::search::BS_Search;
pub use self::calendar2_range::BS_Calendar2Range;
pub use self::chat_square_dots::BS_ChatSquareDots;
pub use self::reception_0::BS_Reception0;
pub use self::calendar2_week_fill::BS_Calendar2WeekFill;
pub use self::file_zip::BS_FileZip;
pub use self::globe_central_south_asia::BS_GlobeCentralSouthAsia;
pub use self::pin_map_fill::BS_PinMapFill;
pub use self::layout_split::BS_LayoutSplit;
pub use self::cloud_check_fill::BS_CloudCheckFill;
pub use self::mortarboard_fill::BS_MortarboardFill;
pub use self::0_square_fill::BS_0SquareFill;
pub use self::file_lock2::BS_FileLock2;
pub use self::house_down::BS_HouseDown;
pub use self::cloud_haze2_fill::BS_CloudHaze2Fill;
pub use self::arrow_down_short::BS_ArrowDownShort;
pub use self::bookmark_x_fill::BS_BookmarkXFill;
pub use self::ev_station::BS_EvStation;
pub use self::filter_right::BS_FilterRight;
pub use self::heart_fill::BS_HeartFill;
pub use self::unlock_fill::BS_UnlockFill;
pub use self::file_earmark_music_fill::BS_FileEarmarkMusicFill;
pub use self::mouse3::BS_Mouse3;
pub use self::hourglass_bottom::BS_HourglassBottom;
pub use self::joystick::BS_Joystick;
pub use self::file_earmark_minus::BS_FileEarmarkMinus;
pub use self::lightbulb::BS_Lightbulb;
pub use self::file_earmark_spreadsheet::BS_FileEarmarkSpreadsheet;
pub use self::filetype_wav::BS_FiletypeWav;
pub use self::bag::BS_Bag;
pub use self::lightning_charge_fill::BS_LightningChargeFill;
pub use self::calendar3::BS_Calendar3;
pub use self::optical_audio::BS_OpticalAudio;
pub use self::database_check::BS_DatabaseCheck;
pub use self::chevron_double_right::BS_ChevronDoubleRight;
pub use self::pc::BS_Pc;
pub use self::envelope_paper_fill::BS_EnvelopePaperFill;
pub use self::file_break::BS_FileBreak;
pub use self::tsunami::BS_Tsunami;
pub use self::check_square_fill::BS_CheckSquareFill;
pub use self::6_square_fill::BS_6SquareFill;
pub use self::text_right::BS_TextRight;
pub use self::skip_start_btn_fill::BS_SkipStartBtnFill;
pub use self::suit_heart_fill::BS_SuitHeartFill;
pub use self::ear_fill::BS_EarFill;
pub use self::robot::BS_Robot;
pub use self::usb_drive_fill::BS_UsbDriveFill;
pub use self::mailbox2::BS_Mailbox2;
pub use self::filetype_heic::BS_FiletypeHeic;
pub use self::box_seam_fill::BS_BoxSeamFill;
pub use self::file_earmark_richtext::BS_FileEarmarkRichtext;
pub use self::signpost_2_fill::BS_Signpost2Fill;
pub use self::nut::BS_Nut;
pub use self::alexa::BS_Alexa;
pub use self::file_earmark_font_fill::BS_FileEarmarkFontFill;
pub use self::cloud_drizzle::BS_CloudDrizzle;
pub use self::key::BS_Key;
pub use self::displayport_fill::BS_DisplayportFill;
pub use self::clipboard_heart::BS_ClipboardHeart;
pub use self::sim::BS_Sim;
pub use self::pencil::BS_Pencil;
pub use self::save::BS_Save;
pub use self::umbrella_fill::BS_UmbrellaFill;
pub use self::tablet_landscape::BS_TabletLandscape;
pub use self::slash_square_fill::BS_SlashSquareFill;
pub use self::ethernet::BS_Ethernet;
pub use self::badge_ad_fill::BS_BadgeAdFill;
pub use self::chevron_down::BS_ChevronDown;
pub use self::badge_wc_fill::BS_BadgeWcFill;
pub use self::eject::BS_Eject;
pub use self::arrow_90deg_right::BS_Arrow90degRight;
pub use self::thermometer_sun::BS_ThermometerSun;
pub use self::chat_right_text_fill::BS_ChatRightTextFill;
pub use self::fast_forward_fill::BS_FastForwardFill;
pub use self::sign_no_parking_fill::BS_SignNoParkingFill;
pub use self::file_earmark_fill::BS_FileEarmarkFill;
pub use self::volume_down_fill::BS_VolumeDownFill;
pub use self::music_note_list::BS_MusicNoteList;
pub use self::envelope_exclamation_fill::BS_EnvelopeExclamationFill;
pub use self::telephone_x::BS_TelephoneX;
pub use self::virus::BS_Virus;
pub use self::basket3::BS_Basket3;
pub use self::box2_heart_fill::BS_Box2HeartFill;
pub use self::sign_intersection_side_fill::BS_SignIntersectionSideFill;
pub use self::hearts::BS_Hearts;
pub use self::sign_merge_left::BS_SignMergeLeft;
pub use self::cart_plus::BS_CartPlus;
pub use self::heart::BS_Heart;
pub use self::intersect::BS_Intersect;
pub use self::cloud_sun_fill::BS_CloudSunFill;
pub use self::arrow_right_circle::BS_ArrowRightCircle;
pub use self::textarea::BS_Textarea;
pub use self::symmetry_vertical::BS_SymmetryVertical;
pub use self::person_add::BS_PersonAdd;
pub use self::share::BS_Share;
pub use self::calendar_range_fill::BS_CalendarRangeFill;
pub use self::envelope_heart::BS_EnvelopeHeart;
pub use self::chat_square_dots_fill::BS_ChatSquareDotsFill;
pub use self::bookshelf::BS_Bookshelf;
pub use self::layer_backward::BS_LayerBackward;
pub use self::calendar4_range::BS_Calendar4Range;
pub use self::calendar3_fill::BS_Calendar3Fill;
pub use self::dash_circle::BS_DashCircle;
pub use self::badge_cc::BS_BadgeCc;
pub use self::spellcheck::BS_Spellcheck;
pub use self::filetype_doc::BS_FiletypeDoc;
pub use self::calendar2_heart_fill::BS_Calendar2HeartFill;
pub use self::badge_hd_fill::BS_BadgeHdFill;
pub use self::receipt_cutoff::BS_ReceiptCutoff;
pub use self::building_fill_lock::BS_BuildingFillLock;
pub use self::graph_down::BS_GraphDown;
pub use self::caret_down_square_fill::BS_CaretDownSquareFill;
pub use self::person_lines_fill::BS_PersonLinesFill;
pub use self::sticky::BS_Sticky;
pub use self::car_front::BS_CarFront;
pub use self::1_circle::BS_1Circle;
pub use self::sina_weibo::BS_SinaWeibo;
pub use self::wrench_adjustable_circle_fill::BS_WrenchAdjustableCircleFill;
pub use self::sign_turn_slight_left::BS_SignTurnSlightLeft;
pub use self::suit_spade_fill::BS_SuitSpadeFill;
pub use self::bookmark_check::BS_BookmarkCheck;
pub use self::fuel_pump_fill::BS_FuelPumpFill;
pub use self::sort_up::BS_SortUp;
pub use self::filetype_md::BS_FiletypeMd;
pub use self::skip_end_fill::BS_SkipEndFill;
pub use self::envelope_dash_fill::BS_EnvelopeDashFill;
pub use self::file_earmark_ruled::BS_FileEarmarkRuled;
pub use self::vimeo::BS_Vimeo;
pub use self::info::BS_Info;
pub use self::sort_numeric_down::BS_SortNumericDown;
pub use self::menu_up::BS_MenuUp;
pub use self::4_square::BS_4Square;
pub use self::cloud_lightning_fill::BS_CloudLightningFill;
pub use self::trophy_fill::BS_TrophyFill;
pub use self::pen_fill::BS_PenFill;
pub use self::file_minus_fill::BS_FileMinusFill;
pub use self::arrow_down_right_circle::BS_ArrowDownRightCircle;
pub use self::emoji_angry_fill::BS_EmojiAngryFill;
pub use self::award_fill::BS_AwardFill;
pub use self::wrench::BS_Wrench;
pub use self::file_earmark_text::BS_FileEarmarkText;
pub use self::cloud_arrow_up::BS_CloudArrowUp;
pub use self::star::BS_Star;
pub use self::file_earmark_ruled_fill::BS_FileEarmarkRuledFill;
pub use self::stripe::BS_Stripe;
pub use self::envelope_x::BS_EnvelopeX;
pub use self::filetype_aac::BS_FiletypeAac;
pub use self::ev_station_fill::BS_EvStationFill;
pub use self::postage::BS_Postage;
pub use self::skip_forward::BS_SkipForward;
pub use self::cart_x::BS_CartX;
pub use self::card_image::BS_CardImage;
pub use self::heart_arrow::BS_HeartArrow;
pub use self::arrow_90deg_left::BS_Arrow90degLeft;
pub use self::usb_plug_fill::BS_UsbPlugFill;
pub use self::file_earmark_plus_fill::BS_FileEarmarkPlusFill;
pub use self::bag_x_fill::BS_BagXFill;
pub use self::bag_heart::BS_BagHeart;
pub use self::person_fill_slash::BS_PersonFillSlash;
pub use self::chat_heart::BS_ChatHeart;
pub use self::database_fill_check::BS_DatabaseFillCheck;
pub use self::sign_stop_fill::BS_SignStopFill;
pub use self::usb_c::BS_UsbC;
pub use self::emoji_laughing::BS_EmojiLaughing;
pub use self::heartbreak::BS_Heartbreak;
pub use self::mouse::BS_Mouse;
pub use self::lamp::BS_Lamp;
pub use self::dice_6_fill::BS_Dice6Fill;
pub use self::currency_exchange::BS_CurrencyExchange;
pub use self::emoji_sunglasses_fill::BS_EmojiSunglassesFill;
pub use self::reception_1::BS_Reception1;
pub use self::clipboard_plus_fill::BS_ClipboardPlusFill;
pub use self::scissors::BS_Scissors;
pub use self::linkedin::BS_Linkedin;
pub use self::arrow_return_right::BS_ArrowReturnRight;
pub use self::repeat_1::BS_Repeat1;
pub use self::bag_heart_fill::BS_BagHeartFill;
pub use self::terminal_fill::BS_TerminalFill;
pub use self::file_plus::BS_FilePlus;
pub use self::telephone_outbound::BS_TelephoneOutbound;
pub use self::database_exclamation::BS_DatabaseExclamation;
pub use self::file_x_fill::BS_FileXFill;
pub use self::file_richtext::BS_FileRichtext;
pub use self::building_add::BS_BuildingAdd;
pub use self::markdown::BS_Markdown;
pub use self::bookmark_plus_fill::BS_BookmarkPlusFill;
pub use self::database_fill_dash::BS_DatabaseFillDash;
pub use self::blockquote_right::BS_BlockquoteRight;
pub use self::person_slash::BS_PersonSlash;
pub use self::filetype_ppt::BS_FiletypePpt;
pub use self::patch_minus_fill::BS_PatchMinusFill;
pub use self::envelope_open_heart::BS_EnvelopeOpenHeart;
pub use self::flower1::BS_Flower1;
pub use self::house_door::BS_HouseDoor;
pub use self::circle_square::BS_CircleSquare;
pub use self::geo_alt::BS_GeoAlt;
pub use self::building_fill_exclamation::BS_BuildingFillExclamation;
pub use self::text_indent_right::BS_TextIndentRight;
pub use self::person_check::BS_PersonCheck;
pub use self::calendar2_check::BS_Calendar2Check;
pub use self::postcard_fill::BS_PostcardFill;
pub use self::person_hearts::BS_PersonHearts;
pub use self::check2::BS_Check2;
pub use self::globe::BS_Globe;
pub use self::printer_fill::BS_PrinterFill;
pub use self::valentine2::BS_Valentine2;
pub use self::yin_yang::BS_YinYang;
pub use self::cash::BS_Cash;
pub use self::graph_up_arrow::BS_GraphUpArrow;
pub use self::calendar3_week_fill::BS_Calendar3WeekFill;
pub use self::2_square_fill::BS_2SquareFill;
pub use self::x_lg::BS_XLg;
pub use self::receipt::BS_Receipt;
pub use self::window_split::BS_WindowSplit;
pub use self::patch_question::BS_PatchQuestion;
pub use self::file_font::BS_FileFont;
pub use self::gender_trans::BS_GenderTrans;
pub use self::tablet::BS_Tablet;
pub use self::upc::BS_Upc;
pub use self::clipboard2_heart_fill::BS_Clipboard2HeartFill;
pub use self::plus_square_dotted::BS_PlusSquareDotted;
pub use self::cloud_haze_fill::BS_CloudHazeFill;
pub use self::display::BS_Display;
pub use self::file_code::BS_FileCode;
pub use self::medium::BS_Medium;
pub use self::sign_intersection_t_fill::BS_SignIntersectionTFill;
pub use self::calendar_month_fill::BS_CalendarMonthFill;
pub use self::tropical_storm::BS_TropicalStorm;
pub use self::cloud_fog_fill::BS_CloudFogFill;
pub use self::easel::BS_Easel;
pub use self::hospital::BS_Hospital;
pub use self::rss::BS_Rss;
pub use self::chat_right_text::BS_ChatRightText;
pub use self::pc_display::BS_PcDisplay;
pub use self::telephone_inbound_fill::BS_TelephoneInboundFill;
pub use self::filetype_raw::BS_FiletypeRaw;
pub use self::chat_square_quote::BS_ChatSquareQuote;
pub use self::person_gear::BS_PersonGear;
pub use self::file_word_fill::BS_FileWordFill;
pub use self::calendar3_event::BS_Calendar3Event;
pub use self::bug_fill::BS_BugFill;
pub use self::emoji_angry::BS_EmojiAngry;
pub use self::egg_fried::BS_EggFried;
pub use self::layers_fill::BS_LayersFill;
pub use self::hourglass_split::BS_HourglassSplit;
pub use self::border::BS_Border;
pub use self::stopwatch::BS_Stopwatch;
pub use self::border_bottom::BS_BorderBottom;
pub use self::badge_cc_fill::BS_BadgeCcFill;
pub use self::rocket::BS_Rocket;
pub use self::calendar2_event::BS_Calendar2Event;
pub use self::person_fill_dash::BS_PersonFillDash;
pub use self::envelope_paper::BS_EnvelopePaper;
pub use self::chat_quote::BS_ChatQuote;
pub use self::inbox_fill::BS_InboxFill;
pub use self::file_earmark_bar_graph_fill::BS_FileEarmarkBarGraphFill;
pub use self::hdd_stack_fill::BS_HddStackFill;
pub use self::snapchat::BS_Snapchat;
pub use self::question_diamond_fill::BS_QuestionDiamondFill;
pub use self::person::BS_Person;
pub use self::box2::BS_Box2;
pub use self::calendar2_range_fill::BS_Calendar2RangeFill;
pub use self::code::BS_Code;
pub use self::person_dash::BS_PersonDash;
pub use self::cloud_snow::BS_CloudSnow;
pub use self::vector_pen::BS_VectorPen;
pub use self::text_paragraph::BS_TextParagraph;
pub use self::border_outer::BS_BorderOuter;
pub use self::layout_text_window_reverse::BS_LayoutTextWindowReverse;
pub use self::box_arrow_up::BS_BoxArrowUp;
pub use self::house_x_fill::BS_HouseXFill;
pub use self::fullscreen_exit::BS_FullscreenExit;
pub use self::arrows_angle_expand::BS_ArrowsAngleExpand;
pub use self::image_fill::BS_ImageFill;
pub use self::eye_fill::BS_EyeFill;
pub use self::plus_circle_dotted::BS_PlusCircleDotted;
pub use self::boxes::BS_Boxes;
pub use self::houses::BS_Houses;
pub use self::house_exclamation_fill::BS_HouseExclamationFill;
pub use self::save2_fill::BS_Save2Fill;
pub use self::easel_fill::BS_EaselFill;
pub use self::person_lock::BS_PersonLock;
pub use self::person_video2::BS_PersonVideo2;
pub use self::ev_front_fill::BS_EvFrontFill;
pub use self::list_nested::BS_ListNested;
pub use self::rewind_fill::BS_RewindFill;
pub use self::droplet_fill::BS_DropletFill;
pub use self::power::BS_Power;
pub use self::screwdriver::BS_Screwdriver;
pub use self::segmented_nav::BS_SegmentedNav;
pub use self::phone_vibrate_fill::BS_PhoneVibrateFill;
pub use self::check_all::BS_CheckAll;
pub use self::dice_4_fill::BS_Dice4Fill;
pub use self::menu_app_fill::BS_MenuAppFill;
pub use self::envelope_plus_fill::BS_EnvelopePlusFill;
pub use self::border_left::BS_BorderLeft;
pub use self::badge_3d_fill::BS_Badge3dFill;
pub use self::calendar3_week::BS_Calendar3Week;
pub use self::chat_text::BS_ChatText;
pub use self::caret_left_square::BS_CaretLeftSquare;
pub use self::slash_circle_fill::BS_SlashCircleFill;
pub use self::wrench_adjustable_circle::BS_WrenchAdjustableCircle;
pub use self::chat_dots::BS_ChatDots;
pub use self::badge_vr::BS_BadgeVr;
pub use self::file_earmark_break_fill::BS_FileEarmarkBreakFill;
pub use self::cloud_hail::BS_CloudHail;
pub use self::folder_x::BS_FolderX;
pub use self::emoji_wink::BS_EmojiWink;
pub use self::4_square_fill::BS_4SquareFill;
pub use self::arrow_down_left::BS_ArrowDownLeft;
pub use self::file_earmark_post_fill::BS_FileEarmarkPostFill;
pub use self::people::BS_People;
pub use self::h_square::BS_HSquare;
pub use self::chat_right_dots::BS_ChatRightDots;
pub use self::usb_drive::BS_UsbDrive;
pub use self::signal::BS_Signal;
pub use self::gender_female::BS_GenderFemale;
pub use self::person_square::BS_PersonSquare;
pub use self::truck::BS_Truck;
pub use self::skip_backward::BS_SkipBackward;
pub use self::plus_square_fill::BS_PlusSquareFill;
pub use self::arrow_down_circle_fill::BS_ArrowDownCircleFill;
pub use self::file::BS_File;
pub use self::app::BS_App;
pub use self::grid_3x2_gap::BS_Grid3x2Gap;
pub use self::windows::BS_Windows;
pub use self::file_zip_fill::BS_FileZipFill;
pub use self::volume_mute::BS_VolumeMute;
pub use self::4_circle::BS_4Circle;
pub use self::sign_railroad::BS_SignRailroad;
pub use self::filetype_mp4::BS_FiletypeMp4;
pub use self::heartbreak_fill::BS_HeartbreakFill;
pub use self::caret_down::BS_CaretDown;
pub use self::microsoft_teams::BS_MicrosoftTeams;
pub use self::shield_plus::BS_ShieldPlus;
pub use self::symmetry_horizontal::BS_SymmetryHorizontal;
pub use self::clipboard2_x_fill::BS_Clipboard2XFill;
pub use self::file_richtext_fill::BS_FileRichtextFill;
pub use self::arrow_down_right_circle_fill::BS_ArrowDownRightCircleFill;
pub use self::map_fill::BS_MapFill;
pub use self::compass_fill::BS_CompassFill;
pub use self::emoji_wink_fill::BS_EmojiWinkFill;
pub use self::file_play::BS_FilePlay;
pub use self::graph_up::BS_GraphUp;
pub use self::sign_turn_right_fill::BS_SignTurnRightFill;
pub use self::5_square_fill::BS_5SquareFill;
pub use self::skip_backward_circle::BS_SkipBackwardCircle;
pub use self::sort_alpha_up::BS_SortAlphaUp;
pub use self::hexagon_fill::BS_HexagonFill;
pub use self::chat_square_text::BS_ChatSquareText;
pub use self::sign_no_left_turn::BS_SignNoLeftTurn;
pub use self::tv_fill::BS_TvFill;
pub use self::menu_button_fill::BS_MenuButtonFill;
pub use self::journal_minus::BS_JournalMinus;
pub use self::cart_plus_fill::BS_CartPlusFill;
pub use self::envelope_at::BS_EnvelopeAt;
pub use self::journal::BS_Journal;
pub use self::percent::BS_Percent;
pub use self::rulers::BS_Rulers;
pub use self::toggle2_off::BS_Toggle2Off;
pub use self::buildings_fill::BS_BuildingsFill;
pub use self::mouse3_fill::BS_Mouse3Fill;
pub use self::filetype_tiff::BS_FiletypeTiff;
pub use self::house_fill::BS_HouseFill;
pub use self::boombox_fill::BS_BoomboxFill;
pub use self::building_fill_dash::BS_BuildingFillDash;
pub use self::clipboard2_data_fill::BS_Clipboard2DataFill;
pub use self::filter::BS_Filter;
pub use self::sliders2::BS_Sliders2;
pub use self::sign_turn_left::BS_SignTurnLeft;
pub use self::radioactive::BS_Radioactive;
pub use self::caret_left_fill::BS_CaretLeftFill;
pub use self::7_square_fill::BS_7SquareFill;
pub use self::cloud_haze2::BS_CloudHaze2;
pub use self::arrow_right::BS_ArrowRight;
pub use self::facebook::BS_Facebook;
pub use self::magnet::BS_Magnet;
pub use self::file_slides_fill::BS_FileSlidesFill;
pub use self::file_excel::BS_FileExcel;
pub use self::arrow_right_square_fill::BS_ArrowRightSquareFill;
pub use self::code_square::BS_CodeSquare;
pub use self::calendar2_x_fill::BS_Calendar2XFill;
pub use self::house_heart::BS_HouseHeart;
pub use self::file_earmark_image::BS_FileEarmarkImage;
pub use self::bucket::BS_Bucket;
pub use self::cup_straw::BS_CupStraw;
pub use self::file_earmark_lock2_fill::BS_FileEarmarkLock2Fill;
pub use self::moon_stars_fill::BS_MoonStarsFill;
pub use self::file_pdf_fill::BS_FilePdfFill;
pub use self::gear::BS_Gear;
pub use self::box_arrow_in_right::BS_BoxArrowInRight;
pub use self::filetype_html::BS_FiletypeHtml;
pub use self::table::BS_Table;
pub use self::arrow_down_right_square::BS_ArrowDownRightSquare;
pub use self::dice_3_fill::BS_Dice3Fill;
pub use self::database_dash::BS_DatabaseDash;
pub use self::2_circle::BS_2Circle;
pub use self::clipboard2_fill::BS_Clipboard2Fill;
pub use self::rewind::BS_Rewind;
pub use self::credit_card_fill::BS_CreditCardFill;
pub use self::dot::BS_Dot;
pub use self::nut_fill::BS_NutFill;
pub use self::filetype_tsx::BS_FiletypeTsx;
pub use self::bookmark_dash::BS_BookmarkDash;
pub use self::cloud_fog::BS_CloudFog;
pub use self::rocket_fill::BS_RocketFill;
pub use self::file_earmark_check_fill::BS_FileEarmarkCheckFill;
pub use self::bluetooth::BS_Bluetooth;
pub use self::shuffle::BS_Shuffle;
pub use self::badge_ar::BS_BadgeAr;
pub use self::pip_fill::BS_PipFill;
pub use self::bootstrap::BS_Bootstrap;
pub use self::film::BS_Film;
pub use self::battery_charging::BS_BatteryCharging;
pub use self::clipboard2_plus::BS_Clipboard2Plus;
pub use self::cloud_sleet_fill::BS_CloudSleetFill;
pub use self::skip_forward_fill::BS_SkipForwardFill;
pub use self::chevron_compact_left::BS_ChevronCompactLeft;
pub use self::mic_fill::BS_MicFill;
pub use self::safe2_fill::BS_Safe2Fill;
pub use self::kanban_fill::BS_KanbanFill;
pub use self::rewind_btn_fill::BS_RewindBtnFill;
pub use self::reply_fill::BS_ReplyFill;
pub use self::twitter::BS_Twitter;
pub use self::toggle_on::BS_ToggleOn;
pub use self::currency_rupee::BS_CurrencyRupee;
pub use self::badge_sd_fill::BS_BadgeSdFill;
pub use self::person_vcard::BS_PersonVcard;
pub use self::cpu::BS_Cpu;
pub use self::caret_down_square::BS_CaretDownSquare;
pub use self::node_minus::BS_NodeMinus;
pub use self::handbag_fill::BS_HandbagFill;
pub use self::door_open_fill::BS_DoorOpenFill;
pub use self::ev_front::BS_EvFront;
pub use self::arrow_bar_up::BS_ArrowBarUp;
pub use self::play_btn::BS_PlayBtn;
pub use self::send_plus::BS_SendPlus;
pub use self::chevron_bar_down::BS_ChevronBarDown;
pub use self::caret_down_fill::BS_CaretDownFill;
pub use self::mic_mute::BS_MicMute;
pub use self::person_fill_down::BS_PersonFillDown;
pub use self::type_h1::BS_TypeH1;
pub use self::play_btn_fill::BS_PlayBtnFill;
pub use self::9_circle::BS_9Circle;
pub use self::database_fill_down::BS_DatabaseFillDown;
pub use self::unlock::BS_Unlock;
pub use self::brightness_alt_high::BS_BrightnessAltHigh;
pub use self::chat_dots_fill::BS_ChatDotsFill;
pub use self::eyeglasses::BS_Eyeglasses;
pub use self::arrows_move::BS_ArrowsMove;
pub use self::mouse_fill::BS_MouseFill;
pub use self::file_earmark_zip::BS_FileEarmarkZip;
pub use self::slash_circle::BS_SlashCircle;
pub use self::alt::BS_Alt;
pub use self::sign_intersection_y_fill::BS_SignIntersectionYFill;
pub use self::brightness_high_fill::BS_BrightnessHighFill;
pub use self::lock::BS_Lock;
pub use self::cloud_moon::BS_CloudMoon;
pub use self::hand_index_fill::BS_HandIndexFill;
pub use self::border_style::BS_BorderStyle;
pub use self::cloud_rain_fill::BS_CloudRainFill;
pub use self::telegram::BS_Telegram;
pub use self::tag::BS_Tag;
pub use self::trello::BS_Trello;
pub use self::calendar_month::BS_CalendarMonth;
pub use self::safe2::BS_Safe2;
pub use self::building_dash::BS_BuildingDash;
pub use self::filetype_js::BS_FiletypeJs;
pub use self::box_arrow_right::BS_BoxArrowRight;
pub use self::messenger::BS_Messenger;
pub use self::arrow_left::BS_ArrowLeft;
pub use self::palette2::BS_Palette2;
pub use self::file_earmark_x_fill::BS_FileEarmarkXFill;
pub use self::fast_forward_btn::BS_FastForwardBtn;
pub use self::file_diff::BS_FileDiff;
pub use self::badge_vr_fill::BS_BadgeVrFill;
pub use self::broadcast::BS_Broadcast;
pub use self::pause_circle::BS_PauseCircle;
pub use self::pause_fill::BS_PauseFill;
pub use self::motherboard_fill::BS_MotherboardFill;
pub use self::arrow_right_square::BS_ArrowRightSquare;
pub use self::database_fill_add::BS_DatabaseFillAdd;
pub use self::filter_square_fill::BS_FilterSquareFill;
pub use self::virus2::BS_Virus2;
pub use self::chat_text_fill::BS_ChatTextFill;
pub use self::skip_forward_circle_fill::BS_SkipForwardCircleFill;
pub use self::chat_right_quote_fill::BS_ChatRightQuoteFill;
pub use self::peace_fill::BS_PeaceFill;
pub use self::pin_fill::BS_PinFill;
pub use self::box2_fill::BS_Box2Fill;
pub use self::chat_square_heart_fill::BS_ChatSquareHeartFill;
pub use self::bar_chart_line::BS_BarChartLine;
pub use self::calendar_heart::BS_CalendarHeart;
pub use self::phone_fill::BS_PhoneFill;
pub use self::send_slash::BS_SendSlash;
pub use self::tags_fill::BS_TagsFill;
pub use self::crop::BS_Crop;
pub use self::filetype_ttf::BS_FiletypeTtf;
pub use self::menu_button_wide::BS_MenuButtonWide;
pub use self::award::BS_Award;
pub use self::3_square_fill::BS_3SquareFill;
pub use self::grip_horizontal::BS_GripHorizontal;
pub use self::9_square_fill::BS_9SquareFill;
pub use self::file_music::BS_FileMusic;
pub use self::cursor_fill::BS_CursorFill;
pub use self::person_video3::BS_PersonVideo3;
pub use self::hdd_fill::BS_HddFill;
pub use self::suit_club::BS_SuitClub;
pub use self::suit_club_fill::BS_SuitClubFill;
pub use self::collection::BS_Collection;
pub use self::option::BS_Option;
pub use self::chevron_double_left::BS_ChevronDoubleLeft;
pub use self::safe_fill::BS_SafeFill;
pub use self::brush_fill::BS_BrushFill;
pub use self::6_square::BS_6Square;
pub use self::shop_window::BS_ShopWindow;
pub use self::shield_exclamation::BS_ShieldExclamation;
pub use self::record_circle_fill::BS_RecordCircleFill;
pub use self::file_ppt_fill::BS_FilePptFill;
pub use self::dice_1_fill::BS_Dice1Fill;
pub use self::arrow_bar_left::BS_ArrowBarLeft;
pub use self::hdd_network::BS_HddNetwork;
pub use self::clock_fill::BS_ClockFill;
pub use self::tablet_fill::BS_TabletFill;
pub use self::heart_pulse_fill::BS_HeartPulseFill;
pub use self::filetype_pdf::BS_FiletypePdf;
pub use self::box_arrow_in_down::BS_BoxArrowInDown;
pub use self::repeat::BS_Repeat;
pub use self::basket_fill::BS_BasketFill;
pub use self::globe2::BS_Globe2;
pub use self::sun_fill::BS_SunFill;
pub use self::gift::BS_Gift;
pub use self::layout_sidebar_inset_reverse::BS_LayoutSidebarInsetReverse;
pub use self::plug_fill::BS_PlugFill;
pub use self::phone::BS_Phone;
pub use self::aspect_ratio::BS_AspectRatio;
pub use self::truck_front_fill::BS_TruckFrontFill;
pub use self::behance::BS_Behance;
pub use self::universal_access_circle::BS_UniversalAccessCircle;
pub use self::graph_down_arrow::BS_GraphDownArrow;
pub use self::file_earmark_zip_fill::BS_FileEarmarkZipFill;
pub use self::bandaid_fill::BS_BandaidFill;
pub use self::arrow_down_square::BS_ArrowDownSquare;
pub use self::calendar_check_fill::BS_CalendarCheckFill;
pub use self::book_fill::BS_BookFill;
pub use self::usb_micro_fill::BS_UsbMicroFill;
pub use self::controller::BS_Controller;
pub use self::fonts::BS_Fonts;
pub use self::envelope_exclamation::BS_EnvelopeExclamation;
pub use self::pause::BS_Pause;
pub use self::bookmark_dash_fill::BS_BookmarkDashFill;
pub use self::box_arrow_down_left::BS_BoxArrowDownLeft;
pub use self::front::BS_Front;
pub use self::sort_numeric_down_alt::BS_SortNumericDownAlt;
pub use self::dice_6::BS_Dice6;
pub use self::rocket_takeoff_fill::BS_RocketTakeoffFill;
pub use self::folder2::BS_Folder2;
pub use self::window_desktop::BS_WindowDesktop;
pub use self::file_minus::BS_FileMinus;
pub use self::mailbox::BS_Mailbox;
pub use self::thermometer_half::BS_ThermometerHalf;
pub use self::person_check_fill::BS_PersonCheckFill;
pub use self::plus_circle_fill::BS_PlusCircleFill;
pub use self::flower2::BS_Flower2;
pub use self::arrows_expand::BS_ArrowsExpand;
pub use self::shield_fill_plus::BS_ShieldFillPlus;
pub use self::flower3::BS_Flower3;
pub use self::filetype_mov::BS_FiletypeMov;
pub use self::pentagon_fill::BS_PentagonFill;
pub use self::cloud_download_fill::BS_CloudDownloadFill;
pub use self::shift::BS_Shift;
pub use self::usb_mini::BS_UsbMini;
pub use self::file_earmark_check::BS_FileEarmarkCheck;
pub use self::4_circle_fill::BS_4CircleFill;
pub use self::node_minus_fill::BS_NodeMinusFill;
pub use self::check2_all::BS_Check2All;
pub use self::sign_stop::BS_SignStop;
pub use self::type::BS_Type;
pub use self::layout_sidebar::BS_LayoutSidebar;
pub use self::file_arrow_down::BS_FileArrowDown;
pub use self::bag_dash::BS_BagDash;
pub use self::cpu_fill::BS_CpuFill;
pub use self::p_circle::BS_PCircle;
pub use self::router_fill::BS_RouterFill;
pub use self::balloon_heart_fill::BS_BalloonHeartFill;
pub use self::file_earmark_easel_fill::BS_FileEarmarkEaselFill;
pub use self::clipboard2_data::BS_Clipboard2Data;
pub use self::person_bounding_box::BS_PersonBoundingBox;
pub use self::kanban::BS_Kanban;
pub use self::box_arrow_in_down_left::BS_BoxArrowInDownLeft;
pub use self::telephone_fill::BS_TelephoneFill;
pub use self::person_exclamation::BS_PersonExclamation;
pub use self::file_check::BS_FileCheck;
pub use self::house_add::BS_HouseAdd;
pub use self::file_easel_fill::BS_FileEaselFill;
pub use self::shop::BS_Shop;
pub use self::arrow_counterclockwise::BS_ArrowCounterclockwise;
pub use self::bullseye::BS_Bullseye;
pub use self::globe_asia_australia::BS_GlobeAsiaAustralia;
pub use self::plus_slash_minus::BS_PlusSlashMinus;
pub use self::pass_fill::BS_PassFill;
pub use self::envelope_check::BS_EnvelopeCheck;
pub use self::wind::BS_Wind;
pub use self::5_circle_fill::BS_5CircleFill;
pub use self::geo::BS_Geo;
pub use self::pci_card::BS_PciCard;
pub use self::calendar4_event::BS_Calendar4Event;
pub use self::music_player_fill::BS_MusicPlayerFill;
pub use self::filetype_java::BS_FiletypeJava;
pub use self::filetype_css::BS_FiletypeCss;
pub use self::file_earmark_diff::BS_FileEarmarkDiff;
pub use self::building_fill_check::BS_BuildingFillCheck;
pub use self::envelope_open::BS_EnvelopeOpen;
pub use self::person_fill_up::BS_PersonFillUp;
pub use self::laptop_fill::BS_LaptopFill;
pub use self::device_ssd_fill::BS_DeviceSsdFill;
pub use self::person_video::BS_PersonVideo;
pub use self::skip_start_fill::BS_SkipStartFill;
pub use self::camera::BS_Camera;
pub use self::ubuntu::BS_Ubuntu;
pub use self::cloud_upload_fill::BS_CloudUploadFill;
pub use self::database_up::BS_DatabaseUp;
pub use self::fire::BS_Fire;
pub use self::border_inner::BS_BorderInner;
pub use self::badge_wc::BS_BadgeWc;
pub use self::exclamation_square_fill::BS_ExclamationSquareFill;
pub use self::brightness_alt_high_fill::BS_BrightnessAltHighFill;
pub use self::bandaid::BS_Bandaid;
pub use self::vr::BS_Vr;
pub use self::plus_lg::BS_PlusLg;
pub use self::webcam::BS_Webcam;
pub use self::person_fill::BS_PersonFill;
pub use self::piggy_bank_fill::BS_PiggyBankFill;
pub use self::sign_turn_right::BS_SignTurnRight;
pub use self::calendar2_check_fill::BS_Calendar2CheckFill;
pub use self::box_arrow_in_up_right::BS_BoxArrowInUpRight;
pub use self::bounding_box_circles::BS_BoundingBoxCircles;
pub use self::send_x::BS_SendX;
pub use self::chevron_bar_left::BS_ChevronBarLeft;
pub use self::bar_chart::BS_BarChart;
pub use self::dash_square::BS_DashSquare;
pub use self::9_square::BS_9Square;
pub use self::phone_vibrate::BS_PhoneVibrate;
pub use self::outlet::BS_Outlet;
pub use self::lightning_charge::BS_LightningCharge;
pub use self::arrow_up_right::BS_ArrowUpRight;
pub use self::hand_thumbs_down_fill::BS_HandThumbsDownFill;
pub use self::dice_3::BS_Dice3;
pub use self::grip_vertical::BS_GripVertical;
pub use self::strava::BS_Strava;
pub use self::envelope_open_heart_fill::BS_EnvelopeOpenHeartFill;
pub use self::filetype_php::BS_FiletypePhp;
pub use self::webcam_fill::BS_WebcamFill;
pub use self::sunrise_fill::BS_SunriseFill;
pub use self::emoji_neutral_fill::BS_EmojiNeutralFill;
pub use self::subtract::BS_Subtract;
pub use self::briefcase::BS_Briefcase;
pub use self::browser_firefox::BS_BrowserFirefox;
pub use self::building::BS_Building;
pub use self::sign_yield_fill::BS_SignYieldFill;
pub use self::caret_up_square_fill::BS_CaretUpSquareFill;
pub use self::folder_plus::BS_FolderPlus;
pub use self::dpad::BS_Dpad;
pub use self::card_text::BS_CardText;
pub use self::calendar4_week::BS_Calendar4Week;
pub use self::emoji_dizzy_fill::BS_EmojiDizzyFill;
pub use self::pc_display_horizontal::BS_PcDisplayHorizontal;
pub use self::box_arrow_in_up::BS_BoxArrowInUp;
pub use self::menu_down::BS_MenuDown;
pub use self::badge_8k::BS_Badge8k;
pub use self::exclamation_diamond_fill::BS_ExclamationDiamondFill;
pub use self::reddit::BS_Reddit;
pub use self::patch_exclamation_fill::BS_PatchExclamationFill;
pub use self::envelope_heart_fill::BS_EnvelopeHeartFill;
pub use self::earbuds::BS_Earbuds;
pub use self::camera2::BS_Camera2;
pub use self::grid_3x3::BS_Grid3x3;
pub use self::toggle2_on::BS_Toggle2On;
pub use self::sign_intersection_side::BS_SignIntersectionSide;
pub use self::filetype_mp3::BS_FiletypeMp3;
pub use self::megaphone_fill::BS_MegaphoneFill;
pub use self::pin_map::BS_PinMap;
pub use self::house_up_fill::BS_HouseUpFill;
pub use self::card_list::BS_CardList;
pub use self::motherboard::BS_Motherboard;
pub use self::credit_card::BS_CreditCard;
pub use self::123::BS_123;
pub use self::filetype_xls::BS_FiletypeXls;
pub use self::stop_fill::BS_StopFill;
pub use self::regex::BS_Regex;
pub use self::trash3::BS_Trash3;
pub use self::arrow_clockwise::BS_ArrowClockwise;
pub use self::trophy::BS_Trophy;
pub use self::magnet_fill::BS_MagnetFill;
pub use self::cone::BS_Cone;
pub use self::view_list::BS_ViewList;
pub use self::type_underline::BS_TypeUnderline;
pub use self::info_lg::BS_InfoLg;
pub use self::check_circle_fill::BS_CheckCircleFill;
pub use self::send_fill::BS_SendFill;
pub use self::file_earmark_text_fill::BS_FileEarmarkTextFill;
pub use self::diagram_2_fill::BS_Diagram2Fill;
pub use self::alipay::BS_Alipay;
pub use self::arrow_left_circle::BS_ArrowLeftCircle;
pub use self::ticket_detailed_fill::BS_TicketDetailedFill;
pub use self::filetype_docx::BS_FiletypeDocx;
pub use self::phone_landscape_fill::BS_PhoneLandscapeFill;
pub use self::chevron_compact_up::BS_ChevronCompactUp;
pub use self::shield::BS_Shield;
pub use self::caret_left_square_fill::BS_CaretLeftSquareFill;
pub use self::wechat::BS_Wechat;
pub use self::arrow_through_heart_fill::BS_ArrowThroughHeartFill;
pub use self::qr_code_scan::BS_QrCodeScan;
pub use self::send_check_fill::BS_SendCheckFill;
pub use self::chat_right_fill::BS_ChatRightFill;
pub use self::link_45deg::BS_Link45deg;
pub use self::file_image::BS_FileImage;
pub use self::check_circle::BS_CheckCircle;
pub use self::filetype_scss::BS_FiletypeScss;
pub use self::house_exclamation::BS_HouseExclamation;
pub use self::emoji_kiss_fill::BS_EmojiKissFill;
pub use self::stars::BS_Stars;
pub use self::shield_fill_minus::BS_ShieldFillMinus;
pub use self::camera_reels_fill::BS_CameraReelsFill;
pub use self::menu_app::BS_MenuApp;
pub use self::arrow_up_square_fill::BS_ArrowUpSquareFill;
pub use self::clipboard2_plus_fill::BS_Clipboard2PlusFill;
pub use self::chat_right_quote::BS_ChatRightQuote;
pub use self::brightness_high::BS_BrightnessHigh;
pub use self::signpost::BS_Signpost;
pub use self::filetype_xml::BS_FiletypeXml;
pub use self::exclamation_lg::BS_ExclamationLg;
pub use self::person_fill_exclamation::BS_PersonFillExclamation;
pub use self::volume_off_fill::BS_VolumeOffFill;
pub use self::send::BS_Send;
pub use self::file_binary::BS_FileBinary;
pub use self::house_down_fill::BS_HouseDownFill;
pub use self::skip_backward_circle_fill::BS_SkipBackwardCircleFill;
pub use self::skip_forward_btn_fill::BS_SkipForwardBtnFill;
pub use self::house_add_fill::BS_HouseAddFill;
pub use self::cloud_fill::BS_CloudFill;
pub use self::8_circle_fill::BS_8CircleFill;
pub use self::clipboard2_pulse::BS_Clipboard2Pulse;
pub use self::calendar3_event_fill::BS_Calendar3EventFill;
pub use self::file_text::BS_FileText;
pub use self::tools::BS_Tools;
pub use self::cloud_haze::BS_CloudHaze;
pub use self::filetype_xlsx::BS_FiletypeXlsx;
pub use self::code_slash::BS_CodeSlash;
pub use self::grid_1x2_fill::BS_Grid1x2Fill;
pub use self::upload::BS_Upload;
pub use self::file_arrow_down_fill::BS_FileArrowDownFill;
pub use self::p_square::BS_PSquare;
pub use self::zoom_out::BS_ZoomOut;
pub use self::arrow_bar_right::BS_ArrowBarRight;
pub use self::7_square::BS_7Square;
pub use self::question_octagon_fill::BS_QuestionOctagonFill;
pub use self::building_down::BS_BuildingDown;
pub use self::blockquote_left::BS_BlockquoteLeft;
pub use self::window_sidebar::BS_WindowSidebar;
pub use self::sign_do_not_enter::BS_SignDoNotEnter;
pub use self::file_earmark_person::BS_FileEarmarkPerson;
pub use self::displayport::BS_Displayport;
pub use self::envelope_paper_heart_fill::BS_EnvelopePaperHeartFill;
pub use self::trash2_fill::BS_Trash2Fill;
pub use self::hdd_rack_fill::BS_HddRackFill;
pub use self::exclamation_triangle_fill::BS_ExclamationTriangleFill;
pub use self::justify_right::BS_JustifyRight;
pub use self::airplane::BS_Airplane;
pub use self::dice_4::BS_Dice4;
pub use self::usb_symbol::BS_UsbSymbol;
pub use self::file_bar_graph::BS_FileBarGraph;
pub use self::triangle_fill::BS_TriangleFill;
pub use self::file_music_fill::BS_FileMusicFill;
pub use self::instagram::BS_Instagram;
pub use self::device_hdd::BS_DeviceHdd;
pub use self::funnel_fill::BS_FunnelFill;
pub use self::filetype_sass::BS_FiletypeSass;
pub use self::escape::BS_Escape;
pub use self::currency_dollar::BS_CurrencyDollar;
pub use self::r_square_fill::BS_RSquareFill;
pub use self::cloud_slash_fill::BS_CloudSlashFill;
pub use self::hand_thumbs_up::BS_HandThumbsUp;
pub use self::bricks::BS_Bricks;
pub use self::card_heading::BS_CardHeading;
pub use self::clipboard_check_fill::BS_ClipboardCheckFill;
pub use self::hdmi::BS_Hdmi;
pub use self::patch_plus::BS_PatchPlus;
pub use self::dash_circle_dotted::BS_DashCircleDotted;
pub use self::wrench_adjustable::BS_WrenchAdjustable;
pub use self::mouse2_fill::BS_Mouse2Fill;
pub use self::wordpress::BS_Wordpress;
pub use self::paint_bucket::BS_PaintBucket;
pub use self::chat_right::BS_ChatRight;
pub use self::shift_fill::BS_ShiftFill;
pub use self::telephone_forward::BS_TelephoneForward;
pub use self::cassette_fill::BS_CassetteFill;
pub use self::slash::BS_Slash;
pub use self::filetype_bmp::BS_FiletypeBmp;
pub use self::building_gear::BS_BuildingGear;
pub use self::clouds_fill::BS_CloudsFill;
pub use self::sign_railroad_fill::BS_SignRailroadFill;
pub use self::grid_3x3_gap_fill::BS_Grid3x3GapFill;
pub use self::keyboard_fill::BS_KeyboardFill;
pub use self::arrow_down_left_circle_fill::BS_ArrowDownLeftCircleFill;
pub use self::terminal_plus::BS_TerminalPlus;
pub use self::ui_radios::BS_UiRadios;
pub use self::arrow_up::BS_ArrowUp;
pub use self::arrow_up_square::BS_ArrowUpSquare;
pub use self::android::BS_Android;
pub use self::water::BS_Water;
pub use self::input_cursor_text::BS_InputCursorText;
pub use self::file_earmark_lock_fill::BS_FileEarmarkLockFill;
pub use self::hospital_fill::BS_HospitalFill;
pub use self::sort_up_alt::BS_SortUpAlt;
pub use self::sim_fill::BS_SimFill;
pub use self::cloud_sun::BS_CloudSun;
pub use self::fingerprint::BS_Fingerprint;
pub use self::wifi_off::BS_WifiOff;
pub use self::arrow_down::BS_ArrowDown;
pub use self::file_ruled::BS_FileRuled;
pub use self::save2::BS_Save2;
pub use self::volume_off::BS_VolumeOff;
pub use self::clipboard_pulse::BS_ClipboardPulse;
pub use self::question_lg::BS_QuestionLg;
pub use self::file_earmark_music::BS_FileEarmarkMusic;
pub use self::1_square::BS_1Square;
pub use self::align_bottom::BS_AlignBottom;
pub use self::lungs_fill::BS_LungsFill;
pub use self::cassette::BS_Cassette;
pub use self::arrow_up_right_circle_fill::BS_ArrowUpRightCircleFill;
pub use self::filetype_jsx::BS_FiletypeJsx;
pub use self::telephone_plus_fill::BS_TelephonePlusFill;
pub use self::moon_stars::BS_MoonStars;
pub use self::diamond::BS_Diamond;
pub use self::bank2::BS_Bank2;
pub use self::arrow_down_right::BS_ArrowDownRight;
pub use self::postcard_heart_fill::BS_PostcardHeartFill;
pub use self::egg_fill::BS_EggFill;
pub use self::r_square::BS_RSquare;
pub use self::rss_fill::BS_RssFill;
pub use self::shield_lock_fill::BS_ShieldLockFill;
pub use self::badge_vo::BS_BadgeVo;
pub use self::mastodon::BS_Mastodon;
pub use self::textarea_resize::BS_TextareaResize;
pub use self::battery::BS_Battery;
pub use self::upc_scan::BS_UpcScan;
pub use self::credit_card_2_back::BS_CreditCard2Back;
pub use self::emoji_kiss::BS_EmojiKiss;
pub use self::cloudy::BS_Cloudy;
pub use self::volume_down::BS_VolumeDown;
pub use self::easel3::BS_Easel3;
pub use self::file_earmark_pdf_fill::BS_FileEarmarkPdfFill;
pub use self::envelope_paper_heart::BS_EnvelopePaperHeart;
pub use self::hand_index::BS_HandIndex;
pub use self::filetype_mdx::BS_FiletypeMdx;
pub use self::fuel_pump::BS_FuelPump;
pub use self::lamp_fill::BS_LampFill;
pub use self::lightbulb_fill::BS_LightbulbFill;
pub use self::send_x_fill::BS_SendXFill;
pub use self::airplane_fill::BS_AirplaneFill;
pub use self::bell::BS_Bell;
pub use self::box_arrow_up_left::BS_BoxArrowUpLeft;
pub use self::sign_merge_left_fill::BS_SignMergeLeftFill;
pub use self::arrow_down_right_square_fill::BS_ArrowDownRightSquareFill;
pub use self::shield_fill::BS_ShieldFill;
pub use self::badge_vo_fill::BS_BadgeVoFill;
pub use self::shield_shaded::BS_ShieldShaded;
pub use self::plug::BS_Plug;
pub use self::suit_spade::BS_SuitSpade;
pub use self::type_strikethrough::BS_TypeStrikethrough;
pub use self::camera_reels::BS_CameraReels;
pub use self::save_fill::BS_SaveFill;
pub use self::calendar_heart_fill::BS_CalendarHeartFill;
pub use self::pencil_fill::BS_PencilFill;
pub use self::cloud_sleet::BS_CloudSleet;
pub use self::taxi_front::BS_TaxiFront;
pub use self::cash_coin::BS_CashCoin;
pub use self::folder_fill::BS_FolderFill;
pub use self::collection_play::BS_CollectionPlay;
pub use self::dropbox::BS_Dropbox;
pub use self::sign_no_parking::BS_SignNoParking;
pub use self::badge_tm_fill::BS_BadgeTmFill;
pub use self::record2_fill::BS_Record2Fill;
pub use self::circle_fill::BS_CircleFill;
pub use self::bag_dash_fill::BS_BagDashFill;
pub use self::arrow_left_square::BS_ArrowLeftSquare;
pub use self::calculator::BS_Calculator;
pub use self::file_earmark::BS_FileEarmark;
pub use self::calendar_plus_fill::BS_CalendarPlusFill;
pub use self::emoji_heart_eyes_fill::BS_EmojiHeartEyesFill;
pub use self::1_square_fill::BS_1SquareFill;
pub use self::play_circle::BS_PlayCircle;
pub use self::capsule::BS_Capsule;
pub use self::database_add::BS_DatabaseAdd;
pub use self::layout_wtf::BS_LayoutWtf;
pub use self::postage_heart_fill::BS_PostageHeartFill;
pub use self::dash_square_fill::BS_DashSquareFill;
pub use self::align_start::BS_AlignStart;
pub use self::emoji_smile_upside_down_fill::BS_EmojiSmileUpsideDownFill;
pub use self::chevron_bar_expand::BS_ChevronBarExpand;
pub use self::geo_alt_fill::BS_GeoAltFill;
pub use self::house_dash_fill::BS_HouseDashFill;
pub use self::send_dash_fill::BS_SendDashFill;
pub use self::exclude::BS_Exclude;
pub use self::emoji_expressionless_fill::BS_EmojiExpressionlessFill;
pub use self::calendar2_plus::BS_Calendar2Plus;
pub use self::newspaper::BS_Newspaper;
pub use self::calendar_week::BS_CalendarWeek;
pub use self::usb_plug::BS_UsbPlug;
pub use self::exclamation_octagon::BS_ExclamationOctagon;
pub use self::cash_stack::BS_CashStack;
pub use self::box_arrow_in_up_left::BS_BoxArrowInUpLeft;
pub use self::reply_all_fill::BS_ReplyAllFill;
pub use self::pause_btn_fill::BS_PauseBtnFill;
pub use self::file_earmark_slides::BS_FileEarmarkSlides;
pub use self::info_square_fill::BS_InfoSquareFill;
pub use self::calendar2_month_fill::BS_Calendar2MonthFill;
pub use self::house::BS_House;
pub use self::journal_arrow_up::BS_JournalArrowUp;
pub use self::reception_2::BS_Reception2;
pub use self::filetype_cs::BS_FiletypeCs;
pub use self::building_fill_x::BS_BuildingFillX;
pub use self::octagon::BS_Octagon;
pub use self::cloud_arrow_down::BS_CloudArrowDown;
pub use self::cloud::BS_Cloud;
pub use self::shield_slash_fill::BS_ShieldSlashFill;
pub use self::gift_fill::BS_GiftFill;
pub use self::disc_fill::BS_DiscFill;
pub use self::columns_gap::BS_ColumnsGap;
pub use self::sign_merge_right::BS_SignMergeRight;
pub use self::emoji_frown::BS_EmojiFrown;
pub use self::house_door_fill::BS_HouseDoorFill;
pub use self::bell_slash::BS_BellSlash;
pub use self::database_lock::BS_DatabaseLock;
pub use self::optical_audio_fill::BS_OpticalAudioFill;
pub use self::cloud_lightning_rain::BS_CloudLightningRain;
pub use self::layout_sidebar_reverse::BS_LayoutSidebarReverse;
pub use self::window_dock::BS_WindowDock;
pub use self::file_earmark_medical::BS_FileEarmarkMedical;
pub use self::c_square_fill::BS_CSquareFill;
pub use self::exclamation_circle_fill::BS_ExclamationCircleFill;
pub use self::layout_three_columns::BS_LayoutThreeColumns;
pub use self::file_earmark_excel::BS_FileEarmarkExcel;
pub use self::quora::BS_Quora;
pub use self::filetype_rb::BS_FiletypeRb;
pub use self::balloon::BS_Balloon;
pub use self::battery_full::BS_BatteryFull;
pub use self::paperclip::BS_Paperclip;
pub use self::clipboard::BS_Clipboard;
pub use self::arrow_down_up::BS_ArrowDownUp;
pub use self::chat_left_text::BS_ChatLeftText;
pub use self::capslock_fill::BS_CapslockFill;
pub use self::cart3::BS_Cart3;
pub use self::modem::BS_Modem;
pub use self::music_player::BS_MusicPlayer;
pub use self::sign_turn_slight_left_fill::BS_SignTurnSlightLeftFill;
pub use self::airplane_engines_fill::BS_AirplaneEnginesFill;
pub use self::building_slash::BS_BuildingSlash;
pub use self::emoji_laughing_fill::BS_EmojiLaughingFill;
pub use self::projector::BS_Projector;
pub use self::flag_fill::BS_FlagFill;
pub use self::body_text::BS_BodyText;
pub use self::magic::BS_Magic;
pub use self::box::BS_Box;
pub use self::indent::BS_Indent;
pub use self::people_fill::BS_PeopleFill;
pub use self::file_earmark_slides_fill::BS_FileEarmarkSlidesFill;
pub use self::file_bar_graph_fill::BS_FileBarGraphFill;
pub use self::funnel::BS_Funnel;
pub use self::house_dash::BS_HouseDash;
pub use self::stoplights::BS_Stoplights;
pub use self::stop_circle_fill::BS_StopCircleFill;
pub use self::key_fill::BS_KeyFill;
pub use self::train_lightrail_front_fill::BS_TrainLightrailFrontFill;
pub use self::steam::BS_Steam;
pub use self::bezier2::BS_Bezier2;
pub use self::camera_video_fill::BS_CameraVideoFill;
pub use self::gear_fill::BS_GearFill;
pub use self::cup_hot_fill::BS_CupHotFill;
pub use self::list_columns::BS_ListColumns;
pub use self::x_diamond_fill::BS_XDiamondFill;
pub use self::sort_alpha_down::BS_SortAlphaDown;
pub use self::file_lock_fill::BS_FileLockFill;
pub use self::justify::BS_Justify;
pub use self::pip::BS_Pip;
pub use self::layers::BS_Layers;
pub use self::building_fill_add::BS_BuildingFillAdd;
pub use self::box_arrow_down_right::BS_BoxArrowDownRight;
pub use self::border_center::BS_BorderCenter;
pub use self::diagram_3::BS_Diagram3;
pub use self::device_hdd_fill::BS_DeviceHddFill;
pub use self::brightness_alt_low::BS_BrightnessAltLow;
pub use self::chat::BS_Chat;
pub use self::telephone_outbound_fill::BS_TelephoneOutboundFill;
pub use self::caret_right_square_fill::BS_CaretRightSquareFill;
pub use self::broadcast_pin::BS_BroadcastPin;
pub use self::x_circle_fill::BS_XCircleFill;
pub use self::skip_end_btn_fill::BS_SkipEndBtnFill;
pub use self::lungs::BS_Lungs;
pub use self::calendar2_plus_fill::BS_Calendar2PlusFill;
pub use self::emoji_sunglasses::BS_EmojiSunglasses;
pub use self::clipboard2_check_fill::BS_Clipboard2CheckFill;
pub use self::voicemail::BS_Voicemail;
pub use self::arrow_up_left_square_fill::BS_ArrowUpLeftSquareFill;
pub use self::patch_exclamation::BS_PatchExclamation;
pub use self::easel2_fill::BS_Easel2Fill;
pub use self::filetype_key::BS_FiletypeKey;
pub use self::filetype_svg::BS_FiletypeSvg;
pub use self::search_heart_fill::BS_SearchHeartFill;
pub use self::file_earmark_arrow_up_fill::BS_FileEarmarkArrowUpFill;
pub use self::circle_half::BS_CircleHalf;
pub use self::align_top::BS_AlignTop;
pub use self::arrows_collapse::BS_ArrowsCollapse;
pub use self::building_fill_slash::BS_BuildingFillSlash;
pub use self::slash_lg::BS_SlashLg;
pub use self::envelope_check_fill::BS_EnvelopeCheckFill;
pub use self::file_earmark_play_fill::BS_FileEarmarkPlayFill;
pub use self::badge_ad::BS_BadgeAd;
pub use self::file_earmark_person_fill::BS_FileEarmarkPersonFill;
pub use self::house_gear_fill::BS_HouseGearFill;
pub use self::git::BS_Git;
pub use self::bell_fill::BS_BellFill;
pub use self::activity::BS_Activity;
pub use self::chat_left_dots::BS_ChatLeftDots;
pub use self::shield_lock::BS_ShieldLock;
pub use self::capslock::BS_Capslock;
pub use self::arrow_up_right_square::BS_ArrowUpRightSquare;
pub use self::node_plus::BS_NodePlus;
pub use self::2_square::BS_2Square;
pub use self::caret_right::BS_CaretRight;
pub use self::building_lock::BS_BuildingLock;
pub use self::usb_fill::BS_UsbFill;
pub use self::postage_heart::BS_PostageHeart;
pub use self::coin::BS_Coin;
pub use self::chevron_double_up::BS_ChevronDoubleUp;
pub use self::grid_3x3_gap::BS_Grid3x3Gap;
pub use self::clouds::BS_Clouds;
pub use self::folder2_open::BS_Folder2Open;
pub use self::filter_square::BS_FilterSquare;
pub use self::hexagon::BS_Hexagon;
pub use self::heptagon_half::BS_HeptagonHalf;
pub use self::dash_circle_fill::BS_DashCircleFill;
pub use self::file_easel::BS_FileEasel;
pub use self::grid_1x2::BS_Grid1x2;
pub use self::hdd::BS_Hdd;
pub use self::question_octagon::BS_QuestionOctagon;
pub use self::send_dash::BS_SendDash;
pub use self::bag_plus_fill::BS_BagPlusFill;
pub use self::file_earmark_bar_graph::BS_FileEarmarkBarGraph;
pub use self::ui_checks::BS_UiChecks;
pub use self::file_spreadsheet::BS_FileSpreadsheet;
pub use self::bug::BS_Bug;
pub use self::arrow_repeat::BS_ArrowRepeat;
pub use self::dice_5::BS_Dice5;
pub use self::3_circle::BS_3Circle;
pub use self::pause_btn::BS_PauseBtn;
pub use self::images::BS_Images;
pub use self::sign_no_left_turn_fill::BS_SignNoLeftTurnFill;
pub use self::calendar_x::BS_CalendarX;
pub use self::filetype_png::BS_FiletypePng;
pub use self::clock::BS_Clock;
pub use self::airplane_engines::BS_AirplaneEngines;
pub use self::emoji_heart_eyes::BS_EmojiHeartEyes;
pub use self::book::BS_Book;
pub use self::sliders2_vertical::BS_Sliders2Vertical;
pub use self::credit_card_2_front::BS_CreditCard2Front;
pub use self::file_earmark_word::BS_FileEarmarkWord;
pub use self::filetype_csv::BS_FiletypeCsv;
pub use self::bar_chart_line_fill::BS_BarChartLineFill;
pub use self::hurricane::BS_Hurricane;
pub use self::envelope::BS_Envelope;
pub use self::octagon_fill::BS_OctagonFill;
pub use self::envelope_slash::BS_EnvelopeSlash;
pub use self::file_earmark_spreadsheet_fill::BS_FileEarmarkSpreadsheetFill;
pub use self::box_arrow_down::BS_BoxArrowDown;
pub use self::building_fill_up::BS_BuildingFillUp;
pub use self::cloud_slash::BS_CloudSlash;
pub use self::clipboard_x_fill::BS_ClipboardXFill;
pub use self::arrow_left_right::BS_ArrowLeftRight;
pub use self::square::BS_Square;
pub use self::filter_left::BS_FilterLeft;
pub use self::ladder::BS_Ladder;
pub use self::file_code_fill::BS_FileCodeFill;
pub use self::badge_4k_fill::BS_Badge4kFill;
pub use self::umbrella::BS_Umbrella;
pub use self::arrow_down_left_circle::BS_ArrowDownLeftCircle;
pub use self::braces_asterisk::BS_BracesAsterisk;
pub use self::browser_edge::BS_BrowserEdge;
pub use self::pen::BS_Pen;
pub use self::bootstrap_reboot::BS_BootstrapReboot;
pub use self::smartwatch::BS_Smartwatch;
pub use self::peace::BS_Peace;
pub use self::person_badge::BS_PersonBadge;
pub use self::x_octagon_fill::BS_XOctagonFill;
pub use self::credit_card_2_back_fill::BS_CreditCard2BackFill;
pub use self::border_width::BS_BorderWidth;
pub use self::toggle_off::BS_ToggleOff;
pub use self::basket2_fill::BS_Basket2Fill;
pub use self::person_x_fill::BS_PersonXFill;
pub use self::cloud_drizzle_fill::BS_CloudDrizzleFill;
pub use self::wallet2::BS_Wallet2;
pub use self::forward::BS_Forward;
pub use self::android2::BS_Android2;
pub use self::h_circle::BS_HCircle;
pub use self::cup_fill::BS_CupFill;
pub use self::calendar_range::BS_CalendarRange;
pub use self::signpost_split_fill::BS_SignpostSplitFill;
pub use self::emoji_neutral::BS_EmojiNeutral;
pub use self::dash_square_dotted::BS_DashSquareDotted;
pub use self::person_circle::BS_PersonCircle;
pub use self::skip_end::BS_SkipEnd;
pub use self::eject_fill::BS_EjectFill;
pub use self::folder_check::BS_FolderCheck;
pub use self::file_break_fill::BS_FileBreakFill;
pub use self::sign_turn_slight_right::BS_SignTurnSlightRight;
pub use self::vinyl::BS_Vinyl;
pub use self::calendar2_x::BS_Calendar2X;
pub use self::collection_fill::BS_CollectionFill;
pub use self::moisture::BS_Moisture;
pub use self::slash_square::BS_SlashSquare;
pub use self::image::BS_Image;
pub use self::clipboard2_heart::BS_Clipboard2Heart;
pub use self::rainbow::BS_Rainbow;
pub use self::valentine::BS_Valentine;
pub use self::house_lock::BS_HouseLock;
pub use self::exclamation_diamond::BS_ExclamationDiamond;
pub use self::menu_button::BS_MenuButton;
pub use self::2_circle_fill::BS_2CircleFill;
pub use self::envelope_x_fill::BS_EnvelopeXFill;
pub use self::backspace::BS_Backspace;
pub use self::badge_ar_fill::BS_BadgeArFill;
pub use self::projector_fill::BS_ProjectorFill;
pub use self::arrow_up_left_square::BS_ArrowUpLeftSquare;
pub use self::skip_backward_btn::BS_SkipBackwardBtn;
pub use self::file_plus_fill::BS_FilePlusFill;
pub use self::align_middle::BS_AlignMiddle;
pub use self::pin_angle::BS_PinAngle;
pub use self::calendar2_fill::BS_Calendar2Fill;
pub use self::sunrise::BS_Sunrise;
pub use self::badge_3d::BS_Badge3d;
pub use self::bag_fill::BS_BagFill;
pub use self::safe::BS_Safe;
pub use self::7_circle::BS_7Circle;
pub use self::clipboard2_minus_fill::BS_Clipboard2MinusFill;
pub use self::file_earmark_code::BS_FileEarmarkCode;
pub use self::file_ppt::BS_FilePpt;
pub use self::filetype_json::BS_FiletypeJson;
pub use self::calendar2_week::BS_Calendar2Week;
pub use self::unity::BS_Unity;
pub use self::window_fullscreen::BS_WindowFullscreen;
pub use self::train_freight_front::BS_TrainFreightFront;
pub use self::file_earmark_richtext_fill::BS_FileEarmarkRichtextFill;
pub use self::badge_tm::BS_BadgeTm;
pub use self::back::BS_Back;
pub use self::skip_start::BS_SkipStart;
pub use self::chat_quote_fill::BS_ChatQuoteFill;
pub use self::fast_forward_circle_fill::BS_FastForwardCircleFill;
pub use self::bucket_fill::BS_BucketFill;
pub use self::sd_card_fill::BS_SdCardFill;
pub use self::playstation::BS_Playstation;
pub use self::file_earmark_medical_fill::BS_FileEarmarkMedicalFill;
pub use self::lock_fill::BS_LockFill;
pub use self::calendar2_heart::BS_Calendar2Heart;
pub use self::x_square_fill::BS_XSquareFill;
pub use self::pin::BS_Pin;
pub use self::folder::BS_Folder;
pub use self::arrow_up_short::BS_ArrowUpShort;
pub use self::calendar2_minus::BS_Calendar2Minus;
pub use self::file_arrow_up::BS_FileArrowUp;
pub use self::line::BS_Line;
pub use self::hdmi_fill::BS_HdmiFill;
pub use self::tiktok::BS_Tiktok;
pub use self::square_fill::BS_SquareFill;
pub use self::heptagon::BS_Heptagon;
pub use self::question_square_fill::BS_QuestionSquareFill;
pub use self::dpad_fill::BS_DpadFill;
pub use self::box_fill::BS_BoxFill;
pub use self::usb_mini_fill::BS_UsbMiniFill;
pub use self::cart_dash_fill::BS_CartDashFill;
pub use self::person_fill_x::BS_PersonFillX;
pub use self::trash::BS_Trash;
pub use self::question_diamond::BS_QuestionDiamond;
pub use self::egg::BS_Egg;
pub use self::headset::BS_Headset;
pub use self::currency_euro::BS_CurrencyEuro;
pub use self::journal_check::BS_JournalCheck;
pub use self::calendar2_month::BS_Calendar2Month;
pub use self::box_arrow_up_right::BS_BoxArrowUpRight;
pub use self::sign_turn_left_fill::BS_SignTurnLeftFill;
pub use self::file_earmark_font::BS_FileEarmarkFont;
pub use self::shield_slash::BS_ShieldSlash;
pub use self::skip_forward_circle::BS_SkipForwardCircle;
pub use self::watch::BS_Watch;
pub use self::arrow_up_left::BS_ArrowUpLeft;
pub use self::calendar2::BS_Calendar2;
pub use self::envelope_fill::BS_EnvelopeFill;
pub use self::skype::BS_Skype;
pub use self::r_circle::BS_RCircle;
pub use self::calendar2_event_fill::BS_Calendar2EventFill;
pub use self::postage_fill::BS_PostageFill;
pub use self::usb_c_fill::BS_UsbCFill;
pub use self::skip_backward_btn_fill::BS_SkipBackwardBtnFill;
pub use self::record_circle::BS_RecordCircle;
pub use self::building_fill_down::BS_BuildingFillDown;
pub use self::gender_male::BS_GenderMale;
pub use self::hr::BS_Hr;
pub use self::cloud_arrow_up_fill::BS_CloudArrowUpFill;
pub use self::meta::BS_Meta;
pub use self::pie_chart_fill::BS_PieChartFill;
pub use self::r_circle_fill::BS_RCircleFill;
pub use self::taxi_front_fill::BS_TaxiFrontFill;
pub use self::star_fill::BS_StarFill;
pub use self::calendar4::BS_Calendar4;
pub use self::sign_intersection::BS_SignIntersection;
pub use self::caret_right_square::BS_CaretRightSquare;
pub use self::patch_question_fill::BS_PatchQuestionFill;
pub use self::x::BS_X;
pub use self::record::BS_Record;
pub use self::cloud_moon_fill::BS_CloudMoonFill;
pub use self::hdd_network_fill::BS_HddNetworkFill;
pub use self::train_front_fill::BS_TrainFrontFill;
pub use self::list_check::BS_ListCheck;
pub use self::cloud_rain::BS_CloudRain;
pub use self::building_exclamation::BS_BuildingExclamation;
pub use self::play_fill::BS_PlayFill;
pub use self::sign_dead_end_fill::BS_SignDeadEndFill;
pub use self::shield_minus::BS_ShieldMinus;
pub use self::hdd_rack::BS_HddRack;
pub use self::cloud_arrow_down_fill::BS_CloudArrowDownFill;
pub use self::database_x::BS_DatabaseX;
pub use self::cloud_plus_fill::BS_CloudPlusFill;
pub use self::sort_numeric_up::BS_SortNumericUp;
pub use self::archive::BS_Archive;
pub use self::border_all::BS_BorderAll;
pub use self::calendar_date_fill::BS_CalendarDateFill;
pub use self::phone_flip::BS_PhoneFlip;
pub use self::person_up::BS_PersonUp;
pub use self::cart_x_fill::BS_CartXFill;
pub use self::palette_fill::BS_PaletteFill;
pub use self::snow::BS_Snow;
pub use self::suit_diamond::BS_SuitDiamond;
pub use self::calendar_event::BS_CalendarEvent;
pub use self::grid::BS_Grid;
pub use self::pc_horizontal::BS_PcHorizontal;
pub use self::x_diamond::BS_XDiamond;
pub use self::speedometer2::BS_Speedometer2;
pub use self::sign_intersection_fill::BS_SignIntersectionFill;
pub use self::hash::BS_Hash;
pub use self::exclamation_square::BS_ExclamationSquare;
pub use self::emoji_smile::BS_EmojiSmile;
pub use self::skip_start_circle_fill::BS_SkipStartCircleFill;
pub use self::door_closed_fill::BS_DoorClosedFill;
pub use self::chevron_compact_right::BS_ChevronCompactRight;
pub use self::currency_pound::BS_CurrencyPound;
pub use self::file_fill::BS_FileFill;
pub use self::boombox::BS_Boombox;
pub use self::type_italic::BS_TypeItalic;
pub use self::toggles2::BS_Toggles2;
pub use self::briefcase_fill::BS_BriefcaseFill;
pub use self::modem_fill::BS_ModemFill;
pub use self::calendar::BS_Calendar;
pub use self::layers_half::BS_LayersHalf;
pub use self::wallet_fill::BS_WalletFill;
pub use self::send_plus_fill::BS_SendPlusFill;
pub use self::border_right::BS_BorderRight;
pub use self::person_fill_add::BS_PersonFillAdd;
pub use self::grid_fill::BS_GridFill;
pub use self::pencil_square::BS_PencilSquare;
pub use self::badge_hd::BS_BadgeHd;
pub use self::chat_left::BS_ChatLeft;
pub use self::8_circle::BS_8Circle;
pub use self::file_check_fill::BS_FileCheckFill;
pub use self::tags::BS_Tags;
pub use self::record_fill::BS_RecordFill;
pub use self::cart4::BS_Cart4;
pub use self::check_lg::BS_CheckLg;
pub use self::chat_fill::BS_ChatFill;
pub use self::calendar_minus_fill::BS_CalendarMinusFill;
pub use self::chevron_bar_contract::BS_ChevronBarContract;
pub use self::alarm_fill::BS_AlarmFill;
pub use self::cloud_download::BS_CloudDownload;
pub use self::usb::BS_Usb;
pub use self::diagram_3_fill::BS_Diagram3Fill;
pub use self::cc_circle::BS_CcCircle;
pub use self::textarea_t::BS_TextareaT;
pub use self::calculator_fill::BS_CalculatorFill;
pub use self::bus_front::BS_BusFront;
pub use self::sun::BS_Sun;
pub use self::question_circle::BS_QuestionCircle;
pub use self::cloud_plus::BS_CloudPlus;
pub use self::search_heart::BS_SearchHeart;
pub use self::hand_index_thumb_fill::BS_HandIndexThumbFill;
pub use self::inboxes::BS_Inboxes;
pub use self::square_half::BS_SquareHalf;
pub use self::menu_button_wide_fill::BS_MenuButtonWideFill;
pub use self::door_open::BS_DoorOpen;
pub use self::ticket::BS_Ticket;
pub use self::currency_yen::BS_CurrencyYen;
pub use self::stack::BS_Stack;
pub use self::download::BS_Download;
pub use self::clipboard2_x::BS_Clipboard2X;
pub use self::markdown_fill::BS_MarkdownFill;
pub use self::forward_fill::BS_ForwardFill;
pub use self::binoculars::BS_Binoculars;
pub use self::play::BS_Play;
pub use self::clipboard2_check::BS_Clipboard2Check;
pub use self::stop_btn::BS_StopBtn;
pub use self::plus_square::BS_PlusSquare;
pub use self::gpu_card::BS_GpuCard;
pub use self::lightning_fill::BS_LightningFill;
pub use self::reply_all::BS_ReplyAll;
pub use self::ui_checks_grid::BS_UiChecksGrid;
pub use self::clipboard_data_fill::BS_ClipboardDataFill;
pub use self::record_btn::BS_RecordBtn;
pub use self::8_square::BS_8Square;
pub use self::share_fill::BS_ShareFill;
pub use self::sort_numeric_up_alt::BS_SortNumericUpAlt;
pub use self::chat_square_quote_fill::BS_ChatSquareQuoteFill;
pub use self::bus_front_fill::BS_BusFrontFill;
pub use self::mouse2::BS_Mouse2;
pub use self::ticket_perforated::BS_TicketPerforated;
pub use self::chat_left_heart::BS_ChatLeftHeart;
pub use self::rewind_circle_fill::BS_RewindCircleFill;
pub use self::calendar3_range::BS_Calendar3Range;
pub use self::app_indicator::BS_AppIndicator;
pub use self::alarm::BS_Alarm;
pub use self::calendar_fill::BS_CalendarFill;
pub use self::window_plus::BS_WindowPlus;
pub use self::chat_square::BS_ChatSquare;
pub use self::github::BS_Github;
pub use self::hand_thumbs_down::BS_HandThumbsDown;
pub use self::bookmarks::BS_Bookmarks;
pub use self::flag::BS_Flag;
pub use self::shield_fill_exclamation::BS_ShieldFillExclamation;
pub use self::piggy_bank::BS_PiggyBank;
pub use self::record_btn_fill::BS_RecordBtnFill;
pub use self::device_ssd::BS_DeviceSsd;
pub use self::person_fill_gear::BS_PersonFillGear;
pub use self::sign_stop_lights::BS_SignStopLights;
pub use self::0_square::BS_0Square;
pub use self::envelope_dash::BS_EnvelopeDash;
pub use self::list::BS_List;
pub use self::list_columns_reverse::BS_ListColumnsReverse;
pub use self::command::BS_Command;
pub use self::emoji_frown_fill::BS_EmojiFrownFill;
pub use self::chevron_bar_right::BS_ChevronBarRight;
pub use self::house_slash::BS_HouseSlash;
pub use self::clipboard_fill::BS_ClipboardFill;
pub use self::input_cursor::BS_InputCursor;
pub use self::arrows_fullscreen::BS_ArrowsFullscreen;
pub use self::skip_end_circle::BS_SkipEndCircle;
pub use self::cast::BS_Cast;
pub use self::tencent_qq::BS_TencentQq;
pub use self::calendar2_date::BS_Calendar2Date;
pub use self::cc_square_fill::BS_CcSquareFill;
pub use self::telephone_inbound::BS_TelephoneInbound;
pub use self::hexagon_half::BS_HexagonHalf;
pub use self::megaphone::BS_Megaphone;
pub use self::chat_square_fill::BS_ChatSquareFill;
pub use self::filetype_yml::BS_FiletypeYml;
pub use self::dice_5_fill::BS_Dice5Fill;
pub use self::patch_check_fill::BS_PatchCheckFill;
pub use self::box_seam::BS_BoxSeam;
pub use self::align_end::BS_AlignEnd;
pub use self::badge_sd::BS_BadgeSd;
pub use self::inboxes_fill::BS_InboxesFill;
pub use self::camera_video_off::BS_CameraVideoOff;
pub use self::clipboard_minus::BS_ClipboardMinus;
pub use self::cloud_check::BS_CloudCheck;
pub use self::fast_forward::BS_FastForward;
pub use self::journal_code::BS_JournalCode;
pub use self::cc_circle_fill::BS_CcCircleFill;
pub use self::telephone_x_fill::BS_TelephoneXFill;
pub use self::wallet::BS_Wallet;
pub use self::chevron_compact_down::BS_ChevronCompactDown;
pub use self::printer::BS_Printer;
pub use self::person_workspace::BS_PersonWorkspace;
pub use self::calendar2_day_fill::BS_Calendar2DayFill;
pub use self::car_front_fill::BS_CarFrontFill;
pub use self::stickies::BS_Stickies;
pub use self::text_wrap::BS_TextWrap;
pub use self::thunderbolt_fill::BS_ThunderboltFill;
pub use self::box_arrow_in_down_right::BS_BoxArrowInDownRight;
pub use self::house_x::BS_HouseX;
pub use self::filter_circle_fill::BS_FilterCircleFill;
pub use self::justify_left::BS_JustifyLeft;
pub use self::window_stack::BS_WindowStack;
pub use self::fast_forward_btn_fill::BS_FastForwardBtnFill;
pub use self::file_binary_fill::BS_FileBinaryFill;
pub use self::sign_no_right_turn::BS_SignNoRightTurn;
pub use self::badge_8k_fill::BS_Badge8kFill;
pub use self::postcard::BS_Postcard;
pub use self::5_square::BS_5Square;
pub use self::list_task::BS_ListTask;
pub use self::clipboard_plus::BS_ClipboardPlus;
pub use self::journal_arrow_down::BS_JournalArrowDown;
pub use self::eye_slash::BS_EyeSlash;
pub use self::cloud_lightning::BS_CloudLightning;
pub use self::file_earmark_post::BS_FileEarmarkPost;
pub use self::bag_check_fill::BS_BagCheckFill;
pub use self::spotify::BS_Spotify;
pub use self::superscript::BS_Superscript;
pub use self::heptagon_fill::BS_HeptagonFill;
pub use self::h_square_fill::BS_HSquareFill;
pub use self::align_center::BS_AlignCenter;
pub use self::patch_minus::BS_PatchMinus;
pub use self::mask::BS_Mask;
pub use self::cloud_upload::BS_CloudUpload;
pub use self::snow3::BS_Snow3;
pub use self::minecart_loaded::BS_MinecartLoaded;
pub use self::cloud_minus::BS_CloudMinus;
pub use self::thermometer_high::BS_ThermometerHigh;
pub use self::terminal_dash::BS_TerminalDash;
pub use self::three_dots::BS_ThreeDots;
pub use self::arrow_bar_down::BS_ArrowBarDown;
pub use self::thunderbolt::BS_Thunderbolt;
pub use self::thermometer_snow::BS_ThermometerSnow;
pub use self::microsoft::BS_Microsoft;
pub use self::telephone_forward_fill::BS_TelephoneForwardFill;
pub use self::caret_up_fill::BS_CaretUpFill;
pub use self::lightning::BS_Lightning;
pub use self::rewind_circle::BS_RewindCircle;
pub use self::clipboard2::BS_Clipboard2;
pub use self::database_fill_up::BS_DatabaseFillUp;
pub use self::cup_hot::BS_CupHot;
pub use self::usb_micro::BS_UsbMicro;
pub use self::building_x::BS_BuildingX;
pub use self::cloud_lightning_rain_fill::BS_CloudLightningRainFill;
pub use self::union::BS_Union;
pub use self::signpost_2::BS_Signpost2;


#[function_component(BSIcon)]
pub fn r#md_icon(props: &IconProps) -> Html {
  let owned_props = props.clone();
  match owned_props.name {
    implicit_clone::unsync::IString::Static("Check2Circle") => html! {
      <MD_Check2Circle ..owned_props />
    },
    implicit_clone::unsync::IString::Static("HouseUp") => html! {
      <MD_HouseUp ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ArrowDownLeftSquareFill") => html! {
      <MD_ArrowDownLeftSquareFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FiletypeExe") => html! {
      <MD_FiletypeExe ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Tornado") => html! {
      <MD_Tornado ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ViewStacked") => html! {
      <MD_ViewStacked ..owned_props />
    },
    implicit_clone::unsync::IString::Static("DropletHalf") => html! {
      <MD_DropletHalf ..owned_props />
    },
    implicit_clone::unsync::IString::Static("TrashFill") => html! {
      <MD_TrashFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("NodePlusFill") => html! {
      <MD_NodePlusFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ArrowUpLeftCircle") => html! {
      <MD_ArrowUpLeftCircle ..owned_props />
    },
    implicit_clone::unsync::IString::Static("PentagonHalf") => html! {
      <MD_PentagonHalf ..owned_props />
    },
    implicit_clone::unsync::IString::Static("DatabaseFill") => html! {
      <MD_DatabaseFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FileEarmarkArrowUp") => html! {
      <MD_FileEarmarkArrowUp ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Buildings") => html! {
      <MD_Buildings ..owned_props />
    },
    implicit_clone::unsync::IString::Static("5Circle") => html! {
      <MD_5Circle ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FiletypeGif") => html! {
      <MD_FiletypeGif ..owned_props />
    },
    implicit_clone::unsync::IString::Static("BorderMiddle") => html! {
      <MD_BorderMiddle ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ExplicitFill") => html! {
      <MD_ExplicitFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Scooter") => html! {
      <MD_Scooter ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Compass") => html! {
      <MD_Compass ..owned_props />
    },
    implicit_clone::unsync::IString::Static("CalendarDate") => html! {
      <MD_CalendarDate ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Youtube") => html! {
      <MD_Youtube ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ChatLeftQuoteFill") => html! {
      <MD_ChatLeftQuoteFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("CloudFog2Fill") => html! {
      <MD_CloudFog2Fill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("7CircleFill") => html! {
      <MD_7CircleFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("PlusCircle") => html! {
      <MD_PlusCircle ..owned_props />
    },
    implicit_clone::unsync::IString::Static("CaretUpSquare") => html! {
      <MD_CaretUpSquare ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Explicit") => html! {
      <MD_Explicit ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Check2Square") => html! {
      <MD_Check2Square ..owned_props />
    },
    implicit_clone::unsync::IString::Static("CSquare") => html! {
      <MD_CSquare ..owned_props />
    },
    implicit_clone::unsync::IString::Static("PatchCheck") => html! {
      <MD_PatchCheck ..owned_props />
    },
    implicit_clone::unsync::IString::Static("CaretUp") => html! {
      <MD_CaretUp ..owned_props />
    },
    implicit_clone::unsync::IString::Static("BarChartSteps") => html! {
      <MD_BarChartSteps ..owned_props />
    },
    implicit_clone::unsync::IString::Static("CardChecklist") => html! {
      <MD_CardChecklist ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Badge4k") => html! {
      <MD_Badge4k ..owned_props />
    },
    implicit_clone::unsync::IString::Static("CCircleFill") => html! {
      <MD_CCircleFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FileEarmarkDiffFill") => html! {
      <MD_FileEarmarkDiffFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FileImageFill") => html! {
      <MD_FileImageFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("HouseSlashFill") => html! {
      <MD_HouseSlashFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("TicketPerforatedFill") => html! {
      <MD_TicketPerforatedFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("PCircleFill") => html! {
      <MD_PCircleFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("StarHalf") => html! {
      <MD_StarHalf ..owned_props />
    },
    implicit_clone::unsync::IString::Static("CloudHailFill") => html! {
      <MD_CloudHailFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("HouseCheck") => html! {
      <MD_HouseCheck ..owned_props />
    },
    implicit_clone::unsync::IString::Static("CalendarWeekFill") => html! {
      <MD_CalendarWeekFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("JournalMedical") => html! {
      <MD_JournalMedical ..owned_props />
    },
    implicit_clone::unsync::IString::Static("CloudFog2") => html! {
      <MD_CloudFog2 ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FiletypeWoff") => html! {
      <MD_FiletypeWoff ..owned_props />
    },
    implicit_clone::unsync::IString::Static("BookmarkPlus") => html! {
      <MD_BookmarkPlus ..owned_props />
    },
    implicit_clone::unsync::IString::Static("VolumeUpFill") => html! {
      <MD_VolumeUpFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FileEarmarkEasel") => html! {
      <MD_FileEarmarkEasel ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Dribbble") => html! {
      <MD_Dribbble ..owned_props />
    },
    implicit_clone::unsync::IString::Static("SunsetFill") => html! {
      <MD_SunsetFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("PersonFillLock") => html! {
      <MD_PersonFillLock ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Keyboard") => html! {
      <MD_Keyboard ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FileEarmarkImageFill") => html! {
      <MD_FileEarmarkImageFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FileEarmarkPdf") => html! {
      <MD_FileEarmarkPdf ..owned_props />
    },
    implicit_clone::unsync::IString::Static("SortAlphaDownAlt") => html! {
      <MD_SortAlphaDownAlt ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ChatSquareTextFill") => html! {
      <MD_ChatSquareTextFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Easel2") => html! {
      <MD_Easel2 ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Inbox") => html! {
      <MD_Inbox ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ArrowUpRightCircle") => html! {
      <MD_ArrowUpRightCircle ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FiletypeTxt") => html! {
      <MD_FiletypeTxt ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FiletypeSql") => html! {
      <MD_FiletypeSql ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Toggles") => html! {
      <MD_Toggles ..owned_props />
    },
    implicit_clone::unsync::IString::Static("SortAlphaUpAlt") => html! {
      <MD_SortAlphaUpAlt ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Trash3Fill") => html! {
      <MD_Trash3Fill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("BalloonHeart") => html! {
      <MD_BalloonHeart ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ClipboardHeartFill") => html! {
      <MD_ClipboardHeartFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FileMedical") => html! {
      <MD_FileMedical ..owned_props />
    },
    implicit_clone::unsync::IString::Static("BuildingFillGear") => html! {
      <MD_BuildingFillGear ..owned_props />
    },
    implicit_clone::unsync::IString::Static("TrainFront") => html! {
      <MD_TrainFront ..owned_props />
    },
    implicit_clone::unsync::IString::Static("HousesFill") => html! {
      <MD_HousesFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Thermometer") => html! {
      <MD_Thermometer ..owned_props />
    },
    implicit_clone::unsync::IString::Static("0Circle") => html! {
      <MD_0Circle ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FileEarmarkBinary") => html! {
      <MD_FileEarmarkBinary ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ChatLeftTextFill") => html! {
      <MD_ChatLeftTextFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Bicycle") => html! {
      <MD_Bicycle ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Translate") => html! {
      <MD_Translate ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FileEarmarkPlus") => html! {
      <MD_FileEarmarkPlus ..owned_props />
    },
    implicit_clone::unsync::IString::Static("BookmarkFill") => html! {
      <MD_BookmarkFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ChevronLeft") => html! {
      <MD_ChevronLeft ..owned_props />
    },
    implicit_clone::unsync::IString::Static("EraserFill") => html! {
      <MD_EraserFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("XOctagon") => html! {
      <MD_XOctagon ..owned_props />
    },
    implicit_clone::unsync::IString::Static("TagFill") => html! {
      <MD_TagFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FileEarmarkPlay") => html! {
      <MD_FileEarmarkPlay ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Trash2") => html! {
      <MD_Trash2 ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Hourglass") => html! {
      <MD_Hourglass ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Question") => html! {
      <MD_Question ..owned_props />
    },
    implicit_clone::unsync::IString::Static("LifePreserver") => html! {
      <MD_LifePreserver ..owned_props />
    },
    implicit_clone::unsync::IString::Static("BookmarkStar") => html! {
      <MD_BookmarkStar ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ConeStriped") => html! {
      <MD_ConeStriped ..owned_props />
    },
    implicit_clone::unsync::IString::Static("QuestionCircleFill") => html! {
      <MD_QuestionCircleFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ExclamationOctagonFill") => html! {
      <MD_ExclamationOctagonFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("DashLg") => html! {
      <MD_DashLg ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FiletypeJpg") => html! {
      <MD_FiletypeJpg ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Dice2") => html! {
      <MD_Dice2 ..owned_props />
    },
    implicit_clone::unsync::IString::Static("SignMergeRightFill") => html! {
      <MD_SignMergeRightFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Telephone") => html! {
      <MD_Telephone ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Hammer") => html! {
      <MD_Hammer ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Twitch") => html! {
      <MD_Twitch ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FileSlides") => html! {
      <MD_FileSlides ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Eraser") => html! {
      <MD_Eraser ..owned_props />
    },
    implicit_clone::unsync::IString::Static("VolumeMuteFill") => html! {
      <MD_VolumeMuteFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("HouseCheckFill") => html! {
      <MD_HouseCheckFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FiletypeOtf") => html! {
      <MD_FiletypeOtf ..owned_props />
    },
    implicit_clone::unsync::IString::Static("CreditCard2FrontFill") => html! {
      <MD_CreditCard2FrontFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Tv") => html! {
      <MD_Tv ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FileEarmarkExcelFill") => html! {
      <MD_FileEarmarkExcelFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("EmojiSmileUpsideDown") => html! {
      <MD_EmojiSmileUpsideDown ..owned_props />
    },
    implicit_clone::unsync::IString::Static("TypeH2") => html! {
      <MD_TypeH2 ..owned_props />
    },
    implicit_clone::unsync::IString::Static("PSquareFill") => html! {
      <MD_PSquareFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("LayerForward") => html! {
      <MD_LayerForward ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Cursor") => html! {
      <MD_Cursor ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FolderMinus") => html! {
      <MD_FolderMinus ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Eyedropper") => html! {
      <MD_Eyedropper ..owned_props />
    },
    implicit_clone::unsync::IString::Static("TicketFill") => html! {
      <MD_TicketFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("StopwatchFill") => html! {
      <MD_StopwatchFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Cup") => html! {
      <MD_Cup ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Nvidia") => html! {
      <MD_Nvidia ..owned_props />
    },
    implicit_clone::unsync::IString::Static("BookmarkHeartFill") => html! {
      <MD_BookmarkHeartFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("TicketDetailed") => html! {
      <MD_TicketDetailed ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Paypal") => html! {
      <MD_Paypal ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Grid3x2") => html! {
      <MD_Grid3x2 ..owned_props />
    },
    implicit_clone::unsync::IString::Static("CCircle") => html! {
      <MD_CCircle ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Mortarboard") => html! {
      <MD_Mortarboard ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ChatSquareHeart") => html! {
      <MD_ChatSquareHeart ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Terminal") => html! {
      <MD_Terminal ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FolderSymlink") => html! {
      <MD_FolderSymlink ..owned_props />
    },
    implicit_clone::unsync::IString::Static("PersonHeart") => html! {
      <MD_PersonHeart ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ArrowRightShort") => html! {
      <MD_ArrowRightShort ..owned_props />
    },
    implicit_clone::unsync::IString::Static("JournalPlus") => html! {
      <MD_JournalPlus ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FileX") => html! {
      <MD_FileX ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Pass") => html! {
      <MD_Pass ..owned_props />
    },
    implicit_clone::unsync::IString::Static("EnvelopePlus") => html! {
      <MD_EnvelopePlus ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Wifi1") => html! {
      <MD_Wifi1 ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FilesAlt") => html! {
      <MD_FilesAlt ..owned_props />
    },
    implicit_clone::unsync::IString::Static("TruckFlatbed") => html! {
      <MD_TruckFlatbed ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ChatLeftQuote") => html! {
      <MD_ChatLeftQuote ..owned_props />
    },
    implicit_clone::unsync::IString::Static("HouseGear") => html! {
      <MD_HouseGear ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FolderSymlinkFill") => html! {
      <MD_FolderSymlinkFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Diagram2") => html! {
      <MD_Diagram2 ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FiletypePsd") => html! {
      <MD_FiletypePsd ..owned_props />
    },
    implicit_clone::unsync::IString::Static("PersonPlus") => html! {
      <MD_PersonPlus ..owned_props />
    },
    implicit_clone::unsync::IString::Static("EmojiSmileFill") => html! {
      <MD_EmojiSmileFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("RewindBtn") => html! {
      <MD_RewindBtn ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ChevronExpand") => html! {
      <MD_ChevronExpand ..owned_props />
    },
    implicit_clone::unsync::IString::Static("WindowDash") => html! {
      <MD_WindowDash ..owned_props />
    },
    implicit_clone::unsync::IString::Static("CloudRainHeavy") => html! {
      <MD_CloudRainHeavy ..owned_props />
    },
    implicit_clone::unsync::IString::Static("SkipEndBtn") => html! {
      <MD_SkipEndBtn ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FileFontFill") => html! {
      <MD_FileFontFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Prescription") => html! {
      <MD_Prescription ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Handbag") => html! {
      <MD_Handbag ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Ear") => html! {
      <MD_Ear ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Xbox") => html! {
      <MD_Xbox ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Plugin") => html! {
      <MD_Plugin ..owned_props />
    },
    implicit_clone::unsync::IString::Static("BrightnessAltLowFill") => html! {
      <MD_BrightnessAltLowFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ChatRightHeartFill") => html! {
      <MD_ChatRightHeartFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("GooglePlay") => html! {
      <MD_GooglePlay ..owned_props />
    },
    implicit_clone::unsync::IString::Static("BalloonFill") => html! {
      <MD_BalloonFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FileEarmarkArrowDownFill") => html! {
      <MD_FileEarmarkArrowDownFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Arrow90degUp") => html! {
      <MD_Arrow90degUp ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Mic") => html! {
      <MD_Mic ..owned_props />
    },
    implicit_clone::unsync::IString::Static("1CircleFill") => html! {
      <MD_1CircleFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("BrightnessLow") => html! {
      <MD_BrightnessLow ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Clipboard2Minus") => html! {
      <MD_Clipboard2Minus ..owned_props />
    },
    implicit_clone::unsync::IString::Static("AspectRatioFill") => html! {
      <MD_AspectRatioFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("SignTurnSlightRightFill") => html! {
      <MD_SignTurnSlightRightFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("WindowX") => html! {
      <MD_WindowX ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ChatLeftDotsFill") => html! {
      <MD_ChatLeftDotsFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("SignDeadEnd") => html! {
      <MD_SignDeadEnd ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ChevronUp") => html! {
      <MD_ChevronUp ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Bookmark") => html! {
      <MD_Bookmark ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FileEarmarkCodeFill") => html! {
      <MD_FileEarmarkCodeFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Subscript") => html! {
      <MD_Subscript ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Fan") => html! {
      <MD_Fan ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Dice1") => html! {
      <MD_Dice1 ..owned_props />
    },
    implicit_clone::unsync::IString::Static("SignStopLightsFill") => html! {
      <MD_SignStopLightsFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ArrowDownLeftSquare") => html! {
      <MD_ArrowDownLeftSquare ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ArrowUpLeftCircleFill") => html! {
      <MD_ArrowUpLeftCircleFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("DatabaseFillExclamation") => html! {
      <MD_DatabaseFillExclamation ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Speedometer") => html! {
      <MD_Speedometer ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ChatHeartFill") => html! {
      <MD_ChatHeartFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("HeartHalf") => html! {
      <MD_HeartHalf ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ShieldFillCheck") => html! {
      <MD_ShieldFillCheck ..owned_props />
    },
    implicit_clone::unsync::IString::Static("CalendarDay") => html! {
      <MD_CalendarDay ..owned_props />
    },
    implicit_clone::unsync::IString::Static("HourglassTop") => html! {
      <MD_HourglassTop ..owned_props />
    },
    implicit_clone::unsync::IString::Static("BackspaceReverseFill") => html! {
      <MD_BackspaceReverseFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Brush") => html! {
      <MD_Brush ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FileEarmarkPpt") => html! {
      <MD_FileEarmarkPpt ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Yelp") => html! {
      <MD_Yelp ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Stop") => html! {
      <MD_Stop ..owned_props />
    },
    implicit_clone::unsync::IString::Static("EmojiExpressionless") => html! {
      <MD_EmojiExpressionless ..owned_props />
    },
    implicit_clone::unsync::IString::Static("SkipForwardBtn") => html! {
      <MD_SkipForwardBtn ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Columns") => html! {
      <MD_Columns ..owned_props />
    },
    implicit_clone::unsync::IString::Static("InfoCircleFill") => html! {
      <MD_InfoCircleFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("6CircleFill") => html! {
      <MD_6CircleFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("QrCode") => html! {
      <MD_QrCode ..owned_props />
    },
    implicit_clone::unsync::IString::Static("DiamondFill") => html! {
      <MD_DiamondFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("CloudRainHeavyFill") => html! {
      <MD_CloudRainHeavyFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("CursorText") => html! {
      <MD_CursorText ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ArchiveFill") => html! {
      <MD_ArchiveFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ChevronBarUp") => html! {
      <MD_ChevronBarUp ..owned_props />
    },
    implicit_clone::unsync::IString::Static("CaretRightFill") => html! {
      <MD_CaretRightFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FastForwardCircle") => html! {
      <MD_FastForwardCircle ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Map") => html! {
      <MD_Map ..owned_props />
    },
    implicit_clone::unsync::IString::Static("CartFill") => html! {
      <MD_CartFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("CartCheckFill") => html! {
      <MD_CartCheckFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FilePlayFill") => html! {
      <MD_FilePlayFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("LayoutSidebarInset") => html! {
      <MD_LayoutSidebarInset ..owned_props />
    },
    implicit_clone::unsync::IString::Static("CcSquare") => html! {
      <MD_CcSquare ..owned_props />
    },
    implicit_clone::unsync::IString::Static("SdCard") => html! {
      <MD_SdCard ..owned_props />
    },
    implicit_clone::unsync::IString::Static("EmojiDizzy") => html! {
      <MD_EmojiDizzy ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Wifi") => html! {
      <MD_Wifi ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ThermometerLow") => html! {
      <MD_ThermometerLow ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FileLock2Fill") => html! {
      <MD_FileLock2Fill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("BagPlus") => html! {
      <MD_BagPlus ..owned_props />
    },
    implicit_clone::unsync::IString::Static("CameraVideo") => html! {
      <MD_CameraVideo ..owned_props />
    },
    implicit_clone::unsync::IString::Static("BoxArrowLeft") => html! {
      <MD_BoxArrowLeft ..owned_props />
    },
    implicit_clone::unsync::IString::Static("BootstrapFill") => html! {
      <MD_BootstrapFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Sunset") => html! {
      <MD_Sunset ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FilterCircle") => html! {
      <MD_FilterCircle ..owned_props />
    },
    implicit_clone::unsync::IString::Static("CapsulePill") => html! {
      <MD_CapsulePill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("SendSlashFill") => html! {
      <MD_SendSlashFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("BuildingFill") => html! {
      <MD_BuildingFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Files") => html! {
      <MD_Files ..owned_props />
    },
    implicit_clone::unsync::IString::Static("BagCheck") => html! {
      <MD_BagCheck ..owned_props />
    },
    implicit_clone::unsync::IString::Static("JournalBookmarkFill") => html! {
      <MD_JournalBookmarkFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("CloudSnowFill") => html! {
      <MD_CloudSnowFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("OctagonHalf") => html! {
      <MD_OctagonHalf ..owned_props />
    },
    implicit_clone::unsync::IString::Static("TerminalSplit") => html! {
      <MD_TerminalSplit ..owned_props />
    },
    implicit_clone::unsync::IString::Static("CalendarMinus") => html! {
      <MD_CalendarMinus ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ArrowLeftSquareFill") => html! {
      <MD_ArrowLeftSquareFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ChevronContract") => html! {
      <MD_ChevronContract ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Router") => html! {
      <MD_Router ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Calendar2Day") => html! {
      <MD_Calendar2Day ..owned_props />
    },
    implicit_clone::unsync::IString::Static("PersonPlusFill") => html! {
      <MD_PersonPlusFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("SignpostFill") => html! {
      <MD_SignpostFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Disc") => html! {
      <MD_Disc ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Grid3x2GapFill") => html! {
      <MD_Grid3x2GapFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("3Square") => html! {
      <MD_3Square ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Basket3Fill") => html! {
      <MD_Basket3Fill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ArrowUpCircle") => html! {
      <MD_ArrowUpCircle ..owned_props />
    },
    implicit_clone::unsync::IString::Static("GlobeEuropeAfrica") => html! {
      <MD_GlobeEuropeAfrica ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FileExcelFill") => html! {
      <MD_FileExcelFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Discord") => html! {
      <MD_Discord ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Prescription2") => html! {
      <MD_Prescription2 ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ArrowReturnLeft") => html! {
      <MD_ArrowReturnLeft ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Cart") => html! {
      <MD_Cart ..owned_props />
    },
    implicit_clone::unsync::IString::Static("CalendarDayFill") => html! {
      <MD_CalendarDayFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("3CircleFill") => html! {
      <MD_3CircleFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("MusicNoteBeamed") => html! {
      <MD_MusicNoteBeamed ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ArrowDownCircle") => html! {
      <MD_ArrowDownCircle ..owned_props />
    },
    implicit_clone::unsync::IString::Static("CloudyFill") => html! {
      <MD_CloudyFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("DisplayFill") => html! {
      <MD_DisplayFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FilePersonFill") => html! {
      <MD_FilePersonFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ClipboardData") => html! {
      <MD_ClipboardData ..owned_props />
    },
    implicit_clone::unsync::IString::Static("RocketTakeoff") => html! {
      <MD_RocketTakeoff ..owned_props />
    },
    implicit_clone::unsync::IString::Static("PersonFillCheck") => html! {
      <MD_PersonFillCheck ..owned_props />
    },
    implicit_clone::unsync::IString::Static("HeadsetVr") => html! {
      <MD_HeadsetVr ..owned_props />
    },
    implicit_clone::unsync::IString::Static("BookmarksFill") => html! {
      <MD_BookmarksFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("DatabaseSlash") => html! {
      <MD_DatabaseSlash ..owned_props />
    },
    implicit_clone::unsync::IString::Static("DistributeVertical") => html! {
      <MD_DistributeVertical ..owned_props />
    },
    implicit_clone::unsync::IString::Static("BarChartFill") => html! {
      <MD_BarChartFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("SuitHeart") => html! {
      <MD_SuitHeart ..owned_props />
    },
    implicit_clone::unsync::IString::Static("HCircleFill") => html! {
      <MD_HCircleFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Server") => html! {
      <MD_Server ..owned_props />
    },
    implicit_clone::unsync::IString::Static("DistributeHorizontal") => html! {
      <MD_DistributeHorizontal ..owned_props />
    },
    implicit_clone::unsync::IString::Static("SignIntersectionY") => html! {
      <MD_SignIntersectionY ..owned_props />
    },
    implicit_clone::unsync::IString::Static("BuildingCheck") => html! {
      <MD_BuildingCheck ..owned_props />
    },
    implicit_clone::unsync::IString::Static("BackspaceReverse") => html! {
      <MD_BackspaceReverse ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ListUl") => html! {
      <MD_ListUl ..owned_props />
    },
    implicit_clone::unsync::IString::Static("SortDown") => html! {
      <MD_SortDown ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FileMedicalFill") => html! {
      <MD_FileMedicalFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FileEarmarkMinusFill") => html! {
      <MD_FileEarmarkMinusFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ShieldFillX") => html! {
      <MD_ShieldFillX ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ChatRightHeart") => html! {
      <MD_ChatRightHeart ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ArrowDownSquareFill") => html! {
      <MD_ArrowDownSquareFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Paragraph") => html! {
      <MD_Paragraph ..owned_props />
    },
    implicit_clone::unsync::IString::Static("TerminalX") => html! {
      <MD_TerminalX ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ChatLeftFill") => html! {
      <MD_ChatLeftFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Soundwave") => html! {
      <MD_Soundwave ..owned_props />
    },
    implicit_clone::unsync::IString::Static("EnvelopeAtFill") => html! {
      <MD_EnvelopeAtFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Arrow90degDown") => html! {
      <MD_Arrow90degDown ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FileRuledFill") => html! {
      <MD_FileRuledFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ExclamationTriangle") => html! {
      <MD_ExclamationTriangle ..owned_props />
    },
    implicit_clone::unsync::IString::Static("UniversalAccess") => html! {
      <MD_UniversalAccess ..owned_props />
    },
    implicit_clone::unsync::IString::Static("SendCheck") => html! {
      <MD_SendCheck ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Hypnotize") => html! {
      <MD_Hypnotize ..owned_props />
    },
    implicit_clone::unsync::IString::Static("PatchPlusFill") => html! {
      <MD_PatchPlusFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("CartDash") => html! {
      <MD_CartDash ..owned_props />
    },
    implicit_clone::unsync::IString::Static("StackOverflow") => html! {
      <MD_StackOverflow ..owned_props />
    },
    implicit_clone::unsync::IString::Static("BellSlashFill") => html! {
      <MD_BellSlashFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FileEarmarkArrowDown") => html! {
      <MD_FileEarmarkArrowDown ..owned_props />
    },
    implicit_clone::unsync::IString::Static("LayoutTextSidebar") => html! {
      <MD_LayoutTextSidebar ..owned_props />
    },
    implicit_clone::unsync::IString::Static("HandIndexThumb") => html! {
      <MD_HandIndexThumb ..owned_props />
    },
    implicit_clone::unsync::IString::Static("StoplightsFill") => html! {
      <MD_StoplightsFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Minecart") => html! {
      <MD_Minecart ..owned_props />
    },
    implicit_clone::unsync::IString::Static("BorderTop") => html! {
      <MD_BorderTop ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Fullscreen") => html! {
      <MD_Fullscreen ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FileLock") => html! {
      <MD_FileLock ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ShieldCheck") => html! {
      <MD_ShieldCheck ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ChatLeftHeartFill") => html! {
      <MD_ChatLeftHeartFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("PersonDown") => html! {
      <MD_PersonDown ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Dash") => html! {
      <MD_Dash ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Reception4") => html! {
      <MD_Reception4 ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Speaker") => html! {
      <MD_Speaker ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ClipboardX") => html! {
      <MD_ClipboardX ..owned_props />
    },
    implicit_clone::unsync::IString::Static("BookmarkCheckFill") => html! {
      <MD_BookmarkCheckFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("BinocularsFill") => html! {
      <MD_BinocularsFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("TrainLightrailFront") => html! {
      <MD_TrainLightrailFront ..owned_props />
    },
    implicit_clone::unsync::IString::Static("TreeFill") => html! {
      <MD_TreeFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ArrowThroughHeart") => html! {
      <MD_ArrowThroughHeart ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FilePerson") => html! {
      <MD_FilePerson ..owned_props />
    },
    implicit_clone::unsync::IString::Static("HouseLockFill") => html! {
      <MD_HouseLockFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FileEarmarkPptFill") => html! {
      <MD_FileEarmarkPptFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Tree") => html! {
      <MD_Tree ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Pinterest") => html! {
      <MD_Pinterest ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Gem") => html! {
      <MD_Gem ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Clipboard2PulseFill") => html! {
      <MD_Clipboard2PulseFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Dice2Fill") => html! {
      <MD_Dice2Fill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("CartCheck") => html! {
      <MD_CartCheck ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FuelPumpDieselFill") => html! {
      <MD_FuelPumpDieselFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ThreeDotsVertical") => html! {
      <MD_ThreeDotsVertical ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FuelPumpDiesel") => html! {
      <MD_FuelPumpDiesel ..owned_props />
    },
    implicit_clone::unsync::IString::Static("XSquare") => html! {
      <MD_XSquare ..owned_props />
    },
    implicit_clone::unsync::IString::Static("PersonRolodex") => html! {
      <MD_PersonRolodex ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Calendar2MinusFill") => html! {
      <MD_Calendar2MinusFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("GlobeAmericas") => html! {
      <MD_GlobeAmericas ..owned_props />
    },
    implicit_clone::unsync::IString::Static("DoorClosed") => html! {
      <MD_DoorClosed ..owned_props />
    },
    implicit_clone::unsync::IString::Static("VinylFill") => html! {
      <MD_VinylFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ListStars") => html! {
      <MD_ListStars ..owned_props />
    },
    implicit_clone::unsync::IString::Static("PlayCircleFill") => html! {
      <MD_PlayCircleFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("SendExclamation") => html! {
      <MD_SendExclamation ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Bezier") => html! {
      <MD_Bezier ..owned_props />
    },
    implicit_clone::unsync::IString::Static("PauseCircleFill") => html! {
      <MD_PauseCircleFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("LayoutTextWindow") => html! {
      <MD_LayoutTextWindow ..owned_props />
    },
    implicit_clone::unsync::IString::Static("PinAngleFill") => html! {
      <MD_PinAngleFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("SpeakerFill") => html! {
      <MD_SpeakerFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Slack") => html! {
      <MD_Slack ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Basket") => html! {
      <MD_Basket ..owned_props />
    },
    implicit_clone::unsync::IString::Static("BookmarkX") => html! {
      <MD_BookmarkX ..owned_props />
    },
    implicit_clone::unsync::IString::Static("BatteryHalf") => html! {
      <MD_BatteryHalf ..owned_props />
    },
    implicit_clone::unsync::IString::Static("CalendarEventFill") => html! {
      <MD_CalendarEventFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("EnvelopeSlashFill") => html! {
      <MD_EnvelopeSlashFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("PuzzleFill") => html! {
      <MD_PuzzleFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("CalendarCheck") => html! {
      <MD_CalendarCheck ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ImageAlt") => html! {
      <MD_ImageAlt ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FiletypePy") => html! {
      <MD_FiletypePy ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FilePostFill") => html! {
      <MD_FilePostFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Braces") => html! {
      <MD_Braces ..owned_props />
    },
    implicit_clone::unsync::IString::Static("SkipEndCircleFill") => html! {
      <MD_SkipEndCircleFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("LightbulbOffFill") => html! {
      <MD_LightbulbOffFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("BrowserChrome") => html! {
      <MD_BrowserChrome ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Plus") => html! {
      <MD_Plus ..owned_props />
    },
    implicit_clone::unsync::IString::Static("GenderAmbiguous") => html! {
      <MD_GenderAmbiguous ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Sliders") => html! {
      <MD_Sliders ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Whatsapp") => html! {
      <MD_Whatsapp ..owned_props />
    },
    implicit_clone::unsync::IString::Static("BrightnessLowFill") => html! {
      <MD_BrightnessLowFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("SkipBackwardFill") => html! {
      <MD_SkipBackwardFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("PersonX") => html! {
      <MD_PersonX ..owned_props />
    },
    implicit_clone::unsync::IString::Static("VolumeUp") => html! {
      <MD_VolumeUp ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Laptop") => html! {
      <MD_Laptop ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Apple") => html! {
      <MD_Apple ..owned_props />
    },
    implicit_clone::unsync::IString::Static("PersonDashFill") => html! {
      <MD_PersonDashFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Unindent") => html! {
      <MD_Unindent ..owned_props />
    },
    implicit_clone::unsync::IString::Static("MoonFill") => html! {
      <MD_MoonFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("8SquareFill") => html! {
      <MD_8SquareFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("HouseHeartFill") => html! {
      <MD_HouseHeartFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FileArrowUpFill") => html! {
      <MD_FileArrowUpFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Snow2") => html! {
      <MD_Snow2 ..owned_props />
    },
    implicit_clone::unsync::IString::Static("At") => html! {
      <MD_At ..owned_props />
    },
    implicit_clone::unsync::IString::Static("CheckSquare") => html! {
      <MD_CheckSquare ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FileTextFill") => html! {
      <MD_FileTextFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FileEarmarkLock") => html! {
      <MD_FileEarmarkLock ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FiletypeM4p") => html! {
      <MD_FiletypeM4p ..owned_props />
    },
    implicit_clone::unsync::IString::Static("SendExclamationFill") => html! {
      <MD_SendExclamationFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("TelephoneMinus") => html! {
      <MD_TelephoneMinus ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Google") => html! {
      <MD_Google ..owned_props />
    },
    implicit_clone::unsync::IString::Static("JournalRichtext") => html! {
      <MD_JournalRichtext ..owned_props />
    },
    implicit_clone::unsync::IString::Static("0CircleFill") => html! {
      <MD_0CircleFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ArrowLeftCircleFill") => html! {
      <MD_ArrowLeftCircleFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("StopBtnFill") => html! {
      <MD_StopBtnFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Moon") => html! {
      <MD_Moon ..owned_props />
    },
    implicit_clone::unsync::IString::Static("TabletLandscapeFill") => html! {
      <MD_TabletLandscapeFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Reply") => html! {
      <MD_Reply ..owned_props />
    },
    implicit_clone::unsync::IString::Static("QuestionSquare") => html! {
      <MD_QuestionSquare ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FiletypeAi") => html! {
      <MD_FiletypeAi ..owned_props />
    },
    implicit_clone::unsync::IString::Static("TruckFront") => html! {
      <MD_TruckFront ..owned_props />
    },
    implicit_clone::unsync::IString::Static("SignIntersectionT") => html! {
      <MD_SignIntersectionT ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ClockHistory") => html! {
      <MD_ClockHistory ..owned_props />
    },
    implicit_clone::unsync::IString::Static("TriangleHalf") => html! {
      <MD_TriangleHalf ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Sunglasses") => html! {
      <MD_Sunglasses ..owned_props />
    },
    implicit_clone::unsync::IString::Static("TypeH3") => html! {
      <MD_TypeH3 ..owned_props />
    },
    implicit_clone::unsync::IString::Static("6Circle") => html! {
      <MD_6Circle ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Pentagon") => html! {
      <MD_Pentagon ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FileEarmarkX") => html! {
      <MD_FileEarmarkX ..owned_props />
    },
    implicit_clone::unsync::IString::Static("StickiesFill") => html! {
      <MD_StickiesFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("NintendoSwitch") => html! {
      <MD_NintendoSwitch ..owned_props />
    },
    implicit_clone::unsync::IString::Static("CollectionPlayFill") => html! {
      <MD_CollectionPlayFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("JournalBookmark") => html! {
      <MD_JournalBookmark ..owned_props />
    },
    implicit_clone::unsync::IString::Static("StickyFill") => html! {
      <MD_StickyFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("CurrencyBitcoin") => html! {
      <MD_CurrencyBitcoin ..owned_props />
    },
    implicit_clone::unsync::IString::Static("BackspaceFill") => html! {
      <MD_BackspaceFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("TextLeft") => html! {
      <MD_TextLeft ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Headphones") => html! {
      <MD_Headphones ..owned_props />
    },
    implicit_clone::unsync::IString::Static("GeoFill") => html! {
      <MD_GeoFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("CloudMinusFill") => html! {
      <MD_CloudMinusFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FileWord") => html! {
      <MD_FileWord ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FileDiffFill") => html! {
      <MD_FileDiffFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("LightbulbOff") => html! {
      <MD_LightbulbOff ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Box2Heart") => html! {
      <MD_Box2Heart ..owned_props />
    },
    implicit_clone::unsync::IString::Static("EnvelopeOpenFill") => html! {
      <MD_EnvelopeOpenFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ArrowLeftShort") => html! {
      <MD_ArrowLeftShort ..owned_props />
    },
    implicit_clone::unsync::IString::Static("PostcardHeart") => html! {
      <MD_PostcardHeart ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Journals") => html! {
      <MD_Journals ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Link") => html! {
      <MD_Link ..owned_props />
    },
    implicit_clone::unsync::IString::Static("TextIndentLeft") => html! {
      <MD_TextIndentLeft ..owned_props />
    },
    implicit_clone::unsync::IString::Static("BookHalf") => html! {
      <MD_BookHalf ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ArrowUpRightSquareFill") => html! {
      <MD_ArrowUpRightSquareFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FileSpreadsheetFill") => html! {
      <MD_FileSpreadsheetFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("UiRadiosGrid") => html! {
      <MD_UiRadiosGrid ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Bank") => html! {
      <MD_Bank ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FiletypeSh") => html! {
      <MD_FiletypeSh ..owned_props />
    },
    implicit_clone::unsync::IString::Static("BoundingBox") => html! {
      <MD_BoundingBox ..owned_props />
    },
    implicit_clone::unsync::IString::Static("CameraFill") => html! {
      <MD_CameraFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Memory") => html! {
      <MD_Memory ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Quote") => html! {
      <MD_Quote ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FilePost") => html! {
      <MD_FilePost ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Calendar3RangeFill") => html! {
      <MD_Calendar3RangeFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("HeartPulse") => html! {
      <MD_HeartPulse ..owned_props />
    },
    implicit_clone::unsync::IString::Static("SkipStartCircle") => html! {
      <MD_SkipStartCircle ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Amd") => html! {
      <MD_Amd ..owned_props />
    },
    implicit_clone::unsync::IString::Static("TextCenter") => html! {
      <MD_TextCenter ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ListOl") => html! {
      <MD_ListOl ..owned_props />
    },
    implicit_clone::unsync::IString::Static("SignDoNotEnterFill") => html! {
      <MD_SignDoNotEnterFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("LayoutTextSidebarReverse") => html! {
      <MD_LayoutTextSidebarReverse ..owned_props />
    },
    implicit_clone::unsync::IString::Static("PersonVcardFill") => html! {
      <MD_PersonVcardFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("DiamondHalf") => html! {
      <MD_DiamondHalf ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Reception3") => html! {
      <MD_Reception3 ..owned_props />
    },
    implicit_clone::unsync::IString::Static("SortDownAlt") => html! {
      <MD_SortDownAlt ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Wifi2") => html! {
      <MD_Wifi2 ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Eye") => html! {
      <MD_Eye ..owned_props />
    },
    implicit_clone::unsync::IString::Static("CalendarXFill") => html! {
      <MD_CalendarXFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Incognito") => html! {
      <MD_Incognito ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ArrowUpCircleFill") => html! {
      <MD_ArrowUpCircleFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("HddStack") => html! {
      <MD_HddStack ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Check") => html! {
      <MD_Check ..owned_props />
    },
    implicit_clone::unsync::IString::Static("InfoSquare") => html! {
      <MD_InfoSquare ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FileEarmarkBreak") => html! {
      <MD_FileEarmarkBreak ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Infinity") => html! {
      <MD_Infinity ..owned_props />
    },
    implicit_clone::unsync::IString::Static("BuildingUp") => html! {
      <MD_BuildingUp ..owned_props />
    },
    implicit_clone::unsync::IString::Static("JournalX") => html! {
      <MD_JournalX ..owned_props />
    },
    implicit_clone::unsync::IString::Static("CalendarPlus") => html! {
      <MD_CalendarPlus ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ExclamationCircle") => html! {
      <MD_ExclamationCircle ..owned_props />
    },
    implicit_clone::unsync::IString::Static("DatabaseFillLock") => html! {
      <MD_DatabaseFillLock ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Calendar2DateFill") => html! {
      <MD_Calendar2DateFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Palette") => html! {
      <MD_Palette ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Window") => html! {
      <MD_Window ..owned_props />
    },
    implicit_clone::unsync::IString::Static("BookmarkHeart") => html! {
      <MD_BookmarkHeart ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Asterisk") => html! {
      <MD_Asterisk ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Basket2") => html! {
      <MD_Basket2 ..owned_props />
    },
    implicit_clone::unsync::IString::Static("MicMuteFill") => html! {
      <MD_MicMuteFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("BrowserSafari") => html! {
      <MD_BrowserSafari ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ChatRightDotsFill") => html! {
      <MD_ChatRightDotsFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ClipboardMinusFill") => html! {
      <MD_ClipboardMinusFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Circle") => html! {
      <MD_Circle ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Easel3Fill") => html! {
      <MD_Easel3Fill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("TrainFreightFrontFill") => html! {
      <MD_TrainFreightFrontFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("GearWide") => html! {
      <MD_GearWide ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ChevronRight") => html! {
      <MD_ChevronRight ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FileEarmarkBinaryFill") => html! {
      <MD_FileEarmarkBinaryFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Wikipedia") => html! {
      <MD_Wikipedia ..owned_props />
    },
    implicit_clone::unsync::IString::Static("9CircleFill") => html! {
      <MD_9CircleFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Droplet") => html! {
      <MD_Droplet ..owned_props />
    },
    implicit_clone::unsync::IString::Static("SignpostSplit") => html! {
      <MD_SignpostSplit ..owned_props />
    },
    implicit_clone::unsync::IString::Static("StopCircle") => html! {
      <MD_StopCircle ..owned_props />
    },
    implicit_clone::unsync::IString::Static("DatabaseFillGear") => html! {
      <MD_DatabaseFillGear ..owned_props />
    },
    implicit_clone::unsync::IString::Static("CaretLeft") => html! {
      <MD_CaretLeft ..owned_props />
    },
    implicit_clone::unsync::IString::Static("InfoCircle") => html! {
      <MD_InfoCircle ..owned_props />
    },
    implicit_clone::unsync::IString::Static("DatabaseFillX") => html! {
      <MD_DatabaseFillX ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Triangle") => html! {
      <MD_Triangle ..owned_props />
    },
    implicit_clone::unsync::IString::Static("HandThumbsUpFill") => html! {
      <MD_HandThumbsUpFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("JournalAlbum") => html! {
      <MD_JournalAlbum ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Cart2") => html! {
      <MD_Cart2 ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ShieldX") => html! {
      <MD_ShieldX ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Record2") => html! {
      <MD_Record2 ..owned_props />
    },
    implicit_clone::unsync::IString::Static("SuitDiamondFill") => html! {
      <MD_SuitDiamondFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("JournalText") => html! {
      <MD_JournalText ..owned_props />
    },
    implicit_clone::unsync::IString::Static("SignYield") => html! {
      <MD_SignYield ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Puzzle") => html! {
      <MD_Puzzle ..owned_props />
    },
    implicit_clone::unsync::IString::Static("TelephoneMinusFill") => html! {
      <MD_TelephoneMinusFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FilePdf") => html! {
      <MD_FilePdf ..owned_props />
    },
    implicit_clone::unsync::IString::Static("MusicNote") => html! {
      <MD_MusicNote ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ArrowsAngleContract") => html! {
      <MD_ArrowsAngleContract ..owned_props />
    },
    implicit_clone::unsync::IString::Static("BoxArrowInLeft") => html! {
      <MD_BoxArrowInLeft ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Recycle") => html! {
      <MD_Recycle ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ChevronDoubleDown") => html! {
      <MD_ChevronDoubleDown ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FiletypePptx") => html! {
      <MD_FiletypePptx ..owned_props />
    },
    implicit_clone::unsync::IString::Static("XCircle") => html! {
      <MD_XCircle ..owned_props />
    },
    implicit_clone::unsync::IString::Static("TypeBold") => html! {
      <MD_TypeBold ..owned_props />
    },
    implicit_clone::unsync::IString::Static("PhoneLandscape") => html! {
      <MD_PhoneLandscape ..owned_props />
    },
    implicit_clone::unsync::IString::Static("DatabaseDown") => html! {
      <MD_DatabaseDown ..owned_props />
    },
    implicit_clone::unsync::IString::Static("DatabaseFillSlash") => html! {
      <MD_DatabaseFillSlash ..owned_props />
    },
    implicit_clone::unsync::IString::Static("PersonBadgeFill") => html! {
      <MD_PersonBadgeFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("PieChart") => html! {
      <MD_PieChart ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Exclamation") => html! {
      <MD_Exclamation ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FileEarmarkLock2") => html! {
      <MD_FileEarmarkLock2 ..owned_props />
    },
    implicit_clone::unsync::IString::Static("DatabaseGear") => html! {
      <MD_DatabaseGear ..owned_props />
    },
    implicit_clone::unsync::IString::Static("BookmarkStarFill") => html! {
      <MD_BookmarkStarFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("SkipStartBtn") => html! {
      <MD_SkipStartBtn ..owned_props />
    },
    implicit_clone::unsync::IString::Static("SignNoRightTurnFill") => html! {
      <MD_SignNoRightTurnFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("BagX") => html! {
      <MD_BagX ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ZoomIn") => html! {
      <MD_ZoomIn ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ArrowRightCircleFill") => html! {
      <MD_ArrowRightCircleFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ClipboardCheck") => html! {
      <MD_ClipboardCheck ..owned_props />
    },
    implicit_clone::unsync::IString::Static("CameraVideoOffFill") => html! {
      <MD_CameraVideoOffFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Database") => html! {
      <MD_Database ..owned_props />
    },
    implicit_clone::unsync::IString::Static("GearWideConnected") => html! {
      <MD_GearWideConnected ..owned_props />
    },
    implicit_clone::unsync::IString::Static("TelephonePlus") => html! {
      <MD_TelephonePlus ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FileEarmarkWordFill") => html! {
      <MD_FileEarmarkWordFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("EyeSlashFill") => html! {
      <MD_EyeSlashFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Search") => html! {
      <MD_Search ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Calendar2Range") => html! {
      <MD_Calendar2Range ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ChatSquareDots") => html! {
      <MD_ChatSquareDots ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Reception0") => html! {
      <MD_Reception0 ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Calendar2WeekFill") => html! {
      <MD_Calendar2WeekFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FileZip") => html! {
      <MD_FileZip ..owned_props />
    },
    implicit_clone::unsync::IString::Static("GlobeCentralSouthAsia") => html! {
      <MD_GlobeCentralSouthAsia ..owned_props />
    },
    implicit_clone::unsync::IString::Static("PinMapFill") => html! {
      <MD_PinMapFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("LayoutSplit") => html! {
      <MD_LayoutSplit ..owned_props />
    },
    implicit_clone::unsync::IString::Static("CloudCheckFill") => html! {
      <MD_CloudCheckFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("MortarboardFill") => html! {
      <MD_MortarboardFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("0SquareFill") => html! {
      <MD_0SquareFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FileLock2") => html! {
      <MD_FileLock2 ..owned_props />
    },
    implicit_clone::unsync::IString::Static("HouseDown") => html! {
      <MD_HouseDown ..owned_props />
    },
    implicit_clone::unsync::IString::Static("CloudHaze2Fill") => html! {
      <MD_CloudHaze2Fill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ArrowDownShort") => html! {
      <MD_ArrowDownShort ..owned_props />
    },
    implicit_clone::unsync::IString::Static("BookmarkXFill") => html! {
      <MD_BookmarkXFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("EvStation") => html! {
      <MD_EvStation ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FilterRight") => html! {
      <MD_FilterRight ..owned_props />
    },
    implicit_clone::unsync::IString::Static("HeartFill") => html! {
      <MD_HeartFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("UnlockFill") => html! {
      <MD_UnlockFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FileEarmarkMusicFill") => html! {
      <MD_FileEarmarkMusicFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Mouse3") => html! {
      <MD_Mouse3 ..owned_props />
    },
    implicit_clone::unsync::IString::Static("HourglassBottom") => html! {
      <MD_HourglassBottom ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Joystick") => html! {
      <MD_Joystick ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FileEarmarkMinus") => html! {
      <MD_FileEarmarkMinus ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Lightbulb") => html! {
      <MD_Lightbulb ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FileEarmarkSpreadsheet") => html! {
      <MD_FileEarmarkSpreadsheet ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FiletypeWav") => html! {
      <MD_FiletypeWav ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Bag") => html! {
      <MD_Bag ..owned_props />
    },
    implicit_clone::unsync::IString::Static("LightningChargeFill") => html! {
      <MD_LightningChargeFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Calendar3") => html! {
      <MD_Calendar3 ..owned_props />
    },
    implicit_clone::unsync::IString::Static("OpticalAudio") => html! {
      <MD_OpticalAudio ..owned_props />
    },
    implicit_clone::unsync::IString::Static("DatabaseCheck") => html! {
      <MD_DatabaseCheck ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ChevronDoubleRight") => html! {
      <MD_ChevronDoubleRight ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Pc") => html! {
      <MD_Pc ..owned_props />
    },
    implicit_clone::unsync::IString::Static("EnvelopePaperFill") => html! {
      <MD_EnvelopePaperFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FileBreak") => html! {
      <MD_FileBreak ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Tsunami") => html! {
      <MD_Tsunami ..owned_props />
    },
    implicit_clone::unsync::IString::Static("CheckSquareFill") => html! {
      <MD_CheckSquareFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("6SquareFill") => html! {
      <MD_6SquareFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("TextRight") => html! {
      <MD_TextRight ..owned_props />
    },
    implicit_clone::unsync::IString::Static("SkipStartBtnFill") => html! {
      <MD_SkipStartBtnFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("SuitHeartFill") => html! {
      <MD_SuitHeartFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("EarFill") => html! {
      <MD_EarFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Robot") => html! {
      <MD_Robot ..owned_props />
    },
    implicit_clone::unsync::IString::Static("UsbDriveFill") => html! {
      <MD_UsbDriveFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Mailbox2") => html! {
      <MD_Mailbox2 ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FiletypeHeic") => html! {
      <MD_FiletypeHeic ..owned_props />
    },
    implicit_clone::unsync::IString::Static("BoxSeamFill") => html! {
      <MD_BoxSeamFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FileEarmarkRichtext") => html! {
      <MD_FileEarmarkRichtext ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Signpost2Fill") => html! {
      <MD_Signpost2Fill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Nut") => html! {
      <MD_Nut ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Alexa") => html! {
      <MD_Alexa ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FileEarmarkFontFill") => html! {
      <MD_FileEarmarkFontFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("CloudDrizzle") => html! {
      <MD_CloudDrizzle ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Key") => html! {
      <MD_Key ..owned_props />
    },
    implicit_clone::unsync::IString::Static("DisplayportFill") => html! {
      <MD_DisplayportFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ClipboardHeart") => html! {
      <MD_ClipboardHeart ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Sim") => html! {
      <MD_Sim ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Pencil") => html! {
      <MD_Pencil ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Save") => html! {
      <MD_Save ..owned_props />
    },
    implicit_clone::unsync::IString::Static("UmbrellaFill") => html! {
      <MD_UmbrellaFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("TabletLandscape") => html! {
      <MD_TabletLandscape ..owned_props />
    },
    implicit_clone::unsync::IString::Static("SlashSquareFill") => html! {
      <MD_SlashSquareFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Ethernet") => html! {
      <MD_Ethernet ..owned_props />
    },
    implicit_clone::unsync::IString::Static("BadgeAdFill") => html! {
      <MD_BadgeAdFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ChevronDown") => html! {
      <MD_ChevronDown ..owned_props />
    },
    implicit_clone::unsync::IString::Static("BadgeWcFill") => html! {
      <MD_BadgeWcFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Eject") => html! {
      <MD_Eject ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Arrow90degRight") => html! {
      <MD_Arrow90degRight ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ThermometerSun") => html! {
      <MD_ThermometerSun ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ChatRightTextFill") => html! {
      <MD_ChatRightTextFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FastForwardFill") => html! {
      <MD_FastForwardFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("SignNoParkingFill") => html! {
      <MD_SignNoParkingFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FileEarmarkFill") => html! {
      <MD_FileEarmarkFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("VolumeDownFill") => html! {
      <MD_VolumeDownFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("MusicNoteList") => html! {
      <MD_MusicNoteList ..owned_props />
    },
    implicit_clone::unsync::IString::Static("EnvelopeExclamationFill") => html! {
      <MD_EnvelopeExclamationFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("TelephoneX") => html! {
      <MD_TelephoneX ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Virus") => html! {
      <MD_Virus ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Basket3") => html! {
      <MD_Basket3 ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Box2HeartFill") => html! {
      <MD_Box2HeartFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("SignIntersectionSideFill") => html! {
      <MD_SignIntersectionSideFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Hearts") => html! {
      <MD_Hearts ..owned_props />
    },
    implicit_clone::unsync::IString::Static("SignMergeLeft") => html! {
      <MD_SignMergeLeft ..owned_props />
    },
    implicit_clone::unsync::IString::Static("CartPlus") => html! {
      <MD_CartPlus ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Heart") => html! {
      <MD_Heart ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Intersect") => html! {
      <MD_Intersect ..owned_props />
    },
    implicit_clone::unsync::IString::Static("CloudSunFill") => html! {
      <MD_CloudSunFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ArrowRightCircle") => html! {
      <MD_ArrowRightCircle ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Textarea") => html! {
      <MD_Textarea ..owned_props />
    },
    implicit_clone::unsync::IString::Static("SymmetryVertical") => html! {
      <MD_SymmetryVertical ..owned_props />
    },
    implicit_clone::unsync::IString::Static("PersonAdd") => html! {
      <MD_PersonAdd ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Share") => html! {
      <MD_Share ..owned_props />
    },
    implicit_clone::unsync::IString::Static("CalendarRangeFill") => html! {
      <MD_CalendarRangeFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("EnvelopeHeart") => html! {
      <MD_EnvelopeHeart ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ChatSquareDotsFill") => html! {
      <MD_ChatSquareDotsFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Bookshelf") => html! {
      <MD_Bookshelf ..owned_props />
    },
    implicit_clone::unsync::IString::Static("LayerBackward") => html! {
      <MD_LayerBackward ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Calendar4Range") => html! {
      <MD_Calendar4Range ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Calendar3Fill") => html! {
      <MD_Calendar3Fill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("DashCircle") => html! {
      <MD_DashCircle ..owned_props />
    },
    implicit_clone::unsync::IString::Static("BadgeCc") => html! {
      <MD_BadgeCc ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Spellcheck") => html! {
      <MD_Spellcheck ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FiletypeDoc") => html! {
      <MD_FiletypeDoc ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Calendar2HeartFill") => html! {
      <MD_Calendar2HeartFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("BadgeHdFill") => html! {
      <MD_BadgeHdFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ReceiptCutoff") => html! {
      <MD_ReceiptCutoff ..owned_props />
    },
    implicit_clone::unsync::IString::Static("BuildingFillLock") => html! {
      <MD_BuildingFillLock ..owned_props />
    },
    implicit_clone::unsync::IString::Static("GraphDown") => html! {
      <MD_GraphDown ..owned_props />
    },
    implicit_clone::unsync::IString::Static("CaretDownSquareFill") => html! {
      <MD_CaretDownSquareFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("PersonLinesFill") => html! {
      <MD_PersonLinesFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Sticky") => html! {
      <MD_Sticky ..owned_props />
    },
    implicit_clone::unsync::IString::Static("CarFront") => html! {
      <MD_CarFront ..owned_props />
    },
    implicit_clone::unsync::IString::Static("1Circle") => html! {
      <MD_1Circle ..owned_props />
    },
    implicit_clone::unsync::IString::Static("SinaWeibo") => html! {
      <MD_SinaWeibo ..owned_props />
    },
    implicit_clone::unsync::IString::Static("WrenchAdjustableCircleFill") => html! {
      <MD_WrenchAdjustableCircleFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("SignTurnSlightLeft") => html! {
      <MD_SignTurnSlightLeft ..owned_props />
    },
    implicit_clone::unsync::IString::Static("SuitSpadeFill") => html! {
      <MD_SuitSpadeFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("BookmarkCheck") => html! {
      <MD_BookmarkCheck ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FuelPumpFill") => html! {
      <MD_FuelPumpFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("SortUp") => html! {
      <MD_SortUp ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FiletypeMd") => html! {
      <MD_FiletypeMd ..owned_props />
    },
    implicit_clone::unsync::IString::Static("SkipEndFill") => html! {
      <MD_SkipEndFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("EnvelopeDashFill") => html! {
      <MD_EnvelopeDashFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FileEarmarkRuled") => html! {
      <MD_FileEarmarkRuled ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Vimeo") => html! {
      <MD_Vimeo ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Info") => html! {
      <MD_Info ..owned_props />
    },
    implicit_clone::unsync::IString::Static("SortNumericDown") => html! {
      <MD_SortNumericDown ..owned_props />
    },
    implicit_clone::unsync::IString::Static("MenuUp") => html! {
      <MD_MenuUp ..owned_props />
    },
    implicit_clone::unsync::IString::Static("4Square") => html! {
      <MD_4Square ..owned_props />
    },
    implicit_clone::unsync::IString::Static("CloudLightningFill") => html! {
      <MD_CloudLightningFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("TrophyFill") => html! {
      <MD_TrophyFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("PenFill") => html! {
      <MD_PenFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FileMinusFill") => html! {
      <MD_FileMinusFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ArrowDownRightCircle") => html! {
      <MD_ArrowDownRightCircle ..owned_props />
    },
    implicit_clone::unsync::IString::Static("EmojiAngryFill") => html! {
      <MD_EmojiAngryFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("AwardFill") => html! {
      <MD_AwardFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Wrench") => html! {
      <MD_Wrench ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FileEarmarkText") => html! {
      <MD_FileEarmarkText ..owned_props />
    },
    implicit_clone::unsync::IString::Static("CloudArrowUp") => html! {
      <MD_CloudArrowUp ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Star") => html! {
      <MD_Star ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FileEarmarkRuledFill") => html! {
      <MD_FileEarmarkRuledFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Stripe") => html! {
      <MD_Stripe ..owned_props />
    },
    implicit_clone::unsync::IString::Static("EnvelopeX") => html! {
      <MD_EnvelopeX ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FiletypeAac") => html! {
      <MD_FiletypeAac ..owned_props />
    },
    implicit_clone::unsync::IString::Static("EvStationFill") => html! {
      <MD_EvStationFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Postage") => html! {
      <MD_Postage ..owned_props />
    },
    implicit_clone::unsync::IString::Static("SkipForward") => html! {
      <MD_SkipForward ..owned_props />
    },
    implicit_clone::unsync::IString::Static("CartX") => html! {
      <MD_CartX ..owned_props />
    },
    implicit_clone::unsync::IString::Static("CardImage") => html! {
      <MD_CardImage ..owned_props />
    },
    implicit_clone::unsync::IString::Static("HeartArrow") => html! {
      <MD_HeartArrow ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Arrow90degLeft") => html! {
      <MD_Arrow90degLeft ..owned_props />
    },
    implicit_clone::unsync::IString::Static("UsbPlugFill") => html! {
      <MD_UsbPlugFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FileEarmarkPlusFill") => html! {
      <MD_FileEarmarkPlusFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("BagXFill") => html! {
      <MD_BagXFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("BagHeart") => html! {
      <MD_BagHeart ..owned_props />
    },
    implicit_clone::unsync::IString::Static("PersonFillSlash") => html! {
      <MD_PersonFillSlash ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ChatHeart") => html! {
      <MD_ChatHeart ..owned_props />
    },
    implicit_clone::unsync::IString::Static("DatabaseFillCheck") => html! {
      <MD_DatabaseFillCheck ..owned_props />
    },
    implicit_clone::unsync::IString::Static("SignStopFill") => html! {
      <MD_SignStopFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("UsbC") => html! {
      <MD_UsbC ..owned_props />
    },
    implicit_clone::unsync::IString::Static("EmojiLaughing") => html! {
      <MD_EmojiLaughing ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Heartbreak") => html! {
      <MD_Heartbreak ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Mouse") => html! {
      <MD_Mouse ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Lamp") => html! {
      <MD_Lamp ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Dice6Fill") => html! {
      <MD_Dice6Fill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("CurrencyExchange") => html! {
      <MD_CurrencyExchange ..owned_props />
    },
    implicit_clone::unsync::IString::Static("EmojiSunglassesFill") => html! {
      <MD_EmojiSunglassesFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Reception1") => html! {
      <MD_Reception1 ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ClipboardPlusFill") => html! {
      <MD_ClipboardPlusFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Scissors") => html! {
      <MD_Scissors ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Linkedin") => html! {
      <MD_Linkedin ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ArrowReturnRight") => html! {
      <MD_ArrowReturnRight ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Repeat1") => html! {
      <MD_Repeat1 ..owned_props />
    },
    implicit_clone::unsync::IString::Static("BagHeartFill") => html! {
      <MD_BagHeartFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("TerminalFill") => html! {
      <MD_TerminalFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FilePlus") => html! {
      <MD_FilePlus ..owned_props />
    },
    implicit_clone::unsync::IString::Static("TelephoneOutbound") => html! {
      <MD_TelephoneOutbound ..owned_props />
    },
    implicit_clone::unsync::IString::Static("DatabaseExclamation") => html! {
      <MD_DatabaseExclamation ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FileXFill") => html! {
      <MD_FileXFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FileRichtext") => html! {
      <MD_FileRichtext ..owned_props />
    },
    implicit_clone::unsync::IString::Static("BuildingAdd") => html! {
      <MD_BuildingAdd ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Markdown") => html! {
      <MD_Markdown ..owned_props />
    },
    implicit_clone::unsync::IString::Static("BookmarkPlusFill") => html! {
      <MD_BookmarkPlusFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("DatabaseFillDash") => html! {
      <MD_DatabaseFillDash ..owned_props />
    },
    implicit_clone::unsync::IString::Static("BlockquoteRight") => html! {
      <MD_BlockquoteRight ..owned_props />
    },
    implicit_clone::unsync::IString::Static("PersonSlash") => html! {
      <MD_PersonSlash ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FiletypePpt") => html! {
      <MD_FiletypePpt ..owned_props />
    },
    implicit_clone::unsync::IString::Static("PatchMinusFill") => html! {
      <MD_PatchMinusFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("EnvelopeOpenHeart") => html! {
      <MD_EnvelopeOpenHeart ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Flower1") => html! {
      <MD_Flower1 ..owned_props />
    },
    implicit_clone::unsync::IString::Static("HouseDoor") => html! {
      <MD_HouseDoor ..owned_props />
    },
    implicit_clone::unsync::IString::Static("CircleSquare") => html! {
      <MD_CircleSquare ..owned_props />
    },
    implicit_clone::unsync::IString::Static("GeoAlt") => html! {
      <MD_GeoAlt ..owned_props />
    },
    implicit_clone::unsync::IString::Static("BuildingFillExclamation") => html! {
      <MD_BuildingFillExclamation ..owned_props />
    },
    implicit_clone::unsync::IString::Static("TextIndentRight") => html! {
      <MD_TextIndentRight ..owned_props />
    },
    implicit_clone::unsync::IString::Static("PersonCheck") => html! {
      <MD_PersonCheck ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Calendar2Check") => html! {
      <MD_Calendar2Check ..owned_props />
    },
    implicit_clone::unsync::IString::Static("PostcardFill") => html! {
      <MD_PostcardFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("PersonHearts") => html! {
      <MD_PersonHearts ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Check2") => html! {
      <MD_Check2 ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Globe") => html! {
      <MD_Globe ..owned_props />
    },
    implicit_clone::unsync::IString::Static("PrinterFill") => html! {
      <MD_PrinterFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Valentine2") => html! {
      <MD_Valentine2 ..owned_props />
    },
    implicit_clone::unsync::IString::Static("YinYang") => html! {
      <MD_YinYang ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Cash") => html! {
      <MD_Cash ..owned_props />
    },
    implicit_clone::unsync::IString::Static("GraphUpArrow") => html! {
      <MD_GraphUpArrow ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Calendar3WeekFill") => html! {
      <MD_Calendar3WeekFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("2SquareFill") => html! {
      <MD_2SquareFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("XLg") => html! {
      <MD_XLg ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Receipt") => html! {
      <MD_Receipt ..owned_props />
    },
    implicit_clone::unsync::IString::Static("WindowSplit") => html! {
      <MD_WindowSplit ..owned_props />
    },
    implicit_clone::unsync::IString::Static("PatchQuestion") => html! {
      <MD_PatchQuestion ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FileFont") => html! {
      <MD_FileFont ..owned_props />
    },
    implicit_clone::unsync::IString::Static("GenderTrans") => html! {
      <MD_GenderTrans ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Tablet") => html! {
      <MD_Tablet ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Upc") => html! {
      <MD_Upc ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Clipboard2HeartFill") => html! {
      <MD_Clipboard2HeartFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("PlusSquareDotted") => html! {
      <MD_PlusSquareDotted ..owned_props />
    },
    implicit_clone::unsync::IString::Static("CloudHazeFill") => html! {
      <MD_CloudHazeFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Display") => html! {
      <MD_Display ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FileCode") => html! {
      <MD_FileCode ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Medium") => html! {
      <MD_Medium ..owned_props />
    },
    implicit_clone::unsync::IString::Static("SignIntersectionTFill") => html! {
      <MD_SignIntersectionTFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("CalendarMonthFill") => html! {
      <MD_CalendarMonthFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("TropicalStorm") => html! {
      <MD_TropicalStorm ..owned_props />
    },
    implicit_clone::unsync::IString::Static("CloudFogFill") => html! {
      <MD_CloudFogFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Easel") => html! {
      <MD_Easel ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Hospital") => html! {
      <MD_Hospital ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Rss") => html! {
      <MD_Rss ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ChatRightText") => html! {
      <MD_ChatRightText ..owned_props />
    },
    implicit_clone::unsync::IString::Static("PcDisplay") => html! {
      <MD_PcDisplay ..owned_props />
    },
    implicit_clone::unsync::IString::Static("TelephoneInboundFill") => html! {
      <MD_TelephoneInboundFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FiletypeRaw") => html! {
      <MD_FiletypeRaw ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ChatSquareQuote") => html! {
      <MD_ChatSquareQuote ..owned_props />
    },
    implicit_clone::unsync::IString::Static("PersonGear") => html! {
      <MD_PersonGear ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FileWordFill") => html! {
      <MD_FileWordFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Calendar3Event") => html! {
      <MD_Calendar3Event ..owned_props />
    },
    implicit_clone::unsync::IString::Static("BugFill") => html! {
      <MD_BugFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("EmojiAngry") => html! {
      <MD_EmojiAngry ..owned_props />
    },
    implicit_clone::unsync::IString::Static("EggFried") => html! {
      <MD_EggFried ..owned_props />
    },
    implicit_clone::unsync::IString::Static("LayersFill") => html! {
      <MD_LayersFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("HourglassSplit") => html! {
      <MD_HourglassSplit ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Border") => html! {
      <MD_Border ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Stopwatch") => html! {
      <MD_Stopwatch ..owned_props />
    },
    implicit_clone::unsync::IString::Static("BorderBottom") => html! {
      <MD_BorderBottom ..owned_props />
    },
    implicit_clone::unsync::IString::Static("BadgeCcFill") => html! {
      <MD_BadgeCcFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Rocket") => html! {
      <MD_Rocket ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Calendar2Event") => html! {
      <MD_Calendar2Event ..owned_props />
    },
    implicit_clone::unsync::IString::Static("PersonFillDash") => html! {
      <MD_PersonFillDash ..owned_props />
    },
    implicit_clone::unsync::IString::Static("EnvelopePaper") => html! {
      <MD_EnvelopePaper ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ChatQuote") => html! {
      <MD_ChatQuote ..owned_props />
    },
    implicit_clone::unsync::IString::Static("InboxFill") => html! {
      <MD_InboxFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FileEarmarkBarGraphFill") => html! {
      <MD_FileEarmarkBarGraphFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("HddStackFill") => html! {
      <MD_HddStackFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Snapchat") => html! {
      <MD_Snapchat ..owned_props />
    },
    implicit_clone::unsync::IString::Static("QuestionDiamondFill") => html! {
      <MD_QuestionDiamondFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Person") => html! {
      <MD_Person ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Box2") => html! {
      <MD_Box2 ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Calendar2RangeFill") => html! {
      <MD_Calendar2RangeFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Code") => html! {
      <MD_Code ..owned_props />
    },
    implicit_clone::unsync::IString::Static("PersonDash") => html! {
      <MD_PersonDash ..owned_props />
    },
    implicit_clone::unsync::IString::Static("CloudSnow") => html! {
      <MD_CloudSnow ..owned_props />
    },
    implicit_clone::unsync::IString::Static("VectorPen") => html! {
      <MD_VectorPen ..owned_props />
    },
    implicit_clone::unsync::IString::Static("TextParagraph") => html! {
      <MD_TextParagraph ..owned_props />
    },
    implicit_clone::unsync::IString::Static("BorderOuter") => html! {
      <MD_BorderOuter ..owned_props />
    },
    implicit_clone::unsync::IString::Static("LayoutTextWindowReverse") => html! {
      <MD_LayoutTextWindowReverse ..owned_props />
    },
    implicit_clone::unsync::IString::Static("BoxArrowUp") => html! {
      <MD_BoxArrowUp ..owned_props />
    },
    implicit_clone::unsync::IString::Static("HouseXFill") => html! {
      <MD_HouseXFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FullscreenExit") => html! {
      <MD_FullscreenExit ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ArrowsAngleExpand") => html! {
      <MD_ArrowsAngleExpand ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ImageFill") => html! {
      <MD_ImageFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("EyeFill") => html! {
      <MD_EyeFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("PlusCircleDotted") => html! {
      <MD_PlusCircleDotted ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Boxes") => html! {
      <MD_Boxes ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Houses") => html! {
      <MD_Houses ..owned_props />
    },
    implicit_clone::unsync::IString::Static("HouseExclamationFill") => html! {
      <MD_HouseExclamationFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Save2Fill") => html! {
      <MD_Save2Fill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("EaselFill") => html! {
      <MD_EaselFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("PersonLock") => html! {
      <MD_PersonLock ..owned_props />
    },
    implicit_clone::unsync::IString::Static("PersonVideo2") => html! {
      <MD_PersonVideo2 ..owned_props />
    },
    implicit_clone::unsync::IString::Static("EvFrontFill") => html! {
      <MD_EvFrontFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ListNested") => html! {
      <MD_ListNested ..owned_props />
    },
    implicit_clone::unsync::IString::Static("RewindFill") => html! {
      <MD_RewindFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("DropletFill") => html! {
      <MD_DropletFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Power") => html! {
      <MD_Power ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Screwdriver") => html! {
      <MD_Screwdriver ..owned_props />
    },
    implicit_clone::unsync::IString::Static("SegmentedNav") => html! {
      <MD_SegmentedNav ..owned_props />
    },
    implicit_clone::unsync::IString::Static("PhoneVibrateFill") => html! {
      <MD_PhoneVibrateFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("CheckAll") => html! {
      <MD_CheckAll ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Dice4Fill") => html! {
      <MD_Dice4Fill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("MenuAppFill") => html! {
      <MD_MenuAppFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("EnvelopePlusFill") => html! {
      <MD_EnvelopePlusFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("BorderLeft") => html! {
      <MD_BorderLeft ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Badge3dFill") => html! {
      <MD_Badge3dFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Calendar3Week") => html! {
      <MD_Calendar3Week ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ChatText") => html! {
      <MD_ChatText ..owned_props />
    },
    implicit_clone::unsync::IString::Static("CaretLeftSquare") => html! {
      <MD_CaretLeftSquare ..owned_props />
    },
    implicit_clone::unsync::IString::Static("SlashCircleFill") => html! {
      <MD_SlashCircleFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("WrenchAdjustableCircle") => html! {
      <MD_WrenchAdjustableCircle ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ChatDots") => html! {
      <MD_ChatDots ..owned_props />
    },
    implicit_clone::unsync::IString::Static("BadgeVr") => html! {
      <MD_BadgeVr ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FileEarmarkBreakFill") => html! {
      <MD_FileEarmarkBreakFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("CloudHail") => html! {
      <MD_CloudHail ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FolderX") => html! {
      <MD_FolderX ..owned_props />
    },
    implicit_clone::unsync::IString::Static("EmojiWink") => html! {
      <MD_EmojiWink ..owned_props />
    },
    implicit_clone::unsync::IString::Static("4SquareFill") => html! {
      <MD_4SquareFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ArrowDownLeft") => html! {
      <MD_ArrowDownLeft ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FileEarmarkPostFill") => html! {
      <MD_FileEarmarkPostFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("People") => html! {
      <MD_People ..owned_props />
    },
    implicit_clone::unsync::IString::Static("HSquare") => html! {
      <MD_HSquare ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ChatRightDots") => html! {
      <MD_ChatRightDots ..owned_props />
    },
    implicit_clone::unsync::IString::Static("UsbDrive") => html! {
      <MD_UsbDrive ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Signal") => html! {
      <MD_Signal ..owned_props />
    },
    implicit_clone::unsync::IString::Static("GenderFemale") => html! {
      <MD_GenderFemale ..owned_props />
    },
    implicit_clone::unsync::IString::Static("PersonSquare") => html! {
      <MD_PersonSquare ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Truck") => html! {
      <MD_Truck ..owned_props />
    },
    implicit_clone::unsync::IString::Static("SkipBackward") => html! {
      <MD_SkipBackward ..owned_props />
    },
    implicit_clone::unsync::IString::Static("PlusSquareFill") => html! {
      <MD_PlusSquareFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ArrowDownCircleFill") => html! {
      <MD_ArrowDownCircleFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("File") => html! {
      <MD_File ..owned_props />
    },
    implicit_clone::unsync::IString::Static("App") => html! {
      <MD_App ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Grid3x2Gap") => html! {
      <MD_Grid3x2Gap ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Windows") => html! {
      <MD_Windows ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FileZipFill") => html! {
      <MD_FileZipFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("VolumeMute") => html! {
      <MD_VolumeMute ..owned_props />
    },
    implicit_clone::unsync::IString::Static("4Circle") => html! {
      <MD_4Circle ..owned_props />
    },
    implicit_clone::unsync::IString::Static("SignRailroad") => html! {
      <MD_SignRailroad ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FiletypeMp4") => html! {
      <MD_FiletypeMp4 ..owned_props />
    },
    implicit_clone::unsync::IString::Static("HeartbreakFill") => html! {
      <MD_HeartbreakFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("CaretDown") => html! {
      <MD_CaretDown ..owned_props />
    },
    implicit_clone::unsync::IString::Static("MicrosoftTeams") => html! {
      <MD_MicrosoftTeams ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ShieldPlus") => html! {
      <MD_ShieldPlus ..owned_props />
    },
    implicit_clone::unsync::IString::Static("SymmetryHorizontal") => html! {
      <MD_SymmetryHorizontal ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Clipboard2XFill") => html! {
      <MD_Clipboard2XFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FileRichtextFill") => html! {
      <MD_FileRichtextFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ArrowDownRightCircleFill") => html! {
      <MD_ArrowDownRightCircleFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("MapFill") => html! {
      <MD_MapFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("CompassFill") => html! {
      <MD_CompassFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("EmojiWinkFill") => html! {
      <MD_EmojiWinkFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FilePlay") => html! {
      <MD_FilePlay ..owned_props />
    },
    implicit_clone::unsync::IString::Static("GraphUp") => html! {
      <MD_GraphUp ..owned_props />
    },
    implicit_clone::unsync::IString::Static("SignTurnRightFill") => html! {
      <MD_SignTurnRightFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("5SquareFill") => html! {
      <MD_5SquareFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("SkipBackwardCircle") => html! {
      <MD_SkipBackwardCircle ..owned_props />
    },
    implicit_clone::unsync::IString::Static("SortAlphaUp") => html! {
      <MD_SortAlphaUp ..owned_props />
    },
    implicit_clone::unsync::IString::Static("HexagonFill") => html! {
      <MD_HexagonFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ChatSquareText") => html! {
      <MD_ChatSquareText ..owned_props />
    },
    implicit_clone::unsync::IString::Static("SignNoLeftTurn") => html! {
      <MD_SignNoLeftTurn ..owned_props />
    },
    implicit_clone::unsync::IString::Static("TvFill") => html! {
      <MD_TvFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("MenuButtonFill") => html! {
      <MD_MenuButtonFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("JournalMinus") => html! {
      <MD_JournalMinus ..owned_props />
    },
    implicit_clone::unsync::IString::Static("CartPlusFill") => html! {
      <MD_CartPlusFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("EnvelopeAt") => html! {
      <MD_EnvelopeAt ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Journal") => html! {
      <MD_Journal ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Percent") => html! {
      <MD_Percent ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Rulers") => html! {
      <MD_Rulers ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Toggle2Off") => html! {
      <MD_Toggle2Off ..owned_props />
    },
    implicit_clone::unsync::IString::Static("BuildingsFill") => html! {
      <MD_BuildingsFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Mouse3Fill") => html! {
      <MD_Mouse3Fill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FiletypeTiff") => html! {
      <MD_FiletypeTiff ..owned_props />
    },
    implicit_clone::unsync::IString::Static("HouseFill") => html! {
      <MD_HouseFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("BoomboxFill") => html! {
      <MD_BoomboxFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("BuildingFillDash") => html! {
      <MD_BuildingFillDash ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Clipboard2DataFill") => html! {
      <MD_Clipboard2DataFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Filter") => html! {
      <MD_Filter ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Sliders2") => html! {
      <MD_Sliders2 ..owned_props />
    },
    implicit_clone::unsync::IString::Static("SignTurnLeft") => html! {
      <MD_SignTurnLeft ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Radioactive") => html! {
      <MD_Radioactive ..owned_props />
    },
    implicit_clone::unsync::IString::Static("CaretLeftFill") => html! {
      <MD_CaretLeftFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("7SquareFill") => html! {
      <MD_7SquareFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("CloudHaze2") => html! {
      <MD_CloudHaze2 ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ArrowRight") => html! {
      <MD_ArrowRight ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Facebook") => html! {
      <MD_Facebook ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Magnet") => html! {
      <MD_Magnet ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FileSlidesFill") => html! {
      <MD_FileSlidesFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FileExcel") => html! {
      <MD_FileExcel ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ArrowRightSquareFill") => html! {
      <MD_ArrowRightSquareFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("CodeSquare") => html! {
      <MD_CodeSquare ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Calendar2XFill") => html! {
      <MD_Calendar2XFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("HouseHeart") => html! {
      <MD_HouseHeart ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FileEarmarkImage") => html! {
      <MD_FileEarmarkImage ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Bucket") => html! {
      <MD_Bucket ..owned_props />
    },
    implicit_clone::unsync::IString::Static("CupStraw") => html! {
      <MD_CupStraw ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FileEarmarkLock2Fill") => html! {
      <MD_FileEarmarkLock2Fill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("MoonStarsFill") => html! {
      <MD_MoonStarsFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FilePdfFill") => html! {
      <MD_FilePdfFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Gear") => html! {
      <MD_Gear ..owned_props />
    },
    implicit_clone::unsync::IString::Static("BoxArrowInRight") => html! {
      <MD_BoxArrowInRight ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FiletypeHtml") => html! {
      <MD_FiletypeHtml ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Table") => html! {
      <MD_Table ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ArrowDownRightSquare") => html! {
      <MD_ArrowDownRightSquare ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Dice3Fill") => html! {
      <MD_Dice3Fill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("DatabaseDash") => html! {
      <MD_DatabaseDash ..owned_props />
    },
    implicit_clone::unsync::IString::Static("2Circle") => html! {
      <MD_2Circle ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Clipboard2Fill") => html! {
      <MD_Clipboard2Fill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Rewind") => html! {
      <MD_Rewind ..owned_props />
    },
    implicit_clone::unsync::IString::Static("CreditCardFill") => html! {
      <MD_CreditCardFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Dot") => html! {
      <MD_Dot ..owned_props />
    },
    implicit_clone::unsync::IString::Static("NutFill") => html! {
      <MD_NutFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FiletypeTsx") => html! {
      <MD_FiletypeTsx ..owned_props />
    },
    implicit_clone::unsync::IString::Static("BookmarkDash") => html! {
      <MD_BookmarkDash ..owned_props />
    },
    implicit_clone::unsync::IString::Static("CloudFog") => html! {
      <MD_CloudFog ..owned_props />
    },
    implicit_clone::unsync::IString::Static("RocketFill") => html! {
      <MD_RocketFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FileEarmarkCheckFill") => html! {
      <MD_FileEarmarkCheckFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Bluetooth") => html! {
      <MD_Bluetooth ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Shuffle") => html! {
      <MD_Shuffle ..owned_props />
    },
    implicit_clone::unsync::IString::Static("BadgeAr") => html! {
      <MD_BadgeAr ..owned_props />
    },
    implicit_clone::unsync::IString::Static("PipFill") => html! {
      <MD_PipFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Bootstrap") => html! {
      <MD_Bootstrap ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Film") => html! {
      <MD_Film ..owned_props />
    },
    implicit_clone::unsync::IString::Static("BatteryCharging") => html! {
      <MD_BatteryCharging ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Clipboard2Plus") => html! {
      <MD_Clipboard2Plus ..owned_props />
    },
    implicit_clone::unsync::IString::Static("CloudSleetFill") => html! {
      <MD_CloudSleetFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("SkipForwardFill") => html! {
      <MD_SkipForwardFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ChevronCompactLeft") => html! {
      <MD_ChevronCompactLeft ..owned_props />
    },
    implicit_clone::unsync::IString::Static("MicFill") => html! {
      <MD_MicFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Safe2Fill") => html! {
      <MD_Safe2Fill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("KanbanFill") => html! {
      <MD_KanbanFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("RewindBtnFill") => html! {
      <MD_RewindBtnFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ReplyFill") => html! {
      <MD_ReplyFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Twitter") => html! {
      <MD_Twitter ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ToggleOn") => html! {
      <MD_ToggleOn ..owned_props />
    },
    implicit_clone::unsync::IString::Static("CurrencyRupee") => html! {
      <MD_CurrencyRupee ..owned_props />
    },
    implicit_clone::unsync::IString::Static("BadgeSdFill") => html! {
      <MD_BadgeSdFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("PersonVcard") => html! {
      <MD_PersonVcard ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Cpu") => html! {
      <MD_Cpu ..owned_props />
    },
    implicit_clone::unsync::IString::Static("CaretDownSquare") => html! {
      <MD_CaretDownSquare ..owned_props />
    },
    implicit_clone::unsync::IString::Static("NodeMinus") => html! {
      <MD_NodeMinus ..owned_props />
    },
    implicit_clone::unsync::IString::Static("HandbagFill") => html! {
      <MD_HandbagFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("DoorOpenFill") => html! {
      <MD_DoorOpenFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("EvFront") => html! {
      <MD_EvFront ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ArrowBarUp") => html! {
      <MD_ArrowBarUp ..owned_props />
    },
    implicit_clone::unsync::IString::Static("PlayBtn") => html! {
      <MD_PlayBtn ..owned_props />
    },
    implicit_clone::unsync::IString::Static("SendPlus") => html! {
      <MD_SendPlus ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ChevronBarDown") => html! {
      <MD_ChevronBarDown ..owned_props />
    },
    implicit_clone::unsync::IString::Static("CaretDownFill") => html! {
      <MD_CaretDownFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("MicMute") => html! {
      <MD_MicMute ..owned_props />
    },
    implicit_clone::unsync::IString::Static("PersonFillDown") => html! {
      <MD_PersonFillDown ..owned_props />
    },
    implicit_clone::unsync::IString::Static("TypeH1") => html! {
      <MD_TypeH1 ..owned_props />
    },
    implicit_clone::unsync::IString::Static("PlayBtnFill") => html! {
      <MD_PlayBtnFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("9Circle") => html! {
      <MD_9Circle ..owned_props />
    },
    implicit_clone::unsync::IString::Static("DatabaseFillDown") => html! {
      <MD_DatabaseFillDown ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Unlock") => html! {
      <MD_Unlock ..owned_props />
    },
    implicit_clone::unsync::IString::Static("BrightnessAltHigh") => html! {
      <MD_BrightnessAltHigh ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ChatDotsFill") => html! {
      <MD_ChatDotsFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Eyeglasses") => html! {
      <MD_Eyeglasses ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ArrowsMove") => html! {
      <MD_ArrowsMove ..owned_props />
    },
    implicit_clone::unsync::IString::Static("MouseFill") => html! {
      <MD_MouseFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FileEarmarkZip") => html! {
      <MD_FileEarmarkZip ..owned_props />
    },
    implicit_clone::unsync::IString::Static("SlashCircle") => html! {
      <MD_SlashCircle ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Alt") => html! {
      <MD_Alt ..owned_props />
    },
    implicit_clone::unsync::IString::Static("SignIntersectionYFill") => html! {
      <MD_SignIntersectionYFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("BrightnessHighFill") => html! {
      <MD_BrightnessHighFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Lock") => html! {
      <MD_Lock ..owned_props />
    },
    implicit_clone::unsync::IString::Static("CloudMoon") => html! {
      <MD_CloudMoon ..owned_props />
    },
    implicit_clone::unsync::IString::Static("HandIndexFill") => html! {
      <MD_HandIndexFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("BorderStyle") => html! {
      <MD_BorderStyle ..owned_props />
    },
    implicit_clone::unsync::IString::Static("CloudRainFill") => html! {
      <MD_CloudRainFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Telegram") => html! {
      <MD_Telegram ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Tag") => html! {
      <MD_Tag ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Trello") => html! {
      <MD_Trello ..owned_props />
    },
    implicit_clone::unsync::IString::Static("CalendarMonth") => html! {
      <MD_CalendarMonth ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Safe2") => html! {
      <MD_Safe2 ..owned_props />
    },
    implicit_clone::unsync::IString::Static("BuildingDash") => html! {
      <MD_BuildingDash ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FiletypeJs") => html! {
      <MD_FiletypeJs ..owned_props />
    },
    implicit_clone::unsync::IString::Static("BoxArrowRight") => html! {
      <MD_BoxArrowRight ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Messenger") => html! {
      <MD_Messenger ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ArrowLeft") => html! {
      <MD_ArrowLeft ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Palette2") => html! {
      <MD_Palette2 ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FileEarmarkXFill") => html! {
      <MD_FileEarmarkXFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FastForwardBtn") => html! {
      <MD_FastForwardBtn ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FileDiff") => html! {
      <MD_FileDiff ..owned_props />
    },
    implicit_clone::unsync::IString::Static("BadgeVrFill") => html! {
      <MD_BadgeVrFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Broadcast") => html! {
      <MD_Broadcast ..owned_props />
    },
    implicit_clone::unsync::IString::Static("PauseCircle") => html! {
      <MD_PauseCircle ..owned_props />
    },
    implicit_clone::unsync::IString::Static("PauseFill") => html! {
      <MD_PauseFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("MotherboardFill") => html! {
      <MD_MotherboardFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ArrowRightSquare") => html! {
      <MD_ArrowRightSquare ..owned_props />
    },
    implicit_clone::unsync::IString::Static("DatabaseFillAdd") => html! {
      <MD_DatabaseFillAdd ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FilterSquareFill") => html! {
      <MD_FilterSquareFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Virus2") => html! {
      <MD_Virus2 ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ChatTextFill") => html! {
      <MD_ChatTextFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("SkipForwardCircleFill") => html! {
      <MD_SkipForwardCircleFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ChatRightQuoteFill") => html! {
      <MD_ChatRightQuoteFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("PeaceFill") => html! {
      <MD_PeaceFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("PinFill") => html! {
      <MD_PinFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Box2Fill") => html! {
      <MD_Box2Fill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ChatSquareHeartFill") => html! {
      <MD_ChatSquareHeartFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("BarChartLine") => html! {
      <MD_BarChartLine ..owned_props />
    },
    implicit_clone::unsync::IString::Static("CalendarHeart") => html! {
      <MD_CalendarHeart ..owned_props />
    },
    implicit_clone::unsync::IString::Static("PhoneFill") => html! {
      <MD_PhoneFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("SendSlash") => html! {
      <MD_SendSlash ..owned_props />
    },
    implicit_clone::unsync::IString::Static("TagsFill") => html! {
      <MD_TagsFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Crop") => html! {
      <MD_Crop ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FiletypeTtf") => html! {
      <MD_FiletypeTtf ..owned_props />
    },
    implicit_clone::unsync::IString::Static("MenuButtonWide") => html! {
      <MD_MenuButtonWide ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Award") => html! {
      <MD_Award ..owned_props />
    },
    implicit_clone::unsync::IString::Static("3SquareFill") => html! {
      <MD_3SquareFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("GripHorizontal") => html! {
      <MD_GripHorizontal ..owned_props />
    },
    implicit_clone::unsync::IString::Static("9SquareFill") => html! {
      <MD_9SquareFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FileMusic") => html! {
      <MD_FileMusic ..owned_props />
    },
    implicit_clone::unsync::IString::Static("CursorFill") => html! {
      <MD_CursorFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("PersonVideo3") => html! {
      <MD_PersonVideo3 ..owned_props />
    },
    implicit_clone::unsync::IString::Static("HddFill") => html! {
      <MD_HddFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("SuitClub") => html! {
      <MD_SuitClub ..owned_props />
    },
    implicit_clone::unsync::IString::Static("SuitClubFill") => html! {
      <MD_SuitClubFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Collection") => html! {
      <MD_Collection ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Option") => html! {
      <MD_Option ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ChevronDoubleLeft") => html! {
      <MD_ChevronDoubleLeft ..owned_props />
    },
    implicit_clone::unsync::IString::Static("SafeFill") => html! {
      <MD_SafeFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("BrushFill") => html! {
      <MD_BrushFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("6Square") => html! {
      <MD_6Square ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ShopWindow") => html! {
      <MD_ShopWindow ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ShieldExclamation") => html! {
      <MD_ShieldExclamation ..owned_props />
    },
    implicit_clone::unsync::IString::Static("RecordCircleFill") => html! {
      <MD_RecordCircleFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FilePptFill") => html! {
      <MD_FilePptFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Dice1Fill") => html! {
      <MD_Dice1Fill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ArrowBarLeft") => html! {
      <MD_ArrowBarLeft ..owned_props />
    },
    implicit_clone::unsync::IString::Static("HddNetwork") => html! {
      <MD_HddNetwork ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ClockFill") => html! {
      <MD_ClockFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("TabletFill") => html! {
      <MD_TabletFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("HeartPulseFill") => html! {
      <MD_HeartPulseFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FiletypePdf") => html! {
      <MD_FiletypePdf ..owned_props />
    },
    implicit_clone::unsync::IString::Static("BoxArrowInDown") => html! {
      <MD_BoxArrowInDown ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Repeat") => html! {
      <MD_Repeat ..owned_props />
    },
    implicit_clone::unsync::IString::Static("BasketFill") => html! {
      <MD_BasketFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Globe2") => html! {
      <MD_Globe2 ..owned_props />
    },
    implicit_clone::unsync::IString::Static("SunFill") => html! {
      <MD_SunFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Gift") => html! {
      <MD_Gift ..owned_props />
    },
    implicit_clone::unsync::IString::Static("LayoutSidebarInsetReverse") => html! {
      <MD_LayoutSidebarInsetReverse ..owned_props />
    },
    implicit_clone::unsync::IString::Static("PlugFill") => html! {
      <MD_PlugFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Phone") => html! {
      <MD_Phone ..owned_props />
    },
    implicit_clone::unsync::IString::Static("AspectRatio") => html! {
      <MD_AspectRatio ..owned_props />
    },
    implicit_clone::unsync::IString::Static("TruckFrontFill") => html! {
      <MD_TruckFrontFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Behance") => html! {
      <MD_Behance ..owned_props />
    },
    implicit_clone::unsync::IString::Static("UniversalAccessCircle") => html! {
      <MD_UniversalAccessCircle ..owned_props />
    },
    implicit_clone::unsync::IString::Static("GraphDownArrow") => html! {
      <MD_GraphDownArrow ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FileEarmarkZipFill") => html! {
      <MD_FileEarmarkZipFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("BandaidFill") => html! {
      <MD_BandaidFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ArrowDownSquare") => html! {
      <MD_ArrowDownSquare ..owned_props />
    },
    implicit_clone::unsync::IString::Static("CalendarCheckFill") => html! {
      <MD_CalendarCheckFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("BookFill") => html! {
      <MD_BookFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("UsbMicroFill") => html! {
      <MD_UsbMicroFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Controller") => html! {
      <MD_Controller ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Fonts") => html! {
      <MD_Fonts ..owned_props />
    },
    implicit_clone::unsync::IString::Static("EnvelopeExclamation") => html! {
      <MD_EnvelopeExclamation ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Pause") => html! {
      <MD_Pause ..owned_props />
    },
    implicit_clone::unsync::IString::Static("BookmarkDashFill") => html! {
      <MD_BookmarkDashFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("BoxArrowDownLeft") => html! {
      <MD_BoxArrowDownLeft ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Front") => html! {
      <MD_Front ..owned_props />
    },
    implicit_clone::unsync::IString::Static("SortNumericDownAlt") => html! {
      <MD_SortNumericDownAlt ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Dice6") => html! {
      <MD_Dice6 ..owned_props />
    },
    implicit_clone::unsync::IString::Static("RocketTakeoffFill") => html! {
      <MD_RocketTakeoffFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Folder2") => html! {
      <MD_Folder2 ..owned_props />
    },
    implicit_clone::unsync::IString::Static("WindowDesktop") => html! {
      <MD_WindowDesktop ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FileMinus") => html! {
      <MD_FileMinus ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Mailbox") => html! {
      <MD_Mailbox ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ThermometerHalf") => html! {
      <MD_ThermometerHalf ..owned_props />
    },
    implicit_clone::unsync::IString::Static("PersonCheckFill") => html! {
      <MD_PersonCheckFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("PlusCircleFill") => html! {
      <MD_PlusCircleFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Flower2") => html! {
      <MD_Flower2 ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ArrowsExpand") => html! {
      <MD_ArrowsExpand ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ShieldFillPlus") => html! {
      <MD_ShieldFillPlus ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Flower3") => html! {
      <MD_Flower3 ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FiletypeMov") => html! {
      <MD_FiletypeMov ..owned_props />
    },
    implicit_clone::unsync::IString::Static("PentagonFill") => html! {
      <MD_PentagonFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("CloudDownloadFill") => html! {
      <MD_CloudDownloadFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Shift") => html! {
      <MD_Shift ..owned_props />
    },
    implicit_clone::unsync::IString::Static("UsbMini") => html! {
      <MD_UsbMini ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FileEarmarkCheck") => html! {
      <MD_FileEarmarkCheck ..owned_props />
    },
    implicit_clone::unsync::IString::Static("4CircleFill") => html! {
      <MD_4CircleFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("NodeMinusFill") => html! {
      <MD_NodeMinusFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Check2All") => html! {
      <MD_Check2All ..owned_props />
    },
    implicit_clone::unsync::IString::Static("SignStop") => html! {
      <MD_SignStop ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Type") => html! {
      <MD_Type ..owned_props />
    },
    implicit_clone::unsync::IString::Static("LayoutSidebar") => html! {
      <MD_LayoutSidebar ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FileArrowDown") => html! {
      <MD_FileArrowDown ..owned_props />
    },
    implicit_clone::unsync::IString::Static("BagDash") => html! {
      <MD_BagDash ..owned_props />
    },
    implicit_clone::unsync::IString::Static("CpuFill") => html! {
      <MD_CpuFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("PCircle") => html! {
      <MD_PCircle ..owned_props />
    },
    implicit_clone::unsync::IString::Static("RouterFill") => html! {
      <MD_RouterFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("BalloonHeartFill") => html! {
      <MD_BalloonHeartFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FileEarmarkEaselFill") => html! {
      <MD_FileEarmarkEaselFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Clipboard2Data") => html! {
      <MD_Clipboard2Data ..owned_props />
    },
    implicit_clone::unsync::IString::Static("PersonBoundingBox") => html! {
      <MD_PersonBoundingBox ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Kanban") => html! {
      <MD_Kanban ..owned_props />
    },
    implicit_clone::unsync::IString::Static("BoxArrowInDownLeft") => html! {
      <MD_BoxArrowInDownLeft ..owned_props />
    },
    implicit_clone::unsync::IString::Static("TelephoneFill") => html! {
      <MD_TelephoneFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("PersonExclamation") => html! {
      <MD_PersonExclamation ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FileCheck") => html! {
      <MD_FileCheck ..owned_props />
    },
    implicit_clone::unsync::IString::Static("HouseAdd") => html! {
      <MD_HouseAdd ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FileEaselFill") => html! {
      <MD_FileEaselFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Shop") => html! {
      <MD_Shop ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ArrowCounterclockwise") => html! {
      <MD_ArrowCounterclockwise ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Bullseye") => html! {
      <MD_Bullseye ..owned_props />
    },
    implicit_clone::unsync::IString::Static("GlobeAsiaAustralia") => html! {
      <MD_GlobeAsiaAustralia ..owned_props />
    },
    implicit_clone::unsync::IString::Static("PlusSlashMinus") => html! {
      <MD_PlusSlashMinus ..owned_props />
    },
    implicit_clone::unsync::IString::Static("PassFill") => html! {
      <MD_PassFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("EnvelopeCheck") => html! {
      <MD_EnvelopeCheck ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Wind") => html! {
      <MD_Wind ..owned_props />
    },
    implicit_clone::unsync::IString::Static("5CircleFill") => html! {
      <MD_5CircleFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Geo") => html! {
      <MD_Geo ..owned_props />
    },
    implicit_clone::unsync::IString::Static("PciCard") => html! {
      <MD_PciCard ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Calendar4Event") => html! {
      <MD_Calendar4Event ..owned_props />
    },
    implicit_clone::unsync::IString::Static("MusicPlayerFill") => html! {
      <MD_MusicPlayerFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FiletypeJava") => html! {
      <MD_FiletypeJava ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FiletypeCss") => html! {
      <MD_FiletypeCss ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FileEarmarkDiff") => html! {
      <MD_FileEarmarkDiff ..owned_props />
    },
    implicit_clone::unsync::IString::Static("BuildingFillCheck") => html! {
      <MD_BuildingFillCheck ..owned_props />
    },
    implicit_clone::unsync::IString::Static("EnvelopeOpen") => html! {
      <MD_EnvelopeOpen ..owned_props />
    },
    implicit_clone::unsync::IString::Static("PersonFillUp") => html! {
      <MD_PersonFillUp ..owned_props />
    },
    implicit_clone::unsync::IString::Static("LaptopFill") => html! {
      <MD_LaptopFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("DeviceSsdFill") => html! {
      <MD_DeviceSsdFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("PersonVideo") => html! {
      <MD_PersonVideo ..owned_props />
    },
    implicit_clone::unsync::IString::Static("SkipStartFill") => html! {
      <MD_SkipStartFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Camera") => html! {
      <MD_Camera ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Ubuntu") => html! {
      <MD_Ubuntu ..owned_props />
    },
    implicit_clone::unsync::IString::Static("CloudUploadFill") => html! {
      <MD_CloudUploadFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("DatabaseUp") => html! {
      <MD_DatabaseUp ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Fire") => html! {
      <MD_Fire ..owned_props />
    },
    implicit_clone::unsync::IString::Static("BorderInner") => html! {
      <MD_BorderInner ..owned_props />
    },
    implicit_clone::unsync::IString::Static("BadgeWc") => html! {
      <MD_BadgeWc ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ExclamationSquareFill") => html! {
      <MD_ExclamationSquareFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("BrightnessAltHighFill") => html! {
      <MD_BrightnessAltHighFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Bandaid") => html! {
      <MD_Bandaid ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Vr") => html! {
      <MD_Vr ..owned_props />
    },
    implicit_clone::unsync::IString::Static("PlusLg") => html! {
      <MD_PlusLg ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Webcam") => html! {
      <MD_Webcam ..owned_props />
    },
    implicit_clone::unsync::IString::Static("PersonFill") => html! {
      <MD_PersonFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("PiggyBankFill") => html! {
      <MD_PiggyBankFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("SignTurnRight") => html! {
      <MD_SignTurnRight ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Calendar2CheckFill") => html! {
      <MD_Calendar2CheckFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("BoxArrowInUpRight") => html! {
      <MD_BoxArrowInUpRight ..owned_props />
    },
    implicit_clone::unsync::IString::Static("BoundingBoxCircles") => html! {
      <MD_BoundingBoxCircles ..owned_props />
    },
    implicit_clone::unsync::IString::Static("SendX") => html! {
      <MD_SendX ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ChevronBarLeft") => html! {
      <MD_ChevronBarLeft ..owned_props />
    },
    implicit_clone::unsync::IString::Static("BarChart") => html! {
      <MD_BarChart ..owned_props />
    },
    implicit_clone::unsync::IString::Static("DashSquare") => html! {
      <MD_DashSquare ..owned_props />
    },
    implicit_clone::unsync::IString::Static("9Square") => html! {
      <MD_9Square ..owned_props />
    },
    implicit_clone::unsync::IString::Static("PhoneVibrate") => html! {
      <MD_PhoneVibrate ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Outlet") => html! {
      <MD_Outlet ..owned_props />
    },
    implicit_clone::unsync::IString::Static("LightningCharge") => html! {
      <MD_LightningCharge ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ArrowUpRight") => html! {
      <MD_ArrowUpRight ..owned_props />
    },
    implicit_clone::unsync::IString::Static("HandThumbsDownFill") => html! {
      <MD_HandThumbsDownFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Dice3") => html! {
      <MD_Dice3 ..owned_props />
    },
    implicit_clone::unsync::IString::Static("GripVertical") => html! {
      <MD_GripVertical ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Strava") => html! {
      <MD_Strava ..owned_props />
    },
    implicit_clone::unsync::IString::Static("EnvelopeOpenHeartFill") => html! {
      <MD_EnvelopeOpenHeartFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FiletypePhp") => html! {
      <MD_FiletypePhp ..owned_props />
    },
    implicit_clone::unsync::IString::Static("WebcamFill") => html! {
      <MD_WebcamFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("SunriseFill") => html! {
      <MD_SunriseFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("EmojiNeutralFill") => html! {
      <MD_EmojiNeutralFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Subtract") => html! {
      <MD_Subtract ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Briefcase") => html! {
      <MD_Briefcase ..owned_props />
    },
    implicit_clone::unsync::IString::Static("BrowserFirefox") => html! {
      <MD_BrowserFirefox ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Building") => html! {
      <MD_Building ..owned_props />
    },
    implicit_clone::unsync::IString::Static("SignYieldFill") => html! {
      <MD_SignYieldFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("CaretUpSquareFill") => html! {
      <MD_CaretUpSquareFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FolderPlus") => html! {
      <MD_FolderPlus ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Dpad") => html! {
      <MD_Dpad ..owned_props />
    },
    implicit_clone::unsync::IString::Static("CardText") => html! {
      <MD_CardText ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Calendar4Week") => html! {
      <MD_Calendar4Week ..owned_props />
    },
    implicit_clone::unsync::IString::Static("EmojiDizzyFill") => html! {
      <MD_EmojiDizzyFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("PcDisplayHorizontal") => html! {
      <MD_PcDisplayHorizontal ..owned_props />
    },
    implicit_clone::unsync::IString::Static("BoxArrowInUp") => html! {
      <MD_BoxArrowInUp ..owned_props />
    },
    implicit_clone::unsync::IString::Static("MenuDown") => html! {
      <MD_MenuDown ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Badge8k") => html! {
      <MD_Badge8k ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ExclamationDiamondFill") => html! {
      <MD_ExclamationDiamondFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Reddit") => html! {
      <MD_Reddit ..owned_props />
    },
    implicit_clone::unsync::IString::Static("PatchExclamationFill") => html! {
      <MD_PatchExclamationFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("EnvelopeHeartFill") => html! {
      <MD_EnvelopeHeartFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Earbuds") => html! {
      <MD_Earbuds ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Camera2") => html! {
      <MD_Camera2 ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Grid3x3") => html! {
      <MD_Grid3x3 ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Toggle2On") => html! {
      <MD_Toggle2On ..owned_props />
    },
    implicit_clone::unsync::IString::Static("SignIntersectionSide") => html! {
      <MD_SignIntersectionSide ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FiletypeMp3") => html! {
      <MD_FiletypeMp3 ..owned_props />
    },
    implicit_clone::unsync::IString::Static("MegaphoneFill") => html! {
      <MD_MegaphoneFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("PinMap") => html! {
      <MD_PinMap ..owned_props />
    },
    implicit_clone::unsync::IString::Static("HouseUpFill") => html! {
      <MD_HouseUpFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("CardList") => html! {
      <MD_CardList ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Motherboard") => html! {
      <MD_Motherboard ..owned_props />
    },
    implicit_clone::unsync::IString::Static("CreditCard") => html! {
      <MD_CreditCard ..owned_props />
    },
    implicit_clone::unsync::IString::Static("123") => html! {
      <MD_123 ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FiletypeXls") => html! {
      <MD_FiletypeXls ..owned_props />
    },
    implicit_clone::unsync::IString::Static("StopFill") => html! {
      <MD_StopFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Regex") => html! {
      <MD_Regex ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Trash3") => html! {
      <MD_Trash3 ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ArrowClockwise") => html! {
      <MD_ArrowClockwise ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Trophy") => html! {
      <MD_Trophy ..owned_props />
    },
    implicit_clone::unsync::IString::Static("MagnetFill") => html! {
      <MD_MagnetFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Cone") => html! {
      <MD_Cone ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ViewList") => html! {
      <MD_ViewList ..owned_props />
    },
    implicit_clone::unsync::IString::Static("TypeUnderline") => html! {
      <MD_TypeUnderline ..owned_props />
    },
    implicit_clone::unsync::IString::Static("InfoLg") => html! {
      <MD_InfoLg ..owned_props />
    },
    implicit_clone::unsync::IString::Static("CheckCircleFill") => html! {
      <MD_CheckCircleFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("SendFill") => html! {
      <MD_SendFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FileEarmarkTextFill") => html! {
      <MD_FileEarmarkTextFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Diagram2Fill") => html! {
      <MD_Diagram2Fill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Alipay") => html! {
      <MD_Alipay ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ArrowLeftCircle") => html! {
      <MD_ArrowLeftCircle ..owned_props />
    },
    implicit_clone::unsync::IString::Static("TicketDetailedFill") => html! {
      <MD_TicketDetailedFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FiletypeDocx") => html! {
      <MD_FiletypeDocx ..owned_props />
    },
    implicit_clone::unsync::IString::Static("PhoneLandscapeFill") => html! {
      <MD_PhoneLandscapeFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ChevronCompactUp") => html! {
      <MD_ChevronCompactUp ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Shield") => html! {
      <MD_Shield ..owned_props />
    },
    implicit_clone::unsync::IString::Static("CaretLeftSquareFill") => html! {
      <MD_CaretLeftSquareFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Wechat") => html! {
      <MD_Wechat ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ArrowThroughHeartFill") => html! {
      <MD_ArrowThroughHeartFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("QrCodeScan") => html! {
      <MD_QrCodeScan ..owned_props />
    },
    implicit_clone::unsync::IString::Static("SendCheckFill") => html! {
      <MD_SendCheckFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ChatRightFill") => html! {
      <MD_ChatRightFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Link45deg") => html! {
      <MD_Link45deg ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FileImage") => html! {
      <MD_FileImage ..owned_props />
    },
    implicit_clone::unsync::IString::Static("CheckCircle") => html! {
      <MD_CheckCircle ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FiletypeScss") => html! {
      <MD_FiletypeScss ..owned_props />
    },
    implicit_clone::unsync::IString::Static("HouseExclamation") => html! {
      <MD_HouseExclamation ..owned_props />
    },
    implicit_clone::unsync::IString::Static("EmojiKissFill") => html! {
      <MD_EmojiKissFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Stars") => html! {
      <MD_Stars ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ShieldFillMinus") => html! {
      <MD_ShieldFillMinus ..owned_props />
    },
    implicit_clone::unsync::IString::Static("CameraReelsFill") => html! {
      <MD_CameraReelsFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("MenuApp") => html! {
      <MD_MenuApp ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ArrowUpSquareFill") => html! {
      <MD_ArrowUpSquareFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Clipboard2PlusFill") => html! {
      <MD_Clipboard2PlusFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ChatRightQuote") => html! {
      <MD_ChatRightQuote ..owned_props />
    },
    implicit_clone::unsync::IString::Static("BrightnessHigh") => html! {
      <MD_BrightnessHigh ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Signpost") => html! {
      <MD_Signpost ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FiletypeXml") => html! {
      <MD_FiletypeXml ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ExclamationLg") => html! {
      <MD_ExclamationLg ..owned_props />
    },
    implicit_clone::unsync::IString::Static("PersonFillExclamation") => html! {
      <MD_PersonFillExclamation ..owned_props />
    },
    implicit_clone::unsync::IString::Static("VolumeOffFill") => html! {
      <MD_VolumeOffFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Send") => html! {
      <MD_Send ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FileBinary") => html! {
      <MD_FileBinary ..owned_props />
    },
    implicit_clone::unsync::IString::Static("HouseDownFill") => html! {
      <MD_HouseDownFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("SkipBackwardCircleFill") => html! {
      <MD_SkipBackwardCircleFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("SkipForwardBtnFill") => html! {
      <MD_SkipForwardBtnFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("HouseAddFill") => html! {
      <MD_HouseAddFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("CloudFill") => html! {
      <MD_CloudFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("8CircleFill") => html! {
      <MD_8CircleFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Clipboard2Pulse") => html! {
      <MD_Clipboard2Pulse ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Calendar3EventFill") => html! {
      <MD_Calendar3EventFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FileText") => html! {
      <MD_FileText ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Tools") => html! {
      <MD_Tools ..owned_props />
    },
    implicit_clone::unsync::IString::Static("CloudHaze") => html! {
      <MD_CloudHaze ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FiletypeXlsx") => html! {
      <MD_FiletypeXlsx ..owned_props />
    },
    implicit_clone::unsync::IString::Static("CodeSlash") => html! {
      <MD_CodeSlash ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Grid1x2Fill") => html! {
      <MD_Grid1x2Fill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Upload") => html! {
      <MD_Upload ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FileArrowDownFill") => html! {
      <MD_FileArrowDownFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("PSquare") => html! {
      <MD_PSquare ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ZoomOut") => html! {
      <MD_ZoomOut ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ArrowBarRight") => html! {
      <MD_ArrowBarRight ..owned_props />
    },
    implicit_clone::unsync::IString::Static("7Square") => html! {
      <MD_7Square ..owned_props />
    },
    implicit_clone::unsync::IString::Static("QuestionOctagonFill") => html! {
      <MD_QuestionOctagonFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("BuildingDown") => html! {
      <MD_BuildingDown ..owned_props />
    },
    implicit_clone::unsync::IString::Static("BlockquoteLeft") => html! {
      <MD_BlockquoteLeft ..owned_props />
    },
    implicit_clone::unsync::IString::Static("WindowSidebar") => html! {
      <MD_WindowSidebar ..owned_props />
    },
    implicit_clone::unsync::IString::Static("SignDoNotEnter") => html! {
      <MD_SignDoNotEnter ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FileEarmarkPerson") => html! {
      <MD_FileEarmarkPerson ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Displayport") => html! {
      <MD_Displayport ..owned_props />
    },
    implicit_clone::unsync::IString::Static("EnvelopePaperHeartFill") => html! {
      <MD_EnvelopePaperHeartFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Trash2Fill") => html! {
      <MD_Trash2Fill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("HddRackFill") => html! {
      <MD_HddRackFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ExclamationTriangleFill") => html! {
      <MD_ExclamationTriangleFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("JustifyRight") => html! {
      <MD_JustifyRight ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Airplane") => html! {
      <MD_Airplane ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Dice4") => html! {
      <MD_Dice4 ..owned_props />
    },
    implicit_clone::unsync::IString::Static("UsbSymbol") => html! {
      <MD_UsbSymbol ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FileBarGraph") => html! {
      <MD_FileBarGraph ..owned_props />
    },
    implicit_clone::unsync::IString::Static("TriangleFill") => html! {
      <MD_TriangleFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FileMusicFill") => html! {
      <MD_FileMusicFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Instagram") => html! {
      <MD_Instagram ..owned_props />
    },
    implicit_clone::unsync::IString::Static("DeviceHdd") => html! {
      <MD_DeviceHdd ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FunnelFill") => html! {
      <MD_FunnelFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FiletypeSass") => html! {
      <MD_FiletypeSass ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Escape") => html! {
      <MD_Escape ..owned_props />
    },
    implicit_clone::unsync::IString::Static("CurrencyDollar") => html! {
      <MD_CurrencyDollar ..owned_props />
    },
    implicit_clone::unsync::IString::Static("RSquareFill") => html! {
      <MD_RSquareFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("CloudSlashFill") => html! {
      <MD_CloudSlashFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("HandThumbsUp") => html! {
      <MD_HandThumbsUp ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Bricks") => html! {
      <MD_Bricks ..owned_props />
    },
    implicit_clone::unsync::IString::Static("CardHeading") => html! {
      <MD_CardHeading ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ClipboardCheckFill") => html! {
      <MD_ClipboardCheckFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Hdmi") => html! {
      <MD_Hdmi ..owned_props />
    },
    implicit_clone::unsync::IString::Static("PatchPlus") => html! {
      <MD_PatchPlus ..owned_props />
    },
    implicit_clone::unsync::IString::Static("DashCircleDotted") => html! {
      <MD_DashCircleDotted ..owned_props />
    },
    implicit_clone::unsync::IString::Static("WrenchAdjustable") => html! {
      <MD_WrenchAdjustable ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Mouse2Fill") => html! {
      <MD_Mouse2Fill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Wordpress") => html! {
      <MD_Wordpress ..owned_props />
    },
    implicit_clone::unsync::IString::Static("PaintBucket") => html! {
      <MD_PaintBucket ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ChatRight") => html! {
      <MD_ChatRight ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ShiftFill") => html! {
      <MD_ShiftFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("TelephoneForward") => html! {
      <MD_TelephoneForward ..owned_props />
    },
    implicit_clone::unsync::IString::Static("CassetteFill") => html! {
      <MD_CassetteFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Slash") => html! {
      <MD_Slash ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FiletypeBmp") => html! {
      <MD_FiletypeBmp ..owned_props />
    },
    implicit_clone::unsync::IString::Static("BuildingGear") => html! {
      <MD_BuildingGear ..owned_props />
    },
    implicit_clone::unsync::IString::Static("CloudsFill") => html! {
      <MD_CloudsFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("SignRailroadFill") => html! {
      <MD_SignRailroadFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Grid3x3GapFill") => html! {
      <MD_Grid3x3GapFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("KeyboardFill") => html! {
      <MD_KeyboardFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ArrowDownLeftCircleFill") => html! {
      <MD_ArrowDownLeftCircleFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("TerminalPlus") => html! {
      <MD_TerminalPlus ..owned_props />
    },
    implicit_clone::unsync::IString::Static("UiRadios") => html! {
      <MD_UiRadios ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ArrowUp") => html! {
      <MD_ArrowUp ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ArrowUpSquare") => html! {
      <MD_ArrowUpSquare ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Android") => html! {
      <MD_Android ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Water") => html! {
      <MD_Water ..owned_props />
    },
    implicit_clone::unsync::IString::Static("InputCursorText") => html! {
      <MD_InputCursorText ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FileEarmarkLockFill") => html! {
      <MD_FileEarmarkLockFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("HospitalFill") => html! {
      <MD_HospitalFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("SortUpAlt") => html! {
      <MD_SortUpAlt ..owned_props />
    },
    implicit_clone::unsync::IString::Static("SimFill") => html! {
      <MD_SimFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("CloudSun") => html! {
      <MD_CloudSun ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Fingerprint") => html! {
      <MD_Fingerprint ..owned_props />
    },
    implicit_clone::unsync::IString::Static("WifiOff") => html! {
      <MD_WifiOff ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ArrowDown") => html! {
      <MD_ArrowDown ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FileRuled") => html! {
      <MD_FileRuled ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Save2") => html! {
      <MD_Save2 ..owned_props />
    },
    implicit_clone::unsync::IString::Static("VolumeOff") => html! {
      <MD_VolumeOff ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ClipboardPulse") => html! {
      <MD_ClipboardPulse ..owned_props />
    },
    implicit_clone::unsync::IString::Static("QuestionLg") => html! {
      <MD_QuestionLg ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FileEarmarkMusic") => html! {
      <MD_FileEarmarkMusic ..owned_props />
    },
    implicit_clone::unsync::IString::Static("1Square") => html! {
      <MD_1Square ..owned_props />
    },
    implicit_clone::unsync::IString::Static("AlignBottom") => html! {
      <MD_AlignBottom ..owned_props />
    },
    implicit_clone::unsync::IString::Static("LungsFill") => html! {
      <MD_LungsFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Cassette") => html! {
      <MD_Cassette ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ArrowUpRightCircleFill") => html! {
      <MD_ArrowUpRightCircleFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FiletypeJsx") => html! {
      <MD_FiletypeJsx ..owned_props />
    },
    implicit_clone::unsync::IString::Static("TelephonePlusFill") => html! {
      <MD_TelephonePlusFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("MoonStars") => html! {
      <MD_MoonStars ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Diamond") => html! {
      <MD_Diamond ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Bank2") => html! {
      <MD_Bank2 ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ArrowDownRight") => html! {
      <MD_ArrowDownRight ..owned_props />
    },
    implicit_clone::unsync::IString::Static("PostcardHeartFill") => html! {
      <MD_PostcardHeartFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("EggFill") => html! {
      <MD_EggFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("RSquare") => html! {
      <MD_RSquare ..owned_props />
    },
    implicit_clone::unsync::IString::Static("RssFill") => html! {
      <MD_RssFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ShieldLockFill") => html! {
      <MD_ShieldLockFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("BadgeVo") => html! {
      <MD_BadgeVo ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Mastodon") => html! {
      <MD_Mastodon ..owned_props />
    },
    implicit_clone::unsync::IString::Static("TextareaResize") => html! {
      <MD_TextareaResize ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Battery") => html! {
      <MD_Battery ..owned_props />
    },
    implicit_clone::unsync::IString::Static("UpcScan") => html! {
      <MD_UpcScan ..owned_props />
    },
    implicit_clone::unsync::IString::Static("CreditCard2Back") => html! {
      <MD_CreditCard2Back ..owned_props />
    },
    implicit_clone::unsync::IString::Static("EmojiKiss") => html! {
      <MD_EmojiKiss ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Cloudy") => html! {
      <MD_Cloudy ..owned_props />
    },
    implicit_clone::unsync::IString::Static("VolumeDown") => html! {
      <MD_VolumeDown ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Easel3") => html! {
      <MD_Easel3 ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FileEarmarkPdfFill") => html! {
      <MD_FileEarmarkPdfFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("EnvelopePaperHeart") => html! {
      <MD_EnvelopePaperHeart ..owned_props />
    },
    implicit_clone::unsync::IString::Static("HandIndex") => html! {
      <MD_HandIndex ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FiletypeMdx") => html! {
      <MD_FiletypeMdx ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FuelPump") => html! {
      <MD_FuelPump ..owned_props />
    },
    implicit_clone::unsync::IString::Static("LampFill") => html! {
      <MD_LampFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("LightbulbFill") => html! {
      <MD_LightbulbFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("SendXFill") => html! {
      <MD_SendXFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("AirplaneFill") => html! {
      <MD_AirplaneFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Bell") => html! {
      <MD_Bell ..owned_props />
    },
    implicit_clone::unsync::IString::Static("BoxArrowUpLeft") => html! {
      <MD_BoxArrowUpLeft ..owned_props />
    },
    implicit_clone::unsync::IString::Static("SignMergeLeftFill") => html! {
      <MD_SignMergeLeftFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ArrowDownRightSquareFill") => html! {
      <MD_ArrowDownRightSquareFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ShieldFill") => html! {
      <MD_ShieldFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("BadgeVoFill") => html! {
      <MD_BadgeVoFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ShieldShaded") => html! {
      <MD_ShieldShaded ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Plug") => html! {
      <MD_Plug ..owned_props />
    },
    implicit_clone::unsync::IString::Static("SuitSpade") => html! {
      <MD_SuitSpade ..owned_props />
    },
    implicit_clone::unsync::IString::Static("TypeStrikethrough") => html! {
      <MD_TypeStrikethrough ..owned_props />
    },
    implicit_clone::unsync::IString::Static("CameraReels") => html! {
      <MD_CameraReels ..owned_props />
    },
    implicit_clone::unsync::IString::Static("SaveFill") => html! {
      <MD_SaveFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("CalendarHeartFill") => html! {
      <MD_CalendarHeartFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("PencilFill") => html! {
      <MD_PencilFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("CloudSleet") => html! {
      <MD_CloudSleet ..owned_props />
    },
    implicit_clone::unsync::IString::Static("TaxiFront") => html! {
      <MD_TaxiFront ..owned_props />
    },
    implicit_clone::unsync::IString::Static("CashCoin") => html! {
      <MD_CashCoin ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FolderFill") => html! {
      <MD_FolderFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("CollectionPlay") => html! {
      <MD_CollectionPlay ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Dropbox") => html! {
      <MD_Dropbox ..owned_props />
    },
    implicit_clone::unsync::IString::Static("SignNoParking") => html! {
      <MD_SignNoParking ..owned_props />
    },
    implicit_clone::unsync::IString::Static("BadgeTmFill") => html! {
      <MD_BadgeTmFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Record2Fill") => html! {
      <MD_Record2Fill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("CircleFill") => html! {
      <MD_CircleFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("BagDashFill") => html! {
      <MD_BagDashFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ArrowLeftSquare") => html! {
      <MD_ArrowLeftSquare ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Calculator") => html! {
      <MD_Calculator ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FileEarmark") => html! {
      <MD_FileEarmark ..owned_props />
    },
    implicit_clone::unsync::IString::Static("CalendarPlusFill") => html! {
      <MD_CalendarPlusFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("EmojiHeartEyesFill") => html! {
      <MD_EmojiHeartEyesFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("1SquareFill") => html! {
      <MD_1SquareFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("PlayCircle") => html! {
      <MD_PlayCircle ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Capsule") => html! {
      <MD_Capsule ..owned_props />
    },
    implicit_clone::unsync::IString::Static("DatabaseAdd") => html! {
      <MD_DatabaseAdd ..owned_props />
    },
    implicit_clone::unsync::IString::Static("LayoutWtf") => html! {
      <MD_LayoutWtf ..owned_props />
    },
    implicit_clone::unsync::IString::Static("PostageHeartFill") => html! {
      <MD_PostageHeartFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("DashSquareFill") => html! {
      <MD_DashSquareFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("AlignStart") => html! {
      <MD_AlignStart ..owned_props />
    },
    implicit_clone::unsync::IString::Static("EmojiSmileUpsideDownFill") => html! {
      <MD_EmojiSmileUpsideDownFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ChevronBarExpand") => html! {
      <MD_ChevronBarExpand ..owned_props />
    },
    implicit_clone::unsync::IString::Static("GeoAltFill") => html! {
      <MD_GeoAltFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("HouseDashFill") => html! {
      <MD_HouseDashFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("SendDashFill") => html! {
      <MD_SendDashFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Exclude") => html! {
      <MD_Exclude ..owned_props />
    },
    implicit_clone::unsync::IString::Static("EmojiExpressionlessFill") => html! {
      <MD_EmojiExpressionlessFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Calendar2Plus") => html! {
      <MD_Calendar2Plus ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Newspaper") => html! {
      <MD_Newspaper ..owned_props />
    },
    implicit_clone::unsync::IString::Static("CalendarWeek") => html! {
      <MD_CalendarWeek ..owned_props />
    },
    implicit_clone::unsync::IString::Static("UsbPlug") => html! {
      <MD_UsbPlug ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ExclamationOctagon") => html! {
      <MD_ExclamationOctagon ..owned_props />
    },
    implicit_clone::unsync::IString::Static("CashStack") => html! {
      <MD_CashStack ..owned_props />
    },
    implicit_clone::unsync::IString::Static("BoxArrowInUpLeft") => html! {
      <MD_BoxArrowInUpLeft ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ReplyAllFill") => html! {
      <MD_ReplyAllFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("PauseBtnFill") => html! {
      <MD_PauseBtnFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FileEarmarkSlides") => html! {
      <MD_FileEarmarkSlides ..owned_props />
    },
    implicit_clone::unsync::IString::Static("InfoSquareFill") => html! {
      <MD_InfoSquareFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Calendar2MonthFill") => html! {
      <MD_Calendar2MonthFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("House") => html! {
      <MD_House ..owned_props />
    },
    implicit_clone::unsync::IString::Static("JournalArrowUp") => html! {
      <MD_JournalArrowUp ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Reception2") => html! {
      <MD_Reception2 ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FiletypeCs") => html! {
      <MD_FiletypeCs ..owned_props />
    },
    implicit_clone::unsync::IString::Static("BuildingFillX") => html! {
      <MD_BuildingFillX ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Octagon") => html! {
      <MD_Octagon ..owned_props />
    },
    implicit_clone::unsync::IString::Static("CloudArrowDown") => html! {
      <MD_CloudArrowDown ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Cloud") => html! {
      <MD_Cloud ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ShieldSlashFill") => html! {
      <MD_ShieldSlashFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("GiftFill") => html! {
      <MD_GiftFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("DiscFill") => html! {
      <MD_DiscFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ColumnsGap") => html! {
      <MD_ColumnsGap ..owned_props />
    },
    implicit_clone::unsync::IString::Static("SignMergeRight") => html! {
      <MD_SignMergeRight ..owned_props />
    },
    implicit_clone::unsync::IString::Static("EmojiFrown") => html! {
      <MD_EmojiFrown ..owned_props />
    },
    implicit_clone::unsync::IString::Static("HouseDoorFill") => html! {
      <MD_HouseDoorFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("BellSlash") => html! {
      <MD_BellSlash ..owned_props />
    },
    implicit_clone::unsync::IString::Static("DatabaseLock") => html! {
      <MD_DatabaseLock ..owned_props />
    },
    implicit_clone::unsync::IString::Static("OpticalAudioFill") => html! {
      <MD_OpticalAudioFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("CloudLightningRain") => html! {
      <MD_CloudLightningRain ..owned_props />
    },
    implicit_clone::unsync::IString::Static("LayoutSidebarReverse") => html! {
      <MD_LayoutSidebarReverse ..owned_props />
    },
    implicit_clone::unsync::IString::Static("WindowDock") => html! {
      <MD_WindowDock ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FileEarmarkMedical") => html! {
      <MD_FileEarmarkMedical ..owned_props />
    },
    implicit_clone::unsync::IString::Static("CSquareFill") => html! {
      <MD_CSquareFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ExclamationCircleFill") => html! {
      <MD_ExclamationCircleFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("LayoutThreeColumns") => html! {
      <MD_LayoutThreeColumns ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FileEarmarkExcel") => html! {
      <MD_FileEarmarkExcel ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Quora") => html! {
      <MD_Quora ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FiletypeRb") => html! {
      <MD_FiletypeRb ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Balloon") => html! {
      <MD_Balloon ..owned_props />
    },
    implicit_clone::unsync::IString::Static("BatteryFull") => html! {
      <MD_BatteryFull ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Paperclip") => html! {
      <MD_Paperclip ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Clipboard") => html! {
      <MD_Clipboard ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ArrowDownUp") => html! {
      <MD_ArrowDownUp ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ChatLeftText") => html! {
      <MD_ChatLeftText ..owned_props />
    },
    implicit_clone::unsync::IString::Static("CapslockFill") => html! {
      <MD_CapslockFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Cart3") => html! {
      <MD_Cart3 ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Modem") => html! {
      <MD_Modem ..owned_props />
    },
    implicit_clone::unsync::IString::Static("MusicPlayer") => html! {
      <MD_MusicPlayer ..owned_props />
    },
    implicit_clone::unsync::IString::Static("SignTurnSlightLeftFill") => html! {
      <MD_SignTurnSlightLeftFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("AirplaneEnginesFill") => html! {
      <MD_AirplaneEnginesFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("BuildingSlash") => html! {
      <MD_BuildingSlash ..owned_props />
    },
    implicit_clone::unsync::IString::Static("EmojiLaughingFill") => html! {
      <MD_EmojiLaughingFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Projector") => html! {
      <MD_Projector ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FlagFill") => html! {
      <MD_FlagFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("BodyText") => html! {
      <MD_BodyText ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Magic") => html! {
      <MD_Magic ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Box") => html! {
      <MD_Box ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Indent") => html! {
      <MD_Indent ..owned_props />
    },
    implicit_clone::unsync::IString::Static("PeopleFill") => html! {
      <MD_PeopleFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FileEarmarkSlidesFill") => html! {
      <MD_FileEarmarkSlidesFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FileBarGraphFill") => html! {
      <MD_FileBarGraphFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Funnel") => html! {
      <MD_Funnel ..owned_props />
    },
    implicit_clone::unsync::IString::Static("HouseDash") => html! {
      <MD_HouseDash ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Stoplights") => html! {
      <MD_Stoplights ..owned_props />
    },
    implicit_clone::unsync::IString::Static("StopCircleFill") => html! {
      <MD_StopCircleFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("KeyFill") => html! {
      <MD_KeyFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("TrainLightrailFrontFill") => html! {
      <MD_TrainLightrailFrontFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Steam") => html! {
      <MD_Steam ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Bezier2") => html! {
      <MD_Bezier2 ..owned_props />
    },
    implicit_clone::unsync::IString::Static("CameraVideoFill") => html! {
      <MD_CameraVideoFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("GearFill") => html! {
      <MD_GearFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("CupHotFill") => html! {
      <MD_CupHotFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ListColumns") => html! {
      <MD_ListColumns ..owned_props />
    },
    implicit_clone::unsync::IString::Static("XDiamondFill") => html! {
      <MD_XDiamondFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("SortAlphaDown") => html! {
      <MD_SortAlphaDown ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FileLockFill") => html! {
      <MD_FileLockFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Justify") => html! {
      <MD_Justify ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Pip") => html! {
      <MD_Pip ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Layers") => html! {
      <MD_Layers ..owned_props />
    },
    implicit_clone::unsync::IString::Static("BuildingFillAdd") => html! {
      <MD_BuildingFillAdd ..owned_props />
    },
    implicit_clone::unsync::IString::Static("BoxArrowDownRight") => html! {
      <MD_BoxArrowDownRight ..owned_props />
    },
    implicit_clone::unsync::IString::Static("BorderCenter") => html! {
      <MD_BorderCenter ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Diagram3") => html! {
      <MD_Diagram3 ..owned_props />
    },
    implicit_clone::unsync::IString::Static("DeviceHddFill") => html! {
      <MD_DeviceHddFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("BrightnessAltLow") => html! {
      <MD_BrightnessAltLow ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Chat") => html! {
      <MD_Chat ..owned_props />
    },
    implicit_clone::unsync::IString::Static("TelephoneOutboundFill") => html! {
      <MD_TelephoneOutboundFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("CaretRightSquareFill") => html! {
      <MD_CaretRightSquareFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("BroadcastPin") => html! {
      <MD_BroadcastPin ..owned_props />
    },
    implicit_clone::unsync::IString::Static("XCircleFill") => html! {
      <MD_XCircleFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("SkipEndBtnFill") => html! {
      <MD_SkipEndBtnFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Lungs") => html! {
      <MD_Lungs ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Calendar2PlusFill") => html! {
      <MD_Calendar2PlusFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("EmojiSunglasses") => html! {
      <MD_EmojiSunglasses ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Clipboard2CheckFill") => html! {
      <MD_Clipboard2CheckFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Voicemail") => html! {
      <MD_Voicemail ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ArrowUpLeftSquareFill") => html! {
      <MD_ArrowUpLeftSquareFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("PatchExclamation") => html! {
      <MD_PatchExclamation ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Easel2Fill") => html! {
      <MD_Easel2Fill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FiletypeKey") => html! {
      <MD_FiletypeKey ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FiletypeSvg") => html! {
      <MD_FiletypeSvg ..owned_props />
    },
    implicit_clone::unsync::IString::Static("SearchHeartFill") => html! {
      <MD_SearchHeartFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FileEarmarkArrowUpFill") => html! {
      <MD_FileEarmarkArrowUpFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("CircleHalf") => html! {
      <MD_CircleHalf ..owned_props />
    },
    implicit_clone::unsync::IString::Static("AlignTop") => html! {
      <MD_AlignTop ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ArrowsCollapse") => html! {
      <MD_ArrowsCollapse ..owned_props />
    },
    implicit_clone::unsync::IString::Static("BuildingFillSlash") => html! {
      <MD_BuildingFillSlash ..owned_props />
    },
    implicit_clone::unsync::IString::Static("SlashLg") => html! {
      <MD_SlashLg ..owned_props />
    },
    implicit_clone::unsync::IString::Static("EnvelopeCheckFill") => html! {
      <MD_EnvelopeCheckFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FileEarmarkPlayFill") => html! {
      <MD_FileEarmarkPlayFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("BadgeAd") => html! {
      <MD_BadgeAd ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FileEarmarkPersonFill") => html! {
      <MD_FileEarmarkPersonFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("HouseGearFill") => html! {
      <MD_HouseGearFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Git") => html! {
      <MD_Git ..owned_props />
    },
    implicit_clone::unsync::IString::Static("BellFill") => html! {
      <MD_BellFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Activity") => html! {
      <MD_Activity ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ChatLeftDots") => html! {
      <MD_ChatLeftDots ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ShieldLock") => html! {
      <MD_ShieldLock ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Capslock") => html! {
      <MD_Capslock ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ArrowUpRightSquare") => html! {
      <MD_ArrowUpRightSquare ..owned_props />
    },
    implicit_clone::unsync::IString::Static("NodePlus") => html! {
      <MD_NodePlus ..owned_props />
    },
    implicit_clone::unsync::IString::Static("2Square") => html! {
      <MD_2Square ..owned_props />
    },
    implicit_clone::unsync::IString::Static("CaretRight") => html! {
      <MD_CaretRight ..owned_props />
    },
    implicit_clone::unsync::IString::Static("BuildingLock") => html! {
      <MD_BuildingLock ..owned_props />
    },
    implicit_clone::unsync::IString::Static("UsbFill") => html! {
      <MD_UsbFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("PostageHeart") => html! {
      <MD_PostageHeart ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Coin") => html! {
      <MD_Coin ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ChevronDoubleUp") => html! {
      <MD_ChevronDoubleUp ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Grid3x3Gap") => html! {
      <MD_Grid3x3Gap ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Clouds") => html! {
      <MD_Clouds ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Folder2Open") => html! {
      <MD_Folder2Open ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FilterSquare") => html! {
      <MD_FilterSquare ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Hexagon") => html! {
      <MD_Hexagon ..owned_props />
    },
    implicit_clone::unsync::IString::Static("HeptagonHalf") => html! {
      <MD_HeptagonHalf ..owned_props />
    },
    implicit_clone::unsync::IString::Static("DashCircleFill") => html! {
      <MD_DashCircleFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FileEasel") => html! {
      <MD_FileEasel ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Grid1x2") => html! {
      <MD_Grid1x2 ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Hdd") => html! {
      <MD_Hdd ..owned_props />
    },
    implicit_clone::unsync::IString::Static("QuestionOctagon") => html! {
      <MD_QuestionOctagon ..owned_props />
    },
    implicit_clone::unsync::IString::Static("SendDash") => html! {
      <MD_SendDash ..owned_props />
    },
    implicit_clone::unsync::IString::Static("BagPlusFill") => html! {
      <MD_BagPlusFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FileEarmarkBarGraph") => html! {
      <MD_FileEarmarkBarGraph ..owned_props />
    },
    implicit_clone::unsync::IString::Static("UiChecks") => html! {
      <MD_UiChecks ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FileSpreadsheet") => html! {
      <MD_FileSpreadsheet ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Bug") => html! {
      <MD_Bug ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ArrowRepeat") => html! {
      <MD_ArrowRepeat ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Dice5") => html! {
      <MD_Dice5 ..owned_props />
    },
    implicit_clone::unsync::IString::Static("3Circle") => html! {
      <MD_3Circle ..owned_props />
    },
    implicit_clone::unsync::IString::Static("PauseBtn") => html! {
      <MD_PauseBtn ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Images") => html! {
      <MD_Images ..owned_props />
    },
    implicit_clone::unsync::IString::Static("SignNoLeftTurnFill") => html! {
      <MD_SignNoLeftTurnFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("CalendarX") => html! {
      <MD_CalendarX ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FiletypePng") => html! {
      <MD_FiletypePng ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Clock") => html! {
      <MD_Clock ..owned_props />
    },
    implicit_clone::unsync::IString::Static("AirplaneEngines") => html! {
      <MD_AirplaneEngines ..owned_props />
    },
    implicit_clone::unsync::IString::Static("EmojiHeartEyes") => html! {
      <MD_EmojiHeartEyes ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Book") => html! {
      <MD_Book ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Sliders2Vertical") => html! {
      <MD_Sliders2Vertical ..owned_props />
    },
    implicit_clone::unsync::IString::Static("CreditCard2Front") => html! {
      <MD_CreditCard2Front ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FileEarmarkWord") => html! {
      <MD_FileEarmarkWord ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FiletypeCsv") => html! {
      <MD_FiletypeCsv ..owned_props />
    },
    implicit_clone::unsync::IString::Static("BarChartLineFill") => html! {
      <MD_BarChartLineFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Hurricane") => html! {
      <MD_Hurricane ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Envelope") => html! {
      <MD_Envelope ..owned_props />
    },
    implicit_clone::unsync::IString::Static("OctagonFill") => html! {
      <MD_OctagonFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("EnvelopeSlash") => html! {
      <MD_EnvelopeSlash ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FileEarmarkSpreadsheetFill") => html! {
      <MD_FileEarmarkSpreadsheetFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("BoxArrowDown") => html! {
      <MD_BoxArrowDown ..owned_props />
    },
    implicit_clone::unsync::IString::Static("BuildingFillUp") => html! {
      <MD_BuildingFillUp ..owned_props />
    },
    implicit_clone::unsync::IString::Static("CloudSlash") => html! {
      <MD_CloudSlash ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ClipboardXFill") => html! {
      <MD_ClipboardXFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ArrowLeftRight") => html! {
      <MD_ArrowLeftRight ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Square") => html! {
      <MD_Square ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FilterLeft") => html! {
      <MD_FilterLeft ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Ladder") => html! {
      <MD_Ladder ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FileCodeFill") => html! {
      <MD_FileCodeFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Badge4kFill") => html! {
      <MD_Badge4kFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Umbrella") => html! {
      <MD_Umbrella ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ArrowDownLeftCircle") => html! {
      <MD_ArrowDownLeftCircle ..owned_props />
    },
    implicit_clone::unsync::IString::Static("BracesAsterisk") => html! {
      <MD_BracesAsterisk ..owned_props />
    },
    implicit_clone::unsync::IString::Static("BrowserEdge") => html! {
      <MD_BrowserEdge ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Pen") => html! {
      <MD_Pen ..owned_props />
    },
    implicit_clone::unsync::IString::Static("BootstrapReboot") => html! {
      <MD_BootstrapReboot ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Smartwatch") => html! {
      <MD_Smartwatch ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Peace") => html! {
      <MD_Peace ..owned_props />
    },
    implicit_clone::unsync::IString::Static("PersonBadge") => html! {
      <MD_PersonBadge ..owned_props />
    },
    implicit_clone::unsync::IString::Static("XOctagonFill") => html! {
      <MD_XOctagonFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("CreditCard2BackFill") => html! {
      <MD_CreditCard2BackFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("BorderWidth") => html! {
      <MD_BorderWidth ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ToggleOff") => html! {
      <MD_ToggleOff ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Basket2Fill") => html! {
      <MD_Basket2Fill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("PersonXFill") => html! {
      <MD_PersonXFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("CloudDrizzleFill") => html! {
      <MD_CloudDrizzleFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Wallet2") => html! {
      <MD_Wallet2 ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Forward") => html! {
      <MD_Forward ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Android2") => html! {
      <MD_Android2 ..owned_props />
    },
    implicit_clone::unsync::IString::Static("HCircle") => html! {
      <MD_HCircle ..owned_props />
    },
    implicit_clone::unsync::IString::Static("CupFill") => html! {
      <MD_CupFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("CalendarRange") => html! {
      <MD_CalendarRange ..owned_props />
    },
    implicit_clone::unsync::IString::Static("SignpostSplitFill") => html! {
      <MD_SignpostSplitFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("EmojiNeutral") => html! {
      <MD_EmojiNeutral ..owned_props />
    },
    implicit_clone::unsync::IString::Static("DashSquareDotted") => html! {
      <MD_DashSquareDotted ..owned_props />
    },
    implicit_clone::unsync::IString::Static("PersonCircle") => html! {
      <MD_PersonCircle ..owned_props />
    },
    implicit_clone::unsync::IString::Static("SkipEnd") => html! {
      <MD_SkipEnd ..owned_props />
    },
    implicit_clone::unsync::IString::Static("EjectFill") => html! {
      <MD_EjectFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FolderCheck") => html! {
      <MD_FolderCheck ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FileBreakFill") => html! {
      <MD_FileBreakFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("SignTurnSlightRight") => html! {
      <MD_SignTurnSlightRight ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Vinyl") => html! {
      <MD_Vinyl ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Calendar2X") => html! {
      <MD_Calendar2X ..owned_props />
    },
    implicit_clone::unsync::IString::Static("CollectionFill") => html! {
      <MD_CollectionFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Moisture") => html! {
      <MD_Moisture ..owned_props />
    },
    implicit_clone::unsync::IString::Static("SlashSquare") => html! {
      <MD_SlashSquare ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Image") => html! {
      <MD_Image ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Clipboard2Heart") => html! {
      <MD_Clipboard2Heart ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Rainbow") => html! {
      <MD_Rainbow ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Valentine") => html! {
      <MD_Valentine ..owned_props />
    },
    implicit_clone::unsync::IString::Static("HouseLock") => html! {
      <MD_HouseLock ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ExclamationDiamond") => html! {
      <MD_ExclamationDiamond ..owned_props />
    },
    implicit_clone::unsync::IString::Static("MenuButton") => html! {
      <MD_MenuButton ..owned_props />
    },
    implicit_clone::unsync::IString::Static("2CircleFill") => html! {
      <MD_2CircleFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("EnvelopeXFill") => html! {
      <MD_EnvelopeXFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Backspace") => html! {
      <MD_Backspace ..owned_props />
    },
    implicit_clone::unsync::IString::Static("BadgeArFill") => html! {
      <MD_BadgeArFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ProjectorFill") => html! {
      <MD_ProjectorFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ArrowUpLeftSquare") => html! {
      <MD_ArrowUpLeftSquare ..owned_props />
    },
    implicit_clone::unsync::IString::Static("SkipBackwardBtn") => html! {
      <MD_SkipBackwardBtn ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FilePlusFill") => html! {
      <MD_FilePlusFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("AlignMiddle") => html! {
      <MD_AlignMiddle ..owned_props />
    },
    implicit_clone::unsync::IString::Static("PinAngle") => html! {
      <MD_PinAngle ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Calendar2Fill") => html! {
      <MD_Calendar2Fill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Sunrise") => html! {
      <MD_Sunrise ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Badge3d") => html! {
      <MD_Badge3d ..owned_props />
    },
    implicit_clone::unsync::IString::Static("BagFill") => html! {
      <MD_BagFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Safe") => html! {
      <MD_Safe ..owned_props />
    },
    implicit_clone::unsync::IString::Static("7Circle") => html! {
      <MD_7Circle ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Clipboard2MinusFill") => html! {
      <MD_Clipboard2MinusFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FileEarmarkCode") => html! {
      <MD_FileEarmarkCode ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FilePpt") => html! {
      <MD_FilePpt ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FiletypeJson") => html! {
      <MD_FiletypeJson ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Calendar2Week") => html! {
      <MD_Calendar2Week ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Unity") => html! {
      <MD_Unity ..owned_props />
    },
    implicit_clone::unsync::IString::Static("WindowFullscreen") => html! {
      <MD_WindowFullscreen ..owned_props />
    },
    implicit_clone::unsync::IString::Static("TrainFreightFront") => html! {
      <MD_TrainFreightFront ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FileEarmarkRichtextFill") => html! {
      <MD_FileEarmarkRichtextFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("BadgeTm") => html! {
      <MD_BadgeTm ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Back") => html! {
      <MD_Back ..owned_props />
    },
    implicit_clone::unsync::IString::Static("SkipStart") => html! {
      <MD_SkipStart ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ChatQuoteFill") => html! {
      <MD_ChatQuoteFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FastForwardCircleFill") => html! {
      <MD_FastForwardCircleFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("BucketFill") => html! {
      <MD_BucketFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("SdCardFill") => html! {
      <MD_SdCardFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Playstation") => html! {
      <MD_Playstation ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FileEarmarkMedicalFill") => html! {
      <MD_FileEarmarkMedicalFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("LockFill") => html! {
      <MD_LockFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Calendar2Heart") => html! {
      <MD_Calendar2Heart ..owned_props />
    },
    implicit_clone::unsync::IString::Static("XSquareFill") => html! {
      <MD_XSquareFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Pin") => html! {
      <MD_Pin ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Folder") => html! {
      <MD_Folder ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ArrowUpShort") => html! {
      <MD_ArrowUpShort ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Calendar2Minus") => html! {
      <MD_Calendar2Minus ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FileArrowUp") => html! {
      <MD_FileArrowUp ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Line") => html! {
      <MD_Line ..owned_props />
    },
    implicit_clone::unsync::IString::Static("HdmiFill") => html! {
      <MD_HdmiFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Tiktok") => html! {
      <MD_Tiktok ..owned_props />
    },
    implicit_clone::unsync::IString::Static("SquareFill") => html! {
      <MD_SquareFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Heptagon") => html! {
      <MD_Heptagon ..owned_props />
    },
    implicit_clone::unsync::IString::Static("QuestionSquareFill") => html! {
      <MD_QuestionSquareFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("DpadFill") => html! {
      <MD_DpadFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("BoxFill") => html! {
      <MD_BoxFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("UsbMiniFill") => html! {
      <MD_UsbMiniFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("CartDashFill") => html! {
      <MD_CartDashFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("PersonFillX") => html! {
      <MD_PersonFillX ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Trash") => html! {
      <MD_Trash ..owned_props />
    },
    implicit_clone::unsync::IString::Static("QuestionDiamond") => html! {
      <MD_QuestionDiamond ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Egg") => html! {
      <MD_Egg ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Headset") => html! {
      <MD_Headset ..owned_props />
    },
    implicit_clone::unsync::IString::Static("CurrencyEuro") => html! {
      <MD_CurrencyEuro ..owned_props />
    },
    implicit_clone::unsync::IString::Static("JournalCheck") => html! {
      <MD_JournalCheck ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Calendar2Month") => html! {
      <MD_Calendar2Month ..owned_props />
    },
    implicit_clone::unsync::IString::Static("BoxArrowUpRight") => html! {
      <MD_BoxArrowUpRight ..owned_props />
    },
    implicit_clone::unsync::IString::Static("SignTurnLeftFill") => html! {
      <MD_SignTurnLeftFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FileEarmarkFont") => html! {
      <MD_FileEarmarkFont ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ShieldSlash") => html! {
      <MD_ShieldSlash ..owned_props />
    },
    implicit_clone::unsync::IString::Static("SkipForwardCircle") => html! {
      <MD_SkipForwardCircle ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Watch") => html! {
      <MD_Watch ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ArrowUpLeft") => html! {
      <MD_ArrowUpLeft ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Calendar2") => html! {
      <MD_Calendar2 ..owned_props />
    },
    implicit_clone::unsync::IString::Static("EnvelopeFill") => html! {
      <MD_EnvelopeFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Skype") => html! {
      <MD_Skype ..owned_props />
    },
    implicit_clone::unsync::IString::Static("RCircle") => html! {
      <MD_RCircle ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Calendar2EventFill") => html! {
      <MD_Calendar2EventFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("PostageFill") => html! {
      <MD_PostageFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("UsbCFill") => html! {
      <MD_UsbCFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("SkipBackwardBtnFill") => html! {
      <MD_SkipBackwardBtnFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("RecordCircle") => html! {
      <MD_RecordCircle ..owned_props />
    },
    implicit_clone::unsync::IString::Static("BuildingFillDown") => html! {
      <MD_BuildingFillDown ..owned_props />
    },
    implicit_clone::unsync::IString::Static("GenderMale") => html! {
      <MD_GenderMale ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Hr") => html! {
      <MD_Hr ..owned_props />
    },
    implicit_clone::unsync::IString::Static("CloudArrowUpFill") => html! {
      <MD_CloudArrowUpFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Meta") => html! {
      <MD_Meta ..owned_props />
    },
    implicit_clone::unsync::IString::Static("PieChartFill") => html! {
      <MD_PieChartFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("RCircleFill") => html! {
      <MD_RCircleFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("TaxiFrontFill") => html! {
      <MD_TaxiFrontFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("StarFill") => html! {
      <MD_StarFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Calendar4") => html! {
      <MD_Calendar4 ..owned_props />
    },
    implicit_clone::unsync::IString::Static("SignIntersection") => html! {
      <MD_SignIntersection ..owned_props />
    },
    implicit_clone::unsync::IString::Static("CaretRightSquare") => html! {
      <MD_CaretRightSquare ..owned_props />
    },
    implicit_clone::unsync::IString::Static("PatchQuestionFill") => html! {
      <MD_PatchQuestionFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("X") => html! {
      <MD_X ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Record") => html! {
      <MD_Record ..owned_props />
    },
    implicit_clone::unsync::IString::Static("CloudMoonFill") => html! {
      <MD_CloudMoonFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("HddNetworkFill") => html! {
      <MD_HddNetworkFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("TrainFrontFill") => html! {
      <MD_TrainFrontFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ListCheck") => html! {
      <MD_ListCheck ..owned_props />
    },
    implicit_clone::unsync::IString::Static("CloudRain") => html! {
      <MD_CloudRain ..owned_props />
    },
    implicit_clone::unsync::IString::Static("BuildingExclamation") => html! {
      <MD_BuildingExclamation ..owned_props />
    },
    implicit_clone::unsync::IString::Static("PlayFill") => html! {
      <MD_PlayFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("SignDeadEndFill") => html! {
      <MD_SignDeadEndFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ShieldMinus") => html! {
      <MD_ShieldMinus ..owned_props />
    },
    implicit_clone::unsync::IString::Static("HddRack") => html! {
      <MD_HddRack ..owned_props />
    },
    implicit_clone::unsync::IString::Static("CloudArrowDownFill") => html! {
      <MD_CloudArrowDownFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("DatabaseX") => html! {
      <MD_DatabaseX ..owned_props />
    },
    implicit_clone::unsync::IString::Static("CloudPlusFill") => html! {
      <MD_CloudPlusFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("SortNumericUp") => html! {
      <MD_SortNumericUp ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Archive") => html! {
      <MD_Archive ..owned_props />
    },
    implicit_clone::unsync::IString::Static("BorderAll") => html! {
      <MD_BorderAll ..owned_props />
    },
    implicit_clone::unsync::IString::Static("CalendarDateFill") => html! {
      <MD_CalendarDateFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("PhoneFlip") => html! {
      <MD_PhoneFlip ..owned_props />
    },
    implicit_clone::unsync::IString::Static("PersonUp") => html! {
      <MD_PersonUp ..owned_props />
    },
    implicit_clone::unsync::IString::Static("CartXFill") => html! {
      <MD_CartXFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("PaletteFill") => html! {
      <MD_PaletteFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Snow") => html! {
      <MD_Snow ..owned_props />
    },
    implicit_clone::unsync::IString::Static("SuitDiamond") => html! {
      <MD_SuitDiamond ..owned_props />
    },
    implicit_clone::unsync::IString::Static("CalendarEvent") => html! {
      <MD_CalendarEvent ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Grid") => html! {
      <MD_Grid ..owned_props />
    },
    implicit_clone::unsync::IString::Static("PcHorizontal") => html! {
      <MD_PcHorizontal ..owned_props />
    },
    implicit_clone::unsync::IString::Static("XDiamond") => html! {
      <MD_XDiamond ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Speedometer2") => html! {
      <MD_Speedometer2 ..owned_props />
    },
    implicit_clone::unsync::IString::Static("SignIntersectionFill") => html! {
      <MD_SignIntersectionFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Hash") => html! {
      <MD_Hash ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ExclamationSquare") => html! {
      <MD_ExclamationSquare ..owned_props />
    },
    implicit_clone::unsync::IString::Static("EmojiSmile") => html! {
      <MD_EmojiSmile ..owned_props />
    },
    implicit_clone::unsync::IString::Static("SkipStartCircleFill") => html! {
      <MD_SkipStartCircleFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("DoorClosedFill") => html! {
      <MD_DoorClosedFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ChevronCompactRight") => html! {
      <MD_ChevronCompactRight ..owned_props />
    },
    implicit_clone::unsync::IString::Static("CurrencyPound") => html! {
      <MD_CurrencyPound ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FileFill") => html! {
      <MD_FileFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Boombox") => html! {
      <MD_Boombox ..owned_props />
    },
    implicit_clone::unsync::IString::Static("TypeItalic") => html! {
      <MD_TypeItalic ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Toggles2") => html! {
      <MD_Toggles2 ..owned_props />
    },
    implicit_clone::unsync::IString::Static("BriefcaseFill") => html! {
      <MD_BriefcaseFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ModemFill") => html! {
      <MD_ModemFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Calendar") => html! {
      <MD_Calendar ..owned_props />
    },
    implicit_clone::unsync::IString::Static("LayersHalf") => html! {
      <MD_LayersHalf ..owned_props />
    },
    implicit_clone::unsync::IString::Static("WalletFill") => html! {
      <MD_WalletFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("SendPlusFill") => html! {
      <MD_SendPlusFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("BorderRight") => html! {
      <MD_BorderRight ..owned_props />
    },
    implicit_clone::unsync::IString::Static("PersonFillAdd") => html! {
      <MD_PersonFillAdd ..owned_props />
    },
    implicit_clone::unsync::IString::Static("GridFill") => html! {
      <MD_GridFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("PencilSquare") => html! {
      <MD_PencilSquare ..owned_props />
    },
    implicit_clone::unsync::IString::Static("BadgeHd") => html! {
      <MD_BadgeHd ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ChatLeft") => html! {
      <MD_ChatLeft ..owned_props />
    },
    implicit_clone::unsync::IString::Static("8Circle") => html! {
      <MD_8Circle ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FileCheckFill") => html! {
      <MD_FileCheckFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Tags") => html! {
      <MD_Tags ..owned_props />
    },
    implicit_clone::unsync::IString::Static("RecordFill") => html! {
      <MD_RecordFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Cart4") => html! {
      <MD_Cart4 ..owned_props />
    },
    implicit_clone::unsync::IString::Static("CheckLg") => html! {
      <MD_CheckLg ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ChatFill") => html! {
      <MD_ChatFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("CalendarMinusFill") => html! {
      <MD_CalendarMinusFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ChevronBarContract") => html! {
      <MD_ChevronBarContract ..owned_props />
    },
    implicit_clone::unsync::IString::Static("AlarmFill") => html! {
      <MD_AlarmFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("CloudDownload") => html! {
      <MD_CloudDownload ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Usb") => html! {
      <MD_Usb ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Diagram3Fill") => html! {
      <MD_Diagram3Fill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("CcCircle") => html! {
      <MD_CcCircle ..owned_props />
    },
    implicit_clone::unsync::IString::Static("TextareaT") => html! {
      <MD_TextareaT ..owned_props />
    },
    implicit_clone::unsync::IString::Static("CalculatorFill") => html! {
      <MD_CalculatorFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("BusFront") => html! {
      <MD_BusFront ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Sun") => html! {
      <MD_Sun ..owned_props />
    },
    implicit_clone::unsync::IString::Static("QuestionCircle") => html! {
      <MD_QuestionCircle ..owned_props />
    },
    implicit_clone::unsync::IString::Static("CloudPlus") => html! {
      <MD_CloudPlus ..owned_props />
    },
    implicit_clone::unsync::IString::Static("SearchHeart") => html! {
      <MD_SearchHeart ..owned_props />
    },
    implicit_clone::unsync::IString::Static("HandIndexThumbFill") => html! {
      <MD_HandIndexThumbFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Inboxes") => html! {
      <MD_Inboxes ..owned_props />
    },
    implicit_clone::unsync::IString::Static("SquareHalf") => html! {
      <MD_SquareHalf ..owned_props />
    },
    implicit_clone::unsync::IString::Static("MenuButtonWideFill") => html! {
      <MD_MenuButtonWideFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("DoorOpen") => html! {
      <MD_DoorOpen ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Ticket") => html! {
      <MD_Ticket ..owned_props />
    },
    implicit_clone::unsync::IString::Static("CurrencyYen") => html! {
      <MD_CurrencyYen ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Stack") => html! {
      <MD_Stack ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Download") => html! {
      <MD_Download ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Clipboard2X") => html! {
      <MD_Clipboard2X ..owned_props />
    },
    implicit_clone::unsync::IString::Static("MarkdownFill") => html! {
      <MD_MarkdownFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ForwardFill") => html! {
      <MD_ForwardFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Binoculars") => html! {
      <MD_Binoculars ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Play") => html! {
      <MD_Play ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Clipboard2Check") => html! {
      <MD_Clipboard2Check ..owned_props />
    },
    implicit_clone::unsync::IString::Static("StopBtn") => html! {
      <MD_StopBtn ..owned_props />
    },
    implicit_clone::unsync::IString::Static("PlusSquare") => html! {
      <MD_PlusSquare ..owned_props />
    },
    implicit_clone::unsync::IString::Static("GpuCard") => html! {
      <MD_GpuCard ..owned_props />
    },
    implicit_clone::unsync::IString::Static("LightningFill") => html! {
      <MD_LightningFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ReplyAll") => html! {
      <MD_ReplyAll ..owned_props />
    },
    implicit_clone::unsync::IString::Static("UiChecksGrid") => html! {
      <MD_UiChecksGrid ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ClipboardDataFill") => html! {
      <MD_ClipboardDataFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("RecordBtn") => html! {
      <MD_RecordBtn ..owned_props />
    },
    implicit_clone::unsync::IString::Static("8Square") => html! {
      <MD_8Square ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ShareFill") => html! {
      <MD_ShareFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("SortNumericUpAlt") => html! {
      <MD_SortNumericUpAlt ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ChatSquareQuoteFill") => html! {
      <MD_ChatSquareQuoteFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("BusFrontFill") => html! {
      <MD_BusFrontFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Mouse2") => html! {
      <MD_Mouse2 ..owned_props />
    },
    implicit_clone::unsync::IString::Static("TicketPerforated") => html! {
      <MD_TicketPerforated ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ChatLeftHeart") => html! {
      <MD_ChatLeftHeart ..owned_props />
    },
    implicit_clone::unsync::IString::Static("RewindCircleFill") => html! {
      <MD_RewindCircleFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Calendar3Range") => html! {
      <MD_Calendar3Range ..owned_props />
    },
    implicit_clone::unsync::IString::Static("AppIndicator") => html! {
      <MD_AppIndicator ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Alarm") => html! {
      <MD_Alarm ..owned_props />
    },
    implicit_clone::unsync::IString::Static("CalendarFill") => html! {
      <MD_CalendarFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("WindowPlus") => html! {
      <MD_WindowPlus ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ChatSquare") => html! {
      <MD_ChatSquare ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Github") => html! {
      <MD_Github ..owned_props />
    },
    implicit_clone::unsync::IString::Static("HandThumbsDown") => html! {
      <MD_HandThumbsDown ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Bookmarks") => html! {
      <MD_Bookmarks ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Flag") => html! {
      <MD_Flag ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ShieldFillExclamation") => html! {
      <MD_ShieldFillExclamation ..owned_props />
    },
    implicit_clone::unsync::IString::Static("PiggyBank") => html! {
      <MD_PiggyBank ..owned_props />
    },
    implicit_clone::unsync::IString::Static("RecordBtnFill") => html! {
      <MD_RecordBtnFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("DeviceSsd") => html! {
      <MD_DeviceSsd ..owned_props />
    },
    implicit_clone::unsync::IString::Static("PersonFillGear") => html! {
      <MD_PersonFillGear ..owned_props />
    },
    implicit_clone::unsync::IString::Static("SignStopLights") => html! {
      <MD_SignStopLights ..owned_props />
    },
    implicit_clone::unsync::IString::Static("0Square") => html! {
      <MD_0Square ..owned_props />
    },
    implicit_clone::unsync::IString::Static("EnvelopeDash") => html! {
      <MD_EnvelopeDash ..owned_props />
    },
    implicit_clone::unsync::IString::Static("List") => html! {
      <MD_List ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ListColumnsReverse") => html! {
      <MD_ListColumnsReverse ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Command") => html! {
      <MD_Command ..owned_props />
    },
    implicit_clone::unsync::IString::Static("EmojiFrownFill") => html! {
      <MD_EmojiFrownFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ChevronBarRight") => html! {
      <MD_ChevronBarRight ..owned_props />
    },
    implicit_clone::unsync::IString::Static("HouseSlash") => html! {
      <MD_HouseSlash ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ClipboardFill") => html! {
      <MD_ClipboardFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("InputCursor") => html! {
      <MD_InputCursor ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ArrowsFullscreen") => html! {
      <MD_ArrowsFullscreen ..owned_props />
    },
    implicit_clone::unsync::IString::Static("SkipEndCircle") => html! {
      <MD_SkipEndCircle ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Cast") => html! {
      <MD_Cast ..owned_props />
    },
    implicit_clone::unsync::IString::Static("TencentQq") => html! {
      <MD_TencentQq ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Calendar2Date") => html! {
      <MD_Calendar2Date ..owned_props />
    },
    implicit_clone::unsync::IString::Static("CcSquareFill") => html! {
      <MD_CcSquareFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("TelephoneInbound") => html! {
      <MD_TelephoneInbound ..owned_props />
    },
    implicit_clone::unsync::IString::Static("HexagonHalf") => html! {
      <MD_HexagonHalf ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Megaphone") => html! {
      <MD_Megaphone ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ChatSquareFill") => html! {
      <MD_ChatSquareFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FiletypeYml") => html! {
      <MD_FiletypeYml ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Dice5Fill") => html! {
      <MD_Dice5Fill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("PatchCheckFill") => html! {
      <MD_PatchCheckFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("BoxSeam") => html! {
      <MD_BoxSeam ..owned_props />
    },
    implicit_clone::unsync::IString::Static("AlignEnd") => html! {
      <MD_AlignEnd ..owned_props />
    },
    implicit_clone::unsync::IString::Static("BadgeSd") => html! {
      <MD_BadgeSd ..owned_props />
    },
    implicit_clone::unsync::IString::Static("InboxesFill") => html! {
      <MD_InboxesFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("CameraVideoOff") => html! {
      <MD_CameraVideoOff ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ClipboardMinus") => html! {
      <MD_ClipboardMinus ..owned_props />
    },
    implicit_clone::unsync::IString::Static("CloudCheck") => html! {
      <MD_CloudCheck ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FastForward") => html! {
      <MD_FastForward ..owned_props />
    },
    implicit_clone::unsync::IString::Static("JournalCode") => html! {
      <MD_JournalCode ..owned_props />
    },
    implicit_clone::unsync::IString::Static("CcCircleFill") => html! {
      <MD_CcCircleFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("TelephoneXFill") => html! {
      <MD_TelephoneXFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Wallet") => html! {
      <MD_Wallet ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ChevronCompactDown") => html! {
      <MD_ChevronCompactDown ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Printer") => html! {
      <MD_Printer ..owned_props />
    },
    implicit_clone::unsync::IString::Static("PersonWorkspace") => html! {
      <MD_PersonWorkspace ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Calendar2DayFill") => html! {
      <MD_Calendar2DayFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("CarFrontFill") => html! {
      <MD_CarFrontFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Stickies") => html! {
      <MD_Stickies ..owned_props />
    },
    implicit_clone::unsync::IString::Static("TextWrap") => html! {
      <MD_TextWrap ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ThunderboltFill") => html! {
      <MD_ThunderboltFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("BoxArrowInDownRight") => html! {
      <MD_BoxArrowInDownRight ..owned_props />
    },
    implicit_clone::unsync::IString::Static("HouseX") => html! {
      <MD_HouseX ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FilterCircleFill") => html! {
      <MD_FilterCircleFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("JustifyLeft") => html! {
      <MD_JustifyLeft ..owned_props />
    },
    implicit_clone::unsync::IString::Static("WindowStack") => html! {
      <MD_WindowStack ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FastForwardBtnFill") => html! {
      <MD_FastForwardBtnFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FileBinaryFill") => html! {
      <MD_FileBinaryFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("SignNoRightTurn") => html! {
      <MD_SignNoRightTurn ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Badge8kFill") => html! {
      <MD_Badge8kFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Postcard") => html! {
      <MD_Postcard ..owned_props />
    },
    implicit_clone::unsync::IString::Static("5Square") => html! {
      <MD_5Square ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ListTask") => html! {
      <MD_ListTask ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ClipboardPlus") => html! {
      <MD_ClipboardPlus ..owned_props />
    },
    implicit_clone::unsync::IString::Static("JournalArrowDown") => html! {
      <MD_JournalArrowDown ..owned_props />
    },
    implicit_clone::unsync::IString::Static("EyeSlash") => html! {
      <MD_EyeSlash ..owned_props />
    },
    implicit_clone::unsync::IString::Static("CloudLightning") => html! {
      <MD_CloudLightning ..owned_props />
    },
    implicit_clone::unsync::IString::Static("FileEarmarkPost") => html! {
      <MD_FileEarmarkPost ..owned_props />
    },
    implicit_clone::unsync::IString::Static("BagCheckFill") => html! {
      <MD_BagCheckFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Spotify") => html! {
      <MD_Spotify ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Superscript") => html! {
      <MD_Superscript ..owned_props />
    },
    implicit_clone::unsync::IString::Static("HeptagonFill") => html! {
      <MD_HeptagonFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("HSquareFill") => html! {
      <MD_HSquareFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("AlignCenter") => html! {
      <MD_AlignCenter ..owned_props />
    },
    implicit_clone::unsync::IString::Static("PatchMinus") => html! {
      <MD_PatchMinus ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Mask") => html! {
      <MD_Mask ..owned_props />
    },
    implicit_clone::unsync::IString::Static("CloudUpload") => html! {
      <MD_CloudUpload ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Snow3") => html! {
      <MD_Snow3 ..owned_props />
    },
    implicit_clone::unsync::IString::Static("MinecartLoaded") => html! {
      <MD_MinecartLoaded ..owned_props />
    },
    implicit_clone::unsync::IString::Static("CloudMinus") => html! {
      <MD_CloudMinus ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ThermometerHigh") => html! {
      <MD_ThermometerHigh ..owned_props />
    },
    implicit_clone::unsync::IString::Static("TerminalDash") => html! {
      <MD_TerminalDash ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ThreeDots") => html! {
      <MD_ThreeDots ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ArrowBarDown") => html! {
      <MD_ArrowBarDown ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Thunderbolt") => html! {
      <MD_Thunderbolt ..owned_props />
    },
    implicit_clone::unsync::IString::Static("ThermometerSnow") => html! {
      <MD_ThermometerSnow ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Microsoft") => html! {
      <MD_Microsoft ..owned_props />
    },
    implicit_clone::unsync::IString::Static("TelephoneForwardFill") => html! {
      <MD_TelephoneForwardFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("CaretUpFill") => html! {
      <MD_CaretUpFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Lightning") => html! {
      <MD_Lightning ..owned_props />
    },
    implicit_clone::unsync::IString::Static("RewindCircle") => html! {
      <MD_RewindCircle ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Clipboard2") => html! {
      <MD_Clipboard2 ..owned_props />
    },
    implicit_clone::unsync::IString::Static("DatabaseFillUp") => html! {
      <MD_DatabaseFillUp ..owned_props />
    },
    implicit_clone::unsync::IString::Static("CupHot") => html! {
      <MD_CupHot ..owned_props />
    },
    implicit_clone::unsync::IString::Static("UsbMicro") => html! {
      <MD_UsbMicro ..owned_props />
    },
    implicit_clone::unsync::IString::Static("BuildingX") => html! {
      <MD_BuildingX ..owned_props />
    },
    implicit_clone::unsync::IString::Static("CloudLightningRainFill") => html! {
      <MD_CloudLightningRainFill ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Union") => html! {
      <MD_Union ..owned_props />
    },
    implicit_clone::unsync::IString::Static("Signpost2") => html! {
      <MD_Signpost2 ..owned_props />
    },

    _ => html! {},
  }
}
