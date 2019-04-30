#![allow(dead_code)]

use std::fs::File;
use std::io::{Error, ErrorKind, Read};
use std::collections::HashMap;

use num_traits::{FromPrimitive, ToPrimitive};

use mem_read::*;
use image::Primitive;
use dat::ThingCategory;

const NODE_ESCAPE: u8 = 0xFD;
const NODE_START: u8 = 0xFE;
const NODE_END: u8 = 0xFF;

/*#[derive(Primitive)]
enum ItemCategoryHeader {
    Invalid = 0,
    Ground = 1,
    Container = 2,
    Weapon = 3,
    Ammunition = 4,
    Armor = 5,
    Charges = 6,
    Teleport = 7,
    MagicField = 8,
    Writable = 9,
    Key = 10,
    Slash = 11,
    Fluid = 12,
    Door = 13,
    Deprecated = 14,
    Last = 15
}

#[derive(PartialEq, Debug)]
pub enum Node {
    Unknown,
    Root(RootNode),
    Ground(GroundNode),
}

#[derive(PartialEq, Debug)]
pub struct GroundNode {

}

impl GroundNode {
    fn parse<T: MemRead>(data: &mut T) -> Result<GroundNode, Error> {
        Ok(GroundNode {

        })
    }
}

#[derive(Primitive)]
enum RootNodeHeader {
    RootNode = 0x01
}

#[derive(PartialEq, Debug)]
pub struct RootNode {
    size: u16,
    otb_major_version: u32,
    otb_minor_version: u32,
    build_number: u32,
    description: u128
}

impl RootNode {
    fn parse<T: MemRead>(data: &mut T) -> Result<RootNode, Error> {
        Ok(RootNode {
            size: data.get()?,
            otb_major_version: data.get()?,
            otb_minor_version: data.get()?,
            build_number: data.get()?,
            description: data.get()?
        })
    }
}

fn read_node<T: MemRead>(data: &mut T, is_child: bool) -> Result<Node, Error> {
    println!("read_node");
    Ok((Node::Unknown))

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

        println!("c_byte: {} ", c_byte);

        match c_byte {
            NODE_START | NODE_END if node == Node::Unknown => {
                let mut identifier = data.get::<u8>()?;
                println!("identifier: {}", identifier);
                node = match RootNodeHeader::from_u8(identifier).expect("from_u8 failed") {
                    RootNodeHeader::RootNode => Node::Root(RootNode::parse(data)?),
                    _ => Node::Unknown
                };
                println!("{:?}", node);
            }
            NODE_ESCAPE => skip = true,
            NODE_START => children.push(read_node(data, true)?),
            NODE_END => return Ok(node),
            x => () // println!("unused_byte: 0x{:02X}", x)
        }
    }
}*/

#[derive(Debug)]
struct ItemType {
    category: u8,
    server_id: u16,
    client_id: u16,
    name: String
}

impl ItemType {
    fn new<T: MemRead>(data: &mut T) -> Result<ItemType, Error> {
        let mut item_type = ItemType { category: 0, server_id: 0, client_id: 0, name: "".to_string() };
        let category = data.get::<u8>()?;
        //println!("{}", category);
        item_type.category = category;
        data.get::<u32>()?; // skip flags
        static mut LAST_ID: u16 = 99;
        loop {
            let attr = match data.get::<u8>() {
                Ok(attr) => attr,
                Err(e) => 0x0
            };

            if attr == 0x0 || attr == 0xFF {
                return Ok(item_type)
            }

            let len = data.get::<u16>()?;
            //println!("attr: {} len: {}", attr, len);
            match attr {
                16 /* ItemTypeAttrServerId */ => {
                    let mut server_id = data.get::<u16>()?;
                    unsafe {
                        if server_id > 30000 && server_id < 30100 {
                            server_id -= 30000;
                        } else if LAST_ID > 99 && LAST_ID != server_id - 1 {
                            while LAST_ID != server_id -1 {
                                // TODO: add item types https://github.com/edubart/otclient/blob/1addf3e1766ca3fe43bdf1114c0655a971123291/src/client/itemtype.cpp#L69
                                LAST_ID += 1;
                            }
                        }
                        LAST_ID = server_id;
                    }
                    item_type.server_id = server_id;
                }
                17 /*ItemTypeAttrClientId*/ => item_type.client_id = data.get::<u16>()?,
                18 /*ItemTypeAttrName DEPRECATED?*/ => data.skip(len as _), //item_type.name = data.get_str(len as _)?,
                _ => data.skip(len as _) // skip irrelevant attributes
            }
        }
    }
}

#[derive(Debug)]
struct Root {
    version: u8,
    children: Vec<ItemType>
}

impl Root {
    fn new<T: MemRead>(data: &mut T) -> Result<Root, Error> {
        let mut root = Root {
            version: 0,
            children: Vec::new()
        };
        root.children = root.read_nodes(data)?;
        Ok(root)
    }
}

trait HasChildren {
    type Child;

    fn parse_child<T: MemRead>(&mut self, data: &mut T) -> Result<Self::Child, Error>;
    fn read_nodes<T: MemRead>(&mut self, data: &mut T) -> Result<Vec<Self::Child>, Error>;
}

impl HasChildren for Root {
    type Child = ItemType;

    fn parse_child<T: MemRead>(&mut self, data: &mut T) -> Result<Self::Child, Error> {
        /*let mut identifier = data.get::<u8>()?;
        match identifier {
            0x01 => (),
            _ => ()
        }*/
        let child = ItemType::new(data)?;
        Ok(child)
    }

    fn read_nodes<T: MemRead>(&mut self, data: &mut T) -> Result<Vec<Self::Child>, Error> {
        let mut children: Vec<Self::Child> = Vec::new();
        let mut parsing_child = false;
        let mut buffer: Vec<u8> = Vec::new();
        loop {
            let byte = data.get::<u8>()?;
            //print!("byte: {} ", byte);
            match byte {
                NODE_START => {
                    parsing_child = true;
                    /*let child = self.parse_child(data)?;
                    children.push(child);*/
                    buffer.clear();
                    //println!("start");
                },
                NODE_END => {
                    if parsing_child {
                        let dat: &mut &[u8] = &mut buffer.as_ref();
                        //println!("end");
                        let child = self.parse_child(dat)?;
                        children.push(child);
                        //println!("end2");
                        parsing_child = false;
                        buffer.clear();
                    } else {
                        return Ok(children)
                    }
                },
                NODE_ESCAPE => {
                    //println!("esc");
                    buffer.push(data.get::<u8>()?);
                },
                _ => {
                    //println!("push");
                    buffer.push(byte)
                }// panic!("unused_byte: 0x{:02X}, aborting!")
            }
        }
    }
}

/*fn read_nodes<T: MemRead, C: HasChildren>(data: &mut T, owner: &mut C) -> Result<(), Error> {
    loop {
        let byte = data.get::<u8>()?;
        match byte {
            NODE_START => owner.parse_child(data),
            NODE_END => return Ok(()),
            NODE_ESCAPE => data.get::<u8>(), // skip 1 byte
            _ => panic!("unused_byte: 0x{:02X}, aborting!")
        }
    }
}*/

#[derive(Debug)]
pub struct OtbItems {
    sid_map: HashMap<u16, ItemType>,
    cid_map: HashMap<u16, *const ItemType>
}

impl OtbItems {
    fn get_sid(&mut self, sid: u16) -> &ItemType {
        self.sid_map.get(&sid).expect("thing type with sid not found")
    }

    fn get_cid(&mut self, cid: u16) -> &ItemType {
        unsafe {
            &**(self.cid_map.get(&cid).expect("thing type with cid not found"))
        }
    }
}

pub fn parse(filename: String) -> Result<OtbItems, Error> {
    let mut file = File::open(filename)?;
    let mut data: Vec<u8> = Vec::new();
    file.read_to_end(&mut data)?;
    let data: &mut &[u8] = &mut data.as_ref();

    /*let preview = &mut data.clone();
    for k in 1..10 {
        for n in 0..50 {
            print!("{} ", preview.get::<u8>()?);
        }
        println!();
    }*/

    let mut signature = data.get::<u32>()?;
    if signature != 0x0 {
        panic!("invalid otb file, signature wasn't 0x0 (1)");
    }

    assert_eq!(data.get::<u8>()?, NODE_START); // skip first node start
    assert_eq!(data.get::<u8>()?, 0x0); // skip first byte always 0 (type info?)

    signature = data.get::<u32>()?;
    if signature != 0x0 {
        panic!("invalid otb file, signature wasn't 0x0 (2)");
    }

    let root_attr = data.get::<u8>()?;
    if root_attr == 0x01 {
        let size = data.get::<u16>()?;
        if size != 4 + 4 + 4 + 128 {
            panic!("invalid otb root attr version size");
        }

        let otb_major_version = data.get::<u32>()?;
        let otb_minor_version = data.get::<u32>()?;
        data.get::<u32>()?; // build number
        //data.get::<[u8; 128]>()?; // description
        data.get::<u64>()?;
        data.get::<u64>()?;
    }

    /*let mut next: Vec<u8> = [0..100].to_vec().into_iter().map(|n| {
        data.get::<u8>()
    }).rev().collect::<Result<_, Error>>()?;

    println!("{:?}", next);*/

    let root_node = Root::new(data)?;
    //println!("{:?}", root_node);
    //println!("{:?}", root_node.children.len());
    //read_node(data, true);

    let sid_map = root_node.children.into_iter().map(|x| (x.server_id, x)).collect::<HashMap<_,_>>();
    let cid_map = (&sid_map).values().map(|v| (v.client_id, v as *const _)).collect::<HashMap<_,_>>();
    let mut otb_items = OtbItems { sid_map, cid_map };

    // crystal coin
    assert_eq!(otb_items.get_sid(2160).client_id, 3043);
    assert_eq!(otb_items.get_cid(3043).server_id, 2160);

    Ok(otb_items)
}