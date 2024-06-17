use core::ptr::{addr_of, addr_of_mut};

static mut INPUT_FLOAT: Vec<f32> = vec![];
static mut OUTPUT_RGBA: Vec<u32> = vec![];

extern "C" {
    fn float2rgba(f: f32) -> u32;
}

#[allow(unsafe_code)]
#[no_mangle]
pub extern "C" fn cnvs_imgdat_ext_input_ptr() -> *const f32 {
    #[allow(unsafe_code)]
    let pcv: *const Vec<f32> = unsafe { addr_of!(INPUT_FLOAT) };

    #[allow(unsafe_code)]
    let oi: Option<_> = unsafe { pcv.as_ref() };

    oi.map(|i| i.as_ptr()).unwrap_or_else(core::ptr::null)
}

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

pub fn input_init(sz: usize, val: f32) -> Result<usize, &'static str> {
    #[allow(unsafe_code)]
    let pmv: *mut Vec<f32> = unsafe { addr_of_mut!(INPUT_FLOAT) };

    #[allow(unsafe_code)]
    let ov: Option<_> = unsafe { pmv.as_mut() };
    let v: &mut Vec<_> = ov.ok_or_else(|| "input unavailable")?;

    init32f(sz, val, v)
}

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
    let v: &mut Vec<_> = ov.ok_or_else(|| "output unavailable")?;

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

pub fn conv() -> Result<u64, &'static str> {
    #[allow(unsafe_code)]
    let pcv: *const Vec<f32> = unsafe { addr_of!(INPUT_FLOAT) };

    #[allow(unsafe_code)]
    let oi: Option<_> = unsafe { pcv.as_ref() };
    let i: &[f32] = oi.ok_or_else(|| "input unavailable")?;

    #[allow(unsafe_code)]
    let pmv: *mut Vec<u32> = unsafe { addr_of_mut!(OUTPUT_RGBA) };

    #[allow(unsafe_code)]
    let ow: Option<_> = unsafe { pmv.as_mut() };
    let o: &mut Vec<u32> = ow.ok_or_else(|| "output unavailable")?;

    let cnv = |f: f32| {
        #[allow(unsafe_code)]
        unsafe {
            float2rgba(f)
        }
    };

    Ok(convert(i, o, &cnv))
}

#[allow(unsafe_code)]
#[no_mangle]
pub extern "C" fn cnvs_imgdat_ext_convert() -> i32 {
    conv().map(|u| u as i32).unwrap_or(-1)
}
