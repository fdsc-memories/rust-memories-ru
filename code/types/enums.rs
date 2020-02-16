// DoSIB5HLcFKAvqx5Kh: decl
#![allow(non_snake_case)]
// #[allow(non_camel_case_types)]

/*
:::match  - описывает конструкции match и типы std::result::Result и std::cmp::Ordering
:::iflet  - описывает конструкцию if let
:::declarations - описывает объявления перечислений
std::option::Option
*/


use std::cmp::Ordering;

fn main()
{
	
// -------------------------------- :::match --------------------------------

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
		// Можно ввести _err вместо _ ; Здесь мы декларируем, что нам не нужно это значение
        Err(_)   =>
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
	
	let x:Option<String> = None;
	match x
	{
		// Придётся указать скобки в любом случае, я так понимаю
		Some(_) => return,
		None    => 1+1
	};
	
	// -------------------------------- :::iflet --------------------------------

	// Если x соответствует шаблону Some
	if let Some(_) = x
	{
		return;
	}
	else
	{}
	
	// while let тоже есть

	// -------------------------------- Декларации и т.п. --------------------------------
	
	next1();
}

// :::declarations

// Это обычное перечисление
enum Version
{
	New,
	Old,
	_Outdated
}

// Декларация, где каждому значению перечисления соответствуют сохранённые данные
enum User
{
	Admin(Version, String, String),
	Auth (Version, String),
	Guest
}

impl User
{
	fn newAdmin(auth1: String, auth2: String) -> User
	{
		return User::Admin(Version::New, auth1, auth2);
	}

	// Если напишем &self, то будут неверные типы у auth (&String вместо String)
	// self - берёт объект во владение и не отдаёт, то есть это последний метод
	fn toOldUser(self) -> User
	{
		match self
		{
			User::Admin(_v, auth1, auth2) =>
			{
				User::Admin(Version::Old, auth1, auth2)
			},
			User::Auth(_v, auth1) =>
			{
				User::Auth(Version::Old, auth1)
			},
			_ => panic!("toOldUser with incorrect User type")
		}
	}
}

fn next1()
{
	let v   = Version::Old;
	let _u1 = User::newAdmin(   String::from(""), String::from(""));
	let _u2 = User::Auth    (v, String::from(""));
	let _u3 = User::Guest;
	
	let _u4 = _u1.toOldUser();
	// Второй раз сделать так не получится, т.к. _u1 перемещён, т.к. он брал не ссылку, а саму переменную
	// _u1.toOldUser();
}


// std::option::Option
/*	Определение класса из стандартной библиотеки
pub enum Option<T>
{
    None,
    Some(T),
}
Использование по умолчанию возможно даже без указания имени типа Option: просто None и Some

Содержит методы, см.https://doc.rust-lang.org/std/option/enum.Option.html
*/

