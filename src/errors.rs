use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("empty elements")]
    EmptyElements,
    #[error("invalid num")]
    InvalidNum,
}

pub fn sum_nums(nums: &[i32]) -> Result<i32, Error> {
    if nums.len() == 0 {
        return Err(Error::EmptyElements);
    }

    let mut total = 0;
    let mut idx = 0 as usize;
    loop {
        if idx >= nums.len() {
            break;
        }

        let j = idx + 1;

        let mut next = 0 as i32;
        if j < nums.len() {
            next = nums[j];
        }

        let sum = sum2(nums[idx], next)?;
        total += sum;
        idx = j + 1;
    }

    Ok(total)
}

fn sum2(a: i32, b: i32) -> Result<i32, Error> {
    if b < 0 {
        return Err(Error::InvalidNum);
    }

    Ok(a + b)
}

#[cfg(test)]
mod tests {
    use super::sum_nums;

    #[test]
    fn test_sum_works() -> anyhow::Result<()> {
        let sum = sum_nums(vec![1, 2, 3].as_slice())?;
        assert_eq!(sum, 6);
        Ok(())
    }
}
