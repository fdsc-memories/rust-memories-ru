#![allow(non_snake_case)]

use std::fs::File;
use std::io::BufWriter;
use std::io::prelude::*;
use std::convert::TryFrom;

fn main()
{
	println!("Программа для расчёта простых чисел в одном потоке методом решета Эратосфена");

	// PN - массив простых чисел
	let mut PN: Vec<u8> = vec![1; 1_000_000]; // vec![2, 3, 5, 7, 11, 13, 17, 19];

	let mut start = 0.0;
	let mut cnt;
	let mut num;

	/*
	loop
	{
		cnt += 1;
		PN[cnt] = 1;

		if cnt > 1_000_000_000
		{
			break;
		}
	}
*/
	let len = PN.len();
	//let len = 111;
	println!("Инициализация закончена");
	cnt = 0;

	loop
	{
		let index = cnt;
		cnt += 1;

		num = (index << 1) + 3;

		if index >= len
		{
			break;
		}

		if PN[index] == 0
		{
			continue;
		}

		let mut i = index;
		loop
		{
			i += num;
			if i >= len
			{
				break;
			}

			if PN[i] == 1
			{
				PN[i] = 0;
			}
		}

		const X: f64 = 0.20;
		let fc: f64 = f64::try_from(u32::try_from(cnt).unwrap()).unwrap();
		if fc.powf(X) * 100.0 / f64::from(u32::try_from(len).unwrap()).powf(X) >= start + 1.0
		{
			start = fc.powf(X) * 100.0 / f64::from(u32::try_from(len).unwrap()).powf(X);
			println!("Выполнено {}%", start);
		}
		
/*
		if (cnt >= 100)
		{
			// println!("{:?}", &PN[0..100]);
			i = 0;
			loop
			{
				i += 1;
				if (PN[i] == 0)
				{
					continue;
				}

				// println!("{}", (i << 1) + 3);
				if (i > 100)
				{
					break;
				}
			}
			break;
		}
*/
	}

	println!("Выполняется запись на диск");

	let fileName = "primenumbers1e.txt";
	let f = File::create(fileName).unwrap();
	let mut writer = BufWriter::with_capacity(10_000_000, f); //BufWriter::new(f);
	let mut i = 0;
	let mut start = 0;

	// Записываем двойку, которой нет в массиве
	writer.write(format!("{}\r\n", 2).as_bytes()).unwrap();
	loop
	{
		if i >= len
		{
			break;
		}

		if PN[i] == 0
		{
			i += 1;
			continue;
		}

		num = (i << 1) + 3;
		writer.write(format!("{}\r\n", num).as_bytes()).unwrap();

		i += 1;

		if i * 100 / len > start
		{
			start = i * 100 / len;
			println!("Записано {}%", start);
		}
	}
	writer.flush().unwrap();

	println!("Числа записаны в файл {}", fileName);
}
