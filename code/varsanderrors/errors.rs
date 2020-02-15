#![allow(non_snake_case)]

/*
Режим паники настраивается в Cargo.toml.

[profile.release]
panic = 'abort'

Это означает, что программа будет немедленно завершена вместо раскрутки стека.
Похоже, что у компилятора это опция "-C panic=abort"

Обратная трассировка (вывод StackTrace) достигается тем, что при компиляции
устанавливается переменная окружения (операционной системы) RUST_BACKTRACE=1
Также должны быть включены отладочные символы (cargo build без флага --release)
*/

use std::fs::File;
use std::io::ErrorKind;	// ErrorKind::NotFound
use std::io::Read;		// Для использования `std::fs::File.read_to_string

fn main()
{
	// Паникуем: аварийное завершение программы
	// panic!("crash");

	// Откроем файл
	println!("openTmpFile:");
	openTmpFile();
	
	// Ещё раз откроем файл, но чуть по-другому
	println!("openTmpFile2:");
	openTmpFile2();
	
	// Откроем файл и распространим результат вверх
	println!("openTmpFile3:");
	let _ = openTmpFile3();

	// Распространение ошибки с помощью оператора "?"
	println!("openTmpFile4:");
	let _ = openTmpFile4();
}

fn openTmpFile() -> i32
{
	// Откроем файл, а если он не открылся, создадим. Иначе - паника
	let f = File::open("tmp.txt");

	// Обрабатываем возможные ошибки
	// Варианты std::result::Result автоматически раскрыты для использования
	let _hFile = 
	match f
	{
		Ok(file) =>
		{
			println!("Файл успешно открыт {:?}", file);
			file
		},
		Err(e) =>
		{
			println!("{:?}", e);
			// use std::io::ErrorKind;
			if e.kind() == ErrorKind::NotFound
			{
				match File::create("tmp.txt")
				{
					Ok(fc) => 
					{
						println!("Файл успешно создан {:?}", fc);
						fc
					},
					Err(e) =>
					{
						println!("{:?}", e);
						return -1;
					}
				}
			}
			else
			{
				println!("{:?}", e);
				return -1;
			}
		}
	};

	0
}

// Можно обратить внимание на то, что в этом случае код ошибки вернуть некуда
fn openTmpFile2()
{
	let fileName = "tmp2.txt";

	let _hFile = File::open(fileName).unwrap_or_else
	(
		|e|	// Если ошибка, делаем лексическое замыкание - обработчик ошибки
		{
			if e.kind() == ErrorKind::NotFound
			{
				File::create(fileName).unwrap_or_else
				(
					|e|
					{
						panic!("{:?} error: ", e);
					}
				)
			}
			else
			{
				panic!("{:?} error: ", e);
			}
		}
	);
	
	// Вместо unwrap_or_else можно просто вызвать .unwrap()
	// В таком случае, если будет зафиксирована ошибка, unwrap просто вызовет панику
	// .expect(panic_message) - то же, что и unwrap(), но с сообщением для паники
}


// Здесь мы возвращаем как хороший, так и плохой результат
// Хороший заключается в том, что мы прочитали строку из файла и вернули её
// В плохом случае мы вернули файловую ошибку
fn openTmpFile3() -> Result<String, std::io::Error>
{
	// Откроем файл, а если он не открылся, создадим. Иначе - паника
	let fileName = "tmp3.txt";

	// Обрабатываем возможные ошибки
	// Варианты std::result::Result автоматически раскрыты для использования
	match File::open(fileName)
	{
		Ok(mut file) =>
		{
			println!("Файл успешно открыт {:?}", file);

			let mut s = String::new();
			// std::io::Read
			match file.read_to_string(&mut s)
			{
				Ok(_)  => Ok(s),
				Err(e) => Err(e)
			}
		},
		Err(e) =>
		{
			println!("{:?}", e);
			// use std::io::ErrorKind;
			if e.kind() == ErrorKind::NotFound
			{
				match File::create(fileName)
				{
					Ok(fc) => 
					{
						println!("Файл успешно создан {:?}", fc);

						let s = String::new();
						Ok(s)
					},
					Err(e) =>
					{
						println!("{:?}", e);
						Err(e)
					}
				}
			}
			else
			{
				println!("{:?}", e);
				Err(e)
			}
		}
	}
}

// То же, что и в openTmpFile3, но с оператором "?"
// Оператор "?" отдаёт значение, обёрнутое Ok,
// но если в "?" попадает значение Err, то он вызывает "return Err"
// Оператор "?" конвертирует тип полученной ошибки в тип возвращаемой функцией ошибки
// Для конвертации используется типаж From стандартной библиотеки
// Возвращаемое функцией значение должно быть типа Result, Option или другого типа,
// реализующего типаж std::ops::Try
fn openTmpFile4() -> Result<String, std::io::Error>
{
	// Откроем файл, а если он не открылся, создадим. Иначе - паника
	let fileName = "tmp4.txt";

	// Обрабатываем возможные ошибки
	// Варианты std::result::Result автоматически раскрыты для использования
	match File::open(fileName)
	{
		Ok(mut file) =>
		{
			println!("Файл успешно открыт {:?}", file);

			let mut s = String::new();
			// std::io::Read
			file.read_to_string(&mut s)?;
			Ok(s)
		},
		Err(e) =>
		{
			println!("Ошибка: {:?}", e);
			// use std::io::ErrorKind;
			if e.kind() == ErrorKind::NotFound
			{
				File::create(fileName)?;
				Ok(  String::new()  )
			}
			else
			{
				println!("Фатальная ошибка {:?}", e);
				Err(e)
			}
		}
	}
}

