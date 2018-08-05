// https://otland.net/threads/a-comphrensive-description-of-the-otbm-format.258583/

#![allow(dead_code)]

use std::fs::File;
//use std::io::prelude::*;
use std::io::{Error, ErrorKind, Read};

use std;

use num_traits::{FromPrimitive, ToPrimitive};

use mem_read::*;

const NODE_ESC: u8 = 0xFD;
const NODE_INIT: u8 = 0xFE;
const NODE_TERM: u8 = 0xFF;

#[derive(Primitive)]
enum Header {
    MapHeader = 0x00,
    MapData = 0x02,
    TileArea = 0x04,
    Tile = 0x05,
    Item = 0x06,
    Towns = 0x0C,
    Town = 0x0D,
    HouseTile = 0x0E,
    Waypoints = 0x0F,
    Waypoint = 0x10,
}

#[derive(PartialEq)]
pub enum Node {
    Unknown,
    MapHeader(MapHeaderNode),
    MapData(MapDataNode),
    TileArea(TileAreaNode),
    Tile(TileNode),
    Item(ItemNode),
    Towns(TownsNode),
    Town(TownNode),
    HouseTile(HouseTileNode),
    Waypoints(WaypointsNode),
    Waypoint(WaypointNode),
}

impl std::fmt::Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Node::MapHeader(x) => write!(
                f,
                "MapHeader - Width: {} Height: {}",
                x.map_width, x.map_height
            ),
            Node::MapData(x) => write!(f, "MapData"),
            Node::TileArea(x) => write!(f, "TileArea x: {} y: {} z: {}", x.x, x.y, x.z),
            Node::Tile(x) => write!(f, "Tile x: {} y: {}", x.x, x.y),
            Node::Item(x) => write!(f, "Item id: {}", x.id),
            Node::Towns(x) => write!(f, "Towns"),
            Node::Town(x) => write!(f, "Town"),
            Node::HouseTile(x) => write!(f, "HouseTile"),
            Node::Waypoints(x) => write!(f, "Waypoints"),
            Node::Waypoint(x) => write!(f, "Waypoint"),
            _ => write!(f, "Unknown"),
        }
    }
}

#[derive(PartialEq)]
pub struct MapHeaderNode {
    version: u32,
    map_width: u16,
    map_height: u16,
    items_major_version: u32,
    items_minor_version: u32,
}

impl MapHeaderNode {
    fn parse<T: MemRead>(data: &mut T) -> Result<MapHeaderNode, Error> {
        Ok(MapHeaderNode {
            version: data.get()?,
            map_width: data.get()?,
            map_height: data.get()?,
            items_major_version: data.get()?,
            items_minor_version: data.get()?,
        })
    }
}

#[derive(PartialEq)]
pub struct MapDataNode {}

impl MapDataNode {
    fn parse<T: MemRead>(data: &mut T) -> Result<MapDataNode, Error> {
        Ok(MapDataNode {})
    }
}

#[derive(PartialEq)]
pub struct TileAreaNode {
    x: u16,
    y: u16,
    z: u8,
    //tiles: Vec<TileNode>
}

impl TileAreaNode {
    fn parse<T: MemRead>(data: &mut T) -> Result<TileAreaNode, Error> {
        Ok(TileAreaNode {
            x: data.get()?,
            y: data.get()?,
            z: data.get()?,
        })
    }
}

#[derive(PartialEq)]
pub struct TileNode {
    x: u8,
    y: u8,
    //items: Vec<ItemNode>
}

impl TileNode {
    fn parse<T: MemRead>(data: &mut T) -> Result<TileNode, Error> {
        Ok(TileNode {
            x: data.get()?,
            y: data.get()?,
        })
    }
}

#[derive(PartialEq)]
pub struct ItemNode {
    id: u16,
    // content: Vec<???>
}

impl ItemNode {
    fn parse<T: MemRead>(data: &mut T) -> Result<ItemNode, Error> {
        Ok(ItemNode { id: data.get()? })
    }
}

#[derive(PartialEq)]
pub struct HouseTileNode {
    x: u16,
    y: u16,
    house_id: u32,
    //items: Vec<ItemNode>
}

impl HouseTileNode {
    fn parse<T: MemRead>(data: &mut T) -> Result<HouseTileNode, Error> {
        Ok(HouseTileNode {
            x: data.get()?,
            y: data.get()?,
            house_id: data.get()?,
        })
    }
}

#[derive(PartialEq)]
pub struct WaypointsNode {
    // nodes: Vec<WaypontNode>
}

impl WaypointsNode {
    fn parse<T: MemRead>(data: &mut T) -> Result<WaypointsNode, Error> {
        Ok(WaypointsNode {})
    }
}

#[derive(PartialEq)]
pub struct WaypointNode {
    name: String,
    x: u16,
    y: u16,
    z: u8,
}

impl WaypointNode {
    fn parse<T: MemRead>(data: &mut T) -> Result<WaypointNode, Error> {
        Ok(WaypointNode {
            name: String::new(),
            x: 0,
            y: 0,
            z: 0,
        })
    }
}

#[derive(PartialEq)]
pub struct TownsNode {
    //towns: Vec<TownNode>
}

impl TownsNode {
    fn parse<T: MemRead>(data: &mut T) -> Result<TownsNode, Error> {
        Ok(TownsNode {})
    }
}

#[derive(PartialEq)]
pub struct TownNode {
    town_id: u32,
    name: String,
    x: u16,
    y: u16,
    z: u8,
}

impl TownNode {
    fn parse<T: MemRead>(data: &mut T) -> Result<TownNode, Error> {
        Ok(TownNode {
            town_id: data.get()?,
            name: String::new(),
            x: 0,
            y: 0,
            z: 0,
        })
    }
}

pub fn read_otbm(filename: String) -> Result<Node, Error> {
    let mut file = File::open(filename)?;
    let mut data: Vec<u8> = Vec::new();
    file.read_to_end(&mut data)?;
    let data: &mut &[u8] = &mut data.as_ref();

    let map_identifier = data.get::<u32>()?;
    if map_identifier != 0x0 && map_identifier != 0x4D42_544F {
        panic!("unknown OTBM format: unexpected magic bytes.");
    }

    read_node(data, false)
}

fn read_node<T: MemRead>(data: &mut T, is_child: bool) -> Result<Node, Error> {
    let mut node = Node::Unknown;
    let mut children = vec![];

    let mut skip = false;
    let mut first = true;

    loop {
        if skip {
            skip = false;
            continue;
        }

        let c_byte = if is_child && first {
            first = false;
            0xFE
        } else {
            data.get::<u8>()?
        };

        match c_byte {
            NODE_INIT | NODE_TERM if node == Node::Unknown => {
                let identifier = data.get::<u8>()?;

                node = Header::from_u8(identifier)
                    .ok_or_else(|| Error::new(ErrorKind::Other, "from_u8 failed".to_string()))
                    .and_then(|x| {
                        Ok(match x {
                            Header::MapHeader => Node::MapHeader(MapHeaderNode::parse(data)?),
                            Header::MapData => Node::MapData(MapDataNode::parse(data)?),
                            Header::TileArea => Node::TileArea(TileAreaNode::parse(data)?),
                            Header::Tile => Node::Tile(TileNode::parse(data)?),
                            Header::Item => Node::Item(ItemNode::parse(data)?),
                            Header::Towns => Node::Towns(TownsNode::parse(data)?),
                            Header::Town => Node::Town(TownNode::parse(data)?),
                            Header::HouseTile => Node::HouseTile(HouseTileNode::parse(data)?),
                            Header::Waypoints => Node::Waypoints(WaypointsNode::parse(data)?),
                            Header::Waypoint => Node::Waypoint(WaypointNode::parse(data)?),
                        })
                    }).expect("unknown header");
                //println!("{}", node);
            }
            NODE_ESC => skip = true,
            NODE_INIT => children.push(read_node(data, true)?),
            NODE_TERM => return Ok(node),
            x => println!("unused_byte: 0x{:02X}", x),
        }
    }
}
