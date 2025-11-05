use leptos::prelude::*;

enum PaymentGateway {
    Payfast,
    Luno,
}

enum MoneyRegion {
    Country(String),
    Global,
}

struct Money {
    pub name: String,
    pub symbol: String,
    pub provider: PaymentGateway,
    pub accepted_region: MoneyRegion,
}

impl Money {
    pub async fn from_symbol(symbol: &str) -> Result<Option<Self>, crate::AppError> {
        let accepted = Self::list_accepted_by_country().await?;
        for money in accepted {
            if money.symbol.eq_ignore_ascii_case(symbol) {
                return Ok(Some(money));
            }
        }
        Ok(None)
    }

    pub async fn list_accepted_by_country() -> Result<Vec<Self>, crate::AppError> {
        let accepted: Vec<Money> = vec![
            Money {
                name: "South African Rand".into(),
                symbol: "ZAR".into(),
                provider: PaymentGateway::Payfast,
                accepted_region: MoneyRegion::Country("ZA".into()),
            },
            Money {
                name: "Solana".into(),
                symbol: "SOL".into(),
                provider: PaymentGateway::Luno,
                accepted_region: MoneyRegion::Global,
            },
        ];

        Ok(accepted)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn money_list_accepted_by_country() {
        let result = Money::from_symbol("ZAR").await;
        assert!(result.is_ok());
        let result_opt = result.unwrap();

        assert!(result_opt.is_some());

        let test_money_currency = result_opt.unwrap();

        assert!(!test_money_currency.name.is_empty());
        assert!(!test_money_currency.symbol.is_empty());
    }
}

#[component]
pub fn CurrencyWalletPicker() -> impl IntoView {
    view! { <div>"Currency Wallet Picker"</div> }
}
