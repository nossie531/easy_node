use drop_tracer::prelude::*;
use easy_node::prelude::*;
use std::cmp::Ordering;
use std::collections::HashSet;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::rc::Rc;

#[test]
fn new() {
    // Act.
    let result = Nr::new(42);
    // Assert.
    assert_eq!(*result, 42);
    assert_eq!(Nr::strong_count(&result), 1);
    assert_eq!(Nr::weak_count(&result), 0);
}

#[test]
fn new_cyclic() {
    drop_test::run(|tracer| {
        // Act.
        let result = Nr::new_cyclic(|w| {
            let value = tracer.trace(42);
            let me = w.clone();
            Cyclic { value, me }
        });
        // Assert.
        assert_eq!(*result.value, 42);
        assert_eq!(&result.me, &Nr::downgrade(&result));
        assert_eq!(Nr::strong_count(&result), 1);
        assert_eq!(Nr::weak_count(&result), 1);
        assert_eq!(Nw::strong_count(&result.me), 1);
        assert_eq!(Nw::weak_count(&result.me), 1);
    });

    #[derive(Debug)]
    struct Cyclic<T> {
        value: T,
        me: Nw<Self>,
    }
}

#[test]
fn as_base() {
    // Arrange.
    let rc = Rc::new(42);
    // Act.
    let result = Nr::as_base(&rc);
    // Assert.
    assert_eq!(result, Nr::as_base(&rc));
}

#[test]
fn from_base() {
    // Arrange.
    let rc = Rc::new(42);
    // Act.
    let result = Nr::from_base(rc.clone());
    // Assert.
    assert_eq!(result, Nr::from_base(rc.clone()));
}

#[test]
fn as_ptr() {
    // Arrange.
    let target = Nr::new(42);
    // Act.
    let result = Nr::as_ptr(&target);
    // Assert.
    assert_eq!(result, Nr::as_ptr(&target.clone()));
}

#[test]
fn base() {
    // Arrange.
    let rc = Rc::new(42);
    let target = Nr::from_base(rc.clone());
    // Act.
    let result = Nr::base(&target);
    // Assert.
    assert!(Rc::ptr_eq(result, &rc));
}

#[test]
fn downgrade() {
    // Arrange.
    let target = Nr::new(42);
    // Act.
    let result = Nr::downgrade(&target);
    // Assert.
    assert_eq!(Nr::strong_count(&target), 1);
    assert_eq!(Nw::strong_count(&result), 1);
    assert_eq!(Nr::weak_count(&target), 1);
    assert_eq!(Nw::weak_count(&result), 1);
}

#[test]
fn clone() {
    // Arrange.
    let target = Nr::new(42);
    // Act.
    let result = Nr::clone(&target);
    // Assert.
    assert_eq!(Nr::strong_count(&target), 2);
    assert_eq!(Nr::strong_count(&result), 2);
    assert_eq!(&result, &target);
}

#[test]
fn default() {
    // Act.
    let result = <Nr<i32> as Default>::default();
    // Assert.
    assert_eq!(*result, i32::default());
}

#[test]
fn fmt() {
    // Arrange.
    let target = Nr::new("test");
    // Act.
    let result = format!("{target}");
    // Assert.
    assert_eq!(&result, "test")
}

#[test]
fn hash() {
    when_same();
    when_diff();

    fn when_same() {
        // Arrange.
        let target1 = Nr::new(());
        let target2 = Nr::clone(&target1);
        let mut hasher1 = DefaultHasher::new();
        let mut hasher2 = DefaultHasher::new();
        // Act.
        target1.hash(&mut hasher1);
        target2.hash(&mut hasher2);
        // Assert.
        assert_eq!(hasher1.finish(), hasher2.finish());
    }

    fn when_diff() {
        // Arrange.
        let target1 = Nr::new(());
        let target2 = Nr::new(());
        let mut hasher1 = DefaultHasher::new();
        let mut hasher2 = DefaultHasher::new();
        // Act.
        target1.hash(&mut hasher1);
        target2.hash(&mut hasher2);
        // Assert.
        assert_ne!(hasher1.finish(), hasher2.finish());
    }
}

#[test]
fn cmp() {
    when_same();
    when_diff();

    fn when_same() {
        // Arrange.
        let target1 = Nr::new(());
        let target2 = Nr::clone(&target1);
        // Act.
        let result = target1.cmp(&target2);
        // Assert.
        assert_eq!(result, Ordering::Equal);
    }

    fn when_diff() {
        // Arrange.
        let target1 = Nr::new(());
        let target2 = Nr::new(());
        // Act.
        let result1 = target1.cmp(&target2);
        let result2 = target2.cmp(&target1);
        let results = [result1, result2];
        // Assert.
        let expecteds = [Ordering::Less, Ordering::Greater];
        assert_eq!(HashSet::from(results), HashSet::from(expecteds));
    }
}

#[test]
fn eq() {
    when_same();
    when_diff();

    fn when_same() {
        // Arrange.
        let target1 = Nr::new(());
        let target2 = Nr::clone(&target1);
        // Act.
        let result = target1.eq(&target2);
        // Assert.
        assert!(result);
    }

    fn when_diff() {
        // Arrange.
        let target1 = Nr::new(());
        let target2 = Nr::new(());
        // Act.
        let result = target1.eq(&target2);
        // Assert.
        assert!(!result);
    }
}

#[test]
fn partial_cmp() {
    when_same();
    when_diff();

    fn when_same() {
        // Arrange.
        let target1 = Nr::new(());
        let target2 = Nr::clone(&target1);
        // Act.
        let result = target1.partial_cmp(&target2);
        // Assert.
        assert_eq!(result, Some(Ordering::Equal));
    }

    fn when_diff() {
        // Arrange.
        let target1 = Nr::new(());
        let target2 = Nr::new(());
        // Act.
        let result1 = target1.partial_cmp(&target2);
        let result2 = target2.partial_cmp(&target1);
        let results = [result1, result2];
        // Assert.
        let expecteds = [Some(Ordering::Less), Some(Ordering::Greater)];
        assert_eq!(HashSet::from(results), HashSet::from(expecteds));
    }
}
