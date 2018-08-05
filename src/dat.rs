// https://github.com/slavidodo/opentibiaunity-datspr-converter/blob/master/OpenTibiaUnity/Core/Sprites/ContentData.cs

use std::collections::HashMap;
use std::fs::File;
use std::io::{Error, Read};

use mem_read::*;

use rand::*;

use num_traits::{FromPrimitive, ToPrimitive};

#[derive(Debug, Eq, Hash, PartialEq, Primitive)]
pub enum DatAttributesHeader {
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

    LastAttr = 255,
}

#[derive(Debug)]
pub struct LightInfo {
    intensity: u16,
    color: u16,
}

#[derive(Debug)]
pub struct Vector2 {
    x: u16,
    y: u16,
}

#[derive(Debug)]
pub struct MarketInfo {
    category: u16,
    trade_as: u16,
    show_as: u16,
    name: String,
    restrict_vocation: u16,
    required_level: u16,
}

#[derive(Debug)]
pub enum DatAttributes {
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

    LastAttr,
}

impl DatAttributes {
    fn new<T: MemRead>(
        header: &DatAttributesHeader,
        category: ThingCategory,
        data: &mut T,
    ) -> Result<DatAttributes, Error> {
        let r = match header {
            DatAttributesHeader::Ground => DatAttributes::Ground(data.get()?),
            DatAttributesHeader::Writeable => DatAttributes::Writeable(data.get()?),
            DatAttributesHeader::WriteableOnce => DatAttributes::WriteableOnce(data.get()?),
            DatAttributesHeader::MinimapColor => DatAttributes::MinimapColor(data.get()?),
            DatAttributesHeader::LensHelp => DatAttributes::LensHelp(data.get()?),
            DatAttributesHeader::Cloth => DatAttributes::Cloth(data.get()?),
            DatAttributesHeader::DefaultAction => DatAttributes::DefaultAction(data.get()?),

            DatAttributesHeader::GroundBorder => DatAttributes::GroundBorder,
            DatAttributesHeader::OnBottom => DatAttributes::OnBottom,
            DatAttributesHeader::OnTop => DatAttributes::OnTop,
            DatAttributesHeader::Container => DatAttributes::Container,
            DatAttributesHeader::Stackable => DatAttributes::Stackable,
            DatAttributesHeader::ForceUse => DatAttributes::ForceUse,
            DatAttributesHeader::MultiUse => DatAttributes::MultiUse,
            DatAttributesHeader::FluidContainer => DatAttributes::FluidContainer,
            DatAttributesHeader::Splash => DatAttributes::Splash,
            DatAttributesHeader::NotWalkable => DatAttributes::NotWalkable,
            DatAttributesHeader::NotMoveable => DatAttributes::NotMoveable,
            DatAttributesHeader::BlockProjectile => DatAttributes::BlockProjectile,
            DatAttributesHeader::NotPathable => DatAttributes::NotPathable,
            DatAttributesHeader::NoMoveAnimation => DatAttributes::NoMoveAnimation,
            DatAttributesHeader::Pickupable => DatAttributes::Pickupable,
            DatAttributesHeader::Hangable => DatAttributes::Hangable,
            DatAttributesHeader::HookSouth => DatAttributes::HookSouth,
            DatAttributesHeader::HookEast => DatAttributes::HookEast,
            DatAttributesHeader::Rotateable => DatAttributes::Rotateable,
            DatAttributesHeader::DontHide => DatAttributes::DontHide,
            DatAttributesHeader::Translucent => DatAttributes::Translucent,
            DatAttributesHeader::LyingCorpse => DatAttributes::LyingCorpse,
            DatAttributesHeader::AnimateAlways => DatAttributes::AnimateAlways,
            DatAttributesHeader::FullGround => DatAttributes::FullGround,
            DatAttributesHeader::Look => DatAttributes::Look,
            DatAttributesHeader::Wrapable => DatAttributes::Wrapable,
            DatAttributesHeader::Unwrapable => DatAttributes::Unwrapable,
            DatAttributesHeader::TopEffect => DatAttributes::TopEffect,
            DatAttributesHeader::Usable => DatAttributes::Usable,

            DatAttributesHeader::Light => DatAttributes::Light(LightInfo {
                intensity: data.get()?,
                color: data.get()?,
            }),
            DatAttributesHeader::Displacement => DatAttributes::Displacement(Vector2 {
                x: data.get()?,
                y: data.get()?,
            }),

            DatAttributesHeader::Elevation => DatAttributes::Elevation(data.get()?),
            DatAttributesHeader::Market => DatAttributes::Market(MarketInfo {
                category: data.get()?,
                trade_as: data.get()?,
                show_as: data.get()?,
                name: data.gets()?,
                restrict_vocation: data.get()?,
                required_level: data.get()?,
            }),

            _ => panic!("unknown item attribute"),
        };
        Ok(r)
    }
}

#[derive(Debug)]
pub struct FrameGroupDuration {
    minimum: u32,
    maximum: u32,
}

impl FrameGroupDuration {
    fn new<T: MemRead>(data: &mut T) -> Result<FrameGroupDuration, Error> {
        Ok(FrameGroupDuration {
            minimum: data.get()?,
            maximum: data.get()?,
        })
    }

    fn duration(&self) -> u32 {
        if self.minimum == self.maximum {
            self.minimum
        } else {
            thread_rng().gen_range(self.minimum, self.maximum)
        }
    }
}

#[derive(Debug)]
pub struct FrameGroupAnimator {
    animation_phases: u8,
    async: bool,
    loop_count: i32,
    start_phase: i8,
    current_phase: i32,
    current_duration: i32,
    last_phase_ticks: u64,
    animation_direction: u8,
    is_complete: bool,
    current_loop: u8,
    frame_group_durations: Vec<FrameGroupDuration>,
}

impl FrameGroupAnimator {
    fn new<T: MemRead>(animation_phases: u8, data: &mut T) -> Result<FrameGroupAnimator, Error> {
        let mut animator = FrameGroupAnimator {
            animation_phases,
            async: data.get::<u8>()? == 0,
            loop_count: data.get()?,
            start_phase: data.get()?,
            current_phase: 0,
            current_duration: 0,
            last_phase_ticks: 0,
            animation_direction: 0,
            is_complete: false,
            current_loop: 0,
            frame_group_durations: Vec::new(),
        };

        for _ in 0..animation_phases {
            let duration = FrameGroupDuration::new(data)?;
            animator.frame_group_durations.push(duration);
        }

        Ok(animator)
    }
}

#[derive(Primitive, Eq, PartialEq, Hash)]
pub enum FrameGroupType {
    Idle = 0,
    Moving = 1,
}

#[derive(Debug)]
pub struct FrameGroup {
    width: u8,
    height: u8,
    exact_size: u8,
    layers: u8,
    pattern_width: u8,
    pattern_height: u8,
    pattern_depth: u8,
    phases: u8,
    animator: Option<FrameGroupAnimator>,
    sprites: Vec<u32>,
}

impl FrameGroup {
    fn new<T: MemRead>(data: &mut T) -> Result<FrameGroup, Error> {
        let width = data.get::<u8>()?;
        let height = data.get::<u8>()?;

        let mut frame_group = FrameGroup {
            width,
            height,
            exact_size: if width > 1 || height > 1 {
                data.get()?
            } else {
                32u8
            },
            layers: data.get()?,
            pattern_width: data.get()?,
            pattern_height: data.get()?,
            pattern_depth: data.get()?,
            phases: data.get()?,
            animator: Option::None,
            sprites: Vec::new(),
        };

        if frame_group.phases > 1 {
            frame_group.animator = Some(FrameGroupAnimator::new(frame_group.phases, data)?);
        }

        let total_sprites: u32 = frame_group.width as u32
            * frame_group.height as u32
            * frame_group.layers as u32
            * frame_group.pattern_width as u32
            * frame_group.pattern_height as u32
            * frame_group.pattern_depth as u32
            * frame_group.phases as u32;

        for _ in 0..total_sprites {
            frame_group.sprites.push(data.get()?);
        }

        //println!("{:?}", frame_group);

        Ok(frame_group)
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Primitive)]
pub enum ThingCategory {
    Item = 0,
    Creature = 1,
    Effect = 2,
    Missile = 3,
}

pub struct Thing {
    id: u16,
    attributes: HashMap<DatAttributesHeader, DatAttributes>,
    frame_groups: HashMap<FrameGroupType, FrameGroup>,
}

impl Thing {
    fn new(id: u16) -> Thing {
        Thing {
            id,
            attributes: HashMap::new(),
            frame_groups: HashMap::new(),
        }
    }
}

const THING_CATEGORIES: &[ThingCategory] = &[
    ThingCategory::Item,
    ThingCategory::Creature,
    ThingCategory::Effect,
    ThingCategory::Missile,
];

pub fn parse_items<T: MemRead>(
    data: &mut T,
) -> Result<HashMap<ThingCategory, HashMap<u16, Thing>>, Error> {
    let mut counts = HashMap::new();
    for &category in THING_CATEGORIES {
        let count = data.get::<u16>()? + 1;
        //println!("count: {}", count);
        counts.insert(category, count);
    }

    let mut things = HashMap::new();
    for &category in THING_CATEGORIES {
        let first_id = if category == ThingCategory::Item {
            100
        } else {
            1
        };

        let mut map = HashMap::new();
        for id in first_id..counts[&category] {
            //println!("id: {}/{} {:?}", id, counts[category], category);
            let mut thing = Thing::new(id);

            let n = DatAttributesHeader::LastAttr.to_u8().expect("Error");
            for _ in 0..n {
                //println!("pos: {}", data.position());
                let header =
                    DatAttributesHeader::from_u8(data.get()?).expect("unknown dat attribute");
                //println!("header: {:?}", header);
                if header == DatAttributesHeader::LastAttr {
                    break;
                }

                let mut attr = DatAttributes::new(&header, category, data)?;
                //println!(" > {:?} -> {:?}", header, attr);
                thing.attributes.insert(header, attr);
            }

            let group_count = if category == ThingCategory::Creature {
                data.get::<u8>()?
            } else {
                1
            };
            //println!("group_count: {}", group_count);
            for i in 0..group_count {
                let group_type = if category == ThingCategory::Creature {
                    FrameGroupType::from_u8(data.get()?).expect("unknown frame group")
                } else {
                    FrameGroupType::Idle
                };

                thing
                    .frame_groups
                    .insert(group_type, FrameGroup::new(data)?);
            }

            map.insert(id, thing);
        }

        things.insert(category, map);
    }

    Ok(things)
}

pub fn parse(filename: String) -> Result<HashMap<ThingCategory, HashMap<u16, Thing>>, Error> {
    let mut file = File::open(filename)?;
    let mut data: Vec<u8> = Vec::new();
    file.read_to_end(&mut data)?;
    let data: &mut &[u8] = &mut data.as_ref();

    let signature = data.get::<u32>();
    parse_items(data)
}
