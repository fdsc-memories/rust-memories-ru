// DoSIB5HLcFKAvqx5Kh: decl

use std::cmp::Ordering;

fn main()
{
	
	// Сравним числа с помощью стандартной библиотеки :)
	let mut num1 = 1;

	// Сравнение принимает ссылку на то, что можно сравнить
	let etalon:u64 = 2;
	match etalon.cmp(&num1)
	{
		// Вариант сравнения называется "рукав". В нём определён "шаблон кода" и "образец".
        Ordering::Less => println!("{} < {}", 2, num1),
        Ordering::Greater => println!("{} > {}", 2, num1),
        Ordering::Equal => println!("{} == {}", 2, num1),
    }

	num1 = 3;
	match etalon.cmp(&num1)
	{
		// Вариант сравнения называется "рукав". В нём определён "шаблон кода" и "образец".
        Ordering::Less => println!("{} < {}", 2, num1),
        Ordering::Greater => println!("{} > {}", 2, num1),
        Ordering::Equal => println!("{} == {}", 2, num1),
    }
}
