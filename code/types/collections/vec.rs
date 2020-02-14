#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

#[derive(Debug)]
enum ArrayElement
{
	i32(i32),
	i64(i64)
}

fn main()
{
	// Реализация вектора обсуждается здесь
	// https://doc.rust-lang.org/stable/nomicon/vec.html

	// Vec - это список элементов одного типа: по сути, это расширяемый массив
	// https://doc.rust-lang.org/std/vec/struct.Vec.html
	let _v: Vec<i32> = Vec::new();

	// Вывод типов понимает, какого типа данный массив
	let mut _v = Vec::new();

	// добавляем элементы
	_v.push(1);
	_v.push(2);
	_v.push(3);

	// Вектор сразу же инициализируется значением из массива
	let _v = vec![1, 2, 3];
	
	println!("{:?}", _v);
	// [1, 2, 3]

	// Векторы индексируются начиная с нуля
	let mut first = &_v[0];
	// value assigned to `first` is never read - предупреждение компиляции

	// Это не изменит элемента массива, т.к. мы перезаписываем ссылку на элемент
	let x = 0;
	first = &x;
	// как надо: см. ниже  let r = &mut b[0];

	// Это не получится, т.к. объявили без mut
	// _v[0] = 0;

	println!("{:?}", _v);
	// [1, 2, 3]

	let mut _v = _v;
	_v[0] = 5;

	// thread 'main' panicked at 'index out of bounds: the len is 3 but the index is 5'
	//_v[5] = 6;

	println!("{:?}", _v);
	// [5, 2, 3]

	/*
		Получаем элементы массива без риска паники
		Вывод
		v[0] == 5
		v[5] not found
	*/
	get(&_v, 0);
	get(&_v, 5);
	
	first = &_v[2];
	
	// Это нельзя
    // _v.push(1);
	// cannot borrow `_v` as mutable because it is also borrowed as immutable
	// Ссылка на элемент вектора есть ссылка и на сам вектор,
	// т.к. массив значений может быть перевыделен при добавлении нового элемента
	println!("{}", first);

	// Теперь можно, т.к. ссылка от first вернулась
	_v.push(1);

	// Итератор по векторам (никаких iter вызывать не надо :) )
	let a = vec!["Первый", "Второй", "Третий"];
	for e in &a
	{
		println!("{}", e);
	}
	
	// В итераторе можно менять значения
	let mut b = vec![0, 0, 0, 0];
	let mut x = 0;
	for e in &mut b
	{
		*e = x;
		x  = x + 1;
	}
	
	let r = &mut b[0];
	*r += 1;

	println!("{:?}", b);
	// [1, 1, 2, 3]

	let b = vec![ArrayElement::i32(0), ArrayElement::i64(1)];
	println!("{:?}", b);
	// [i32(0), i64(1)]
}

fn get<T: std::fmt::Display>(v: &Vec<T>, index: usize)
{
	match v.get(index)
	{
		Some(e) => println!("v[{}] == {}", index, e),
		None    => println!("v[{}] not found", index),
	};
}
