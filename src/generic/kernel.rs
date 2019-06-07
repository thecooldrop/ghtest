pub unsafe fn sgemm_16x4_packed(
    k: usize,
    a: *const f32,
    b: *const f32,
    beta: f32,
    c: *mut f32,
    ldc: usize,
) {
    #[cfg(target_arch = "x86_64")]
    {
        if is_x86_feature_detected!("fma") {
            crate::fma::sgemm_16x4_packed(k, a, b, beta, c, ldc);
            return;
        }
    }

    let cptr0 = c;
    let cptr1 = c.add(ldc);
    let cptr2 = c.add(ldc * 2);
    let cptr3 = c.add(ldc * 3);

    for i in 0..16 {
        let mut creg0 = *cptr0.add(i) * beta;
        let mut creg1 = *cptr1.add(i) * beta;
        let mut creg2 = *cptr2.add(i) * beta;
        let mut creg3 = *cptr3.add(i) * beta;

        let mut bptr = b;

        for p in 0..k {
            let areg = *a.add(i + p * 16);

            let breg0 = *bptr;
            let breg1 = *bptr.add(1);
            let breg2 = *bptr.add(2);
            let breg3 = *bptr.add(3);

            creg0 += breg0 * areg;
            creg1 += breg1 * areg;
            creg2 += breg2 * areg;
            creg3 += breg3 * areg;

            bptr = bptr.add(4);
        }

        *cptr0.add(i) = creg0;
        *cptr1.add(i) = creg1;
        *cptr2.add(i) = creg2;
        *cptr3.add(i) = creg3;
    }
}

pub unsafe fn s_pack_a(
    k: usize,
    alpha: f32,
    mut a: *const f32,
    lda: usize,
    mut packed_a: *mut f32,
) {
    #[cfg(target_arch = "x86_64")]
    {
        if is_x86_feature_detected!("fma") {
            crate::fma::s_pack_a(k, alpha, a, lda, packed_a);
            return;
        }
    }

    for _ in 0..k {
        let mut aptr = a;

        for _ in 0..16 {
            *packed_a = alpha * *aptr;

            aptr = aptr.add(1);
            packed_a = packed_a.add(1);
        }

        a = a.add(lda);
    }
}

pub unsafe fn dgemm_8x4_packed(
    k: usize,
    a: *const f64,
    b: *const f64,
    beta: f64,
    c: *mut f64,
    ldc: usize,
) {
    #[cfg(target_arch = "x86_64")]
    {
        if is_x86_feature_detected!("fma") {
            crate::fma::dgemm_8x4_packed(k, a, b, beta, c, ldc);
            return;
        }
    }

    let cptr0 = c;
    let cptr1 = c.add(ldc);
    let cptr2 = c.add(ldc * 2);
    let cptr3 = c.add(ldc * 3);

    for i in 0..8 {
        let mut creg0 = *cptr0.add(i) * beta;
        let mut creg1 = *cptr1.add(i) * beta;
        let mut creg2 = *cptr2.add(i) * beta;
        let mut creg3 = *cptr3.add(i) * beta;

        let mut bptr = b;

        for p in 0..k {
            let areg = *a.add(i + p * 8);

            let breg0 = *bptr;
            let breg1 = *bptr.add(1);
            let breg2 = *bptr.add(2);
            let breg3 = *bptr.add(3);

            creg0 += breg0 * areg;
            creg1 += breg1 * areg;
            creg2 += breg2 * areg;
            creg3 += breg3 * areg;

            bptr = bptr.add(4);
        }

        *cptr0.add(i) = creg0;
        *cptr1.add(i) = creg1;
        *cptr2.add(i) = creg2;
        *cptr3.add(i) = creg3;
    }
}

pub unsafe fn d_pack_a(
    k: usize,
    alpha: f64,
    mut a: *const f64,
    lda: usize,
    mut packed_a: *mut f64,
) {
    #[cfg(target_arch = "x86_64")]
    {
        if is_x86_feature_detected!("fma") {
            crate::fma::d_pack_a(k, alpha, a, lda, packed_a);
            return;
        }
    }

    for _ in 0..k {
        let mut aptr = a;

        for _ in 0..8 {
            *packed_a = alpha * *aptr;

            aptr = aptr.add(1);
            packed_a = packed_a.add(1);
        }

        a = a.add(lda);
    }
}

#[target_feature(enable = "fma")]
pub unsafe fn scombine_4(
    m: usize,
    alpha: f32,
    a: *const f32,
    lda: usize,
    x: *const f32,
    incx: usize,
    beta: f32,
    y: *mut f32,
) {
    #[cfg(target_arch = "x86_64")]
    {
        if is_x86_feature_detected!("fma") {
            crate::fma::scombine_4(m, alpha, a, lda, x, incx, beta, y);
            return;
        }
    }

    let mut aptr0 = a;
    let mut aptr1 = a.add(lda);
    let mut aptr2 = a.add(lda * 2);
    let mut aptr3 = a.add(lda * 3);

    let xreg0 = *x * alpha;
    let xreg1 = *x.add(incx) * alpha;
    let xreg2 = *x.add(2 * incx) * alpha;
    let xreg3 = *x.add(3 * incx) * alpha;

    let mut yptr = y;

    for _ in 0..m {
        *yptr = beta * *yptr + *aptr0 * xreg0 + *aptr1 * xreg1 + *aptr2 * xreg2 + *aptr3 * xreg3;
        yptr = yptr.add(1);
        aptr0 = aptr0.add(1);
        aptr1 = aptr1.add(1);
        aptr2 = aptr2.add(1);
        aptr3 = aptr3.add(1);
    }
}

#[target_feature(enable = "fma")]
pub unsafe fn dcombine_4(
    m: usize,
    alpha: f64,
    a: *const f64,
    lda: usize,
    x: *const f64,
    incx: usize,
    beta: f64,
    y: *mut f64,
) {
    #[cfg(target_arch = "x86_64")]
    {
        if is_x86_feature_detected!("fma") {
            crate::fma::dcombine_4(m, alpha, a, lda, x, incx, beta, y);
            return;
        }
    }
    let mut aptr0 = a;
    let mut aptr1 = a.add(lda);
    let mut aptr2 = a.add(lda * 2);
    let mut aptr3 = a.add(lda * 3);

    let xreg0 = *x * alpha;
    let xreg1 = *x.add(incx) * alpha;
    let xreg2 = *x.add(2 * incx) * alpha;
    let xreg3 = *x.add(3 * incx) * alpha;

    let mut yptr = y;

    for _ in 0..m {
        *yptr = beta * *yptr + *aptr0 * xreg0 + *aptr1 * xreg1 + *aptr2 * xreg2 + *aptr3 * xreg3;
        yptr = yptr.add(1);
        aptr0 = aptr0.add(1);
        aptr1 = aptr1.add(1);
        aptr2 = aptr2.add(1);
        aptr3 = aptr3.add(1);
    }
}

#[target_feature(enable = "fma")]
pub unsafe fn scombine_1(
    m: usize,
    alpha: f32,
    a: *const f32,
    x: *const f32,
    beta: f32,
    y: *mut f32,
) {
    #[cfg(target_arch = "x86_64")]
    {
        if is_x86_feature_detected!("fma") {
            crate::fma::scombine_1(m, alpha, a, x, beta, y);
            return;
        }
    }

    let mut aptr = a;

    let xreg = *x * alpha;

    let mut yptr = y;

    for _ in 0..m {
        *yptr = beta * *yptr + *aptr * xreg;

        yptr = yptr.add(1);
        aptr = aptr.add(1);
    }
}

#[target_feature(enable = "fma")]
pub unsafe fn dcombine_1(
    m: usize,
    alpha: f64,
    a: *const f64,
    x: *const f64,
    beta: f64,
    y: *mut f64,
) {
    #[cfg(target_arch = "x86_64")]
    {
        if is_x86_feature_detected!("fma") {
            crate::fma::dcombine_1(m, alpha, a, x, beta, y);
            return;
        }
    }

    let mut aptr = a;

    let xreg = *x * alpha;

    let mut yptr = y;

    for _ in 0..m {
        *yptr = beta * *yptr + *aptr * xreg;

        yptr = yptr.add(1);
        aptr = aptr.add(1);
    }
}
