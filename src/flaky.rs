#[cfg(test)]
mod tests {
    use rand::Rng;

    #[test]
    fn rand() {
        if rand::thread_rng().gen_range(0.0..1.0) > 0.8 {
            assert!(false);
        } else {
            assert!(true);
        }
    }
}
