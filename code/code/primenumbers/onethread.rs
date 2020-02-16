#![allow(non_snake_case)]

use std::fs::File;
use std::io::BufWriter;
use std::io::prelude::*;
use std::io::SeekFrom;

fn main()
{
	println!("Программа для расчёта простых чисел в одном потоке путём последовательного деления");

	// PN - массив простых чисел
	let mut PN: Vec<u64> = vec![3]; // vec![2, 3, 5, 7, 11, 13, 17, 19];

	let mut len   = PN.len();
	let mut num   = PN[len - 1];
	let mut cnt   = 0;
	let mut start = 0;

	let fileName = "primenumbers1.txt";
	let mut f = File::create(fileName).unwrap();
	let mut writer = BufWriter::with_capacity(10_000_000, f); //BufWriter::new(f);

	loop
	{
		num += 2;

		let mut flag = true;
		for e in &PN
		{
			if e << 1 > num
			{
				break;
			}

			if num % e == 0
			{
				flag = false;
				break;
			}
		}

		len = PN.len();
		if flag
		{
			PN.push(num);
			cnt += 1;

			writer.seek(SeekFrom::Start(0)).unwrap();
			let mut i:usize = 0;
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

				let k = (i << 1) + 3;
				writer.write(format!("{}\r\n", k).as_bytes()).unwrap();

				i += 1;
			}
			writer.flush().unwrap();
		}
	}
}