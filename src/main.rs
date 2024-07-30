fn main() {
    let gifts = vec![
        "a Partridge in a Pear Tree",
        "Two Turtle Doves",
        "Three French Hens",
        "Four Calling Birds",
        "Five Golden Rings",
        "Six Geese a-Laying",
        "Seven Swans a-Swimming",
        "Eight Maids a-Milking",
        "Nine Ladies Dancing",
        "Ten Lords a-Leaping",
        "Eleven Pipers Piping",
        "Twelve Drummers Drumming",
    ];

    for (day, gift) in (1..=12).zip(gifts.clone()) {
        println!("On the {} day of Christmas, my true love gave to me", day);
        for (i, g) in gifts.clone().into_iter().take(day).enumerate() {
            println!("\t{} {}", i + 1, g);
        }
    }
}
