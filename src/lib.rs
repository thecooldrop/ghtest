#![deny(warnings)]
#![allow(clippy::many_single_char_names)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::float_cmp)]

mod level1;
pub use level1::*;

mod level2;
pub use level2::*;

mod level3;
pub use level3::*;

#[derive(Clone, Copy)]
struct SSend(*const f32);

unsafe impl Send for SSend {}
unsafe impl Sync for SSend {}

#[derive(Clone, Copy)]
struct SSendMut(*mut f32);

unsafe impl Send for SSendMut {}
unsafe impl Sync for SSendMut {}
