use metacore::Record;
use rand::Rng;

pub fn create_random_record(dimension: usize) -> Record {
    let mut rng = rand::thread_rng();
    Record::new(
        format!("id_{}", rng.gen::<u32>()),
        (0..dimension).map(|_| rng.gen_range(-10.0..10.0)).collect(),
    )
}
