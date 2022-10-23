/*
    Code to print Twelve days to christ carol song to console
    Author: James Faith Shameh
*/

fn main() {
    let days = [
        "first", "Second", "third", "fourth", "fifth", "sixth", "seventh", "eigth", "ninth",
        "tenth", "eleventh", "twelfth",
    ];

    let mut lyrics = [
        "A song and a Christmas tree",
        "(Two candy canes)",
        "(Three boughs of holly)",
        "(Four colored lights)",
        "(A shining star)",
        "(Little silver bells)",
        "(Candles a glowing)",
        "(Gold and silver tinsel)",
        "(A guardian angel)",
        "(Some mistletoe)",
        "(Gifts for one and all)",
        "(All their good wishes)",
    ];

    // loop through each day in the twelve days
    for (i, day) in days.iter().enumerate() {
        println!(
            "\nOn the {} day of Christmas
My good friends brought to me",
            day
        );

        // change the first lyrics after the first day passes
        if i == 1 {
            lyrics[0] = "And a song for the christmas tree"
        }

        // print lyrics from array based on days lyrics
        for x in (0..i + 1).rev() {
            println!("{}", lyrics[x])
        }

        print!("\n");
    }
}
