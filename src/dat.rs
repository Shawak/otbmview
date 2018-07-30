// https://github.com/slavidodo/opentibiaunity-datspr-converter/blob/master/OpenTibiaUnity/Core/Sprites/ContentData.cs

use std;
use std::fs::File;
use std::io::{Error, Cursor, Read, Write, Seek, SeekFrom};
use std::collections::HashMap;
use std::collections::hash_map::DefaultHasher;

use mem_read::*;
use mem_type::*;

use num_traits::{FromPrimitive, ToPrimitive};

#[derive(Primitive)]
enum DatAttributesHeader {
    Ground = 0,
    GroundBorder = 1,
    OnBottom = 2,
    OnTop = 3,
    Container = 4,
    Stackable = 5,
    ForceUse = 6,
    MultiUse = 7,
    Writeable = 8,
    WriteableOnce = 9,
    FluidContainer = 10,
    Splash = 11,
    NotWalkable = 12,
    NotMoveable = 13,
    BlockProjectile = 14,
    NotPathable = 15,
    NoMoveAnimation = 16,
    Pickupable = 17,
    Hangable = 18,
    HookSouth = 19,
    HookEast = 20,
    Rotateable = 21,
    Light = 22,
    DontHide = 23,
    Translucent = 24,
    Displacement = 25,
    Elevation = 26,
    LyingCorpse = 27,
    AnimateAlways = 28,
    MinimapColor = 29,
    LensHelp = 30,
    FullGround = 31,
    Look = 32,
    Cloth = 33,
    Market = 34,
    DefaultAction = 35,
    Wrapable = 36,
    Unwrapable = 37,
    TopEffect = 38,

    NotPreWalkable = 100,
    Unknown1 = 101,

    Usable = 254,

    LastAttr = 255
}

impl std::cmp::Eq for DatAttributesHeader {

}

impl std::cmp::PartialEq for DatAttributesHeader {
    fn eq(&self, other: &DatAttributesHeader) -> bool {
        self == other
    }
}

impl std::hash::Hash for DatAttributesHeader {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) -> u64 {
        let mut s = DefaultHasher::new();
        t.hash(&mut s);
        s.finish()
    }
}

struct LightInfo {
    intensity: u16,
    color: u16
}

struct Vector2 {
    x: u16,
    y: u16
}

struct MarketInfo {
    category: u16,
    trade_as: u16,
    show_as: u16,
    name: String,
    restrict_vocation: u16,
    required_level: u16
}

enum DatAttributes {
    Ground(u16),
    GroundBorder,
    OnBottom,
    OnTop,
    Container,
    Stackable,
    ForceUse,
    MultiUse,
    Writeable(u16),
    WriteableOnce(u16),
    FluidContainer,
    Splash,
    NotWalkable,
    NotMoveable,
    BlockProjectile,
    NotPathable,
    NoMoveAnimation,
    Pickupable,
    Hangable,
    HookSouth,
    HookEast,
    Rotateable,
    Light(LightInfo),
    DontHide,
    Translucent,
    Displacement(Vector2),
    Elevation(u16),
    LyingCorpse,
    AnimateAlways,
    MinimapColor(u16),
    LensHelp(u16),
    FullGround,
    Look,
    Cloth(u16),
    Market(MarketInfo),
    DefaultAction(u16),
    Wrapable,
    Unwrapable,
    TopEffect,

    NotPreWalkable,
    Unknown1,

    Usable,

    LastAttr
}

impl DatAttributes {
    fn new(header: &DatAttributesHeader, category: ThingCategory, data: &mut Cursor<Vec<u8>>) -> Result<DatAttributes, Error> {
        let r = match header {
            Ground => DatAttributes::Ground(data.get()?),
            Writeable =>  DatAttributes::Writeable(data.get()?),
            WriteableOnce =>  DatAttributes::WriteableOnce(data.get()?),
            MinimapColor =>  DatAttributes::MinimapColor(data.get()?),
            LensHelp =>  DatAttributes::LensHelp(data.get()?),
            Cloth =>  DatAttributes::Cloth(data.get()?),
            DefaultAction =>  DatAttributes::DefaultAction(data.get()?),

            GroundBorder => DatAttributes::GroundBorder,
            OnBottom => DatAttributes::OnBottom,
            OnTop => DatAttributes::OnTop,
            Container => DatAttributes::Container,
            Stackable => DatAttributes::Stackable,
            ForceUse => DatAttributes::ForceUse,
            MultiUse => DatAttributes::MultiUse,
            FluidContainer => DatAttributes::FluidContainer,
            Splash => DatAttributes::Splash,
            NotWalkable => DatAttributes::NotWalkable,
            NotMoveable => DatAttributes::NotMoveable,
            BlockProjectile => DatAttributes::BlockProjectile,
            NotPathable => DatAttributes::NotPathable,
            NoMoveAnimation => DatAttributes::NoMoveAnimation,
            Pickupable => DatAttributes::Pickupable,
            Hangable => DatAttributes::Hangable,
            HookSouth => DatAttributes::HookSouth,
            HookEast => DatAttributes::HookEast,
            Rotateable => DatAttributes::Rotateable,
            DontHide => DatAttributes::DontHide,
            Translucent => DatAttributes::Translucent,
            LyingCorpse => DatAttributes::LyingCorpse,
            AnimateAlways => DatAttributes::AnimateAlways,
            FullGround => DatAttributes::FullGround,
            Look => DatAttributes::Look,
            Wrapable => DatAttributes::Wrapable,
            Unwrapable => DatAttributes::Unwrapable,
            TopEffect => DatAttributes::TopEffect,
            Usable => DatAttributes::Usable,

            Light => DatAttributes::Light(LightInfo { intensity: data.get()?, color: data.get()? }),
            Displacement => DatAttributes::Displacement(Vector2 { x: data.get()?, y: data.get()? }),

            Elevation => DatAttributes::Elevation(data.get()?),
            Market => DatAttributes::Market(MarketInfo {
                category: data.get()?,
                trade_as: data.get()?,
                show_as: data.get()?,
                name: data.gets()?,
                restrict_vocation: data.get()?,
                required_level: data.get()?
            }),

            _ => panic!("unknown item attribute")
        };

        let group_count = if category == ThingCategory::Creature { data.get::<u8>()? } else { 1 };
        for i in 0..group_count {

        }


        Ok(r)
    }
}

#[derive(Primitive)]
enum ThingCategory {
    Item = 0,
    Creature = 1,
    Effect = 2,
    Missile = 3
}

struct Thing {
    id: u16,
    attributes: HashMap<DatAttributesHeader, DatAttributes>
}

impl Thing {
    fn new(id: u16) -> Thing {
        Thing { id, attributes: HashMap::new() }
    }
}

const THING_CATEGORIES: &'static[ThingCategory] = &[ThingCategory::Item, ThingCategory::Creature, ThingCategory::Effect, ThingCategory::Missile];

pub fn parse_items(data: &mut Cursor<Vec<u8>>) -> Result<(), Error> {
    let mut things: HashMap<ThingCategory, HashMap<u16, Thing>> = HashMap::new();
    for category in THING_CATEGORIES {
        let first_id = if category == ThingCategory::Item { 100 } else { 1 };
        let thing_count = data.get::<u16>()?;

        for id in first_id..thing_count {
            let mut thing = Thing::new(id);

            let n = DatAttributesHeader::LastAttr.to_u8().expect("Error");
            for k in 0..n {
                let header = DatAttributesHeader::from_u8(data.get::<u8>()?).expect("unknown dat attribute");
                if header == DatAttributesHeader::LastAttr {
                    break;
                }

                let mut attr = DatAttributes::new(&header, *category, data)?;
                thing.attributes.insert(header, attr);
            }
        }
    }

    Ok(())
}

pub fn parse(filename: String) -> Result<(), Error> {
    let mut file = File::open(filename)?;
    let mut data: Vec<u8> = Vec::new();
    file.read_to_end(&mut data)?;
    let mut data = Cursor::new(data);

    let signature = data.get::<u32>();
    parse_items(&mut data)?;

    Ok(())
}