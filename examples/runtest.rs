//! This is NOT a real example. This is a test designed to see if we can actually run the turtle
//! process

use std::process;

use turtle::{Drawing, Turtle};

fn main() {
    let mut drawing = Drawing::new();
    let mut turtle = drawing.add_turtle();

    turtle.set_speed(2);
    turtle.right(90.0);
    turtle.forward(50.0);

    run_tests(&mut drawing, &mut turtle);

    process::exit(0);
}

/// These are cases that need to be checked in a real, running turtle instance and cannot be
/// checked in the test environment
fn run_tests(drawing: &mut Drawing, turtle: &mut Turtle) {
    ignores_nan_inf_zero(drawing, turtle);
    ignores_nan_inf(drawing, turtle);
    ignores_zero(drawing, turtle);
}

fn ignores_nan_inf_zero(_drawing: &mut Drawing, turtle: &mut Turtle) {
    turtle.forward(0.0);
    turtle.forward(::std::f64::NAN);
    turtle.forward(::std::f64::INFINITY);
    turtle.forward(-::std::f64::INFINITY);

    turtle.backward(0.0);
    turtle.backward(::std::f64::NAN);
    turtle.backward(::std::f64::INFINITY);
    turtle.backward(-::std::f64::INFINITY);

    turtle.left(0.0);
    turtle.left(::std::f64::NAN);
    turtle.left(::std::f64::INFINITY);
    turtle.left(-::std::f64::INFINITY);

    turtle.right(0.0);
    turtle.right(::std::f64::NAN);
    turtle.right(::std::f64::INFINITY);
    turtle.right(-::std::f64::INFINITY);

    turtle.wait(0.0);
    turtle.wait(::std::f64::NAN);
    turtle.wait(::std::f64::INFINITY);
    turtle.wait(-::std::f64::INFINITY);
}

fn ignores_nan_inf(drawing: &mut Drawing, turtle: &mut Turtle) {
    drawing.set_center([::std::f64::NAN, 0.0]);
    drawing.set_center([0.0, ::std::f64::NAN]);
    drawing.set_center([::std::f64::NAN, ::std::f64::NAN]);
    drawing.set_center([::std::f64::INFINITY, 0.0]);
    drawing.set_center([0.0, ::std::f64::INFINITY]);
    drawing.set_center([::std::f64::INFINITY, ::std::f64::INFINITY]);
    drawing.set_center([-::std::f64::INFINITY, 0.0]);
    drawing.set_center([0.0, -::std::f64::INFINITY]);
    drawing.set_center([-::std::f64::INFINITY, -::std::f64::INFINITY]);

    turtle.turn_towards([::std::f64::NAN, 0.0]);
    turtle.turn_towards([0.0, ::std::f64::NAN]);
    turtle.turn_towards([::std::f64::NAN, ::std::f64::NAN]);
    turtle.turn_towards([::std::f64::INFINITY, 0.0]);
    turtle.turn_towards([0.0, ::std::f64::INFINITY]);
    turtle.turn_towards([::std::f64::INFINITY, ::std::f64::INFINITY]);
    turtle.turn_towards([-::std::f64::INFINITY, 0.0]);
    turtle.turn_towards([0.0, -::std::f64::INFINITY]);
    turtle.turn_towards([-::std::f64::INFINITY, -::std::f64::INFINITY]);

    turtle.go_to([::std::f64::NAN, 0.0]);
    turtle.go_to([0.0, ::std::f64::NAN]);
    turtle.go_to([::std::f64::NAN, ::std::f64::NAN]);
    turtle.go_to([::std::f64::INFINITY, 0.0]);
    turtle.go_to([0.0, ::std::f64::INFINITY]);
    turtle.go_to([::std::f64::INFINITY, ::std::f64::INFINITY]);
    turtle.go_to([-::std::f64::INFINITY, 0.0]);
    turtle.go_to([0.0, -::std::f64::INFINITY]);
    turtle.go_to([-::std::f64::INFINITY, -::std::f64::INFINITY]);

    turtle.set_x(::std::f64::NAN);
    turtle.set_x(::std::f64::INFINITY);
    turtle.set_x(-::std::f64::INFINITY);

    turtle.set_y(::std::f64::NAN);
    turtle.set_y(::std::f64::INFINITY);
    turtle.set_y(-::std::f64::INFINITY);

    turtle.set_heading(::std::f64::NAN);
    turtle.set_heading(::std::f64::INFINITY);
    turtle.set_heading(-::std::f64::INFINITY);
}

fn ignores_zero(drawing: &mut Drawing, _turtle: &mut Turtle) {
    drawing.set_size([0, 0]);
}
