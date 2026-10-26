// Borrow check example (not from book)
struct Foo {
    value: usize,
}

fn reverse_and_print(foo: &mut Vec<Foo>) {
    foo.reverse();

    for f in foo.iter() {
        println!("{}", f.value);
    }
}

fn main() {
    let mut vector = vec![Foo { value: 1 }, Foo { value: 2 }, Foo { value: 3 }];

    reverse_and_print(&mut vector);
    reverse_and_print(&mut vector);
}
