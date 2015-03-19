use {Assertion};

impl<T, T0> Assertion<T> for (T0,)
where T0: Assertion<T> {
    fn assert(self, target: &T) {
        self.0.assert(target);
    }
}

impl<T, T0, T1> Assertion<T> for (T0, T1)
where T0: Assertion<T>,
      T1: Assertion<T> {
    fn assert(self, target: &T) {
        self.0.assert(target);
        self.1.assert(target);
    }
}

impl<T, T0, T1, T2> Assertion<T> for (T0, T1, T2)
where T0: Assertion<T>,
      T1: Assertion<T>,
      T2: Assertion<T> {
    fn assert(self, target: &T) {
        self.0.assert(target);
        self.1.assert(target);
        self.2.assert(target);
    }
}

impl<T, T0, T1, T2, T3> Assertion<T> for (T0, T1, T2, T3)
where T0: Assertion<T>,
      T1: Assertion<T>,
      T2: Assertion<T>,
      T3: Assertion<T> {
    fn assert(self, target: &T) {
        self.0.assert(target);
        self.1.assert(target);
        self.2.assert(target);
        self.3.assert(target);
    }
}

