// Copyright (c) 2021 Murilo Ijanc' <mbsd@m0x.ru>
//
// Permission to use, copy, modify, and distribute this software for any
// purpose with or without fee is hereby granted, provided that the above
// copyright notice and this permission notice appear in all copies.
//
// THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES
// WITH REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF
// MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR
// ANY SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES
// WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN
// ACTION OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF
// OR IN CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.
//

use rand::prelude::*;
use std::collections::HashMap;
use std::env;
use std::hash;
use std::process;
use std::thread;
use std::time::Duration;

#[derive(Debug)]
struct Config {
    intensity: u32,
    random_number: u32,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Self, &str> {
        if args.len() != 2 {
            return Err("please add exercise intensity");
        }

        let mut rng = thread_rng();
        let intensity: u32 = args[1].parse().unwrap();
        let random_number: u32 = rng.gen_range(1..5);
        let config = Self {
            intensity,
            random_number,
        };

        log::debug!("Init Config: {:?}", config);

        Ok(config)
    }
}

#[derive(Debug)]
struct Cacher<T, K, V>
where
    T: Fn(K) -> V,
    K: Eq + hash::Hash + Copy,
    V: Copy,
{
    calculation: T,
    value: HashMap<K, V>,
}

impl<T, K, V> Cacher<T, K, V>
where
    T: Fn(K) -> V,
    K: Eq + hash::Hash + Copy,
    V: Copy,
{
    fn new(calculation: T) -> Cacher<T, K, V> {
        Cacher {
            calculation,
            value: HashMap::new(),
        }
    }

    fn value(&mut self, arg: K) -> V {
        match self.value.get(&arg) {
            Some(v) => *v,
            None => {
                let v = (self.calculation)(arg);
                self.value.insert(arg, v);
                v
            }
        }
    }
}

fn generate_workout(config: &Config) {
    let mut expensive_result = Cacher::new(|num| {
        log::warn!("calculation slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if config.intensity < 25 {
        log::info!(
            "Today, do {} pushups!",
            expensive_result.value(config.intensity)
        );
        log::info!(
            "Next, do {} situps!",
            expensive_result.value(config.intensity)
        );
    } else if config.random_number == 3 {
        log::info!("Take a break today! Remember to stay hydrated");
    } else {
        log::info!(
            "Today, run for {} minutes!",
            expensive_result.value(config.intensity)
        );
    }
}

fn main() {
    env_logger::init();

    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Parsing arguments: {}", err);
        process::exit(1);
    });

    generate_workout(&config);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn call_with_different_values() {
        let mut c = Cacher::new(|a| a);
        let _v1 = c.value(1);
        let v2 = c.value(2);

        assert_eq!(v2, 2);
    }
}
