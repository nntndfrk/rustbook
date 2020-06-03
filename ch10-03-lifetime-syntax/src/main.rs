// fn foo() {
//     {
//         let r;                // ---------+-- 'a
//                               //          |
//         {                     //          |
//             let x = 5;        // -+-- 'b  |
//             r = &x;           //  |       |
//         }                     // -+       |
//                               //          |
//         println!("r: {}", r); //          |
//     }                         // ---------+
//
// }
/*
Здесь мы описали время жизни для r с помощью 'a и время жизни x с помощью 'b .
Внутренний блок времени жизни 'b гораздо меньше времени жизни внешнего блока 'a.
Во время компиляции Rust сравнивает размер двух времён жизни и видит,
что r имеет время жизни 'a, но ссылается на память со временем жизни 'b.
Программа отклоняется, потому что 'b короче, чем 'a :
    объект ссылки не живёт достаточно долго как сама ссылка.
*/


fn main() {
    let x= 5;
    let r = &x;

    println!("r: {}", r);
    println!("=======================");

    let str1 = "abcd".to_string();
    let str2 = "xyz";

    let res = longest(str1.as_str(), str2);
    println!("The longest string is {}", res);

    println!("=======================");
    let str1 = "long string is long".to_string();
    {
        let str2 = "xyz";

        let res = longest(str1.as_str(), str2);
        println!("The longest string is {}", res);
    }
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}