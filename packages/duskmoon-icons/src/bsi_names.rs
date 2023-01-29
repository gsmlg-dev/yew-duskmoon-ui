use yew::prelude::*;

use self::bsi::IconProps;

use super::bsi::BS_Check2Circle;
use super::bsi::BS_HouseUp;
use super::bsi::BS_ArrowDownLeftSquareFill;
use super::bsi::BS_FiletypeExe;
use super::bsi::BS_Tornado;
use super::bsi::BS_ViewStacked;
use super::bsi::BS_DropletHalf;
use super::bsi::BS_TrashFill;
use super::bsi::BS_NodePlusFill;
use super::bsi::BS_ArrowUpLeftCircle;
use super::bsi::BS_PentagonHalf;
use super::bsi::BS_DatabaseFill;
use super::bsi::BS_FileEarmarkArrowUp;
use super::bsi::BS_Buildings;
use super::bsi::BS_5Circle;
use super::bsi::BS_FiletypeGif;
use super::bsi::BS_BorderMiddle;
use super::bsi::BS_ExplicitFill;
use super::bsi::BS_Scooter;
use super::bsi::BS_Compass;
use super::bsi::BS_CalendarDate;
use super::bsi::BS_Youtube;
use super::bsi::BS_ChatLeftQuoteFill;
use super::bsi::BS_CloudFog2Fill;
use super::bsi::BS_7CircleFill;
use super::bsi::BS_PlusCircle;
use super::bsi::BS_CaretUpSquare;
use super::bsi::BS_Explicit;
use super::bsi::BS_Check2Square;
use super::bsi::BS_CSquare;
use super::bsi::BS_PatchCheck;
use super::bsi::BS_CaretUp;
use super::bsi::BS_BarChartSteps;
use super::bsi::BS_CardChecklist;
use super::bsi::BS_Badge4k;
use super::bsi::BS_CCircleFill;
use super::bsi::BS_FileEarmarkDiffFill;
use super::bsi::BS_FileImageFill;
use super::bsi::BS_HouseSlashFill;
use super::bsi::BS_TicketPerforatedFill;
use super::bsi::BS_PCircleFill;
use super::bsi::BS_StarHalf;
use super::bsi::BS_CloudHailFill;
use super::bsi::BS_HouseCheck;
use super::bsi::BS_CalendarWeekFill;
use super::bsi::BS_JournalMedical;
use super::bsi::BS_CloudFog2;
use super::bsi::BS_FiletypeWoff;
use super::bsi::BS_BookmarkPlus;
use super::bsi::BS_VolumeUpFill;
use super::bsi::BS_FileEarmarkEasel;
use super::bsi::BS_Dribbble;
use super::bsi::BS_SunsetFill;
use super::bsi::BS_PersonFillLock;
use super::bsi::BS_Keyboard;
use super::bsi::BS_FileEarmarkImageFill;
use super::bsi::BS_FileEarmarkPdf;
use super::bsi::BS_SortAlphaDownAlt;
use super::bsi::BS_ChatSquareTextFill;
use super::bsi::BS_Easel2;
use super::bsi::BS_Inbox;
use super::bsi::BS_ArrowUpRightCircle;
use super::bsi::BS_FiletypeTxt;
use super::bsi::BS_FiletypeSql;
use super::bsi::BS_Toggles;
use super::bsi::BS_SortAlphaUpAlt;
use super::bsi::BS_Trash3Fill;
use super::bsi::BS_BalloonHeart;
use super::bsi::BS_ClipboardHeartFill;
use super::bsi::BS_FileMedical;
use super::bsi::BS_BuildingFillGear;
use super::bsi::BS_TrainFront;
use super::bsi::BS_HousesFill;
use super::bsi::BS_Thermometer;
use super::bsi::BS_0Circle;
use super::bsi::BS_FileEarmarkBinary;
use super::bsi::BS_ChatLeftTextFill;
use super::bsi::BS_Bicycle;
use super::bsi::BS_Translate;
use super::bsi::BS_FileEarmarkPlus;
use super::bsi::BS_BookmarkFill;
use super::bsi::BS_ChevronLeft;
use super::bsi::BS_EraserFill;
use super::bsi::BS_XOctagon;
use super::bsi::BS_TagFill;
use super::bsi::BS_FileEarmarkPlay;
use super::bsi::BS_Trash2;
use super::bsi::BS_Hourglass;
use super::bsi::BS_Question;
use super::bsi::BS_LifePreserver;
use super::bsi::BS_BookmarkStar;
use super::bsi::BS_ConeStriped;
use super::bsi::BS_QuestionCircleFill;
use super::bsi::BS_ExclamationOctagonFill;
use super::bsi::BS_DashLg;
use super::bsi::BS_FiletypeJpg;
use super::bsi::BS_Dice2;
use super::bsi::BS_SignMergeRightFill;
use super::bsi::BS_Telephone;
use super::bsi::BS_Hammer;
use super::bsi::BS_Twitch;
use super::bsi::BS_FileSlides;
use super::bsi::BS_Eraser;
use super::bsi::BS_VolumeMuteFill;
use super::bsi::BS_HouseCheckFill;
use super::bsi::BS_FiletypeOtf;
use super::bsi::BS_CreditCard2FrontFill;
use super::bsi::BS_Tv;
use super::bsi::BS_FileEarmarkExcelFill;
use super::bsi::BS_EmojiSmileUpsideDown;
use super::bsi::BS_TypeH2;
use super::bsi::BS_PSquareFill;
use super::bsi::BS_LayerForward;
use super::bsi::BS_Cursor;
use super::bsi::BS_FolderMinus;
use super::bsi::BS_Eyedropper;
use super::bsi::BS_TicketFill;
use super::bsi::BS_StopwatchFill;
use super::bsi::BS_Cup;
use super::bsi::BS_Nvidia;
use super::bsi::BS_BookmarkHeartFill;
use super::bsi::BS_TicketDetailed;
use super::bsi::BS_Paypal;
use super::bsi::BS_Grid3x2;
use super::bsi::BS_CCircle;
use super::bsi::BS_Mortarboard;
use super::bsi::BS_ChatSquareHeart;
use super::bsi::BS_Terminal;
use super::bsi::BS_FolderSymlink;
use super::bsi::BS_PersonHeart;
use super::bsi::BS_ArrowRightShort;
use super::bsi::BS_JournalPlus;
use super::bsi::BS_FileX;
use super::bsi::BS_Pass;
use super::bsi::BS_EnvelopePlus;
use super::bsi::BS_Wifi1;
use super::bsi::BS_FilesAlt;
use super::bsi::BS_TruckFlatbed;
use super::bsi::BS_ChatLeftQuote;
use super::bsi::BS_HouseGear;
use super::bsi::BS_FolderSymlinkFill;
use super::bsi::BS_Diagram2;
use super::bsi::BS_FiletypePsd;
use super::bsi::BS_PersonPlus;
use super::bsi::BS_EmojiSmileFill;
use super::bsi::BS_RewindBtn;
use super::bsi::BS_ChevronExpand;
use super::bsi::BS_WindowDash;
use super::bsi::BS_CloudRainHeavy;
use super::bsi::BS_SkipEndBtn;
use super::bsi::BS_FileFontFill;
use super::bsi::BS_Prescription;
use super::bsi::BS_Handbag;
use super::bsi::BS_Ear;
use super::bsi::BS_Xbox;
use super::bsi::BS_Plugin;
use super::bsi::BS_BrightnessAltLowFill;
use super::bsi::BS_ChatRightHeartFill;
use super::bsi::BS_GooglePlay;
use super::bsi::BS_BalloonFill;
use super::bsi::BS_FileEarmarkArrowDownFill;
use super::bsi::BS_Arrow90degUp;
use super::bsi::BS_Mic;
use super::bsi::BS_1CircleFill;
use super::bsi::BS_BrightnessLow;
use super::bsi::BS_Clipboard2Minus;
use super::bsi::BS_AspectRatioFill;
use super::bsi::BS_SignTurnSlightRightFill;
use super::bsi::BS_WindowX;
use super::bsi::BS_ChatLeftDotsFill;
use super::bsi::BS_SignDeadEnd;
use super::bsi::BS_ChevronUp;
use super::bsi::BS_Bookmark;
use super::bsi::BS_FileEarmarkCodeFill;
use super::bsi::BS_Subscript;
use super::bsi::BS_Fan;
use super::bsi::BS_Dice1;
use super::bsi::BS_SignStopLightsFill;
use super::bsi::BS_ArrowDownLeftSquare;
use super::bsi::BS_ArrowUpLeftCircleFill;
use super::bsi::BS_DatabaseFillExclamation;
use super::bsi::BS_Speedometer;
use super::bsi::BS_ChatHeartFill;
use super::bsi::BS_HeartHalf;
use super::bsi::BS_ShieldFillCheck;
use super::bsi::BS_CalendarDay;
use super::bsi::BS_HourglassTop;
use super::bsi::BS_BackspaceReverseFill;
use super::bsi::BS_Brush;
use super::bsi::BS_FileEarmarkPpt;
use super::bsi::BS_Yelp;
use super::bsi::BS_Stop;
use super::bsi::BS_EmojiExpressionless;
use super::bsi::BS_SkipForwardBtn;
use super::bsi::BS_Columns;
use super::bsi::BS_InfoCircleFill;
use super::bsi::BS_6CircleFill;
use super::bsi::BS_QrCode;
use super::bsi::BS_DiamondFill;
use super::bsi::BS_CloudRainHeavyFill;
use super::bsi::BS_CursorText;
use super::bsi::BS_ArchiveFill;
use super::bsi::BS_ChevronBarUp;
use super::bsi::BS_CaretRightFill;
use super::bsi::BS_FastForwardCircle;
use super::bsi::BS_Map;
use super::bsi::BS_CartFill;
use super::bsi::BS_CartCheckFill;
use super::bsi::BS_FilePlayFill;
use super::bsi::BS_LayoutSidebarInset;
use super::bsi::BS_CcSquare;
use super::bsi::BS_SdCard;
use super::bsi::BS_EmojiDizzy;
use super::bsi::BS_Wifi;
use super::bsi::BS_ThermometerLow;
use super::bsi::BS_FileLock2Fill;
use super::bsi::BS_BagPlus;
use super::bsi::BS_CameraVideo;
use super::bsi::BS_BoxArrowLeft;
use super::bsi::BS_BootstrapFill;
use super::bsi::BS_Sunset;
use super::bsi::BS_FilterCircle;
use super::bsi::BS_CapsulePill;
use super::bsi::BS_SendSlashFill;
use super::bsi::BS_BuildingFill;
use super::bsi::BS_Files;
use super::bsi::BS_BagCheck;
use super::bsi::BS_JournalBookmarkFill;
use super::bsi::BS_CloudSnowFill;
use super::bsi::BS_OctagonHalf;
use super::bsi::BS_TerminalSplit;
use super::bsi::BS_CalendarMinus;
use super::bsi::BS_ArrowLeftSquareFill;
use super::bsi::BS_ChevronContract;
use super::bsi::BS_Router;
use super::bsi::BS_Calendar2Day;
use super::bsi::BS_PersonPlusFill;
use super::bsi::BS_SignpostFill;
use super::bsi::BS_Disc;
use super::bsi::BS_Grid3x2GapFill;
use super::bsi::BS_3Square;
use super::bsi::BS_Basket3Fill;
use super::bsi::BS_ArrowUpCircle;
use super::bsi::BS_GlobeEuropeAfrica;
use super::bsi::BS_FileExcelFill;
use super::bsi::BS_Discord;
use super::bsi::BS_Prescription2;
use super::bsi::BS_ArrowReturnLeft;
use super::bsi::BS_Cart;
use super::bsi::BS_CalendarDayFill;
use super::bsi::BS_3CircleFill;
use super::bsi::BS_MusicNoteBeamed;
use super::bsi::BS_ArrowDownCircle;
use super::bsi::BS_CloudyFill;
use super::bsi::BS_DisplayFill;
use super::bsi::BS_FilePersonFill;
use super::bsi::BS_ClipboardData;
use super::bsi::BS_RocketTakeoff;
use super::bsi::BS_PersonFillCheck;
use super::bsi::BS_HeadsetVr;
use super::bsi::BS_BookmarksFill;
use super::bsi::BS_DatabaseSlash;
use super::bsi::BS_DistributeVertical;
use super::bsi::BS_BarChartFill;
use super::bsi::BS_SuitHeart;
use super::bsi::BS_HCircleFill;
use super::bsi::BS_Server;
use super::bsi::BS_DistributeHorizontal;
use super::bsi::BS_SignIntersectionY;
use super::bsi::BS_BuildingCheck;
use super::bsi::BS_BackspaceReverse;
use super::bsi::BS_ListUl;
use super::bsi::BS_SortDown;
use super::bsi::BS_FileMedicalFill;
use super::bsi::BS_FileEarmarkMinusFill;
use super::bsi::BS_ShieldFillX;
use super::bsi::BS_ChatRightHeart;
use super::bsi::BS_ArrowDownSquareFill;
use super::bsi::BS_Paragraph;
use super::bsi::BS_TerminalX;
use super::bsi::BS_ChatLeftFill;
use super::bsi::BS_Soundwave;
use super::bsi::BS_EnvelopeAtFill;
use super::bsi::BS_Arrow90degDown;
use super::bsi::BS_FileRuledFill;
use super::bsi::BS_ExclamationTriangle;
use super::bsi::BS_UniversalAccess;
use super::bsi::BS_SendCheck;
use super::bsi::BS_Hypnotize;
use super::bsi::BS_PatchPlusFill;
use super::bsi::BS_CartDash;
use super::bsi::BS_StackOverflow;
use super::bsi::BS_BellSlashFill;
use super::bsi::BS_FileEarmarkArrowDown;
use super::bsi::BS_LayoutTextSidebar;
use super::bsi::BS_HandIndexThumb;
use super::bsi::BS_StoplightsFill;
use super::bsi::BS_Minecart;
use super::bsi::BS_BorderTop;
use super::bsi::BS_Fullscreen;
use super::bsi::BS_FileLock;
use super::bsi::BS_ShieldCheck;
use super::bsi::BS_ChatLeftHeartFill;
use super::bsi::BS_PersonDown;
use super::bsi::BS_Dash;
use super::bsi::BS_Reception4;
use super::bsi::BS_Speaker;
use super::bsi::BS_ClipboardX;
use super::bsi::BS_BookmarkCheckFill;
use super::bsi::BS_BinocularsFill;
use super::bsi::BS_TrainLightrailFront;
use super::bsi::BS_TreeFill;
use super::bsi::BS_ArrowThroughHeart;
use super::bsi::BS_FilePerson;
use super::bsi::BS_HouseLockFill;
use super::bsi::BS_FileEarmarkPptFill;
use super::bsi::BS_Tree;
use super::bsi::BS_Pinterest;
use super::bsi::BS_Gem;
use super::bsi::BS_Clipboard2PulseFill;
use super::bsi::BS_Dice2Fill;
use super::bsi::BS_CartCheck;
use super::bsi::BS_FuelPumpDieselFill;
use super::bsi::BS_ThreeDotsVertical;
use super::bsi::BS_FuelPumpDiesel;
use super::bsi::BS_XSquare;
use super::bsi::BS_PersonRolodex;
use super::bsi::BS_Calendar2MinusFill;
use super::bsi::BS_GlobeAmericas;
use super::bsi::BS_DoorClosed;
use super::bsi::BS_VinylFill;
use super::bsi::BS_ListStars;
use super::bsi::BS_PlayCircleFill;
use super::bsi::BS_SendExclamation;
use super::bsi::BS_Bezier;
use super::bsi::BS_PauseCircleFill;
use super::bsi::BS_LayoutTextWindow;
use super::bsi::BS_PinAngleFill;
use super::bsi::BS_SpeakerFill;
use super::bsi::BS_Slack;
use super::bsi::BS_Basket;
use super::bsi::BS_BookmarkX;
use super::bsi::BS_BatteryHalf;
use super::bsi::BS_CalendarEventFill;
use super::bsi::BS_EnvelopeSlashFill;
use super::bsi::BS_PuzzleFill;
use super::bsi::BS_CalendarCheck;
use super::bsi::BS_ImageAlt;
use super::bsi::BS_FiletypePy;
use super::bsi::BS_FilePostFill;
use super::bsi::BS_Braces;
use super::bsi::BS_SkipEndCircleFill;
use super::bsi::BS_LightbulbOffFill;
use super::bsi::BS_BrowserChrome;
use super::bsi::BS_Plus;
use super::bsi::BS_GenderAmbiguous;
use super::bsi::BS_Sliders;
use super::bsi::BS_Whatsapp;
use super::bsi::BS_BrightnessLowFill;
use super::bsi::BS_SkipBackwardFill;
use super::bsi::BS_PersonX;
use super::bsi::BS_VolumeUp;
use super::bsi::BS_Laptop;
use super::bsi::BS_Apple;
use super::bsi::BS_PersonDashFill;
use super::bsi::BS_Unindent;
use super::bsi::BS_MoonFill;
use super::bsi::BS_8SquareFill;
use super::bsi::BS_HouseHeartFill;
use super::bsi::BS_FileArrowUpFill;
use super::bsi::BS_Snow2;
use super::bsi::BS_At;
use super::bsi::BS_CheckSquare;
use super::bsi::BS_FileTextFill;
use super::bsi::BS_FileEarmarkLock;
use super::bsi::BS_FiletypeM4p;
use super::bsi::BS_SendExclamationFill;
use super::bsi::BS_TelephoneMinus;
use super::bsi::BS_Google;
use super::bsi::BS_JournalRichtext;
use super::bsi::BS_0CircleFill;
use super::bsi::BS_ArrowLeftCircleFill;
use super::bsi::BS_StopBtnFill;
use super::bsi::BS_Moon;
use super::bsi::BS_TabletLandscapeFill;
use super::bsi::BS_Reply;
use super::bsi::BS_QuestionSquare;
use super::bsi::BS_FiletypeAi;
use super::bsi::BS_TruckFront;
use super::bsi::BS_SignIntersectionT;
use super::bsi::BS_ClockHistory;
use super::bsi::BS_TriangleHalf;
use super::bsi::BS_Sunglasses;
use super::bsi::BS_TypeH3;
use super::bsi::BS_6Circle;
use super::bsi::BS_Pentagon;
use super::bsi::BS_FileEarmarkX;
use super::bsi::BS_StickiesFill;
use super::bsi::BS_NintendoSwitch;
use super::bsi::BS_CollectionPlayFill;
use super::bsi::BS_JournalBookmark;
use super::bsi::BS_StickyFill;
use super::bsi::BS_CurrencyBitcoin;
use super::bsi::BS_BackspaceFill;
use super::bsi::BS_TextLeft;
use super::bsi::BS_Headphones;
use super::bsi::BS_GeoFill;
use super::bsi::BS_CloudMinusFill;
use super::bsi::BS_FileWord;
use super::bsi::BS_FileDiffFill;
use super::bsi::BS_LightbulbOff;
use super::bsi::BS_Box2Heart;
use super::bsi::BS_EnvelopeOpenFill;
use super::bsi::BS_ArrowLeftShort;
use super::bsi::BS_PostcardHeart;
use super::bsi::BS_Journals;
use super::bsi::BS_Link;
use super::bsi::BS_TextIndentLeft;
use super::bsi::BS_BookHalf;
use super::bsi::BS_ArrowUpRightSquareFill;
use super::bsi::BS_FileSpreadsheetFill;
use super::bsi::BS_UiRadiosGrid;
use super::bsi::BS_Bank;
use super::bsi::BS_FiletypeSh;
use super::bsi::BS_BoundingBox;
use super::bsi::BS_CameraFill;
use super::bsi::BS_Memory;
use super::bsi::BS_Quote;
use super::bsi::BS_FilePost;
use super::bsi::BS_Calendar3RangeFill;
use super::bsi::BS_HeartPulse;
use super::bsi::BS_SkipStartCircle;
use super::bsi::BS_Amd;
use super::bsi::BS_TextCenter;
use super::bsi::BS_ListOl;
use super::bsi::BS_SignDoNotEnterFill;
use super::bsi::BS_LayoutTextSidebarReverse;
use super::bsi::BS_PersonVcardFill;
use super::bsi::BS_DiamondHalf;
use super::bsi::BS_Reception3;
use super::bsi::BS_SortDownAlt;
use super::bsi::BS_Wifi2;
use super::bsi::BS_Eye;
use super::bsi::BS_CalendarXFill;
use super::bsi::BS_Incognito;
use super::bsi::BS_ArrowUpCircleFill;
use super::bsi::BS_HddStack;
use super::bsi::BS_Check;
use super::bsi::BS_InfoSquare;
use super::bsi::BS_FileEarmarkBreak;
use super::bsi::BS_Infinity;
use super::bsi::BS_BuildingUp;
use super::bsi::BS_JournalX;
use super::bsi::BS_CalendarPlus;
use super::bsi::BS_ExclamationCircle;
use super::bsi::BS_DatabaseFillLock;
use super::bsi::BS_Calendar2DateFill;
use super::bsi::BS_Palette;
use super::bsi::BS_Window;
use super::bsi::BS_BookmarkHeart;
use super::bsi::BS_Asterisk;
use super::bsi::BS_Basket2;
use super::bsi::BS_MicMuteFill;
use super::bsi::BS_BrowserSafari;
use super::bsi::BS_ChatRightDotsFill;
use super::bsi::BS_ClipboardMinusFill;
use super::bsi::BS_Circle;
use super::bsi::BS_Easel3Fill;
use super::bsi::BS_TrainFreightFrontFill;
use super::bsi::BS_GearWide;
use super::bsi::BS_ChevronRight;
use super::bsi::BS_FileEarmarkBinaryFill;
use super::bsi::BS_Wikipedia;
use super::bsi::BS_9CircleFill;
use super::bsi::BS_Droplet;
use super::bsi::BS_SignpostSplit;
use super::bsi::BS_StopCircle;
use super::bsi::BS_DatabaseFillGear;
use super::bsi::BS_CaretLeft;
use super::bsi::BS_InfoCircle;
use super::bsi::BS_DatabaseFillX;
use super::bsi::BS_Triangle;
use super::bsi::BS_HandThumbsUpFill;
use super::bsi::BS_JournalAlbum;
use super::bsi::BS_Cart2;
use super::bsi::BS_ShieldX;
use super::bsi::BS_Record2;
use super::bsi::BS_SuitDiamondFill;
use super::bsi::BS_JournalText;
use super::bsi::BS_SignYield;
use super::bsi::BS_Puzzle;
use super::bsi::BS_TelephoneMinusFill;
use super::bsi::BS_FilePdf;
use super::bsi::BS_MusicNote;
use super::bsi::BS_ArrowsAngleContract;
use super::bsi::BS_BoxArrowInLeft;
use super::bsi::BS_Recycle;
use super::bsi::BS_ChevronDoubleDown;
use super::bsi::BS_FiletypePptx;
use super::bsi::BS_XCircle;
use super::bsi::BS_TypeBold;
use super::bsi::BS_PhoneLandscape;
use super::bsi::BS_DatabaseDown;
use super::bsi::BS_DatabaseFillSlash;
use super::bsi::BS_PersonBadgeFill;
use super::bsi::BS_PieChart;
use super::bsi::BS_Exclamation;
use super::bsi::BS_FileEarmarkLock2;
use super::bsi::BS_DatabaseGear;
use super::bsi::BS_BookmarkStarFill;
use super::bsi::BS_SkipStartBtn;
use super::bsi::BS_SignNoRightTurnFill;
use super::bsi::BS_BagX;
use super::bsi::BS_ZoomIn;
use super::bsi::BS_ArrowRightCircleFill;
use super::bsi::BS_ClipboardCheck;
use super::bsi::BS_CameraVideoOffFill;
use super::bsi::BS_Database;
use super::bsi::BS_GearWideConnected;
use super::bsi::BS_TelephonePlus;
use super::bsi::BS_FileEarmarkWordFill;
use super::bsi::BS_EyeSlashFill;
use super::bsi::BS_Search;
use super::bsi::BS_Calendar2Range;
use super::bsi::BS_ChatSquareDots;
use super::bsi::BS_Reception0;
use super::bsi::BS_Calendar2WeekFill;
use super::bsi::BS_FileZip;
use super::bsi::BS_GlobeCentralSouthAsia;
use super::bsi::BS_PinMapFill;
use super::bsi::BS_LayoutSplit;
use super::bsi::BS_CloudCheckFill;
use super::bsi::BS_MortarboardFill;
use super::bsi::BS_0SquareFill;
use super::bsi::BS_FileLock2;
use super::bsi::BS_HouseDown;
use super::bsi::BS_CloudHaze2Fill;
use super::bsi::BS_ArrowDownShort;
use super::bsi::BS_BookmarkXFill;
use super::bsi::BS_EvStation;
use super::bsi::BS_FilterRight;
use super::bsi::BS_HeartFill;
use super::bsi::BS_UnlockFill;
use super::bsi::BS_FileEarmarkMusicFill;
use super::bsi::BS_Mouse3;
use super::bsi::BS_HourglassBottom;
use super::bsi::BS_Joystick;
use super::bsi::BS_FileEarmarkMinus;
use super::bsi::BS_Lightbulb;
use super::bsi::BS_FileEarmarkSpreadsheet;
use super::bsi::BS_FiletypeWav;
use super::bsi::BS_Bag;
use super::bsi::BS_LightningChargeFill;
use super::bsi::BS_Calendar3;
use super::bsi::BS_OpticalAudio;
use super::bsi::BS_DatabaseCheck;
use super::bsi::BS_ChevronDoubleRight;
use super::bsi::BS_Pc;
use super::bsi::BS_EnvelopePaperFill;
use super::bsi::BS_FileBreak;
use super::bsi::BS_Tsunami;
use super::bsi::BS_CheckSquareFill;
use super::bsi::BS_6SquareFill;
use super::bsi::BS_TextRight;
use super::bsi::BS_SkipStartBtnFill;
use super::bsi::BS_SuitHeartFill;
use super::bsi::BS_EarFill;
use super::bsi::BS_Robot;
use super::bsi::BS_UsbDriveFill;
use super::bsi::BS_Mailbox2;
use super::bsi::BS_FiletypeHeic;
use super::bsi::BS_BoxSeamFill;
use super::bsi::BS_FileEarmarkRichtext;
use super::bsi::BS_Signpost2Fill;
use super::bsi::BS_Nut;
use super::bsi::BS_Alexa;
use super::bsi::BS_FileEarmarkFontFill;
use super::bsi::BS_CloudDrizzle;
use super::bsi::BS_Key;
use super::bsi::BS_DisplayportFill;
use super::bsi::BS_ClipboardHeart;
use super::bsi::BS_Sim;
use super::bsi::BS_Pencil;
use super::bsi::BS_Save;
use super::bsi::BS_UmbrellaFill;
use super::bsi::BS_TabletLandscape;
use super::bsi::BS_SlashSquareFill;
use super::bsi::BS_Ethernet;
use super::bsi::BS_BadgeAdFill;
use super::bsi::BS_ChevronDown;
use super::bsi::BS_BadgeWcFill;
use super::bsi::BS_Eject;
use super::bsi::BS_Arrow90degRight;
use super::bsi::BS_ThermometerSun;
use super::bsi::BS_ChatRightTextFill;
use super::bsi::BS_FastForwardFill;
use super::bsi::BS_SignNoParkingFill;
use super::bsi::BS_FileEarmarkFill;
use super::bsi::BS_VolumeDownFill;
use super::bsi::BS_MusicNoteList;
use super::bsi::BS_EnvelopeExclamationFill;
use super::bsi::BS_TelephoneX;
use super::bsi::BS_Virus;
use super::bsi::BS_Basket3;
use super::bsi::BS_Box2HeartFill;
use super::bsi::BS_SignIntersectionSideFill;
use super::bsi::BS_Hearts;
use super::bsi::BS_SignMergeLeft;
use super::bsi::BS_CartPlus;
use super::bsi::BS_Heart;
use super::bsi::BS_Intersect;
use super::bsi::BS_CloudSunFill;
use super::bsi::BS_ArrowRightCircle;
use super::bsi::BS_Textarea;
use super::bsi::BS_SymmetryVertical;
use super::bsi::BS_PersonAdd;
use super::bsi::BS_Share;
use super::bsi::BS_CalendarRangeFill;
use super::bsi::BS_EnvelopeHeart;
use super::bsi::BS_ChatSquareDotsFill;
use super::bsi::BS_Bookshelf;
use super::bsi::BS_LayerBackward;
use super::bsi::BS_Calendar4Range;
use super::bsi::BS_Calendar3Fill;
use super::bsi::BS_DashCircle;
use super::bsi::BS_BadgeCc;
use super::bsi::BS_Spellcheck;
use super::bsi::BS_FiletypeDoc;
use super::bsi::BS_Calendar2HeartFill;
use super::bsi::BS_BadgeHdFill;
use super::bsi::BS_ReceiptCutoff;
use super::bsi::BS_BuildingFillLock;
use super::bsi::BS_GraphDown;
use super::bsi::BS_CaretDownSquareFill;
use super::bsi::BS_PersonLinesFill;
use super::bsi::BS_Sticky;
use super::bsi::BS_CarFront;
use super::bsi::BS_1Circle;
use super::bsi::BS_SinaWeibo;
use super::bsi::BS_WrenchAdjustableCircleFill;
use super::bsi::BS_SignTurnSlightLeft;
use super::bsi::BS_SuitSpadeFill;
use super::bsi::BS_BookmarkCheck;
use super::bsi::BS_FuelPumpFill;
use super::bsi::BS_SortUp;
use super::bsi::BS_FiletypeMd;
use super::bsi::BS_SkipEndFill;
use super::bsi::BS_EnvelopeDashFill;
use super::bsi::BS_FileEarmarkRuled;
use super::bsi::BS_Vimeo;
use super::bsi::BS_Info;
use super::bsi::BS_SortNumericDown;
use super::bsi::BS_MenuUp;
use super::bsi::BS_4Square;
use super::bsi::BS_CloudLightningFill;
use super::bsi::BS_TrophyFill;
use super::bsi::BS_PenFill;
use super::bsi::BS_FileMinusFill;
use super::bsi::BS_ArrowDownRightCircle;
use super::bsi::BS_EmojiAngryFill;
use super::bsi::BS_AwardFill;
use super::bsi::BS_Wrench;
use super::bsi::BS_FileEarmarkText;
use super::bsi::BS_CloudArrowUp;
use super::bsi::BS_Star;
use super::bsi::BS_FileEarmarkRuledFill;
use super::bsi::BS_Stripe;
use super::bsi::BS_EnvelopeX;
use super::bsi::BS_FiletypeAac;
use super::bsi::BS_EvStationFill;
use super::bsi::BS_Postage;
use super::bsi::BS_SkipForward;
use super::bsi::BS_CartX;
use super::bsi::BS_CardImage;
use super::bsi::BS_HeartArrow;
use super::bsi::BS_Arrow90degLeft;
use super::bsi::BS_UsbPlugFill;
use super::bsi::BS_FileEarmarkPlusFill;
use super::bsi::BS_BagXFill;
use super::bsi::BS_BagHeart;
use super::bsi::BS_PersonFillSlash;
use super::bsi::BS_ChatHeart;
use super::bsi::BS_DatabaseFillCheck;
use super::bsi::BS_SignStopFill;
use super::bsi::BS_UsbC;
use super::bsi::BS_EmojiLaughing;
use super::bsi::BS_Heartbreak;
use super::bsi::BS_Mouse;
use super::bsi::BS_Lamp;
use super::bsi::BS_Dice6Fill;
use super::bsi::BS_CurrencyExchange;
use super::bsi::BS_EmojiSunglassesFill;
use super::bsi::BS_Reception1;
use super::bsi::BS_ClipboardPlusFill;
use super::bsi::BS_Scissors;
use super::bsi::BS_Linkedin;
use super::bsi::BS_ArrowReturnRight;
use super::bsi::BS_Repeat1;
use super::bsi::BS_BagHeartFill;
use super::bsi::BS_TerminalFill;
use super::bsi::BS_FilePlus;
use super::bsi::BS_TelephoneOutbound;
use super::bsi::BS_DatabaseExclamation;
use super::bsi::BS_FileXFill;
use super::bsi::BS_FileRichtext;
use super::bsi::BS_BuildingAdd;
use super::bsi::BS_Markdown;
use super::bsi::BS_BookmarkPlusFill;
use super::bsi::BS_DatabaseFillDash;
use super::bsi::BS_BlockquoteRight;
use super::bsi::BS_PersonSlash;
use super::bsi::BS_FiletypePpt;
use super::bsi::BS_PatchMinusFill;
use super::bsi::BS_EnvelopeOpenHeart;
use super::bsi::BS_Flower1;
use super::bsi::BS_HouseDoor;
use super::bsi::BS_CircleSquare;
use super::bsi::BS_GeoAlt;
use super::bsi::BS_BuildingFillExclamation;
use super::bsi::BS_TextIndentRight;
use super::bsi::BS_PersonCheck;
use super::bsi::BS_Calendar2Check;
use super::bsi::BS_PostcardFill;
use super::bsi::BS_PersonHearts;
use super::bsi::BS_Check2;
use super::bsi::BS_Globe;
use super::bsi::BS_PrinterFill;
use super::bsi::BS_Valentine2;
use super::bsi::BS_YinYang;
use super::bsi::BS_Cash;
use super::bsi::BS_GraphUpArrow;
use super::bsi::BS_Calendar3WeekFill;
use super::bsi::BS_2SquareFill;
use super::bsi::BS_XLg;
use super::bsi::BS_Receipt;
use super::bsi::BS_WindowSplit;
use super::bsi::BS_PatchQuestion;
use super::bsi::BS_FileFont;
use super::bsi::BS_GenderTrans;
use super::bsi::BS_Tablet;
use super::bsi::BS_Upc;
use super::bsi::BS_Clipboard2HeartFill;
use super::bsi::BS_PlusSquareDotted;
use super::bsi::BS_CloudHazeFill;
use super::bsi::BS_Display;
use super::bsi::BS_FileCode;
use super::bsi::BS_Medium;
use super::bsi::BS_SignIntersectionTFill;
use super::bsi::BS_CalendarMonthFill;
use super::bsi::BS_TropicalStorm;
use super::bsi::BS_CloudFogFill;
use super::bsi::BS_Easel;
use super::bsi::BS_Hospital;
use super::bsi::BS_Rss;
use super::bsi::BS_ChatRightText;
use super::bsi::BS_PcDisplay;
use super::bsi::BS_TelephoneInboundFill;
use super::bsi::BS_FiletypeRaw;
use super::bsi::BS_ChatSquareQuote;
use super::bsi::BS_PersonGear;
use super::bsi::BS_FileWordFill;
use super::bsi::BS_Calendar3Event;
use super::bsi::BS_BugFill;
use super::bsi::BS_EmojiAngry;
use super::bsi::BS_EggFried;
use super::bsi::BS_LayersFill;
use super::bsi::BS_HourglassSplit;
use super::bsi::BS_Border;
use super::bsi::BS_Stopwatch;
use super::bsi::BS_BorderBottom;
use super::bsi::BS_BadgeCcFill;
use super::bsi::BS_Rocket;
use super::bsi::BS_Calendar2Event;
use super::bsi::BS_PersonFillDash;
use super::bsi::BS_EnvelopePaper;
use super::bsi::BS_ChatQuote;
use super::bsi::BS_InboxFill;
use super::bsi::BS_FileEarmarkBarGraphFill;
use super::bsi::BS_HddStackFill;
use super::bsi::BS_Snapchat;
use super::bsi::BS_QuestionDiamondFill;
use super::bsi::BS_Person;
use super::bsi::BS_Box2;
use super::bsi::BS_Calendar2RangeFill;
use super::bsi::BS_Code;
use super::bsi::BS_PersonDash;
use super::bsi::BS_CloudSnow;
use super::bsi::BS_VectorPen;
use super::bsi::BS_TextParagraph;
use super::bsi::BS_BorderOuter;
use super::bsi::BS_LayoutTextWindowReverse;
use super::bsi::BS_BoxArrowUp;
use super::bsi::BS_HouseXFill;
use super::bsi::BS_FullscreenExit;
use super::bsi::BS_ArrowsAngleExpand;
use super::bsi::BS_ImageFill;
use super::bsi::BS_EyeFill;
use super::bsi::BS_PlusCircleDotted;
use super::bsi::BS_Boxes;
use super::bsi::BS_Houses;
use super::bsi::BS_HouseExclamationFill;
use super::bsi::BS_Save2Fill;
use super::bsi::BS_EaselFill;
use super::bsi::BS_PersonLock;
use super::bsi::BS_PersonVideo2;
use super::bsi::BS_EvFrontFill;
use super::bsi::BS_ListNested;
use super::bsi::BS_RewindFill;
use super::bsi::BS_DropletFill;
use super::bsi::BS_Power;
use super::bsi::BS_Screwdriver;
use super::bsi::BS_SegmentedNav;
use super::bsi::BS_PhoneVibrateFill;
use super::bsi::BS_CheckAll;
use super::bsi::BS_Dice4Fill;
use super::bsi::BS_MenuAppFill;
use super::bsi::BS_EnvelopePlusFill;
use super::bsi::BS_BorderLeft;
use super::bsi::BS_Badge3dFill;
use super::bsi::BS_Calendar3Week;
use super::bsi::BS_ChatText;
use super::bsi::BS_CaretLeftSquare;
use super::bsi::BS_SlashCircleFill;
use super::bsi::BS_WrenchAdjustableCircle;
use super::bsi::BS_ChatDots;
use super::bsi::BS_BadgeVr;
use super::bsi::BS_FileEarmarkBreakFill;
use super::bsi::BS_CloudHail;
use super::bsi::BS_FolderX;
use super::bsi::BS_EmojiWink;
use super::bsi::BS_4SquareFill;
use super::bsi::BS_ArrowDownLeft;
use super::bsi::BS_FileEarmarkPostFill;
use super::bsi::BS_People;
use super::bsi::BS_HSquare;
use super::bsi::BS_ChatRightDots;
use super::bsi::BS_UsbDrive;
use super::bsi::BS_Signal;
use super::bsi::BS_GenderFemale;
use super::bsi::BS_PersonSquare;
use super::bsi::BS_Truck;
use super::bsi::BS_SkipBackward;
use super::bsi::BS_PlusSquareFill;
use super::bsi::BS_ArrowDownCircleFill;
use super::bsi::BS_File;
use super::bsi::BS_App;
use super::bsi::BS_Grid3x2Gap;
use super::bsi::BS_Windows;
use super::bsi::BS_FileZipFill;
use super::bsi::BS_VolumeMute;
use super::bsi::BS_4Circle;
use super::bsi::BS_SignRailroad;
use super::bsi::BS_FiletypeMp4;
use super::bsi::BS_HeartbreakFill;
use super::bsi::BS_CaretDown;
use super::bsi::BS_MicrosoftTeams;
use super::bsi::BS_ShieldPlus;
use super::bsi::BS_SymmetryHorizontal;
use super::bsi::BS_Clipboard2XFill;
use super::bsi::BS_FileRichtextFill;
use super::bsi::BS_ArrowDownRightCircleFill;
use super::bsi::BS_MapFill;
use super::bsi::BS_CompassFill;
use super::bsi::BS_EmojiWinkFill;
use super::bsi::BS_FilePlay;
use super::bsi::BS_GraphUp;
use super::bsi::BS_SignTurnRightFill;
use super::bsi::BS_5SquareFill;
use super::bsi::BS_SkipBackwardCircle;
use super::bsi::BS_SortAlphaUp;
use super::bsi::BS_HexagonFill;
use super::bsi::BS_ChatSquareText;
use super::bsi::BS_SignNoLeftTurn;
use super::bsi::BS_TvFill;
use super::bsi::BS_MenuButtonFill;
use super::bsi::BS_JournalMinus;
use super::bsi::BS_CartPlusFill;
use super::bsi::BS_EnvelopeAt;
use super::bsi::BS_Journal;
use super::bsi::BS_Percent;
use super::bsi::BS_Rulers;
use super::bsi::BS_Toggle2Off;
use super::bsi::BS_BuildingsFill;
use super::bsi::BS_Mouse3Fill;
use super::bsi::BS_FiletypeTiff;
use super::bsi::BS_HouseFill;
use super::bsi::BS_BoomboxFill;
use super::bsi::BS_BuildingFillDash;
use super::bsi::BS_Clipboard2DataFill;
use super::bsi::BS_Filter;
use super::bsi::BS_Sliders2;
use super::bsi::BS_SignTurnLeft;
use super::bsi::BS_Radioactive;
use super::bsi::BS_CaretLeftFill;
use super::bsi::BS_7SquareFill;
use super::bsi::BS_CloudHaze2;
use super::bsi::BS_ArrowRight;
use super::bsi::BS_Facebook;
use super::bsi::BS_Magnet;
use super::bsi::BS_FileSlidesFill;
use super::bsi::BS_FileExcel;
use super::bsi::BS_ArrowRightSquareFill;
use super::bsi::BS_CodeSquare;
use super::bsi::BS_Calendar2XFill;
use super::bsi::BS_HouseHeart;
use super::bsi::BS_FileEarmarkImage;
use super::bsi::BS_Bucket;
use super::bsi::BS_CupStraw;
use super::bsi::BS_FileEarmarkLock2Fill;
use super::bsi::BS_MoonStarsFill;
use super::bsi::BS_FilePdfFill;
use super::bsi::BS_Gear;
use super::bsi::BS_BoxArrowInRight;
use super::bsi::BS_FiletypeHtml;
use super::bsi::BS_Table;
use super::bsi::BS_ArrowDownRightSquare;
use super::bsi::BS_Dice3Fill;
use super::bsi::BS_DatabaseDash;
use super::bsi::BS_2Circle;
use super::bsi::BS_Clipboard2Fill;
use super::bsi::BS_Rewind;
use super::bsi::BS_CreditCardFill;
use super::bsi::BS_Dot;
use super::bsi::BS_NutFill;
use super::bsi::BS_FiletypeTsx;
use super::bsi::BS_BookmarkDash;
use super::bsi::BS_CloudFog;
use super::bsi::BS_RocketFill;
use super::bsi::BS_FileEarmarkCheckFill;
use super::bsi::BS_Bluetooth;
use super::bsi::BS_Shuffle;
use super::bsi::BS_BadgeAr;
use super::bsi::BS_PipFill;
use super::bsi::BS_Bootstrap;
use super::bsi::BS_Film;
use super::bsi::BS_BatteryCharging;
use super::bsi::BS_Clipboard2Plus;
use super::bsi::BS_CloudSleetFill;
use super::bsi::BS_SkipForwardFill;
use super::bsi::BS_ChevronCompactLeft;
use super::bsi::BS_MicFill;
use super::bsi::BS_Safe2Fill;
use super::bsi::BS_KanbanFill;
use super::bsi::BS_RewindBtnFill;
use super::bsi::BS_ReplyFill;
use super::bsi::BS_Twitter;
use super::bsi::BS_ToggleOn;
use super::bsi::BS_CurrencyRupee;
use super::bsi::BS_BadgeSdFill;
use super::bsi::BS_PersonVcard;
use super::bsi::BS_Cpu;
use super::bsi::BS_CaretDownSquare;
use super::bsi::BS_NodeMinus;
use super::bsi::BS_HandbagFill;
use super::bsi::BS_DoorOpenFill;
use super::bsi::BS_EvFront;
use super::bsi::BS_ArrowBarUp;
use super::bsi::BS_PlayBtn;
use super::bsi::BS_SendPlus;
use super::bsi::BS_ChevronBarDown;
use super::bsi::BS_CaretDownFill;
use super::bsi::BS_MicMute;
use super::bsi::BS_PersonFillDown;
use super::bsi::BS_TypeH1;
use super::bsi::BS_PlayBtnFill;
use super::bsi::BS_9Circle;
use super::bsi::BS_DatabaseFillDown;
use super::bsi::BS_Unlock;
use super::bsi::BS_BrightnessAltHigh;
use super::bsi::BS_ChatDotsFill;
use super::bsi::BS_Eyeglasses;
use super::bsi::BS_ArrowsMove;
use super::bsi::BS_MouseFill;
use super::bsi::BS_FileEarmarkZip;
use super::bsi::BS_SlashCircle;
use super::bsi::BS_Alt;
use super::bsi::BS_SignIntersectionYFill;
use super::bsi::BS_BrightnessHighFill;
use super::bsi::BS_Lock;
use super::bsi::BS_CloudMoon;
use super::bsi::BS_HandIndexFill;
use super::bsi::BS_BorderStyle;
use super::bsi::BS_CloudRainFill;
use super::bsi::BS_Telegram;
use super::bsi::BS_Tag;
use super::bsi::BS_Trello;
use super::bsi::BS_CalendarMonth;
use super::bsi::BS_Safe2;
use super::bsi::BS_BuildingDash;
use super::bsi::BS_FiletypeJs;
use super::bsi::BS_BoxArrowRight;
use super::bsi::BS_Messenger;
use super::bsi::BS_ArrowLeft;
use super::bsi::BS_Palette2;
use super::bsi::BS_FileEarmarkXFill;
use super::bsi::BS_FastForwardBtn;
use super::bsi::BS_FileDiff;
use super::bsi::BS_BadgeVrFill;
use super::bsi::BS_Broadcast;
use super::bsi::BS_PauseCircle;
use super::bsi::BS_PauseFill;
use super::bsi::BS_MotherboardFill;
use super::bsi::BS_ArrowRightSquare;
use super::bsi::BS_DatabaseFillAdd;
use super::bsi::BS_FilterSquareFill;
use super::bsi::BS_Virus2;
use super::bsi::BS_ChatTextFill;
use super::bsi::BS_SkipForwardCircleFill;
use super::bsi::BS_ChatRightQuoteFill;
use super::bsi::BS_PeaceFill;
use super::bsi::BS_PinFill;
use super::bsi::BS_Box2Fill;
use super::bsi::BS_ChatSquareHeartFill;
use super::bsi::BS_BarChartLine;
use super::bsi::BS_CalendarHeart;
use super::bsi::BS_PhoneFill;
use super::bsi::BS_SendSlash;
use super::bsi::BS_TagsFill;
use super::bsi::BS_Crop;
use super::bsi::BS_FiletypeTtf;
use super::bsi::BS_MenuButtonWide;
use super::bsi::BS_Award;
use super::bsi::BS_3SquareFill;
use super::bsi::BS_GripHorizontal;
use super::bsi::BS_9SquareFill;
use super::bsi::BS_FileMusic;
use super::bsi::BS_CursorFill;
use super::bsi::BS_PersonVideo3;
use super::bsi::BS_HddFill;
use super::bsi::BS_SuitClub;
use super::bsi::BS_SuitClubFill;
use super::bsi::BS_Collection;
use super::bsi::BS_Option;
use super::bsi::BS_ChevronDoubleLeft;
use super::bsi::BS_SafeFill;
use super::bsi::BS_BrushFill;
use super::bsi::BS_6Square;
use super::bsi::BS_ShopWindow;
use super::bsi::BS_ShieldExclamation;
use super::bsi::BS_RecordCircleFill;
use super::bsi::BS_FilePptFill;
use super::bsi::BS_Dice1Fill;
use super::bsi::BS_ArrowBarLeft;
use super::bsi::BS_HddNetwork;
use super::bsi::BS_ClockFill;
use super::bsi::BS_TabletFill;
use super::bsi::BS_HeartPulseFill;
use super::bsi::BS_FiletypePdf;
use super::bsi::BS_BoxArrowInDown;
use super::bsi::BS_Repeat;
use super::bsi::BS_BasketFill;
use super::bsi::BS_Globe2;
use super::bsi::BS_SunFill;
use super::bsi::BS_Gift;
use super::bsi::BS_LayoutSidebarInsetReverse;
use super::bsi::BS_PlugFill;
use super::bsi::BS_Phone;
use super::bsi::BS_AspectRatio;
use super::bsi::BS_TruckFrontFill;
use super::bsi::BS_Behance;
use super::bsi::BS_UniversalAccessCircle;
use super::bsi::BS_GraphDownArrow;
use super::bsi::BS_FileEarmarkZipFill;
use super::bsi::BS_BandaidFill;
use super::bsi::BS_ArrowDownSquare;
use super::bsi::BS_CalendarCheckFill;
use super::bsi::BS_BookFill;
use super::bsi::BS_UsbMicroFill;
use super::bsi::BS_Controller;
use super::bsi::BS_Fonts;
use super::bsi::BS_EnvelopeExclamation;
use super::bsi::BS_Pause;
use super::bsi::BS_BookmarkDashFill;
use super::bsi::BS_BoxArrowDownLeft;
use super::bsi::BS_Front;
use super::bsi::BS_SortNumericDownAlt;
use super::bsi::BS_Dice6;
use super::bsi::BS_RocketTakeoffFill;
use super::bsi::BS_Folder2;
use super::bsi::BS_WindowDesktop;
use super::bsi::BS_FileMinus;
use super::bsi::BS_Mailbox;
use super::bsi::BS_ThermometerHalf;
use super::bsi::BS_PersonCheckFill;
use super::bsi::BS_PlusCircleFill;
use super::bsi::BS_Flower2;
use super::bsi::BS_ArrowsExpand;
use super::bsi::BS_ShieldFillPlus;
use super::bsi::BS_Flower3;
use super::bsi::BS_FiletypeMov;
use super::bsi::BS_PentagonFill;
use super::bsi::BS_CloudDownloadFill;
use super::bsi::BS_Shift;
use super::bsi::BS_UsbMini;
use super::bsi::BS_FileEarmarkCheck;
use super::bsi::BS_4CircleFill;
use super::bsi::BS_NodeMinusFill;
use super::bsi::BS_Check2All;
use super::bsi::BS_SignStop;
use super::bsi::BS_Type;
use super::bsi::BS_LayoutSidebar;
use super::bsi::BS_FileArrowDown;
use super::bsi::BS_BagDash;
use super::bsi::BS_CpuFill;
use super::bsi::BS_PCircle;
use super::bsi::BS_RouterFill;
use super::bsi::BS_BalloonHeartFill;
use super::bsi::BS_FileEarmarkEaselFill;
use super::bsi::BS_Clipboard2Data;
use super::bsi::BS_PersonBoundingBox;
use super::bsi::BS_Kanban;
use super::bsi::BS_BoxArrowInDownLeft;
use super::bsi::BS_TelephoneFill;
use super::bsi::BS_PersonExclamation;
use super::bsi::BS_FileCheck;
use super::bsi::BS_HouseAdd;
use super::bsi::BS_FileEaselFill;
use super::bsi::BS_Shop;
use super::bsi::BS_ArrowCounterclockwise;
use super::bsi::BS_Bullseye;
use super::bsi::BS_GlobeAsiaAustralia;
use super::bsi::BS_PlusSlashMinus;
use super::bsi::BS_PassFill;
use super::bsi::BS_EnvelopeCheck;
use super::bsi::BS_Wind;
use super::bsi::BS_5CircleFill;
use super::bsi::BS_Geo;
use super::bsi::BS_PciCard;
use super::bsi::BS_Calendar4Event;
use super::bsi::BS_MusicPlayerFill;
use super::bsi::BS_FiletypeJava;
use super::bsi::BS_FiletypeCss;
use super::bsi::BS_FileEarmarkDiff;
use super::bsi::BS_BuildingFillCheck;
use super::bsi::BS_EnvelopeOpen;
use super::bsi::BS_PersonFillUp;
use super::bsi::BS_LaptopFill;
use super::bsi::BS_DeviceSsdFill;
use super::bsi::BS_PersonVideo;
use super::bsi::BS_SkipStartFill;
use super::bsi::BS_Camera;
use super::bsi::BS_Ubuntu;
use super::bsi::BS_CloudUploadFill;
use super::bsi::BS_DatabaseUp;
use super::bsi::BS_Fire;
use super::bsi::BS_BorderInner;
use super::bsi::BS_BadgeWc;
use super::bsi::BS_ExclamationSquareFill;
use super::bsi::BS_BrightnessAltHighFill;
use super::bsi::BS_Bandaid;
use super::bsi::BS_Vr;
use super::bsi::BS_PlusLg;
use super::bsi::BS_Webcam;
use super::bsi::BS_PersonFill;
use super::bsi::BS_PiggyBankFill;
use super::bsi::BS_SignTurnRight;
use super::bsi::BS_Calendar2CheckFill;
use super::bsi::BS_BoxArrowInUpRight;
use super::bsi::BS_BoundingBoxCircles;
use super::bsi::BS_SendX;
use super::bsi::BS_ChevronBarLeft;
use super::bsi::BS_BarChart;
use super::bsi::BS_DashSquare;
use super::bsi::BS_9Square;
use super::bsi::BS_PhoneVibrate;
use super::bsi::BS_Outlet;
use super::bsi::BS_LightningCharge;
use super::bsi::BS_ArrowUpRight;
use super::bsi::BS_HandThumbsDownFill;
use super::bsi::BS_Dice3;
use super::bsi::BS_GripVertical;
use super::bsi::BS_Strava;
use super::bsi::BS_EnvelopeOpenHeartFill;
use super::bsi::BS_FiletypePhp;
use super::bsi::BS_WebcamFill;
use super::bsi::BS_SunriseFill;
use super::bsi::BS_EmojiNeutralFill;
use super::bsi::BS_Subtract;
use super::bsi::BS_Briefcase;
use super::bsi::BS_BrowserFirefox;
use super::bsi::BS_Building;
use super::bsi::BS_SignYieldFill;
use super::bsi::BS_CaretUpSquareFill;
use super::bsi::BS_FolderPlus;
use super::bsi::BS_Dpad;
use super::bsi::BS_CardText;
use super::bsi::BS_Calendar4Week;
use super::bsi::BS_EmojiDizzyFill;
use super::bsi::BS_PcDisplayHorizontal;
use super::bsi::BS_BoxArrowInUp;
use super::bsi::BS_MenuDown;
use super::bsi::BS_Badge8k;
use super::bsi::BS_ExclamationDiamondFill;
use super::bsi::BS_Reddit;
use super::bsi::BS_PatchExclamationFill;
use super::bsi::BS_EnvelopeHeartFill;
use super::bsi::BS_Earbuds;
use super::bsi::BS_Camera2;
use super::bsi::BS_Grid3x3;
use super::bsi::BS_Toggle2On;
use super::bsi::BS_SignIntersectionSide;
use super::bsi::BS_FiletypeMp3;
use super::bsi::BS_MegaphoneFill;
use super::bsi::BS_PinMap;
use super::bsi::BS_HouseUpFill;
use super::bsi::BS_CardList;
use super::bsi::BS_Motherboard;
use super::bsi::BS_CreditCard;
use super::bsi::BS_123;
use super::bsi::BS_FiletypeXls;
use super::bsi::BS_StopFill;
use super::bsi::BS_Regex;
use super::bsi::BS_Trash3;
use super::bsi::BS_ArrowClockwise;
use super::bsi::BS_Trophy;
use super::bsi::BS_MagnetFill;
use super::bsi::BS_Cone;
use super::bsi::BS_ViewList;
use super::bsi::BS_TypeUnderline;
use super::bsi::BS_InfoLg;
use super::bsi::BS_CheckCircleFill;
use super::bsi::BS_SendFill;
use super::bsi::BS_FileEarmarkTextFill;
use super::bsi::BS_Diagram2Fill;
use super::bsi::BS_Alipay;
use super::bsi::BS_ArrowLeftCircle;
use super::bsi::BS_TicketDetailedFill;
use super::bsi::BS_FiletypeDocx;
use super::bsi::BS_PhoneLandscapeFill;
use super::bsi::BS_ChevronCompactUp;
use super::bsi::BS_Shield;
use super::bsi::BS_CaretLeftSquareFill;
use super::bsi::BS_Wechat;
use super::bsi::BS_ArrowThroughHeartFill;
use super::bsi::BS_QrCodeScan;
use super::bsi::BS_SendCheckFill;
use super::bsi::BS_ChatRightFill;
use super::bsi::BS_Link45deg;
use super::bsi::BS_FileImage;
use super::bsi::BS_CheckCircle;
use super::bsi::BS_FiletypeScss;
use super::bsi::BS_HouseExclamation;
use super::bsi::BS_EmojiKissFill;
use super::bsi::BS_Stars;
use super::bsi::BS_ShieldFillMinus;
use super::bsi::BS_CameraReelsFill;
use super::bsi::BS_MenuApp;
use super::bsi::BS_ArrowUpSquareFill;
use super::bsi::BS_Clipboard2PlusFill;
use super::bsi::BS_ChatRightQuote;
use super::bsi::BS_BrightnessHigh;
use super::bsi::BS_Signpost;
use super::bsi::BS_FiletypeXml;
use super::bsi::BS_ExclamationLg;
use super::bsi::BS_PersonFillExclamation;
use super::bsi::BS_VolumeOffFill;
use super::bsi::BS_Send;
use super::bsi::BS_FileBinary;
use super::bsi::BS_HouseDownFill;
use super::bsi::BS_SkipBackwardCircleFill;
use super::bsi::BS_SkipForwardBtnFill;
use super::bsi::BS_HouseAddFill;
use super::bsi::BS_CloudFill;
use super::bsi::BS_8CircleFill;
use super::bsi::BS_Clipboard2Pulse;
use super::bsi::BS_Calendar3EventFill;
use super::bsi::BS_FileText;
use super::bsi::BS_Tools;
use super::bsi::BS_CloudHaze;
use super::bsi::BS_FiletypeXlsx;
use super::bsi::BS_CodeSlash;
use super::bsi::BS_Grid1x2Fill;
use super::bsi::BS_Upload;
use super::bsi::BS_FileArrowDownFill;
use super::bsi::BS_PSquare;
use super::bsi::BS_ZoomOut;
use super::bsi::BS_ArrowBarRight;
use super::bsi::BS_7Square;
use super::bsi::BS_QuestionOctagonFill;
use super::bsi::BS_BuildingDown;
use super::bsi::BS_BlockquoteLeft;
use super::bsi::BS_WindowSidebar;
use super::bsi::BS_SignDoNotEnter;
use super::bsi::BS_FileEarmarkPerson;
use super::bsi::BS_Displayport;
use super::bsi::BS_EnvelopePaperHeartFill;
use super::bsi::BS_Trash2Fill;
use super::bsi::BS_HddRackFill;
use super::bsi::BS_ExclamationTriangleFill;
use super::bsi::BS_JustifyRight;
use super::bsi::BS_Airplane;
use super::bsi::BS_Dice4;
use super::bsi::BS_UsbSymbol;
use super::bsi::BS_FileBarGraph;
use super::bsi::BS_TriangleFill;
use super::bsi::BS_FileMusicFill;
use super::bsi::BS_Instagram;
use super::bsi::BS_DeviceHdd;
use super::bsi::BS_FunnelFill;
use super::bsi::BS_FiletypeSass;
use super::bsi::BS_Escape;
use super::bsi::BS_CurrencyDollar;
use super::bsi::BS_RSquareFill;
use super::bsi::BS_CloudSlashFill;
use super::bsi::BS_HandThumbsUp;
use super::bsi::BS_Bricks;
use super::bsi::BS_CardHeading;
use super::bsi::BS_ClipboardCheckFill;
use super::bsi::BS_Hdmi;
use super::bsi::BS_PatchPlus;
use super::bsi::BS_DashCircleDotted;
use super::bsi::BS_WrenchAdjustable;
use super::bsi::BS_Mouse2Fill;
use super::bsi::BS_Wordpress;
use super::bsi::BS_PaintBucket;
use super::bsi::BS_ChatRight;
use super::bsi::BS_ShiftFill;
use super::bsi::BS_TelephoneForward;
use super::bsi::BS_CassetteFill;
use super::bsi::BS_Slash;
use super::bsi::BS_FiletypeBmp;
use super::bsi::BS_BuildingGear;
use super::bsi::BS_CloudsFill;
use super::bsi::BS_SignRailroadFill;
use super::bsi::BS_Grid3x3GapFill;
use super::bsi::BS_KeyboardFill;
use super::bsi::BS_ArrowDownLeftCircleFill;
use super::bsi::BS_TerminalPlus;
use super::bsi::BS_UiRadios;
use super::bsi::BS_ArrowUp;
use super::bsi::BS_ArrowUpSquare;
use super::bsi::BS_Android;
use super::bsi::BS_Water;
use super::bsi::BS_InputCursorText;
use super::bsi::BS_FileEarmarkLockFill;
use super::bsi::BS_HospitalFill;
use super::bsi::BS_SortUpAlt;
use super::bsi::BS_SimFill;
use super::bsi::BS_CloudSun;
use super::bsi::BS_Fingerprint;
use super::bsi::BS_WifiOff;
use super::bsi::BS_ArrowDown;
use super::bsi::BS_FileRuled;
use super::bsi::BS_Save2;
use super::bsi::BS_VolumeOff;
use super::bsi::BS_ClipboardPulse;
use super::bsi::BS_QuestionLg;
use super::bsi::BS_FileEarmarkMusic;
use super::bsi::BS_1Square;
use super::bsi::BS_AlignBottom;
use super::bsi::BS_LungsFill;
use super::bsi::BS_Cassette;
use super::bsi::BS_ArrowUpRightCircleFill;
use super::bsi::BS_FiletypeJsx;
use super::bsi::BS_TelephonePlusFill;
use super::bsi::BS_MoonStars;
use super::bsi::BS_Diamond;
use super::bsi::BS_Bank2;
use super::bsi::BS_ArrowDownRight;
use super::bsi::BS_PostcardHeartFill;
use super::bsi::BS_EggFill;
use super::bsi::BS_RSquare;
use super::bsi::BS_RssFill;
use super::bsi::BS_ShieldLockFill;
use super::bsi::BS_BadgeVo;
use super::bsi::BS_Mastodon;
use super::bsi::BS_TextareaResize;
use super::bsi::BS_Battery;
use super::bsi::BS_UpcScan;
use super::bsi::BS_CreditCard2Back;
use super::bsi::BS_EmojiKiss;
use super::bsi::BS_Cloudy;
use super::bsi::BS_VolumeDown;
use super::bsi::BS_Easel3;
use super::bsi::BS_FileEarmarkPdfFill;
use super::bsi::BS_EnvelopePaperHeart;
use super::bsi::BS_HandIndex;
use super::bsi::BS_FiletypeMdx;
use super::bsi::BS_FuelPump;
use super::bsi::BS_LampFill;
use super::bsi::BS_LightbulbFill;
use super::bsi::BS_SendXFill;
use super::bsi::BS_AirplaneFill;
use super::bsi::BS_Bell;
use super::bsi::BS_BoxArrowUpLeft;
use super::bsi::BS_SignMergeLeftFill;
use super::bsi::BS_ArrowDownRightSquareFill;
use super::bsi::BS_ShieldFill;
use super::bsi::BS_BadgeVoFill;
use super::bsi::BS_ShieldShaded;
use super::bsi::BS_Plug;
use super::bsi::BS_SuitSpade;
use super::bsi::BS_TypeStrikethrough;
use super::bsi::BS_CameraReels;
use super::bsi::BS_SaveFill;
use super::bsi::BS_CalendarHeartFill;
use super::bsi::BS_PencilFill;
use super::bsi::BS_CloudSleet;
use super::bsi::BS_TaxiFront;
use super::bsi::BS_CashCoin;
use super::bsi::BS_FolderFill;
use super::bsi::BS_CollectionPlay;
use super::bsi::BS_Dropbox;
use super::bsi::BS_SignNoParking;
use super::bsi::BS_BadgeTmFill;
use super::bsi::BS_Record2Fill;
use super::bsi::BS_CircleFill;
use super::bsi::BS_BagDashFill;
use super::bsi::BS_ArrowLeftSquare;
use super::bsi::BS_Calculator;
use super::bsi::BS_FileEarmark;
use super::bsi::BS_CalendarPlusFill;
use super::bsi::BS_EmojiHeartEyesFill;
use super::bsi::BS_1SquareFill;
use super::bsi::BS_PlayCircle;
use super::bsi::BS_Capsule;
use super::bsi::BS_DatabaseAdd;
use super::bsi::BS_LayoutWtf;
use super::bsi::BS_PostageHeartFill;
use super::bsi::BS_DashSquareFill;
use super::bsi::BS_AlignStart;
use super::bsi::BS_EmojiSmileUpsideDownFill;
use super::bsi::BS_ChevronBarExpand;
use super::bsi::BS_GeoAltFill;
use super::bsi::BS_HouseDashFill;
use super::bsi::BS_SendDashFill;
use super::bsi::BS_Exclude;
use super::bsi::BS_EmojiExpressionlessFill;
use super::bsi::BS_Calendar2Plus;
use super::bsi::BS_Newspaper;
use super::bsi::BS_CalendarWeek;
use super::bsi::BS_UsbPlug;
use super::bsi::BS_ExclamationOctagon;
use super::bsi::BS_CashStack;
use super::bsi::BS_BoxArrowInUpLeft;
use super::bsi::BS_ReplyAllFill;
use super::bsi::BS_PauseBtnFill;
use super::bsi::BS_FileEarmarkSlides;
use super::bsi::BS_InfoSquareFill;
use super::bsi::BS_Calendar2MonthFill;
use super::bsi::BS_House;
use super::bsi::BS_JournalArrowUp;
use super::bsi::BS_Reception2;
use super::bsi::BS_FiletypeCs;
use super::bsi::BS_BuildingFillX;
use super::bsi::BS_Octagon;
use super::bsi::BS_CloudArrowDown;
use super::bsi::BS_Cloud;
use super::bsi::BS_ShieldSlashFill;
use super::bsi::BS_GiftFill;
use super::bsi::BS_DiscFill;
use super::bsi::BS_ColumnsGap;
use super::bsi::BS_SignMergeRight;
use super::bsi::BS_EmojiFrown;
use super::bsi::BS_HouseDoorFill;
use super::bsi::BS_BellSlash;
use super::bsi::BS_DatabaseLock;
use super::bsi::BS_OpticalAudioFill;
use super::bsi::BS_CloudLightningRain;
use super::bsi::BS_LayoutSidebarReverse;
use super::bsi::BS_WindowDock;
use super::bsi::BS_FileEarmarkMedical;
use super::bsi::BS_CSquareFill;
use super::bsi::BS_ExclamationCircleFill;
use super::bsi::BS_LayoutThreeColumns;
use super::bsi::BS_FileEarmarkExcel;
use super::bsi::BS_Quora;
use super::bsi::BS_FiletypeRb;
use super::bsi::BS_Balloon;
use super::bsi::BS_BatteryFull;
use super::bsi::BS_Paperclip;
use super::bsi::BS_Clipboard;
use super::bsi::BS_ArrowDownUp;
use super::bsi::BS_ChatLeftText;
use super::bsi::BS_CapslockFill;
use super::bsi::BS_Cart3;
use super::bsi::BS_Modem;
use super::bsi::BS_MusicPlayer;
use super::bsi::BS_SignTurnSlightLeftFill;
use super::bsi::BS_AirplaneEnginesFill;
use super::bsi::BS_BuildingSlash;
use super::bsi::BS_EmojiLaughingFill;
use super::bsi::BS_Projector;
use super::bsi::BS_FlagFill;
use super::bsi::BS_BodyText;
use super::bsi::BS_Magic;
use super::bsi::BS_Box;
use super::bsi::BS_Indent;
use super::bsi::BS_PeopleFill;
use super::bsi::BS_FileEarmarkSlidesFill;
use super::bsi::BS_FileBarGraphFill;
use super::bsi::BS_Funnel;
use super::bsi::BS_HouseDash;
use super::bsi::BS_Stoplights;
use super::bsi::BS_StopCircleFill;
use super::bsi::BS_KeyFill;
use super::bsi::BS_TrainLightrailFrontFill;
use super::bsi::BS_Steam;
use super::bsi::BS_Bezier2;
use super::bsi::BS_CameraVideoFill;
use super::bsi::BS_GearFill;
use super::bsi::BS_CupHotFill;
use super::bsi::BS_ListColumns;
use super::bsi::BS_XDiamondFill;
use super::bsi::BS_SortAlphaDown;
use super::bsi::BS_FileLockFill;
use super::bsi::BS_Justify;
use super::bsi::BS_Pip;
use super::bsi::BS_Layers;
use super::bsi::BS_BuildingFillAdd;
use super::bsi::BS_BoxArrowDownRight;
use super::bsi::BS_BorderCenter;
use super::bsi::BS_Diagram3;
use super::bsi::BS_DeviceHddFill;
use super::bsi::BS_BrightnessAltLow;
use super::bsi::BS_Chat;
use super::bsi::BS_TelephoneOutboundFill;
use super::bsi::BS_CaretRightSquareFill;
use super::bsi::BS_BroadcastPin;
use super::bsi::BS_XCircleFill;
use super::bsi::BS_SkipEndBtnFill;
use super::bsi::BS_Lungs;
use super::bsi::BS_Calendar2PlusFill;
use super::bsi::BS_EmojiSunglasses;
use super::bsi::BS_Clipboard2CheckFill;
use super::bsi::BS_Voicemail;
use super::bsi::BS_ArrowUpLeftSquareFill;
use super::bsi::BS_PatchExclamation;
use super::bsi::BS_Easel2Fill;
use super::bsi::BS_FiletypeKey;
use super::bsi::BS_FiletypeSvg;
use super::bsi::BS_SearchHeartFill;
use super::bsi::BS_FileEarmarkArrowUpFill;
use super::bsi::BS_CircleHalf;
use super::bsi::BS_AlignTop;
use super::bsi::BS_ArrowsCollapse;
use super::bsi::BS_BuildingFillSlash;
use super::bsi::BS_SlashLg;
use super::bsi::BS_EnvelopeCheckFill;
use super::bsi::BS_FileEarmarkPlayFill;
use super::bsi::BS_BadgeAd;
use super::bsi::BS_FileEarmarkPersonFill;
use super::bsi::BS_HouseGearFill;
use super::bsi::BS_Git;
use super::bsi::BS_BellFill;
use super::bsi::BS_Activity;
use super::bsi::BS_ChatLeftDots;
use super::bsi::BS_ShieldLock;
use super::bsi::BS_Capslock;
use super::bsi::BS_ArrowUpRightSquare;
use super::bsi::BS_NodePlus;
use super::bsi::BS_2Square;
use super::bsi::BS_CaretRight;
use super::bsi::BS_BuildingLock;
use super::bsi::BS_UsbFill;
use super::bsi::BS_PostageHeart;
use super::bsi::BS_Coin;
use super::bsi::BS_ChevronDoubleUp;
use super::bsi::BS_Grid3x3Gap;
use super::bsi::BS_Clouds;
use super::bsi::BS_Folder2Open;
use super::bsi::BS_FilterSquare;
use super::bsi::BS_Hexagon;
use super::bsi::BS_HeptagonHalf;
use super::bsi::BS_DashCircleFill;
use super::bsi::BS_FileEasel;
use super::bsi::BS_Grid1x2;
use super::bsi::BS_Hdd;
use super::bsi::BS_QuestionOctagon;
use super::bsi::BS_SendDash;
use super::bsi::BS_BagPlusFill;
use super::bsi::BS_FileEarmarkBarGraph;
use super::bsi::BS_UiChecks;
use super::bsi::BS_FileSpreadsheet;
use super::bsi::BS_Bug;
use super::bsi::BS_ArrowRepeat;
use super::bsi::BS_Dice5;
use super::bsi::BS_3Circle;
use super::bsi::BS_PauseBtn;
use super::bsi::BS_Images;
use super::bsi::BS_SignNoLeftTurnFill;
use super::bsi::BS_CalendarX;
use super::bsi::BS_FiletypePng;
use super::bsi::BS_Clock;
use super::bsi::BS_AirplaneEngines;
use super::bsi::BS_EmojiHeartEyes;
use super::bsi::BS_Book;
use super::bsi::BS_Sliders2Vertical;
use super::bsi::BS_CreditCard2Front;
use super::bsi::BS_FileEarmarkWord;
use super::bsi::BS_FiletypeCsv;
use super::bsi::BS_BarChartLineFill;
use super::bsi::BS_Hurricane;
use super::bsi::BS_Envelope;
use super::bsi::BS_OctagonFill;
use super::bsi::BS_EnvelopeSlash;
use super::bsi::BS_FileEarmarkSpreadsheetFill;
use super::bsi::BS_BoxArrowDown;
use super::bsi::BS_BuildingFillUp;
use super::bsi::BS_CloudSlash;
use super::bsi::BS_ClipboardXFill;
use super::bsi::BS_ArrowLeftRight;
use super::bsi::BS_Square;
use super::bsi::BS_FilterLeft;
use super::bsi::BS_Ladder;
use super::bsi::BS_FileCodeFill;
use super::bsi::BS_Badge4kFill;
use super::bsi::BS_Umbrella;
use super::bsi::BS_ArrowDownLeftCircle;
use super::bsi::BS_BracesAsterisk;
use super::bsi::BS_BrowserEdge;
use super::bsi::BS_Pen;
use super::bsi::BS_BootstrapReboot;
use super::bsi::BS_Smartwatch;
use super::bsi::BS_Peace;
use super::bsi::BS_PersonBadge;
use super::bsi::BS_XOctagonFill;
use super::bsi::BS_CreditCard2BackFill;
use super::bsi::BS_BorderWidth;
use super::bsi::BS_ToggleOff;
use super::bsi::BS_Basket2Fill;
use super::bsi::BS_PersonXFill;
use super::bsi::BS_CloudDrizzleFill;
use super::bsi::BS_Wallet2;
use super::bsi::BS_Forward;
use super::bsi::BS_Android2;
use super::bsi::BS_HCircle;
use super::bsi::BS_CupFill;
use super::bsi::BS_CalendarRange;
use super::bsi::BS_SignpostSplitFill;
use super::bsi::BS_EmojiNeutral;
use super::bsi::BS_DashSquareDotted;
use super::bsi::BS_PersonCircle;
use super::bsi::BS_SkipEnd;
use super::bsi::BS_EjectFill;
use super::bsi::BS_FolderCheck;
use super::bsi::BS_FileBreakFill;
use super::bsi::BS_SignTurnSlightRight;
use super::bsi::BS_Vinyl;
use super::bsi::BS_Calendar2X;
use super::bsi::BS_CollectionFill;
use super::bsi::BS_Moisture;
use super::bsi::BS_SlashSquare;
use super::bsi::BS_Image;
use super::bsi::BS_Clipboard2Heart;
use super::bsi::BS_Rainbow;
use super::bsi::BS_Valentine;
use super::bsi::BS_HouseLock;
use super::bsi::BS_ExclamationDiamond;
use super::bsi::BS_MenuButton;
use super::bsi::BS_2CircleFill;
use super::bsi::BS_EnvelopeXFill;
use super::bsi::BS_Backspace;
use super::bsi::BS_BadgeArFill;
use super::bsi::BS_ProjectorFill;
use super::bsi::BS_ArrowUpLeftSquare;
use super::bsi::BS_SkipBackwardBtn;
use super::bsi::BS_FilePlusFill;
use super::bsi::BS_AlignMiddle;
use super::bsi::BS_PinAngle;
use super::bsi::BS_Calendar2Fill;
use super::bsi::BS_Sunrise;
use super::bsi::BS_Badge3d;
use super::bsi::BS_BagFill;
use super::bsi::BS_Safe;
use super::bsi::BS_7Circle;
use super::bsi::BS_Clipboard2MinusFill;
use super::bsi::BS_FileEarmarkCode;
use super::bsi::BS_FilePpt;
use super::bsi::BS_FiletypeJson;
use super::bsi::BS_Calendar2Week;
use super::bsi::BS_Unity;
use super::bsi::BS_WindowFullscreen;
use super::bsi::BS_TrainFreightFront;
use super::bsi::BS_FileEarmarkRichtextFill;
use super::bsi::BS_BadgeTm;
use super::bsi::BS_Back;
use super::bsi::BS_SkipStart;
use super::bsi::BS_ChatQuoteFill;
use super::bsi::BS_FastForwardCircleFill;
use super::bsi::BS_BucketFill;
use super::bsi::BS_SdCardFill;
use super::bsi::BS_Playstation;
use super::bsi::BS_FileEarmarkMedicalFill;
use super::bsi::BS_LockFill;
use super::bsi::BS_Calendar2Heart;
use super::bsi::BS_XSquareFill;
use super::bsi::BS_Pin;
use super::bsi::BS_Folder;
use super::bsi::BS_ArrowUpShort;
use super::bsi::BS_Calendar2Minus;
use super::bsi::BS_FileArrowUp;
use super::bsi::BS_Line;
use super::bsi::BS_HdmiFill;
use super::bsi::BS_Tiktok;
use super::bsi::BS_SquareFill;
use super::bsi::BS_Heptagon;
use super::bsi::BS_QuestionSquareFill;
use super::bsi::BS_DpadFill;
use super::bsi::BS_BoxFill;
use super::bsi::BS_UsbMiniFill;
use super::bsi::BS_CartDashFill;
use super::bsi::BS_PersonFillX;
use super::bsi::BS_Trash;
use super::bsi::BS_QuestionDiamond;
use super::bsi::BS_Egg;
use super::bsi::BS_Headset;
use super::bsi::BS_CurrencyEuro;
use super::bsi::BS_JournalCheck;
use super::bsi::BS_Calendar2Month;
use super::bsi::BS_BoxArrowUpRight;
use super::bsi::BS_SignTurnLeftFill;
use super::bsi::BS_FileEarmarkFont;
use super::bsi::BS_ShieldSlash;
use super::bsi::BS_SkipForwardCircle;
use super::bsi::BS_Watch;
use super::bsi::BS_ArrowUpLeft;
use super::bsi::BS_Calendar2;
use super::bsi::BS_EnvelopeFill;
use super::bsi::BS_Skype;
use super::bsi::BS_RCircle;
use super::bsi::BS_Calendar2EventFill;
use super::bsi::BS_PostageFill;
use super::bsi::BS_UsbCFill;
use super::bsi::BS_SkipBackwardBtnFill;
use super::bsi::BS_RecordCircle;
use super::bsi::BS_BuildingFillDown;
use super::bsi::BS_GenderMale;
use super::bsi::BS_Hr;
use super::bsi::BS_CloudArrowUpFill;
use super::bsi::BS_Meta;
use super::bsi::BS_PieChartFill;
use super::bsi::BS_RCircleFill;
use super::bsi::BS_TaxiFrontFill;
use super::bsi::BS_StarFill;
use super::bsi::BS_Calendar4;
use super::bsi::BS_SignIntersection;
use super::bsi::BS_CaretRightSquare;
use super::bsi::BS_PatchQuestionFill;
use super::bsi::BS_X;
use super::bsi::BS_Record;
use super::bsi::BS_CloudMoonFill;
use super::bsi::BS_HddNetworkFill;
use super::bsi::BS_TrainFrontFill;
use super::bsi::BS_ListCheck;
use super::bsi::BS_CloudRain;
use super::bsi::BS_BuildingExclamation;
use super::bsi::BS_PlayFill;
use super::bsi::BS_SignDeadEndFill;
use super::bsi::BS_ShieldMinus;
use super::bsi::BS_HddRack;
use super::bsi::BS_CloudArrowDownFill;
use super::bsi::BS_DatabaseX;
use super::bsi::BS_CloudPlusFill;
use super::bsi::BS_SortNumericUp;
use super::bsi::BS_Archive;
use super::bsi::BS_BorderAll;
use super::bsi::BS_CalendarDateFill;
use super::bsi::BS_PhoneFlip;
use super::bsi::BS_PersonUp;
use super::bsi::BS_CartXFill;
use super::bsi::BS_PaletteFill;
use super::bsi::BS_Snow;
use super::bsi::BS_SuitDiamond;
use super::bsi::BS_CalendarEvent;
use super::bsi::BS_Grid;
use super::bsi::BS_PcHorizontal;
use super::bsi::BS_XDiamond;
use super::bsi::BS_Speedometer2;
use super::bsi::BS_SignIntersectionFill;
use super::bsi::BS_Hash;
use super::bsi::BS_ExclamationSquare;
use super::bsi::BS_EmojiSmile;
use super::bsi::BS_SkipStartCircleFill;
use super::bsi::BS_DoorClosedFill;
use super::bsi::BS_ChevronCompactRight;
use super::bsi::BS_CurrencyPound;
use super::bsi::BS_FileFill;
use super::bsi::BS_Boombox;
use super::bsi::BS_TypeItalic;
use super::bsi::BS_Toggles2;
use super::bsi::BS_BriefcaseFill;
use super::bsi::BS_ModemFill;
use super::bsi::BS_Calendar;
use super::bsi::BS_LayersHalf;
use super::bsi::BS_WalletFill;
use super::bsi::BS_SendPlusFill;
use super::bsi::BS_BorderRight;
use super::bsi::BS_PersonFillAdd;
use super::bsi::BS_GridFill;
use super::bsi::BS_PencilSquare;
use super::bsi::BS_BadgeHd;
use super::bsi::BS_ChatLeft;
use super::bsi::BS_8Circle;
use super::bsi::BS_FileCheckFill;
use super::bsi::BS_Tags;
use super::bsi::BS_RecordFill;
use super::bsi::BS_Cart4;
use super::bsi::BS_CheckLg;
use super::bsi::BS_ChatFill;
use super::bsi::BS_CalendarMinusFill;
use super::bsi::BS_ChevronBarContract;
use super::bsi::BS_AlarmFill;
use super::bsi::BS_CloudDownload;
use super::bsi::BS_Usb;
use super::bsi::BS_Diagram3Fill;
use super::bsi::BS_CcCircle;
use super::bsi::BS_TextareaT;
use super::bsi::BS_CalculatorFill;
use super::bsi::BS_BusFront;
use super::bsi::BS_Sun;
use super::bsi::BS_QuestionCircle;
use super::bsi::BS_CloudPlus;
use super::bsi::BS_SearchHeart;
use super::bsi::BS_HandIndexThumbFill;
use super::bsi::BS_Inboxes;
use super::bsi::BS_SquareHalf;
use super::bsi::BS_MenuButtonWideFill;
use super::bsi::BS_DoorOpen;
use super::bsi::BS_Ticket;
use super::bsi::BS_CurrencyYen;
use super::bsi::BS_Stack;
use super::bsi::BS_Download;
use super::bsi::BS_Clipboard2X;
use super::bsi::BS_MarkdownFill;
use super::bsi::BS_ForwardFill;
use super::bsi::BS_Binoculars;
use super::bsi::BS_Play;
use super::bsi::BS_Clipboard2Check;
use super::bsi::BS_StopBtn;
use super::bsi::BS_PlusSquare;
use super::bsi::BS_GpuCard;
use super::bsi::BS_LightningFill;
use super::bsi::BS_ReplyAll;
use super::bsi::BS_UiChecksGrid;
use super::bsi::BS_ClipboardDataFill;
use super::bsi::BS_RecordBtn;
use super::bsi::BS_8Square;
use super::bsi::BS_ShareFill;
use super::bsi::BS_SortNumericUpAlt;
use super::bsi::BS_ChatSquareQuoteFill;
use super::bsi::BS_BusFrontFill;
use super::bsi::BS_Mouse2;
use super::bsi::BS_TicketPerforated;
use super::bsi::BS_ChatLeftHeart;
use super::bsi::BS_RewindCircleFill;
use super::bsi::BS_Calendar3Range;
use super::bsi::BS_AppIndicator;
use super::bsi::BS_Alarm;
use super::bsi::BS_CalendarFill;
use super::bsi::BS_WindowPlus;
use super::bsi::BS_ChatSquare;
use super::bsi::BS_Github;
use super::bsi::BS_HandThumbsDown;
use super::bsi::BS_Bookmarks;
use super::bsi::BS_Flag;
use super::bsi::BS_ShieldFillExclamation;
use super::bsi::BS_PiggyBank;
use super::bsi::BS_RecordBtnFill;
use super::bsi::BS_DeviceSsd;
use super::bsi::BS_PersonFillGear;
use super::bsi::BS_SignStopLights;
use super::bsi::BS_0Square;
use super::bsi::BS_EnvelopeDash;
use super::bsi::BS_List;
use super::bsi::BS_ListColumnsReverse;
use super::bsi::BS_Command;
use super::bsi::BS_EmojiFrownFill;
use super::bsi::BS_ChevronBarRight;
use super::bsi::BS_HouseSlash;
use super::bsi::BS_ClipboardFill;
use super::bsi::BS_InputCursor;
use super::bsi::BS_ArrowsFullscreen;
use super::bsi::BS_SkipEndCircle;
use super::bsi::BS_Cast;
use super::bsi::BS_TencentQq;
use super::bsi::BS_Calendar2Date;
use super::bsi::BS_CcSquareFill;
use super::bsi::BS_TelephoneInbound;
use super::bsi::BS_HexagonHalf;
use super::bsi::BS_Megaphone;
use super::bsi::BS_ChatSquareFill;
use super::bsi::BS_FiletypeYml;
use super::bsi::BS_Dice5Fill;
use super::bsi::BS_PatchCheckFill;
use super::bsi::BS_BoxSeam;
use super::bsi::BS_AlignEnd;
use super::bsi::BS_BadgeSd;
use super::bsi::BS_InboxesFill;
use super::bsi::BS_CameraVideoOff;
use super::bsi::BS_ClipboardMinus;
use super::bsi::BS_CloudCheck;
use super::bsi::BS_FastForward;
use super::bsi::BS_JournalCode;
use super::bsi::BS_CcCircleFill;
use super::bsi::BS_TelephoneXFill;
use super::bsi::BS_Wallet;
use super::bsi::BS_ChevronCompactDown;
use super::bsi::BS_Printer;
use super::bsi::BS_PersonWorkspace;
use super::bsi::BS_Calendar2DayFill;
use super::bsi::BS_CarFrontFill;
use super::bsi::BS_Stickies;
use super::bsi::BS_TextWrap;
use super::bsi::BS_ThunderboltFill;
use super::bsi::BS_BoxArrowInDownRight;
use super::bsi::BS_HouseX;
use super::bsi::BS_FilterCircleFill;
use super::bsi::BS_JustifyLeft;
use super::bsi::BS_WindowStack;
use super::bsi::BS_FastForwardBtnFill;
use super::bsi::BS_FileBinaryFill;
use super::bsi::BS_SignNoRightTurn;
use super::bsi::BS_Badge8kFill;
use super::bsi::BS_Postcard;
use super::bsi::BS_5Square;
use super::bsi::BS_ListTask;
use super::bsi::BS_ClipboardPlus;
use super::bsi::BS_JournalArrowDown;
use super::bsi::BS_EyeSlash;
use super::bsi::BS_CloudLightning;
use super::bsi::BS_FileEarmarkPost;
use super::bsi::BS_BagCheckFill;
use super::bsi::BS_Spotify;
use super::bsi::BS_Superscript;
use super::bsi::BS_HeptagonFill;
use super::bsi::BS_HSquareFill;
use super::bsi::BS_AlignCenter;
use super::bsi::BS_PatchMinus;
use super::bsi::BS_Mask;
use super::bsi::BS_CloudUpload;
use super::bsi::BS_Snow3;
use super::bsi::BS_MinecartLoaded;
use super::bsi::BS_CloudMinus;
use super::bsi::BS_ThermometerHigh;
use super::bsi::BS_TerminalDash;
use super::bsi::BS_ThreeDots;
use super::bsi::BS_ArrowBarDown;
use super::bsi::BS_Thunderbolt;
use super::bsi::BS_ThermometerSnow;
use super::bsi::BS_Microsoft;
use super::bsi::BS_TelephoneForwardFill;
use super::bsi::BS_CaretUpFill;
use super::bsi::BS_Lightning;
use super::bsi::BS_RewindCircle;
use super::bsi::BS_Clipboard2;
use super::bsi::BS_DatabaseFillUp;
use super::bsi::BS_CupHot;
use super::bsi::BS_UsbMicro;
use super::bsi::BS_BuildingX;
use super::bsi::BS_CloudLightningRainFill;
use super::bsi::BS_Union;
use super::bsi::BS_Signpost2;


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

#[function_component(BSIcon)]
pub fn icon(props: &IconProps) -> Html {
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
