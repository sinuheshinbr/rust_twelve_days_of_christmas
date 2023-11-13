fn main() {
    let gifts = [
        "A partridge in a pear tree",
        "Two turtle doves",
        "Three French hens",
        "four calling birds",
        "five gold rings",
        "six geese a-laying",
        "seven swans a-swimming",
        "eight maids a-milking",
        "nine ladies dancing",
        "ten lords a-leaping",
        "eleven pipers piping",
        "twelve drummers drumming",
    ];

    let ordinal = [
        "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eigth", "ninth",
        "tenth", "eleventh", "twelfth",
    ];

    let mut lyrics = String::new();

    for (i, gift) in gifts.iter().enumerate() {
        let current_lyrics = lyrics.clone();

        let new_verse = format!(
            "On the {} day of Christmas\nmy true love sent to me \n{}\n",
            ordinal[i], gift
        );

        lyrics = format!("{}\n{}", current_lyrics, new_verse);
        println!("{lyrics}");
    }
}
