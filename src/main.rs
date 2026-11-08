use dioxus::prelude::*;

const MAIN_CSS: Asset = asset!("/assets/main.css");
const BEER_CSS: Asset = asset!("/assets/beercss/beercss.min.css");
const BEER_JS: Asset = asset!("/assets/beercss/beer.min.js");
const MDC_JS: Asset = asset!("/assets/beercss/material-dynamic-colors.min.js");


fn main() { dioxus::launch(App); }

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Link { rel: "stylesheet", href: BEER_CSS }
        document::Script { type: "module", src: BEER_JS }
        document::Script { type: "module", src: MDC_JS }
        Main {}
    }
}

#[component]
pub fn Main() -> Element {
    let mut height_var = use_signal(|| 0.0);
    let mut weight_var = use_signal(|| 0.0);
    let mut final_bmi_var = use_signal(|| 0.0);

    let mut calculate_bmi = move |height: f32, weight: f32| {
        let bmi = weight / (height * height);
        final_bmi_var.set(bmi.round());
    };

    rsx! {
        div {
            class: "main",

            article {
                class: "column",
                span {
                    style: "margin-bottom: 12px;",
                    class: "bigText",
                    "BMI Calculator"
                }

                div {
                    class: "rows_of_input",
                    div {
                        class: "field label border",
                        input { oninput: move |e| height_var.set(e.value().parse::<f32>().unwrap_or(0.0)) }
                        label { "Height" }
                    }
    
                    div {
                        class: "field label border",
                        input { oninput: move |e| weight_var.set(e.value().parse::<f32>().unwrap_or(0.0)) }
                        label { "Weight" }
                    }
                }

                button {
                    style: "width: fit-content",
                    onclick: move |_| {
                        let h: f32 = height_var();
                        let w: f32 = weight_var();
                        calculate_bmi(h, w)
                    },
                    i { "check" }
                    "Calculate"
                }

                span {
                    class: "mediumText",
                    "Your BMI is; {final_bmi_var}"
                }
            }
        }
    }
}
