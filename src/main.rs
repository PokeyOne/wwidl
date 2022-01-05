/*
    What Was I Doing Last?, this is a command line utility that makes writing
    notes on what you were doing last easier.

    Copyright (C) 2022  Mateo Carreras

    This program is free software: you can redistribute it and/or modify
    it under the terms of the GNU General Public License as published by
    the Free Software Foundation, either version 3 of the License, or
    (at your option) any later version.

    This program is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU General Public License for more details.

    You should have received a copy of the GNU General Public License
    along with this program.  If not, see <https://www.gnu.org/licenses/>.

    File created by Mateo Carreras January 4, 2022
 */

mod config;

use config::Config;

fn main() {
    let config = Config::load();

    println!("Config loaded: {:?}", config);
}