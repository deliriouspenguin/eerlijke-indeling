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

use matchmaker::{Category, MatchResult};
use std::cell::RefCell;
use yew::prelude::*;

pub struct MatchResultBlock {
    props: Props,
}

#[derive(Properties, Clone)]
pub struct Props {
    pub match_result: RefCell<MatchResult>,
    pub categories: RefCell<Vec<Category>>,
}

impl Component for MatchResultBlock {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self { props }
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;
        true
    }

    fn view(&self) -> Html {
        html! {
            <>
                <div class="row">
                    <div class="col shadow p-3 mb-5 bg-white rounded">
                    {
                        self.props.categories.borrow().chunks(4).map(|categories| {
                            html! {
                                <div class="row">
                                {
                                    categories.iter().map(|category| {
                                        html! {
                                            <div class="col col-md-3 pt-3">
                                                <h4>{ &category.name }</h4>
                                                {
                                                    self.props.match_result.borrow().placed.get(&category.name).map(|students| {
                                                        html! {
                                                            <ul class="list-group">
                                                            {
                                                                students.iter().map(|student| {
                                                                    html! { <li class="list-group-item">{ &student.name }</li> }
                                                                }).collect::<Html>()
                                                            }
                                                            </ul>
                                                        }
                                                    }).unwrap_or({
                                                        html! {<p class="font-italic">{ "Geen leerlingen in deze activiteit" }</p>}
                                                    })
                                                }
                                            </div>
                                        }
                                    }).collect::<Vec<Html>>()
                                }
                                </div>
                            }
                        }).collect::<Html>()
                    }
                    </div>
                </div>

                <div class="row">
                    {
                        if !self.props.match_result.borrow().not_placable.is_empty() {
                            html! {
                                <div class="col shadow p-3 mb-5 bg-white rounded">
                                    <h3>{ "Niet ingedeelde leerlingen" }</h3>
                                    <ul class="list-group">
                                    {
                                        self.props.match_result.borrow().not_placable.iter().map(|student| {
                                            html! { <li class="list-group-item">{ &student.name }</li> }
                                        }).collect::<Html>()
                                    }
                                    </ul>
                                </div>
                            }
                        } else {
                            html! {}
                        }
                    }
                </div>
            </>
        }
    }
}
