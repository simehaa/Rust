fn main() {
    let sentences = [
        "Twelve drummers drummin'",
        "Eleven pipers pipin'",
        "Ten lords a leapin'",
        "Nine ladies dancin'",
        "Eight maids milkin'",
        "Seven swans a swimmin'",
        "Six geese a layin'",
        "Five golden rings",
        "Four calling birds",
        "Three french hens",
        "Two turtle doves and",
        "A partridge in a pear tree",
    ];

    let counts = [
        "first",
        "second",
        "third",
        "fourth",
        "fifth",
        "sixth",
        "seventh",
        "eighth",
        "ninth",
        "tenth",
        "eleventh",
        "twelfth",
    ];

    for (i, count) in counts.iter().enumerate() {
        println!("\nOn the {} day of Christmas", count);
        println!("My true love sent to me");
        for sentence in &sentences[12 - i - 1..] {
            println!("{}", sentence);
        }
    }
}
