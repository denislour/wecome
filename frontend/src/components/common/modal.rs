use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
	pub children: Children,
	pub title: &'static str,
	pub id: &'static str,
}

#[function_component(Modal)]
pub fn modal(props: &Props) -> Html {
	let modal_ready = crate::utils::interop::use_tw();
	if modal_ready {
		html! {
			<div class="modal fade fixed top-0 left-0 hidden w-full h-full outline-none overflow-x-hidden overflow-y-auto" id={props.id} tabindex="-1" aria-labelledby={props.id} aria-modal="true" role="dialog">
				<div class="modal-dialog modal-dialog-centered modal-dialog-scrollable relative w-auto pointer-events-none">
					<div class="modal-content border-none shadow-lg relative flex flex-col w-full pointer-events-auto bg-white bg-clip-padding rounded-md outline-none text-current">
					<div class="modal-header flex flex-shrink-0 items-center justify-between p-4 border-b border-gray-200 rounded-t-md">
						<h5 class="text-xl font-medium leading-normal text-gray-800" id="exampleModalCenteredScrollableLabel">
						{props.title}
						</h5>
						<button type="button"
						class="btn-close box-content w-4 h-4 p-1 text-black border-none rounded-none opacity-50 focus:shadow-none focus:outline-none focus:opacity-100 hover:text-black hover:opacity-75 hover:no-underline"
						data-bs-dismiss="modal" aria-label="Close"></button>
					</div>
					<div class="modal-body relative p-4">
						{ for props.children.iter() }
					</div>
					<div
						class="modal-footer flex flex-shrink-0 flex-wrap items-center justify-end p-4 border-t border-gray-200 rounded-b-md">
						<button type="button"
						class="inline-block px-6 py-2.5 bg-purple-600 text-white font-medium text-xs leading-tight uppercase rounded shadow-md hover:bg-purple-700 hover:shadow-lg focus:bg-purple-700 focus:shadow-lg focus:outline-none focus:ring-0 active:bg-purple-800 active:shadow-lg transition duration-150 ease-in-out"
						data-bs-dismiss="modal">
							{"Close"}
						</button>
					</div>
					</div>
				</div>
			</div>
		}
	} else {
		html! {
			<>
			</>
		}
	}
}
