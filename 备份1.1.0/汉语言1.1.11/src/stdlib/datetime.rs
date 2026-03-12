use crate::error::{RuntimeError, RuntimeResult};
use crate::runtime::value::Value;
use chrono::{Datelike, Timelike};
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

fn system_time_to_timestamp(time: SystemTime) -> i64 {
    time.duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs() as i64
}

fn timestamp_to_system_time(timestamp: i64) -> SystemTime {
    UNIX_EPOCH + std::time::Duration::from_secs(timestamp as u64)
}

fn format_time(time: SystemTime, format: &str) -> String {
    let datetime: chrono::DateTime<chrono::Local> = time.into();
    datetime.format(format).to_string()
}

pub fn builtin_now(_args: Vec<Value>) -> RuntimeResult<Value> {
    let now = SystemTime::now();
    let timestamp = system_time_to_timestamp(now);
    
    let mut result = HashMap::new();
    result.insert("时间戳".to_string(), Value::整数(timestamp));
    result.insert("格式化".to_string(), Value::字符串(format_time(now, "%Y-%m-%d %H:%M:%S")));
    
    Ok(Value::字典(result))
}

pub fn builtin_timestamp(_args: Vec<Value>) -> RuntimeResult<Value> {
    let timestamp = system_time_to_timestamp(SystemTime::now());
    Ok(Value::整数(timestamp))
}

pub fn builtin_format_date(args: Vec<Value>) -> RuntimeResult<Value> {
    if args.is_empty() {
        return Err(RuntimeError::ArgumentCountError {
            function: "格式化日期".to_string(),
            expected: 1,
            actual: 0,
        });
    }

    let timestamp = match &args[0] {
        Value::整数(n) => *n,
        Value::浮点数(n) => *n as i64,
        _ => return Err(RuntimeError::TypeError("时间戳必须是数字".to_string())),
    };

    let format = if args.len() > 1 {
        match &args[1] {
            Value::字符串(s) => s.as_str(),
            _ => "%Y-%m-%d %H:%M:%S",
        }
    } else {
        "%Y-%m-%d %H:%M:%S"
    };

    let time = timestamp_to_system_time(timestamp);
    let formatted = format_time(time, format);
    Ok(Value::字符串(formatted))
}

pub fn builtin_parse_date(args: Vec<Value>) -> RuntimeResult<Value> {
    if args.len() < 2 {
        return Err(RuntimeError::ArgumentCountError {
            function: "解析日期".to_string(),
            expected: 2,
            actual: args.len(),
        });
    }

    let date_str = match &args[0] {
        Value::字符串(s) => s,
        _ => return Err(RuntimeError::TypeError("日期字符串必须是字符串".to_string())),
    };

    let format = match &args[1] {
        Value::字符串(s) => s,
        _ => return Err(RuntimeError::TypeError("格式必须是字符串".to_string())),
    };

    match chrono::NaiveDateTime::parse_from_str(date_str, format) {
        Ok(datetime) => {
            let timestamp = datetime.and_utc().timestamp();
            let mut result = HashMap::new();
            result.insert("时间戳".to_string(), Value::整数(timestamp));
            result.insert("年".to_string(), Value::整数(datetime.year() as i64));
            result.insert("月".to_string(), Value::整数(datetime.month() as i64));
            result.insert("日".to_string(), Value::整数(datetime.day() as i64));
            result.insert("时".to_string(), Value::整数(datetime.hour() as i64));
            result.insert("分".to_string(), Value::整数(datetime.minute() as i64));
            result.insert("秒".to_string(), Value::整数(datetime.second() as i64));
            Ok(Value::字典(result))
        }
        Err(e) => Err(RuntimeError::General(format!("日期解析失败: {}", e))),
    }
}

pub fn builtin_date_add(args: Vec<Value>) -> RuntimeResult<Value> {
    if args.len() < 2 {
        return Err(RuntimeError::ArgumentCountError {
            function: "日期加法".to_string(),
            expected: 2,
            actual: args.len(),
        });
    }

    let timestamp = match &args[0] {
        Value::整数(n) => *n,
        Value::浮点数(n) => *n as i64,
        _ => return Err(RuntimeError::TypeError("时间戳必须是数字".to_string())),
    };

    let seconds = match &args[1] {
        Value::整数(n) => *n,
        Value::浮点数(n) => *n as i64,
        _ => return Err(RuntimeError::TypeError("秒数必须是数字".to_string())),
    };

    let new_timestamp = timestamp + seconds;
    Ok(Value::整数(new_timestamp))
}

pub fn builtin_date_diff(args: Vec<Value>) -> RuntimeResult<Value> {
    if args.len() < 2 {
        return Err(RuntimeError::ArgumentCountError {
            function: "日期差值".to_string(),
            expected: 2,
            actual: args.len(),
        });
    }

    let timestamp1 = match &args[0] {
        Value::整数(n) => *n,
        Value::浮点数(n) => *n as i64,
        _ => return Err(RuntimeError::TypeError("时间戳必须是数字".to_string())),
    };

    let timestamp2 = match &args[1] {
        Value::整数(n) => *n,
        Value::浮点数(n) => *n as i64,
        _ => return Err(RuntimeError::TypeError("时间戳必须是数字".to_string())),
    };

    let diff = timestamp2 - timestamp1;
    Ok(Value::整数(diff))
}

pub fn builtin_get_year(args: Vec<Value>) -> RuntimeResult<Value> {
    let timestamp = if args.is_empty() {
        system_time_to_timestamp(SystemTime::now())
    } else {
        match &args[0] {
            Value::整数(n) => *n,
            Value::浮点数(n) => *n as i64,
            _ => return Err(RuntimeError::TypeError("时间戳必须是数字".to_string())),
        }
    };

    let time = timestamp_to_system_time(timestamp);
    let datetime: chrono::DateTime<chrono::Local> = time.into();
    Ok(Value::整数(datetime.year() as i64))
}

pub fn builtin_get_month(args: Vec<Value>) -> RuntimeResult<Value> {
    let timestamp = if args.is_empty() {
        system_time_to_timestamp(SystemTime::now())
    } else {
        match &args[0] {
            Value::整数(n) => *n,
            Value::浮点数(n) => *n as i64,
            _ => return Err(RuntimeError::TypeError("时间戳必须是数字".to_string())),
        }
    };

    let time = timestamp_to_system_time(timestamp);
    let datetime: chrono::DateTime<chrono::Local> = time.into();
    Ok(Value::整数(datetime.month() as i64))
}

pub fn builtin_get_day(args: Vec<Value>) -> RuntimeResult<Value> {
    let timestamp = if args.is_empty() {
        system_time_to_timestamp(SystemTime::now())
    } else {
        match &args[0] {
            Value::整数(n) => *n,
            Value::浮点数(n) => *n as i64,
            _ => return Err(RuntimeError::TypeError("时间戳必须是数字".to_string())),
        }
    };

    let time = timestamp_to_system_time(timestamp);
    let datetime: chrono::DateTime<chrono::Local> = time.into();
    Ok(Value::整数(datetime.day() as i64))
}

pub fn builtin_get_hour(args: Vec<Value>) -> RuntimeResult<Value> {
    let timestamp = if args.is_empty() {
        system_time_to_timestamp(SystemTime::now())
    } else {
        match &args[0] {
            Value::整数(n) => *n,
            Value::浮点数(n) => *n as i64,
            _ => return Err(RuntimeError::TypeError("时间戳必须是数字".to_string())),
        }
    };

    let time = timestamp_to_system_time(timestamp);
    let datetime: chrono::DateTime<chrono::Local> = time.into();
    Ok(Value::整数(datetime.hour() as i64))
}

pub fn builtin_get_minute(args: Vec<Value>) -> RuntimeResult<Value> {
    let timestamp = if args.is_empty() {
        system_time_to_timestamp(SystemTime::now())
    } else {
        match &args[0] {
            Value::整数(n) => *n,
            Value::浮点数(n) => *n as i64,
            _ => return Err(RuntimeError::TypeError("时间戳必须是数字".to_string())),
        }
    };

    let time = timestamp_to_system_time(timestamp);
    let datetime: chrono::DateTime<chrono::Local> = time.into();
    Ok(Value::整数(datetime.minute() as i64))
}

pub fn builtin_get_second(args: Vec<Value>) -> RuntimeResult<Value> {
    let timestamp = if args.is_empty() {
        system_time_to_timestamp(SystemTime::now())
    } else {
        match &args[0] {
            Value::整数(n) => *n,
            Value::浮点数(n) => *n as i64,
            _ => return Err(RuntimeError::TypeError("时间戳必须是数字".to_string())),
        }
    };

    let time = timestamp_to_system_time(timestamp);
    let datetime: chrono::DateTime<chrono::Local> = time.into();
    Ok(Value::整数(datetime.second() as i64))
}

pub fn builtin_get_weekday(args: Vec<Value>) -> RuntimeResult<Value> {
    let timestamp = if args.is_empty() {
        system_time_to_timestamp(SystemTime::now())
    } else {
        match &args[0] {
            Value::整数(n) => *n,
            Value::浮点数(n) => *n as i64,
            _ => return Err(RuntimeError::TypeError("时间戳必须是数字".to_string())),
        }
    };

    let time = timestamp_to_system_time(timestamp);
    let datetime: chrono::DateTime<chrono::Local> = time.into();
    let weekday = datetime.weekday();
    
    let weekday_names = vec!["周一", "周二", "周三", "周四", "周五", "周六", "周日"];
    let index = weekday.num_days_from_monday() as usize;
    
    let mut result = HashMap::new();
    result.insert("数字".to_string(), Value::整数(index as i64 + 1));
    result.insert("名称".to_string(), Value::字符串(weekday_names[index].to_string()));
    
    Ok(Value::字典(result))
}

pub fn builtin_sleep(args: Vec<Value>) -> RuntimeResult<Value> {
    if args.is_empty() {
        return Err(RuntimeError::ArgumentCountError {
            function: "休眠".to_string(),
            expected: 1,
            actual: 0,
        });
    }

    let seconds = match &args[0] {
        Value::整数(n) => *n as u64,
        Value::浮点数(n) => *n as u64,
        _ => return Err(RuntimeError::TypeError("秒数必须是数字".to_string())),
    };

    std::thread::sleep(std::time::Duration::from_secs(seconds));
    Ok(Value::空值)
}

pub fn builtin_millisleep(args: Vec<Value>) -> RuntimeResult<Value> {
    if args.is_empty() {
        return Err(RuntimeError::ArgumentCountError {
            function: "毫秒休眠".to_string(),
            expected: 1,
            actual: 0,
        });
    }

    let millis = match &args[0] {
        Value::整数(n) => *n as u64,
        Value::浮点数(n) => *n as u64,
        _ => return Err(RuntimeError::TypeError("毫秒数必须是数字".to_string())),
    };

    std::thread::sleep(std::time::Duration::from_millis(millis));
    Ok(Value::空值)
}
