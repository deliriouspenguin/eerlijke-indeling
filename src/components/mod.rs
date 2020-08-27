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

mod category_block;
mod category_row;
mod external;
mod match_result_block;
mod modal;
mod student_block;
mod student_row;

use category_block::CategoryBlock;
use match_result_block::MatchResultBlock;
use matchmaker::{
    da_stb::{match_students, match_students_to_multiple_categories},
    Category, MatchResult, Student,
};
use modal::Modal;
use rand::rngs::OsRng;
use serde::{Deserialize, Serialize};
use std::cell::RefCell;
use student_block::StudentBlock;
use web_sys::HtmlInputElement;
use yew::format::Json;
use yew::prelude::*;
use yew::services::storage::{Area, StorageService};

const KEY: &str = "fair_assignment.components.main";

#[derive(Deserialize, Serialize, Debug)]
pub struct State {
    categories: RefCell<Vec<Category>>,
    students: RefCell<Vec<Student>>,
    multi_matches: bool,
    match_result: Option<RefCell<MatchResult>>,
}

impl std::default::Default for State {
    fn default() -> Self {
        State {
            categories: RefCell::new(vec![]),
            students: RefCell::new(vec![]),
            multi_matches: false,
            match_result: None,
        }
    }
}

#[derive(Deserialize, Serialize)]
pub struct EphemeralState {
    editing: bool,
    show_delete_modal: bool,
}

pub struct Main {
    state: State,
    ephemeral_state: EphemeralState,
    link: ComponentLink<Self>,
    storage: StorageService,
    multi_matches_ref: NodeRef,
}

pub enum Msg {
    AddCategory(Category),
    AddStudent(Student),
    AddPreference(Student, Category),
    AddExclude(Student, Category),
    MovePreference(Student, Category, Category),
    RemovePreference((Student, Category)),
    RemoveExclude((Student, Category)),
    Editing(()),
    EditStudent((String, String)),
    RemoveStudent(Student),
    EditCategory((String, String, Option<usize>)),
    RemoveCategory(Category),
    ToggleMultiMatches,
    DeleteAllData,
    MakeMatches,
    ChangeData,
    PrintPage,
}

impl Component for Main {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let storage = StorageService::new(Area::Local).expect("storage was disabled by the user");
        let state = {
            if let Json(Ok(restored_model)) = storage.restore(KEY) {
                restored_model
            } else {
                State::default()
            }
        };

        Self {
            state,
            ephemeral_state: EphemeralState {
                editing: false,
                show_delete_modal: false,
            },
            link,
            storage,
            multi_matches_ref: NodeRef::default(),
        }
    }

    fn update(&mut self, message: Self::Message) -> ShouldRender {
        let response = match message {
            Msg::AddCategory(category) => {
                log::info!("Adding category: {:?}", category);
                self.state.categories.borrow_mut().push(category);
                true
            }
            Msg::AddStudent(student) => {
                log::info!("Adding student: {:?}", student);
                self.state.students.borrow_mut().push(student);
                true
            }
            Msg::AddPreference(student, preference) => {
                log::info!("Adding preference: {:?} {:?}", student, preference);
                self.state
                    .students
                    .borrow_mut()
                    .iter_mut()
                    .find(|s| **s == student)
                    .map(|s| s.preferences.push_back(preference));
                true
            }
            Msg::AddExclude(student, exclude) => {
                log::info!("Adding exclude: {:?} {:?}", student, exclude);
                self.state
                    .students
                    .borrow_mut()
                    .iter_mut()
                    .find(|s| **s == student)
                    .map(|s| s.exclude.push(exclude));
                true
            }
            Msg::MovePreference(student, drag_category, target_category) => {
                log::info!(
                    "Move preference {:?} to {:?} for {:?}",
                    &drag_category,
                    &target_category,
                    &student.name
                );
                if let Some(drag_category_index) =
                    student.preferences.iter().position(|c| c == &drag_category)
                {
                    if let Some(target_category_index) = student
                        .preferences
                        .iter()
                        .position(|c| c == &target_category)
                    {
                        self.state
                            .students
                            .borrow_mut()
                            .iter_mut()
                            .find(|s| **s == student)
                            .map(|s| {
                                if let Some(drag_category) =
                                    s.preferences.remove(drag_category_index)
                                {
                                    s.preferences.insert(target_category_index, drag_category);
                                }
                            });
                    }
                }
                true
            }
            Msg::RemovePreference((student, preference)) => {
                log::info!(
                    "Remove preference {:?} for {:?}",
                    &preference,
                    &student.name
                );

                self.state
                    .students
                    .borrow_mut()
                    .iter_mut()
                    .find(|s| **s == student)
                    .map(|s| {
                        if let Some(preference_index) =
                            student.preferences.iter().position(|c| c == &preference)
                        {
                            s.preferences.remove(preference_index);
                        }
                    });
                true
            }
            Msg::RemoveExclude((student, exclude)) => {
                log::info!("Remove exclude {:?} for {:?}", &exclude, &student.name);

                self.state
                    .students
                    .borrow_mut()
                    .iter_mut()
                    .find(|s| **s == student)
                    .map(|s| {
                        if let Some(exclude_index) =
                            student.exclude.iter().position(|c| c == &exclude)
                        {
                            s.exclude.remove(exclude_index);
                        }
                    });
                true
            }
            Msg::EditStudent((pre_editing_student_name, new_student_name)) => {
                log::info!(
                    "Change student name from {:?} to {:?}",
                    &pre_editing_student_name,
                    &new_student_name
                );

                if pre_editing_student_name != new_student_name
                    && self
                        .state
                        .students
                        .borrow()
                        .iter()
                        .find(|s| s.name == new_student_name)
                        .is_some()
                {
                    self.ephemeral_state.editing = false;
                    return true;
                }

                self.state
                    .students
                    .borrow_mut()
                    .iter_mut()
                    .find(|s| s.name == pre_editing_student_name)
                    .map(|s| {
                        s.name = new_student_name;
                    });

                self.ephemeral_state.editing = false;
                true
            }
            Msg::RemoveStudent(student) => {
                log::info!("Remove student {:?}", &student);

                let position = self
                    .state
                    .students
                    .borrow_mut()
                    .iter_mut()
                    .position(|s| *s == student);

                if let Some(index) = position {
                    self.state.students.borrow_mut().remove(index);
                }
                true
            }
            Msg::EditCategory((
                pre_editing_category_name,
                new_category_name,
                new_max_placements,
            )) => {
                log::info!(
                    "Change category name from {:?} to {:?} with max_placements {:?}",
                    &pre_editing_category_name,
                    &new_category_name,
                    &new_max_placements
                );

                if pre_editing_category_name != new_category_name
                    && self
                        .state
                        .categories
                        .borrow()
                        .iter()
                        .find(|c| c.name == new_category_name)
                        .is_some()
                {
                    self.ephemeral_state.editing = false;
                    return true;
                }

                self.state
                    .categories
                    .borrow_mut()
                    .iter_mut()
                    .find(|s| s.name == pre_editing_category_name)
                    .map(|s| {
                        s.name = new_category_name.clone();
                        if let Some(mp) = new_max_placements.clone() {
                            s.max_placements = mp;
                        }
                    });

                for student in self.state.students.borrow_mut().iter_mut() {
                    let pref_position = student
                        .preferences
                        .iter()
                        .position(|c| *c.name == pre_editing_category_name);
                    if let Some(index) = pref_position {
                        student.preferences.get_mut(index).map(|c| {
                            c.name = new_category_name.clone();
                            new_max_placements.clone().map(|mp| c.max_placements = mp);
                        });
                    }

                    let exc_position = student
                        .exclude
                        .iter()
                        .position(|c| *c.name == pre_editing_category_name);
                    if let Some(index) = exc_position {
                        student.exclude.get_mut(index).map(|c| {
                            c.name = new_category_name.clone();
                            new_max_placements.clone().map(|mp| c.max_placements = mp);
                        });
                    }
                }

                self.ephemeral_state.editing = false;
                true
            }
            Msg::RemoveCategory(category) => {
                log::info!("Remove category {:?}", &category);

                let position = self
                    .state
                    .categories
                    .borrow_mut()
                    .iter_mut()
                    .position(|c| *c == category);

                if let Some(index) = position {
                    self.state.categories.borrow_mut().remove(index);
                }

                for student in self.state.students.borrow_mut().iter_mut() {
                    let pref_position = student.preferences.iter().position(|c| *c == category);
                    if let Some(index) = pref_position {
                        student.preferences.remove(index);
                    }

                    let exc_position = student.exclude.iter().position(|c| *c == category);
                    if let Some(index) = exc_position {
                        student.exclude.remove(index);
                    }
                }
                true
            }
            Msg::Editing(_) => {
                self.ephemeral_state.editing = true;
                true
            }
            Msg::ToggleMultiMatches => {
                if let Some(input) = self.multi_matches_ref.cast::<HtmlInputElement>() {
                    self.state.multi_matches = input.checked();
                    log::info!(
                        "multi_match checkbox set to: {:?}",
                        self.state.multi_matches
                    );
                } else {
                    log::error!("Couldn't cast multi_match_ref to HtmlInputElement!");
                }

                true
            }
            Msg::DeleteAllData => {
                self.state = State::default();
                true
            }
            Msg::MakeMatches => {
                log::info!("Making matches...");
                let mut rng = OsRng;
                log::info!("Multi matches: {:?}", self.state.multi_matches);
                let match_result = if self.state.multi_matches {
                    log::info!("Make multi matches");
                    match_students_to_multiple_categories(
                        self.state.students.borrow().clone(),
                        &*self.state.categories.borrow(),
                        &mut rng,
                    )
                } else {
                    log::info!("Make single matches");
                    match_students(
                        self.state.students.borrow().clone(),
                        &*self.state.categories.borrow(),
                        &mut rng,
                    )
                };
                self.state.match_result = Some(RefCell::new(match_result));
                log::info!("Matches made: {:?}", &self.state.match_result);
                true
            }
            Msg::ChangeData => {
                self.state.match_result = None;
                true
            }
            Msg::PrintPage => {
                external::print_page();
                false
            }
        };

        let json = Json(&self.state);
        self.storage.store(KEY, json);

        response
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let handle_on_add_category = self.link.callback(Msg::AddCategory);
        let handle_on_add_student = self
            .link
            .callback(|student: Student| Msg::AddStudent(student));
        let handle_on_add_preference = self
            .link
            .callback(|(student, preference)| Msg::AddPreference(student, preference));
        let handle_on_add_exclude = self
            .link
            .callback(|(student, exclude)| Msg::AddExclude(student, exclude));
        let handle_on_move_preference =
            self.link
                .callback(|(student, drag_category, target_category)| {
                    Msg::MovePreference(student, drag_category, target_category)
                });
        let handle_on_remove_preference = self.link.callback(Msg::RemovePreference);
        let handle_on_remove_exclude = self.link.callback(Msg::RemoveExclude);
        let handle_on_edit_student = self.link.callback(Msg::EditStudent);
        let handle_on_editing = self.link.callback(Msg::Editing);
        let handle_on_remove_student = self.link.callback(Msg::RemoveStudent);
        let handle_on_remove_category = self.link.callback(Msg::RemoveCategory);
        let handle_on_edit_category = self.link.callback(Msg::EditCategory);

        let handle_toggle_multitmatches =
            self.link.callback(|_: MouseEvent| Msg::ToggleMultiMatches);
        let handle_delete_all_data = self.link.callback(|_| Msg::DeleteAllData);
        let handle_make_matches = self.link.callback(|e: MouseEvent| {
            e.prevent_default();
            Msg::MakeMatches
        });
        let handle_change_data = self.link.callback(|e: MouseEvent| {
            e.prevent_default();
            Msg::ChangeData
        });
        let handle_print_page = self.link.callback(|e: MouseEvent| {
            e.prevent_default();
            Msg::PrintPage
        });

        html! {
            <>
            <div class="container-lg">
                <div class="row d-print-none mt-3">
                    <div class="col shadow p-3 mb-5 bg-white rounded">
                        <h1>{"Eerlijke Indeling"}</h1>
                        <p>{ "Eerlijke Indeling is een veilige applicatie om leerlingen in te delen bij activiteiten of workshops. Om een rechtvaardige verdeling te maken, wordt gebruik gemaakt van het algoritme dat in Amsterdam wordt ingezet bij toewijzing van leerlingen aan scholen." }</p>
                        <button name="more_info" class="btn btn-info btn-sm float-right" data-toggle="modal" data-target="#more_info_modal">{ "Over deze applicatie" }</button>
                    </div>
                </div>
                {
                    if let Some(match_result) = &self.state.match_result {
                        html! {
                            <MatchResultBlock match_result=match_result.clone() categories=self.state.categories.clone() />
                        }
                    } else {
                        html! {
                            <>
                                <div class="row">
                                    <CategoryBlock categories=self.state.categories.clone() editing=self.ephemeral_state.editing on_add_category=handle_on_add_category on_edit_category=handle_on_edit_category on_editing=handle_on_editing.clone() on_remove_category=handle_on_remove_category />
                                </div>
                                <div class="row">
                                    <StudentBlock students=self.state.students.clone() categories=self.state.categories.clone() editing=self.ephemeral_state.editing on_editing=handle_on_editing.clone() on_add_student=handle_on_add_student on_add_preference=handle_on_add_preference on_add_exclude=handle_on_add_exclude on_move_preference=handle_on_move_preference.clone() on_remove_preference=handle_on_remove_preference.clone() on_remove_exclude=handle_on_remove_exclude.clone() on_edit_student=handle_on_edit_student.clone() on_remove_student=handle_on_remove_student.clone() />
                                </div>
                            </>
                        }
                    }
                }
                <div class="row d-print-none">
                    <div class="col shadow p-3 mb-5 bg-white rounded">
                        <h2 class="mb-3">{ "Stap 3: Indeling maken" }</h2>
                        {
                            if self.state.match_result.is_none() {
                                html! {
                                    <>
                                    <p>{ "Klik op 'Indeling' maken om de leerlingen eerlijk te verdelen over de ingevoerde activiteiten." }</p>
                                    <p>{ "Standaard wordt iedere leerling toegewezen aan één van de activiteiten. Kunnen leerlingen aan meer dan één activiteit meedoen, bijvoorbeeld tijdens een sportdag of wanneer de activiteiten op verschillende dagen plaatsvinden? Selecteer dan het vinkje 'Leerlingen kunnen aan meerdere activiteiten meedoen'." }</p>
                                    </>
                                }
                            } else {
                                html! {
                                    <>
                                    <p>{ "Je kunt nu de gemaakte indeling printen of handmatig kopiëren naar Word of Excel. Als je nog iets wilt aanpassen, klik je op 'Gegevens aanpassen'." }</p>
                                    <p>{ "Let op: klik alleen op de knop 'Alles verwijderen' als je de gemaakte indeling hebt gekopieerd. Je invoer wordt gewist en alle velden zijn hierna leeg. Zo kun je weer een nieuwe indeling maken." }</p>
                                    </>
                                }
                            }
                        }
                        <div class="row">
                            <div class="col col-sm-4">
                                <button name="clear_data" class="btn btn-danger" data-toggle="modal" data-target="#confirm_delete_all_modal">{ "Alles verwijderen" }</button>
                                {
                                    if self.state.match_result.is_some() {
                                        html! {
                                            <button name="change_data" class="btn btn-warning ml-sm-3" onclick=handle_change_data>{ "Gegevens aanpassen" }</button>
                                        }
                                    } else {
                                        html! { }
                                    }
                                }
                            </div>
                            <div class="col col-sm-8">
                                <form class="form-inline float-right">
                                    {
                                        if self.state.match_result.is_none() {
                                            html! {
                                                <>
                                                    <div class="form-group form-check mr-sm-3">
                                                        <input type="checkbox" class="form-check-input" id="multi_matches" ref=self.multi_matches_ref.clone() onclick=handle_toggle_multitmatches checked=self.state.multi_matches />
                                                        <label class="form-check-label" for="multi_matches" data-toggle="tooltip" title="Standaard wordt iedere leerling in slechts één activiteit ingedeeld. Door er voor te kiezen leerlingen aan meerdere activiteiten mee te laten doen, worden leerlingen ingedeeld aan alle activiteiten waar zij aan mee willen doen, zolang er plekken zijn binnen deze activiteiten. Hierbij wordt rekening gehouden met de voorkeuren van de leerling.">{ "Leerlingen kunnen aan meerdere activiteiten meedoen" }</label>
                                                    </div>
                                                    <button name="make_matches" class="btn btn-success" onclick=handle_make_matches>{ "Indeling maken" }</button>
                                                </>
                                            }
                                        } else {
                                            html! {
                                                <button name="print" class="btn btn-info" onclick=handle_print_page>{ "Printen" }</button>
                                            }
                                        }
                                    }
                                </form>
                            </div>
                        </div>
                    </div>
                </div>
            </div>

            <Modal id="more_info_modal" title="Over deze applicatie" btn_label="" btn_type="" handle_modal_action=None >
                <p>{ "Alle ingevoerde gegevens voor de applicatie Eerlijke Indeling worden enkel in de browser van je eigen computer opgeslagen. Er wordt dus géén data naar een server of externe partijen verstuurd. De knop 'Alles verwijderen' zorgt ook daadwerkelijk dat alle gegevens worden verwijderd. De namen van de leerlingen zijn daarmee veilig en hun privacy blijft gewaarborgd. Let op dat de uiteindelijke Eerlijke Indeling daarmee ook komt te verdwijnen, noteer deze dus tijdig." }</p>
                <p>{ "Zonder het gebruik van de knop 'Alles Verwijderen' blijven je gegevens bewaard, ook na het sluiten van je browser. Dat betekent dat je op een later moment je Eerlijke Indeling af kunt maken, aan kunt passen of opnieuw kunt kopiëren." }</p>
                <p>{ "De achterliggende techniek waar deze website gebruik van maakt, is het zogenaamde Deferred Acceptance, Single-Tie-Break algoritme. Meer informatie over dit algoritme en de gebruikte manier van indelen kun je vinden in het document " }<a href="https://staff.fnwi.uva.nl/b.bredeweg/pdf/BSc/20152016/Klijnsma.pdf" target="_blank">{ "Matching algorithms for the secondary school admission problem in Amsterdam" }</a></p>
                <h5>{ "Contact" }</h5>
                <p>{ "Mocht je vragen, ideeën of opmerkingen hebben, neem dan contact op via onderstaand formulier." }</p>
                <form name="contact" method="POST">
                    <input type="hidden" name="form-name" value="contact" />
                    <div class="form-group">
                        <label for="email">{ "E-mailadres" }</label>
                        <input type="email" class="form-control" id="email" name="email" aria-describedby="emailHelp"/>
                        <small id="emailHelp" class="form-text text-muted">{ "Je e-mailadres wordt enkel gebruikt om antwoord te kunnen geven op eventuele vragen." }</small>
                    </div>
                    <div class="form-group">
                        <label for="name">{ "Naam" }</label>
                        <input type="text" class="form-control" id="name" name="name"/>
                    </div>
                      <div class="form-group">
                        <label for="message">{ "Bericht" }</label>
                        <textarea class="form-control" id="message" name="message" rows="3" required=true></textarea>
                    </div>
                    <button type="submit" class="btn btn-primary">{ "Verstuur" }</button>
                </form>
                <br/>
                <hr/>
                <p>
                    <a href="https://www.flickr.com/photos/14829735@N00/4483674964">{ "Achtergrondfoto" }</a>{ " door " }<a href="https://www.flickr.com/photos/14829735@N00">{ "dullhunk" }</a>{ " met licentie " }<a href="https://creativecommons.org/licenses/by/2.0/?ref=ccsearch&atype=html" style="margin-right: 5px;">{ "CC BY 2.0" }</a>
                </p>
                <p>{ "Deze app is vrije software: je mag het herdistribueren en/of wijzigen onder de voorwaarden van de GNU Algemene Publieke Licentie zoals gepubliceerd door de Free Software Foundation, onder versie 3 van de " }<a href="http://www.gnu.org/licenses/" target="_blank">{ "licentie" }</a>{ " of (naar jouw keuze) elke latere versie." }</p>
                <p>{ "Deze app is gedistribueerd in de hoop dat het nuttig zal zijn maar ZONDER ENIGE GARANTIE; zelfs zonder de impliciete garanties die GEBRUIKELIJK ZIJN IN DE HANDEL of voor BRUIKBAARHEID VOOR EEN SPECIFIEK DOEL." }</p>
                <p>{ "De broncode voor deze app is beschikbaar op " }<a href="https://github.com/deliriouspenguin/eerlijke-indeling" target="_blank">{ "https://github.com/deliriouspenguin/eerlijke-indeling" }</a></p>
            </Modal>
            <Modal id="confirm_delete_all_modal" title="Alles verwijderen" btn_label="Alles verwijderen" btn_type="danger" handle_modal_action=Some(handle_delete_all_data)>
                { "Weet je zeker dat je ingevoerde gegevens wilt verwijderen? Dit is niet ongedaan te maken." }
            </Modal>
        </>
        }
    }
}
