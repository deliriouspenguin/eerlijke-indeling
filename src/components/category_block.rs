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

use super::category_row::CategoryRow;
use matchmaker::Category;
use std::cell::RefCell;
use wasm_bindgen::JsCast;
use web_sys::{FormData, HtmlFormElement, HtmlInputElement};
use yew::prelude::*;

pub enum FieldError {
    Duplicate,
    Empty,
}

pub struct State {
    error: Option<FieldError>,
    name: String,
    max_placements: String,
}

pub struct CategoryBlock {
    props: Props,
    link: ComponentLink<Self>,
    name_input_ref: NodeRef,
    state: State,
}

pub enum Msg {
    AddCategory(FormData),
}

#[derive(Properties, Clone)]
pub struct Props {
    pub categories: RefCell<Vec<Category>>,
    pub on_add_category: Callback<Category>,
    pub on_remove_category: Callback<Category>,
    pub editing: bool,
    pub on_edit_category: Callback<(String, String, Option<usize>)>,
    pub on_editing: Callback<()>,
}

impl Component for CategoryBlock {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            props,
            link,
            name_input_ref: NodeRef::default(),
            state: State {
                error: None,
                name: "".into(),
                max_placements: "".into(),
            },
        }
    }

    fn update(&mut self, message: Self::Message) -> ShouldRender {
        match message {
            Msg::AddCategory(form_data) => {
                let name = form_data.get("category_name").as_string().map(|n| n);
                let max_placements_string = form_data.get("category_max_placements").as_string();
                let max_placements = max_placements_string
                    .clone()
                    .and_then(|mp| mp.parse::<usize>().ok());

                self.state.name = name.clone().unwrap_or("".to_string());
                self.state.max_placements = max_placements_string.unwrap_or("".to_string());

                if let Some(name) = name {
                    if let Some(max_placements) = max_placements {
                        if name != "" && max_placements != 0 {
                            let category = Category::new(name.trim(), max_placements);
                            if self.props.categories.borrow().contains(&category) {
                                self.state.error = Some(FieldError::Duplicate);
                            } else {
                                self.props.on_add_category.emit(category);
                                self.focus_on_input();
                                self.state.error = None;
                                self.state.name = "".into();
                                self.state.max_placements = "".into();
                                return false;
                            }
                        } else {
                            self.state.error = Some(FieldError::Empty);
                        }
                    } else {
                        self.state.error = Some(FieldError::Empty);
                    }
                }
                true
            }
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;
        true
    }

    fn view(&self) -> Html {
        let categories: Vec<Html> = self
            .props
            .categories
            .borrow()
            .iter()
            .map(|category: &Category| {
                html! {
                    <CategoryRow category=category.clone() editing=self.props.editing.clone() on_edit_category=self.props.on_edit_category.clone() on_remove_category=self.props.on_remove_category.clone() on_editing=self.props.on_editing.clone() />
                }
            })
            .collect();

        let submit_form = self.link.callback(|e: FocusEvent| {
            e.prevent_default();
            Msg::AddCategory(
                FormData::new_with_form(
                    &e.target()
                        .or_else(|| {
                            log::error!("No target");
                            None
                        })
                        .unwrap()
                        .dyn_ref::<HtmlFormElement>()
                        .or_else(|| {
                            log::error!("No form element");
                            None
                        })
                        .unwrap()
                        .clone(),
                )
                .or_else(|e| {
                    log::info!("FormData can not be created: {:?}", e);
                    Err(e)
                })
                .unwrap(),
            )
        });

        html! {
            <div class="col shadow p-3 mb-5 bg-white rounded">
                <h2>{"Stap 1: Activiteiten toevoegen"}</h2>
                <p>{ "Voeg bij 'Naam activiteit' de verschillende activiteiten toe waar de leerlingen aan kunnen deelnemen. Vul per activiteit het maximum aantal leerlingen in." }</p>
                <table class="table table-responsive-sm">
                    <tr>
                        <th>{ "Naam activiteit" }</th>
                        <th>{ "Aantal plekken" }</th>
                        <th class="control"></th>
                    </tr>
                    <tbody>
                        { categories }
                    </tbody>
                </table>

                <br/>
                <form class="form-inline" novalidate=true onsubmit=submit_form>
                    <div class="form-group mb-2">
                        <input type="text" class="form-control" name="category_name" value=&self.state.name id="category_name" disabled=self.props.editing placeholder="Naam activiteit" ref=self.name_input_ref.clone() />
                    </div>
                    <div class="form-group mb-2 mx-sm-3">
                        <input type="number" class="form-control" name="category_max_placements" value=&self.state.max_placements disabled=self.props.editing id="category_max_placements" placeholder="Aantal plekken" />
                    </div>
                    <button type="submit" class="btn btn-primary mb-2" disabled=self.props.editing>{ "Toevoegen" }</button>
                    { if self.state.error.is_some() {
                        html! {
                            <div class="invalid-feedback d-block">
                                {
                                    match self.state.error {
                                        Some(FieldError::Duplicate) => "Er bestaat al een activiteit met deze naam!",
                                        Some(FieldError::Empty) => "Een naam en aantal beschikbare plekken is noodzakelijk!",
                                        _ => "Onbekende fout",
                                    }
                                }
                            </div>
                        }
                    } else {
                        html! {}
                    } }
                </form>
            </div>
        }
    }

    fn rendered(&mut self, first_render: bool) {
        if first_render {
            self.focus_on_input();
        }
    }
}

impl CategoryBlock {
    fn focus_on_input(&self) {
        if let Some(input) = self.name_input_ref.cast::<HtmlInputElement>() {
            input.focus().unwrap_or(());
        }
    }
}
