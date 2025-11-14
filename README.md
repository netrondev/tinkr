# Tinkr

`⚠️ EVERYTHING UNDER EXPERIMENTAL DEVELOPMENT ⚠️`

[![Crates.io](https://img.shields.io/crates/v/tinkr.svg)](https://crates.io/crates/tinkr)
[![Documentation](https://docs.rs/tinkr/badge.svg)](https://docs.rs/tinkr)
[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](https://github.com/netrondev/tinkr)

A full-stack web framework for Rust that makes building modern web applications fast and enjoyable. Built on [Leptos](https://leptos.dev), Tinkr provides a comprehensive suite of UI components, authentication, database integration, and deployment tools to help you ship production-ready applications quickly.

## Features

### 🎨 Rich UI Component Library
- **Pre-built Components**: Buttons, inputs, modals, dropdowns, tabs, tooltips, and more
- **Form Handling**: Integrated form components with validation support
- **Loading States**: Progress bars, loading indicators, and skeletons
- **Navigation**: Sidebar, navbar, breadcrumbs, and navigation helpers
- **Avatars**: Boring Avatars integration with multiple styles (Beam, Marble, Ring, Pixel, Bauhaus, Sunset)
- **Alerts & Notifications**: Built-in alert system with different severity levels
- **Responsive Design**: Mobile-first components that work across all devices

### 🔐 Authentication & Authorization
- **OAuth Integration**: Built-in OAuth2 support for third-party authentication
- **Session Management**: Secure session handling with token-based authentication
- **User Management**: Complete user profile and account management system
- **Guest Access**: Support for guest users and anonymous sessions
- **Authorization Checks**: Easy-to-use authorization middleware and guards

### 🗄️ Database & Storage
- **SurrealDB Integration**: First-class support for SurrealDB with async operations
- **Caching Layer**: Built-in caching with the `cached` crate for improved performance
- **Storage Traits**: Flexible storage abstraction for implementing custom backends
- **Type-Safe Queries**: Strongly typed database operations with compile-time safety
- **Migrations**: Schema management and database initialization helpers

### 💳 Payment Processing
- **PayFast Integration**: South African payment gateway support
- **Transaction Management**: Handle payments and subscriptions
- **Webhook Handling**: Process payment notifications and updates

### 🪙 Web3 & Wallet Support
- **MetaMask Integration**: Connect and interact with MetaMask wallets
- **EVM Chains**: Support for Ethereum and EVM-compatible chains
- **Address Verification**: Checksum validation and address utilities
- **Wallet Authentication**: Sign-in with Ethereum/Web3 wallets

### 📊 Observability & Telemetry
- **OpenTelemetry**: Built-in metrics and tracing support
- **Grafana Loki**: Log aggregation integration
- **Prometheus Metrics**: Metrics endpoint for monitoring
- **Pyroscope Profiling**: CPU profiling integration
- **Custom Logging**: Structured logging with tracing

### 🚀 Server & Middleware
- **Axum Integration**: High-performance async HTTP server
- **Compression**: Gzip, Brotli, Deflate, and Zstd compression support
- **CORS**: Cross-origin resource sharing middleware
- **Health Checks**: Built-in health and readiness endpoints
- **Metrics Endpoint**: Prometheus-compatible metrics
- **Request Logging**: Comprehensive request/response logging

### 📧 Email Integration
- **Resend API**: Send transactional emails via Resend
- **Email Validation**: Built-in email address validation
- **Template Support**: Email template helpers

### 🛠️ Utilities
- **Date & Time**: Chrono-based date utilities and helpers
- **String Manipulation**: Common string operations and transformations
- **Color Utilities**: Color parsing, conversion, and manipulation
- **URL Handling**: URL parsing and construction helpers
- **Image Processing**: Image upload, compression, and format conversion
- **MIME Type Detection**: Automatic file type detection

## Installation

Add Tinkr to your `Cargo.toml`:

```toml
[dependencies]
tinkr = "0.0.32"
leptos = "0.8"
```

For SSR (Server-Side Rendering) support:

```toml
[dependencies]
tinkr = { version = "0.0.32", features = ["ssr"] }
```

For hydration (client-side):

```toml
[dependencies]
tinkr = { version = "0.0.32", features = ["hydrate"] }
```

## Quick Start

### Basic Component Usage

```rust
use leptos::*;
use tinkr::components::*;

#[component]
pub fn App() -> impl IntoView {
    view! {
        <Page>
            <Hero
                title="Welcome to Tinkr"
                subtitle="Build full-stack web apps with Rust"
            >
                <HeroButton href="/docs">"Get Started"</HeroButton>
            </Hero>

            <Section>
                <FeatureGrid>
                    <FeatureCard
                        title="Fast"
                        description="Built with performance in mind"
                    />
                    <FeatureCard
                        title="Type-Safe"
                        description="Catch errors at compile time"
                    />
                    <FeatureCard
                        title="Full-Stack"
                        description="Frontend and backend in one language"
                    />
                </FeatureGrid>
            </Section>
        </Page>
    }
}
```

### Form Handling

```rust
use leptos::*;
use tinkr::components::*;

#[component]
pub fn LoginForm() -> impl IntoView {
    let (email, set_email) = create_signal(String::new());
    let (password, set_password) = create_signal(String::new());

    view! {
        <FormSection title="Sign In">
            <FormGrid>
                <Input
                    label="Email"
                    type_="email"
                    value=email
                    on_input=move |v| set_email(v)
                />
                <Input
                    label="Password"
                    type_="password"
                    value=password
                    on_input=move |v| set_password(v)
                />
            </FormGrid>
            <FormActions>
                <SubmitButton>"Sign In"</SubmitButton>
            </FormActions>
        </FormSection>
    }
}
```

### Authentication

```rust
use leptos::*;
use tinkr::auth::*;

#[server]
async fn check_auth() -> Result<User, ServerFnError> {
    let user = user()?; // Get current authenticated user
    Ok(user)
}

#[component]
pub fn ProtectedPage() -> impl IntoView {
    let user = create_resource(|| (), |_| check_auth());

    view! {
        <Suspense fallback=|| view! { <Loading /> }>
            {move || user.get().map(|result| match result {
                Ok(user) => view! {
                    <div>"Welcome, " {user.email}</div>
                }.into_view(),
                Err(_) => view! {
                    <div>"Please sign in"</div>
                }.into_view(),
            })}
        </Suspense>
    }
}
```

### Datetime

```rust
use tinkr::Datetime;
use tinkr::date_utils::FormatDatetime;

let created_at: Datetime;
created_at.format_custom("%a %d %b"); // Mon 01 Jan
created_at.ago(); // 9 days ago
```


### Database Integration (SSR)

```rust
#[cfg(feature = "ssr")]
use tinkr::db::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Todo {
    id: Option<RecordId>,
    title: String,
    completed: bool,
}

#[server]
async fn get_todos() -> Result<Vec<Todo>, ServerFnError> {
    let db = db_init().await?;
    let todos: Vec<Todo> = db.select("todos").await?;
    Ok(todos)
}

#[server]
async fn create_todo(title: String) -> Result<Todo, ServerFnError> {
    let db = db_init().await?;
    let todo: Todo = db.create("todos")
        .content(Todo {
            id: None,
            title,
            completed: false,
        })
        .await?;
    Ok(todo)
}
```

## Configuration

### Environment Variables

Tinkr uses environment variables for configuration. Create a `.env` file:

```env
# Database
SURREALDB_HOST=ws://localhost:8000
SURREALDB_NS=your_namespace
SURREALDB_DB=your_database
SURREALDB_USER=root
SURREALDB_PASS=root

# Auth
JWT_SECRET=your-secret-key-here

# Email (optional)
RESEND_API_KEY=your-resend-api-key

# OAuth (optional)
OAUTH_CLIENT_ID=your-client-id
OAUTH_CLIENT_SECRET=your-client-secret

# Telemetry (optional)
OTEL_EXPORTER_OTLP_ENDPOINT=http://localhost:4317
```

## Components Reference

### Layout Components
- `Page` - Main page wrapper with responsive layout
- `Section` - Content section with optional header
- `Hero` - Hero section with title, subtitle, and CTA buttons
- `Footer` - Application footer

### Navigation
- `NavBar` - Top navigation bar
- `SideBar` - Collapsible sidebar navigation
- `NavigationBackButton` - Browser back button
- `Dropdown` - Dropdown menu with trigger and items

### Forms
- `Input` - Text input with label and validation
- `Checkbox` - Checkbox input
- `Select` - Select dropdown
- `SubmitButton` - Form submit button with loading state
- `FormSection` - Form wrapper with title
- `FormGrid` - Responsive form grid layout

### UI Elements
- `Button` - Customizable button component
- `Modal` - Modal dialog with backdrop
- `Alert` - Alert messages with severity levels
- `Tooltip` - Hover tooltip
- `Loading` - Loading spinner
- `ProgressBar` - Progress indicator
- `Tabs` - Tab navigation

### Display
- `Image` - Optimized image component
- `UserAvatar` - User avatar with fallback
- `Logo` - Application logo
- `Timer` - Countdown/elapsed time display

## Features Flag

Tinkr supports the following feature flags:

- `default` - Includes `ssr` feature
- `ssr` - Server-side rendering with Axum, SurrealDB, and all backend features
- `hydrate` - Client-side hydration support

## Architecture

Tinkr follows a modular architecture:

```
tinkr/
├── auth/           # Authentication & authorization
├── components/     # UI component library
├── db/             # Database layer
├── email/          # Email integration
├── middleware/     # Server middleware
├── payments/       # Payment processing
├── telemetry/      # Observability
├── wallet/         # Web3 wallet integration
└── utils/          # Shared utilities
```

## Development

### Running Tests

```bash
cargo test
```

### Building

```bash
# Development build
cargo build

# Production build
cargo build --release
```

### Release

The project includes a release script:

```bash
./release.sh
```

## Contributing

Contributions are welcome! Please feel free to submit issues and pull requests.

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Links

- [Documentation](https://docs.rs/tinkr)
- [Crates.io](https://crates.io/crates/tinkr)
- [GitHub Repository](https://github.com/netrondev/tinkr)
- [Leptos Framework](https://leptos.dev)

## Support

For questions and support, please open an issue on [GitHub](https://github.com/netrondev/tinkr/issues).
