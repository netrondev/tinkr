use leptos::prelude::*;

#[component]
pub fn EvmHome() -> impl IntoView {
    view! {
        <div class="container mx-auto px-4 py-8">
            <h1 class="text-3xl font-bold mb-6">"EVM Explorer"</h1>

            <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
                // Wallet Explorer Card
                <a href="/evm/wallet" class="block">
                    <div class="border border-neutral-200 dark:border-neutral-700 rounded-lg p-6 hover:shadow-lg cursor-pointer">
                        <div class="flex items-center mb-4">
                            <svg class="w-8 h-8 text-blue-500" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 10h18M7 15h1m4 0h1m-7 4h12a3 3 0 003-3V8a3 3 0 00-3-3H6a3 3 0 00-3 3v8a3 3 0 003 3z" />
                            </svg>
                            <h3 class="text-xl font-semibold ml-3">"Wallet Explorer"</h3>
                        </div>
                        <p class="text-neutral-600 dark:text-neutral-400">
                            "Check balances across multiple chains and testnets for any wallet address"
                        </p>
                    </div>
                </a>

                // RPC Demo Card
                <a href="/evm/demo" class="block">
                    <div class="border border-neutral-200 dark:border-neutral-700 rounded-lg p-6 hover:shadow-lg cursor-pointer">
                        <div class="flex items-center mb-4">
                            <svg class="w-8 h-8 text-green-500" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 9l3 3-3 3m5 0h3M5 20h14a2 2 0 002-2V6a2 2 0 00-2-2H5a2 2 0 00-2 2v12a2 2 0 002 2z" />
                            </svg>
                            <h3 class="text-xl font-semibold ml-3">"RPC Demo"</h3>
                        </div>
                        <p class="text-neutral-600 dark:text-neutral-400">
                            "Test Ethereum RPC endpoints and explore blockchain data"
                        </p>
                    </div>
                </a>

                // Chain Explorer Card (future feature)
                <div class="opacity-50 cursor-not-allowed">
                    <div class="border border-neutral-200 dark:border-neutral-700 rounded-lg p-6">
                        <div class="flex items-center mb-4">
                            <svg class="w-8 h-8 text-purple-500" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 10V3L4 14h7v7l9-11h-7z" />
                            </svg>
                            <h3 class="text-xl font-semibold ml-3">"Chain Explorer"</h3>
                        </div>
                        <p class="text-neutral-600 dark:text-neutral-400">
                            "Browse and compare different blockchain networks (Coming Soon)"
                        </p>
                    </div>
                </div>
            </div>

            <div class="mt-12">
                <h2 class="text-2xl font-bold mb-4">"Quick Stats"</h2>
                <div class="grid grid-cols-1 md:grid-cols-3 gap-4">
                    <div class="bg-neutral-100 dark:bg-neutral-800 rounded-lg p-4">
                        <p class="text-sm text-neutral-600 dark:text-neutral-400">"Supported Chains"</p>
                        <p class="text-2xl font-bold">500+</p>
                    </div>
                    <div class="bg-neutral-100 dark:bg-neutral-800 rounded-lg p-4">
                        <p class="text-sm text-neutral-600 dark:text-neutral-400">"Available RPCs"</p>
                        <p class="text-2xl font-bold">1000+</p>
                    </div>
                    <div class="bg-neutral-100 dark:bg-neutral-800 rounded-lg p-4">
                        <p class="text-sm text-neutral-600 dark:text-neutral-400">"Testnet Support"</p>
                        <p class="text-2xl font-bold">Yes</p>
                    </div>
                </div>
            </div>
        </div>
    }
}
