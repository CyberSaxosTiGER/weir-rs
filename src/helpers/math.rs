pub fn linspace(start: f32, end: f32, steps: u32) -> impl Iterator<Item = f32> + 'static {
    let step_size = (end - start) / ((steps - 1) as f32);
    (1..steps).scan(0.0, move |acc, _| {
        *acc += step_size;
        Some(*acc)
    })
}
