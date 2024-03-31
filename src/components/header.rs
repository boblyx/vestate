#![allow(non_snake_case)]

use dioxus::prelude::*;

#[component]
pub fn Header() -> Element {
    rsx! {
        div {
            width: "95"
            ,min_height: "5svw"
            ,background_color:"#347B98"
            ,color: "#FFF"
            ,class: "m-2 p-4"
            ,div{
                p{
                class: "text-xl",
                "Vestate"
                }
                p{
                class: "text-m",
                "Explorer for estate data"
                }
            }
        }    
    }
}
