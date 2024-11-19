#[derive(Debug, PartialEq, Eq)]
pub enum CurveError {
    Overflow,
    ZeroAmount
}

/// Calculates the constant product (k) from reserves x and y
/// Returns k = x * y
#[inline]
pub fn k_from_xy(x: u64, y: u64) -> Result<u128, CurveError> {
    if x == 0 || y == 0 {
        return Err(CurveError::ZeroAmount);
    }
    Ok((x as u128).checked_mul(y as u128).ok_or(CurveError::Overflow)?)
}

/// Calculates the spot price of one token in terms of the other
/// Returns price = (x * precision) / y
#[inline]
pub fn spot_price_from_pair(x: u64, y: u64, precision: u32) -> Result<u64, CurveError> {
    if x == 0 || y == 0 {
        return Err(CurveError::ZeroAmount);
    }
    Ok(
        u64::try_from(
            (x as u128)
            .checked_mul(precision as u128).ok_or(CurveError::Overflow)?
            .checked_div(y as u128).ok_or(CurveError::Overflow)?
            .checked_div(precision as u128).ok_or(CurveError::Overflow)?
        ).map_err(|_| CurveError::Overflow)?
    )
}

/// Calculates deposit amounts of X and Y tokens from liquidity token amount
#[inline]
pub fn xy_deposit_amounts_from_l(x: u64, y: u64, l: u64, a: u64, precision: u32) -> Result<(u64, u64), CurveError> {
    let ratio = (l as u128)
        .checked_add(a as u128).ok_or(CurveError::Overflow)?
        .checked_mul(precision as u128).ok_or(CurveError::Overflow)?
        .checked_div(l as u128).ok_or(CurveError::Overflow)?;
    let deposit_x = (x as u128)
        .checked_mul(ratio).ok_or(CurveError::Overflow)?
        .checked_div(precision as u128).ok_or(CurveError::Overflow)?
        .checked_sub(x as u128).ok_or(CurveError::Overflow)? as u64;
    let deposit_y = (y as u128)
        .checked_mul(ratio).ok_or(CurveError::Overflow)?
        .checked_div(precision as u128).ok_or(CurveError::Overflow)?
        .checked_sub(y as u128).ok_or(CurveError::Overflow)? as u64;
    Ok((
        deposit_x,
        deposit_y
    ))
}

/// Calculates withdrawal amounts of X and Y tokens from liquidity token amount
#[inline]
pub fn xy_withdraw_amounts_from_l(x: u64, y: u64, l: u64, a: u64, precision: u32) -> Result<(u64, u64), CurveError> {
    let ratio = ((l - a) as u128)
    .checked_mul(precision as u128).ok_or(CurveError::Overflow)?
    .checked_div(l as u128).ok_or(CurveError::Overflow)?;

    let withdraw_x = (x as u128)
        .checked_sub((x as u128)
            .checked_mul(ratio).ok_or(CurveError::Overflow)?
            .checked_div(precision as u128).ok_or(CurveError::Overflow)?
        ).ok_or(CurveError::Overflow)? as u64;

    let withdraw_y = (y as u128)
        .checked_sub((y as u128)
            .checked_mul(ratio).ok_or(CurveError::Overflow)?
            .checked_div(precision as u128).ok_or(CurveError::Overflow)?
        ).ok_or(CurveError::Overflow)? as u64;

    Ok((
        withdraw_x, 
        withdraw_y
    ))
}

/// Calculates new X balance after Y token swap
#[inline]
pub fn x2_from_y_swap_amount(x: u64, y: u64, a: u64) -> Result<u64, CurveError> {
    let k = k_from_xy(x, y)?;
    let x_new = (y as u128).checked_add(a as u128).ok_or(CurveError::Overflow)?;
    Ok(k.checked_div(x_new).ok_or(CurveError::Overflow)? as u64)
}

/// Calculates new Y balance after X token swap
#[inline]
pub fn y2_from_x_swap_amount(x: u64, y: u64, a: u64) -> Result<u64, CurveError> {
    x2_from_y_swap_amount(y,x,a)
}

/// Calculates X token output amount from Y token input
#[inline]
pub fn delta_x_from_y_swap_amount(x: u64, y: u64, a: u64) -> Result<u64, CurveError> {
    Ok(x.checked_sub(x2_from_y_swap_amount(x,y,a)?).ok_or(CurveError::Overflow)?)
}

/// Calculates Y token output amount from X token input
#[inline]
pub fn delta_y_from_x_swap_amount(x: u64, y: u64, a: u64) -> Result<u64, CurveError> {
    delta_x_from_y_swap_amount(y,x,a)
}

/// Calculates X token output amount and fee from Y token input
#[inline]
pub fn delta_x_from_y_swap_amount_with_fee(x: u64, y: u64, a: u64, fee: u16) -> Result<(u64, u64), CurveError> {
    let raw_amount = x.checked_sub(x2_from_y_swap_amount(x,y,a)?).ok_or(CurveError::Overflow)?;
    let amount = raw_amount.checked_mul((10_000 - fee).into()).ok_or(CurveError::Overflow)?.saturating_div(10_000);
    Ok((amount, raw_amount - amount))
}

/// Calculates Y token output amount and fee from X token input
#[inline]
pub fn delta_y_from_x_swap_amount_with_fee(x: u64, y: u64, a: u64, fee: u16) -> Result<(u64, u64), CurveError> {
    delta_x_from_y_swap_amount_with_fee(y,x,a, fee)
}

#[cfg(test)]
mod tests {
    use crate::delta_y_from_x_swap_amount_with_fee;
    #[test]
    fn swap() {
        // Test case with unbalanced pool (100X, 150Y)
        let (amount_out, fee) = delta_y_from_x_swap_amount_with_fee(100, 150, 10, 0).unwrap();
        assert_eq!(amount_out, 14);
        assert_eq!(fee, 0);

        // Test case with balanced pool (100X, 100Y)
        let (amount_out, fee) = delta_y_from_x_swap_amount_with_fee(100, 100, 10, 0).unwrap();
        assert_eq!(amount_out, 10);
        assert_eq!(fee, 0);
    }

    #[test]
    fn swap_with_fee() {
        // Test with 1% fee (100 basis points)
        let (amount_out, fee) = delta_y_from_x_swap_amount_with_fee(100, 150, 10, 100).unwrap();
        assert_eq!(amount_out, 13);
        assert_eq!(fee, 1);
    }
}