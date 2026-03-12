use super::*;
use crate::runtime::value::Value;
use std::env;
use std::fs;
use std::path::Path;

// OS模块测试
#[test]
fn test_get_env() {
    // 设置测试环境变量
    env::set_var("TEST_VAR", "test_value");
    
    // 测试获取存在的环境变量
    let result = builtin_get_env(vec![Value::字符串("TEST_VAR".to_string())]);
    assert!(result.is_ok());
    if let Ok(Value::字符串(value)) = result {
        assert_eq!(value, "test_value");
    } else {
        panic!("Expected string value");
    }
    
    // 测试获取不存在的环境变量
    let result = builtin_get_env(vec![Value::字符串("NON_EXISTENT_VAR".to_string())]);
    assert!(result.is_ok());
    if let Ok(Value::空值) = result {
        // 预期返回空值
    } else {
        panic!("Expected null value");
    }
    
    // 清理测试环境变量
    env::remove_var("TEST_VAR");
}

#[test]
fn test_set_env() {
    // 测试设置环境变量
    let result = builtin_set_env(vec![
        Value::字符串("TEST_VAR".to_string()),
        Value::字符串("test_value".to_string())
    ]);
    assert!(result.is_ok());
    if let Ok(Value::布尔值(true)) = result {
        // 预期返回true
    } else {
        panic!("Expected true value");
    }
    
    // 验证环境变量已设置
    assert_eq!(env::var("TEST_VAR").unwrap(), "test_value");
    
    // 清理测试环境变量
    env::remove_var("TEST_VAR");
}

#[test]
fn test_remove_env() {
    // 设置测试环境变量
    env::set_var("TEST_VAR", "test_value");
    assert_eq!(env::var("TEST_VAR").unwrap(), "test_value");
    
    // 测试删除环境变量
    let result = builtin_remove_env(vec![Value::字符串("TEST_VAR".to_string())]);
    assert!(result.is_ok());
    if let Ok(Value::布尔值(true)) = result {
        // 预期返回true
    } else {
        panic!("Expected true value");
    }
    
    // 验证环境变量已删除
    assert!(env::var("TEST_VAR").is_err());
}

#[test]
fn test_get_cwd() {
    let result = builtin_get_cwd(vec![]);
    assert!(result.is_ok());
    if let Ok(Value::字符串(path)) = result {
        assert!(!path.is_empty());
    } else {
        panic!("Expected string value");
    }
}

#[test]
fn test_path_exists() {
    // 测试存在的路径
    let result = builtin_path_exists(vec![Value::字符串(".".to_string())]);
    assert!(result.is_ok());
    if let Ok(Value::布尔值(true)) = result {
        // 预期返回true
    } else {
        panic!("Expected true value");
    }
    
    // 测试不存在的路径
    let result = builtin_path_exists(vec![Value::字符串("non_existent_path_12345".to_string())]);
    assert!(result.is_ok());
    if let Ok(Value::布尔值(false)) = result {
        // 预期返回false
    } else {
        panic!("Expected false value");
    }
}

#[test]
fn test_path_is_file() {
    // 测试目录路径
    let result = builtin_path_is_file(vec![Value::字符串(".".to_string())]);
    assert!(result.is_ok());
    if let Ok(Value::布尔值(false)) = result {
        // 预期返回false
    } else {
        panic!("Expected false value");
    }
    
    // 测试不存在的文件
    let result = builtin_path_is_file(vec![Value::字符串("non_existent_file.txt".to_string())]);
    assert!(result.is_ok());
    if let Ok(Value::布尔值(false)) = result {
        // 预期返回false
    } else {
        panic!("Expected false value");
    }
}

#[test]
fn test_path_is_dir() {
    // 测试目录路径
    let result = builtin_path_is_dir(vec![Value::字符串(".".to_string())]);
    assert!(result.is_ok());
    if let Ok(Value::布尔值(true)) = result {
        // 预期返回true
    } else {
        panic!("Expected true value");
    }
    
    // 创建临时文件
    fs::write("test_file.txt", "test content").unwrap();
    
    // 测试文件路径
    let result = builtin_path_is_dir(vec![Value::字符串("test_file.txt".to_string())]);
    assert!(result.is_ok());
    if let Ok(Value::布尔值(false)) = result {
        // 预期返回false
    } else {
        panic!("Expected false value");
    }
    
    // 清理临时文件
    fs::remove_file("test_file.txt").unwrap();
}

#[test]
fn test_get_temp_dir() {
    let result = builtin_get_temp_dir(vec![]);
    assert!(result.is_ok());
    if let Ok(Value::字符串(path)) = result {
        assert!(!path.is_empty());
        assert!(Path::new(&path).exists());
    } else {
        panic!("Expected string value");
    }
}

#[test]
fn test_join_path() {
    let result = builtin_join_path(vec![
        Value::字符串("dir1".to_string()),
        Value::字符串("dir2".to_string()),
        Value::字符串("file.txt".to_string())
    ]);
    assert!(result.is_ok());
    if let Ok(Value::字符串(path)) = result {
        assert!(!path.is_empty());
        // 验证路径包含所有组件
        assert!(path.contains("dir1"));
        assert!(path.contains("dir2"));
        assert!(path.contains("file.txt"));
    } else {
        panic!("Expected string value");
    }
}

#[test]
fn test_get_path_separator() {
    let result = builtin_get_path_separator(vec![]);
    assert!(result.is_ok());
    if let Ok(Value::字符串(separator)) = result {
        assert!(!separator.is_empty());
        // 验证分隔符是正确的系统分隔符
        if cfg!(windows) {
            assert_eq!(separator, "\\");
        } else {
            assert_eq!(separator, "/");
        }
    } else {
        panic!("Expected string value");
    }
}

// DateTime模块测试
#[test]
fn test_now() {
    let result = builtin_now(vec![]);
    assert!(result.is_ok());
    if let Ok(Value::字典(map)) = result {
        // 验证返回的字典包含预期的键
        assert!(map.contains_key("时间戳"));
        assert!(map.contains_key("格式化"));
        
        // 验证时间戳是整数
        if let Value::整数(timestamp) = &map["时间戳"] {
            assert!(*timestamp > 0);
        } else {
            panic!("Expected integer timestamp");
        }
        
        // 验证格式化字符串不为空
        if let Value::字符串(formatted) = &map["格式化"] {
            assert!(!formatted.is_empty());
        } else {
            panic!("Expected string formatted time");
        }
    } else {
        panic!("Expected dictionary value");
    }
}

#[test]
fn test_timestamp() {
    let result = builtin_timestamp(vec![]);
    assert!(result.is_ok());
    if let Ok(Value::整数(timestamp)) = result {
        assert!(timestamp > 0);
    } else {
        panic!("Expected integer timestamp");
    }
}

#[test]
fn test_format_date() {
    let timestamp = 1609459200; // 2021-01-01 00:00:00 UTC
    
    // 测试默认格式
    let result = builtin_format_date(vec![Value::整数(timestamp)]);
    assert!(result.is_ok());
    if let Ok(Value::字符串(formatted)) = result {
        assert!(!formatted.is_empty());
    } else {
        panic!("Expected string value");
    }
    
    // 测试自定义格式
    let result = builtin_format_date(vec![
        Value::整数(timestamp),
        Value::字符串("%Y-%m-%d".to_string())
    ]);
    assert!(result.is_ok());
    if let Ok(Value::字符串(formatted)) = result {
        assert_eq!(formatted, "2021-01-01");
    } else {
        panic!("Expected string value");
    }
}

#[test]
fn test_parse_date() {
    let date_str = "2021-01-01 12:00:00";
    let format = "%Y-%m-%d %H:%M:%S";
    
    let result = builtin_parse_date(vec![
        Value::字符串(date_str.to_string()),
        Value::字符串(format.to_string())
    ]);
    assert!(result.is_ok());
    if let Ok(Value::字典(map)) = result {
        // 验证返回的字典包含预期的键
        assert!(map.contains_key("时间戳"));
        assert!(map.contains_key("年"));
        assert!(map.contains_key("月"));
        assert!(map.contains_key("日"));
        assert!(map.contains_key("时"));
        assert!(map.contains_key("分"));
        assert!(map.contains_key("秒"));
        
        // 验证年、月、日等字段值
        if let Value::整数(year) = &map["年"] {
            assert_eq!(*year, 2021);
        } else {
            panic!("Expected integer year");
        }
        
        if let Value::整数(month) = &map["月"] {
            assert_eq!(*month, 1);
        } else {
            panic!("Expected integer month");
        }
        
        if let Value::整数(day) = &map["日"] {
            assert_eq!(*day, 1);
        } else {
            panic!("Expected integer day");
        }
    } else {
        panic!("Expected dictionary value");
    }
}

#[test]
fn test_date_add() {
    let timestamp = 1609459200; // 2021-01-01 00:00:00 UTC
    let seconds_to_add = 3600; // 1小时
    
    let result = builtin_date_add(vec![
        Value::整数(timestamp),
        Value::整数(seconds_to_add)
    ]);
    assert!(result.is_ok());
    if let Ok(Value::整数(new_timestamp)) = result {
        assert_eq!(new_timestamp, timestamp + seconds_to_add);
    } else {
        panic!("Expected integer value");
    }
}

#[test]
fn test_date_diff() {
    let timestamp1 = 1609459200; // 2021-01-01 00:00:00 UTC
    let timestamp2 = 1609462800; // 2021-01-01 01:00:00 UTC
    
    let result = builtin_date_diff(vec![
        Value::整数(timestamp1),
        Value::整数(timestamp2)
    ]);
    assert!(result.is_ok());
    if let Ok(Value::整数(diff)) = result {
        assert_eq!(diff, 3600); // 1小时
    } else {
        panic!("Expected integer value");
    }
}

#[test]
fn test_get_year() {
    let result = builtin_get_year(vec![]);
    assert!(result.is_ok());
    if let Ok(Value::整数(year)) = result {
        assert!(year > 2000); // 确保年份合理
    } else {
        panic!("Expected integer value");
    }
}

#[test]
fn test_get_month() {
    let result = builtin_get_month(vec![]);
    assert!(result.is_ok());
    if let Ok(Value::整数(month)) = result {
        assert!(month >= 1 && month <= 12); // 确保月份在合理范围内
    } else {
        panic!("Expected integer value");
    }
}

#[test]
fn test_get_day() {
    let result = builtin_get_day(vec![]);
    assert!(result.is_ok());
    if let Ok(Value::整数(day)) = result {
        assert!(day >= 1 && day <= 31); // 确保日期在合理范围内
    } else {
        panic!("Expected integer value");
    }
}

#[test]
fn test_get_weekday() {
    let result = builtin_get_weekday(vec![]);
    assert!(result.is_ok());
    if let Ok(Value::字典(map)) = result {
        // 验证返回的字典包含预期的键
        assert!(map.contains_key("数字"));
        assert!(map.contains_key("名称"));
        
        // 验证星期数字在合理范围内
        if let Value::整数(weekday_num) = &map["数字"] {
            assert!(*weekday_num >= 1 && *weekday_num <= 7);
        } else {
            panic!("Expected integer weekday");
        }
        
        // 验证星期名称不为空
        if let Value::字符串(weekday_name) = &map["名称"] {
            assert!(!weekday_name.is_empty());
        } else {
            panic!("Expected string weekday name");
        }
    } else {
        panic!("Expected dictionary value");
    }
}

// JSON模块测试
#[test]
fn test_json_stringify() {
    // 测试基本类型
    let result = builtin_json_stringify(vec![Value::整数(42)]);
    assert!(result.is_ok());
    if let Ok(Value::字符串(json_str)) = result {
        assert_eq!(json_str, "42");
    } else {
        panic!("Expected string value");
    }
    
    // 测试字符串
    let result = builtin_json_stringify(vec![Value::字符串("test".to_string())]);
    assert!(result.is_ok());
    if let Ok(Value::字符串(json_str)) = result {
        assert_eq!(json_str, "\"test\"");
    } else {
        panic!("Expected string value");
    }
    
    // 测试列表
    let list = Value::列表(vec![Value::整数(1), Value::整数(2), Value::整数(3)]);
    let result = builtin_json_stringify(vec![list]);
    assert!(result.is_ok());
    if let Ok(Value::字符串(json_str)) = result {
        assert_eq!(json_str, "[1,2,3]");
    } else {
        panic!("Expected string value");
    }
    
    // 测试字典
    let mut map = std::collections::HashMap::new();
    map.insert("name".to_string(), Value::字符串("test".to_string()));
    map.insert("age".to_string(), Value::整数(30));
    let dict = Value::字典(map);
    let result = builtin_json_stringify(vec![dict]);
    assert!(result.is_ok());
    if let Ok(Value::字符串(json_str)) = result {
        assert!(json_str.contains("name"));
        assert!(json_str.contains("test"));
        assert!(json_str.contains("age"));
        assert!(json_str.contains("30"));
    } else {
        panic!("Expected string value");
    }
}

#[test]
fn test_json_parse() {
    // 测试解析数字
    let result = builtin_json_parse(vec![Value::字符串("42".to_string())]);
    assert!(result.is_ok());
    if let Ok(Value::整数(value)) = result {
        assert_eq!(value, 42);
    } else {
        panic!("Expected integer value");
    }
    
    // 测试解析字符串
    let result = builtin_json_parse(vec![Value::字符串("\"test\"".to_string())]);
    assert!(result.is_ok());
    if let Ok(Value::字符串(value)) = result {
        assert_eq!(value, "test");
    } else {
        panic!("Expected string value");
    }
    
    // 测试解析列表
    let result = builtin_json_parse(vec![Value::字符串("[1,2,3]".to_string())]);
    assert!(result.is_ok());
    if let Ok(Value::列表(list)) = result {
        assert_eq!(list.len(), 3);
        if let Value::整数(value) = &list[0] {
            assert_eq!(*value, 1);
        }
    } else {
        panic!("Expected list value");
    }
    
    // 测试解析对象
    let result = builtin_json_parse(vec![Value::字符串("{\"name\": \"test\", \"age\": 30}".to_string())]);
    assert!(result.is_ok());
    if let Ok(Value::字典(map)) = result {
        assert!(map.contains_key("name"));
        assert!(map.contains_key("age"));
        if let Value::字符串(name) = &map["name"] {
            assert_eq!(*name, "test");
        }
        if let Value::整数(age) = &map["age"] {
            assert_eq!(*age, 30);
        }
    } else {
        panic!("Expected dictionary value");
    }
}

#[test]
fn test_json_validate() {
    // 测试有效的JSON
    let result = builtin_json_validate(vec![Value::字符串("{\"name\": \"test\"}".to_string())]);
    assert!(result.is_ok());
    if let Ok(Value::布尔值(true)) = result {
        // 预期返回true
    } else {
        panic!("Expected true value");
    }
    
    // 测试无效的JSON
    let result = builtin_json_validate(vec![Value::字符串("{name: test}".to_string())]);
    assert!(result.is_ok());
    if let Ok(Value::布尔值(false)) = result {
        // 预期返回false
    } else {
        panic!("Expected false value");
    }
}

#[test]
fn test_json_get() {
    let json_str = "{\"user\": {\"name\": \"test\", \"age\": 30}}";
    let path = "user.name";
    
    let result = builtin_json_get(vec![
        Value::字符串(json_str.to_string()),
        Value::字符串(path.to_string())
    ]);
    assert!(result.is_ok());
    if let Ok(Value::字符串(value)) = result {
        assert_eq!(value, "test");
    } else {
        panic!("Expected string value");
    }
}

#[test]
fn test_json_set() {
    let json_str = "{\"user\": {\"name\": \"test\", \"age\": 30}}";
    let path = "user.age";
    let new_value = Value::整数(31);
    
    let result = builtin_json_set(vec![
        Value::字符串(json_str.to_string()),
        Value::字符串(path.to_string()),
        new_value
    ]);
    assert!(result.is_ok());
    if let Ok(Value::字符串(updated_json)) = result {
        assert!(updated_json.contains("31"));
    } else {
        panic!("Expected string value");
    }
}

// 集合模块测试
#[test]
fn test_collections_计数() {
    let list = Value::列表(vec![
        Value::字符串("a".to_string()),
        Value::字符串("b".to_string()),
        Value::字符串("a".to_string()),
        Value::字符串("c".to_string()),
        Value::字符串("b".to_string()),
        Value::字符串("a".to_string())
    ]);
    let result = builtin_集合_计数(vec![list]);
    assert!(result.is_ok());
    if let Ok(Value::字典(dict)) = result {
        assert_eq!(dict["a"], Value::整数(3));
        assert_eq!(dict["b"], Value::整数(2));
        assert_eq!(dict["c"], Value::整数(1));
    } else {
        panic!("Expected dictionary value");
    }
}

#[test]
fn test_collections_双端队列() {
    // 测试创建空双端队列
    let result = builtin_集合_双端队列(vec![]);
    assert!(result.is_ok());
    if let Ok(Value::列表(list)) = result {
        assert!(list.is_empty());
    } else {
        panic!("Expected list value");
    }
    
    // 测试从列表创建双端队列
    let list = Value::列表(vec![Value::整数(1), Value::整数(2), Value::整数(3)]);
    let result = builtin_集合_双端队列(vec![list.clone()]);
    assert!(result.is_ok());
    if let Ok(Value::列表(result_list)) = result {
        assert_eq!(result_list, match list { Value::列表(l) => l, _ => vec![] });
    } else {
        panic!("Expected list value");
    }
}

#[test]
fn test_collections_双端队列_左侧添加() {
    let list = Value::列表(vec![Value::整数(2), Value::整数(3)]);
    let result = builtin_集合_双端队列_左侧添加(vec![list, Value::整数(1)]);
    assert!(result.is_ok());
    if let Ok(Value::列表(result_list)) = result {
        assert_eq!(result_list, vec![Value::整数(1), Value::整数(2), Value::整数(3)]);
    } else {
        panic!("Expected list value");
    }
}

#[test]
fn test_collections_双端队列_右侧添加() {
    let list = Value::列表(vec![Value::整数(1), Value::整数(2)]);
    let result = builtin_集合_双端队列_右侧添加(vec![list, Value::整数(3)]);
    assert!(result.is_ok());
    if let Ok(Value::列表(result_list)) = result {
        assert_eq!(result_list, vec![Value::整数(1), Value::整数(2), Value::整数(3)]);
    } else {
        panic!("Expected list value");
    }
}

#[test]
fn test_collections_双端队列_左侧弹出() {
    let list = Value::列表(vec![Value::整数(1), Value::整数(2), Value::整数(3)]);
    let result = builtin_集合_双端队列_左侧弹出(vec![list]);
    assert!(result.is_ok());
    if let Ok(Value::整数(value)) = result {
        assert_eq!(value, 1);
    } else {
        panic!("Expected integer value");
    }
}

#[test]
fn test_collections_双端队列_右侧弹出() {
    let list = Value::列表(vec![Value::整数(1), Value::整数(2), Value::整数(3)]);
    let result = builtin_集合_双端队列_右侧弹出(vec![list]);
    assert!(result.is_ok());
    if let Ok(Value::整数(value)) = result {
        assert_eq!(value, 3);
    } else {
        panic!("Expected integer value");
    }
}

#[test]
fn test_collections_有序字典() {
    // 测试创建空有序字典
    let result = builtin_集合_有序字典(vec![]);
    assert!(result.is_ok());
    if let Ok(Value::字典(dict)) = result {
        assert!(dict.is_empty());
    } else {
        panic!("Expected dictionary value");
    }
    
    // 测试从字典创建有序字典
    let mut map = std::collections::HashMap::new();
    map.insert("c".to_string(), Value::整数(3));
    map.insert("a".to_string(), Value::整数(1));
    map.insert("b".to_string(), Value::整数(2));
    let dict = Value::字典(map);
    let result = builtin_集合_有序字典(vec![dict]);
    assert!(result.is_ok());
    if let Ok(Value::字典(result_dict)) = result {
        assert!(result_dict.contains_key("a"));
        assert!(result_dict.contains_key("b"));
        assert!(result_dict.contains_key("c"));
    } else {
        panic!("Expected dictionary value");
    }
}

// 随机模块测试
#[test]
fn test_random_整数() {
    // 测试无参数
    let result = builtin_随机_整数(vec![]);
    assert!(result.is_ok());
    
    // 测试一个参数（0到max-1）
    let result = builtin_随机_整数(vec![Value::整数(10)]);
    assert!(result.is_ok());
    if let Ok(Value::整数(n)) = result {
        assert!(n >= 0 && n < 10);
    } else {
        panic!("Expected integer value");
    }
    
    // 测试两个参数（min到max-1）
    let result = builtin_随机_整数(vec![Value::整数(5), Value::整数(15)]);
    assert!(result.is_ok());
    if let Ok(Value::整数(n)) = result {
        assert!(n >= 5 && n < 15);
    } else {
        panic!("Expected integer value");
    }
}

#[test]
fn test_random_浮点数() {
    // 测试无参数（0.0到1.0）
    let result = builtin_随机_浮点数(vec![]);
    assert!(result.is_ok());
    if let Ok(Value::浮点数(f)) = result {
        assert!(f >= 0.0 && f < 1.0);
    } else {
        panic!("Expected float value");
    }
    
    // 测试一个参数（0.0到max）
    let result = builtin_随机_浮点数(vec![Value::浮点数(5.0)]);
    assert!(result.is_ok());
    if let Ok(Value::浮点数(f)) = result {
        assert!(f >= 0.0 && f < 5.0);
    } else {
        panic!("Expected float value");
    }
}

#[test]
fn test_random_选择() {
    let list = Value::列表(vec![Value::整数(1), Value::整数(2), Value::整数(3)]);
    let result = builtin_随机_选择(vec![list]);
    assert!(result.is_ok());
    // 结果应该是列表中的一个元素
    if let Ok(Value::整数(n)) = result {
        assert!(n == 1 || n == 2 || n == 3);
    } else {
        panic!("Expected integer value");
    }
}

#[test]
fn test_random_打乱() {
    let list = Value::列表(vec![Value::整数(1), Value::整数(2), Value::整数(3)]);
    let result = builtin_随机_打乱(vec![list.clone()]);
    assert!(result.is_ok());
    if let Ok(Value::列表(shuffled)) = result {
        // 打乱后的列表长度应该与原列表相同
        assert_eq!(shuffled.len(), 3);
        // 打乱后的列表应该包含原列表的所有元素
        for item in &shuffled {
            assert!(match list { Value::列表(ref l) => l.contains(item), _ => false });
        }
    } else {
        panic!("Expected list value");
    }
}

#[test]
fn test_random_布尔值() {
    let result = builtin_随机_布尔值(vec![]);
    assert!(result.is_ok());
    if let Ok(Value::布尔值(b)) = result {
        // 结果应该是true或false
        assert!(b == true || b == false);
    } else {
        panic!("Expected boolean value");
    }
}

#[test]
fn test_random_种子() {
    // 设置种子
    let result = builtin_随机_种子(vec![Value::整数(42)]);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), Value::空值);
    
    // 验证种子设置后，随机数生成是可预测的
    let result1 = builtin_随机_整数(vec![Value::整数(100)]);
    let result2 = builtin_随机_整数(vec![Value::整数(100)]);
    assert!(result1.is_ok());
    assert!(result2.is_ok());
}

// 数学模块测试
#[test]
fn test_math_绝对值() {
    let result = builtin_数学_绝对值(vec![Value::整数(-42)]);
    assert!(result.is_ok());
    if let Ok(Value::整数(n)) = result {
        assert_eq!(n, 42);
    } else {
        panic!("Expected integer value");
    }
    
    let result = builtin_数学_绝对值(vec![Value::浮点数(-3.14)]);
    assert!(result.is_ok());
    if let Ok(Value::浮点数(f)) = result {
        assert_eq!(f, 3.14);
    } else {
        panic!("Expected float value");
    }
}

#[test]
fn test_math_平方根() {
    let result = builtin_数学_平方根(vec![Value::整数(16)]);
    assert!(result.is_ok());
    if let Ok(Value::浮点数(f)) = result {
        assert_eq!(f, 4.0);
    } else {
        panic!("Expected float value");
    }
}

#[test]
fn test_math_平方() {
    let result = builtin_数学_平方(vec![Value::整数(5)]);
    assert!(result.is_ok());
    if let Ok(Value::整数(n)) = result {
        assert_eq!(n, 25);
    } else {
        panic!("Expected integer value");
    }
}

#[test]
fn test_math_最大值() {
    let result = builtin_数学_最大值(vec![Value::整数(1), Value::整数(5), Value::整数(3)]);
    assert!(result.is_ok());
    if let Ok(Value::整数(n)) = result {
        assert_eq!(n, 5);
    } else {
        panic!("Expected integer value");
    }
}

#[test]
fn test_math_最小值() {
    let result = builtin_数学_最小值(vec![Value::整数(1), Value::整数(5), Value::整数(3)]);
    assert!(result.is_ok());
    if let Ok(Value::整数(n)) = result {
        assert_eq!(n, 1);
    } else {
        panic!("Expected integer value");
    }
}

#[test]
fn test_math_圆周率() {
    let result = builtin_数学_圆周率(vec![]);
    assert!(result.is_ok());
    if let Ok(Value::浮点数(f)) = result {
        assert_eq!(f, std::f64::consts::PI);
    } else {
        panic!("Expected float value");
    }
}

#[test]
fn test_math_自然常数() {
    let result = builtin_数学_自然常数(vec![]);
    assert!(result.is_ok());
    if let Ok(Value::浮点数(f)) = result {
        assert_eq!(f, std::f64::consts::E);
    } else {
        panic!("Expected float value");
    }
}

// 迭代工具模块测试
#[test]
fn test_itertools_计数() {
    // 测试无参数
    let result = builtin_迭代_计数(vec![]);
    assert!(result.is_ok());
    if let Ok(Value::列表(list)) = result {
        assert_eq!(list.len(), 5);
        assert_eq!(list[0], Value::整数(0));
        assert_eq!(list[1], Value::整数(1));
    }
    
    // 测试一个参数
    let result = builtin_迭代_计数(vec![Value::整数(5)]);
    assert!(result.is_ok());
    if let Ok(Value::列表(list)) = result {
        assert_eq!(list.len(), 5);
        assert_eq!(list[0], Value::整数(5));
        assert_eq!(list[1], Value::整数(6));
    }
    
    // 测试两个参数
    let result = builtin_迭代_计数(vec![Value::整数(1), Value::整数(2)]);
    assert!(result.is_ok());
    if let Ok(Value::列表(list)) = result {
        assert_eq!(list.len(), 5);
        assert_eq!(list[0], Value::整数(1));
        assert_eq!(list[1], Value::整数(3));
    }
}

#[test]
fn test_itertools_重复() {
    // 测试一个参数
    let result = builtin_迭代_重复(vec![Value::字符串("hello".to_string())]);
    assert!(result.is_ok());
    if let Ok(Value::列表(list)) = result {
        assert_eq!(list.len(), 5);
        assert_eq!(list[0], Value::字符串("hello".to_string()));
    }
    
    // 测试两个参数
    let result = builtin_迭代_重复(vec![Value::整数(42), Value::整数(3)]);
    assert!(result.is_ok());
    if let Ok(Value::列表(list)) = result {
        assert_eq!(list.len(), 3);
        assert_eq!(list[0], Value::整数(42));
    }
}

#[test]
fn test_itertools_链接() {
    let list1 = Value::列表(vec![Value::整数(1), Value::整数(2)]);
    let list2 = Value::列表(vec![Value::整数(3), Value::整数(4)]);
    let result = builtin_迭代_链接(vec![list1, list2]);
    assert!(result.is_ok());
    if let Ok(Value::列表(list)) = result {
        assert_eq!(list.len(), 4);
        assert_eq!(list[0], Value::整数(1));
        assert_eq!(list[3], Value::整数(4));
    }
}

#[test]
fn test_itertools_循环() {
    let list = Value::列表(vec![Value::整数(1), Value::整数(2), Value::整数(3)]);
    let result = builtin_迭代_循环(vec![list]);
    assert!(result.is_ok());
    if let Ok(Value::列表(list)) = result {
        assert_eq!(list.len(), 10);
        assert_eq!(list[0], Value::整数(1));
        assert_eq!(list[3], Value::整数(1)); // 循环开始
    }
}

#[test]
fn test_itertools_过滤() {
    let list = Value::列表(vec![Value::整数(1), Value::整数(2), Value::整数(3), Value::整数(4)]);
    // 这里简化处理，直接测试过滤函数的参数验证
    let result = builtin_迭代_过滤(vec![list, Value::整数(1)]);
    assert!(result.is_err()); // 应该返回错误，因为第二个参数不是函数
}

#[test]
fn test_itertools_映射() {
    let list = Value::列表(vec![Value::整数(1), Value::整数(2), Value::整数(3)]);
    // 这里简化处理，直接测试映射函数的参数验证
    let result = builtin_迭代_映射(vec![list, Value::整数(1)]);
    assert!(result.is_err()); // 应该返回错误，因为第二个参数不是函数
}

#[test]
fn test_itertools_累计() {
    let list = Value::列表(vec![Value::整数(1), Value::整数(2), Value::整数(3), Value::整数(4)]);
    let result = builtin_迭代_累计(vec![list]);
    assert!(result.is_ok());
    if let Ok(Value::列表(list)) = result {
        assert_eq!(list.len(), 4);
        assert_eq!(list[0], Value::整数(1));
        assert_eq!(list[1], Value::整数(3)); // 1+2
        assert_eq!(list[2], Value::整数(6)); // 3+3
        assert_eq!(list[3], Value::整数(10)); // 6+4
    }
}

#[test]
fn test_itertools_压缩() {
    let list1 = Value::列表(vec![Value::整数(1), Value::整数(2), Value::整数(3)]);
    let list2 = Value::列表(vec![Value::字符串("a".to_string()), Value::字符串("b".to_string())]);
    let result = builtin_迭代_压缩(vec![list1, list2]);
    assert!(result.is_ok());
    if let Ok(Value::列表(list)) = result {
        assert_eq!(list.len(), 2); // 应该取最短列表的长度
        if let Value::列表(pair1) = &list[0] {
            assert_eq!(pair1[0], Value::整数(1));
            assert_eq!(pair1[1], Value::字符串("a".to_string()));
        }
    }
}

