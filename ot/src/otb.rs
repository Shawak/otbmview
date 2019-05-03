#![allow(dead_code)]

use std::fs::File;
use std::io::{Error, Read};
use std::collections::HashMap;

use image::Primitive;
use mem_read::*;
use binary_tree::*;

#[derive(Debug)]
pub struct Main {
    signature: u32,
    children: Vec<MainChild>
    // parse_child
}

#[derive(Debug)]
pub enum MainChild {
    Root(Root),
}

#[derive(Debug)]
pub struct Root {
    otb_major_version: u32,
    otb_minor_version: u32,
    children: Vec<ItemCategory>
    // parse_child
}

#[derive(Debug)]
pub enum ItemCategory {
    Invalid(ItemType),
    Ground(ItemType),
    Container(ItemType),
    Weapon(ItemType),
    Ammunition(ItemType),
    Armor(ItemType),
    Charges(ItemType),
    Teleport(ItemType),
    MagicField(ItemType),
    Writable(ItemType),
    Key(ItemType),
    Splash(ItemType),
    Fluid(ItemType),
    Door(ItemType),
    Deprecated(ItemType)
}

pub trait ItemCategoryTrait {
    fn item_type(&self) -> &ItemType;
}

impl ItemCategoryTrait for ItemCategory {
    fn item_type(&self) -> &ItemType {
        match self {
             ItemCategory::Invalid(val) => val,
             ItemCategory::Ground(val) => val,
             ItemCategory::Container(val) => val,
             ItemCategory::Weapon(val) => val,
             ItemCategory::Ammunition(val) => val,
             ItemCategory::Armor(val) => val,
             ItemCategory::Charges(val) => val,
             ItemCategory::Teleport(val) => val,
             ItemCategory::MagicField(val) => val,
             ItemCategory::Writable(val) => val,
             ItemCategory::Key(val) => val,
             ItemCategory::Splash(val) => val,
             ItemCategory::Fluid(val) => val,
             ItemCategory::Door(val) => val,
             ItemCategory::Deprecated(val) => val
        }
    }
}

#[derive(Debug)]
pub struct ItemType {
    server_id: u16,
    client_id: u16,
    name: String
}

impl HasChildren for Main {
    type Output = Main;
    type Child = MainChild;

    fn parse<T: MemRead>(data: &mut T, children: Vec<Self::Child>) -> Result<Self, Error> {
        //println!("parse main");

        let signature = data.get::<u32>()?;
        if signature != 0x0 {
            panic!("main signature wasn't 0x0")
        }

        Ok(Main { signature, children })
    }

    fn parse_child<T: MemRead>(data: &mut T) -> Result<Self::Child, Error> {
        //println!("parse main child");
        let identifier = data.get::<u8>()?;
        match identifier {
            0x00 => Ok(MainChild::Root(Root::read_node(data)?)),
            _ => panic!("unknown type info")
        }
    }
}

impl HasChildren for Root {
    type Output = Root;
    type Child = ItemCategory;

    fn parse<T: MemRead>(data: &mut T, children: Vec<Self::Child>) -> Result<Self, Error> {
        //println!("parse root");

        let signature = data.get::<u32>()?;
        if signature != 0x0 {
            panic!("root signature wasn't 0x0");
        }

        let root_attr = data.get::<u8>()?;
        if root_attr != 0x01 {
            panic!("invalid root attr");
        }

        let size = data.get::<u16>()?;
        if size != 4 + 4 + 4 + 128 {
            panic!("invalid root attr version size");
        }

        let otb_major_version = data.get::<u32>()?;
        let otb_minor_version = data.get::<u32>()?;
        data.get::<u32>()?; // build number
        //data.get::<[u8; 128]>()?; // description
        data.get::<u64>()?;
        data.get::<u64>()?;

        Ok(Root { otb_major_version, otb_minor_version, children })
    }

    fn parse_child<T: MemRead>(data: &mut T) -> Result<Self::Child, Error> {
        //println!("parse root children");

        let item_category = data.get::<u8>()?;
        //println!("item_category: {}", item_category);
        Ok(match item_category {
            0 => ItemCategory::Invalid(ItemType::new(data)?),
            1 => ItemCategory::Ground(ItemType::new(data)?),
            2 => ItemCategory::Container(ItemType::new(data)?),
            3 => ItemCategory::Weapon(ItemType::new(data)?),
            4 => ItemCategory::Ammunition(ItemType::new(data)?),
            5 => ItemCategory::Armor(ItemType::new(data)?),
            6 => ItemCategory::Charges(ItemType::new(data)?),
            7 => ItemCategory::Teleport(ItemType::new(data)?),
            8 => ItemCategory::MagicField(ItemType::new(data)?),
            9 => ItemCategory::Writable(ItemType::new(data)?),
            10 => ItemCategory::Key(ItemType::new(data)?),
            11 => ItemCategory::Splash(ItemType::new(data)?),
            12 => ItemCategory::Fluid(ItemType::new(data)?),
            13 => ItemCategory::Door(ItemType::new(data)?),
            14 => ItemCategory::Deprecated(ItemType::new(data)?),
            _ => panic!("unknown item category {}", item_category)
        })
    }
}

impl ItemType {
    fn new<T: MemRead>(data: &mut T) -> Result<ItemType, Error> {
        //println!("parse item type");

        let mut item_type = ItemType { server_id: 0, client_id: 0, name: "".to_string() };
        data.get::<u32>()?; // skip flags
        static mut LAST_ID: u16 = 99;
        loop {
            let attr = match data.get::<u8>() {
                Ok(attr) => attr,
                Err(_) => 0x0
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
pub struct OtbItems {
    sid_map: HashMap<u16, ItemCategory>,
    cid_map: HashMap<u16, *const ItemCategory>
}

impl OtbItems {
    fn get_sid(&mut self, sid: u16) -> &ItemCategory {
        self.sid_map.get(&sid).expect("thing type with sid not found")
    }

    fn get_cid(&mut self, cid: u16) -> &ItemCategory {
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

    let mut main = Main::read_node(data)?;
    //println!("{:?}", main);
    let root = match main.children.pop() {
        Some(x) => match x {
            MainChild::Root(a) => a
        },
        _ => panic!("root element not found")
    };

    let sid_map = root.children.into_iter().map(|x| (x.item_type().server_id, x)).collect::<HashMap<_,_>>();
    let cid_map = (&sid_map).values().map(|v| (v.item_type().client_id, v as *const _)).collect::<HashMap<_,_>>();
    let mut otb_items = OtbItems { sid_map, cid_map };

    // crystal coin
    assert_eq!(otb_items.get_sid(2160).item_type().client_id, 3043);
    assert_eq!(otb_items.get_cid(3043).item_type().server_id, 2160);

    Ok(otb_items)
}