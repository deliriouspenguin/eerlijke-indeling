#![recursion_limit = "2048"]
// Copyright (C) 2020 Delirious Penguin
//
// This file is part of Eerlijke Indeling.
//
// Eerlijke Indeling is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// Eerlijke Indeling is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with Eerlijke Indeling.  If not, see <http://www.gnu.org/licenses/>.

mod components;

use components::Main;
use wasm_bindgen::prelude::*;
use yew::prelude::*;
use yew::utils::document;

#[wasm_bindgen(start)]
pub fn run_app() {
    wasm_logger::init(wasm_logger::Config::default());
    let content_element = document()
        .query_selector("#content")
        .expect("can't get body node for rendering")
        .expect("can't unwrap body node");
    App::<Main>::new().mount(content_element);
}
