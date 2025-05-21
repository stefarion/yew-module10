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
        <div class="bg-gradient-to-br from-violet-700 to-indigo-900 min-h-screen w-screen flex items-center justify-center">
            <div class="bg-white/90 rounded-2xl shadow-2xl p-8 w-full max-w-md flex flex-col items-center">
            <h1 class="text-4xl font-black text-violet-700 mb-6 tracking-tight drop-shadow-lg text-center">
                {"Welcome to YewChat"}
            </h1>
            <p class="text-base text-gray-700 mb-8 text-center font-medium">
                {"Enter your username to start chatting with the community!"}
            </p>
            <form class="w-full flex" onsubmit={Callback::from(|e: FocusEvent| e.prevent_default())}>
                <input
                {oninput}
                class="flex-1 rounded-l-lg px-4 py-2 text-base border-gray-200 text-gray-800 bg-white focus:outline-none focus:ring-2 focus:ring-violet-400"
                placeholder="Username"
                />
                <Link<Route> to={Route::Chat} classes="contents">
                <button
                    {onclick}
                    disabled={username.is_empty()}
                    class="px-6 py-2 rounded-r-lg bg-violet-600 text-white font-bold text-base uppercase border-violet-600 transition-colors duration-200 hover:bg-violet-700 disabled:opacity-50"
                >
                    {"Go Chatting!"}
                </button>
                </Link<Route>>
            </form>
            </div>
        </div>
    }
}