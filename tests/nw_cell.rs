mod common;

pub use crate::common::*;
use drop_tracer::DropTracer;
use easy_node::{NrCell, NwCell};
use std::cmp::Ordering;
use std::collections::{hash_map::DefaultHasher, HashSet};
use std::hash::{Hash, Hasher};

#[test]
fn new() {
    let result = NwCell::<()>::new();
    assert_eq!(result.strong_count(), 0);
    assert_eq!(result.weak_count(), 0);
}

#[test]
fn upgrade() {
    with_empty();
    with_droped();
    with_single();
    with_double();
    with_self_cycle();
    with_parent_and_child_cycle();

    fn with_empty() {
        let target = NwCell::<()>::new();
        let result = target.upgrade();
        assert_eq!(result, None);
    }

    fn with_droped() {
        DropTracer::test_drop(|tracer| {
            let nr = NrCell::new(tracer.new_item());
            let target = NrCell::downgrade(&nr);
            std::mem::drop(nr);

            let result = target.upgrade();

            assert_eq!(result, None);
        });
    }

    fn with_single() {
        DropTracer::test_drop(|tracer| {
            let nr = NrCell::new(tracer.new_item());
            let target = NrCell::downgrade(&nr);

            let result = target.upgrade();

            assert_eq!(result, Some(nr));
        });
    }

    fn with_double() {
        DropTracer::test_drop(|tracer| {
            let nr = NrCell::new(tracer.new_item());
            let target1 = NrCell::downgrade(&nr);
            let target2 = NrCell::downgrade(&nr);

            let result1 = target1.upgrade();
            let result2 = target2.upgrade();

            assert_eq!(result1, Some(nr.clone()));
            assert_eq!(result2, Some(nr.clone()));
        });
    }

    fn with_self_cycle() {
        DropTracer::test_drop(|tracer| {
            let nr = NrCell::new_cyclic(|w| {
                let value = tracer.new_item();
                CyclicCell::new(value, w)
            });

            let target = NrCell::downgrade(&nr);

            let result = target.upgrade();

            assert_eq!(result, Some(nr));
        });
    }

    fn with_parent_and_child_cycle() {
        DropTracer::test_drop(|tracer| {
            let nr = NrCell::new_cyclic(|w| {
                let c_value = tracer.new_item();
                let p_value = tracer.new_item();
                let child = NrCell::new(ChildCell::new(c_value, w.clone()));
                let parent = ParentCell::new(p_value, child);
                parent
            });

            let target = NrCell::downgrade(&nr);

            let result = target.upgrade_ref();

            assert_eq!(result, Some(&nr));
        });
    }
}

#[test]
fn upgrade_ref() {
    with_empty();
    with_droped();
    with_single();
    with_double();
    with_self_cycle();
    with_parent_and_child_cycle();

    fn with_empty() {
        let target = NwCell::<()>::new();
        let result = target.upgrade_ref();
        assert_eq!(result, None);
    }

    fn with_droped() {
        DropTracer::test_drop(|tracer| {
            let nr = NrCell::new(tracer.new_item());
            let target = NrCell::downgrade(&nr);
            std::mem::drop(nr);

            let result = target.upgrade_ref();

            assert_eq!(result, None);
        });
    }

    fn with_single() {
        DropTracer::test_drop(|tracer| {
            let nr = NrCell::new(tracer.new_item());
            let target = NrCell::downgrade(&nr);

            let result = target.upgrade_ref();

            assert_eq!(result, Some(&nr));
        });
    }

    fn with_double() {
        DropTracer::test_drop(|tracer| {
            let nr = NrCell::new(tracer.new_item());
            let target1 = NrCell::downgrade(&nr);
            let target2 = NrCell::downgrade(&nr);

            let result1 = target1.upgrade_ref();
            let result2 = target2.upgrade_ref();

            assert_eq!(result1, Some(&nr));
            assert_eq!(result2, Some(&nr));
        });
    }

    fn with_self_cycle() {
        DropTracer::test_drop(|tracer| {
            let nr = NrCell::new_cyclic(|w| {
                let value = tracer.new_item();
                CyclicCell::new(value, w)
            });

            let target = NrCell::downgrade(&nr);

            let result = target.upgrade_ref();

            assert_eq!(result, Some(&nr));
        });
    }

    fn with_parent_and_child_cycle() {
        DropTracer::test_drop(|tracer| {
            let nr = NrCell::new_cyclic(|w| {
                let c_value = tracer.new_item();
                let p_value = tracer.new_item();
                let child = NrCell::new(ChildCell::new(c_value, w.clone()));
                let parent = ParentCell::new(p_value, child);
                parent
            });

            let target = NrCell::downgrade(&nr);

            let result = target.upgrade_ref();

            assert_eq!(result, Some(&nr));
        });
    }
}

#[test]
fn clone() {
    with_empty();
    with_normal();

    fn with_empty() {
        let target = NwCell::<()>::new();

        let result = NwCell::clone(&target);

        assert_eq!(NwCell::strong_count(&target), 0);
        assert_eq!(NwCell::strong_count(&result), 0);
        assert_eq!(NwCell::weak_count(&target), 0);
        assert_eq!(NwCell::weak_count(&result), 0);
        assert_eq!(&result, &target);
    }

    fn with_normal() {
        let nr = NrCell::new(());
        let target = NrCell::downgrade(&nr);

        let result = NwCell::clone(&target);

        assert_eq!(NrCell::strong_count(&nr), 1);
        assert_eq!(NwCell::strong_count(&target), 1);
        assert_eq!(NwCell::strong_count(&result), 1);
        assert_eq!(NrCell::weak_count(&nr), 2);
        assert_eq!(NwCell::weak_count(&target), 2);
        assert_eq!(NwCell::weak_count(&result), 2);
        assert_eq!(&result, &target);
    }
}

#[test]
fn default() {
    let result = <NwCell<()> as Default>::default();
    assert_eq!(result, NwCell::new());
}

#[test]
fn drop() {
    with_empty();
    with_noref();
    with_strong();
    with_weak();

    fn with_empty() {
        let target = NwCell::<()>::new();
        std::mem::drop(target);
    }

    fn with_noref() {
        DropTracer::test_drop(|tracer| {
            let nr = NrCell::new(tracer.new_item());
            let target = NrCell::downgrade(&nr);

            std::mem::drop(nr);
            std::mem::drop(target);
        });
    }

    fn with_strong() {
        DropTracer::test_drop(|tracer| {
            let nr = NrCell::new(tracer.new_item());
            let target = NrCell::downgrade(&nr);

            std::mem::drop(target);

            assert_eq!(NrCell::strong_count(&nr), 1);
            assert_eq!(NrCell::weak_count(&nr), 0);
        });
    }

    fn with_weak() {
        DropTracer::test_drop(|tracer| {
            let nr = NrCell::new(tracer.new_item());
            let target = NrCell::downgrade(&nr);
            let nw = NrCell::downgrade(&nr);

            std::mem::drop(target);

            assert_eq!(NrCell::strong_count(&nr), 1);
            assert_eq!(NwCell::strong_count(&nw), 1);
            assert_eq!(NrCell::weak_count(&nr), 1);
            assert_eq!(NwCell::weak_count(&nw), 1);
        });
    }
}

#[test]
fn hash() {
    with_empty();
    with_same();
    with_diff();

    fn with_empty() {
        let target1 = NwCell::<()>::new();
        let target2 = NwCell::<()>::new();
        let mut hasher1 = DefaultHasher::new();
        let mut hasher2 = DefaultHasher::new();

        target1.hash(&mut hasher1);
        target2.hash(&mut hasher2);

        assert_eq!(hasher1.finish(), hasher2.finish());
    }

    fn with_same() {
        let nr = NrCell::new(());
        let target1 = NrCell::downgrade(&nr);
        let target2 = NrCell::downgrade(&nr);
        let mut hasher1 = DefaultHasher::new();
        let mut hasher2 = DefaultHasher::new();

        target1.hash(&mut hasher1);
        target2.hash(&mut hasher2);

        assert_eq!(hasher1.finish(), hasher2.finish());
    }

    fn with_diff() {
        let rc1 = NrCell::new(());
        let rc2 = NrCell::new(());
        let target1 = NrCell::downgrade(&rc1);
        let target2 = NrCell::downgrade(&rc2);
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
        let target1 = NwCell::<()>::new();
        let target2 = NwCell::<()>::new();

        let result = target1.cmp(&target2);

        assert_eq!(result, Ordering::Equal);
    }

    fn with_same() {
        let nr = NrCell::new(());
        let target1 = NrCell::downgrade(&nr);
        let target2 = NrCell::downgrade(&nr);

        let result = target1.cmp(&target2);

        assert_eq!(result, Ordering::Equal);
    }

    fn with_diff() {
        let rc1 = NrCell::new(());
        let rc2 = NrCell::new(());
        let target1 = NrCell::downgrade(&rc1);
        let target2 = NrCell::downgrade(&rc2);

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
        let target1 = NwCell::<()>::new();
        let target2 = NwCell::<()>::new();

        let result = target1.eq(&target2);

        assert!(result);
    }

    fn with_same() {
        let nr = NrCell::new(());
        let target1 = NrCell::downgrade(&nr);
        let target2 = NrCell::downgrade(&nr);

        let result = target1.eq(&target2);

        assert!(result);
    }

    fn with_diff() {
        let rc1 = NrCell::new(());
        let rc2 = NrCell::new(());
        let target1 = NrCell::downgrade(&rc1);
        let target2 = NrCell::downgrade(&rc2);

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
        let target1 = NwCell::<()>::new();
        let target2 = NwCell::<()>::new();

        let result = target1.partial_cmp(&target2);

        assert_eq!(result, Some(Ordering::Equal));
    }

    fn with_same() {
        let nr = NrCell::new(());
        let target1 = NrCell::downgrade(&nr);
        let target2 = NrCell::downgrade(&nr);

        let result = target1.partial_cmp(&target2);

        assert_eq!(result, Some(Ordering::Equal));
    }

    fn with_diff() {
        let rc1 = NrCell::new(());
        let rc2 = NrCell::new(());
        let target1 = NrCell::downgrade(&rc1);
        let target2 = NrCell::downgrade(&rc2);

        let result1 = target1.partial_cmp(&target2);
        let result2 = target2.partial_cmp(&target1);
        let results = [result1, result2];

        let expecteds = [Some(Ordering::Less), Some(Ordering::Greater)];
        assert_eq!(HashSet::from(results), HashSet::from(expecteds));
    }
}
