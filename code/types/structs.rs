// DoSIB5HLcFKAvqx5Kh: decl
#![allow(non_snake_case)]

// Обычная структура
struct User
{
	_name:   String,
	_weight: u64
}

// Кортежная структура
struct Point(f32, f32);

// В определении структур разрешаются шаблонные типы
struct _WithTemplate<T, B>
{
	_a: T,
	_b: B
}
impl<T, B> _WithTemplate<T, B>
{
	fn _x(_a: T, _b: B)
	{
	}
}

// Добавим в User метод
// Блок имплементации
impl User
{
	// Первый параметр - self. Может быть как с mut, так и без, если не нужно менять структуру
	// self придётся указывать всегда, короткой записи без self нет
	//  (метод заберёт у кода владение self, затем вернёт назад)
	fn setWeight(&mut self, newWeight: u64)
	{
		self._weight = newWeight;
	}

	// Так тоже можно, см. ниже пример
	// Этот метод забирает владение self, насколько я понимаю, не копируя переменную по значению
	// То есть код, который вызвал этот метод, не сможет дальше пользоваться self
	fn getNewUser(mut self, name: String) -> User
	{
		self._name = name;
		self
	}

	// Метод принимает неизменяемую структуру
	fn print(&self)
	{
		println!("User '{}': {}", self._name, self._weight);
	}
}

// Можно сделать ещё один блок impl
impl User
{
	// Ассоциированная функция
	// User::static_a();
	// Можно использоваться как конструктор, но не обязательно
	fn static_a() -> User
	{
		User
		{
			_name:   String::from(""),
			_weight: 123456789
		}
	}
}

fn main()
{
	// Неизменяемую ссылку можно потом конвертировать в изменяемую (странно, кстат)
	// Весь экземляр структуры либо изменяемый, либо нет. Поля помечать mutable не получится.
	let _user1 = User::static_a();

	//  cannot assign to `_user1.name`, as `_user1` is not declared as mutable
	// _user1.name = String::from("aaa");

	let mut _user1 = _user1;
	_user1._name = String::from("aaa");
	let mut _user2 = _user1.getNewUser(String::from("New user"));

	// _user1.setWeight(12); // это теперь нельзя, так как getNewUser забрала владение self
	_user2.setWeight(12);
	_user2.print();

	// Создание кортежной структуры
	let _p = Point(0.0, 0.0);

	// Попробуем использовать структуру для контроля состояния программы на этапе компиляции
	control();
}

fn _newUser(_name: String) -> (User, User)
{
	let preUser = User
	{
		_name,	// Быстрая инициализация из параметра функции
		_weight: 64
	};
	
	(
		User
		{
			_name: String::from(""),
			..preUser	// Все незаполненные поля структуры заполняются также, как в preUser
			// Скопировать так _name не получится, т.к. это ссылка и её надо клонировать вручную
			// Иначе она будет заимствоваться. Это можно делать, только если preUser уже не нужна.
		},
		preUser
	)
}


// Попробуем использовать структуру для контроля состояния программы на этапе компиляции
struct Control
{
	state1: String,
	state2: String,
	state3: String,
	owner:  String
}

fn control()
{
	// Инициализируем структуру так, чтобы в структуре осталась только одна ссылка
	let mut state = Control
	{
		state1: String::from(""),
		state2: String::from(""),
		state3: String::from(""),
		owner:  String::from(""),
	};

	state.state3 = state.owner;
	state.state2 = state.state3;
	state.state1 = state.state2;

	// Структура инициализирована
	// Состояние 1
	// Вызываем для проверки функцию
	state.state1.len();	// 1-1
	// state.state2.len();	// 2-1 - это выдаст ошибку
	
	// Переходим в другое состояние
	state.state2 = state.state1;

	// state.state1.len();	// 1-2 - это выдаст ошибку
	state.state2.len();	// 2-2
	
	// Переходим в другое состояние
	state.state3 = state.state2;

	// state.state1.len();	// 1-3 - это выдаст ошибку
	// state.state2.len();	// 2-3 - это выдаст ошибку
	state.state3.len();	// 3-3

	state.state1 = state.state3;

	state.state1.len();	// 1-4
	// state.state2.len();	// 2-4 - это выдаст ошибку
	// state.state3.len();	// 3-4 - это выдаст ошибку
}
