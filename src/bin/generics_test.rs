fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for n in list {
        if largest < n {
            largest = n;
        }
    }
    largest
}

struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}
enum Option_num<T> {
    Some(T),
    None,
}

fn main() {
    let number = vec![34, 50, 25, 100, 65];
    print!("{}", largest(&number));

    let p1 = Point { x: 5, y: 10.4 };
    let p2 = Point { x: "Hello", y: 'c' };

    let p3 = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);

    let a=Option_num::Some(5.0);
    let b=Option_num::Some(5);
    let c=Some(8);

}
