mod elision;
mod structs;
mod the_problem;
mod the_solution;

// Rust ensures that a reference is always valid at runtime.

// What does it mean "valid"?
// -> It means the data that is present at the
//      location that the ref is pointing to is
//      the data which is intended.
// -> It shouldn't be data that was cleared up
//      and now stores something else.
// -> If we do 'let x = 10' and 'let y = &x'
//      and even if we use 'y' after 'x' is
//      goes out of scope (is shan't be dropped),
//      'y' must still be valid, that is,
//      the original location of 'x',
//      must still store 10 and value of 'y'
//      must still be the same address that was
//      where 'x' was stored.

// So how does it ensures that?
// -> Compiler magic.
// -> Sometimes it's not entirely the compiler,
//      we do some "extra work" as well.
// -> Compiler only ensures that
//      a value ('x' from previous example)
//      must not be dropped (or cleaned up)
//      if it detects that a ref is still using it.

// Okay, but why is the topic called 'lifetimes',
// how do it all fit in?
// -> The compiler assigns every ref a 'lifetime',
//      and sometimes it can not detect the exact
//      lifetime of a ref.
// -> Which is where lifetime annotation comes in.
// -> This annotating is the "extra work" that we do,
//      to help the compiler determine
//      an exact lifetime of a ref.

// If the compiler can not figure out the exact lifetime
// of a ref, the program doesn't compile.

// Which makes sure that a ref is always valid at runtime
// because there is no runtime. LOL.

// By us annotating lifetime, we help the compiler figure out
// the exact lifetime when it couldn't figure out on it's own.
// --> Program compiles --> It runs --> Refs are valid at runtime.

pub fn lifetimes() {
    // the_problem::the_problem();
    // the_solution::the_solution();
    // structs::structs();
    elision::elision();
}
