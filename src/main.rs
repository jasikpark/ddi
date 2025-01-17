//A safer dd
//Copyright (C) 2021  Tomás Ralph
//
//This program is free software: you can redistribute it and/or modify
//it under the terms of the GNU General Public License as published by
//the Free Software Foundation, either version 3 of the License, or
//(at your option) any later version.
//
//This program is distributed in the hope that it will be useful,
//but WITHOUT ANY WARRANTY; without even the implied warranty of
//MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
//GNU General Public License for more details.
//
//You should have received a copy of the GNU General Public License
//along with this program.  If not, see <https://www.gnu.org/licenses/>.
//
////////////////////////////////////
//                                //
//       Created by tralph3       //
//   https://github.com/tralph3   //
//                                //
////////////////////////////////////

use std::env;

fn main() {

    let args: Vec<String> = env::args().collect();


    if let Err(e) = ddi::run(args) {
        println!("Error executing program: {}", e);
        std::process::exit(1);
    }
}
