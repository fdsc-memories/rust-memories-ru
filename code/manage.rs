use std::cmp::Ordering;

fn main()
{

// -------------------------------- loop --------------------------------

	let mut i = 0;
	loop
	{
		println!("Число {}", i);
		if i >= 3
		{
			break;
		}

		if i == 0
		{
			i += 2;
			continue;
		}

		i += 1;
	}
	/*
		Вывод:
		Число 0
		Число 2
		Число 3
	*/


	let a = [1, 2, 3, 4, 5];

	let mut index = 0;
	let element5 = loop
	{
		if a[index] == 4
		{
			break (index, a[index]);
		}

		index += 1;
	};

	// a[3] == 4
	println!("a[{}] == {}", element5.0, element5.1);

// -------------------------------- match --------------------------------

	let str1 = String::from("aaa");
	let str2 = String::from("0065536");

	let mut num1:u64 = 0;

	// let str1:u64 = match возвращает результат! (см. далее)
	// В таком случае нужно поставить ; после match, а уточнение типа в parse уже не нужно - будет само выведено
	match str1.parse::<u64>()
	{
		// Вариант сравнения называется "рукав". В нём определён "шаблон кода" и "образец".
        std::result::Result::Ok (num)  => num1 = num,
        std::result::Result::Err(err) => println!("Не число! Сообщение об ошибке:\r\n{}", err),
    }

	let num2:u64 = match str2.parse()
	{
		// Вариант сравнения называется "рукав". В нём определён "шаблон кода" и "образец".
        Ok (num) => num,
        Err(_)   => 	// Можно ввести _err вместо _ ; Здесь мы декларируем, что нам не нужно это значение
		{
			println!("Не число! Введённая строка: {}", str2);
			0
		},
    };

	let etalon:u64 = 2;
	match etalon.cmp(&num1)
	{
		// Вариант сравнения называется "рукав". В нём определён "шаблон кода" и "образец".
        Ordering::Less    => println!("{} < {}",  2, num1),
        Ordering::Greater => println!("{} > {}",  2, num1),
        Ordering::Equal   => println!("{} == {}", 2, num1),
    }

	match etalon.cmp(&num2)
	{
		// Вариант сравнения называется "рукав". В нём определён "шаблон кода" и "образец".
        Ordering::Less => println!("{} < {}", 2, num2),
		_ => println!("{} >= {}", 2, num2),
    }
}
