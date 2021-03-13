pub trait Synchronize: Sized + Eq {
    type Time;

    fn time(&self) -> Self::Time;
    fn synchronize(&self, time: &Self::Time) -> Option<Self>;

    fn can_synchronize(&self, time: &Self::Time) -> bool {
        self.synchronize(time).is_some()
    }

    fn equiv(&self, b: &Self) -> Option<bool> {
        if let Some(b2) = b.synchronize(&self.time()) {
            Some(self == &b2)
        } else if let Some(a2) = self.synchronize(&b.time()) {
            Some(&a2 == b)
        } else {
            None
        }
    }
}

pub trait CoreEquiv: Iterator + Sized
    where Self::Item: Synchronize
{
    fn core(&self) -> Self::Item;
    fn cosynchronizable() -> bool;
    fn synchronizable() -> bool;

    fn member(&self, a: &Self::Item) -> Option<bool> {self.core().equiv(a)}

    fn check(mut self) -> bool {
        let core = self.core();
        if Self::cosynchronizable() {
            let core_time = core.time();
            self.all(|a| {
                if let Some(a2) = a.synchronize(&core_time) {
                    core == a2
                } else {
                    false
                }
            })
        } else if Self::synchronizable() {
            self.all(|a| {
                if let Some(core2) = core.synchronize(&a.time()) {
                    core2 == a
                } else {
                    false
                }
            })
        } else {
            false
        }
    }
}
