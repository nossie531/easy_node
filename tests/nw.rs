use easy_node::prelude::*;
use std::cmp::Ordering;
use std::collections::HashSet;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::rc::Rc;

#[test]
fn new() {
    let result = Nw::<()>::new();
    assert_eq!(result.strong_count(), 0);
    assert_eq!(result.weak_count(), 0);
}

#[test]
fn as_base() {
    let rc = Rc::new(42);
    let weak = Rc::downgrade(&rc);

    let result = Nw::as_base(&weak);

    assert_eq!(result, Nw::as_base(&weak));
}

#[test]
fn from_base() {
    let rc = Rc::new(42);
    let weak = Rc::downgrade(&rc);

    let result = Nw::from_base(weak.clone());

    assert_eq!(result, Nw::from_base(weak.clone()));
}

#[test]
fn base() {
    let rc = Rc::new(42);
    let weak = Rc::downgrade(&rc);
    let target = Nw::from_base(weak.clone());

    let result = target.base();

    assert!(result.ptr_eq(&weak));
}

#[test]
fn upgrade() {
    with_empty();
    with_droped_nr();
    with_normal();

    fn with_empty() {
        let target = Nw::<()>::new();
        let result = target.upgrade();
        assert_eq!(result, None);
    }

    fn with_droped_nr() {
        let nr = Nr::new(42);
        let target = Nr::downgrade(&nr);
        std::mem::drop(nr);

        let result = target.upgrade();

        assert_eq!(result, None);
    }

    fn with_normal() {
        let nr = Nr::new(42);
        let target = Nr::downgrade(&nr);

        let result = target.upgrade();
        drop(nr);

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
        let target = Nw::<()>::new();

        let result = Nw::clone(&target);

        assert_eq!(Nw::strong_count(&target), 0);
        assert_eq!(Nw::strong_count(&result), 0);
        assert_eq!(Nw::weak_count(&target), 0);
        assert_eq!(Nw::weak_count(&result), 0);
        assert_eq!(&result, &target);
    }

    fn with_normal() {
        let nr = Nr::new(());
        let target = Nr::downgrade(&nr);

        let result = Nw::clone(&target);

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
        let target1 = Nw::<()>::new();
        let target2 = Nw::<()>::new();
        let mut hasher1 = DefaultHasher::new();
        let mut hasher2 = DefaultHasher::new();

        target1.hash(&mut hasher1);
        target2.hash(&mut hasher2);

        assert_eq!(hasher1.finish(), hasher2.finish());
    }

    fn with_same() {
        let nr = Nr::new(());
        let target1 = Nr::downgrade(&nr);
        let target2 = Nr::downgrade(&nr);
        let mut hasher1 = DefaultHasher::new();
        let mut hasher2 = DefaultHasher::new();

        target1.hash(&mut hasher1);
        target2.hash(&mut hasher2);

        assert_eq!(hasher1.finish(), hasher2.finish());
    }

    fn with_diff() {
        let rc1 = Nr::new(());
        let rc2 = Nr::new(());
        let target1 = Nr::downgrade(&rc1);
        let target2 = Nr::downgrade(&rc2);
        let mut hasher1 = DefaultHasher::new();
        let mut hasher2 = DefaultHasher::new();

        target1.hash(&mut hasher1);
        target2.hash(&mut hasher2);

        assert_ne!(hasher1.finish(), hasher2.finish());
    }
}

#[test]
fn cmp() {
    with_empty();
    with_same();
    with_diff();

    fn with_empty() {
        let target1 = Nw::<()>::new();
        let target2 = Nw::<()>::new();

        let result = target1.cmp(&target2);

        assert_eq!(result, Ordering::Equal);
    }

    fn with_same() {
        let nr = Nr::new(());
        let target1 = Nr::downgrade(&nr);
        let target2 = Nr::downgrade(&nr);

        let result = target1.cmp(&target2);

        assert_eq!(result, Ordering::Equal);
    }

    fn with_diff() {
        let rc1 = Nr::new(());
        let rc2 = Nr::new(());
        let target1 = Nr::downgrade(&rc1);
        let target2 = Nr::downgrade(&rc2);

        let result1 = target1.cmp(&target2);
        let result2 = target2.cmp(&target1);
        let results = [result1, result2];

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
        let target1 = Nw::<()>::new();
        let target2 = Nw::<()>::new();

        let result = target1.eq(&target2);

        assert!(result);
    }

    fn with_same() {
        let nr = Nr::new(());
        let target1 = Nr::downgrade(&nr);
        let target2 = Nr::downgrade(&nr);

        let result = target1.eq(&target2);

        assert!(result);
    }

    fn with_diff() {
        let rc1 = Nr::new(());
        let rc2 = Nr::new(());
        let target1 = Nr::downgrade(&rc1);
        let target2 = Nr::downgrade(&rc2);

        let result = target1.eq(&target2);

        assert!(!result);
    }
}

#[test]
fn partial_cmp() {
    with_empty();
    with_same();
    with_diff();

    fn with_empty() {
        let target1 = Nw::<()>::new();
        let target2 = Nw::<()>::new();

        let result = target1.partial_cmp(&target2);

        assert_eq!(result, Some(Ordering::Equal));
    }

    fn with_same() {
        let nr = Nr::new(());
        let target1 = Nr::downgrade(&nr);
        let target2 = Nr::downgrade(&nr);

        let result = target1.partial_cmp(&target2);

        assert_eq!(result, Some(Ordering::Equal));
    }

    fn with_diff() {
        let rc1 = Nr::new(());
        let rc2 = Nr::new(());
        let target1 = Nr::downgrade(&rc1);
        let target2 = Nr::downgrade(&rc2);

        let result1 = target1.partial_cmp(&target2);
        let result2 = target2.partial_cmp(&target1);
        let results = [result1, result2];

        let expecteds = [Some(Ordering::Less), Some(Ordering::Greater)];
        assert_eq!(HashSet::from(results), HashSet::from(expecteds));
    }
}
