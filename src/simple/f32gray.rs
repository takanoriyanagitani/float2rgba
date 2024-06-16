pub fn float2gray_rgba<N>(f: f32, normalizer: N) -> (u8, u8, u8, u8)
where
    N: Fn(f32) -> f32,
{
    let normalized: f32 = normalizer(f);
    let mult: f32 = 255.0 * normalized;
    let u: u8 = mult as u8;
    (u, u, u, 255)
}

#[cfg(feature = "simple_gray_wasm")]
pub mod wasm {
    use super::float2gray_rgba;
    use crate::normalize::clampf32default;

    #[allow(unsafe_code)]
    #[no_mangle]
    pub extern "C" fn float2gray_rgba_clamped(f: f32) -> u32 {
        let (r, g, b, a) = float2gray_rgba(f, clampf32default);

        let r5: u32 = r.into();
        let g5: u32 = g.into();
        let b5: u32 = b.into();
        let a5: u32 = a.into();

        (r5 << 24) | (g5 << 16) | (b5 << 8) | a5
    }

    #[test]
    fn test_float2gray_rgba_clamped() -> Result<(), String> {
        struct TestData {
            input: f32,
            expected: u32,
        }

        let test_data = [
            TestData {
                input: -5.0,
                expected: 0x0000_00ff,
            },
            TestData {
                input: -1.0,
                expected: 0x0000_00ff,
            },
            TestData {
                input: 0.0,
                expected: 0x0000_00ff,
            },
            TestData {
                input: 1.0,
                expected: 0xffff_ffff,
            },
            TestData {
                input: 0.5,
                expected: 0x7f7f_7fff,
            },
            TestData {
                input: 0.25,
                expected: 0x3f3f_3fff,
            },
        ];

        for data in test_data {
            let input: f32 = data.input;
            let expected: u32 = data.expected;
            let got: u32 = float2gray_rgba_clamped(input);
            assert_eq!(
                got, expected,
                "Input: {input}, Expected: {expected}, Got: {got}"
            );
        }

        Ok(())
    }
}
