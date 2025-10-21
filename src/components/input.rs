use leptos::prelude::*;

#[derive(Debug, Clone, PartialEq)]
pub enum InputType {
    Text,
    Email,
    Password,
    Number,
    Tel,
    Url,
    Search,
    Date,
    Time,
    DateTime,
}

impl InputType {
    pub fn as_str(&self) -> &'static str {
        match self {
            InputType::Text => "text",
            InputType::Email => "email",
            InputType::Password => "password",
            InputType::Number => "number",
            InputType::Tel => "tel",
            InputType::Url => "url",
            InputType::Search => "search",
            InputType::Date => "date",
            InputType::Time => "time",
            InputType::DateTime => "datetime-local",
        }
    }
}

pub fn input_class_default() -> String {
    "
        mt-1 block w-full border rounded-md border-neutral-300 dark:border-neutral-600 dark:bg-neutral-700 dark:text-white shadow-sm focus:border-indigo-500 focus:ring-indigo-500 dark:focus:border-indigo-400 dark:focus:ring-indigo-400 sm:text-sm p-2 bg-white dark:bg-neutral-900 text-neutral-600
    ".into()
}

#[component]
pub fn Input(
    #[prop(optional, into)] id: Option<String>,
    #[prop(optional, into)] name: Option<String>,
    #[prop(optional)] r#type: Option<InputType>,
    #[prop(optional, into)] placeholder: Option<String>,
    #[prop(optional)] value: Option<RwSignal<String>>,
    #[prop(optional)] on_input: Option<Box<dyn Fn(String) + 'static>>,
    #[prop(optional)] on_change: Option<Box<dyn Fn(String) + 'static>>,
    #[prop(optional)] required: Option<bool>,
    #[prop(optional)] disabled: Option<bool>,
    #[prop(optional)] readonly: Option<bool>,
    #[prop(optional, into)] class: Option<String>,
    #[prop(optional, into)] min: Option<String>,
    #[prop(optional, into)] max: Option<String>,
    #[prop(optional, into)] step: Option<String>,
    #[prop(optional, into)] pattern: Option<String>,
    #[prop(optional)] maxlength: Option<i32>,
    #[prop(optional)] autofocus: Option<bool>,
    #[prop(optional, into)] autocomplete: Option<String>,
) -> impl IntoView {
    let input_type = r#type.unwrap_or(InputType::Text);
    let base_class = r#"
        mt-1 block w-full border rounded-md border-neutral-300 dark:border-neutral-600 dark:bg-neutral-700 dark:text-white 
        shadow-sm focus:border-indigo-500 focus:ring-indigo-500 dark:focus:border-indigo-400 dark:focus:ring-indigo-400 sm:text-sm
        p-2 bg-white dark:bg-neutral-900 text-neutral-600
    "#;
    let final_class = match class {
        Some(custom_class) => format!("{} {}", base_class, custom_class),
        None => base_class.to_string(),
    };

    view! {
        <input
            type=input_type.as_str()
            id=id
            name=name
            class=final_class
            placeholder=placeholder
            value=move || value.map(|v| v.get()).unwrap_or_default()
            on:input=move |ev| {
                let val = event_target_value(&ev);
                if let Some(v) = value {
                    v.set(val.clone());
                }
                if let Some(handler) = &on_input {
                    handler(val);
                }
            }
            on:change=move |ev| {
                if let Some(handler) = &on_change {
                    handler(event_target_value(&ev));
                }
            }
            required=required.unwrap_or(false)
            disabled=disabled.unwrap_or(false)
            readonly=readonly.unwrap_or(false)
            min=min
            max=max
            step=step
            pattern=pattern
            maxlength=maxlength
            autofocus=autofocus.unwrap_or(false)
            autocomplete=autocomplete
        />
    }
}

#[component]
pub fn Label(
    #[prop(optional, into)] r#for: Option<String>,
    #[prop(optional, into)] class: Option<String>,
    children: Children,
) -> impl IntoView {
    let base_class = "block text-sm font-medium text-neutral-700 dark:text-neutral-300";
    let final_class = match class {
        Some(custom_class) => format!("{} {}", base_class, custom_class),
        None => base_class.to_string(),
    };

    view! {
        <label for=r#for class=final_class>
            {children()}
        </label>
    }
}

#[component]
pub fn FormField(
    #[prop(optional, into)] label: Option<String>,
    #[prop(optional, into)] label_for: Option<String>,
    #[prop(optional, into)] error: Option<String>,
    #[prop(optional, into)] help_text: Option<String>,
    #[prop(optional, into)] class: Option<String>,
    children: Children,
) -> impl IntoView {
    view! {
        <div class=class
            .unwrap_or_else(|| {
                "".to_string()
            })>
            {label
                .map(|l| view! { <Label r#for=label_for.clone().unwrap_or_default()>{l}</Label> })}
            {children()}
            {help_text
                .map(|text| {
                    view! {
                        <p class="mt-1 text-sm text-neutral-500 dark:text-neutral-400">{text}</p>
                    }
                })}
            {error
                .map(|err| {
                    view! { <p class="mt-1 text-sm text-red-600 dark:text-red-400">{err}</p> }
                })}
        </div>
    }
}
