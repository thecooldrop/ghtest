use core::arch::x86_64::*;

pub unsafe fn sgemm(
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
    const MC: usize = 256;
    const KC: usize = 128;
    const NB: usize = 1000;

    let mut packed_a = vec![0.0; MC * KC];
    let mut packed_b = vec![0.0; KC * NB];

    for p in (0..k).step_by(KC) {
        let pb = std::cmp::min(k - p, KC);
        for i in (0..m).step_by(MC) {
            let ib = std::cmp::min(m - i, MC);
            inner_kernel(
                ib,
                n,
                pb,
                a.add(i + p * lda),
                lda,
                b.add(p),
                ldb,
                c.add(i),
                ldc,
                packed_a.as_mut_ptr(),
                packed_b.as_mut_ptr(),
                i == 0,
            );
        }
    }

    unsafe fn inner_kernel(
        m: usize,
        n: usize,
        k: usize,
        a: *const f32,
        lda: usize,
        b: *const f32,
        ldb: usize,
        c: *mut f32,
        ldc: usize,
        packed_a: *mut f32,
        packed_b: *mut f32,
        first_time: bool,
    ) {
        for j in (0..n).step_by(4) {
            if first_time {
                pack_b(k, b.add(j * ldb), ldb, packed_b.add(j * k));
            }
            for i in (0..m).step_by(8) {
                if j == 0 {
                    pack_a(k, a.add(i), lda, packed_a.add(i * k));
                }

                add_dot_4x8(
                    k,
                    packed_a.add(i * k),
                    packed_b.add(j * k),
                    c.add(i + j * ldc),
                    ldc,
                );
            }
        }
    }

    unsafe fn pack_b(k: usize, b: *const f32, ldb: usize, mut packed_b: *mut f32) {
        let mut bptr0 = b;
        let mut bptr1 = b.add(ldb);
        let mut bptr2 = b.add(ldb * 2);
        let mut bptr3 = b.add(ldb * 3);

        for _ in 0..k {
            *packed_b = *bptr0;
            *packed_b.add(1) = *bptr1;
            *packed_b.add(2) = *bptr2;
            *packed_b.add(3) = *bptr3;

            packed_b = packed_b.add(4);
            bptr0 = bptr0.add(1);
            bptr1 = bptr1.add(1);
            bptr2 = bptr2.add(1);
            bptr3 = bptr3.add(1);
        }
    }

    unsafe fn pack_a(k: usize, mut a: *const f32, lda: usize, mut packed_a: *mut f32) {
        for _ in 0..k {
            _mm256_storeu_ps(packed_a, _mm256_loadu_ps(a));

            a = a.add(lda);
            packed_a = packed_a.add(8);
        }
    }

    unsafe fn add_dot_4x8(k: usize, mut a: *const f32, b: *const f32, c: *mut f32, ldc: usize) {
        let mut bptr0 = b;
        let mut bptr1 = b.add(k);
        let mut bptr2 = b.add(2 * k);
        let mut bptr3 = b.add(3 * k);

        let mut c0_reg_v = _mm256_setzero_ps();
        let mut c1_reg_v = _mm256_setzero_ps();
        let mut c2_reg_v = _mm256_setzero_ps();
        let mut c3_reg_v = _mm256_setzero_ps();

        for _ in 0..k {
            let a0_reg_v = _mm256_loadu_ps(a);
            let bp0reg = _mm256_broadcast_ss(&*bptr0);
            let bp1reg = _mm256_broadcast_ss(&*bptr1);
            let bp2reg = _mm256_broadcast_ss(&*bptr2);
            let bp3reg = _mm256_broadcast_ss(&*bptr3);

            c0_reg_v = _mm256_fmadd_ps(a0_reg_v, bp0reg, c0_reg_v);
            c1_reg_v = _mm256_fmadd_ps(a0_reg_v, bp1reg, c1_reg_v);
            c2_reg_v = _mm256_fmadd_ps(a0_reg_v, bp2reg, c2_reg_v);
            c3_reg_v = _mm256_fmadd_ps(a0_reg_v, bp3reg, c3_reg_v);

            a = a.add(8);
            bptr0 = bptr0.add(1);
            bptr1 = bptr1.add(1);
            bptr2 = bptr2.add(1);
            bptr3 = bptr3.add(1);
        }

        let cptr0 = &mut *c;
        let cptr1 = &mut *c.add(ldc);
        let cptr2 = &mut *c.add(2 * ldc);
        let cptr3 = &mut *c.add(3 * ldc);

        _mm256_storeu_ps(cptr0, _mm256_add_ps(_mm256_loadu_ps(cptr0), c0_reg_v));
        _mm256_storeu_ps(cptr1, _mm256_add_ps(_mm256_loadu_ps(cptr1), c1_reg_v));
        _mm256_storeu_ps(cptr2, _mm256_add_ps(_mm256_loadu_ps(cptr2), c2_reg_v));
        _mm256_storeu_ps(cptr3, _mm256_add_ps(_mm256_loadu_ps(cptr3), c3_reg_v));
    }
}