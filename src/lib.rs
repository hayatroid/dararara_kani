#![cfg_attr(kani, feature(stmt_expr_attributes, proc_macro_hygiene))]

pub fn meguru(mut ok: i64, mut ng: i64, pred: impl Fn(i64) -> bool) -> i64 {
    #[cfg_attr(kani, kani::loop_invariant(pred(ok) && !pred(ng)))]
    // Unchecked in Kani 0.68.0 (model-checking/kani#4942); once fixed, always fails (model-checking/kani#4585).
    // #[cfg_attr(kani, kani::loop_decreases(ok.abs_diff(ng)))]
    while ok.abs_diff(ng) > 1 {
        #[cfg(kani)]
        let d = ok.abs_diff(ng);
        let mid = ok.midpoint(ng);
        if pred(mid) {
            ok = mid;
        } else {
            ng = mid;
        }
        #[cfg(kani)]
        assert!(ok.abs_diff(ng) < d);
    }
    ok
}

#[test]
fn test_meguru() {
    assert_eq!(meguru(0, 100, |x| x * x <= 50), 7);
}

#[cfg(kani)]
#[kani::proof]
fn check_meguru() {
    let (ok, ng, t) = kani::any();
    let pred = |x| if ok > ng { x >= t } else { x <= t };
    kani::assume(pred(ok) && !pred(ng));
    assert_eq!(meguru(ok, ng, pred), t);
}
