#[test]
fn hello() {
    let x: f64 = 11111111.1515151515;
    print!("there is a\npurpose\n");
    println!("there is a purpose");
    println!("the number is {}", x);
    println!("there is a number {:?} {:.5}", x, x);
    println!("{}", calculateCompileTime());
}

const fn calculateCompileTime() -> f64 {
    123.123
}
pub struct S;

impl S {
    fn method(&self) -> &S {
        self
    }
}

#[test]
fn controlFlow() {
    'l1: loop {
        'l2: loop {
            'l3: loop {
                println!("l{}", 3);
                break 'l1;
            }
            println!("l{}", 2);
        }
        println!("l{}", 1);
    }
}

#[test]
fn optionTake() {
    let mut o: Option<i32> = Some(-50);
    let t = o.take();
    dbg!(o);
    dbg!(t);
    let t = t.unwrap();
    dbg!(t);
}
