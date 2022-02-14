use crate::components::common::modal::Modal;
use crate::router::{KompetisiQuery, Route};
use crate::utils::interop;
use wasm_bindgen::JsCast;
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(Hero)]
pub fn hero() -> Html {
	let history = use_history().unwrap();
	let input_ref = use_node_ref();
	let modal_ref = use_node_ref();

	let onclick = {
		let input_ref = input_ref.clone();
		Callback::once(move |_| {
			if let Some(input) = input_ref.cast::<web_sys::HtmlInputElement>() {
				let value = input.value();
				history.push_with_query(
					Route::Kompetisi,
					KompetisiQuery {
						search: value,
						category: String::from(""),
					},
				);
			};
		})
	};

	let onclick_test = {
		move |_| {
			crate::utils::interop::show_modal("modal".to_string());
		}
	};

	html! {
		<>
		<Modal title="nani">
			<div>
				{"YO MAN"}
			</div>
		</Modal>
		<button onclick={onclick_test}>
			{"YO MAN"}
		</button>
		<div class="sm:h-[90vh] md:h-auto md:w-auto w-screen flex relative bg-slate-400 sm:bg-white sm:shadow sm:justify-center">
			  <div class="relative w-full sm:w-auto p-16 flex justify-center items-center">
				<img class="opacity-40 sm:opacity-100 min-w-[20rem]" src="https://res.cloudinary.com/dw4bwn79m/image/upload/v1644602940/Frame_sf5tth.png" alt="Logo Web" />
			</div>
			<div class="absolute z-2 flex flex-col justify-center align-center p-16 gap-3 w-screen sm:relative sm:w-auto">
				<div class="w-20 h-1 bg-blue-300"></div>
				<div class="font-bold text-[3rem]">{"WE-COME"}</div>
				<p class="text-2xl">{"Find competition information with ease"}</p>
				<div class="flex-col flex">
				<input ref={input_ref} class="appearance-none border border-blue-300 rounded w-full py-2 px-3 text-gray-700 leading-tight" id="username" type="text" placeholder="Cari Kompetisi" />
				<button {onclick} class="w-24 px-4 py-2 my-2 rounded-lg hover:text-cyan-400 hover:bg-white text-white shadow block bg-cyan-400 border-cyan-400 font-bold transition">{"Cari"}</button>
				</div>
			</div>
		</div>
		</>
	}
}
