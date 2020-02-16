#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

// Функция может быть помечена unsafe вся, либо только её блок
fn main()
{
	unsafe
	{
		func();
	}
/*
	Указатель на статический адрес в памяти
	let address = 0x012345usize;
	let r = address as *const i32;
*/
}

// Вся функция целиком является unsafe-блоком
unsafe fn func()
{
	let mut num = 5;

	let r1 = &num as *const i32;
	let r2 = &mut num as *mut i32;

	println!("r1 is: {}", *r1);
	println!("r2 is: {}", *r2);
}
// Статическая глобальная переменная
// Тип указывать обязательно
static _HELLO_WORLD: &str = "Hello, world!";

// Доступ к статической изменяемой переменной является небезопасным и должен быть обрамлён unsafe
static mut _COUNTER: u32 = 0;

/*
// Импорт функции abs по соглашению о вызовах языка "C"
extern "C"
{
    fn abs(input: i32) -> i32;
}

// Экспорт функцииcall_from_c по соглашению о вызовах языка "C"
#[no_mangle]	// Сообщает компилятору о том, что функцию не нужно подвергать анализу, который её может повредить
pub extern "C" fn call_from_c()
{
    println!("Just called a Rust function from C!");
}

extern является словом, которое включает в свою семантику unsafe, то есть дополнительно указывать unsafe не нужно.
*/

unsafe trait _Foo
{
    // methods go here
}

unsafe impl _Foo for i32
{
    // method implementations go here
}
