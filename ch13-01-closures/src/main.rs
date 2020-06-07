use std::thread;
use std::time::Duration;
use std::hash::Hash;
use std::collections::HashMap;

fn main() {
    let simulated_use_specified_value = 10;
    let simulated_random_number = 7;

    generate_workout(
        simulated_use_specified_value,
        simulated_random_number,
    );
}

// fn simulated_expensive_calculation(intensity: i32) -> i32 {
//     println!("calculating slowly...");
//     thread::sleep(Duration::from_secs(2));
//     intensity
// }

struct Cacher<T, V>
    where T: Fn(V) -> V,
          V: Copy + Eq + Hash
{
    calculation: T,
    values: HashMap<V, V>,
}

impl<T, V> Cacher<T, V>
    where T: Fn(V) -> V,
          V: Copy + Eq + Hash
{
    fn new(calculation: T) -> Cacher<T, V> {
        Cacher {
            calculation,
            values: HashMap::new(),
        }
    }

    fn value(&mut self, arg: V) -> V {
        match self.values.get(&arg) {
            Some(v) => *v,
            None => {
                let v = (self.calculation)(arg);
                self.values.insert(arg, v);
                v
            }
        }
    }
}

fn generate_workout(intensity: u32, random_number: u32) {
    // I

    // let expensive_result = simulated_expensive_calculation(intensity);

    // II

    // let expensive_closure = |num: i32| -> i32 {
    //     println!("calculating slowly...");
    //     thread::sleep(Duration::from_secs(2));
    //     num
    // };

    // III

    let mut expensive_result = Cacher::new(|num: u32| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity < 25 {
        println!(
            "Today, do {} pushups!",
            expensive_result.value(intensity)
        );
        println!(
            "Next, do {} situps!",
            expensive_result.value(intensity)
        );
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                expensive_result.value(intensity)
            )
        }
    }
}

#[test]
fn call_with_different_values() {
    let mut c = Cacher::new(|a| i32::pow(a, 2));

    let v1 = c.value(1);
    let v2 = c.value(2);

    assert_eq!(v2, 4);
}