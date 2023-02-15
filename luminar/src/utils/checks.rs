use crate::utils::luminar::{LuminarContext, LuminarError};

pub async fn owner_check(ctx: LuminarContext<'_>) -> Result<bool, LuminarError> {
    // Replace 7 with your ID to make this check pass.
    if ctx.author().id != 594627968668794896 {
        return Ok(false);
    }

    Ok(true)
}
