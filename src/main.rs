extern crate sysfs_gpio;
extern crate chrono;

use sysfs_gpio::{Direction, Edge, Pin};
use std::io::{self, Write};
use std::env;
use std::io::stdout;
use chrono::prelude::*;

fn interrupt(pin: u64) -> sysfs_gpio::Result<()> {
    let input = Pin::new(pin);
    let mut ns:i64 = 0;
    let mut w:i64 = 0;
    let mut x:i64 = 0;
    let mut y:i64 = 0;
    let mut z:i64 = 0;
    //let mut difference1:i64 = 0;
    //let mut difference2:i64 = 0;
    let mut count:i64 = 0;
    let mut urandom: Vec<u8> = vec![];
    let mut decimal:u8 = 0;
    input.with_exported(|| {
	let mut vecstore: Vec<i64> = vec![];
    	input.set_direction(Direction::In)?;
        input.set_edge(Edge::RisingEdge)?;
        let mut poller = input.get_poller()?;
        loop {
	    	if let Some(value) = poller.poll(1000)? {
				let now = Utc::now();
				count += 1;
				ns = now.nanosecond() as i64;
				vecstore.push(ns);
				//println!("Current Count: {}", count);
				if vecstore.len() == 4 {
		    		w = vecstore[0];
		    		x = vecstore[1];
		    		y = vecstore[2];
		    		z = vecstore[3];
		    		//difference1 = x - w;
		    		//difference2 = z - y;
		    		//if difference1 < difference2 {
		    		//	urandom.push(0);
		    		//} else if difference2 < difference1 {
		    		//	urandom.push(1);
		    		//}
		    		urandom.push(swap_bits(w, x, y, z));
		    		vecstore.clear();
			} if urandom.len() == 8 {
		    	for x in urandom.iter() {
		    		print!("{}", x); 
		    		io::stdout().flush().unwrap();
		    		decimal = decimal * 2 + x;
		    		if decimal < 128 && decimal > 32 {
		    			//println!("Conversion: {}", decimal as char);
		    		}
				}
				decimal = 0;
				urandom.clear();
			}
	    } else {
	        let mut stdout = stdout();
			stdout.write_all(b".")?;
			stdout.flush()?;
	    }
	}
    })
}

fn swap_bits(time1: i64, time2: i64, time3: i64, time4: i64) -> u8 {
	let mut flipper = false;
	let difference1:i64;
	let difference2:i64;
	difference1 = time2 - time1;
	difference2 = time4 - time3;
	if difference1 != difference2 {
		flipper ^= true;
		flipper ^ (difference1 > difference2);
		return flipper as u8;
	} else {
		return 0;
	}
}

fn main() {
    let args: Vec<String> = env::args().collect();
    	if args.len() != 2 {
        println!("Usage: ./interrupt <pin>");
    } else {
        match args[1].parse::<u64>() {
            Ok(pin) => {
                match interrupt(pin) {
                    Ok(()) => println!("Interrupting Complete!"),
                    Err(err) => println!("Error: {}", err),
                }
            }
            Err(_) => println!("Usage: ./interrupt <pin>"),
        }
    }
}