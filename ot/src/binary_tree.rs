use std::io::Error;
use mem_read::*;

const NODE_ESCAPE: u8 = 0xFD;
const NODE_START: u8 = 0xFE;
const NODE_END: u8 = 0xFF;

pub trait BinaryTree {
    type Output;
    type Child;

    fn read_node<T: MemRead>(data: &mut T) -> Result<Self::Output, Error>;
}

pub trait HasChildren {
    type Output;
    type Child;

    fn parse<T: MemRead>(data: &mut T, children: Vec<Self::Child>) -> Result<Self::Output, Error>;
    fn parse_child<T: MemRead>(data: &mut T) -> Result<Self::Child, Error>;
}

impl <D: HasChildren> BinaryTree for D {
    type Output = D::Output;
    type Child = D::Child;

    fn read_node<T: MemRead>(data: &mut T) -> Result<Self::Output, Error> {
        //println!("read_node");

        let mut buffer: Vec<u8> = Vec::new();
        let mut children: Vec<Self::Child> = Vec::new();

        loop {
            let byte = match data.get::<u8>() {
                Ok(b) => b,
                Err(_) => break
            };

            match byte {
                NODE_START => {
                    children.push({

                        let mut child_buffer: Vec<u8> = Vec::new();
                        let mut depth = 0;

                        loop {
                            let s = match data.get::<u8>() {
                                Ok(s) => s,
                                Err(_) => break ()
                            };

                            match s {
                                NODE_START =>{
                                    depth+=1;
                                    child_buffer.push(s);
                                },
                                NODE_END => {
                                    if depth == 0 {
                                        break;
                                    } else {
                                        child_buffer.push(s);
                                    }
                                },
                                NODE_ESCAPE => {
                                    if depth == 0 {
                                        child_buffer.push(data.get::<u8>()?)
                                    } else {
                                        child_buffer.push(s);
                                        child_buffer.push(data.get::<u8>()?)
                                    }
                                },
                                _ => {
                                    child_buffer.push(s)
                                }
                            }
                        }

                        //println!("depth: {}", depth);
                        //println!("child_buffer: {:?}", &child_buffer[0..std::cmp::min(100, child_buffer.len())]);
                        Self::parse_child(&mut child_buffer.as_ref() as &mut &[u8])?

                    })
                },
                NODE_END => {
                    break;
                },
                NODE_ESCAPE => {
                    buffer.push(data.get::<u8>()?)
                },
                _ => buffer.push(byte)
            }
        }

        //println!("parent_buffer: {:?}", &buffer[0..std::cmp::min(100, buffer.len())]);
        Ok(Self::parse(&mut buffer.as_ref() as &mut &[u8], children)?)
    }
}