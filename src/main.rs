fn main() {
    let gifts = [
        "a partridge in a pear tree",
        "Two turtle doves",
        "Three French hens",
        "Four calling birds",
        "Five gold rings",
        "Six geese a-laying",
        "Seven swans a-swimming",
        "Eight maids a-milking",
        "Nine ladies dancing",
        "Ten lords a-leaping",
        "Eleven pipers piping",
        "Twelve drummers drumming",
    ];

    let ordinal = [
        "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eigth", "ninth",
        "tenth", "eleventh", "twelfth",
    ];

    for day in 1..=12 {
        println!(
            "On the {} day of Christmas\nmy true love sent to me:",
            ordinal[day - 1],
        );
        for gift in (1..=day).rev() {
            match gift {
                _ if gift == 1 => println!(
                    "{}{}\n",
                    if day != 1 { "and " } else { "" },
                    gifts[gift - 1]
                ),
                _ => println!("{}", gifts[gift - 1]),
            }
        }
    }
}
