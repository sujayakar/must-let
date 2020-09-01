#![feature(box_patterns)]

use must_let::must_let;

#[test]
fn test_ident() {
    must_let!(let x = 3);
    assert_eq!(x, 3);
}

#[test]
fn test_tuple() {
    must_let!(let (a, b) = (4, 5));
    assert_eq!(a, 4);
    assert_eq!(b, 5);

    must_let!(let (x, _) = (6, 7));
    assert_eq!(x, 6);

    must_let!(let (a, (b, c)) = (1, (2, 3)));
    assert_eq!(a, 1);
    assert_eq!(b, 2);
    assert_eq!(c, 3);

    let mut t = (4, 5);
    must_let!(let (ref a, ref mut b) = t);
    *b += 1;
    assert_eq!(a, &4);
    assert_eq!(t.1, 6);
}

#[test]
fn test_box_pattern() {
    must_let!(let box v = Box::new(3));
    assert_eq!(v, 3);
}

#[test]
fn test_slice_patterns() {
    let v = vec![1, 2, 3, 4, 5, 6];
    must_let!(let [a, b, ref c @ .., y, z] = &v[..]);
    assert_eq!(a, &1);
    assert_eq!(b, &2);
    assert_eq!(c, &[3, 4]);
    assert_eq!(y, &5);
    assert_eq!(z, &6);
}

#[test]
fn test_structs() {
    #[derive(Debug)]
    struct Test1 {
        a: (usize, usize),
        b: usize,
    }
    let t = Test1 { a: (1, 2), b: 3 };
    must_let!(let Test1 { a, ref b } = t);
    assert_eq!(a, (1, 2));
    assert_eq!(b, &3);

    #[derive(Debug)]
    struct Test2(usize);
    must_let!(let Test2(c) = Test2(3));
    assert_eq!(c, 3);
}

#[test]
fn test_enums() {
    must_let!(let Ok(a) = Ok::<_, usize>(0));
    assert_eq!(a, 0);
}
