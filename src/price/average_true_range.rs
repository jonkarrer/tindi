pub fn calc_average_true_range(
    highs: &[f32],
    lows: &[f32],
    closes: &[f32],
    period: usize,
) -> Vec<f32> {
    if highs.len() < 2 || highs.len() != lows.len() || highs.len() != closes.len() {
        println!("Error: Invalid input data for atr calculation");
        return vec![];
    }

    // Calculate True Range for each candle
    let mut tr_values: Vec<f32> = Vec::with_capacity(highs.len());

    // First TR is just High - Low for the first candle
    tr_values.push(highs[0] - lows[0]);

    // Calculate TR for remaining candles
    for i in 1..highs.len() {
        let tr = (highs[i] - lows[i])
            .max((highs[i] - closes[i - 1]).abs())
            .max((lows[i] - closes[i - 1]).abs());
        tr_values.push(tr);
    }

    // Calculate ATR using simple moving average
    let mut atr_values: Vec<f32> = Vec::with_capacity(highs.len());

    // First ATR is average of first 'period' TR values
    if tr_values.len() >= period {
        let first_atr: f32 = tr_values[0..period].iter().sum::<f32>() / period as f32;
        atr_values.push(first_atr);

        // Calculate subsequent ATR values using the smoothing formula
        // ATR = ((Prior ATR * (period - 1)) + Current TR) / period
        for i in period..tr_values.len() {
            let atr =
                (atr_values.last().unwrap() * (period - 1) as f32 + tr_values[i]) / period as f32;
            atr_values.push(atr);
        }
    }

    atr_values
}
