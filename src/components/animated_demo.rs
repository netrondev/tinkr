use leptos::prelude::*;
use phosphor_leptos::{Icon, IconData};

#[component]
pub fn AnimatedDemo(title: &'static str, icon: IconData, children: Children) -> impl IntoView {
    view! {
        <div class="relative bg-neutral-900 dark:bg-neutral-950 rounded-2xl p-8 overflow-hidden group">
            // Background pattern
            <div class="absolute inset-0 opacity-5">
                <div class="absolute inset-0 bg-[radial-gradient(circle_at_50%_50%,rgba(255,255,255,0.1)_0%,transparent_50%)]"></div>
            </div>

            // Header
            <div class="relative z-10 flex items-center gap-3 mb-6">
                <div class="w-10 h-10 rounded-lg bg-gradient-to-br from-neutral-700 to-neutral-800 flex items-center justify-center">
                    <Icon icon={icon} size="20px" color="white" />
                </div>
                <h3 class="text-lg font-semibold text-neutral-100">{title}</h3>
            </div>

            // Demo content
            <div class="relative z-10">
                {children()}
            </div>
        </div>
    }
}

#[component]
pub fn WebGLDemo() -> impl IntoView {
    view! {
        <div class="relative h-64 bg-gradient-to-br from-neutral-900 via-purple-900/20 to-blue-900/20 rounded-lg overflow-hidden">
            <style>
                {r#"
                @keyframes rotate3d {
                    0% {
                        transform: rotateX(0deg) rotateY(0deg);
                    }
                    100% {
                        transform: rotateX(0deg) rotateY(360deg);
                    }
                }
                @keyframes glow-pulse {
                    0%, 100% {
                        opacity: 0.3;
                        transform: scale(1);
                    }
                    50% {
                        opacity: 0.8;
                        transform: scale(1.1);
                    }
                }
                .perspective-1000 {
                    perspective: 1000px;
                }
                .preserve-3d {
                    transform-style: preserve-3d;
                }
                .face {
                    position: absolute;
                    width: 100%;
                    height: 100%;
                    backface-visibility: hidden;
                    border: 1px solid rgba(255, 255, 255, 0.1);
                }
                "#}
            </style>

            // Grid pattern background
            <div class="absolute inset-0">
                <svg class="w-full h-full opacity-10">
                    <defs>
                        <pattern id="grid" width="40" height="40" patternUnits="userSpaceOnUse">
                            <path d="M 40 0 L 0 0 0 40" fill="none" stroke="white" stroke-width="1"/>
                        </pattern>
                    </defs>
                    <rect width="100%" height="100%" fill="url(#grid)" />
                </svg>
            </div>

            <div class="absolute inset-0 flex items-center justify-center perspective-1000">
                <div class="relative w-40 h-40 preserve-3d" style="animation: rotate3d 15s infinite linear">
                    // Main cube with simplified faces
                    <div class="absolute w-full h-full preserve-3d">
                        // Front face
                        <div class="face bg-gradient-to-br from-blue-500/40 to-purple-500/40 backdrop-blur-sm flex items-center justify-center"
                             style="transform: translateZ(80px)">
                            <div class="text-white/80 text-4xl font-bold">3D</div>
                        </div>
                        // Back face
                        <div class="face bg-gradient-to-br from-purple-500/40 to-pink-500/40 backdrop-blur-sm"
                             style="transform: rotateY(180deg) translateZ(80px)" />
                        // Right face
                        <div class="face bg-gradient-to-br from-cyan-500/40 to-blue-500/40 backdrop-blur-sm"
                             style="transform: rotateY(90deg) translateZ(80px)" />
                        // Left face
                        <div class="face bg-gradient-to-br from-pink-500/40 to-purple-500/40 backdrop-blur-sm"
                             style="transform: rotateY(-90deg) translateZ(80px)" />
                        // Top face
                        <div class="face bg-gradient-to-br from-purple-500/40 to-blue-500/40 backdrop-blur-sm"
                             style="transform: rotateX(90deg) translateZ(80px)" />
                        // Bottom face
                        <div class="face bg-gradient-to-br from-blue-500/40 to-cyan-500/40 backdrop-blur-sm"
                             style="transform: rotateX(-90deg) translateZ(80px)" />
                    </div>

                    // Glowing corners
                    {[
                        (-1, -1, -1), (1, -1, -1), (-1, 1, -1), (1, 1, -1),
                        (-1, -1, 1), (1, -1, 1), (-1, 1, 1), (1, 1, 1)
                    ].into_iter().enumerate().map(|(i, (x, y, z))| {
                        let transform = format!(
                            "translateX({}px) translateY({}px) translateZ({}px)",
                            x * 80, y * 80, z * 80
                        );
                        let delay = format!("{}s", i as f32 * 0.2);
                        view! {
                            <div
                                class="absolute w-4 h-4 bg-white rounded-full"
                                style:transform=transform
                                style:animation=format!("glow-pulse 2s infinite {}", delay)
                            />
                        }
                    }).collect_view()}
                </div>
            </div>

            // UI overlay
            <div class="absolute bottom-4 left-4 right-4 flex items-center justify-between">
                <div class="text-neutral-300 text-sm font-medium">WebGL 3D Graphics</div>
                <div class="flex gap-2">
                    <div class="w-2 h-2 bg-green-400 rounded-full animate-pulse" />
                    <span class="text-neutral-400 text-xs">60 FPS</span>
                </div>
            </div>
        </div>
    }
}

#[component]
pub fn RealtimeDataDemo() -> impl IntoView {
    // Crypto trading data
    let (current_price, _set_current_price) = signal(42875.23f64);
    let (change_percent, _set_change_percent) = signal(3.47f64);
    let (volume_24h, _set_volume_24h) = signal("$28.4B");

    view! {
        <div class="relative h-64 bg-gradient-to-br from-neutral-900 to-neutral-950 rounded-lg overflow-hidden">
            <style>
                {r#"
                @keyframes candle-update {
                    0% {
                        opacity: 0.6;
                    }
                    100% {
                        opacity: 1;
                    }
                }
                @keyframes price-pulse {
                    0%, 100% {
                        transform: scale(1);
                    }
                    50% {
                        transform: scale(1.02);
                    }
                }
                .grid-line {
                    stroke: rgba(255, 255, 255, 0.05);
                    stroke-width: 1;
                }
                .price-line {
                    stroke: #10b981;
                    stroke-width: 2;
                    fill: none;
                    filter: drop-shadow(0 0 4px rgba(16, 185, 129, 0.5));
                }
                .candle-green {
                    fill: #10b981;
                }
                .candle-red {
                    fill: #ef4444;
                }
                "#}
            </style>

            // Trading UI Header
            <div class="relative z-20 p-4 pb-2">
                <div class="flex justify-between items-start">
                    <div>
                        <div class="flex items-center gap-2 text-sm text-neutral-400 mb-1">
                            <span class="font-semibold text-white">BTC/USD</span>
                            <span>Bitcoin</span>
                        </div>
                        <div class="flex items-baseline gap-3">
                            <span class="text-2xl font-bold text-white" style="animation: price-pulse 2s infinite">
                                ${move || {
                                    let price = current_price.get();
                                    let whole = price as u64;
                                    let decimal = ((price - whole as f64) * 100.0) as u64;
                                    format!("{},{:02}", whole, decimal)
                                }}
                            </span>
                            <span class=move || {
                                if change_percent.get() > 0.0 {
                                    "text-green-400 text-sm font-medium"
                                } else {
                                    "text-red-400 text-sm font-medium"
                                }
                            }>
                                {move || format!("{:+.2}%", change_percent.get())}
                            </span>
                        </div>
                    </div>
                    <div class="text-right">
                        <div class="text-xs text-neutral-500">24h Volume</div>
                        <div class="text-sm font-medium text-white">{move || volume_24h.get()}</div>
                    </div>
                </div>
            </div>

            // Chart area
            <div class="absolute inset-x-0 bottom-0 top-16">
                <svg class="w-full h-full" viewBox="0 0 400 140" preserveAspectRatio="none">
                    <defs>
                        <linearGradient id="green-gradient" x1="0%" y1="0%" x2="0%" y2="100%">
                            <stop offset="0%" style="stop-color:#10b981;stop-opacity:0.2" />
                            <stop offset="100%" style="stop-color:#10b981;stop-opacity:0" />
                        </linearGradient>
                    </defs>

                    // Grid
                    {(0..5).map(|i| {
                        let y = i * 35;
                        view! {
                            <line x1="0" y1=y x2="400" y2=y class="grid-line" />
                        }
                    }).collect_view()}
                    {(0..9).map(|i| {
                        let x = i * 50;
                        view! {
                            <line x1=x y1="0" x2=x y2="140" class="grid-line" />
                        }
                    }).collect_view()}

                    // Candlesticks
                    {(0..20).map(|i| {
                        let x = i * 20 + 10;
                        let height = 20 + (i * 7 % 30);
                        let y = 70 - height / 2;
                        let is_green = i % 3 != 0;
                        let wick_top = y - 10;
                        let wick_bottom = y + height + 10;

                        view! {
                            <g style=format!("animation: candle-update 0.5s ease-out {}s", i as f32 * 0.05)>
                                // Wick
                                <line
                                    x1=x
                                    y1=wick_top
                                    x2=x
                                    y2=wick_bottom
                                    stroke=if is_green { "#10b981" } else { "#ef4444" }
                                    stroke-width="1"
                                />
                                // Candle body
                                <rect
                                    x=x - 4
                                    y=y
                                    width="8"
                                    height=height
                                    class=if is_green { "candle-green" } else { "candle-red" }
                                    rx="1"
                                />
                            </g>
                        }
                    }).collect_view()}

                    // Moving average line
                    <path
                        d="M 0,80 Q 50,75 100,70 T 200,65 T 300,60 T 400,55"
                        class="price-line"
                    />
                    <path
                        d="M 0,80 Q 50,75 100,70 T 200,65 T 300,60 T 400,55 L 400,140 L 0,140 Z"
                        fill="url(#green-gradient)"
                    />

                    // Current price line
                    <line x1="0" y1="55" x2="400" y2="55" stroke="#10b981" stroke-width="1" stroke-dasharray="5,5" opacity="0.5" />
                </svg>
            </div>

            // Bottom indicators
            <div class="absolute bottom-2 left-4 right-4 flex items-center justify-between text-xs">
                <div class="flex gap-4">
                    <span class="text-neutral-500">15m</span>
                    <span class="text-neutral-500">RSI: 64.2</span>
                    <span class="text-neutral-500">MA(20)</span>
                </div>
                <div class="flex items-center gap-2">
                    <div class="w-2 h-2 bg-green-400 rounded-full animate-pulse" />
                    <span class="text-neutral-400">Live Trading</span>
                </div>
            </div>
        </div>
    }
}

#[component]
pub fn FileUploadDemo() -> impl IntoView {
    let (progress, set_progress) = signal(0);
    let (is_uploading, set_is_uploading) = signal(false);

    let start_upload = move |_| {
        if !is_uploading.get() {
            set_is_uploading.set(true);
            set_progress.set(0);
            // Simulate upload progress
            set_progress.set(100);
        }
    };

    view! {
        <div class="space-y-4">
            <button
                on:click=start_upload
                disabled=move || is_uploading.get()
                class="w-full py-3 px-4 bg-blue-600 hover:bg-blue-700 disabled:bg-neutral-700 text-white rounded-lg"
            >
                {move || if is_uploading.get() { "Uploading..." } else { "Start Upload" }}
            </button>

            <div class="relative h-2 bg-neutral-700 rounded-full overflow-hidden">
                <div
                    class=move || format!(
                        "absolute inset-y-0 left-0 bg-gradient-to-r from-blue-500 to-cyan-500 {}",
                        if is_uploading.get() { "duration-[2000ms]" } else { "duration-200" }
                    )
                    style:width=move || format!("{}%", progress.get())
                ></div>
            </div>

            <div class="text-neutral-400 text-sm">
                Progress: {move || progress.get()}%
            </div>
        </div>
    }
}

#[component]
pub fn AIChatDemo() -> impl IntoView {
    view! {
        <div class="space-y-3">
            <div class="flex gap-3">
                <div class="w-8 h-8 rounded-full bg-blue-600 flex-shrink-0"></div>
                <div class="flex-1 bg-neutral-800 rounded-lg p-3 text-neutral-300 text-sm">
                    "How can I optimize my application's performance?"
                </div>
            </div>
            <div class="flex gap-3">
                <div class="w-8 h-8 rounded-full bg-gradient-to-br from-purple-600 to-pink-600 flex-shrink-0"></div>
                <div class="flex-1 bg-neutral-800 rounded-lg p-3 text-neutral-300 text-sm">
                    <div class="flex gap-1 mb-2">
                        <div class="w-2 h-2 bg-neutral-500 rounded-full animate-pulse"></div>
                        <div class="w-2 h-2 bg-neutral-500 rounded-full animate-pulse animation-delay-200"></div>
                        <div class="w-2 h-2 bg-neutral-500 rounded-full animate-pulse animation-delay-400"></div>
                    </div>
                    "I can help you analyze performance bottlenecks..."
                </div>
            </div>
        </div>
    }
}

#[component]
pub fn CompressionDemo() -> impl IntoView {
    let (size, set_size) = signal(100);
    let (is_compressing, set_is_compressing) = signal(false);

    let compress = move |_| {
        if !is_compressing.get() {
            set_is_compressing.set(true);
            set_size.set(15);
        }
    };

    view! {
        <div class="space-y-4">
            <div class="flex items-center justify-between">
                <div class="text-neutral-300">Original: 2.5 MB</div>
                <div class="text-neutral-300">Compressed: {move || format!("{:.1} MB", size.get() as f32 * 0.025)}</div>
            </div>

            <div class="relative h-16 bg-neutral-800 rounded-lg overflow-hidden">
                <div
                    class=move || format!(
                        "absolute inset-y-0 left-0 bg-gradient-to-r from-orange-500 to-red-500 {}",
                        if is_compressing.get() { "duration-[1500ms]" } else { "duration-300" }
                    )
                    style:width=move || format!("{}%", size.get())
                ></div>
            </div>

            <button
                on:click=compress
                disabled=move || is_compressing.get()
                class="w-full py-3 px-4 bg-orange-600 hover:bg-orange-700 disabled:bg-neutral-700 text-white rounded-lg"
            >
                {move || if is_compressing.get() { "Compressing..." } else { "Compress File" }}
            </button>
        </div>
    }
}
