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

use web_sys::HtmlElement;
use yew::prelude::*;

pub struct Modal {
    link: ComponentLink<Self>,
    props: Props,
    close_btn_ref: NodeRef,
}

#[derive(Properties, Clone)]
pub struct Props {
    pub id: String,
    pub title: String,
    pub children: Children,
    pub btn_type: String,
    pub btn_label: String,
    pub handle_modal_action: Option<Callback<()>>,
}

pub enum Msg {
    HandleModalAction,
}

impl Component for Modal {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            props,
            close_btn_ref: NodeRef::default(),
        }
    }

    fn update(&mut self, message: Self::Message) -> ShouldRender {
        match message {
            Msg::HandleModalAction => {
                if let Some(close_btn) = self.close_btn_ref.cast::<HtmlElement>() {
                    close_btn.click();
                }
                if let Some(action) = &self.props.handle_modal_action {
                    action.emit(());
                }
                false
            }
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;
        true
    }

    fn view(&self) -> Html {
        let handle_modal_action = self
            .link
            .callback(move |_: MouseEvent| Msg::HandleModalAction);

        html! {
            <div class="modal fade" id=&self.props.id tabindex="-1" role="dialog" aria-labelledby=format!("{}_label", &self.props.id) aria-hidden="true">
                <div class="modal-dialog" role="document">
                    <div class="modal-content">
                    <div class="modal-header">
                        <h5 class="modal-title" id=format!("{}_label", &self.props.id)>{ &self.props.title }</h5>
                        <button type="button" class="close" data-dismiss="modal" aria-label="Sluiten">
                        <span aria-hidden="true">{ "×" }</span>
                        </button>
                    </div>
                    <div class="modal-body">
                        {
                            for self.props.children.iter().map(|item| {
                                item
                            })
                        }
                    </div>
                    <div class="modal-footer">
                        <button type="button" class="btn btn-secondary" data-dismiss="modal" ref=self.close_btn_ref.clone()>{ "Sluiten" }</button>
                        { if self.props.handle_modal_action.is_some() {
                            html! {
                                <button type="button" class=format!("btn btn-{}", &self.props.btn_type) onclick=handle_modal_action>{ &self.props.btn_label }</button>
                            }
                        } else {
                            html! {}
                        } }
                    </div>
                    </div>
                </div>
            </div>
        }
    }
}
