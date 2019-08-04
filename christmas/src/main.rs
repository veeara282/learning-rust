fn main() {
    for i in 1..13 {
        verse(i);
    }
}

fn verse(i: u8) {
    verse_header(i);
    verse_body(i);
    println!();
}

fn verse_header(i: u8) {
    println!(
        "On the {} day of Christmas,\nMy true love gave to me:",
        ordinal(i)
    );
}

fn verse_body(i: u8) {
    for j in (1..i + 1).rev() {
        println!("{}", verse_line(j));
    }
}

fn verse_line(i: u8) -> &'static str {
    match i {
        12 => "Twelve drummers drumming",
        11 => "Eleven pipers piping",
        10 => "Ten lords a-leaping,",
        9 => "Nine ladies dancing,",
        8 => "Eight maids a-milking,",
        7 => "Seven swans a-swimming,",
        6 => "Six geese a-laying,",
        5 => "Five golden rings,", // North American variant
        4 => "Four calling birds,",
        3 => "Three French hens,",
        2 => "Two turtle doves, and", // https://commons.wikimedia.org/wiki/File:12_days_angus.png
        1 => "A partridge in a pear tree!",
        0 => "Segmentation fault (core dumped)",
        _ => "Several bugs a-bugging",
    }
}

fn ordinal(i: u8) -> String {
    match i {
        1 => "1st".to_string(),
        2 => "2nd".to_string(),
        3 => "3rd".to_string(),
        _ => format!("{}th", i),
    }
}
