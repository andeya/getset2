use getset2::Getset2;

#[derive(Default, Getset2)]
#[getset2(get_ref, set_with)]
pub struct Foo<T>
where
    T: Copy + Clone + Default,
{
    /// Doc comments are supported!
    /// Multiline, even.
    #[getset2(set, get_mut, skip(get_ref))]
    private: T,

    /// Doc comments are supported!
    /// Multiline, even.
    #[getset2(
        get_copy(pub, const),
        set(pub = "crate"),
        get_mut(pub = "super"),
        set_with(pub = "self", const)
    )]
    public: T,

    #[getset2(skip)]
    skip: (),
}

impl<T: Copy + Clone + Default> Foo<T> {
    fn private(&self) -> &T {
        &self.private
    }
    fn skip(&self) {
        self.skip
    }
}

// cargo expand --example simple

fn main() {
    let mut foo = Foo::default();
    foo.set_private(1);
    (*foo.private_mut()) += 1;
    assert_eq!(*foo.private(), 2);
    foo = foo.with_private(3);
    assert_eq!(*foo.private(), 3);
    foo.set_public(3);
    assert_eq!(foo.public(), 3);
    assert_eq!(foo.skip(), ());
}
