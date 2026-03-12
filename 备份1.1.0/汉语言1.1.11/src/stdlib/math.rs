// 数学计算模块

use crate::runtime::value::Value;

/// 计算绝对值
pub fn builtin_数学_绝对值(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 1 {
        return Err("绝对值函数需要一个参数".to_string());
    }
    
    match &args[0] {
        Value::整数(n) => Ok(Value::整数(n.abs())),
        Value::浮点数(f) => Ok(Value::浮点数(f.abs())),
        _ => Err("绝对值函数只支持数字类型".to_string()),
    }
}

/// 计算平方根
pub fn builtin_数学_平方根(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 1 {
        return Err("平方根函数需要一个参数".to_string());
    }
    
    match &args[0] {
        Value::整数(n) => {
            if *n < 0 {
                return Err("平方根函数参数不能为负数".to_string());
            }
            Ok(Value::浮点数(((*n) as f64).sqrt()))
        }
        Value::浮点数(f) => {
            if *f < 0.0 {
                return Err("平方根函数参数不能为负数".to_string());
            }
            Ok(Value::浮点数(f.sqrt()))
        }
        _ => Err("平方根函数只支持数字类型".to_string()),
    }
}

/// 计算平方
pub fn builtin_数学_平方(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 1 {
        return Err("平方函数需要一个参数".to_string());
    }
    
    match &args[0] {
        Value::整数(n) => Ok(Value::整数(n * n)),
        Value::浮点数(f) => Ok(Value::浮点数(f * f)),
        _ => Err("平方函数只支持数字类型".to_string()),
    }
}

/// 计算立方
pub fn builtin_数学_立方(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 1 {
        return Err("立方函数需要一个参数".to_string());
    }
    
    match &args[0] {
        Value::整数(n) => Ok(Value::整数(n * n * n)),
        Value::浮点数(f) => Ok(Value::浮点数(f * f * f)),
        _ => Err("立方函数只支持数字类型".to_string()),
    }
}

/// 计算最大值
pub fn builtin_数学_最大值(args: Vec<Value>) -> Result<Value, String> {
    if args.is_empty() {
        return Err("最大值函数至少需要一个参数".to_string());
    }
    
    let mut max_value = args[0].clone();
    
    for arg in args.iter().skip(1) {
        match (max_value.clone(), arg.clone() as Value) {
            (Value::整数(a), Value::整数(b)) => {
                if b > a {
                    max_value = Value::整数(b);
                }
            }
            (Value::浮点数(a), Value::浮点数(b)) => {
                if b > a {
                    max_value = Value::浮点数(b);
                }
            }
            (Value::整数(a), Value::浮点数(b)) => {
                if b > a as f64 {
                    max_value = Value::浮点数(b);
                }
            }
            (Value::浮点数(a), Value::整数(b)) => {
                if b as f64 > a {
                    max_value = Value::整数(b);
                }
            }
            _ => return Err("最大值函数只支持数字类型".to_string()),
        }
    }
    
    Ok(max_value)
}

/// 计算最小值
pub fn builtin_数学_最小值(args: Vec<Value>) -> Result<Value, String> {
    if args.is_empty() {
        return Err("最小值函数至少需要一个参数".to_string());
    }
    
    let mut min_value = args[0].clone();
    
    for arg in args.iter().skip(1) {
        match (min_value.clone(), arg.clone() as Value) {
            (Value::整数(a), Value::整数(b)) => {
                if b < a {
                    min_value = Value::整数(b);
                }
            }
            (Value::浮点数(a), Value::浮点数(b)) => {
                if b < a {
                    min_value = Value::浮点数(b);
                }
            }
            (Value::整数(a), Value::浮点数(b)) => {
                if b < a as f64 {
                    min_value = Value::浮点数(b);
                }
            }
            (Value::浮点数(a), Value::整数(b)) => {
                if (b as f64) < a {
                    min_value = Value::整数(b);
                }
            }
            _ => return Err("最小值函数只支持数字类型".to_string()),
        }
    }
    
    Ok(min_value)
}

/// 计算正弦值
pub fn builtin_数学_正弦(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 1 {
        return Err("正弦函数需要一个参数".to_string());
    }
    
    match &args[0] {
        Value::整数(n) => Ok(Value::浮点数(((*n) as f64).sin())),
        Value::浮点数(f) => Ok(Value::浮点数(f.sin())),
        _ => Err("正弦函数只支持数字类型".to_string()),
    }
}

/// 计算余弦值
pub fn builtin_数学_余弦(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 1 {
        return Err("余弦函数需要一个参数".to_string());
    }
    
    match &args[0] {
        Value::整数(n) => Ok(Value::浮点数(((*n) as f64).cos())),
        Value::浮点数(f) => Ok(Value::浮点数(f.cos())),
        _ => Err("余弦函数只支持数字类型".to_string()),
    }
}

/// 计算正切值
pub fn builtin_数学_正切(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 1 {
        return Err("正切函数需要一个参数".to_string());
    }
    
    match &args[0] {
        Value::整数(n) => Ok(Value::浮点数(((*n) as f64).tan())),
        Value::浮点数(f) => Ok(Value::浮点数(f.tan())),
        _ => Err("正切函数只支持数字类型".to_string()),
    }
}

/// 计算反正弦值
pub fn builtin_数学_反正弦(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 1 {
        return Err("反正弦函数需要一个参数".to_string());
    }
    
    match &args[0] {
        Value::整数(n) => {
            let val = *n as f64;
            if val < -1.0 || val > 1.0 {
                return Err("反正弦函数参数必须在[-1, 1]范围内".to_string());
            }
            Ok(Value::浮点数(val.asin()))
        }
        Value::浮点数(f) => {
            if *f < -1.0 || *f > 1.0 {
                return Err("反正弦函数参数必须在[-1, 1]范围内".to_string());
            }
            Ok(Value::浮点数(f.asin()))
        }
        _ => Err("反正弦函数只支持数字类型".to_string()),
    }
}

/// 计算反余弦值
pub fn builtin_数学_反余弦(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 1 {
        return Err("反余弦函数需要一个参数".to_string());
    }
    
    match &args[0] {
        Value::整数(n) => {
            let val = *n as f64;
            if val < -1.0 || val > 1.0 {
                return Err("反余弦函数参数必须在[-1, 1]范围内".to_string());
            }
            Ok(Value::浮点数(val.acos()))
        }
        Value::浮点数(f) => {
            if *f < -1.0 || *f > 1.0 {
                return Err("反余弦函数参数必须在[-1, 1]范围内".to_string());
            }
            Ok(Value::浮点数(f.acos()))
        }
        _ => Err("反余弦函数只支持数字类型".to_string()),
    }
}

/// 计算反正切值
pub fn builtin_数学_反正切(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 1 {
        return Err("反正切函数需要一个参数".to_string());
    }
    
    match &args[0] {
        Value::整数(n) => Ok(Value::浮点数(((*n) as f64).atan())),
        Value::浮点数(f) => Ok(Value::浮点数(f.atan())),
        _ => Err("反正切函数只支持数字类型".to_string()),
    }
}

/// 计算自然对数
pub fn builtin_数学_自然对数(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 1 {
        return Err("自然对数函数需要一个参数".to_string());
    }
    
    match &args[0] {
        Value::整数(n) => {
            if *n <= 0 {
                return Err("自然对数函数参数必须大于0".to_string());
            }
            Ok(Value::浮点数(((*n) as f64).ln()))
        }
        Value::浮点数(f) => {
            if *f <= 0.0 {
                return Err("自然对数函数参数必须大于0".to_string());
            }
            Ok(Value::浮点数(f.ln()))
        }
        _ => Err("自然对数函数只支持数字类型".to_string()),
    }
}

/// 计算以10为底的对数
pub fn builtin_数学_对数10(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 1 {
        return Err("对数10函数需要一个参数".to_string());
    }
    
    match &args[0] {
        Value::整数(n) => {
            if *n <= 0 {
                return Err("对数10函数参数必须大于0".to_string());
            }
            Ok(Value::浮点数(((*n) as f64).log10()))
        }
        Value::浮点数(f) => {
            if *f <= 0.0 {
                return Err("对数10函数参数必须大于0".to_string());
            }
            Ok(Value::浮点数(f.log10()))
        }
        _ => Err("对数10函数只支持数字类型".to_string()),
    }
}

/// 计算指数函数
pub fn builtin_数学_指数(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 1 {
        return Err("指数函数需要一个参数".to_string());
    }
    
    match &args[0] {
        Value::整数(n) => Ok(Value::浮点数(((*n) as f64).exp())),
        Value::浮点数(f) => Ok(Value::浮点数(f.exp())),
        _ => Err("指数函数只支持数字类型".to_string()),
    }
}

/// 计算幂函数
pub fn builtin_数学_幂(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 2 {
        return Err("幂函数需要两个参数".to_string());
    }
    
    match (&args[0], &args[1]) {
        (Value::整数(base), Value::整数(exponent)) => {
            Ok(Value::整数(base.pow(*exponent as u32)))
        }
        (Value::整数(base), Value::浮点数(exponent)) => {
            Ok(Value::浮点数(((*base) as f64).powf(*exponent)))
        }
        (Value::浮点数(base), Value::整数(exponent)) => {
            Ok(Value::浮点数(base.powf(*exponent as f64)))
        }
        (Value::浮点数(base), Value::浮点数(exponent)) => {
            Ok(Value::浮点数(base.powf(*exponent)))
        }
        _ => Err("幂函数只支持数字类型".to_string()),
    }
}

/// 计算圆周率
pub fn builtin_数学_圆周率(_args: Vec<Value>) -> Result<Value, String> {
    Ok(Value::浮点数(std::f64::consts::PI))
}

/// 计算自然常数e
pub fn builtin_数学_自然常数(_args: Vec<Value>) -> Result<Value, String> {
    Ok(Value::浮点数(std::f64::consts::E))
}

/// 向下取整
pub fn builtin_数学_向下取整(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 1 {
        return Err("向下取整函数需要一个参数".to_string());
    }
    
    match &args[0] {
        Value::整数(n) => Ok(Value::整数(*n)),
        Value::浮点数(f) => Ok(Value::浮点数(f.floor())),
        _ => Err("向下取整函数只支持数字类型".to_string()),
    }
}

/// 向上取整
pub fn builtin_数学_向上取整(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 1 {
        return Err("向上取整函数需要一个参数".to_string());
    }
    
    match &args[0] {
        Value::整数(n) => Ok(Value::整数(*n)),
        Value::浮点数(f) => Ok(Value::浮点数(f.ceil())),
        _ => Err("向上取整函数只支持数字类型".to_string()),
    }
}

/// 四舍五入
pub fn builtin_数学_四舍五入(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 1 {
        return Err("四舍五入函数需要一个参数".to_string());
    }
    
    match &args[0] {
        Value::整数(n) => Ok(Value::整数(*n)),
        Value::浮点数(f) => Ok(Value::浮点数(f.round())),
        _ => Err("四舍五入函数只支持数字类型".to_string()),
    }
}

/// 阶乘
pub fn builtin_数学_阶乘(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 1 {
        return Err("阶乘函数需要一个参数".to_string());
    }
    
    match &args[0] {
        Value::整数(n) => {
            if *n < 0 {
                return Err("阶乘函数参数不能为负数".to_string());
            }
            let mut result = 1;
            for i in 1..=*n {
                result *= i;
            }
            Ok(Value::整数(result))
        }
        _ => Err("阶乘函数只支持整数类型".to_string()),
    }
}

/// 模运算
pub fn builtin_数学_模(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 2 {
        return Err("模运算函数需要两个参数".to_string());
    }
    
    match (&args[0], &args[1]) {
        (Value::整数(a), Value::整数(b)) => {
            if *b == 0 {
                return Err("模运算除数不能为0".to_string());
            }
            Ok(Value::整数(a % b))
        }
        (Value::浮点数(a), Value::浮点数(b)) => {
            if *b == 0.0 {
                return Err("模运算除数不能为0".to_string());
            }
            Ok(Value::浮点数(a % b))
        }
        _ => Err("模运算函数只支持数字类型".to_string()),
    }
}

/// 数学模块函数表
pub fn get_math_functions() -> Vec<(&'static str, fn(Vec<Value>) -> Result<Value, String>)> {
    vec![
        ("绝对值", builtin_数学_绝对值),
        ("平方根", builtin_数学_平方根),
        ("平方", builtin_数学_平方),
        ("立方", builtin_数学_立方),
        ("最大值", builtin_数学_最大值),
        ("最小值", builtin_数学_最小值),
        ("正弦", builtin_数学_正弦),
        ("余弦", builtin_数学_余弦),
        ("正切", builtin_数学_正切),
        ("反正弦", builtin_数学_反正弦),
        ("反余弦", builtin_数学_反余弦),
        ("反正切", builtin_数学_反正切),
        ("自然对数", builtin_数学_自然对数),
        ("对数10", builtin_数学_对数10),
        ("指数", builtin_数学_指数),
        ("幂", builtin_数学_幂),
        ("圆周率", builtin_数学_圆周率),
        ("自然常数", builtin_数学_自然常数),
        ("向下取整", builtin_数学_向下取整),
        ("向上取整", builtin_数学_向上取整),
        ("四舍五入", builtin_数学_四舍五入),
        ("阶乘", builtin_数学_阶乘),
        ("模", builtin_数学_模),
    ]
}
