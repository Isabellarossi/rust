#![feature(plugin)]
#![plugin(clippy)]

#![allow(unused_variables)]

fn takes_an_immutable_reference(a: &i32) {}
fn takes_a_mutable_reference(a: &mut i32) {}

struct MyStruct;

impl MyStruct {
    fn takes_an_immutable_reference(&self, a: &i32) {
    }

    fn takes_a_mutable_reference(&self, a: &mut i32) {
    }
}

#[deny(unnecessary_mut_passed)]
fn main() {
    // Functions
    takes_an_immutable_reference(&mut 42); //~ERROR The function/method "takes_an_immutable_reference" doesn't need a mutable reference
    let foo: fn(&i32) = takes_an_immutable_reference;
    foo(&mut 42); //~ERROR The function/method "foo" doesn't need a mutable reference

    // Methods
    let my_struct = MyStruct;
    my_struct.takes_an_immutable_reference(&mut 42); //~ERROR The function/method "takes_an_immutable_reference" doesn't need a mutable reference


    // No error

    // Functions
    takes_an_immutable_reference(&42);
    let foo: fn(&i32) = takes_an_immutable_reference;
    foo(&42);

    takes_a_mutable_reference(&mut 42);
    let foo: fn(&mut i32) = takes_a_mutable_reference;
    foo(&mut 42);

    let a = &mut 42;
    takes_an_immutable_reference(a);

    // Methods
    my_struct.takes_an_immutable_reference(&42);
    my_struct.takes_a_mutable_reference(&mut 42);
    my_struct.takes_an_immutable_reference(a);
}
