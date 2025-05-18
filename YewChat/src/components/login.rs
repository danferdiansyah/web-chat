use web_sys::HtmlInputElement;
use yew::functional::*;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::Route;
use crate::User;

#[function_component(Login)]
pub fn login() -> Html {
    let username = use_state(|| String::new());
    let user = use_context::<User>().expect("No context found.");

    let oninput = {
        let current_username = username.clone();

        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            current_username.set(input.value());
        })
    };

    let onclick = {
        let username = username.clone();
        let user = user.clone();
        Callback::from(move |_| *user.username.borrow_mut() = (*username).clone())
    };

    html! {
       <div class="bg-gray-900 flex w-screen min-h-screen">
            <div class="container mx-auto flex flex-col justify-center items-center">
                <form class="m-4 flex w-full max-w-md">
                    <input {oninput} class="rounded-l-lg p-4 border-t mr-0 border-b border-l text-gray-200 border-gray-600 bg-gray-700 placeholder-gray-400 w-full focus:outline-none focus:ring-2 focus:ring-green-500" placeholder="Username" />
                    <Link<Route> to={Route::Chat}> 
                        <button {onclick} disabled={username.len()<1} class="px-8 rounded-r-lg bg-green-500 hover:bg-green-600 text-white font-bold p-4 uppercase border-green-500 border-t border-b border-r focus:outline-none focus:ring-2 focus:ring-green-300 transition-colors duration-200 disabled:opacity-50 disabled:cursor-not-allowed">
                            {"Go Chatting!"}
                        </button>
                    </Link<Route>>
                </form>
            </div>
        </div>
    }
}
