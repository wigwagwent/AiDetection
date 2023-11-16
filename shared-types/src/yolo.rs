use deku::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(
    Debug, PartialEq, DekuRead, DekuWrite, Serialize, Deserialize, Eq, PartialOrd, Clone, Copy,
)]
#[deku(type = "u8")]
pub enum YoloClasses {
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

#[derive(Serialize, Deserialize, PartialEq, PartialOrd, Clone, Copy)]
pub struct ItemBox {
    pub x1: f32,
    pub y1: f32,
    pub x2: f32,
    pub y2: f32,
    pub label: YoloClasses,
    pub probablility: f32,
}
