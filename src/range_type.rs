use serde_json::Value;

use crate::JQError;

/// Contains the start and stop indexes of a range operation.
///
/// A range can be expresses as one of:
/// * `[start:stop]` : returns the elements from start (inclusive) to stop (exclusive)
/// * `[isize:]` : returns all elements from stop to end of the array
/// * `[:isize]`
/// * `[:]`
#[derive(PartialEq, Debug, Clone)]
pub struct RangeType {
    start: Option<isize>,
    end: Option<isize>,
}

impl Default for RangeType {
    fn default() -> Self {
        Self::new()
    }
}

impl RangeType {
    /// A range can be empty, in which case, all elements are iterated
    /// Example, the range `[:]` for `[1,2,3]` returns
    /// 1
    /// 2
    /// 3
    pub fn new() -> Self {
        Self {
            start: None,
            end: None,
        }
    }

    /// Open ended range from start
    pub fn from_start(start: isize) -> Self {
        Self {
            start: Some(start),
            end: None,
        }
    }

    /// Open ended range from 0 to end
    pub fn from_end(end: isize) -> Self {
        Self {
            start: None,
            end: Some(end),
        }
    }

    /// Closed range from start (inclusive) to end (exclusive)
    pub fn from_both(start: isize, end: isize) -> Self {
        Self {
            start: Some(start),
            end: Some(end),
        }
    }

    /// True if start and end are both None
    pub fn is_empty(&self) -> bool {
        self.start.is_none() && self.end.is_none()
    }

    /// Return the values as a slice
    pub fn as_slice(&self, len: usize) -> (usize, usize) {
        let ilen = len as isize;
        let mut start = self.start.unwrap_or(0);
        let mut end = self.end.unwrap_or(ilen);

        // If start is negative, it is an inclusive reference from the end of the array
        if start < 0 {
            start += ilen;
        }

        // If end is negative, it is an exclusive reference from the end of the array
        if end < 0 {
            end += ilen;
        }

        if end > ilen {
            end = ilen
        }

        // At this point, start and end should never be negative
        assert!(start >= 0);
        assert!(end >= 0);
        (start as usize, end as usize)
    }
}

impl From<Option<RangeType>> for RangeType {
    fn from(r: Option<RangeType>) -> Self {
        match r {
            Some(range) => range,
            _ => Self::new(),
        }
    }
}

/// Iterate on an array with a jq style range
/// The `.[start:end]` syntax can be used to return a subarray of an
/// array or substring of a string. The array returned by `.[10:15]`
/// will be of length `5`, containing the elements from index `10` (inclusive)
/// to index `15` (exclusive). Either index may be negative (in which case
/// it counts backwards from the end of the array), or omitted (in which
/// case it refers to the start or end of the array).
pub fn from_range(array: &Vec<Value>, range: &RangeType) -> Result<Vec<Value>, JQError> {
    let len = array.len();
    let (start, end) = range.as_slice(len);
    // Inclusive start = 0 <= x < len
    // Exclusive end = 0 > x <= len
    // start < end

    //    if start < 0 || start >= len as usize {
    //        // Oops!  RangeType error!  Per JQ policy, return an empty array
    //        return Ok(Vec::new());
    //    }

    println!("RangeType {:?} - {:?}:{:?}", array.len(), &start, &end);
    let val = array[start as usize..end as usize].to_vec();
    Ok(val)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::Value;

    fn make_array(string: &str) -> Vec<Value> {
        let json: Value = serde_json::from_str(string).expect("Failed to parse json");
        json.as_array().unwrap().to_owned()
    }

    #[test]
    fn test_start_end() {
        let array = make_array(r#"["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"]"#);
        let len = array.len() as isize;

        // Test full range
        let range = RangeType::new();
        let result = from_range(&array, &range).expect("Failed");
        assert_eq!(&result, &array);

        // Test full range
        let range = RangeType::from_start(0);
        let result = from_range(&array, &range).expect("Failed");
        assert_eq!(&result, &array);

        // Test full range
        let range = RangeType::from_end(len);
        let result = from_range(&array, &range).expect("Failed");
        assert_eq!(&result, &array);

        let range = RangeType::from_both(0, len);
        let result = from_range(&array, &range).expect("Failed");
        assert_eq!(&result, &array);

        let range = RangeType::from_both(0, 1);
        let result = from_range(&array, &range).expect("Failed");
        let cmp_array = make_array(r#"["0"]"#);
        assert_eq!(&result, &cmp_array);
    }

    #[test]
    fn test_start_offset() {
        let array = make_array(r#"["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"]"#);

        let range = RangeType::from_start(-1);
        let result = from_range(&array, &range).expect("Failed");
        let cmp_array = make_array(r#"["9"]"#);
        assert_eq!(&result, &cmp_array);
    }

    #[test]
    fn test_end_offset() {
        let array = make_array(r#"["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"]"#);
        let range = RangeType::from_end(-1);
        let result = from_range(&array, &range).expect("Failed");
        let cmp_array = make_array(r#"["0", "1", "2", "3", "4", "5", "6", "7", "8"]"#);
        assert_eq!(&result, &cmp_array);
    }

    #[test]
    fn test_slice() {
        let array = make_array(r#"["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"]"#);

        let range = RangeType::from_both(0, 1);
        let result = from_range(&array, &range).expect("Oops");
        let cmp_array = make_array(r#"["0"]"#);
        assert_eq!(&result, &cmp_array);

        let range = RangeType::from_both(0, 0);
        let result = from_range(&array, &range).expect("Failed");
        assert!(result.is_empty());
    }

    #[test]
    fn test_out_of_bounds() {
        let array = make_array(r#"["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"]"#);

        let range = RangeType::from_start(array.len() as isize);
        let result = from_range(&array, &range).expect("Failed");
        assert!(result.is_empty());

        let range = RangeType::from_end(array.len() as isize + 1);
        let result = from_range(&array, &range).expect("Failed");
        assert_eq!(&result, &array);

        let range = RangeType::from_both(9, array.len() as isize + 1);
        let result = from_range(&array, &range).expect("Failed");
        let cmp_array = make_array(r#"["9"]"#);
        assert_eq!(&result, &cmp_array);
    }
}
