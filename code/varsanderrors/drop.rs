#![allow(non_snake_case)]

// Типаж Drop
struct CustomSmartPointer
{
    data: String,
}

// Trait Drop - типаж для обработки выхода из области видимости
// Входит в прелюдию
impl Drop for CustomSmartPointer
{
    fn drop(&mut self)
	{
        println!("{}", self.data);
    }
}

use std::rc::Rc;
// Типаж Rc<T>

struct Au
{
	val: String
}

impl Au
{
	fn new(v: &str) -> Au
	{
		Au { val: String::from(v) }
	}
}

impl Drop for Au
{
    fn drop(&mut self)
	{
        println!("dropped {}", self.val);
    }
}

fn main()
{
	// Типаж Drop
	let _c = CustomSmartPointer { data: String::from("smart pointer 0") };
	let _d = CustomSmartPointer { data: String::from("smart pointer 1") };
	let _c = CustomSmartPointer { data: String::from("smart pointer 2") };
	let _e = CustomSmartPointer { data: String::from("smart pointer 3") };
	// _e.drop(); // Так нельзя
	drop(_e);		// std::mem::drop
	println!("CustomSmartPointers created.");
	/*
	smart pointer 3
	CustomSmartPointers created.
	dropped For Rc
	smart pointer 2
	smart pointer 1
	smart pointer 0
	*/

	let _u1 = Au::new("For Rc 1");
	let _u2 = Au::new("For Rc 2");

	let mut _r1 = Rc::new(&_u1);
	let mut _r2 = _r1.clone();
	let mut _r3 = _r2.clone();

	drop(_u1);
	// let mut _r4 = _r3.clone();
}
