//! Bulk color conversion using external color converter.

use core::ptr::{addr_of, addr_of_mut};

// Wasm runtime should not use threads for this module.
static mut INPUT_FLOAT: Vec<f32> = vec![];
static mut OUTPUT_RGBA: Vec<u32> = vec![];

extern "C" {
    // external color converter(float -> RGBA)
    fn float2rgba(f: f32) -> u32;
}

/// The pointer(offset integer @ WASM) to the input f32 vector.
#[allow(unsafe_code)]
#[no_mangle]
pub extern "C" fn cnvs_imgdat_ext_input_ptr() -> *mut f32 {
    #[allow(unsafe_code)]
    let pmv: *mut Vec<f32> = unsafe { addr_of_mut!(INPUT_FLOAT) };

    #[allow(unsafe_code)]
    let oi: Option<_> = unsafe { pmv.as_mut() };

    oi.map(|i| i.as_mut_ptr())
        .unwrap_or_else(core::ptr::null_mut)
}

/// The pointer(offset integer @ WASM) to the output u32 vector.
#[allow(unsafe_code)]
#[no_mangle]
pub extern "C" fn cnvs_imgdat_ext_output_ptr() -> *const u32 {
    #[allow(unsafe_code)]
    let pcv: *const Vec<u32> = unsafe { addr_of!(OUTPUT_RGBA) };

    #[allow(unsafe_code)]
    let oi: Option<_> = unsafe { pcv.as_ref() };

    oi.map(|i| i.as_ptr()).unwrap_or_else(core::ptr::null)
}

pub fn init32f(sz: usize, val: f32, v: &mut Vec<f32>) -> Result<usize, &'static str> {
    let cap: usize = v.len();
    let add: usize = sz.saturating_sub(cap);
    v.try_reserve(add).map_err(|_| "out of memory")?;

    v.clear();
    for _ in 0..sz {
        v.push(val);
    }

    Ok(v.capacity())
}

/// Initializes the input vector and fill it with the specified value.
pub fn input_init(sz: usize, val: f32) -> Result<usize, &'static str> {
    #[allow(unsafe_code)]
    let pmv: *mut Vec<f32> = unsafe { addr_of_mut!(INPUT_FLOAT) };

    #[allow(unsafe_code)]
    let ov: Option<_> = unsafe { pmv.as_mut() };
    let v: &mut Vec<_> = ov.ok_or("input unavailable")?;

    init32f(sz, val, v)
}

/// Clears the output vector after resizing it.
pub fn reset32u(sz: usize, v: &mut Vec<u32>) -> Result<usize, &'static str> {
    let cap: usize = v.len();
    let add: usize = sz.saturating_sub(cap);
    v.try_reserve(add).map_err(|_| "out of memory")?;

    v.clear();
    Ok(v.capacity())
}

pub fn output_reset(sz: usize) -> Result<usize, &'static str> {
    #[allow(unsafe_code)]
    let pmv: *mut Vec<u32> = unsafe { addr_of_mut!(OUTPUT_RGBA) };

    #[allow(unsafe_code)]
    let ov: Option<_> = unsafe { pmv.as_mut() };
    let v: &mut Vec<_> = ov.ok_or("output unavailable")?;

    reset32u(sz, v)
}

#[allow(unsafe_code)]
#[no_mangle]
pub extern "C" fn cnvs_imgdat_ext_input_init(sz: i32, val: f32) -> i32 {
    input_init(sz as usize, val).map(|u| u as i32).unwrap_or(-1)
}

#[allow(unsafe_code)]
#[no_mangle]
pub extern "C" fn cnvs_imgdat_ext_output_reset(sz: i32) -> i32 {
    output_reset(sz as usize).map(|u| u as i32).unwrap_or(-1)
}

pub fn convert<C>(input: &[f32], output: &mut Vec<u32>, converter: &C) -> u64
where
    C: Fn(f32) -> u32,
{
    output.clear();
    input.iter().fold(0, |state, next| {
        let f: f32 = *next;
        let converted: u32 = converter(f);
        output.push(converted);
        state + 1
    })
}

pub fn conv_swap(swap: bool) -> Result<u64, &'static str> {
    #[allow(unsafe_code)]
    let pcv: *const Vec<f32> = unsafe { addr_of!(INPUT_FLOAT) };

    #[allow(unsafe_code)]
    let oi: Option<_> = unsafe { pcv.as_ref() };
    let i: &[f32] = oi.ok_or("input unavailable")?;

    #[allow(unsafe_code)]
    let pmv: *mut Vec<u32> = unsafe { addr_of_mut!(OUTPUT_RGBA) };

    #[allow(unsafe_code)]
    let ow: Option<_> = unsafe { pmv.as_mut() };
    let o: &mut Vec<u32> = ow.ok_or("output unavailable")?;

    let cnv_unsafe = |f: f32| {
        #[allow(unsafe_code)]
        unsafe {
            float2rgba(f)
        }
    };

    let cnv_swap = |f: f32| {
        let u: u32 = cnv_unsafe(f);
        u.swap_bytes()
    };

    match swap {
        false => Ok(convert(i, o, &cnv_unsafe)),
        true => Ok(convert(i, o, &cnv_swap)),
    }
}

pub fn conv() -> Result<u64, &'static str> {
    conv_swap(false)
}

#[allow(unsafe_code)]
#[no_mangle]
pub extern "C" fn cnvs_imgdat_ext_convert_swap(swap: bool) -> i32 {
    conv_swap(swap).map(|u| u as i32).unwrap_or(-1)
}

#[allow(unsafe_code)]
#[no_mangle]
pub extern "C" fn cnvs_imgdat_ext_convert() -> i32 {
    cnvs_imgdat_ext_convert_swap(false)
}
