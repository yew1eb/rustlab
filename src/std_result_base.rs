mod checked {
    // 我们想要捕获的数学“错误”
    #[derive(Debug)]
    pub enum MathError {
        DivisionByZero,
        NegativeLogarithm,
        NegativeSquareRoot,
    }

    pub type MathResult = Result<f64, MathError>;

    pub fn div(x: f64, y: f64) -> MathResult {
        if y == 0.0 {
            // 此操作将会失败，反而让我们返回失败的理由，并装包成 `Err`
            Err(MathError::DivisionByZero)
        } else {
            // 此操作是有效的，返回装包成 `Ok` 的结果
            Ok(x / y)
        }
    }

    pub fn sqrt(x: f64) -> MathResult {
        if x < 0.0 {
            Err(MathError::NegativeSquareRoot)
        } else {
            Ok(x.sqrt())
        }
    }

    pub fn ln(x: f64) -> MathResult {
        if x < 0.0 {
            Err(MathError::NegativeLogarithm)
        } else {
            Ok(x.ln())
        }
    }

    // 中间函数
    pub fn op_q(x: f64, y: f64) -> MathResult {
        // 如果 `div` “失败”了，那么 `DivisionByZero` 将被返回
        let ratio = div(x, y)?;

        // 如果 `ln` “失败”了，那么 `NegativeLogarithm` 将被返回
        let ln = ln(ratio)?;

        sqrt(ln)
    }
}

// `op(x, y)` === `sqrt(ln(x / y))`
pub fn op_div(x: f64, y: f64) -> f64 {
    // 这是一个三层的匹配金字塔！
    // （原文：This is a three level match pyramid!）
    match checked::div(x, y) {
        Err(why) => panic!("{:?}", why),
        Ok(ratio) => match checked::ln(ratio) {
            Err(why) => panic!("{:?}", why),
            Ok(ln) => match checked::sqrt(ln) {
                Err(why) => panic!("{:?}", why),
                Ok(sqrt) => sqrt,
            },
        },
    }
}

pub fn op_q(x: f64, y: f64) {
    match checked::op_q(x, y) {
        Err(why) => panic!(match why {
            checked::MathError::NegativeLogarithm => "logarithm of negative number",
            checked::MathError::DivisionByZero => "division by zero",
            checked::MathError::NegativeSquareRoot => "square root of negative number",
        }),
        Ok(value) => println!("{}", value),
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    #[should_panic(expected = "")]
    fn tmain() {
        // 这会失败吗？
        println!("{}", op_div(1.0, 10.0));
    }

    #[test]
    #[should_panic(expected = "")]
    fn question_mark() {
        //使用匹配链接结果会得到极其繁琐的内容；幸运的是，? 运算符可以使事情再次变得干净漂亮。? 运算符用在返回值为 Result 的表式式后面，等同于这样一个匹配表式，其中 Err(err) 分支展开成提前（返回）return Err(err)，同时 Ok(ok) 分支展开成 ok 表达式。
        op_q(1.0, 10.0);
    }
}
