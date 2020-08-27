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

use matchmaker::{Category, Student};
use std::cell::RefCell;
use web_sys::{DragEvent, HtmlInputElement};
use yew::prelude::*;

pub struct State {
    dragging_category: Option<Category>,
    editing: bool,
    pre_editing_student_name: String,
}

pub struct StudentRow {
    props: Props,
    link: ComponentLink<Self>,
    state: State,
    name_input_ref: NodeRef,
}

#[derive(Properties, Clone)]
pub struct Props {
    pub student: Student,
    pub categories: RefCell<Vec<Category>>,
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

#[derive(Debug)]
pub enum Msg {
    AddPreference(Student, ChangeData),
    AddExclude(Student, ChangeData),
    StartDrag(Category),
    DragOver(Category),
    EndDrag,
    RemovePreference(Category),
    RemoveExclude(Category),
    EditStudent,
    EditStudentEnd,
    RemoveStudent,
}

impl Component for StudentRow {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            props,
            link,
            name_input_ref: NodeRef::default(),
            state: State {
                dragging_category: None,
                editing: false,
                pre_editing_student_name: "".to_string(),
            },
        }
    }

    fn update(&mut self, message: Self::Message) -> ShouldRender {
        match message {
            Msg::AddPreference(student, change_data) => {
                let category_name = match change_data {
                    ChangeData::Select(select_data) => select_data.value(),
                    ChangeData::Value(value) => value,
                    ChangeData::Files(_) => unreachable!(),
                };

                if let Some(category) = self
                    .props
                    .categories
                    .borrow()
                    .iter()
                    .find(|category| category.name == category_name)
                {
                    self.props
                        .on_add_preference
                        .emit((student, category.clone()));
                }
                false
            }
            Msg::AddExclude(student, change_data) => {
                let category_name = match change_data {
                    ChangeData::Select(select_data) => select_data.value(),
                    ChangeData::Value(value) => value,
                    ChangeData::Files(_) => unreachable!(),
                };

                if let Some(category) = self
                    .props
                    .categories
                    .borrow()
                    .iter()
                    .find(|category| category.name == category_name)
                {
                    self.props.on_add_exclude.emit((student, category.clone()));
                }
                false
            }
            Msg::StartDrag(category) => {
                self.state.dragging_category = Some(category);
                true
            }
            Msg::DragOver(target_category) => {
                if let Some(dragging_category) = &self.state.dragging_category {
                    if dragging_category != &target_category {
                        self.props.on_move_preference.emit((
                            self.props.student.clone(),
                            dragging_category.clone(),
                            target_category,
                        ));
                    }
                }
                false
            }
            Msg::EndDrag => {
                self.state.dragging_category = None;
                true
            }
            Msg::RemovePreference(preference) => {
                self.props
                    .on_remove_preference
                    .emit((self.props.student.clone(), preference));
                false
            }
            Msg::RemoveExclude(exclude) => {
                self.props
                    .on_remove_exclude
                    .emit((self.props.student.clone(), exclude));
                false
            }
            Msg::EditStudent => {
                self.state.editing = true;
                self.state.pre_editing_student_name = self.props.student.name.clone();
                self.props.on_editing.emit(());
                true
            }
            Msg::EditStudentEnd => {
                self.state.editing = false;
                if let Some(input) = self.name_input_ref.cast::<HtmlInputElement>() {
                    self.props
                        .on_edit_student
                        .emit((self.state.pre_editing_student_name.clone(), input.value()));
                }
                true
            }
            Msg::RemoveStudent => {
                self.props
                    .on_remove_student
                    .emit(self.props.student.clone());
                false
            }
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;
        true
    }

    fn view(&self) -> Html {
        let preferences: Vec<Html> = self
            .props
            .student
            .preferences
            .iter()
            .enumerate()
            .map(|(i, preference): (usize, &Category)| {
                let p = preference.clone();
                let handle_on_drag_start = self.link.callback(move |_: DragEvent| {
                    Msg::StartDrag(p.clone())
                });

                let handle_on_drag_end = self.link.callback(move |e: DragEvent| {
                    e.prevent_default();
                    Msg::EndDrag
                });

                let p = preference.clone();
                let handle_on_drag_over = self.link.callback(move |e: DragEvent| {
                    e.prevent_default();

                    Msg::DragOver(p.clone())
                });

                let p = preference.clone();
                let handle_remove_preference = self.link.callback(move |_: MouseEvent| {
                    Msg::RemovePreference(p.clone())
                });

                let mut style = "padding-right: 1.5rem;";
                if let Some(category) = &self.state.dragging_category {
                    if category == preference {
                        style = "padding-right: 1.5rem; opacity: 0.4;";

                    }
                }

                html! {
                    <li class="list-group-item" style=style id=format!("list-item-{}-{}", i,  &self.props.student.name) data-preference_name=&preference.name draggable=!self.props.editing ondragstart=handle_on_drag_start.clone() ondragend=handle_on_drag_end.clone() ondragover=handle_on_drag_over.clone()>
                        <span class="badge badge-info">{ i+1 }</span>{ " " }
                        { &preference.name }
                        {
                            if !self.props.editing {
                                html! {
                                    <button class="btn close" style="position:absolute; right: 0; top:0; margin: 2px;" aria-label="Close" onclick=handle_remove_preference.clone()><span aria-hidden="true">{ "×" }</span></button>
                                }
                            } else {
                                html! {}
                            }
                        }
                    </li>
                }
            })
            .collect();

        let excludes: Vec<Html> = self
            .props
            .student
            .exclude
            .iter()
            .map(|exclude: &Category| {
                let c = exclude.clone();
                let handle_remove_exclude = self.link.callback(move |_: MouseEvent| {
                    Msg::RemoveExclude(c.clone())
                });

                html! {
                    <li class="list-group-item" style="padding-right: 1.5rem;">
                        { &exclude.name }
                        {
                            if !self.props.editing {
                                html! {<button class="btn close" style="position:absolute; right: 0; top:0; margin: 2px;" aria-label="Close" onclick=handle_remove_exclude.clone()><span aria-hidden="true">{ "×" }</span></button>}
                            }else{
                                html! {}
                            }
                        }
                    </li>
                }
            })
            .collect();

        let category_options: Vec<Html> = self
            .props
            .categories
            .borrow()
            .iter()
            .filter(|category| !self.props.student.preferences.contains(category))
            .filter(|category| !self.props.student.exclude.contains(category))
            .map(|category: &Category| {
                html! {
                    <option value=&category.name>{ &category.name }</option>
                }
            })
            .collect();

        let student = self.props.student.clone();
        let handle_on_add_preference = self
            .link
            .callback(move |e: ChangeData| Msg::AddPreference(student.clone(), e));

        let student = self.props.student.clone();
        let handle_on_add_exclude = self
            .link
            .callback(move |e: ChangeData| Msg::AddExclude(student.clone(), e));

        let handle_on_edit_student = self.link.callback(|e: MouseEvent| {
            e.prevent_default();
            Msg::EditStudent
        });

        let handle_on_edit_student_end = self.link.callback(|e: MouseEvent| {
            e.prevent_default();
            Msg::EditStudentEnd
        });

        let handle_on_edit_student_submit = self.link.callback(|e: FocusEvent| {
            e.prevent_default();
            Msg::EditStudentEnd
        });

        let handle_on_remove_student = self.link.callback(|e: MouseEvent| {
            e.prevent_default();
            Msg::RemoveStudent
        });

        html! {
            <tr>
                <td>
                    {
                        if self.state.editing {
                            html! {
                            <form class="inline" onsubmit=handle_on_edit_student_submit>
                                <input type="text" name="student_name" value=&self.props.student.name ref=self.name_input_ref.clone() />
                            </form> }
                        } else {
                            html! { &self.props.student.name }
                        }
                    }
                </td>
                <td>
                    <ul class="list-group list-group-horizontal">
                        { preferences }
                        {
                            if !self.props.editing {
                                html! {
                                    <li class="list-group-item">
                                        <form class="inline">
                                            <select class="form-control form-control-sm" onchange=handle_on_add_preference>
                                                <option></option>
                                                { category_options.clone() }
                                            </select>
                                        </form>
                                    </li>
                                }
                            }else{
                                html! {}
                            }
                        }
                    </ul>
                </td>
                <td>
                    <ul class="list-group list-group-horizontal">
                        { excludes }
                        {
                            if !self.props.editing {
                                html! {
                                    <li class="list-group-item">
                                        <form class="inline">
                                            <select class="form-control form-control-sm" onchange=handle_on_add_exclude>
                                                <option></option>
                                                { category_options.clone() }
                                            </select>
                                        </form>
                                    </li>
                                }
                            }else{
                                html! {}
                            }
                        }
                    </ul>
                </td>
                <td class="control">
                    <form class="inline">
                        {
                            if !self.state.editing && !self.props.editing {
                                html! {
                                    <button class="btn btn-info btn-sm" onclick=handle_on_edit_student>{ "Bewerk" }</button>
                                }
                            } else if self.state.editing {
                                html! {
                                    <button class="btn btn-info btn-sm" onclick=handle_on_edit_student_end>{ "Opslaan" }</button>
                                }
                            } else {
                                html! {
                                    <button class="btn btn-info btn-sm" disabled=true>{ "Bewerk" }</button>
                                }
                            }
                        }
                        {
                            if !self.props.editing {
                                html! {
                                    <button class="btn btn-danger btn-sm" onclick=handle_on_remove_student>{ "Verwijder" }</button>
                                }
                            }else{
                                html! {
                                    <button class="btn btn-danger btn-sm" disabled=true>{ "Verwijder" }</button>
                                }
                            }
                        }
                    </form>
                </td>
            </tr>
        }
    }

    fn rendered(&mut self, _first_render: bool) {
        if let Some(input) = self.name_input_ref.cast::<HtmlInputElement>() {
            input.focus().unwrap_or(());
        }
    }
}
