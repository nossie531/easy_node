use easy_node::prelude::*;
use std::cmp::Ordering;
use std::collections::HashSet;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::rc::Rc;

#[test]
fn new() {
    // Act.
    let result = Nw::<()>::new();
    // Assert.
    assert_eq!(result.strong_count(), 0);
    assert_eq!(result.weak_count(), 0);
}

#[test]
fn as_base() {
    // Arrange.
    let rc = Rc::new(42);
    let weak = Rc::downgrade(&rc);
    // Act.
    let result = Nw::as_base(&weak);
    // Assert.
    assert_eq!(result, Nw::as_base(&weak));
}

#[test]
fn from_base() {
    // Arrange.
    let rc = Rc::new(42);
    let weak = Rc::downgrade(&rc);
    // Act.
    let result = Nw::from_base(weak.clone());
    // Assert.
    assert_eq!(result, Nw::from_base(weak.clone()));
}

#[test]
fn as_ptr() {
    with_normal();
    with_dangling();

    fn with_normal() {
        // Arrange.
        let nr = Nr::new(42);
        let target = Nr::downgrade(&nr);
        // Act.
        let result = target.as_ptr();
        // Assert.
        assert_eq!(result, Nr::as_ptr(&nr));
    }

    fn with_dangling() {
        // Arrange.
        let nr = Nr::new(42);
        let target = Nr::downgrade(&nr);
        // Act and assert.
        let _dangling = target.as_ptr();
    }
}

#[test]
fn base() {
    // Arrange.
    let rc = Rc::new(42);
    let weak = Rc::downgrade(&rc);
    let target = Nw::from_base(weak.clone());
    // Act.
    let result = target.base();
    // Assert.
    assert!(result.ptr_eq(&weak));
}

#[test]
fn upgrade() {
    with_empty();
    with_droped_nr();
    with_normal();

    fn with_empty() {
        // Arrange.
        let target = Nw::<()>::new();
        // Act.
        let result = target.upgrade();
        // Assert.
        assert_eq!(result, None);
    }

    fn with_droped_nr() {
        // Arragne.
        let nr = Nr::new(42);
        let target = Nr::downgrade(&nr);
        std::mem::drop(nr);
        // Act.
        let result = target.upgrade();
        // Assert.
        assert_eq!(result, None);
    }

    fn with_normal() {
        // Arrange.
        let nr = Nr::new(42);
        let target = Nr::downgrade(&nr);
        // Act.
        let result = target.upgrade();
        drop(nr);
        // Assert.
        assert_eq!(Nw::strong_count(&target), 1);
        assert_eq!(Nr::strong_count(&result.as_ref().unwrap()), 1);
        assert_eq!(Nw::weak_count(&target), 1);
        assert_eq!(Nr::weak_count(&result.as_ref().unwrap()), 1);
    }
}

#[test]
fn clone() {
    with_empty();
    with_normal();

    fn with_empty() {
        // Arrange.
        let target = Nw::<()>::new();
        // Act.
        let result = Nw::clone(&target);
        // Assert.
        assert_eq!(Nw::strong_count(&target), 0);
        assert_eq!(Nw::strong_count(&result), 0);
        assert_eq!(Nw::weak_count(&target), 0);
        assert_eq!(Nw::weak_count(&result), 0);
        assert_eq!(&result, &target);
    }

    fn with_normal() {
        // Arrange.
        let nr = Nr::new(());
        let target = Nr::downgrade(&nr);
        // Act.
        let result = Nw::clone(&target);
        // Assert.
        assert_eq!(Nr::strong_count(&nr), 1);
        assert_eq!(Nw::strong_count(&target), 1);
        assert_eq!(Nw::strong_count(&result), 1);
        assert_eq!(Nr::weak_count(&nr), 2);
        assert_eq!(Nw::weak_count(&target), 2);
        assert_eq!(Nw::weak_count(&result), 2);
        assert_eq!(&result, &target);
    }
}

#[test]
fn default() {
    let result = <Nw<()> as Default>::default();
    assert_eq!(result, Nw::new());
}

#[test]
fn hash() {
    with_empty();
    with_same();
    with_diff();

    fn with_empty() {
        // Arrange.
        let target1 = Nw::<()>::new();
        let target2 = Nw::<()>::new();
        let mut hasher1 = DefaultHasher::new();
        let mut hasher2 = DefaultHasher::new();
        // Act.
        target1.hash(&mut hasher1);
        target2.hash(&mut hasher2);
        // Assert.
        assert_eq!(hasher1.finish(), hasher2.finish());
    }

    fn with_same() {
        // Arrange.
        let nr = Nr::new(());
        let target1 = Nr::downgrade(&nr);
        let target2 = Nr::downgrade(&nr);
        let mut hasher1 = DefaultHasher::new();
        let mut hasher2 = DefaultHasher::new();
        // Act.
        target1.hash(&mut hasher1);
        target2.hash(&mut hasher2);
        // Assert.
        assert_eq!(hasher1.finish(), hasher2.finish());
    }

    fn with_diff() {
        // Arrange.
        let rc1 = Nr::new(());
        let rc2 = Nr::new(());
        let target1 = Nr::downgrade(&rc1);
        let target2 = Nr::downgrade(&rc2);
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
    with_empty();
    with_same();
    with_diff();

    fn with_empty() {
        // Arrange.
        let target1 = Nw::<()>::new();
        let target2 = Nw::<()>::new();
        // Act.
        let result = target1.cmp(&target2);
        // Assert.
        assert_eq!(result, Ordering::Equal);
    }

    fn with_same() {
        // Arrange.
        let nr = Nr::new(());
        let target1 = Nr::downgrade(&nr);
        let target2 = Nr::downgrade(&nr);
        // Act.
        let result = target1.cmp(&target2);
        // Assert.
        assert_eq!(result, Ordering::Equal);
    }

    fn with_diff() {
        // Arrange.
        let rc1 = Nr::new(());
        let rc2 = Nr::new(());
        let target1 = Nr::downgrade(&rc1);
        let target2 = Nr::downgrade(&rc2);
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
    with_empty();
    with_same();
    with_diff();

    fn with_empty() {
        // Arrange.
        let target1 = Nw::<()>::new();
        let target2 = Nw::<()>::new();
        // Act.
        let result = target1.eq(&target2);
        // Assert.
        assert!(result);
    }

    fn with_same() {
        // Arrange.
        let nr = Nr::new(());
        let target1 = Nr::downgrade(&nr);
        let target2 = Nr::downgrade(&nr);
        // Act.
        let result = target1.eq(&target2);
        // Assert.
        assert!(result);
    }

    fn with_diff() {
        // Arrange.
        let rc1 = Nr::new(());
        let rc2 = Nr::new(());
        let target1 = Nr::downgrade(&rc1);
        let target2 = Nr::downgrade(&rc2);
        // Act.
        let result = target1.eq(&target2);
        // Assert.
        assert!(!result);
    }
}

#[test]
fn partial_cmp() {
    with_empty();
    with_same();
    with_diff();

    fn with_empty() {
        // Arrange.
        let target1 = Nw::<()>::new();
        let target2 = Nw::<()>::new();
        // Act.
        let result = target1.partial_cmp(&target2);
        // Assert.
        assert_eq!(result, Some(Ordering::Equal));
    }

    fn with_same() {
        // Arrange.
        let nr = Nr::new(());
        let target1 = Nr::downgrade(&nr);
        let target2 = Nr::downgrade(&nr);
        // Act.
        let result = target1.partial_cmp(&target2);
        // Assert.
        assert_eq!(result, Some(Ordering::Equal));
    }

    fn with_diff() {
        // Arrange.
        let rc1 = Nr::new(());
        let rc2 = Nr::new(());
        let target1 = Nr::downgrade(&rc1);
        let target2 = Nr::downgrade(&rc2);
        // Act.
        let result1 = target1.partial_cmp(&target2);
        let result2 = target2.partial_cmp(&target1);
        let results = [result1, result2];
        // Assert.
        let expecteds = [Some(Ordering::Less), Some(Ordering::Greater)];
        assert_eq!(HashSet::from(results), HashSet::from(expecteds));
    }
}
