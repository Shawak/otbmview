pub trait MemType {}

impl MemType for u8 {}
impl MemType for u16 {}
impl MemType for u32 {}
impl MemType for u64 {}

impl MemType for i8 {}
impl MemType for i16 {}
impl MemType for i32 {}
impl MemType for i64 {}