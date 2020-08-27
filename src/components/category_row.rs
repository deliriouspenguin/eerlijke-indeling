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

use matchmaker::Category;
use web_sys::HtmlInputElement;
use yew::prelude::*;

pub struct State {
    editing: bool,
    pre_editing_category_name: String,
}

pub struct CategoryRow {
    props: Props,
    link: ComponentLink<Self>,
    state: State,
    name_input_ref: NodeRef,
    max_placements_input_ref: NodeRef,
}

#[derive(Properties, Clone)]
pub struct Props {
    pub on_edit_category: Callback<(String, String, Option<usize>)>,
    pub category: Category,
    pub on_remove_category: Callback<Category>,
    pub editing: bool,
    pub on_editing: Callback<()>,
}

#[derive(Debug)]
pub enum Msg {
    EditCategory,
    EditCategoryEnd,
    RemoveCategory,
}

impl Component for CategoryRow {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            props,
            link,
            name_input_ref: NodeRef::default(),
            max_placements_input_ref: NodeRef::default(),
            state: State {
                editing: false,
                pre_editing_category_name: "".to_string(),
            },
        }
    }

    fn update(&mut self, message: Self::Message) -> ShouldRender {
        match message {
            Msg::EditCategory => {
                self.state.editing = true;
                self.state.pre_editing_category_name = self.props.category.name.clone();
                self.props.on_editing.emit(());
                true
            }
            Msg::EditCategoryEnd => {
                self.state.editing = false;
                if let Some(name_input) = self.name_input_ref.cast::<HtmlInputElement>() {
                    if let Some(max_placements_input) =
                        self.max_placements_input_ref.cast::<HtmlInputElement>()
                    {
                        let max_placements = max_placements_input.value().parse::<usize>().ok();
                        self.props.on_edit_category.emit((
                            self.state.pre_editing_category_name.clone(),
                            name_input.value(),
                            max_placements,
                        ));
                    }
                }
                true
            }
            Msg::RemoveCategory => {
                self.props
                    .on_remove_category
                    .emit(self.props.category.clone());
                false
            }
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;
        true
    }

    fn view(&self) -> Html {
        let handle_on_edit_category = self.link.callback(|e: MouseEvent| {
            e.prevent_default();
            Msg::EditCategory
        });

        let handle_on_edit_category_end = self.link.callback(|e: MouseEvent| {
            e.prevent_default();
            Msg::EditCategoryEnd
        });

        let handle_on_edit_category_submit = self.link.callback(|e: FocusEvent| {
            e.prevent_default();
            Msg::EditCategoryEnd
        });

        let handle_on_remove_category = self.link.callback(|e: MouseEvent| {
            e.prevent_default();
            Msg::RemoveCategory
        });

        html! {
            <tr>
                <td>
                    {
                        if self.state.editing {
                            html! {
                            <form class="inline" onsubmit=handle_on_edit_category_submit.clone()>
                                <input type="text" name="category_name" value=&self.props.category.name ref=self.name_input_ref.clone() />
                            </form> }
                        } else {
                            html! { &self.props.category.name }
                        }
                    }
                </td>
                <td>
                    {
                        if self.state.editing {
                            html! {
                            <form class="inline" onsubmit=handle_on_edit_category_submit>
                                <input type="number" name="category_max_placements" value=&self.props.category.max_placements ref=self.max_placements_input_ref.clone() />
                            </form> }
                        } else {
                            html! { &self.props.category.max_placements }
                        }
                    }
                </td>
                <td class="control">
                    <form class="inline">
                        {
                            if !self.state.editing && !self.props.editing {
                                html! {
                                    <button class="btn btn-info btn-sm" onclick=handle_on_edit_category>{ "Bewerk" }</button>
                                }
                            } else if self.state.editing {
                                html! {
                                    <button class="btn btn-info btn-sm" onclick=handle_on_edit_category_end>{ "Opslaan" }</button>
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
                                    <button class="btn btn-danger btn-sm" onclick=handle_on_remove_category>{ "Verwijder" }</button>
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
