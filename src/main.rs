#[derive(Copy, Clone)]
struct Lift<T>(T);

impl<T> Lift<T> {
    fn force(&self) -> &T {
        &self.0
    }

    fn lift(x: T) -> Lift<T> {
        Lift(x)
    }
}

#[derive(Debug)]
struct Unit();

#[derive(Debug)]
enum Nat {
    Left(Unit),
    Right(Box<Nat>)
}

enum List<A> {
    Left(Unit),
    Right(A, Box<List<A>>)
}

struct Stream<T>(T, Box<Lift<Stream<T>>>);

fn zero() -> Nat {
    Nat::Left(Unit())
}

fn succ(n: Nat) -> Nat {
    Nat::Right(Box::new(n))
}

fn main() {
    println!("Hello, world!");
    let lifted_one = Lift::lift(1);
    let copy_lift = lifted_one;
    println!("{}", lifted_one.force());
    let zero = zero();
    println!("{:?}", zero);
    let one = succ(zero);
    println!("{:?}", one);
    let two = succ(one);
    println!("{:?}", two);
}
