/****************************************************************************
 * main.rs
 *
 *   Copyright (C) 2020 Augusto Fraga Giachero. All rights reserved.
 *   Author: Augusto Fraga Giachero <afg@augustofg.net>
 *
 * This file is part of the ddc-control program.
 *
 * ddc-control is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * ddc-control is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with ddc-control.  If not, see <https://www.gnu.org/licenses/>.
 *
 ****************************************************************************/

extern crate ddc_hi;
extern crate clap;
extern crate parse_int;

use ddc_hi::{Ddc, Display};
use clap::{Arg, App, AppSettings, SubCommand};
use parse_int::parse;

fn main() {
    let matches = App::new("ddc-control").version("0.1.0")
        .author("Augusto F. Giachero <afg@augustofg.net>")
        .about("Set / Get VCP parameters from connected monitors via DDC")
        .arg(Arg::with_name("monitor-number")
             .required(true)
             .short("m")
             .long("monitor-number")
             .value_name("NUM")
             .help("Monitor number")
             .takes_value(true))
        .setting(AppSettings::SubcommandRequired)
        .subcommand(SubCommand::with_name("get-vcp")
                    .about("Get the VCP feature value")
                    .arg(Arg::with_name("FEATURE")
                         .help("VCP feature number")
                         .index(1)
                         .required(true)))
        .subcommand(SubCommand::with_name("set-vcp")
                    .about("Set the VCP feature value")
                    .arg(Arg::with_name("FEATURE")
                         .help("VCP feature number")
                         .index(1)
                         .required(true))
                    .arg(Arg::with_name("VALUE")
                         .help("VCP feature value to set")
                         .index(2)
                         .required(true)))
        .get_matches();

    let monitor = match parse::<usize>(matches.value_of("monitor-number").unwrap()) {
        Ok(num) => num,
        Err(e) => {
            eprintln!("Invalid argument for -m / --monitor: {}", e);
            std::process::exit(1)
        },
    };


    if monitor < 1 {
        eprintln!("Monitor number should be greater than zero.");
        std::process::exit(1);
    }

    let (feature, value, write) =
        if let Some(feature_arg) = matches.subcommand_matches("get-vcp") {
            (match parse::<u8>(feature_arg.value_of("FEATURE").unwrap()) {
                Ok(num) => num,
                Err(e) => {
                    eprintln!("Invalid argument for get-vcp: {}", e);
                    std::process::exit(1)
                },
            }, 0u16, false)
        } else if let Some(feature_arg) = matches.subcommand_matches("set-vcp") {
            (match parse::<u8>(feature_arg.value_of("FEATURE").unwrap()) {
                Ok(num) => num,
                Err(e) => {
                    eprintln!("Invalid argument for set-vcp: {}", e);
                    std::process::exit(1)
                },
            },
             match parse::<u16>(feature_arg.value_of("VALUE").unwrap()) {
                 Ok(num) => num,
                 Err(e) => {
                     eprintln!("Invalid argument for set-vcp: {}", e);
                     std::process::exit(1)
                 },
             }, true)
        } else {panic!("Invalid arguments! Check the argument parsing logic.")};

    let mut displays = Display::enumerate();

    let display =
        if monitor <= displays.len() {
            &mut displays[monitor - 1]
        } else {
            eprintln!("Monitor number {} not available.", monitor);
            std::process::exit(1)
        };

    if write {
        display.handle.set_vcp_feature(feature, value).unwrap();
    } else {
        let value = display.handle.get_vcp_feature(feature).unwrap();
        println!("0x{:02X}", value.value());
    }
}
