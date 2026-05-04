#![no_main]
use libfuzzer_sys::fuzz_target;
use raven_core::cosine_similarity;

fuzz_target!(|data: &[u8]| {
    // Interpret raw bytes as pairs of f32 vectors
    if data.len() < 8 {
        return;
    }
    let floats: Vec<f32> = data
        .chunks_exact(4)
        .map(|chunk| f32::from_le_bytes([chunk[0], chunk[1], chunk[2], chunk[3]]))
        .filter(|f| f.is_finite())
        .collect();

    if floats.len() < 2 {
        return;
    }

    let half = floats.len() / 2;
    let a = &floats[..half];
    let b = &floats[half..half * 2];

    let sim = cosine_similarity(a, b);

    // Must not panic, must not be NaN
    assert!(!sim.is_nan(), "cosine_similarity returned NaN");
});
