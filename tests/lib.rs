use elias_fano::EliasFano;

#[test]
fn test_membership() {
    const NUM: u64 = 1000;
    let mut ef = EliasFano::new(NUM, NUM);
    let array: Vec<u64> = vec![0; NUM as usize]
        .iter()
        .enumerate()
        .map(|(idx, _)| idx as u64)
        .collect();

    ef.compress(array.iter()).unwrap();

    for (idx, v) in array.iter().enumerate() {
        if ef.value() != *v {
            panic!("{} is not the same as {}", ef.value(), v);
        }

        match ef.next() {
            Ok(_) => (),
            Err(_) => {
                if idx != array.len() - 1 {
                    panic!("Error returned when not at end of items");
                }
            }
        }
    }
}

#[test]
fn test_position() {
    const NUM: u64 = 1000;
    let mut ef = EliasFano::new(NUM, NUM);
    let array: Vec<u64> = vec![0; NUM as usize]
        .iter()
        .enumerate()
        .map(|(idx, _)| idx as u64)
        .collect();

    ef.compress(array.iter()).unwrap();

    for i in 0..NUM {
        if ef.position() != i {
            panic!("Index is returning wrong position for linear increment")
        }
        let _ = ef.next();
    }
}

#[test]
fn test_skip() {
    const NUM: u64 = 1000;
    let mut ef = EliasFano::new(NUM, NUM);
    let array: Vec<u64> = vec![0; NUM as usize]
        .iter()
        .enumerate()
        .map(|(idx, _)| idx as u64)
        .collect();

    ef.compress(array.iter()).unwrap();

    ef.skip(500).unwrap();
    assert_eq!(ef.value(), 500);

    ef.skip(350).unwrap();
    assert_eq!(ef.value(), 850);

    assert!(ef.skip(149).is_ok());
    assert!(ef.skip(150).is_err());
}

#[test]
fn test_reset() {
    const NUM: u64 = 1000;
    let mut ef = EliasFano::new(NUM, NUM);
    let array: Vec<u64> = vec![0; NUM as usize]
        .iter()
        .enumerate()
        .map(|(idx, _)| idx as u64)
        .collect();

    ef.compress(array.iter()).unwrap();

    if ef.position() != 0 {
        panic!("Initial position is not equal to 0");
    }

    let _ = ef.next();
    ef.reset();

    if ef.position() != 0 {
        panic!("Position was not reset correctly");
    }

    if ef.value() != 0 {
        panic!("Initial value is incorrect");
    }
}

#[test]
fn test_move() {
    const NUM: u64 = 1000;
    let mut ef = EliasFano::new(NUM, NUM);
    let array: Vec<u64> = vec![0; NUM as usize]
        .iter()
        .enumerate()
        .map(|(idx, _)| idx as u64)
        .collect();

    ef.compress(array.iter()).unwrap();

    if ef.position() != 0 {
        panic!("Initial position is not equal to 0");
    }

    for (idx, val) in array.iter().enumerate() {
        let _ = ef.visit(idx as u64);
        if ef.value() != *val {
            panic!("Received unexpected value after visit");
        }
    }

    for i in 0..NUM {
        let _ = ef.visit((array.len() - i as usize - 1) as u64);
        if ef.value() != array[array.len() - i as usize - 1] {
            panic!("Incorrect value found while visiting backwards");
        }
    }
}

#[test]
fn test_generic() {
    let mut ef = EliasFano::new(1000, 5);
    ef.compress([0, 5, 9, 800, 1000].iter()).unwrap();

    if ef.value() != 0 {
        panic!("Incorrect start value");
    }

    let _ = ef.visit(0);

    if ef.value() != 0 {
        panic!("0 visit returns different value");
    }

    let _ = ef.visit(4);

    if ef.value() != 1000 {
        panic!(
            "Visit returning incorrect value, expected: {}, received: {}",
            1000,
            ef.value()
        );
    }

    ef.reset();

    if ef.value() != 0 {
        panic!("Incorrect behaviour on reset");
    }

    let _ = ef.next();

    if ef.value() != 5 {
        panic!(
            "Next value is incorrect, expected: {}, received: {}",
            5,
            ef.value()
        );
    }

    let _ = ef.next();

    if ef.value() != 9 {
        panic!(
            "Next value is incorrect, expected: {}, received: {}",
            9,
            ef.value()
        );
    }

    let _ = ef.visit(1);

    if ef.value() != 5 {
        panic!(
            "Visit returning incorrect value, expected: {}, received: {}",
            5,
            ef.value()
        );
    }
}

#[test]
fn test_into_vec() {
    const NUM: u64 = 1000;
    let mut ef = EliasFano::new(NUM, NUM);
    let array: Vec<u64> = vec![0; NUM as usize]
        .iter()
        .enumerate()
        .map(|(idx, _)| idx as u64)
        .collect();
    ef.compress(array.iter()).unwrap();
    let vals = ef.into_vec();
    assert_eq!(array.len(), vals.len());
    assert_eq!(array, vals);
}
