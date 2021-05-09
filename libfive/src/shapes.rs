//
// Rust API for the libfive standard library’s ‘shapes’ module.
//
// DO NOT EDIT BY HAND!
// This file is automatically generated from libfive/stdlib/stdlib.h.
//
// It was last generated on 2021-05-09 16:23:13 by user moritz.
//

/// # Shapes <a name="shapes"></a>
impl Tree {

    pub fn circle(r: TreeFloat, center: TreeVec2) -> Self {
        Self(unsafe { sys::libfivestd_circle(r.0, sys::tvec2 { x: center.x.0, y: center.y.0 }) })
    }

    pub fn ring(ro: TreeFloat, ri: TreeFloat, center: TreeVec2) -> Self {
        Self(unsafe { sys::libfivestd_ring(ro.0, ri.0, sys::tvec2 { x: center.x.0, y: center.y.0 }) })
    }

    pub fn polygon(r: TreeFloat, n: u32, center: TreeVec2) -> Self {
        Self(unsafe { sys::libfivestd_polygon(r.0, n.try_into().unwrap(), sys::tvec2 { x: center.x.0, y: center.y.0 }) })
    }

    pub fn rectangle(a: TreeVec2, b: TreeVec2) -> Self {
        Self(unsafe { sys::libfivestd_rectangle(sys::tvec2 { x: a.x.0, y: a.y.0 }, sys::tvec2 { x: b.x.0, y: b.y.0 }) })
    }

    pub fn rounded_rectangle(a: TreeVec2, b: TreeVec2, r: TreeFloat) -> Self {
        Self(unsafe { sys::libfivestd_rounded_rectangle(sys::tvec2 { x: a.x.0, y: a.y.0 }, sys::tvec2 { x: b.x.0, y: b.y.0 }, r.0) })
    }

    pub fn rectangle_exact(a: TreeVec2, b: TreeVec2) -> Self {
        Self(unsafe { sys::libfivestd_rectangle_exact(sys::tvec2 { x: a.x.0, y: a.y.0 }, sys::tvec2 { x: b.x.0, y: b.y.0 }) })
    }

    pub fn rectangle_centered_exact(size: TreeVec2, center: TreeVec2) -> Self {
        Self(unsafe { sys::libfivestd_rectangle_centered_exact(sys::tvec2 { x: size.x.0, y: size.y.0 }, sys::tvec2 { x: center.x.0, y: center.y.0 }) })
    }

    pub fn triangle(a: TreeVec2, b: TreeVec2, c: TreeVec2) -> Self {
        Self(unsafe { sys::libfivestd_triangle(sys::tvec2 { x: a.x.0, y: a.y.0 }, sys::tvec2 { x: b.x.0, y: b.y.0 }, sys::tvec2 { x: c.x.0, y: c.y.0 }) })
    }

    pub fn box_mitered(a: TreeVec3, b: TreeVec3) -> Self {
        Self(unsafe { sys::libfivestd_box_mitered(sys::tvec3 { x: a.x.0, y: a.y.0, z: a.z.0 }, sys::tvec3 { x: b.x.0, y: b.y.0, z: b.z.0 }) })
    }

    pub fn box_mitered_centered(size: TreeVec3, center: TreeVec3) -> Self {
        Self(unsafe { sys::libfivestd_box_mitered_centered(sys::tvec3 { x: size.x.0, y: size.y.0, z: size.z.0 }, sys::tvec3 { x: center.x.0, y: center.y.0, z: center.z.0 }) })
    }

    pub fn box_exact_centered(size: TreeVec3, center: TreeVec3) -> Self {
        Self(unsafe { sys::libfivestd_box_exact_centered(sys::tvec3 { x: size.x.0, y: size.y.0, z: size.z.0 }, sys::tvec3 { x: center.x.0, y: center.y.0, z: center.z.0 }) })
    }

    pub fn box_exact(a: TreeVec3, b: TreeVec3) -> Self {
        Self(unsafe { sys::libfivestd_box_exact(sys::tvec3 { x: a.x.0, y: a.y.0, z: a.z.0 }, sys::tvec3 { x: b.x.0, y: b.y.0, z: b.z.0 }) })
    }

    pub fn rounded_box(a: TreeVec3, b: TreeVec3, r: TreeFloat) -> Self {
        Self(unsafe { sys::libfivestd_rounded_box(sys::tvec3 { x: a.x.0, y: a.y.0, z: a.z.0 }, sys::tvec3 { x: b.x.0, y: b.y.0, z: b.z.0 }, r.0) })
    }

    pub fn sphere(radius: TreeFloat, center: TreeVec3) -> Self {
        Self(unsafe { sys::libfivestd_sphere(radius.0, sys::tvec3 { x: center.x.0, y: center.y.0, z: center.z.0 }) })
    }

    pub fn half_space(norm: TreeVec3, point: TreeVec3) -> Self {
        Self(unsafe { sys::libfivestd_half_space(sys::tvec3 { x: norm.x.0, y: norm.y.0, z: norm.z.0 }, sys::tvec3 { x: point.x.0, y: point.y.0, z: point.z.0 }) })
    }

    pub fn cylinder_z(r: TreeFloat, h: TreeFloat, base: TreeVec3) -> Self {
        Self(unsafe { sys::libfivestd_cylinder_z(r.0, h.0, sys::tvec3 { x: base.x.0, y: base.y.0, z: base.z.0 }) })
    }

    pub fn cone_ang_z(angle: TreeFloat, height: TreeFloat, base: TreeVec3) -> Self {
        Self(unsafe { sys::libfivestd_cone_ang_z(angle.0, height.0, sys::tvec3 { x: base.x.0, y: base.y.0, z: base.z.0 }) })
    }

    pub fn cone_z(radius: TreeFloat, height: TreeFloat, base: TreeVec3) -> Self {
        Self(unsafe { sys::libfivestd_cone_z(radius.0, height.0, sys::tvec3 { x: base.x.0, y: base.y.0, z: base.z.0 }) })
    }

    pub fn pyramid_z(a: TreeVec2, b: TreeVec2, zmin: TreeFloat, height: TreeFloat) -> Self {
        Self(unsafe { sys::libfivestd_pyramid_z(sys::tvec2 { x: a.x.0, y: a.y.0 }, sys::tvec2 { x: b.x.0, y: b.y.0 }, zmin.0, height.0) })
    }

    pub fn torus_z(ro: TreeFloat, ri: TreeFloat, center: TreeVec3) -> Self {
        Self(unsafe { sys::libfivestd_torus_z(ro.0, ri.0, sys::tvec3 { x: center.x.0, y: center.y.0, z: center.z.0 }) })
    }

    pub fn gyroid(period: TreeVec3, thickness: TreeFloat) -> Self {
        Self(unsafe { sys::libfivestd_gyroid(sys::tvec3 { x: period.x.0, y: period.y.0, z: period.z.0 }, thickness.0) })
    }

    pub fn emptiness() -> Self {
        Self(unsafe { sys::libfivestd_emptiness() })
    }
}