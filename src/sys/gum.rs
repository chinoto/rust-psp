use core::{mem::MaybeUninit, ffi::c_void};
use crate::vfpu_asm;
use crate::sys::{
    gu,
    types::{ScePspFMatrix4, ScePspFVector3, ScePspFVector4},
    vfpu_context::{Context, MatrixSet},
};

// TODO: Change all register names in `llvm_asm` to register numbers. Fixes
// assembler bug.

// TODO: Replace this with the definiton in `gu` once merged.
#[repr(i32)]
#[derive(Copy, Debug, Clone)]
pub enum Mode {
    Projection = 0,
    View = 1,
    Model = 2,
    Texture = 3,
}

static mut MATRIX_STACK: [[ScePspFMatrix4; 32]; 4] = {
    let zero_vector = ScePspFVector4 { x: 0.0, y: 0.0, z: 0.0, w: 0.0 };
    let zero_matrix = ScePspFMatrix4 {
        x: zero_vector,
        y: zero_vector,
        z: zero_vector,
        w: zero_vector,
    };

    let stack = [
        zero_matrix, zero_matrix, zero_matrix, zero_matrix,
        zero_matrix, zero_matrix, zero_matrix, zero_matrix,
        zero_matrix, zero_matrix, zero_matrix, zero_matrix,
        zero_matrix, zero_matrix, zero_matrix, zero_matrix,

        zero_matrix, zero_matrix, zero_matrix, zero_matrix,
        zero_matrix, zero_matrix, zero_matrix, zero_matrix,
        zero_matrix, zero_matrix, zero_matrix, zero_matrix,
        zero_matrix, zero_matrix, zero_matrix, zero_matrix,
    ];

    [stack, stack, stack, stack]
};

static mut MATRIX_UPDATE: [i32; 4] = [0, 0, 0, 0];
static mut CURRENT_MATRIX_UPDATE: i32 = 0;

static mut CURRENT_MATRIX: *mut ScePspFMatrix4 = unsafe {
    &mut MATRIX_STACK[Mode::Projection as usize][0]
};

static mut CURRENT_MODE: Mode = Mode::Projection;
static mut STACK_DEPTH: [*mut ScePspFMatrix4; 4] = unsafe {
    [
        &mut MATRIX_STACK[Mode::Projection as usize][0],
        &mut MATRIX_STACK[Mode::View as usize][0],
        &mut MATRIX_STACK[Mode::Model as usize][0],
        &mut MATRIX_STACK[Mode::Texture as usize][0],
    ]
};

static mut VFPU_CONTEXT: Option<Context> = None;
unsafe fn get_context_unchecked() -> &'static mut Context {
    match VFPU_CONTEXT.as_mut() {
        Some(r) => r,
        None => core::intrinsics::unreachable(),
    }
}

pub const EPSILON: f32 = 0.00001;

pub unsafe fn sce_gum_draw_array(
    prim: gu::Primitive,
    v_type: gu::VertexType,
    count: i32,
    indices: *const c_void,
    vertices: *const c_void,
) {
    sce_gum_update_matrix();
    gu::sce_gu_draw_array(prim, v_type, count, indices, vertices);
}

pub unsafe fn sce_gum_draw_array_n(
    prim: gu::Primitive,
    v_type: gu::VertexType,
    count: i32,
    a3: i32,
    indices: *const c_void,
    vertices: *const c_void,
) {
    sce_gum_update_matrix();
    gu::sce_gu_draw_array_n(prim, v_type, count, a3, indices, vertices);
}

pub unsafe fn sce_gum_draw_bezier(
    v_type: gu::VertexType,
    u_count: i32,
    v_count: i32,
    indices: *const c_void,
    vertices: *const c_void,
) {
    sce_gum_update_matrix();
    gu::sce_gu_draw_bezier(v_type, u_count, v_count, indices, vertices);
}

pub unsafe fn sce_gum_draw_spline(
    v_type: gu::VertexType,
    u_count: i32,
    v_count: i32,
    u_edge: i32,
    v_edge: i32,
    indices: *const c_void,
    vertices: *const c_void,
) {
    sce_gum_update_matrix();
    gu::sce_gu_draw_spline(v_type, u_count, v_count, u_edge, v_edge, indices, vertices);
}

pub unsafe fn sce_gum_fast_inverse() {
    get_context_unchecked().prepare(MatrixSet::VMAT3, MatrixSet::VMAT0 | MatrixSet::VMAT1);

    vfpu_asm!(
        vmidt_q M000;
        vmmov_t M000, E300;
        vneg_t  C100, C330;
        vtfm3_t C030, M300, C100;
        vmmov_q M300, M000;

        : : : : "volatile"
    );

    CURRENT_MATRIX_UPDATE = 1;
}

pub unsafe fn sce_gum_full_inverse() {
    let mut t = MaybeUninit::uninit();

    get_context_unchecked().prepare(MatrixSet::VMAT3, MatrixSet::VMAT0 | MatrixSet::VMAT1);

    vfpu_asm!(
        sv_q C300, a0;
        sv_q C310, 16(a0);
        sv_q C320, 32(a0);
        sv_q C330, 48(a0);

        : : "{a0}"(t.as_mut_ptr()) : "memory" : "volatile"
    );

    let t = gum_fast_inverse(&*t.as_ptr());

    vfpu_asm!(
        lv_q C300, t0;
        lv_q C310, 16(t0);
        lv_q C320, 32(t0);
        lv_q C330, 48(a0);

        : : "{t0}"(&t) : "memory" : "volatile"
    );

    CURRENT_MATRIX_UPDATE = 1;
}

/// Load identity matrix
///
/// ```txt
/// [1 0 0 0]
/// [0 1 0 0]
/// [0 0 1 0]
/// [0 0 0 1]
/// ```
pub unsafe fn sce_gum_load_identity() {
    VFPU_CONTEXT
        .get_or_insert_with(Context::new)
        .prepare(MatrixSet::VMAT3, MatrixSet::empty());

    vfpu_asm!(vmidt_q M300; : : : : "volatile");

    CURRENT_MATRIX_UPDATE = 1;
}

/// Load matrix
///
/// # Parameters
///
/// - `m`: Matrix to load into stack
pub unsafe fn sce_gum_load_matrix(m: &ScePspFMatrix4) {
    VFPU_CONTEXT
        .get_or_insert_with(Context::new)
        .prepare(MatrixSet::VMAT3, MatrixSet::empty());

    vfpu_asm!(
        lv_q C300,  0(a0);
        lv_q C310, 16(a0);
        lv_q C320, 32(a0);
        lv_q C330, 48(a0);

        : : "{a0}"(m) : "memory" : "volatile"
    );

    CURRENT_MATRIX_UPDATE = 1;
}

pub unsafe fn sce_gum_look_at(eye: &ScePspFVector3, center: &ScePspFVector3, up: &ScePspFVector3) {
    let mut t = gum_load_identity();
    gum_look_at(&mut t, eye, center, up);

    get_context_unchecked().prepare(MatrixSet::VMAT3, MatrixSet::VMAT0 | MatrixSet::VMAT1);

    vfpu_asm!(
        lv_q C000, t0;
        lv_q C010, 16(t0);
        lv_q C020, 32(t0);
        lv_q C030, 48(t0);
        vmmul_q M100, M300, M000;
        vmmov_q M300, M100;

        : : "{t0}"(&t) : : "volatile"
    );

    CURRENT_MATRIX_UPDATE = 1;
}

/// Select which matrix stack to operate on
///
/// # Parameters
///
/// - `mode`: Matrix mode to use
pub unsafe fn sce_gum_matrix_mode(mode: Mode) {
    get_context_unchecked().prepare(MatrixSet::VMAT3, MatrixSet::empty());

    vfpu_asm!(
        sv_q C300, t0;
        sv_q C310, 16(t0);
        sv_q C320, 32(t0);
        sv_q C330, 48(t0);

        : : "{8}"(CURRENT_MATRIX) : "memory" : "volatile"
    );

    MATRIX_UPDATE[CURRENT_MODE as usize] = CURRENT_MATRIX_UPDATE;
    STACK_DEPTH[CURRENT_MODE as usize] = CURRENT_MATRIX;
    CURRENT_MATRIX = STACK_DEPTH[mode as usize];
    CURRENT_MODE = mode;
    CURRENT_MATRIX_UPDATE = MATRIX_UPDATE[CURRENT_MODE as usize];

    vfpu_asm!(
        lv_q C300, t0;
        lv_q C310, 16(t0);
        lv_q C320, 32(t0);
        lv_q C330, 48(t0);

        : : "{8}"(CURRENT_MATRIX) : "memory" : "volatile"
    );
}

/// Multiply current matrix with input
///
/// # Parameters
///
/// - `m`: Matrix to multiply stack with
pub unsafe fn sce_gum_mult_matrix(m: &ScePspFMatrix4) {
    get_context_unchecked().prepare(MatrixSet::VMAT3, MatrixSet::VMAT0 | MatrixSet::VMAT1);

    vfpu_asm!(
        lv_q C000,  0(t0);
        lv_q C010, 16(t0);
        lv_q C020, 32(t0);
        lv_q C030, 48(t0);

        vmmul_q M100, M300, M000;
        vmmov_q M300, M100;

        : : "{t0}"(m) : "memory" : "volatile"
    );

    CURRENT_MATRIX_UPDATE = 1;
}

/// Apply ortho projection matrix
///
/// # Note
///
/// The matrix loses its orthonogal status after executing this function.
pub unsafe fn sce_gum_ortho(
    left: f32,
    right: f32,
    bottom: f32,
    top: f32,
    near: f32,
    far: f32
) {
    get_context_unchecked().prepare(MatrixSet::VMAT3, MatrixSet::VMAT0 | MatrixSet::VMAT1);

    vfpu_asm!(
        .mips "mfc1 $$t0, $0";
        .mips "mfc1 $$t1, $1";
        .mips "mfc1 $$t2, $2";
        .mips "mfc1 $$t3, $3";
        .mips "mfc1 $$t4, $4";
        .mips "mfc1 $$t5, $5";

        vmidt_q M100;                         // set M100 to identity
        mtv    t1, S000;                      // C000 = [right, ?,      ?,  ]
        mtv    t3, S001;                      // C000 = [right, top,    ?,  ]
        mtv    t5, S002;                      // C000 = [right, top,    far ]
        mtv    t0, S010;                      // C010 = [left,  ?,      ?,  ]
        mtv    t2, S011;                      // C010 = [left,  bottom, ?,  ]
        mtv    t4, S012;                      // C010 = [left,  bottom, near]
        vsub_t  C020, C000, C010;             // C020 = [  dx,   dy,   dz]
        vrcp_t  C020, C020;                   // C020 = [1/dx, 1/dy, 1/dz]

        vpfxs [2];                            // S100 = m->x.x = 2.0 / dx
        vmul_s S100, S100, S020;

        vpfxs [2];                            // S110 = m->y.y = 2.0 / dy
        vmul_s  S111, S111, S021;

        vpfxs [2];                            // S122 = m->z.z = -2.0 / dz
        vpfxt [-X];
        vmul_s  S122, S122, S022;

        vpfxs [-X], [-Y], [-Z];               // C130 = m->w[x, y, z] = [-(right+left), -(top+bottom), -(far+near)]
        vsub_t  C130, C000, C010;             // we do vsub here since -(a+b) => (-1*a) + (-1*b) => -a - b

        vmul_t  C130, C130, C020;             // C130 = [-(right+left)/dx, -(top+bottom)/dy, -(far+near)/dz]
        vmmul_q M000, M300, M100;
        vmmov_q M300, M000;

        : : "f"(left), "f"(right), "f"(bottom), "f"(top), "f"(near), "f"(far)
        : "t0", "t1", "t2", "t3", "t4", "t5" : "volatile"
    );

    CURRENT_MATRIX_UPDATE = 1;
}

/// Apply perspective projection matrix
///
/// # Note
///
/// The matrix loses its orthonogal status after executing this function.
pub unsafe fn sce_gum_perspective(fovy: f32, aspect: f32, near: f32, far: f32) {
    get_context_unchecked().prepare(MatrixSet::VMAT3, MatrixSet::VMAT0 | MatrixSet::VMAT1);

    vfpu_asm!(
        .mips "mfc1 $$t0, $0";
        .mips "mfc1 $$t1, $1";
        .mips "mfc1 $$t2, $2";
        .mips "mfc1 $$t3, $3";

        vmzero_q M100;                   // set M100 to all zeros
        mtv     t0, S000;                // S000 = fovy
        viim_s  S001, 90;                // S002 = 90.0f
        vrcp_s  S001, S001;              // S002 = 1/90

        vpfxt [1/2];                     // S000 = fovy * 0.5 = fovy/2
        vmul_s  S000, S000, S000;

        vmul_s  S000, S000, S001;        // S000 = (fovy/2)/90
        vrot_p  C002, S000, [C, S];      // S002 = cos(angle), S003 = sin(angle)
        vdiv_s  S100, S002, S003;        // S100 = m->x.x = cotangent = cos(angle)/sin(angle)
        mtv     t2, S001;                // S001 = near
        mtv     t3, S002;                // S002 = far
        vsub_s  S003, S001, S002;        // S003 = deltaz = near-far
        vrcp_s  S003, S003;              // S003 = 1/deltaz
        mtv     t1, S000;                // S000 = aspect
        vmov_s  S111, S100;              // S111 = m->y.y = cotangent
        vdiv_s  S100, S100, S000;        // S100 = m->x.x = cotangent / aspect
        vadd_s  S122, S001, S002;        // S122 = m->z.z = far + near
        vmul_s  S122, S122, S003;        // S122 = m->z.z = (far+near)/deltaz
        vmul_s  S132, S001, S002;        // S132 = m->w.z = far * near

        vpfxt [2];                       // S132 = m->w.z = 2 * (far*near)
        vmul_s  S132, S132, S132;

        vmul_s  S132, S132, S003;        // S132 = m->w.z = 2 * (far*near) / deltaz

        vpfxt [1];                       // S123 = m->z.w = -1.0
        vsub_s  S123, S123, S123;

        vmmul_q M000, M300, M100;
        vmmov_q M300, M000;

        : : "f"(fovy), "f"(aspect), "f"(near), "f"(far)
        : "8", "9", "10", "11" : "volatile"
    );

    CURRENT_MATRIX_UPDATE = 1;
}

/// Pop matrix from stack
pub unsafe fn sce_gum_pop_matrix() {
    CURRENT_MATRIX = CURRENT_MATRIX.offset(-1);
    get_context_unchecked().prepare(MatrixSet::VMAT3, MatrixSet::empty());

    vfpu_asm!(
        lv_q C300,  0(t0);
        lv_q C310, 16(t0);
        lv_q C320, 32(t0);
        lv_q C330, 48(t0);

        : : "{8}"(CURRENT_MATRIX) : : "volatile"
    );

    CURRENT_MATRIX_UPDATE = 1;
}

/// Push current matrix onto stack
pub unsafe fn sce_gum_push_matrix() {
    CURRENT_MATRIX = CURRENT_MATRIX.offset(1);
    get_context_unchecked().prepare(MatrixSet::VMAT3, MatrixSet::empty());

    vfpu_asm!(
        sv_q C300,  0(t0);
        sv_q C310, 16(t0);
        sv_q C320, 32(t0);
        sv_q C330, 48(t0);

        : : "{8}"(CURRENT_MATRIX) : "memory" : "volatile"
    );
}

/// Rotate around the X axis
///
/// # Parameters
///
/// - `angle`: Angle in radians
pub unsafe fn sce_gum_rotate_x(angle: f32) {
    get_context_unchecked().prepare(MatrixSet::VMAT3, MatrixSet::VMAT0 | MatrixSet::VMAT1);

    vfpu_asm!(
        .mips "mfc1 $$t0, $0";
        vmidt_q M000;
        mtv t0, S100;
        vcst_s S101, VFPU_2_PI;
        vmul_s S100, S101, S100;
        vrot_q C010, S100, [0, C, S, 0];
        vrot_q C020, S100, [0, -S, C, 0];
        vmmul_q M100, M300, M000;
        vmmov_q M300, M100;

        : : "f"(angle) : "8" : "volatile"
    );

    CURRENT_MATRIX_UPDATE = 1;
}

/// Rotate around the Y axis
///
/// # Parameters
///
/// - `angle`: Angle in radians
pub unsafe fn sce_gum_rotate_y(angle: f32) {
    get_context_unchecked().prepare(MatrixSet::VMAT3, MatrixSet::VMAT0 | MatrixSet::VMAT1);

    vfpu_asm!(
        .mips "mfc1 $$t0, $0";
        vmidt_q M000;
        mtv     t0, S100;
        vcst_s  S101, VFPU_2_PI;
        vmul_s  S100, S101, S100;
        vrot_q  C000, S100, [C, 0,-S, 0];
        vrot_q  C020, S100, [S, 0, C, 0];
        vmmul_q M100, M300, M000;
        vmmov_q M300, M100;

        : : "f"(angle) : "t0" : "volatile"
    );

    CURRENT_MATRIX_UPDATE = 1;
}

/// Rotate around the Z axis
///
/// # Parameters
///
/// - `angle`: Angle in radians
pub unsafe fn sce_gum_rotate_z(angle: f32) {
    get_context_unchecked().prepare(MatrixSet::VMAT3, MatrixSet::VMAT0 | MatrixSet::VMAT1);

    vfpu_asm!(
        .mips "mfc1 $$t0, $0";
        vmidt_q M000;
        mtv     t0, S100;
        vcst_s  S101, VFPU_2_PI;
        vmul_s  S100, S101, S100;
        vrot_q  C000, S100, [ C, S, 0, 0];
        vrot_q  C010, S100, [-S, C, 0, 0];
        vmmul_q M100, M300, M000;
        vmmov_q M300, M100;

        : : "f"(angle) : "t0" : "volatile"
    );

    CURRENT_MATRIX_UPDATE = 1;
}

/// Rotate around all 3 axis in order X, Y, Z
///
/// # Parameters
///
/// - `v`: Pointer to vector containing angles
pub unsafe fn sce_gum_rotate_xyz(v: &ScePspFVector3) {
    sce_gum_rotate_x(v.x);
    sce_gum_rotate_y(v.y);
    sce_gum_rotate_z(v.z);
}

/// Rotate around all 3 axis in order Z, Y, X
///
/// # Parameters
///
/// - `v`: Pointer to vector containing angles
pub unsafe fn sce_gum_rotate_zyx(v: &ScePspFVector3) {
    sce_gum_rotate_z(v.z);
    sce_gum_rotate_y(v.y);
    sce_gum_rotate_x(v.x);
}

/// Scale matrix
///
/// # Note
///
/// The matrix loses its orthonogal status after executing this function.
pub unsafe fn sce_gum_scale(v: &ScePspFVector3) {
    get_context_unchecked().prepare(MatrixSet::VMAT3, MatrixSet::VMAT0);

    vfpu_asm!(
        lv_q C000, a0;
        vscl_t C300, C300, S000;
        vscl_t C310, C310, S001;
        vscl_t C320, C320, S002;

        : : "{a0}"(v) : : "volatile"
    );
}

/// Store current matrix in the stack
///
/// # Parameters
///
/// - `m`: Matrix to write result to
pub unsafe fn sce_gum_store_matrix(m: &mut ScePspFMatrix4) {
    get_context_unchecked().prepare(MatrixSet::VMAT3, MatrixSet::VMAT0);

    vfpu_asm!(
        sv_q C300,  0(a0);
        sv_q C310, 16(a0);
        sv_q C320, 32(a0);
        sv_q C330, 48(a0);

        : : "{a0}"(m) : "memory" : "volatile"
    );
}

/// Translate coordinate system
///
/// # Parameters
///
/// - `v`: Translation coordinates
pub unsafe fn sce_gum_translate(v: &ScePspFVector3) {
    get_context_unchecked().prepare(MatrixSet::VMAT3, MatrixSet::VMAT0 | MatrixSet::VMAT1);

    vfpu_asm!(
        vmidt_q M000;
        lv_q    C100, a0;
        vmov_t  C030, C100;
        vmmul_q M100, M300, M000;
        vmmov_q M300, M100;

        : : "{4}"(v) : "memory" : "volatile"
    );

    CURRENT_MATRIX_UPDATE = 1;
}

/// Explicitly flush dirty matrices to the hardware
pub unsafe fn sce_gum_update_matrix() {
    STACK_DEPTH[CURRENT_MODE as usize] = CURRENT_MATRIX;

    if CURRENT_MATRIX_UPDATE != 0 {
        get_context_unchecked().prepare(MatrixSet::VMAT3, MatrixSet::empty());

        vfpu_asm!(
            sv_q C300,  0(t0);
            sv_q C310, 16(t0);
            sv_q C320, 32(t0);
            sv_q C330, 48(t0);

            : : "{8}"(CURRENT_MATRIX) : "memory" : "volatile"
        );

        MATRIX_UPDATE[CURRENT_MODE as usize] = CURRENT_MATRIX_UPDATE;
        CURRENT_MATRIX_UPDATE = 0;
    }

    for i in 0..4 {
        if MATRIX_UPDATE[i] != 0 {
            let mode = match i {
                0 => gu::MatrixMode::Projection,
                1 => gu::MatrixMode::View,
                2 => gu::MatrixMode::Model,
                3 => gu::MatrixMode::Texture,
                _ => core::intrinsics::unreachable(),
            };

            gu::sce_gu_set_matrix(mode, &*STACK_DEPTH[i]);

            MATRIX_UPDATE[i] = 0;
        }
    }
}

fn gum_normalize(v: &mut ScePspFVector3) {
    let l = unsafe {
        use core::intrinsics::sqrtf32;
        sqrtf32((v.x * v.x) + (v.y * v.y) + (v.z * v.z))
    };

    if l > EPSILON {
        let il = 1.0 / l;

        v.x *= il;
        v.y *= il;
        v.z *= il;
    }
}

fn gum_cross_product(a: &ScePspFVector3, b: &ScePspFVector3) -> ScePspFVector3 {
    ScePspFVector3 {
        x: a.y * b.z - a.z * b.y,
        y: a.z * b.x - a.x * b.z,
        z: a.x * b.y - a.y * b.x,
    }
}

unsafe fn gum_look_at(
    mat: &mut ScePspFMatrix4,
    eye: &ScePspFVector3,
    center: &ScePspFVector3,
    up: &ScePspFVector3,
) {
    let mut forward = ScePspFVector3 {
        x: center.x - eye.x,
        y: center.y - eye.y,
        z: center.z - eye.z,
    };

    gum_normalize(&mut forward);

    let mut side = gum_cross_product(&forward, &up);
    gum_normalize(&mut side);

    let lup = gum_cross_product(&side, &forward);

    let mut t = gum_load_identity();

    t.x.x = side.x;
    t.y.x = side.y;
    t.z.x = side.z;

    t.x.y = lup.x;
    t.y.y = lup.y;
    t.z.y = lup.z;

    t.x.z = -forward.x;
    t.y.z = -forward.y;
    t.z.z = -forward.z;

    let ieye = ScePspFVector3 {
        x: -eye.x,
        y: -eye.y,
        z: -eye.z,
    };

    let mut mat = gum_mult_matrix(mat, &t);
    gum_translate(&mut mat, &ieye);
}

unsafe fn gum_translate(m: &mut ScePspFMatrix4, v: &ScePspFVector3) {
    get_context_unchecked().prepare(
        MatrixSet::empty(),
        MatrixSet::VMAT0 | MatrixSet::VMAT1 | MatrixSet::VMAT2,
    );

    vfpu_asm!(
        lv_q C100,  0(a0);
        lv_q C110, 16(a0);
        lv_q C120, 32(a0);
        lv_q C130, 48(a0);

        vmidt_q M000;
        lv_q    C200, a1;
        vmov_t  C030, C200;
        vmmul_q M200, M100, M000;

        sv_q C200,  0(a0);
        sv_q C210, 16(a0);
        sv_q C220, 32(a0);
        sv_q C230, 48(a0);

        : : "{a0}"(m), "{a1}"(v) : "memory" : "volatile"
    );
}

unsafe fn gum_load_identity() -> ScePspFMatrix4 {
    get_context_unchecked().prepare(MatrixSet::empty(), MatrixSet::VMAT0);

    let mut out = MaybeUninit::uninit();

    vfpu_asm!(
        vmidt_q M000;
        sv_q C000,  0(a0);
        sv_q C010, 16(a0);
        sv_q C020, 32(a0);
        sv_q C030, 48(a0);

        : : "{a0}"(out.as_mut_ptr()) : "memory" : "volatile"
    );

    out.assume_init()
}

unsafe fn gum_fast_inverse(a: &ScePspFMatrix4) -> ScePspFMatrix4 {
    get_context_unchecked().prepare(
        MatrixSet::empty(),
        MatrixSet::VMAT0 | MatrixSet::VMAT1 | MatrixSet::VMAT2,
    );

    let mut out = MaybeUninit::uninit();

    vfpu_asm!(
        lv_q C200,  0(a1);
        lv_q C210, 16(a1);
        lv_q C220, 32(a1);
        lv_q C230, 48(a1);

        vmidt_q M000;
        vmmov_t M000, E200;
        vneg_t C100, C230;
        vtfm3_t C030, M200, C100;

        sv_q C000,  0(a0);
        sv_q C010, 16(a0);
        sv_q C020, 32(a0);
        sv_q C030, 48(a0);

        : : "{a0}"(out.as_mut_ptr()), "{a1}"(a) : "memory" : "volatile"
    );

    out.assume_init()
}

unsafe fn gum_mult_matrix(a: &ScePspFMatrix4, b: &ScePspFMatrix4) -> ScePspFMatrix4 {
    get_context_unchecked().prepare(
        MatrixSet::empty(),
        MatrixSet::VMAT0 | MatrixSet::VMAT1 | MatrixSet::VMAT2,
    );

    let mut out = MaybeUninit::uninit();

    vfpu_asm!(
        lv_q C000,  0(a1);
        lv_q C010, 16(a1);
        lv_q C020, 32(a1);
        lv_q C030, 48(a1);

        lv_q C100,  0(a2);
        lv_q C110, 16(a2);
        lv_q C120, 32(a2);
        lv_q C130, 48(a2);

        vmmul_q M200, M000, M100;

        sv_q C200,  0(a0);
        sv_q C210, 16(a0);
        sv_q C220, 32(a0);
        sv_q C230, 48(a0);

        : : "{a0}"(&mut out), "{a1}"(a), "{a2}"(b) : "memory" : "volatile"
    );

    out.assume_init()
}
