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

    let elf_total_calories =
        elf_calories
            .iter()
            .map(|calorieStr|
                calorieStr
                    .split_whitespace()
                    .collect::<Vec<_>>()
            )
            .collect::<Vec<_>>();

    println!("elf_total_calories={:?}", &elf_total_calories);

    let elf_totals =
        elf_total_calories
            .into_iter()
            .flat_map(|calories| {
                let total =
                    calories
                        .into_iter()
                        .map(|c|
                            c.parse::<i32>().unwrap()
                        )
                        .collect::<Vec<_>>()
                        .into_iter()
                        .sum::<i32>();
                total
            })
            .collect::<Vec<_>>();

    println!("elf_totals={:?}", &elf_totals);
}
