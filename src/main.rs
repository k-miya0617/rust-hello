
// fn copy_trait_check<T: Copy>(_: T){}

fn main() {

    /*
    // これは変数 num がmutableではないため、変数の中身を書き換えることはできない...と思われる
    let num = 10;
    num = 20;
    */

    // 変数の中身を書き換えたい場合は、mutをつける必要があるようだ
    // let mut a = 10;
    // a = 20;


    // 参照オブジェクト

    /* 
    ref1はmutではないため、ref1への代入は不可能
    let ref1 = &a;
    ref1 = 30;
    */

    // let a_ref1 = &a;
    // let a_mut_ref1 = &mut a;
    // let a_mut_ref2 = &mut a;

    // *a_mut_ref2 = 30;

    /*
    let mut a = 10;
    let a_ref1 = &a;
    let a_mut_ref1 = &mut a;
    let a_mut_ref2 = &mut a;

    *a_mut_ref2 = 20;

    println!("{}", a_mut_ref2);
    */


    /*
    let s = String::from("Hello World");
    let a = 10;
    copy_trait_check(s);
    println!("{}", s);
    */

    /* 
    let (x, y, z) = (1, 2, 3);
    let [a, b, c] = [4, 5, 6];
    let (i, _, _) = (7, 8, 9);

    println!("xyz= {} {} {}", x, y, z);
    println!("abc= {} {} {}", a, b, c);
    println!("  i= {}", i);
    */

    /* 
    let str_len = String::from("Hello World");
    let str_len = str_len.len();

    println!("{}", str_len);
    */

    /*
    let a = 10;

    {
        let mut a = 20;
        a += 30;
        println!("{}", a);
    }

    println!("{}", a);
    */

    /*
    let a = 13u8;
    let b = 7u32;
    let c = a as u32 + b;
    println!("{}", c);

    let mut t = false;
    t = true;
    println!("{}", t);
    println!("{}", t as i32);
    */

    /*
    let mut sum = 0;
    for i in 1..=10 {
        sum += i;
        println!("{}", i);
    }
    println!("{}", sum);
    */

    // println!("{}", add(1, 2)); 

    /*
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };

    println!("{}", counter);

    let a = [10, 20, 30, 40, 50];
    for ele in a.iter() {
        println!("{}", ele);
    }

    for ele in 0..10 {
        println!("{}", ele);
    }
    */

    /*
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    user1.email = String::from("hoge");
    */

    /* 
    let mut user1 = build_user(
        String::from("miya@gmail.com"),
        String::from("miya"),
    );

    let mut user2 = User {
        email: String::from("sakaki@gmail.com"),
        ..user1
    };

    // println!("{}", user1.username); // error
    println!("{}", user2.username);
    */

    let black = Color(0,0,0);
    let point = Point(0,0,0);

    println!("{} {} {}", black.0, black.1, black.2,);
    println!("{} {} {}", point.0, point.1, point.2);
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

/* 
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
*/

/*
fn add(a: i32, b: i32) -> i32 {
    a + b
}
*/