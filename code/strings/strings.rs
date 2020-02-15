#![allow(non_snake_case)]

/*
В ядро языка встроен только срез str.
Сами же строки типа String встроены в стандартную библиотеку.
String хранят значения в кодировке UTF-8 - это стандартная строка
https://doc.rust-lang.org/std/string/struct.String.html

Есть также строки OsString, OsStr, CString и CStr
Также есть сторонние реализации, например, https://crates.io/crates/widestring

Строка String может быть увеличена в размере, аналогично классу Vec (расширяемый массив)

*/

fn main()
{
	firstWord();

	// Срезы
	slice();
	
	
	// Создаём пустую строку. В константах можно использовать любые UTF-8 символы
	let mut _s  = String::new();
	let mut _s2 = "initial contents®©".to_string();
	let mut _s2 = String::from("initial contents®©");

	let mut _st = "123456789";

	// Добавить в конец строковый срез
	_s2.push_str("added");
	_s .push_str(&_s2);
	_s .push    ('s');	// Добавили один символ
	
	// Первая строка - String. Остальные - строковые срезы
	// ссылка на _s2 принудительно приведена к строковому срезу
	// _s перемещена (забрано владение) в _s3, остальные заимствованы и отданы назад
	let mut _s3 = _s + &_s2 + _st + "aaa bbb";

	println!("{}", _s3);
	// Ввывод:
	// initial contentsRcaddedsinitial contentsRcadded123456789aaa bbb


	let _s = "aaaaa bbbbb".to_string();

	// Не забирает вообще ничего во владение
	// Аналогично println!, только вывод на консоль отсутствует: просто форматирование
	let _s = format!("{}-{}-{}", _s, _s2, _st);
	println!("{}", _s);
	// aaaaa bbbbb-initial contentsRcadded-123456789

	// Так не получится. Строки не поддерживают индексацию, только срезы
	// let h = _s[0];
	
	let c = "Здравствуйте str";
	for _e in c.chars()
	{
		// Выведет каждый символ по одному
		// println!("{}", _e);
	}

	// Итератор по байтам
	// for b in c.bytes()
}

// Заодно найдём первое слово (до пробела) в предложении
fn firstWord() -> usize
{
	let s = String::from("Слово\u{2021}1 Word2");
	let a = s.as_bytes();

	for _e in a.iter()
	{
		println!("a.iter(): {}", _e);
	}
/*
a.iter(): 208
a.iter(): 161
a.iter(): 208
a.iter(): 187
a.iter(): 208
a.iter(): 190
a.iter(): 208
a.iter(): 178
a.iter(): 208
a.iter(): 190
a.iter(): 226
a.iter(): 128
a.iter(): 161
a.iter(): 49
a.iter(): 32
a.iter(): 87
a.iter(): 111
a.iter(): 114
a.iter(): 100
a.iter(): 50
*/

	let mut len = s.len();
	for (i, &e) in a.iter().enumerate()
	{
		// println!("a.iter(): a[{}] == {}", i, e);	// i - индекс, e - элемент
		if e == b' ' || e == b'\t' || e < 32
		{
			len = i;
			break;
		}
	};

	len
}

// Срезы нужны для того, чтобы привязать переменные, обозначающие границы найденных подстрок
// к самой строке (для обеспечения большей надёжности, неотрывности индексов от существования и неизменности строки)
// Тип сторокового среза - это тип с именем &str
fn slice()
{
	let mut s = String::from("Слово\u{2021}1 Word2");

	/*
	Это сообщение
	thread 'main' panicked at 'byte index 5 is not a char boundary; it is inside 'о'
	(bytes 4..6) of `Слово╪1 Word2`', src\libcore\str\mod.rs:2069:5
	будет, если раскомментировать строку ниже со срезом "& s[0 .. 5]"
	То есть мы сделали срез, граница которого проходит внутри UTF-8 символа
	*/

	// Создаём срез с 0 по 5 байт, не включая 5-ый байт (то есть 5 байтов)
	// let _slice = & s[0 .. 5];

	// [0 .. 6] == [.. 6] Берём первые 6-ть БАЙТОВ, не символов!
	// [6 .. len] == [6 ..] Берём байты, начиная с 6-ого до конца строки

	let _slice = & s[0 .. 6];
	let full    = & s[..];
	
	// s.clear();
	// error[E0502]: cannot borrow `s` as mutable because it is also borrowed as immutable
	// s нельзя изменить, потому что на него уже есть другие ссылки только для чтения
	// (ссылки в виде срезов)

	println!("Срез  '{}', Срез всей строки '{}'", _slice, full);

	s.clear();

	// _slice - это срез
	// Чтобы это проверить можно его даже явно задекларировать
	let _slice: &str = "this is slice";

	// Если мы объявляем функцию, которая принимает срез, мы делаем её более общей
	// чем если бы она принимала строку
	let s = String::from("Эта строка передана в виде строкового среза");
	getsliceInsteadString(&s);
}

fn getsliceInsteadString(Str: &str)
{
	println!("{}", Str);
}
