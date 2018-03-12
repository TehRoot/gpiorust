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
    let mut x:i64 = 0;
    let mut y:i64 = 0;
    let mut z:i64 = 0;
    let mut difference1:i64 = 0;
    let mut difference2:i64 = 0;
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
				if vecstore.len() == 3 {
		    		x = vecstore[0];
		    		y = vecstore[1];
		    		z = vecstore[2];
		    		difference1 = y - x;
		    		difference2 = z - y;
		    		if difference1 < difference2 {
		    			urandom.push(0);
		    		} else if difference2 < difference1 {
		    			urandom.push(1);
		    		}
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