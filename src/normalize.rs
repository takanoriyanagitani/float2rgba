pub fn clampf32new(min: f32, max: f32) -> impl Fn(f32) -> f32 {
    move |original: f32| original.clamp(min, max)
}

pub fn clampf32default(original: f32) -> f32 {
    clampf32new(0.0, 1.0)(original)
}
