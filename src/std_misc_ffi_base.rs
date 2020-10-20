use std::fmt;

// 此外部代码块链接到 libm 库
#[link(name = "m")]
extern "C" {
    // 这是外部语言函数
    // 这计算了一个单精度复数的平方根
    fn csqrtf(z: Complex) -> Complex;

    fn ccosf(z: Complex) -> Complex;

}

// 安全装包（原文：safe wrapper）
fn cos(z: Complex) -> Complex {
    unsafe { ccosf(z) }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn tmain() {
        // z = -1 + 0i
        let z = Complex { re: -1., im: 0. };

        // 调用一个外部语言函数是一种不安全的操作
        let z_sqrt = unsafe { csqrtf(z) };

        println!("the square root of {:?} is {:?}", z, z_sqrt);

        println!("cos({:?}) = {:?}", z, cos(z));
    }
}

// 最小化实现单精度复数
#[repr(C)]
#[derive(Copy, Clone)]
struct Complex {
    re: f32,
    im: f32,
}
impl fmt::Debug for Complex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.im < 0. {
            write!(f, "{}-{}i", self.re, -self.im)
        } else {
            write!(f, "{}+{}i", self.re, self.im)
        }
    }
}
