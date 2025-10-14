// Leave to see what compiler will do
// #![allow(unreachable_code, unused_labels)]
fn main() {
    println!("Control flow");

    // if/else
    // let mut n = 10;
    // let mut n = 0;
    let mut n = -123;

    if n < 0 {
        println!("N jest negatywne");
    } else if n > 0 {
        println!("N jest pozytywne");
    } else {
        println!("N to zero :O");
    }
    
    // n = 123;
    n = -140;

    let big_n = if n < -100 && n > 100 {
        println!("N jest super mocno negatywne :(");
        n * 100
    } else {
        println!("N jest super mocno pozytywne! :)");
        n * 100
    }; // semicolon at the end of expression! (not statement)
    println!("... i wynosi: {}", big_n);

    // pętla - loop
    let mut count = 50u8;
    
    loop {
        count += 1;

        if count == 69 {
            println!("Pomińmy to...");
            println!("... i przeskoczmy trochę do przodu");
            count = 240;
            continue;
        }

        if count % 3 == 0 {
            println!("{} jest podzielne przez 3", count);
        }


        if count % 2 == 0 {
            println!("{} jest parzyste!", count);
        }

        // Comment to see panic
        if count == 255 {
            println!("Nie szalejemy...");
            break;
        }
    }

    'outer: loop {
        println!("Entering outer loop");
        'inner: loop {
            println!("Entering the inner loop");
            
            break 'outer;
        }
        println!("Unreachable code");
    }

    println!("Exited the outer loop");

    // Loops are expressions too
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 68 {
            break counter + 1;
        }
    }; // Expressions need semicolon

    assert_eq!(result, 69);

    // while
    let mut m = 10;

    while m < 20 {
        if m % 2 == 0 {
            println!("M jest parzyste! {}", m);
        }

        m += 1;
    }

    // for loop
    for n in 1..10 {
        println!("n = {}", n);
    }

    println!("Let's be more inclusive");
    // inclusive end
    for n in 1..=10 {
        println!("n = {}", n);
    }

    // for loops and iterators
    let names = vec!["Kasia", "Mariusz", "Janusz", "Ferris"];

    for name in names.iter() {
        match name {
            &"Ferris" => println!("Znaleziono dzikiego Ferrisa!"),
            // "Ferris" => println!("Znaleziono dzikiego Ferrisa!"),
            // Switch above statements to see what'll happen
            _ => println!("Hello, {}", name),
        }
    }

    println!("names: {:?}", names);

    // Lets consume the collection
    for name in names.into_iter() {
        match name {
            "Ferris" => println!("Kolejny dziki Ferris..."),
            _ => println!("Hello {}", name),
        }
    }

    // println!("names: {:?}", names);
    // FIXME! Comment out above line

    // Lets modify the collection in place
    let mut samochody = ["Toyota", "Ford", "Nissan", "Tesla", "Polonez"];

    println!("samochody: {:?}", samochody);

    for samochód in samochody.iter_mut() {
        *samochód = match samochód {
            &mut "Polonez" => "Duma polskiej motoryzacji!",
            _ => "Jakieś auto ugh.",
        }
    }

    println!("samochody: {:?}", samochody);

    // match
    let liczba = 13;
    // let liczba = 123;

    match liczba {
        13 => println!("Trzynastka!"),
        2 | 3 | 5 | 7 | 11 => println!("Liczba pierwsza"),
        _ => println!("W sumie to nie wiem"),
    }
    let triple = (0, -2, 3);
    // TODO ^ Try different values for `triple`

    println!("Tell me about {:?}", triple);
    // Match can be used to destructure a tuple
    match triple {
        // Destructure the second and third elements
        (0, y, z) => println!("First is `0`, `y` is {:?}, and `z` is {:?}", y, z),
        (1, ..)  => println!("First is `1` and the rest doesn't matter"),
        (.., 2)  => println!("last is `2` and the rest doesn't matter"),
        (3, .., 4)  => println!("First is `3`, last is `4`, and the rest doesn't matter"),
        // `..` can be used to ignore the rest of the tuple
        _      => println!("It doesn't matter what they are"),
        // `_` means don't bind the value to a variable
    }

    // Dereferencing uses *
    // Destructuring uses &, ref and ref mut
    let reference = &4;

    match reference {
        &val => println!("Got a value via destructuring: {:?}", val),
    }

    match *reference {
        val => println!("Got a value via dereferenginc: {:?}", val),
    }

    let _not_a_reference = 3;

    let ref _is_a_reference = 3;

    let value = 5;
    let mut mut_value = 6;

    match value {
        ref r => println!("Got a reference to a value: {:?}", r),
    }

    match mut_value {
        ref mut m => {
            *m += 10;
            println!("We added 10. 'mut_value': {:?}", m);
        }
    }
}

