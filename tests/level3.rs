#[test]
fn test_sgemm() {
    const LEN: usize = 513;
    let mut a = vec![0.5; LEN * LEN];
    let mut b = vec![0.5; LEN * LEN];
    let mut c = vec![0.0; LEN * LEN];
    let mut cref = vec![0.0; LEN * LEN];

    for i in 0..LEN {
        for j in 0..LEN {
            a[i + j * LEN] = i as f32;
            b[i + j * LEN] = j as f32 + i as f32;
        }
    }

    unsafe {
        blasoxide::sgemm(
            false,
            false,
            LEN,
            LEN,
            LEN,
            1.0,
            a.as_ptr(),
            LEN,
            b.as_ptr(),
            LEN,
            1.0,
            c.as_mut_ptr(),
            LEN,
        );
    }

    unsafe {
        sgemm_ref(
            false,
            false,
            LEN,
            LEN,
            LEN,
            1.0,
            a.as_ptr(),
            LEN,
            b.as_ptr(),
            LEN,
            1.0,
            cref.as_mut_ptr(),
            LEN,
        );
    }

    for i in 0..LEN {
        for j in 0..LEN {
            let (a, b) = (c[i + j * LEN], cref[i + j * LEN]);
            assert!((a - b).abs() < 1000.0, "a!=b, a={}, b={}", a, b);
        }
    }
}

unsafe fn sgemm_ref(
    _transa: bool,
    _transb: bool,
    m: usize,
    n: usize,
    k: usize,
    _alpha: f32,
    a: *const f32,
    lda: usize,
    b: *const f32,
    ldb: usize,
    _beta: f32,
    c: *mut f32,
    ldc: usize,
) {
    for j in 0..n {
        for i in 0..m {
            let mut ci = *c.add(i + j * ldc);
            for p in 0..k {
                ci += *a.add(i + p * lda) * *b.add(p + j * ldb);
            }
            *c.add(i + j * ldc) = ci;
        }
    }
}
