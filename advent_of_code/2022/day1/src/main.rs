
fn main() {

    // initial input
    let calories_input =
"1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";

    println!("calories_input={}", calories_input);

    let elf_calories =
        calories_input
            .split("\n\n")
            .collect::<Vec<_>>();
    println!("elf_calories={:?}", elf_calories);

    let calories_per_elf =
        elf_calories
            .iter()
            .map(|calorie_str|
                calorie_str
                    .split_whitespace()
                    .collect::<Vec<_>>()
            )
            .collect::<Vec<_>>();

    println!("calories_per_elf={:?}", &calories_per_elf);

    let per_elf_total_calories =
        calories_per_elf
            .into_iter()
            .map(|calories| {
                let total =
                    calories
                        .into_iter()
                        .map(|c|
                            c.parse::<i32>().unwrap()
                        )
                        .collect::<Vec<_>>();
                total
            }).map(|calories| {
                calories.iter().sum::<i32>()
            })
            .into_iter()
            .collect::<Vec<_>>();

    println!("per_elf_total_calories={:?}", &per_elf_total_calories);

    let max_calories_carried_by_elf = per_elf_total_calories.iter().max();

    match max_calories_carried_by_elf {
        Some(max_value) => {
            println!("max calories {}", max_value.clone());
            let expected: &i32 = &24000;
            assert_eq!(max_value, expected)
        },
        None => println!("Should not happen"),
    }

}
