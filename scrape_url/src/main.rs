use std::fs;

#[derive(Debug)]
enum Gender {
    Unspecified = 0,
    Female = 1,
    Male = 2,
}

#[derive(Debug, Copy, Clone)]
struct UserID(u64);
#[derive(Debug, Copy, Clone)]
struct TopicID(u64);

#[derive(Debug)]
struct User {
    id: UserID,
    name: String,
    gender: Gender,
}
#[derive(Debug)]
struct Topic {
    id: TopicID,
    name: String,
    owner: UserID,
}

// 定义聊天中农可能发生的事件
#[derive(Debug)]
enum Event {
    Join((UserID, TopicID)),
    Leave((UserID, TopicID)),
    Message((UserID, TopicID, String)),
}

// fn main() {
//     // let url = "https://www.rust-lang.org";
//     // let output = "rust.md";

//     // println!("Fetching url: {}", url);

//     // let body = reqwest::blocking::get(url).unwrap().text().unwrap();
//     // println!("Converting html to markdown");

//     // let md = html2md::parse_html(&body);
//     // fs::write(output, md.as_bytes()).unwrap();
//     // println!("转换成功 {}", output);

//     // println!("apply square {}", apply(2, square));
//     // println!("apply cute {}", apply(3, cube));

//     // let is_pi = pi();
//     // let is_unit1 = not_pi();
//     // let is_unit2 = {
//     //     pi();
//     // };
//     // println!(
//     //     "is_pi : {:?}, is_unit1 : {:?}, is_unit2 : {:?}",
//     //     is_pi, is_unit1, is_unit2
//     // );

//     // let alice = User {
//     //     id: UserID(1),
//     //     name: "Alice".into(),
//     //     gender: Gender::Female,
//     // };

//     // let bob = User {
//     //     id: UserID(2),
//     //     name: "bob".into(),
//     //     gender: Gender::Male,
//     // };

//     // let topic = Topic {
//     //     id: TopicID(1),
//     //     name: "rust".into(),
//     //     owner: UserID(1),
//     // };

//     // let event1 = Event::Join((alice.id, topic.id));
//     // let event2 = Event::Join((bob.id, topic.id));
//     // let event3 = Event::Message((alice.id, topic.id, "Hello World!".into()));

//     // println!("event1 : {:?}", event1);
//     // println!("event2 : {:?}", event2);
//     // println!("event3 : {:?}", event3);

//     let n = 10;
//     fib_loop(n);
//     fib_while(n);
//     fib_for(n);

// }

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url = "https://www.rust-lang.org/";
    let output = "rust.md";

    println!("fetch url :{} ", url);
    let body = reqwest::blocking::get(url)?.text()?;

    println!("convert html to markdown...");

    let md = html2md::parse_html(&body);

    fs::write(output, md.as_bytes())?;
    println!(" saved in {}", output);
    Ok(())
}

fn apply(val: i32, f: fn(i32) -> i32) -> i32 {
    f(val)
}

fn square(val: i32) -> i32 {
    val * val
}

fn cube(val: i32) -> i32 {
    val * val * val
}

fn pi() -> f64 {
    3.1415926
}

fn not_pi() {
    3.1415926;
}

fn fib_loop(n: u8) {
    let mut a = 1;
    let mut b = 1;
    let mut i = 2u8;

    loop {
        let c = a + b;
        a = b;
        b = c;
        i += 1;
        println!("next val is {}", b);

        if i >= n {
            break;
        }
    }
}

fn fib_while(n: u8) {
    let (mut a, mut b, mut i) = (1, 1, 2);

    while i < n {
        let c = a + b;
        a = b;
        b = c;
        i += 1;
        println!("next val is {}", b);
    }
}

fn fib_for(n: u8) {
    let (mut a, mut b) = (1, 1);
    for _i in 2..n {
        let c = a + b;
        a = b;
        b = c;
        println!("next val is {}", b);
    }
}

fn process_event(e: &Event) {
    match e {
        Event::Join((uid, tid)) => println!("user {:?} joined", uid),
        Event::Leave((uid, tid)) => println!("user {:?} left", uid),
        Event::Message((_, _, msg)) => println!("broadcast {:?}", msg),
    }
}

fn process_message(event: &Event) {
    if let Event::Message((_, _, msg)) = event {
        println!("broadcast : {}", msg);
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4)
    }
}
