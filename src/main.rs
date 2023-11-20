#![allow(non_snake_case)]
pub mod result;
pub mod models;


use yew::prelude::*;
use crate::result::ResponseResult;
use crate::models::*;

#[derive(Properties, PartialEq)]
pub struct SubjectProps {
    pub name: AttrValue
}

#[function_component(SubjectItem)]
fn subject_item(props: &SubjectProps) -> Html {
    html! {
        <li>
            {props.name.clone()}
        </li>
    }
}

#[derive(Properties, PartialEq)]
pub struct SubjectsProps {
    pub subjects: UseStateHandle<Vec<Subject>>
}

#[function_component(SubjectList)]
fn subject_list(props: &SubjectsProps) -> Html {
    let subjects = props.subjects.clone();
    html! {
        <ol>
            { 
                subjects
                .iter()
                .map(|i| html! {
                    <SubjectItem name={AttrValue::from(i.ten_mon_hoc.as_ref().unwrap().as_str().to_owned())}>
                    </SubjectItem>
                })
                .collect::<Html>() 
            }
        </ol>
    }
}

#[derive(Properties, PartialEq)]
pub struct CourseProps {
    pub id: i8,
    pub name: AttrValue,
    pub subjects: UseStateHandle<Vec<Subject>>
}

#[function_component(CourseItem)]
fn course_item(props: &CourseProps) -> Html {
    let id = props.id.clone();
    let subjects = props.subjects.clone();
    let onclick = Callback::from (move |_| {
        let subjects = subjects.clone();
        wasm_bindgen_futures::spawn_local(async move {
            let url = format!("http://localhost:5149/api/v1/subjects/course/{}", id);
            let result = reqwest::get(url)
            .await
            .unwrap()
            .json::<ResponseResult<Vec<Subject>>>()
            .await
            .unwrap();

            subjects.set(result.data.unwrap());
        });
    });
    html! {
        <li onclick={onclick}>
            {props.name.clone()}
        </li>
    }
}

#[derive(Properties, PartialEq)]
pub struct CoursesProps {
    pub courses: UseStateHandle<Vec<Course>>,
    pub subjects: UseStateHandle<Vec<Subject>>
}

#[function_component(CourseList)]
fn course_list(props: &CoursesProps) -> Html {
    let courses = props.courses.clone();
    let subjects = props.subjects.clone();
    html! {
        <ul>
            { 
                courses
                .iter()
                .map(|i| html! {
                    <CourseItem id={i.id} name={AttrValue::from(i.ten_khoa_hoc.as_ref().unwrap().as_str().to_owned())} subjects={subjects.clone()}>
                    </CourseItem>
                })
                .collect::<Html>() 
            }
        </ul>
    }
}

#[function_component(App)]
fn app() -> Html {
    let courses = use_state(|| Vec::new());
    let courses_cloned = courses.clone();
    let subjects = use_state(|| Vec::new());
    let subjects_cloned = subjects.clone();
    let default_name = "".to_string();

    let course_name = match subjects_cloned.iter().last() {
        Some(r) => {
            *&(r as &Subject).khoa_hoc.ten_khoa_hoc.as_ref().unwrap()
        },
        None => {
            &default_name
        }
    };

    use_effect_with((), move |_| {
        let courses = courses.clone();
        wasm_bindgen_futures::spawn_local(async move {
            let courses = courses.clone();
            let result = reqwest::get("http://localhost:5149/api/v1/courses")
            .await
            .unwrap()
            .json::<ResponseResult<Vec<Course>>>()
            .await
            .unwrap();

            courses.set(result.data.unwrap());
        });
    });

    html! {
        <div id="root">
            <div id="subjects">
                <div id="course_name">
                    <p>
                        {format!("Khoá học: {}", course_name)}
                    </p>
                </div>
                <div>
                    <SubjectList subjects={subjects.clone()}>
                    </SubjectList>
                </div>
            </div>
            <div id="courses">
                <div id="courses_text">
                    <p>
                        {"Khoá học"}
                    </p>
                </div>
                <div>
                    <CourseList courses={courses_cloned} subjects={subjects}>
                    </CourseList>
                </div>
            </div>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}