#![allow(dead_code)]

use std::fs::File;
//use std::io::prelude::*;
use std::io::Error;

mod mem_type;
mod mem_read;

use mem_read::*;

#[macro_use]
extern crate enum_primitive_derive;
extern crate num_traits;

use num_traits::{FromPrimitive, ToPrimitive};

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

enum Node {
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
            Node::Tile(x) => write!(f, "Tile x: {} y: {} id: {}", x.x, x.y, x.id),
            Node::Item(x) => write!(f, "Item"),
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
    fn parse(data: &mut File) -> Result<MapHeaderNode, Error> {
        println!("parsing MapHeaderNode");
        let ret = MapHeaderNode {
            version: data.get()?,
            map_width: data.get()?,
            map_height: data.get()?,
            items_major_version: data.get()?,
            items_minor_version: data.get()?,
        };

        println!("MapWidth: {}", ret.map_width);

        Ok(ret)
    }
}

struct MapDataNode {}

impl MapDataNode {
    fn parse(data: &mut File) -> Result<MapDataNode, Error> {
        println!("parsing MapDataNode");
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
    fn parse(data: &mut File) -> Result<TileAreaNode, Error> {
        println!("parsing TileAreaNode");
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
    id: u16

    //items: Vec<ItemNode>
}

impl TileNode {
    fn parse(data: &mut File) -> Result<TileNode, Error> {
        println!("parsing TileNode");
        Ok(TileNode {
            x: 0,
            y: 0,
            id: 0
        })
    }
}

struct ItemNode {
    id: u16,

    // content: Vec<???>
}

impl ItemNode {
    fn parse(data: &mut File) -> Result<ItemNode, Error> {
        println!("parsing ItemNode");
        Ok(ItemNode {
            id: 0,
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
    fn parse(data: &mut File) -> Result<HouseTileNode, Error> {
        println!("parsing HouseTileNode");
        Ok(HouseTileNode {
            x: 0,
            y: 0,
            house_id: 0,
        })
    }
}

struct WaypointsNode {
    // nodes: Vec<WaypontNode>
}

impl WaypointsNode {
    fn parse(data: &mut File) -> Result<WaypointsNode, Error> {
        println!("parsing WaypointsNode");
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
    fn parse(data: &mut File) -> Result<WaypointNode, Error> {
        println!("parsing WaypointNode");
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
    fn parse(data: &mut File) -> Result<TownsNode, Error> {
        println!("parsing TownsNode");
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
    fn parse(data: &mut File) -> Result<TownNode, Error> {
        println!("parsing TownNode");
        Ok(TownNode {
            town_id: 0,
            name: String::new(),
            x: 0,
            y: 0,
            z: 0,
        })
    }
}


fn read_otbm() -> Result<(), Error> {
    let file = File::open("map.otbm");
    match file {
        Ok(mut data) => {
            let map_identifier: u32 = data.get()?;
            if map_identifier != 0x0 && map_identifier != 0x4D42544F {
                panic!("unknown OTBM format: unexpected magic bytes.");
            }

            let mut header: Option<Node> = read_node(&mut data, false)?;
        }
        Err(e) => {
            panic!(e);
        }
    }
    Ok(())
}

const NODE_ESC: u8 = 0xFD;
const NODE_INIT: u8 = 0xFE;
const NODE_TERM: u8 = 0xFF;

fn read(data: &mut File) -> Result<Vec<u8>, Error> {
    use std::io::Read;

    let mut v: Vec<u8> = Vec::new();
    data.read_to_end(&mut v)?;
    Ok(v)
}

fn read_node(data: &mut File, is_child: bool) -> Result<Option<Node>, Error> {
    let mut node: Option<Node> = None;
    let mut children: Vec<Node> = Vec::new();

    let mut skip = false;
    let mut first = true;

    while let Ok(c_byte) = if is_child && first { first = false; Ok(0xFE) } else { data.get::<u8>() } {
        println!("c_byte: {:#X} {}", c_byte, node.is_none());

        if skip {
            skip = false;
            continue;
        }

        let mut x = false;

        if node.is_none() && (c_byte == NODE_INIT || c_byte == NODE_TERM) {
            println!("node found!");

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
                None => panic!("unknown header? 2 - {:X} {:#X?}", identifier, read(data)?),
            };
            println!("{}", node.is_none());

            if let Some(node) = &node {
                println!("{}", node);
            } else {
                println!("could not print node");
            }

            continue;
        }

        if c_byte == NODE_ESC {
            skip = true;
            continue;
        }

        if c_byte == NODE_INIT && !x {
            println!("start parsing children");
            let mut child = read_node(data, true)?;
            if let Some(child) = child {
                children.push(child);
            }
            continue;
        }

        if c_byte == NODE_TERM {
            //break;
            return Ok(node);
        }
    }

    //Ok(node)
    Ok(None)
}

fn main() {

    //let mut nodes: Vec<Node> = Vec::new();
    read_otbm().unwrap();


    println!("done");
}