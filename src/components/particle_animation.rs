use leptos::prelude::*;

#[component]
pub fn ParticleAnimation() -> impl IntoView {
    view! {
        <div class="absolute inset-0 overflow-hidden">
            <div class="particle-container">
                // Create multiple particle elements
                {(0..50).map(|i| {
                    let delay = format!("{}s", i as f32 * 0.1);
                    let duration = format!("{}s", 20.0 + (i as f32 * 0.5));
                    let start_x = format!("{}%", (i * 17) % 100);
                    let start_y = format!("{}%", (i * 23) % 100);

                    view! {
                        <div
                            class="particle"
                            style={format!(
                                "left: {}; top: {}; animation-delay: {}; animation-duration: {};",
                                start_x, start_y, delay, duration
                            )}
                        />
                    }
                }).collect::<Vec<_>>()}
            </div>

            // Add connecting lines as pseudo-elements via CSS
            <style>
                r#"
                .particle-container {
                    position: absolute;
                    inset: 0;
                    width: 100%;
                    height: 100%;
                }
                
                .particle {
                    position: absolute;
                    width: 4px;
                    height: 4px;
                    background-color: rgba(23, 23, 23, 0.5);
                    border-radius: 50%;
                    animation: float-particle 20s infinite ease-in-out;
                }
                
                .dark .particle {
                    background-color: rgba(245, 245, 245, 0.5);
                }
                
                .particle::before {
                    content: '';
                    position: absolute;
                    width: 80px;
                    height: 1px;
                    background: linear-gradient(90deg, transparent, rgba(23, 23, 23, 0.2), transparent);
                    transform-origin: 0 50%;
                    animation: connect-line 8s infinite ease-in-out;
                }
                
                .dark .particle::before {
                    background: linear-gradient(90deg, transparent, rgba(245, 245, 245, 0.2), transparent);
                }
                
                @keyframes float-particle {
                    0%, 100% {
                        transform: translate(0, 0) scale(1);
                    }
                    25% {
                        transform: translate(30px, -40px) scale(1.1);
                    }
                    50% {
                        transform: translate(-20px, 20px) scale(0.9);
                    }
                    75% {
                        transform: translate(40px, 30px) scale(1.05);
                    }
                }
                
                @keyframes connect-line {
                    0%, 100% {
                        transform: rotate(0deg) scaleX(0);
                        opacity: 0;
                    }
                    20% {
                        transform: rotate(45deg) scaleX(1);
                        opacity: 0.3;
                    }
                    40% {
                        transform: rotate(-30deg) scaleX(1.2);
                        opacity: 0.2;
                    }
                    60% {
                        transform: rotate(60deg) scaleX(0.8);
                        opacity: 0.3;
                    }
                    80% {
                        transform: rotate(-45deg) scaleX(1);
                        opacity: 0.1;
                    }
                }
                
                /* Add some particles with different animation timing */
                .particle:nth-child(odd) {
                    animation-direction: reverse;
                }
                
                .particle:nth-child(3n) {
                    animation-duration: 25s;
                }
                
                .particle:nth-child(5n) {
                    width: 6px;
                    height: 6px;
                }
                
                .particle:nth-child(7n)::before {
                    width: 120px;
                    animation-duration: 10s;
                }
                "#
            </style>
        </div>
    }
}
