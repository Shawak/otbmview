#![allow(dead_code)]

use std::fs::File;
//use std::io::prelude::*;
use std::io::{Error, Cursor, Read, Write};

use std;

use num_traits::{FromPrimitive, ToPrimitive};

use mem_read::*;
use mem_type::*;

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
            Node::MapHeader(x) => write!(f, "MapHeader - Width: {} Height: {}", x.map_width, x.map_height),
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

struct MapHeaderNode {
    version: u32,
    map_width: u16,
    map_height: u16,
    items_major_version: u32,
    items_minor_version: u32,
}

impl MapHeaderNode {
    fn parse(data: &mut Cursor<Vec<u8>>) -> Result<MapHeaderNode, Error> {
        Ok(MapHeaderNode {
            version: data.get()?,
            map_width: data.get()?,
            map_height: data.get()?,
            items_major_version: data.get()?,
            items_minor_version: data.get()?,
        })
    }
}

struct MapDataNode {}

impl MapDataNode {
    fn parse(data: &mut Cursor<Vec<u8>>) -> Result<MapDataNode, Error> {
        Ok(MapDataNode {})
    }
}

struct TileAreaNode {
    x: u16,
    y: u16,
    z: u8,

    //tiles: Vec<TileNode>
}

impl TileAreaNode {
    fn parse(data: &mut Cursor<Vec<u8>>) -> Result<TileAreaNode, Error> {
        Ok(TileAreaNode {
            x: data.get()?,
            y: data.get()?,
            z: data.get()?,
        })
    }
}

struct TileNode {
    x: u8,
    y: u8,

    //items: Vec<ItemNode>
}

impl TileNode {
    fn parse(data: &mut Cursor<Vec<u8>>) -> Result<TileNode, Error> {
        Ok(TileNode {
            x: data.get()?,
            y: data.get()?
        })
    }
}

struct ItemNode {
    id: u16,

    // content: Vec<???>
}

impl ItemNode {
    fn parse(data: &mut Cursor<Vec<u8>>) -> Result<ItemNode, Error> {
        Ok(ItemNode {
            id: data.get()?,
        })
    }
}

struct HouseTileNode {
    x: u16,
    y: u16,
    house_id: u32,

    //items: Vec<ItemNode>
}

impl HouseTileNode {
    fn parse(data: &mut Cursor<Vec<u8>>) -> Result<HouseTileNode, Error> {
        Ok(HouseTileNode {
            x: data.get()?,
            y: data.get()?,
            house_id: data.get()?,
        })
    }
}

struct WaypointsNode {
    // nodes: Vec<WaypontNode>
}

impl WaypointsNode {
    fn parse(data: &mut Cursor<Vec<u8>>) -> Result<WaypointsNode, Error> {
        Ok(WaypointsNode {})
    }
}

struct WaypointNode {
    name: String,
    x: u16,
    y: u16,
    z: u8,
}

impl WaypointNode {
    fn parse(data: &mut Cursor<Vec<u8>>) -> Result<WaypointNode, Error> {
        Ok(WaypointNode {
            name: String::new(),
            x: 0,
            y: 0,
            z: 0,
        })
    }
}

struct TownsNode {
    //towns: Vec<TownNode>
}

impl TownsNode {
    fn parse(data: &mut Cursor<Vec<u8>>) -> Result<TownsNode, Error> {
        Ok(TownsNode {})
    }
}

struct TownNode {
    town_id: u32,
    name: String,
    x: u16,
    y: u16,
    z: u8,
}

impl TownNode {
    fn parse(data: &mut Cursor<Vec<u8>>) -> Result<TownNode, Error> {
        Ok(TownNode {
            town_id: data.get()?,
            name: String::new(),
            x: 0,
            y: 0,
            z: 0,
        })
    }
}

pub fn read_otbm(filename: String) -> Result<Option<Node>, Error> {
    let mut file = File::open(filename)?;
    let mut data: Vec<u8> = Vec::new();
    file.read_to_end(&mut data)?;
    let mut data = Cursor::new(data);

    let map_identifier: u32 = data.get()?;
    if map_identifier != 0x0 && map_identifier != 0x4D42544F {
        panic!("unknown OTBM format: unexpected magic bytes.");
    }

    let mut header: Option<Node> = read_node(&mut data, false)?;
    Ok(header)
}

fn read_node(data: &mut Cursor<Vec<u8>>, is_child: bool) -> Result<Option<Node>, Error> {
    let mut node: Option<Node> = None;
    let mut children: Vec<Node> = Vec::new();

    let mut skip = false;
    let mut first = true;

    while let Ok(c_byte) = if is_child && first { first = false; Ok(0xFE) } else { data.get::<u8>() } {
        if skip {
            skip = false;
            continue;
        }

        if node.is_none() {
            if c_byte == NODE_INIT || c_byte == NODE_TERM {
                let identifier = data.get::<u8>()?;
                node = match Header::from_u8(identifier) {
                    Some(Header::MapHeader) => Option::from(Node::MapHeader(MapHeaderNode::parse(data)?)),
                    Some(Header::MapData) => Option::from(Node::MapData(MapDataNode::parse(data)?)),
                    Some(Header::TileArea) => Option::from(Node::TileArea(TileAreaNode::parse(data)?)),
                    Some(Header::Tile) => Option::from(Node::Tile(TileNode::parse(data)?)),
                    Some(Header::Item) => Option::from(Node::Item(ItemNode::parse(data)?)),
                    Some(Header::Towns) => Option::from(Node::Towns(TownsNode::parse(data)?)),
                    Some(Header::Town) => Option::from(Node::Town(TownNode::parse(data)?)),
                    Some(Header::HouseTile) => Option::from(Node::HouseTile(HouseTileNode::parse(data)?)),
                    Some(Header::Waypoints) => Option::from(Node::Waypoints(WaypointsNode::parse(data)?)),
                    Some(Header::Waypoint) => Option::from(Node::Waypoint(WaypointNode::parse(data)?)),
                    None => panic!("unknown header 0x{:02X}", identifier),
                };

                if let Some(node) = &node {
                    println!("{}", node);
                } else {
                    println!("could not print node");
                }

                continue;
            }
        }

        if c_byte == NODE_ESC {
            skip = true;
            continue;
        }

        if c_byte == NODE_INIT {
            let mut child = read_node(data, true)?;
            if let Some(child) = child {
                children.push(child);
            }
            continue;
        }

        if c_byte == NODE_TERM {
            return Ok(node);
        }

        //println!("unused_byte: 0x{:02X}", c_byte);
    }

    //Ok(node)
    Ok(None)
}