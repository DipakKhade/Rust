use anyhow::Ok;

fn validate_even_no(n: u32) -> anyhow::Result<bool> {
    if n % 2 == 0 {
        Ok(true)
    } else {
        Err(anyhow::anyhow!("the no {} is not even", n))
    }
}


fn main() -> anyhow::Result<()> {
    let is_even = validate_even_no(4)?;
    Ok(())
}