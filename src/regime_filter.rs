pub fn apply_regime_filter(
    watchlist: &[&str],
    regime_symbol: &str,
    target_trend: Trend,
) -> Vec<String> {
    let mut symbols: Vec<String> = Vec::new();

    if regime_trend(regime_symbol) == target_trend {
        let multi_bars = get_multi_bars(watchlist, "1Day", Some("start=2023-01-01"));
        for (symbol, bars) in multi_bars {
            let prices = bars.get_closes();

            if simple_moving_average_trend(&prices, 20, 100) == target_trend {
                // ** Stock is trending with regime
                symbols.push(String::from(symbol));
            }
        }
    }
    symbols
}

fn regime_trend(regime_symbol: &str) -> Trend {
    let prices = get_bars(regime_symbol, "1Day", Some("start=2023-01-01")).get_closes();

    if simple_moving_average_trend(&prices, 20, 100) == Trend::Bullish {
        return Trend::Bullish;
    };
    Trend::Bearish
}
