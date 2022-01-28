// Test suggesting tuples where bare arguments may have been passed
// See issue #86481 for details.

// run-rustfix

fn main() {
    let _: Result<(i32, i8), ()> = Ok(1, 2);
    //~^ ERROR this enum variant takes 1 argument but 2 arguments were supplied
    let _: Option<(i32, i8, &'static str)> = Some(1, 2, "hi");
    //~^ ERROR this enum variant takes 1 argument but 3 arguments were supplied
    let _: Option<()> = Some();
    //~^ ERROR this enum variant takes 1 argument but 0 arguments were supplied

    two_ints(1, 2); //~ ERROR this function takes 1 argument

    with_generic(3, 4); //~ ERROR this function takes 1 argument
}

fn two_ints(_: (i32, i32)) {
}

fn with_generic<T: Copy + Send>((a, b): (i32, T)) {
    if false {
        // test generics/bound handling
        with_generic(a, b); //~ ERROR this function takes 1 argument
    }
}
