use super::forms::{detail_fields, kontak_fields, pendaftar_fields};
use crate::components::common::form_field::FormField;
use wasm_bindgen::JsCast;
use yew::prelude::*;
use yew_hooks::use_map;

#[function_component(DaftarLombaComponent)]
pub fn daftar_lomba_component() -> Html {
	let pendaftar_data = use_map(
		pendaftar_fields
			.iter()
			.cloned()
			.map(|fields| (fields.key, "".to_string()))
			.collect(),
	);

	let detail_data = use_map(
		detail_fields
			.iter()
			.cloned()
			.map(|fields| (fields.key, "".to_string()))
			.collect(),
	);

	let kontak_data = use_map(
		kontak_fields
			.iter()
			.cloned()
			.map(|fields| (fields.key, "".to_string()))
			.collect(),
	);

	html! {
		<>
			<div class="bg-blue-gradient-app shadow-xl drop-shadow-xl px-6 py-10 gap-5">
				<div class="text-white text-2xl font-semibold">{"Daftarkan Lomba"}</div>
				<div class="text-white text-2xl font-semibold">{"Lembaga / Kampus Anda"}</div>
			</div>
			<div class="p-10">
			<div class="text-cyan-500 font-semibold text-xl">{"Identitas Pendaftar"}</div>
			<hr class="mb-3"/>
			<div class="grid grid-cols-2 gap-8 mb-5">
				{
					for pendaftar_fields.iter().cloned().map(|field_property| {
						let form_data = pendaftar_data.clone();
						let field_property = field_property.clone();
						html_nested! {
							<FormField
								field_property={field_property.clone()}
								key_input={field_property.key}
								form_data={form_data.clone()}
							/>
						}
					})
				}
			</div>
			<div class="text-cyan-500 font-semibold text-xl">{"Detil Kompetisi"}</div>
			<hr class="mb-3"/>
			<div class="grid grid-cols-2 gap-8 mb-5">
				{
					for detail_fields.iter().cloned().map(|field_property| {
						let form_data = detail_data.clone();
						let field_property = field_property.clone();
						html_nested! {
							<FormField
								field_property={field_property.clone()}
								key_input={field_property.key}
								form_data={form_data.clone()}
							/>
						}
					})
				}
			</div>
			<div class="text-cyan-500 font-semibold text-xl">{"Kontak Penyelenggara / Kompetisi"}</div>
			<hr class="mb-3"/>
			<div class="grid grid-cols-2 gap-8 mb-5">
				{
					for kontak_fields.iter().cloned().map(|field_property| {
						let form_data = kontak_data.clone();
						let field_property = field_property.clone();
						html_nested! {
							<FormField
								field_property={field_property.clone()}
								key_input={field_property.key}
								form_data={form_data.clone()}
							/>
						}
					})
				}
			</div>
			<label class="text-sm font-bold py-2 px-1 capitalize" for="user-avatar"> {"Poster Kompetisi"} </label>
			<input class="block w-[45%] cursor-pointer bg-gray-50 border border-gray-300 text-gray-900 focus:outline-none focus:border-transparent text-sm rounded-lg" aria-describedby="user_avatar_help" id="user_avatar" type="file"/>
			<span>{"Catatan: file harus dalam format .png atau .jpg"}</span>
			<button class="px-8 py-2 my-5 rounded-lg hover:text-cyan-400 hover:bg-white text-white shadow block bg-cyan-400 border-cyan-400 font-bold transition">{"Kirim"}</button>
			</div>
		</>
	}
}
