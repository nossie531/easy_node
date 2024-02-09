mod common;

pub use crate::common::*;
use drop_tracer::DropTracer;
use easy_node::{Nr, Nw};
use std::cmp::Ordering;
use std::collections::hash_map::DefaultHasher;
use std::collections::HashSet;
use std::hash::{Hash, Hasher};

#[test]
fn new() {
    DropTracer::test_drop(|tracer| {
        let result = Nr::new((42, tracer.new_item()));

        assert_eq!(result.0, 42);
        assert_eq!(Nr::strong_count(&result), 1);
        assert_eq!(Nr::weak_count(&result), 0);
    });
}

#[test]
fn new_cyclic() {
    DropTracer::test_drop(|tracer| {
        let result = Nr::new_cyclic(|w| Cyclic::new((42, tracer.new_item()), w));

        assert_eq!(result.val().0, 42);
        assert_eq!(result.me(), &Nr::downgrade(&result));
        assert_eq!(Nr::strong_count(&result), 1);
        assert_eq!(Nr::weak_count(&result), 1);
        assert_eq!(Nw::strong_count(result.me()), 1);
        assert_eq!(Nw::weak_count(result.me()), 1);
    });
}

#[test]
fn downgrade() {
    with_normal();
    with_double();
    with_cyclic();

    fn with_normal() {
        DropTracer::test_drop(|tracer| {
            let target = Nr::new(tracer.new_item());

            let result = Nr::downgrade(&target);

            assert_eq!(Nr::strong_count(&target), 1);
            assert_eq!(Nw::strong_count(&result), 1);
            assert_eq!(Nr::weak_count(&target), 1);
            assert_eq!(Nw::weak_count(&result), 1);
        });
    }

    fn with_double() {
        DropTracer::test_drop(|tracer| {
            let target = Nr::new(tracer.new_item());

            let result1 = Nr::downgrade(&target);
            let result2 = Nr::downgrade(&target);

            assert_eq!(Nr::strong_count(&target), 1);
            assert_eq!(Nw::strong_count(&result1), 1);
            assert_eq!(Nw::strong_count(&result2), 1);
            assert_eq!(Nr::weak_count(&target), 2);
            assert_eq!(Nw::weak_count(&result1), 2);
            assert_eq!(Nw::weak_count(&result2), 2);
        });
    }

    fn with_cyclic() {
        DropTracer::test_drop(|tracer| {
            let target = Nr::new_cyclic(|w| Cyclic::new(tracer.new_item(), w));

            let result = Nr::downgrade(&target);

            assert_eq!(Nr::strong_count(&target), 1);
            assert_eq!(Nw::strong_count(&result), 1);
            assert_eq!(Nr::weak_count(&target), 2);
            assert_eq!(Nw::weak_count(&result), 2);
        });
    }
}

#[test]
fn clone() {
    let target = Nr::new(42);

    let result = Nr::clone(&target);

    assert_eq!(Nr::strong_count(&target), 2);
    assert_eq!(Nr::strong_count(&result), 2);
    assert_eq!(Nr::weak_count(&target), 0);
    assert_eq!(Nr::weak_count(&result), 0);
    assert_eq!(&result, &target);
    assert_eq!(*result, *target);
}

#[test]
fn drop() {
    with_weak();
    with_strong();
    with_cyclic();

    fn with_strong() {
        DropTracer::test_drop(|tracer| {
            let target = Nr::new(tracer.new_item());
            let strong = Nr::clone(&target);

            std::mem::drop(target);

            assert_eq!(Nr::strong_count(&strong), 1);
            assert_eq!(Nr::weak_count(&strong), 0);
        });
    }

    fn with_weak() {
        DropTracer::test_drop(|tracer| {
            let target = Nr::new(tracer.new_item());
            let nw = Nr::downgrade(&target);

            std::mem::drop(target);

            assert_eq!(Nw::strong_count(&nw), 0);
            assert_eq!(Nw::weak_count(&nw), 0);
        });
    }

    fn with_cyclic() {
        DropTracer::test_drop(|tracer| {
            let create = |w: &_| Cyclic::new(tracer.new_item(), w);
            let target = Nr::new_cyclic(create);
            let nw = Nr::downgrade(&target);

            std::mem::drop(target);

            assert_eq!(Nw::weak_count(&nw), 0);
            assert_eq!(Nw::strong_count(&nw), 0);
        });
    }
}

#[test]
fn default() {
    let result = <Nr<i32> as Default>::default();
    assert_eq!(*result, i32::default());
}

#[test]
fn fmt() {
    let target = Nr::new("test");
    let result = format!("{target}");
    assert_eq!(&result, "test")
}

#[test]
fn hash() {
    with_same();
    with_diff();

    fn with_same() {
        let target1 = Nr::new(());
        let target2 = Nr::clone(&target1);
        let mut hasher1 = DefaultHasher::new();
        let mut hasher2 = DefaultHasher::new();

        target1.hash(&mut hasher1);
        target2.hash(&mut hasher2);

        assert_eq!(hasher1.finish(), hasher2.finish());
    }

    fn with_diff() {
        let target1 = Nr::new(());
        let target2 = Nr::new(());
        let mut hasher1 = DefaultHasher::new();
        let mut hasher2 = DefaultHasher::new();

        target1.hash(&mut hasher1);
        target2.hash(&mut hasher2);

        assert_ne!(hasher1.finish(), hasher2.finish());
    }
}

#[test]
fn cmp() {
    with_same();
    with_diff();

    fn with_same() {
        let target1 = Nr::new(());
        let target2 = Nr::clone(&target1);

        let result = target1.cmp(&target2);

        assert_eq!(result, Ordering::Equal);
    }

    fn with_diff() {
        let target1 = Nr::new(());
        let target2 = Nr::new(());

        let result1 = target1.cmp(&target2);
        let result2 = target2.cmp(&target1);
        let results = [result1, result2];

        let expecteds = [Ordering::Less, Ordering::Greater];
        assert_eq!(HashSet::from(results), HashSet::from(expecteds));
    }
}

#[test]
fn eq() {
    with_same();
    with_diff();

    fn with_same() {
        let target1 = Nr::new(());
        let target2 = Nr::clone(&target1);

        let result = target1.eq(&target2);

        assert!(result);
    }

    fn with_diff() {
        let target1 = Nr::new(());
        let target2 = Nr::new(());

        let result = target1.eq(&target2);

        assert!(!result);
    }
}

#[test]
fn partial_cmp() {
    with_same();
    with_diff();

    fn with_same() {
        let target1 = Nr::new(());
        let target2 = Nr::clone(&target1);

        let result = target1.partial_cmp(&target2);

        assert_eq!(result, Some(Ordering::Equal));
    }

    fn with_diff() {
        let target1 = Nr::new(());
        let target2 = Nr::new(());

        let result1 = target1.partial_cmp(&target2);
        let result2 = target2.partial_cmp(&target1);
        let results = [result1, result2];

        let expecteds = [Some(Ordering::Less), Some(Ordering::Greater)];
        assert_eq!(HashSet::from(results), HashSet::from(expecteds));
    }
}