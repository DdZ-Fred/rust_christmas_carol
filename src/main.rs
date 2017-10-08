struct Day {
    nth: String,
    txt: String,
}

fn print_n_th_start(days: &[Day;12], n: u8) {
    println!("On the {} day of Christmas\nmy true love sent to me:", days[n as usize].nth);
}

fn print_n_th_body(days: &[Day;12], n: u8) {
    match n {
        0u8 => print!("{}\n\n", days[0].txt),
        _ => {
            let mut index = n;
            while index > 0 {
                println!("{}", days[index as usize].txt);
                index -= 1;
            }
            println!("and {}\n", days[0].txt);
        },
    }
}

fn main() {
    let mut i: u8 = 0;

    let days: [Day; 12] = [
        Day {
            nth: String::from("first"),
            txt: String::from("a Partridge in a Pear Tree")
        },
        Day {
            nth: String::from("second"),
            txt: String::from("2 Turtle Doves")
        },
        Day {
            nth: String::from("third"),
            txt: String::from("3 French Hens")
        },
        Day {
            nth: String::from("fourth"),
            txt: String::from("4 Calling Birds")
        },
        Day {
            nth: String::from("fifth"),
            txt: String::from("5 Golden Rings")
        },
        Day {
            nth: String::from("sixth"),
            txt: String::from("6 Geese a Laying")
        },
        Day {
            nth: String::from("seventh"),
            txt: String::from("7 Swans a Swimming")
        },
        Day {
            nth: String::from("eighth"),
            txt: String::from("8 Maids a Milking")
        },
        Day {
            nth: String::from("ninth"),
            txt: String::from("9 Ladies Dancing")
        },
        Day {
            nth: String::from("tenth"),
            txt: String::from("10 Lords a Leaping")
        },
        Day {
            nth: String::from("eleventh"),
            txt: String::from("11 Pipers Piping")
        },
        Day {
            nth: String::from("twelfth"),
            txt: String::from("12 Drummers Drumming")
        }
    ];

    while i <= 11 {
        print_n_th_start(&days, i);
        print_n_th_body(&days, i);
        i += 1;
    }
}