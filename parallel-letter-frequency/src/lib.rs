use std::collections::HashMap;

pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    if input.is_empty() {
        return HashMap::new();
    }

    use std::thread;

    thread::scope(|s| {
        let mut chunk_size = input.len() / worker_count;
        if chunk_size == 0 {
            chunk_size = input.len();
        }

        let handles = input
            .chunks(chunk_size)
            .map(|chunk| {
                s.spawn(|| {
                    chunk.iter().fold::<HashMap<char, usize>, _>(
                        HashMap::with_capacity(26),
                        |mut freq, s| {
                            s.chars().filter(|&ch| ch.is_alphabetic()).for_each(|ch| {
                                *freq.entry(ch.to_ascii_lowercase()).or_default() += 1;
                            });

                            freq
                        },
                    )
                })
            })
            .collect::<Vec<_>>();

        handles
            .into_iter()
            .fold(HashMap::with_capacity(26), |mut count, handle| {
                let hm = handle.join().unwrap();
                hm.into_iter().for_each(|(ch, freq)| {
                    *count.entry(ch).or_default() += freq;
                });
                count
            })
    })
}
