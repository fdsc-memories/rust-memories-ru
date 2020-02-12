#![allow(non_snake_case)]

fn main()
{
	firstWord();

	// Срезы
	slice();
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
	будет, если раскомментировать строку ниже
	То есть мы сделали срез, граница которого проходит внутри UTF-8 символа
	*/

	// Создаём срез с 0 по 5 байт, не включая 5-ый байт (то есть 5 байтов)
	// let _slice = & s[0 .. 5];

	// [0 .. 6] == [.. 6] Берём первые 6-ть символов
	// [6 .. len] == [6 ..] Берём символы, начиная с 6-ого до конца строки

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
