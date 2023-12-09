pub enum BoxOrRef<'a, T> {
    Box(Box<T>),
    Ref(&'a T)
}

impl<'a, T: Clone> BoxOrRef<'a, T> {
    pub fn boxed(t: T) -> BoxOrRef<'a, T> {
        return BoxOrRef::Box(Box::new(t));
    }

    pub fn from_ref(r: &'a T) -> BoxOrRef<'a, T> {
        return BoxOrRef::Ref(r);
    }

    pub fn into_owned(self) -> T {
        match self {
            BoxOrRef::Box(b) => *b,
            BoxOrRef::Ref(r) => r.clone()
        }
    }
}


impl<'a, T> AsRef<T> for BoxOrRef<'a, T> {
    fn as_ref(&self) -> &T {
        match self {
            BoxOrRef::Box(b) => &b,
            BoxOrRef::Ref(r) => *r,
        }
    }
}