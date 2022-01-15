use std::collections::HashMap;
use std::thread;
use std::time::Duration;

mod iterators;

fn main() {
    let simulated_user_specific_value = 25;
    let simulated_random_number = 3;

    generate_workout(simulated_user_specific_value, simulated_random_number);

    environment_borrow();
}

fn environment_borrow() {
    let x = vec![1, 2, 3];

    let equal_to_x = move |z| z == x;

    // println!("can't use x here: {:?}", x); // illegal borrow after move

    let y = vec![1, 2, 3];

    assert!(equal_to_x(y));
}

struct Cacher<T>
where
    T: Fn(u32) -> u32,
{
    calculation: T,
    value: HashMap<u32, u32>,
}

impl<T> Cacher<T>
where
    T: Fn(u32) -> u32,
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation: calculation,
            value: HashMap::new(),
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        match self.value.get(&arg) {
            Some(v) => *v,
            None => {
                let v = (self.calculation)(arg);
                self.value.insert(arg, v);
                v
            }
        }

        // if self.value.contains_key(&arg) {
        //     *self.value.get(&arg).unwrap()
        // } else {
        //     let v = (self.calculation)(arg);
        //     self.value.insert(arg, v);
        //     v
        // }

        // match self.value {
        //     Some(v) => v,
        //     None => {
        //         let v = (self.calculation)(arg);
        //         self.value = Some(v);
        //         v
        //     }
        // }
    }
}

fn generate_workout(intensity: u32, random_number: u32) {
    let mut expensive_result = Cacher::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_result.value(intensity));
        println!("Next, do {} situps!", expensive_result.value(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today run for {} minutes!",
                expensive_result.value(intensity)
            );
        }
    }
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]
    fn call_twice() {
        let mut c = Cacher::new(|a| a + 1);

        let v1 = c.value(1);
        let v2 = c.value(1);

        assert_eq!(v1, 2);
        assert_eq!(v2, 2);
    }

    #[test]
    fn call_with_different_values() {
        let mut c = Cacher::new(|a| a);

        let v1 = c.value(1);
        let v2 = c.value(2);

        assert_eq!(v1, 1);
        assert_eq!(v2, 2);
    }
}
