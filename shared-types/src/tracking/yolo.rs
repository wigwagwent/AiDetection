use deku::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(
    Debug, PartialEq, DekuRead, DekuWrite, Serialize, Deserialize, Eq, PartialOrd, Clone, Copy,
)]
#[deku(type = "usize")]
pub enum YoloClasses80 {
    #[deku(id = "0")]
    Person,
    #[deku(id = "1")]
    Bicycle,
    #[deku(id = "2")]
    Car,
    #[deku(id = "3")]
    Motorcycle,
    #[deku(id = "4")]
    Airplane,
    #[deku(id = "5")]
    Bus,
    #[deku(id = "6")]
    Train,
    #[deku(id = "7")]
    Truck,
    #[deku(id = "8")]
    Boat,
    #[deku(id = "9")]
    TrafficLight,
    #[deku(id = "10")]
    FireHydrant,
    #[deku(id = "11")]
    StopSign,
    #[deku(id = "12")]
    ParkingMeter,
    #[deku(id = "13")]
    Bench,
    #[deku(id = "14")]
    Bird,
    #[deku(id = "15")]
    Cat,
    #[deku(id = "16")]
    Dog,
    #[deku(id = "17")]
    Horse,
    #[deku(id = "18")]
    Sheep,
    #[deku(id = "19")]
    Cow,
    #[deku(id = "20")]
    Elephant,
    #[deku(id = "21")]
    Bear,
    #[deku(id = "22")]
    Zebra,
    #[deku(id = "23")]
    Giraffe,
    #[deku(id = "24")]
    Backpack,
    #[deku(id = "25")]
    Umbrella,
    #[deku(id = "26")]
    Handbag,
    #[deku(id = "27")]
    Tie,
    #[deku(id = "28")]
    Suitcase,
    #[deku(id = "29")]
    Frisbee,
    #[deku(id = "30")]
    Skis,
    #[deku(id = "31")]
    Snowboard,
    #[deku(id = "32")]
    SportsBall,
    #[deku(id = "33")]
    Kite,
    #[deku(id = "34")]
    BaseballBat,
    #[deku(id = "35")]
    BaseballGlove,
    #[deku(id = "36")]
    Skateboard,
    #[deku(id = "37")]
    Surfboard,
    #[deku(id = "38")]
    TennisRacket,
    #[deku(id = "39")]
    Bottle,
    #[deku(id = "40")]
    WineGlass,
    #[deku(id = "41")]
    Cup,
    #[deku(id = "42")]
    Fork,
    #[deku(id = "43")]
    Knife,
    #[deku(id = "44")]
    Spoon,
    #[deku(id = "45")]
    Bowl,
    #[deku(id = "46")]
    Banana,
    #[deku(id = "47")]
    Apple,
    #[deku(id = "48")]
    Sandwich,
    #[deku(id = "49")]
    Orange,
    #[deku(id = "50")]
    Broccoli,
    #[deku(id = "51")]
    Carrot,
    #[deku(id = "52")]
    HotDog,
    #[deku(id = "53")]
    Pizza,
    #[deku(id = "54")]
    Donut,
    #[deku(id = "55")]
    Cake,
    #[deku(id = "56")]
    Chair,
    #[deku(id = "57")]
    Couch,
    #[deku(id = "58")]
    PottedPlant,
    #[deku(id = "59")]
    Bed,
    #[deku(id = "60")]
    DiningTable,
    #[deku(id = "61")]
    Toilet,
    #[deku(id = "62")]
    Tv,
    #[deku(id = "63")]
    Laptop,
    #[deku(id = "64")]
    Mouse,
    #[deku(id = "65")]
    Remote,
    #[deku(id = "66")]
    Keyboard,
    #[deku(id = "67")]
    CellPhone,
    #[deku(id = "68")]
    Microwave,
    #[deku(id = "69")]
    Oven,
    #[deku(id = "70")]
    Toaster,
    #[deku(id = "71")]
    Sink,
    #[deku(id = "72")]
    Refrigerator,
    #[deku(id = "73")]
    Book,
    #[deku(id = "74")]
    Clock,
    #[deku(id = "75")]
    Vase,
    #[deku(id = "76")]
    Scissors,
    #[deku(id = "77")]
    TeddyBear,
    #[deku(id = "78")]
    HairDrier,
    #[deku(id = "79")]
    Toothbrush,
}

#[derive(
    Debug, PartialEq, DekuRead, DekuWrite, Serialize, Deserialize, Eq, PartialOrd, Clone, Copy,
)]
#[deku(type = "usize")]
pub enum YoloClassesOIV7 {
    #[deku(id = "0")]
    Accordion,
    #[deku(id = "1")]
    AdhesiveTape,
    #[deku(id = "2")]
    Aircraft,
    #[deku(id = "3")]
    Airplane,
    #[deku(id = "4")]
    AlarmClock,
    #[deku(id = "5")]
    Alpaca,
    #[deku(id = "6")]
    Ambulance,
    #[deku(id = "7")]
    Animal,
    #[deku(id = "8")]
    Ant,
    #[deku(id = "9")]
    Antelope,
    #[deku(id = "10")]
    Apple,
    #[deku(id = "11")]
    Armadillo,
    #[deku(id = "12")]
    Artichoke,
    #[deku(id = "13")]
    AutoPart,
    #[deku(id = "14")]
    Axe,
    #[deku(id = "15")]
    Backpack,
    #[deku(id = "16")]
    Bagel,
    #[deku(id = "17")]
    BakedGoods,
    #[deku(id = "18")]
    BalanceBeam,
    #[deku(id = "19")]
    Ball,
    #[deku(id = "20")]
    Balloon,
    #[deku(id = "21")]
    Banana,
    #[deku(id = "22")]
    BandAid,
    #[deku(id = "23")]
    Banjo,
    #[deku(id = "24")]
    Barge,
    #[deku(id = "25")]
    Barrel,
    #[deku(id = "26")]
    BaseballBat,
    #[deku(id = "27")]
    BaseballGlove,
    #[deku(id = "28")]
    Bat, // (Animal)
    #[deku(id = "29")]
    BathroomAccessory,
    #[deku(id = "30")]
    BathroomCabinet,
    #[deku(id = "31")]
    Bathtub,
    #[deku(id = "32")]
    Beaker,
    #[deku(id = "33")]
    Bear,
    #[deku(id = "34")]
    Bed,
    #[deku(id = "35")]
    Bee,
    #[deku(id = "36")]
    Beehive,
    #[deku(id = "37")]
    Beer,
    #[deku(id = "38")]
    Beetle,
    #[deku(id = "39")]
    BellPepper,
    #[deku(id = "40")]
    Belt,
    #[deku(id = "41")]
    Bench,
    #[deku(id = "42")]
    Bicycle,
    #[deku(id = "43")]
    BicycleHelmet,
    #[deku(id = "44")]
    BicycleWheel,
    #[deku(id = "45")]
    Bidet,
    #[deku(id = "46")]
    Billboard,
    #[deku(id = "47")]
    BilliardTable,
    #[deku(id = "48")]
    Binoculars,
    #[deku(id = "49")]
    Bird,
    #[deku(id = "50")]
    Blender,
    #[deku(id = "51")]
    BlueJay,
    #[deku(id = "52")]
    Boat,
    #[deku(id = "53")]
    Bomb,
    #[deku(id = "54")]
    Book,
    #[deku(id = "55")]
    Bookcase,
    #[deku(id = "56")]
    Boot,
    #[deku(id = "57")]
    Bottle,
    #[deku(id = "58")]
    BottleOpener,
    #[deku(id = "59")]
    BowAndArrow,
    #[deku(id = "60")]
    Bowl,
    #[deku(id = "61")]
    BowlingEquipment,
    #[deku(id = "62")]
    Box,
    #[deku(id = "63")]
    Boy,
    #[deku(id = "64")]
    Brassiere,
    #[deku(id = "65")]
    Bread,
    #[deku(id = "66")]
    Briefcase,
    #[deku(id = "67")]
    Broccoli,
    #[deku(id = "68")]
    BronzeSculpture,
    #[deku(id = "69")]
    BrownBear,
    #[deku(id = "70")]
    Building,
    #[deku(id = "71")]
    Bull,
    #[deku(id = "72")]
    Burrito,
    #[deku(id = "73")]
    Bus,
    #[deku(id = "74")]
    Bust,
    #[deku(id = "75")]
    Butterfly,
    #[deku(id = "76")]
    Cabbage,
    #[deku(id = "77")]
    Cabinetry,
    #[deku(id = "78")]
    Cake,
    #[deku(id = "79")]
    CakeStand,
    #[deku(id = "80")]
    Calculator,
    #[deku(id = "81")]
    Camel,
    #[deku(id = "82")]
    Camera,
    #[deku(id = "83")]
    CanOpener,
    #[deku(id = "84")]
    Canary,
    #[deku(id = "85")]
    Candle,
    #[deku(id = "86")]
    Candy,
    #[deku(id = "87")]
    Cannon,
    #[deku(id = "88")]
    Canoe,
    #[deku(id = "89")]
    Cantaloupe,
    #[deku(id = "90")]
    Car,
    #[deku(id = "91")]
    Carnivore,
    #[deku(id = "92")]
    Carrot,
    #[deku(id = "93")]
    Cart,
    #[deku(id = "94")]
    CassetteDeck,
    #[deku(id = "95")]
    Castle,
    #[deku(id = "96")]
    Cat,
    #[deku(id = "97")]
    CatFurniture,
    #[deku(id = "98")]
    Caterpillar,
    #[deku(id = "99")]
    Cattle,
    #[deku(id = "100")]
    CeilingFan,
    #[deku(id = "101")]
    Cello,
    #[deku(id = "102")]
    Centipede,
    #[deku(id = "103")]
    Chainsaw,
    #[deku(id = "104")]
    Chair,
    #[deku(id = "105")]
    Cheese,
    #[deku(id = "106")]
    Cheetah,
    #[deku(id = "107")]
    ChestOfDrawers,
    #[deku(id = "108")]
    Chicken,
    #[deku(id = "109")]
    Chime,
    #[deku(id = "110")]
    Chisel,
    #[deku(id = "111")]
    Chopsticks,
    #[deku(id = "112")]
    ChristmasTree,
    #[deku(id = "113")]
    Clock,
    #[deku(id = "114")]
    Closet,
    #[deku(id = "115")]
    Clothing,
    #[deku(id = "116")]
    Coat,
    #[deku(id = "117")]
    Cocktail,
    #[deku(id = "118")]
    CocktailShaker,
    #[deku(id = "119")]
    Coconut,
    #[deku(id = "120")]
    Coffee,
    #[deku(id = "121")]
    CoffeeCup,
    #[deku(id = "122")]
    CoffeeTable,
    #[deku(id = "123")]
    Coffeemaker,
    #[deku(id = "124")]
    Coin,
    #[deku(id = "125")]
    CommonFig,
    #[deku(id = "126")]
    CommonSunflower,
    #[deku(id = "127")]
    ComputerKeyboard,
    #[deku(id = "128")]
    ComputerMonitor,
    #[deku(id = "129")]
    ComputerMouse,
    #[deku(id = "130")]
    Container,
    #[deku(id = "131")]
    ConvenienceStore,
    #[deku(id = "132")]
    Cookie,
    #[deku(id = "133")]
    CookingSpray,
    #[deku(id = "134")]
    CordedPhone,
    #[deku(id = "135")]
    Cosmetics,
    #[deku(id = "136")]
    Couch,
    #[deku(id = "137")]
    Countertop,
    #[deku(id = "138")]
    CowboyHat,
    #[deku(id = "139")]
    Crab,
    #[deku(id = "140")]
    Cream,
    #[deku(id = "141")]
    CricketBall,
    #[deku(id = "142")]
    Crocodile,
    #[deku(id = "143")]
    Croissant,
    #[deku(id = "144")]
    Crown,
    #[deku(id = "145")]
    Crutch,
    #[deku(id = "146")]
    Cucumber,
    #[deku(id = "147")]
    Cupboard,
    #[deku(id = "148")]
    Curtain,
    #[deku(id = "149")]
    CuttingBoard,
    #[deku(id = "150")]
    Dagger,
    #[deku(id = "151")]
    DairyProduct,
    #[deku(id = "152")]
    Deer,
    #[deku(id = "153")]
    Desk,
    #[deku(id = "154")]
    Dessert,
    #[deku(id = "155")]
    Diaper,
    #[deku(id = "156")]
    Dice,
    #[deku(id = "157")]
    DigitalClock,
    #[deku(id = "158")]
    Dinosaur,
    #[deku(id = "159")]
    Dishwasher,
    #[deku(id = "160")]
    Dog,
    #[deku(id = "161")]
    DogBed,
    #[deku(id = "162")]
    Doll,
    #[deku(id = "163")]
    Dolphin,
    #[deku(id = "164")]
    Door,
    #[deku(id = "165")]
    DoorHandle,
    #[deku(id = "166")]
    Doughnut,
    #[deku(id = "167")]
    Dragonfly,
    #[deku(id = "168")]
    Drawer,
    #[deku(id = "169")]
    Dress,
    #[deku(id = "170")]
    Drill, // (Tool)
    #[deku(id = "171")]
    Drink,
    #[deku(id = "172")]
    DrinkingStraw,
    #[deku(id = "173")]
    Drum,
    #[deku(id = "174")]
    Duck,
    #[deku(id = "175")]
    Dumbbell,
    #[deku(id = "176")]
    Eagle,
    #[deku(id = "177")]
    Earrings,
    #[deku(id = "178")]
    Egg, //  (Food)
    #[deku(id = "179")]
    Elephant,
    #[deku(id = "180")]
    Envelope,
    #[deku(id = "181")]
    Eraser,
    #[deku(id = "182")]
    FacePowder,
    #[deku(id = "183")]
    FacialTissueHolder,
    #[deku(id = "184")]
    Falcon,
    #[deku(id = "185")]
    FashionAccessory,
    #[deku(id = "186")]
    FastFood,
    #[deku(id = "187")]
    Fax,
    #[deku(id = "188")]
    Fedora,
    #[deku(id = "189")]
    FilingCabinet,
    #[deku(id = "190")]
    FireHydrant,
    #[deku(id = "191")]
    Fireplace,
    #[deku(id = "192")]
    Fish,
    #[deku(id = "193")]
    Flag,
    #[deku(id = "194")]
    Flashlight,
    #[deku(id = "195")]
    Flower,
    #[deku(id = "196")]
    Flowerpot,
    #[deku(id = "197")]
    Flute,
    #[deku(id = "198")]
    FlyingDisc,
    #[deku(id = "199")]
    Food,
    #[deku(id = "200")]
    FoodProcessor,
    #[deku(id = "201")]
    Football,
    #[deku(id = "202")]
    FootballHelmet,
    #[deku(id = "203")]
    Footwear,
    #[deku(id = "204")]
    Fork,
    #[deku(id = "205")]
    Fountain,
    #[deku(id = "206")]
    Fox,
    #[deku(id = "207")]
    FrenchFries,
    #[deku(id = "208")]
    FrenchHorn,
    #[deku(id = "209")]
    Frog,
    #[deku(id = "210")]
    Fruit,
    #[deku(id = "211")]
    FryingPan,
    #[deku(id = "212")]
    Furniture,
    #[deku(id = "213")]
    GardenAsparagus,
    #[deku(id = "214")]
    GasStove,
    #[deku(id = "215")]
    Giraffe,
    #[deku(id = "216")]
    Girl,
    #[deku(id = "217")]
    Glasses,
    #[deku(id = "218")]
    Glove,
    #[deku(id = "219")]
    Goat,
    #[deku(id = "220")]
    Goggles,
    #[deku(id = "221")]
    Goldfish,
    #[deku(id = "222")]
    GolfBall,
    #[deku(id = "223")]
    GolfCart,
    #[deku(id = "224")]
    Gondola,
    #[deku(id = "225")]
    Goose,
    #[deku(id = "226")]
    Grape,
    #[deku(id = "227")]
    Grapefruit,
    #[deku(id = "228")]
    Grinder,
    #[deku(id = "229")]
    Guacamole,
    #[deku(id = "230")]
    Guitar,
    #[deku(id = "231")]
    HairDryer,
    #[deku(id = "232")]
    HairSpray,
    #[deku(id = "233")]
    Hamburger,
    #[deku(id = "234")]
    Hammer,
    #[deku(id = "235")]
    Hamster,
    #[deku(id = "236")]
    HandDryer,
    #[deku(id = "237")]
    Handbag,
    #[deku(id = "238")]
    Handgun,
    #[deku(id = "239")]
    HarborSeal,
    #[deku(id = "240")]
    Harmonica,
    #[deku(id = "241")]
    Harp,
    #[deku(id = "242")]
    Harpsichord,
    #[deku(id = "243")]
    Hat,
    #[deku(id = "244")]
    Headphones,
    #[deku(id = "245")]
    Heater,
    #[deku(id = "246")]
    Hedgehog,
    #[deku(id = "247")]
    Helicopter,
    #[deku(id = "248")]
    Helmet,
    #[deku(id = "249")]
    HighHeels,
    #[deku(id = "250")]
    HikingEquipment,
    #[deku(id = "251")]
    Hippopotamus,
    #[deku(id = "252")]
    HomeAppliance,
    #[deku(id = "253")]
    Honeycomb,
    #[deku(id = "254")]
    HorizontalBar,
    #[deku(id = "255")]
    Horse,
    #[deku(id = "256")]
    HotDog,
    #[deku(id = "257")]
    House,
    #[deku(id = "258")]
    Houseplant,
    #[deku(id = "259")]
    HumanArm,
    #[deku(id = "260")]
    HumanBeard,
    #[deku(id = "261")]
    HumanBody,
    #[deku(id = "262")]
    HumanEar,
    #[deku(id = "263")]
    HumanEye,
    #[deku(id = "264")]
    HumanFace,
    #[deku(id = "265")]
    HumanFoot,
    #[deku(id = "266")]
    HumanHair,
    #[deku(id = "267")]
    HumanHand,
    #[deku(id = "268")]
    HumanHead,
    #[deku(id = "269")]
    HumanLeg,
    #[deku(id = "270")]
    HumanMouth,
    #[deku(id = "271")]
    HumanNose,
    #[deku(id = "272")]
    Humidifier,
    #[deku(id = "273")]
    IceCream,
    #[deku(id = "274")]
    IndoorRower,
    #[deku(id = "275")]
    InfantBed,
    #[deku(id = "276")]
    Insect,
    #[deku(id = "277")]
    Invertebrate,
    #[deku(id = "278")]
    Ipod,
    #[deku(id = "279")]
    Isopod,
    #[deku(id = "280")]
    Jacket,
    #[deku(id = "281")]
    Jacuzzi,
    #[deku(id = "282")]
    Jaguar, // (Animal)
    #[deku(id = "283")]
    Jeans,
    #[deku(id = "284")]
    Jellyfish,
    #[deku(id = "285")]
    JetSki,
    #[deku(id = "286")]
    Jug,
    #[deku(id = "287")]
    Juice,
    #[deku(id = "288")]
    Kangaroo,
    #[deku(id = "289")]
    Kettle,
    #[deku(id = "290")]
    DinnerTable,
    #[deku(id = "291")]
    KitchenAppliance,
    #[deku(id = "292")]
    KitchenKnife,
    #[deku(id = "293")]
    KitchenUtensil,
    #[deku(id = "294")]
    Kitchenware,
    #[deku(id = "295")]
    Kite,
    #[deku(id = "296")]
    Knife,
    #[deku(id = "297")]
    Koala,
    #[deku(id = "298")]
    Ladder,
    #[deku(id = "299")]
    Ladle,
    #[deku(id = "300")]
    Ladybug,
    #[deku(id = "301")]
    Lamp,
    #[deku(id = "302")]
    LandVehicle,
    #[deku(id = "303")]
    Lantern,
    #[deku(id = "304")]
    Laptop,
    #[deku(id = "305")]
    Lavender, // (Plant)
    #[deku(id = "306")]
    Lemon,
    #[deku(id = "307")]
    Leopard,
    #[deku(id = "308")]
    LightBulb,
    #[deku(id = "309")]
    LightSwitch,
    #[deku(id = "310")]
    Lighthouse,
    #[deku(id = "311")]
    Lily,
    #[deku(id = "312")]
    Limousine,
    #[deku(id = "313")]
    Lion,
    #[deku(id = "314")]
    Lipstick,
    #[deku(id = "315")]
    Lizard,
    #[deku(id = "316")]
    Lobster,
    #[deku(id = "317")]
    Loveseat,
    #[deku(id = "318")]
    LuggageAndBags,
    #[deku(id = "319")]
    Lynx,
    #[deku(id = "320")]
    Magpie,
    #[deku(id = "321")]
    Mammal,
    #[deku(id = "322")]
    Man,
    #[deku(id = "323")]
    Mango,
    #[deku(id = "324")]
    Maple,
    #[deku(id = "325")]
    Maracas,
    #[deku(id = "326")]
    MarineInvertebrates,
    #[deku(id = "327")]
    MarineMammal,
    #[deku(id = "328")]
    MeasuringCup,
    #[deku(id = "329")]
    MechanicalFan,
    #[deku(id = "330")]
    MedicalEquipment,
    #[deku(id = "331")]
    Microphone,
    #[deku(id = "332")]
    MicrowaveOven,
    #[deku(id = "333")]
    Milk,
    #[deku(id = "334")]
    Miniskirt,
    #[deku(id = "335")]
    Mirror,
    #[deku(id = "336")]
    Missile,
    #[deku(id = "337")]
    Mixer,
    #[deku(id = "338")]
    MixingBowl,
    #[deku(id = "339")]
    MobilePhone,
    #[deku(id = "340")]
    Monkey,
    #[deku(id = "341")]
    MothsAndButterflies,
    #[deku(id = "342")]
    Motorcycle,
    #[deku(id = "343")]
    Mouse,
    #[deku(id = "344")]
    Muffin,
    #[deku(id = "345")]
    Mug,
    #[deku(id = "346")]
    Mule,
    #[deku(id = "347")]
    Mushroom,
    #[deku(id = "348")]
    MusicalInstrument,
    #[deku(id = "349")]
    MusicalKeyboard,
    #[deku(id = "350")]
    Nail, // (Construction)
    #[deku(id = "351")]
    Necklace,
    #[deku(id = "352")]
    Nightstand,
    #[deku(id = "353")]
    Oboe,
    #[deku(id = "354")]
    OfficeBuilding,
    #[deku(id = "355")]
    OfficeSupplies,
    #[deku(id = "356")]
    Orange,
    #[deku(id = "357")]
    Organ, // (Musical Instrument)
    #[deku(id = "358")]
    Ostrich,
    #[deku(id = "359")]
    Otter,
    #[deku(id = "360")]
    Oven,
    #[deku(id = "361")]
    Owl,
    #[deku(id = "362")]
    Oyster,
    #[deku(id = "363")]
    Paddle,
    #[deku(id = "364")]
    PalmTree,
    #[deku(id = "365")]
    Pancake,
    #[deku(id = "366")]
    Panda,
    #[deku(id = "367")]
    PaperCutter,
    #[deku(id = "368")]
    PaperTowel,
    #[deku(id = "369")]
    Parachute,
    #[deku(id = "370")]
    ParkingMeter,
    #[deku(id = "371")]
    Parrot,
    #[deku(id = "372")]
    Pasta,
    #[deku(id = "373")]
    Pastry,
    #[deku(id = "374")]
    Peach,
    #[deku(id = "375")]
    Pear,
    #[deku(id = "376")]
    Pen,
    #[deku(id = "377")]
    PencilCase,
    #[deku(id = "378")]
    PencilSharpener,
    #[deku(id = "379")]
    Penguin,
    #[deku(id = "380")]
    Perfume,
    #[deku(id = "381")]
    Person,
    #[deku(id = "382")]
    PersonalCare,
    #[deku(id = "383")]
    PersonalFlotationDevice,
    #[deku(id = "384")]
    Piano,
    #[deku(id = "385")]
    PicnicBasket,
    #[deku(id = "386")]
    PictureFrame,
    #[deku(id = "387")]
    Pig,
    #[deku(id = "388")]
    Pillow,
    #[deku(id = "389")]
    Pineapple,
    #[deku(id = "390")]
    Pitcher, // (Container)
    #[deku(id = "391")]
    Pizza,
    #[deku(id = "392")]
    PizzaCutter,
    #[deku(id = "393")]
    Plant,
    #[deku(id = "394")]
    PlasticBag,
    #[deku(id = "395")]
    Plate,
    #[deku(id = "396")]
    Platter,
    #[deku(id = "397")]
    PlumbingFixture,
    #[deku(id = "398")]
    PolarBear,
    #[deku(id = "399")]
    Pomegranate,
    #[deku(id = "400")]
    Popcorn,
    #[deku(id = "401")]
    Porch,
    #[deku(id = "402")]
    Porcupine,
    #[deku(id = "403")]
    Poster,
    #[deku(id = "404")]
    Potato,
    #[deku(id = "405")]
    PowerPlugsAndSockets,
    #[deku(id = "406")]
    PressureCooker,
    #[deku(id = "407")]
    Pretzel,
    #[deku(id = "408")]
    Printer,
    #[deku(id = "409")]
    Pumpkin,
    #[deku(id = "410")]
    PunchingBag,
    #[deku(id = "411")]
    Rabbit,
    #[deku(id = "412")]
    Raccoon,
    #[deku(id = "413")]
    Racket,
    #[deku(id = "414")]
    Radish,
    #[deku(id = "415")]
    Ratchet, // (Device)
    #[deku(id = "416")]
    Raven,
    #[deku(id = "417")]
    RaysAndSkates,
    #[deku(id = "418")]
    RedPanda,
    #[deku(id = "419")]
    Refrigerator,
    #[deku(id = "420")]
    RemoteControl,
    #[deku(id = "421")]
    Reptile,
    #[deku(id = "422")]
    Rhinoceros,
    #[deku(id = "423")]
    Rifle,
    #[deku(id = "424")]
    RingBinder,
    #[deku(id = "425")]
    Rocket,
    #[deku(id = "426")]
    RollerSkates,
    #[deku(id = "427")]
    Rose,
    #[deku(id = "428")]
    RugbyBall,
    #[deku(id = "429")]
    Ruler,
    #[deku(id = "430")]
    Salad,
    #[deku(id = "431")]
    SaltAndPepperShakers,
    #[deku(id = "432")]
    Sandal,
    #[deku(id = "433")]
    Sandwich,
    #[deku(id = "434")]
    Saucer,
    #[deku(id = "435")]
    Saxophone,
    #[deku(id = "436")]
    Scale,
    #[deku(id = "437")]
    Scarf,
    #[deku(id = "438")]
    Scissors,
    #[deku(id = "439")]
    Scoreboard,
    #[deku(id = "440")]
    Scorpion,
    #[deku(id = "441")]
    Screwdriver,
    #[deku(id = "442")]
    Sculpture,
    #[deku(id = "443")]
    SeaLion,
    #[deku(id = "444")]
    SeaTurtle,
    #[deku(id = "445")]
    Seafood,
    #[deku(id = "446")]
    Seahorse,
    #[deku(id = "447")]
    SeatBelt,
    #[deku(id = "448")]
    Segway,
    #[deku(id = "449")]
    ServingTray,
    #[deku(id = "450")]
    SewingMachine,
    #[deku(id = "451")]
    Shark,
    #[deku(id = "452")]
    Sheep,
    #[deku(id = "453")]
    Shelf,
    #[deku(id = "454")]
    Shellfish,
    #[deku(id = "455")]
    Shirt,
    #[deku(id = "456")]
    Shorts,
    #[deku(id = "457")]
    Shotgun,
    #[deku(id = "458")]
    Shower,
    #[deku(id = "459")]
    Shrimp,
    #[deku(id = "460")]
    Sink,
    #[deku(id = "461")]
    Skateboard,
    #[deku(id = "462")]
    Ski,
    #[deku(id = "463")]
    Skirt,
    #[deku(id = "464")]
    Skull,
    #[deku(id = "465")]
    Skunk,
    #[deku(id = "466")]
    Skyscraper,
    #[deku(id = "467")]
    SlowCooker,
    #[deku(id = "468")]
    Snack,
    #[deku(id = "469")]
    Snail,
    #[deku(id = "470")]
    Snake,
    #[deku(id = "471")]
    Snowboard,
    #[deku(id = "472")]
    Snowman,
    #[deku(id = "473")]
    Snowmobile,
    #[deku(id = "474")]
    Snowplow,
    #[deku(id = "475")]
    SoapDispenser,
    #[deku(id = "476")]
    Sock,
    #[deku(id = "477")]
    SofaBed,
    #[deku(id = "478")]
    Sombrero,
    #[deku(id = "479")]
    Sparrow,
    #[deku(id = "480")]
    Spatula,
    #[deku(id = "481")]
    SpiceRack,
    #[deku(id = "482")]
    Spider,
    #[deku(id = "483")]
    Spoon,
    #[deku(id = "484")]
    SportsEquipment,
    #[deku(id = "485")]
    SportsUniform,
    #[deku(id = "486")]
    Squash, //(Plant)
    #[deku(id = "487")]
    Squid,
    #[deku(id = "488")]
    Squirrel,
    #[deku(id = "489")]
    Stairs,
    #[deku(id = "490")]
    Stapler,
    #[deku(id = "491")]
    Starfish,
    #[deku(id = "492")]
    StationaryBicycle,
    #[deku(id = "493")]
    Stethoscope,
    #[deku(id = "494")]
    Stool,
    #[deku(id = "495")]
    StopSign,
    #[deku(id = "496")]
    Strawberry,
    #[deku(id = "497")]
    StreetLight,
    #[deku(id = "498")]
    Stretcher,
    #[deku(id = "499")]
    StudioCouch,
    #[deku(id = "500")]
    Submarine,
    #[deku(id = "501")]
    SubmarineSandwich,
    #[deku(id = "502")]
    Suit,
    #[deku(id = "503")]
    Suitcase,
    #[deku(id = "504")]
    SunHat,
    #[deku(id = "505")]
    Sunglasses,
    #[deku(id = "506")]
    Surfboard,
    #[deku(id = "507")]
    Sushi,
    #[deku(id = "508")]
    Swan,
    #[deku(id = "509")]
    SwimCap,
    #[deku(id = "510")]
    SwimmingPool,
    #[deku(id = "511")]
    Swimwear,
    #[deku(id = "512")]
    Sword,
    #[deku(id = "513")]
    Syringe,
    #[deku(id = "514")]
    Table,
    #[deku(id = "515")]
    TableTennisRacket,
    #[deku(id = "516")]
    TabletComputer,
    #[deku(id = "517")]
    Tableware,
    #[deku(id = "518")]
    Taco,
    #[deku(id = "519")]
    Tank,
    #[deku(id = "520")]
    Tap,
    #[deku(id = "521")]
    Tart,
    #[deku(id = "522")]
    Taxi,
    #[deku(id = "523")]
    Tea,
    #[deku(id = "524")]
    Teapot,
    #[deku(id = "525")]
    TeddyBear,
    #[deku(id = "526")]
    Telephone,
    #[deku(id = "527")]
    Television,
    #[deku(id = "528")]
    TennisBall,
    #[deku(id = "529")]
    TennisRacket,
    #[deku(id = "530")]
    Tent,
    #[deku(id = "531")]
    Tiara,
    #[deku(id = "532")]
    Tick,
    #[deku(id = "533")]
    Tie,
    #[deku(id = "534")]
    Tiger,
    #[deku(id = "535")]
    TinCan,
    #[deku(id = "536")]
    Tire,
    #[deku(id = "537")]
    Toaster,
    #[deku(id = "538")]
    Toilet,
    #[deku(id = "539")]
    ToiletPaper,
    #[deku(id = "540")]
    Tomato,
    #[deku(id = "541")]
    Tool,
    #[deku(id = "542")]
    Toothbrush,
    #[deku(id = "543")]
    Torch,
    #[deku(id = "544")]
    Tortoise,
    #[deku(id = "545")]
    Towel,
    #[deku(id = "546")]
    Tower,
    #[deku(id = "547")]
    Toy,
    #[deku(id = "548")]
    TrafficLight,
    #[deku(id = "549")]
    TrafficSign,
    #[deku(id = "550")]
    Train,
    #[deku(id = "551")]
    TrainingBench,
    #[deku(id = "552")]
    Treadmill,
    #[deku(id = "553")]
    Tree,
    #[deku(id = "554")]
    TreeHouse,
    #[deku(id = "555")]
    Tripod,
    #[deku(id = "556")]
    Trombone,
    #[deku(id = "557")]
    Trousers,
    #[deku(id = "558")]
    Truck,
    #[deku(id = "559")]
    Trumpet,
    #[deku(id = "560")]
    Turkey,
    #[deku(id = "561")]
    Turtle,
    #[deku(id = "562")]
    Umbrella,
    #[deku(id = "563")]
    Unicycle,
    #[deku(id = "564")]
    Van,
    #[deku(id = "565")]
    Vase,
    #[deku(id = "566")]
    Vegetable,
    #[deku(id = "567")]
    Vehicle,
    #[deku(id = "568")]
    VehicleRegistrationPlate,
    #[deku(id = "569")]
    Violin,
    #[deku(id = "570")]
    Volleyball, // (Ball)
    #[deku(id = "571")]
    Waffle,
    #[deku(id = "572")]
    WaffleIron,
    #[deku(id = "573")]
    WallClock,
    #[deku(id = "574")]
    Wardrobe,
    #[deku(id = "575")]
    WashingMachine,
    #[deku(id = "576")]
    WasteContainer,
    #[deku(id = "577")]
    Watch,
    #[deku(id = "578")]
    Watercraft,
    #[deku(id = "579")]
    Watermelon,
    #[deku(id = "580")]
    Weapon,
    #[deku(id = "581")]
    Whale,
    #[deku(id = "582")]
    Wheel,
    #[deku(id = "583")]
    Wheelchair,
    #[deku(id = "584")]
    Whisk,
    #[deku(id = "585")]
    Whiteboard,
    #[deku(id = "586")]
    Willow,
    #[deku(id = "587")]
    Window,
    #[deku(id = "588")]
    WindowBlind,
    #[deku(id = "589")]
    Wine,
    #[deku(id = "590")]
    WineGlass,
    #[deku(id = "591")]
    WineRack,
    #[deku(id = "592")]
    WinterMelon,
    #[deku(id = "593")]
    Wok,
    #[deku(id = "594")]
    Woman,
    #[deku(id = "595")]
    WoodBurningStove,
    #[deku(id = "596")]
    Woodpecker,
    #[deku(id = "597")]
    Worm,
    #[deku(id = "598")]
    Wrench,
    #[deku(id = "599")]
    Zebra,
    #[deku(id = "600")]
    Zucchini,
}
