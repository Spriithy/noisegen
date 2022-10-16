pub trait MinMax<T>
where
    T: PartialOrd + Copy,
{
    fn min_max(&self) -> Option<(T, T)>;
}

impl<T> MinMax<T> for &[T]
where
    T: PartialOrd + Copy,
{
    fn min_max(&self) -> Option<(T, T)> {
        let mut res: Option<(T, T)> = None;

        for &x in *self {
            match res {
                Some((min, max)) => {
                    if x < min {
                        res = Some((x, max))
                    } else if x > max {
                        res = Some((min, x))
                    }
                }
                None => res = Some((x, x)),
            }
        }

        res
    }
}

impl<T> MinMax<T> for Vec<T>
where
    T: PartialOrd + Copy,
{
    fn min_max(&self) -> Option<(T, T)> {
        (&self as &[T]).min_max()
    }
}
