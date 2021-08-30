#[derive(Debug)]
enum C {
    Cc(String),
}

#[derive(Debug)]
enum B {
    Bb(C),
}

#[derive(Debug)]
enum A {
    Aa(B),
}

fn main() {
    let _result: String = match get() {
        A::Aa(res) => match res {
            B::Bb(res2) => match res2 {
                C::Cc(res3) => res3
            }
        }
    };
    dbg!(&_result);
    // println!("{:#?}", &_result);

    let _result_b: A = get();
}

fn get() -> A {
    A::Aa(B::Bb(C::Cc(String::from("OK"))))
}