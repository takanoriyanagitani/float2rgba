use std::sync::RwLock;

use colorgrad::Color;
use colorgrad::Gradient;

use super::wasm::color2u32;

static RAINBOW: RwLock<Option<Gradient>> = RwLock::new(None);

#[allow(unsafe_code)]
#[no_mangle]
pub extern "C" fn ext_colorgrad_rainbow_init() -> i32 {
    match RAINBOW.write() {
        Err(_) => -1,
        Ok(mut guard) => {
            let mo: &mut Option<_> = &mut guard;
            mo.replace(colorgrad::rainbow());
            0
        }
    }
}

#[allow(unsafe_code)]
#[no_mangle]
pub extern "C" fn float2rgba32u_ext_rainbow(f: f32) -> u32 {
    let f: f64 = f.into();
    match RAINBOW.read() {
        Err(_) => 0,
        Ok(guard) => {
            let og: &Option<_> = &guard;
            match og {
                None => 0,
                Some(g) => {
                    let c: Color = g.at(f);
                    color2u32(c)
                }
            }
        }
    }
}
