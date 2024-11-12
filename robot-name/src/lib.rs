pub struct Robot {
    name: String,
}

use once_cell::sync::Lazy;
use std::collections::HashSet;
use std::sync::Mutex;

static GENERATED_NAMES: Lazy<Mutex<HashSet<String>>> = Lazy::new(|| Mutex::new(HashSet::new()));

impl Robot {
    pub fn new() -> Self {
        Self {
            name: Self::gen_name(),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn reset_name(&mut self) {
        let old_name = std::mem::replace(&mut self.name, Self::gen_name());
        GENERATED_NAMES.lock().unwrap().remove(&old_name);
    }

    fn gen_name() -> String {
        use rand::Rng;

        let mut rng = rand::thread_rng();
        let mut name = String::with_capacity(5);

        let mut generated_names = GENERATED_NAMES.lock().unwrap();

        loop {
            (0..2).for_each(|_| name.push(rng.gen_range(b'A'..=b'Z') as char));
            name.push_str(&format!("{:03}", rng.gen_range(0..=999)));

            if !generated_names.contains(&name) {
                generated_names.insert(name.clone());
                drop(generated_names);
                break;
            }

            name.clear();
        }

        name
    }
}
