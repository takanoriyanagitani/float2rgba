//! Functions using "turbo" color map.

use std::sync::RwLock;

use colorgrad::Color;
use colorgrad::Gradient;

use super::wasm::color2u32;

static TURBO: RwLock<Option<Gradient>> = RwLock::new(None);

#[allow(unsafe_code)]
#[no_mangle]
pub extern "C" fn ext_colorgrad_turbo_init() -> i32 {
    match TURBO.write() {
        Err(_) => -1,
        Ok(mut guard) => {
            let mo: &mut Option<_> = &mut guard;
            mo.replace(colorgrad::turbo());
            0
        }
    }
}

#[allow(unsafe_code)]
#[no_mangle]
pub extern "C" fn float2rgba32u_ext_turbo(f: f32) -> u32 {
    let f: f64 = f.into();
    match TURBO.read() {
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
