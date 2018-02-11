extern crate sysfs_gpio;
extern crate chrono;

use sysfs_gpio::{Direction, Edge, Pin};
use std::env;
use std::io::prelude::*;
use std::io::stdout;
use chrono::prelude::*;

fn interrupt(pin: u64) -> sysfs_gpio::Result<()> {
    let input = Pin::new(pin);
    let mut ns:i64 = 0;
    let mut x:i64 = 0;
    let mut y:i64 = 0;
    let mut count:i64 = 0;
    let mut urandom: Vec<i64> = vec![];
    input.with_exported(|| {
	let mut vecstore: Vec<i64> = vec![];
    	input.set_direction(Direction::In)?;
        input.set_edge(Edge::RisingEdge)?;
        let mut poller = input.get_poller()?;
        loop {
	    	if let Some(value) = poller.poll(1000)? {
				println!("GPIO Val: {}", value);
				let now = Utc::now();
				count += 1;
				ns = now.nanosecond() as i64;
				println!("Nanosecond: {}", ns);
				vecstore.push(ns);
				println!("Current Count: {}", count);
				if vecstore.len() == 2 {
		    		y = vecstore[0];
		    		x = vecstore[1];
		    		if vecstore[0] > vecstore[1] {
						urandom.push(0);
		    		} else if vecstore[1] < vecstore[0] {
						urandom.push(1);
		     			//println!("Time Difference: {}", difference);
		    		}
		    	vecstore.clear();
			} if urandom.len() == 8 {
		    	for x in urandom.iter() {
				println!("Random: {}", x);
				}
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
