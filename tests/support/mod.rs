use rand::Rng;

pub fn given_a_multiple_of_three() -> usize {
    given_a_multiple_of(3)
}
pub fn given_a_multiple_of_five() -> usize {
    given_a_multiple_of(5)
}

fn given_a_multiple_of(number: usize) -> usize {
    number * rand::thread_rng().gen_range(1..100)
}
