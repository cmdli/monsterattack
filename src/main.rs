
fn main() {
    for limit in 1..13 {
        println!("On the {} day of Christmas my true love gave to me...",
                 limit);    
        for num in (1..limit+1).rev() {
            println!("{}", match num {
                1 => {
                    if limit == 1 {
                        "a partridge in a pear tree!"
                    } else {
                        "and a partridge in a pear tree!"
                    }
                },
                2 => "two turtle doves,",
                3 => "three French hens,",
                4 => "four calling birds,",
                5 => "5 gold rings!",
                6 => "6 geese a-laying,",
                7 => "7 swans a-swimming,",
                8 => "8 maids a-milking,",
                9 => "9 ladies dancing,",
                10 => "10 lords a-leaping",
                11 => "11 pipers piping,",
                12 => "12 drummers drumming,",
                _ => "Error! Error! Error!",
            });
        }
        println!("------------------");
    }
}
