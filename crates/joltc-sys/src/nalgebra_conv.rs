use crate::generated::*;

impl From<JPC_Float3> for nalgebra::Vector3<f32> {
    #[inline]
    fn from(value: JPC_Float3) -> Self {
        Self::new(value.x, value.y, value.z)
    }
}

impl From<nalgebra::Vector3<f32>> for JPC_Float3 {
    #[inline]
    fn from(value: nalgebra::Vector3<f32>) -> Self {
        Self {
            x: value.x,
            y: value.y,
            z: value.z,
        }
    }
}

impl From<JPC_Vec2> for nalgebra::Vector2<f32> {
    #[inline]
    fn from(value: JPC_Vec2) -> Self {
        Self::new(value.x, value.y)
    }
}

impl From<nalgebra::Vector2<f32>> for JPC_Vec2 {
    #[inline]
    fn from(value: nalgebra::Vector2<f32>) -> Self {
        Self {
            x: value.x,
            y: value.y,
        }
    }
}

impl From<JPC_Vec3> for nalgebra::Vector3<f32> {
    #[inline]
    fn from(value: JPC_Vec3) -> Self {
        Self::new(value.x, value.y, value.z)
    }
}

impl From<nalgebra::Vector3<f32>> for JPC_Vec3 {
    #[inline]
    fn from(value: nalgebra::Vector3<f32>) -> Self {
        Self {
            x: value.x,
            y: value.y,
            z: value.z,
            _w: 0.0,
        }
    }
}

impl From<JPC_DVec3> for nalgebra::Vector3<f64> {
    fn from(value: JPC_DVec3) -> Self {
        Self::new(value.x, value.y, value.z)
    }
}

impl From<nalgebra::Vector3<f64>> for JPC_DVec3 {
    #[inline]
    fn from(value: nalgebra::Vector3<f64>) -> Self {
        Self {
            x: value.x,
            y: value.y,
            z: value.z,
            _w: 0.0,
        }
    }
}

impl From<JPC_Mat44> for nalgebra::Matrix4<f32> {
    #[inline]
    fn from(value: JPC_Mat44) -> Self {
        let c0 = value.col[0];
        let c1 = value.col[1];
        let c2 = value.col[2];
        let c3 = value.col3;

        // 4, 4, [m11, m12, m13, m14;
        //       m21, m22, m23, m24;
        //       m31, m32, m33, m34;
        //       m41, m42, m43, m44];
        Self::new(
            c0.x, c1.x, c2.x, c3.x, //
            c0.y, c1.y, c2.y, c3.y, //
            c0.z, c1.z, c2.z, c3.z, //
            c0.w, c1.w, c2.w, 1.0, //
        )
    }
}

impl From<nalgebra::Matrix4<f32>> for JPC_Mat44 {
    #[inline]
    fn from(v: nalgebra::Matrix4<f32>) -> Self {
        // 4, 4, [m11, m12, m13, m14;
        //       m21, m22, m23, m24;
        //       m31, m32, m33, m34;
        //       m41, m42, m43, m44];
        let c0 = v.column(0);
        let c1 = v.column(1);
        let c2 = v.column(2);
        let c3 = v.column(3);

        Self {
            col: [
                JPC_Vec4 {
                    x: c0.x,
                    y: c0.y,
                    z: c0.z,
                    w: c0.w,
                },
                JPC_Vec4 {
                    x: c1.x,
                    y: c1.y,
                    z: c1.z,
                    w: c1.w,
                },
                JPC_Vec4 {
                    x: c2.x,
                    y: c2.y,
                    z: c2.z,
                    w: c2.w,
                },
            ],
            col3: JPC_Vec3 {
                x: c3.x,
                y: c3.y,
                z: c3.z,
                _w: 1.0,
            },
        }
    }
}

impl From<JPC_DMat44> for nalgebra::Matrix4<f64> {
    #[inline]
    fn from(value: JPC_DMat44) -> Self {
        let c0 = value.col[0];
        let c1 = value.col[1];
        let c2 = value.col[2];
        let c3 = value.col3;

        // 4, 4, [m11, m12, m13, m14;
        //       m21, m22, m23, m24;
        //       m31, m32, m33, m34;
        //       m41, m42, m43, m44];
        Self::new(
            c0.x as f64,
            c1.x as f64,
            c2.x as f64,
            c3.x, //
            c0.y as f64,
            c1.y as f64,
            c2.y as f64,
            c3.y, //
            c0.z as f64,
            c1.z as f64,
            c2.z as f64,
            c3.z, //
            c0.w as f64,
            c1.w as f64,
            c2.w as f64,
            1.0, //
        )
    }
}

impl From<nalgebra::Matrix4<f64>> for JPC_DMat44 {
    #[inline]
    fn from(v: nalgebra::Matrix4<f64>) -> Self {
        // 4, 4, [m11, m12, m13, m14;
        //       m21, m22, m23, m24;
        //       m31, m32, m33, m34;
        //       m41, m42, m43, m44];
        let c0 = v.column(0);
        let c1 = v.column(1);
        let c2 = v.column(2);
        let c3 = v.column(3);

        Self {
            col: [
                JPC_Vec4 {
                    x: c0.x as f32,
                    y: c0.y as f32,
                    z: c0.z as f32,
                    w: c0.w as f32,
                },
                JPC_Vec4 {
                    x: c1.x as f32,
                    y: c1.y as f32,
                    z: c1.z as f32,
                    w: c1.w as f32,
                },
                JPC_Vec4 {
                    x: c2.x as f32,
                    y: c2.y as f32,
                    z: c2.z as f32,
                    w: c2.w as f32,
                },
            ],
            __bindgen_padding_0: Default::default(),
            col3: JPC_DVec3 {
                x: c3.x,
                y: c3.y,
                z: c3.z,
                _w: 1.0,
            },
        }
    }
}

impl From<JPC_Quat> for nalgebra::Quaternion<f32> {
    #[inline]
    fn from(value: JPC_Quat) -> Self {
        Self::new(value.w, value.x, value.y, value.z)
    }
}

impl From<nalgebra::Quaternion<f32>> for JPC_Quat {
    #[inline]
    fn from(value: nalgebra::Quaternion<f32>) -> Self {
        Self {
            x: value.coords.x,
            y: value.coords.y,
            z: value.coords.z,
            w: value.coords.w,
        }
    }
}

impl From<JPC_Quat> for nalgebra::UnitQuaternion<f32> {
    #[inline]
    fn from(value: JPC_Quat) -> Self {
        Self::from_quaternion(nalgebra::Quaternion::new(
            value.w, value.x, value.y, value.z,
        ))
    }
}

impl From<nalgebra::UnitQuaternion<f32>> for JPC_Quat {
    #[inline]
    fn from(value: nalgebra::UnitQuaternion<f32>) -> Self {
        Self {
            x: value.coords.x,
            y: value.coords.y,
            z: value.coords.z,
            w: value.coords.w,
        }
    }
}
