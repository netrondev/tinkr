pub trait ArrayUtils<T> {
    fn unique<F>(self, comparator: F) -> Vec<T>
    where
        F: FnMut(&T, &T) -> bool;
}

impl<T> ArrayUtils<T> for Vec<T> {
    fn unique<F>(self, mut comparator: F) -> Vec<T>
    where
        F: FnMut(&T, &T) -> bool,
    {
        let mut result = Vec::new();

        for item in self {
            if !result.iter().any(|existing| comparator(existing, &item)) {
                result.push(item);
            }
        }

        result
    }
}
