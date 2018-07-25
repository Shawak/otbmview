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
    OtbmMapHeader = 0x00,
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

trait Node {}

impl Node for MapHeaderNode {}
impl Node for MapDataNode {}
impl Node for TileAreaNode {}
impl Node for TileNode {}
impl Node for ItemNode {}
impl Node for TownsNode {}
impl Node for TownNode {}
impl Node for HouseTileNode {}
impl Node for WaypointsNode {}
impl Node for WaypointNode {}

enum NodeType {
    MapHeader(MapHeaderNode),
    MapData(MapDataNode)
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
        Ok(MapDataNode{})
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
        Ok(TileAreaNode {
            x: 0,
            y: 0,
            z: 0
        })
    }
}

struct TileNode {
    x: u8,
    y: u8,

    //items: Vec<ItemNode>
}

impl TileNode {
    fn parse(data: &mut File) -> Result<TileNode, Error> {
        Ok(TileNode {
            x: 0,
            y: 0,
        })
    }
}

struct ItemNode {
    id: u16,

    // content: Vec<???>
}

impl ItemNode {
    fn parse(data: &mut File) -> Result<ItemNode, Error> {
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
        Ok(HouseTileNode {
            x: 0,
            y: 0,
            house_id: 0
        })
    }
}

struct WaypointsNode {
    // nodes: Vec<WaypontNode>
}

impl WaypointsNode {
    fn parse(data: &mut File) -> Result<WaypointsNode, Error> {
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
        Ok(WaypointNode {
            name: String::new(),
            x: 0,
            y: 0,
            z: 0
        })
    }
}

struct TownsNode {
    //towns: Vec<TownNode>
}

impl TownsNode {
    fn parse(data: &mut File) -> Result<TownsNode, Error> {
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
        Ok(TownNode {
            town_id: 0,
            name: String::new(),
            x: 0,
            y: 0,
            z: 0
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

            let mut header: Box<MapHeaderNode> = read_node(&mut data)?;
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

fn read_node<T: Node>(mut data: &mut File) -> Result<Box<T>, Error> {
    let mut node: Option<Box<T>> = None;
    let mut children: Vec<Box<T>> = Vec::new();

    let mut skip = false;

    while let Ok(c_byte) = data.get::<u8>() {
        if skip {
            skip = false;
            continue;
        }

        if node.is_none() && (c_byte == NODE_INIT || c_byte == NODE_TERM) {
            /*let x = match Header::from_u8(data.get::<u8>()?) {
                Some(Header::OtbmMapHeader) => Option::from(Box::from(MapHeaderNode::parse(data))),
                Some(Header::MapData) => Option::from(Box::from(MapDataNode::parse(data))),
                Some(Header::TileArea) => Option::from(Box::from(TileAreaNode::parse(data))),
                Some(Header::Tile) => Option::from(Box::from(TileNode::parse(data))),
                Some(Header::Item) => Option::from(Box::from(ItemNode::parse(data))),
                Some(Header::Towns) => Option::from(Box::from(TownsNode::parse(data))),
                Some(Header::Town) => Option::from(Box::from(TownNode::parse(data))),
                Some(Header::HouseTile) => Option::from(Box::from(HouseTileNode::parse(data))),
                Some(Header::Waypoints) => Option::from(Box::from(WaypointsNode::parse(data))),
                Some(Header::Waypoint) => Option::from(Box::from(WaypointNode::parse(data))),
                Some(_) => None,
                None => None
            };*/
            let x = data.get::<u8>()?;
            if x == 0x00 {
                node = Option::from(Box::from(MapHeaderNode::parse(data)?));
            }
        }

        if c_byte == NODE_ESC {
            skip = true;
            continue;
        }

        if c_byte == NODE_INIT {
            let mut child = read_node(&mut data)?;
            children.push(child);
        }

        if c_byte == NODE_TERM {
            break;
        }
    }

    Ok(node.unwrap())
}

fn main() {

    //let mut nodes: Vec<Node> = Vec::new();
    read_otbm().unwrap();


    println!("done");
}