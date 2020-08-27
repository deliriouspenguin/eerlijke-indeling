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

use super::student_row::StudentRow;
use matchmaker::{Category, Student};
use std::cell::RefCell;
use std::collections::VecDeque;
use wasm_bindgen::JsCast;
use web_sys::{FormData, HtmlFormElement, HtmlInputElement};
use yew::prelude::*;

pub enum FieldError {
    Duplicate,
    Empty,
}

pub struct State {
    name: String,
    error: Option<FieldError>,
}

pub struct StudentBlock {
    props: Props,
    link: ComponentLink<Self>,
    name_input_ref: NodeRef,
    state: State,
}

pub enum Msg {
    AddStudent(FormData),
}

#[derive(Properties, Clone)]
pub struct Props {
    pub students: RefCell<Vec<Student>>,
    pub categories: RefCell<Vec<Category>>,
    pub on_add_student: Callback<Student>,
    pub on_add_preference: Callback<(Student, Category)>,
    pub on_add_exclude: Callback<(Student, Category)>,
    pub on_move_preference: Callback<(Student, Category, Category)>,
    pub on_remove_preference: Callback<(Student, Category)>,
    pub on_remove_exclude: Callback<(Student, Category)>,
    pub on_edit_student: Callback<(String, String)>,
    pub on_remove_student: Callback<Student>,
    pub editing: bool,
    pub on_editing: Callback<()>,
}

impl Component for StudentBlock {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            props,
            link,
            name_input_ref: NodeRef::default(),
            state: State {
                name: "".into(),
                error: None,
            },
        }
    }

    fn update(&mut self, message: Self::Message) -> ShouldRender {
        match message {
            Msg::AddStudent(form_data) => {
                let name = form_data.get("student_name").as_string().map(|n| n);
                if let Some(name) = name {
                    self.state.name = name.clone();
                    if name != "" {
                        let student = Student::new(name.trim(), VecDeque::new(), Vec::new());
                        if self.props.students.borrow().contains(&student) {
                            self.state.error = Some(FieldError::Duplicate);
                        } else {
                            self.props.on_add_student.emit(student);
                            self.focus_on_input();
                            self.state.name = "".into();
                            self.state.error = None;
                            return false;
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
        let students: Vec<Html> = self
            .props
            .students
            .borrow()
            .iter()
            .map(|student: &Student| {
                html! {
                    <StudentRow student=student categories=self.props.categories.clone() editing=self.props.editing on_editing=self.props.on_editing.clone() on_add_preference=self.props.on_add_preference.clone() on_add_exclude=self.props.on_add_exclude.clone() on_move_preference=self.props.on_move_preference.clone() on_remove_preference=self.props.on_remove_preference.clone() on_remove_exclude=self.props.on_remove_exclude.clone() on_edit_student=self.props.on_edit_student.clone() on_remove_student=self.props.on_remove_student.clone() />
                }
            })
            .collect();

        let submit_form = self.link.callback(|e: FocusEvent| {
            e.prevent_default();
            Msg::AddStudent(
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
                <h2>{"Stap 2: Leerlingen toevoegen"}</h2>
                <p>{ "Vermeld bij 'Naam leerling' de namen van de leerlingen die aan de activiteit willen deelnemen. Nadat je een leerling hebt toegevoegd, kun je de de eerste, tweede en opvolgende voorkeuren voor activiteiten selecteren uit de lijst naast diens naam." }</p>
                <p>{ "Het systeem probeert leerlingen eerst bij de activiteiten van hun voorkeur in te delen. Wanneer dat niet mogelijk blijkt, wordt de leerling ingedeeld bij een willekeurige activiteit waar wel plek is." }</p>
                <p>{ "Geef bij 'Uitsluitingen' de activiteiten aan waar de leerling niet aan wil deelnemen. De leerling zal in dat geval niet bij deze activiteiten worden ingedeeld." }</p>
                <table class="table table-responsive-sm">
                    <tr>
                        <th>{ "Naam leerling" }</th>
                        <th>{ "Voorkeuren" }</th>
                        <th>{ "Uitsluitingen" }</th>
                        <th class="control"></th>
                    </tr>
                    <tbody>
                        { students }
                    </tbody>
                </table>

                <br/>
                <form class="form-inline" onsubmit=submit_form>
                    <div class="form-group mb-2">
                        <input type="text" disabled=self.props.editing class="form-control" name="student_name" id="student_name" placeholder="Naam leerling" value=&self.state.name ref=self.name_input_ref.clone() />
                    </div>
                    <button type="submit" class="btn btn-primary mb-2" disabled=self.props.editing>{ "Toevoegen" }</button>
                    { if self.state.error.is_some() {
                        html! {
                            <div class="invalid-feedback d-block">
                                {
                                    match self.state.error {
                                        Some(FieldError::Duplicate) => "Er bestaat al een leerling met deze naam!",
                                        Some(FieldError::Empty) => "Een naam is noodzakelijk!",
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

impl StudentBlock {
    fn focus_on_input(&self) {
        if let Some(input) = self.name_input_ref.cast::<HtmlInputElement>() {
            input.focus().unwrap_or(());
        }
    }
}
