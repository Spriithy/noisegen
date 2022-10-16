pub trait MinMax<T>
where
    T: Copy,
{
    fn new() -> Self;
    fn from_vec(vec: &Vec<T>) -> Self;

    fn add(&mut self, value: T);

    fn min(self) -> Option<T>;
    fn max(self) -> Option<T>;
    fn min_max(self) -> (Option<T>, Option<T>);
}

pub struct MinMaxStore<T> {
    min: Option<T>,
    max: Option<T>,
}

impl<T> MinMax<T> for MinMaxStore<T>
where
    T: PartialOrd + Copy,
{
    fn new() -> Self {
        MinMaxStore {
            min: None,
            max: None,
        }
    }

    fn from_vec(vec: &Vec<T>) -> Self {
        let mut mmo = MinMaxStore::<T>::new();
        vec.iter().for_each(|&x| mmo.add(x));
        mmo
    }

    fn add(&mut self, value: T) {
        self.min = match &self.min {
            Some(t) => Some(if value < *t { value } else { *t }),
            None => Some(value),
        };

        self.max = match self.max {
            Some(t) => Some(if value > t { value } else { t }),
            None => Some(value),
        }
    }

    fn min(self) -> Option<T> {
        self.min
    }

    fn max(self) -> Option<T> {
        self.max
    }

    fn min_max(self) -> (Option<T>, Option<T>) {
        (self.min, self.max)
    }
}
