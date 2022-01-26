macro_rules! run_alu {
    { $input:ident, inp $a:ident } => { $a = $input.next().unwrap(); };

    { $_input:ident, add $a:ident $b:ident } => { $a = $a + $b; };
    { $_input:ident, add $a:ident $b:literal } => { $a = $a + $b; };

    { $_input:ident, mul $a:ident $b:ident } => { $a = $a * $b; };
    { $_input:ident, mul $a:ident $b:literal } => { $a = $a * $b; };

    { $_input:ident, div $a:ident $b:ident } => { $a = $a / $b; };
    { $_input:ident, div $a:ident $b:literal } => { $a = $a / $b; };

    { $_input:ident, mod $a:ident $b:ident } => { $a = $a % $b; };
    { $_input:ident, mod $a:ident $b:literal } => { $a = $a % $b; };

    { $_input:ident, eql $a:ident $b:ident } => { $a = if $a == $b { 1 } else { 0 } };
    { $_input:ident, eql $a:ident $b:literal } => { $a = if $a == $b { 1 } else { 0 } };

    // Recursion
    { $input:ident, inp $a:ident $($tt:tt)+ } => { run_alu!{ $input, inp $a }; run_alu!{ $input, $($tt)* } };
    { $input:ident, $l:tt $a:ident $b:ident $($tt:tt)+ } => { run_alu!{ $input, $l $a $b }; run_alu!{ $input, $($tt)* } };
    { $input:ident, $l:tt $a:ident $b:literal $($tt:tt)+ } => { run_alu!{ $input, $l $a $b }; run_alu!{ $input, $($tt)* } };
}

#[derive(Debug)]
pub struct AluResult {
    pub w: i64,
    pub x: i64,
    pub y: i64,
    pub z: i64,    
}

#[allow(unused_mut, unused_assignments)]
pub fn example1(mut input: impl Iterator<Item = i64>) -> AluResult {
    let mut w = 0;
    let mut x = 0;
    let mut y = 0;
    let mut z = 0;

    run_alu! {
        input,
        inp x
        mul x -1
    }

    AluResult { w, x, y, z }
}

#[allow(unused_mut, unused_assignments)]
pub fn example2(mut input: impl Iterator<Item = i64>) -> AluResult {
    let mut w = 0;
    let mut x = 0;
    let mut y = 0;
    let mut z = 0;

    run_alu! {
        input,
        inp z
        inp x
        mul z 3
        eql z x
    }

    AluResult { w, x, y, z }
}

#[allow(unused_mut, unused_assignments)]
pub fn example3(mut input: impl Iterator<Item = i64>) -> AluResult {
    let mut w = 0;
    let mut x = 0;
    let mut y = 0;
    let mut z = 0;

    run_alu! {
        input,
        inp w
        add z w
        mod z 2
        div w 2
        add y w
        mod y 2
        div w 2
        add x w
        mod x 2
        div w 2
        mod w 2
    }

    AluResult { w, x, y, z }
}

#[allow(unused_assignments)]
pub fn user(mut input: impl Iterator<Item = i64>) -> AluResult {
    let mut w = 0;
    let mut x = 0;
    let mut y = 0;
    let mut z = 0;

    run_alu! {
        input,
        inp w
        mul x 0
        add x z
        mod x 26
        div z 1
        add x 10
        eql x w
        eql x 0
        mul y 0
        add y 25
        mul y x
        add y 1
        mul z y
        mul y 0
        add y w
        add y 2
        mul y x
        add z y
    };

    run_alu! {
        input,
        inp w
        mul x 0
        add x z
        mod x 26
        div z 1
        add x 15
        eql x w
        eql x 0
        mul y 0
        add y 25
        mul y x
        add y 1
        mul z y
        mul y 0
        add y w
        add y 16
        mul y x
        add z y
    };

    run_alu! {
        input,
        inp w
        mul x 0
        add x z
        mod x 26
        div z 1
        add x 14
        eql x w
        eql x 0
        mul y 0
        add y 25
        mul y x
        add y 1
        mul z y
        mul y 0
        add y w
        add y 9
        mul y x
        add z y
    };

    run_alu! {
        input,
        inp w
        mul x 0
        add x z
        mod x 26
        div z 1
        add x 15
        eql x w
        eql x 0
        mul y 0
        add y 25
        mul y x
        add y 1
        mul z y
        mul y 0
        add y w
        add y 0
        mul y x
        add z y
    };

    run_alu! {
        input,
        inp w
        mul x 0
        add x z
        mod x 26
        div z 26
        add x -8
        eql x w
        eql x 0
        mul y 0
        add y 25
        mul y x
        add y 1
        mul z y
        mul y 0
        add y w
        add y 1
        mul y x
        add z y
    };

    run_alu! {
        input,
        inp w
        mul x 0
        add x z
        mod x 26
        div z 1
        add x 10
        eql x w
        eql x 0
        mul y 0
        add y 25
        mul y x
        add y 1
        mul z y
        mul y 0
        add y w
        add y 12
        mul y x
        add z y
    };

    run_alu! {
        input,
        inp w
        mul x 0
        add x z
        mod x 26
        div z 26
        add x -16
        eql x w
        eql x 0
        mul y 0
        add y 25
        mul y x
        add y 1
        mul z y
        mul y 0
        add y w
        add y 6
        mul y x
        add z y
    };

    run_alu! {
        input,
        inp w
        mul x 0
        add x z
        mod x 26
        div z 26
        add x -4
        eql x w
        eql x 0
        mul y 0
        add y 25
        mul y x
        add y 1
        mul z y
        mul y 0
        add y w
        add y 6
        mul y x
        add z y
    };

    run_alu! {
        input,
        inp w
        mul x 0
        add x z
        mod x 26
        div z 1
        add x 11
        eql x w
        eql x 0
        mul y 0
        add y 25
        mul y x
        add y 1
        mul z y
        mul y 0
        add y w
        add y 3
        mul y x
        add z y
    };

    run_alu! {
        input,
        inp w
        mul x 0
        add x z
        mod x 26
        div z 26
        add x -3
        eql x w
        eql x 0
        mul y 0
        add y 25
        mul y x
        add y 1
        mul z y
        mul y 0
        add y w
        add y 5
        mul y x
        add z y
    };

    run_alu! {
        input,
        inp w
        mul x 0
        add x z
        mod x 26
        div z 1
        add x 12
        eql x w
        eql x 0
        mul y 0
        add y 25
        mul y x
        add y 1
        mul z y
        mul y 0
        add y w
        add y 9
        mul y x
        add z y
    };

    run_alu! {
        input,
        inp w
        mul x 0
        add x z
        mod x 26
        div z 26
        add x -7
        eql x w
        eql x 0
        mul y 0
        add y 25
        mul y x
        add y 1
        mul z y
        mul y 0
        add y w
        add y 3
        mul y x
        add z y
    };

    run_alu! {
        input,
        inp w
        mul x 0
        add x z
        mod x 26
        div z 26
        add x -15
        eql x w
        eql x 0
        mul y 0
        add y 25
        mul y x
        add y 1
        mul z y
        mul y 0
        add y w
        add y 2
        mul y x
        add z y
    };

    run_alu! {
        input,
        inp w
        mul x 0
        add x z
        mod x 26
        div z 26
        add x -7
        eql x w
        eql x 0
        mul y 0
        add y 25
        mul y x
        add y 1
        mul z y
        mul y 0
        add y w
        add y 3
        mul y x
        add z y
    };

    AluResult { w, x, y, z }
}

#[test]
fn test_example1() {
    let result = example1([42].into_iter());
    assert_eq!(-42, result.x);
}

#[test]
fn test_example2() {
    let result1 = example2([3, 9].into_iter());
    assert_eq!(1, result1.z);

    let result2 = example2([3, 10].into_iter());
    assert_eq!(0, result2.z);
}

#[test]
fn test_example3() {
    let result = example3([11].into_iter());
    assert_eq!(1, result.z);
    assert_eq!(1, result.y);
    assert_eq!(0, result.x);
    assert_eq!(1, result.w);
}