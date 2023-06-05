use yew::prelude::*;

#[function_component]
pub fn Login() -> Html {
    html! {
        <div class="bg-white">
            <div class="relative isolate px-6 pt-14">
                <div class="absolute inset-x-0 -top-40 -z-10 transform-gpu overflow-hidden blur-3xl sm:-top-80" aria-hidden="true">
                    <div class="relative left-[calc(50%-11rem)] aspect-[1155/678] w-[36.125rem] -translate-x-1/2 rotate-[30deg] bg-gradient-to-tr from-[#ff80b5] to-[#9089fc] opacity-30" style="clip-path: polygon(74.1% 44.1%, 100% 61.6%, 97.5% 26.9%, 85.5% 0.1%, 80.7% 2%, 72.5% 32.5%, 60.2% 62.4%, 52.4% 68.1%, 47.5% 58.3%, 45.2% 34.5%, 27.5% 76.7%, 0.1% 64.9%, 17.9% 100%, 27.6% 76.8%, 76.1% 97.7%, 74.1% 44.1%)"></div>
                </div>
                <div class="mx-auto max-w-2xl py-32">
                    <div class="mb-8 flex justify-center">
                        <div class="relative rounded-full px-3 py-1 text-sm leading-6 text-gray-600 ring-1 ring-gray-900/10 hover:ring-gray-900/20">
                            {"지금은 일부 사용자만 접근 가능해요. "}
                            <a href="#" class="font-semibold text-gray-900">
                                <span class="absolute inset-0" aria-hidden="true"></span>
                                {"더 보기"}
                            </a>
                        </div>
                    </div>
                    <div class="text-center">
                        <h1 class="text-4xl font-bold tracking-tight text-gray-900">
                            {"Sched Bird"}
                        </h1>
                        <p class="mt-6 text-lg leading-relaxed text-gray-600">{"간편하게 일정을 관리해보세요."}<br/>{"구성원 간 일정을 쉽게 공유하고 확인할 수 있습니다."}</p>
                        <div class="mt-10 flex flex-col items-center justify-center gap-y-5">
                            <a href="https://sched.sinabro.io/auth" class="rounded-md bg-stone-900 text-sm px-3.5 py-2.5 font-semibold text-white shadow-sm hover:bg-stone-700 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-600">
                                {"GitHub로 로그인하기"}
                            </a>
                            <a href="#" class="text-sm font-semibold leading-normal text-gray-900">
                                {"문의하러 가기 "}
                                <span aria-hidden="true">{"→"}</span>
                            </a>
                        </div>
                    </div>
                </div>
                <div class="absolute inset-x-0 top-[calc(100%-13rem)] -z-10 transform-gpu overflow-hidden blur-3xl top-[calc(100%-30rem)]" aria-hidden="true">
                    <div class="relative left-[calc(50%+3rem)] aspect-[1155/678] w-[36.125rem] -translate-x-1/2 bg-gradient-to-tr from-[#ff80b5] to-[#9089fc] opacity-30 sm:left-[calc(50%+36rem)] sm:w-[72.1875rem]" style="clip-path: polygon(74.1% 44.1%, 100% 61.6%, 97.5% 26.9%, 85.5% 0.1%, 80.7% 2%, 72.5% 32.5%, 60.2% 62.4%, 52.4% 68.1%, 47.5% 58.3%, 45.2% 34.5%, 27.5% 76.7%, 0.1% 64.9%, 17.9% 100%, 27.6% 76.8%, 76.1% 97.7%, 74.1% 44.1%)"></div>
                </div>
            </div>
        </div>
    }
}
