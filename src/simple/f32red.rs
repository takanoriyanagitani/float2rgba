pub fn float2red<N>(f: f32, normalizer: N) -> u8
where
    N: Fn(f32) -> f32,
{
    let normalized: f32 = normalizer(f);
    let mult: f32 = normalized * 255.0;
    mult as u8
}

#[cfg(feature = "simple_red_wasm")]
pub mod wasm {
    use super::float2red;
    use crate::normalize::clampf32default;

    #[allow(unsafe_code)]
    #[no_mangle]
    pub extern "C" fn float2red_clamped(f: f32) -> u8 {
        float2red(f, clampf32default)
    }

    #[test]
    fn test_float2red_clamped() -> Result<(), String> {
        struct TestData {
            input: f32,
            expected: u8,
        }

        let data = [
            TestData {
                input: -7.0,
                expected: 0,
            },
            TestData {
                input: -1.0,
                expected: 0,
            },
            TestData {
                input: 0.0,
                expected: 0,
            },
            TestData {
                input: 0.125,
                expected: 31,
            },
            TestData {
                input: 0.25,
                expected: 63,
            },
            TestData {
                input: 0.375,
                expected: 95,
            },
            TestData {
                input: 0.5,
                expected: 127,
            },
            TestData {
                input: 1.0,
                expected: 255,
            },
            TestData {
                input: 2.0,
                expected: 255,
            },
            TestData {
                input: 5.0,
                expected: 255,
            },
        ];

        for td in data {
            let i: f32 = td.input;
            let e: u8 = td.expected;
            let got: u8 = float2red_clamped(i);
            assert_eq!(got, e, "Input: {i}, Expected: {e}, got: {got}");
        }

        Ok(())
    }
}
