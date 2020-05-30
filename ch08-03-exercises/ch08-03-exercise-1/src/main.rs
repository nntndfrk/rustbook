/*
Есть список целых чисел.
Создайте функцию, используйте вектор и верните из списка:
    - среднее значение;
    - медиану (значение элемента из середины списка после его сортировки);
    - mode значение (то которое встречается в списке наибольшее количество раз; хэш-карта будет полезна в данном случае)
*/

use std::collections::HashMap;

fn main() {
    let l1 = vec![ 18, 17, 23, 45, 456, 43, 33, 900, 33, 323, 67, 1, 1, 4, 77, 22, 33];

    // check(&l1);

    let avg = calc_avg(&l1);
    let med = calc_median(&l1);
    let mode = get_mode(&l1);

    println!("avg is {}", avg);
    println!("median is {}", med);
    println!("mode is {}", mode);
}

fn calc_avg(num_list: &Vec<i32>) -> f32 {
    let sum = num_list.iter().fold(0, |s, v| s + *v);
    sum as f32 / num_list.len() as f32
}

fn calc_median(num_list: &Vec<i32>) -> f32 {
    let len = num_list.len();
    let mut sorted_list = num_list.clone();
    sorted_list.sort();

    let is_odd_len = (len % 2) != 0;

    if is_odd_len {
        sorted_list[(len - 1)/2] as f32
    } else {
        (sorted_list[len/2] + sorted_list[len/2 -1]) as f32 / 2.0
    }
}

fn get_mode(num_list: &Vec<i32>) -> i32 {
    let mut mode_map: HashMap<_, _> = HashMap::new();

    for num in num_list {
        let c = mode_map.entry(num).or_insert(0);
        *c += 1;
    }

    let mut larger = 0;
    let mut larger_key = 0;

    for (k, &v) in mode_map.iter() {
        if v > larger {
            larger = v;
            larger_key = **k;
        }
    }
    return larger_key;
}