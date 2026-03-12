use crate::error::{RuntimeError, RuntimeResult};
use crate::runtime::value::Value;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

static mut DB_TABLES: Option<HashMap<String, Vec<HashMap<String, Value>>>> = None;

fn get_tables() -> &'static mut HashMap<String, Vec<HashMap<String, Value>>> {
    unsafe {
        if DB_TABLES.is_none() {
            DB_TABLES = Some(HashMap::new());
        }
        DB_TABLES.as_mut().unwrap()
    }
}

pub fn builtin_数据库创建表(参数: Vec<Value>) -> RuntimeResult<Value> {
    if 参数.len() < 2 {
        return Err(RuntimeError::General("数据库创建表需要两个参数：表名和字段列表".to_string()));
    }
    
    let table_name = match &参数[0] {
        Value::字符串(s) => s.clone(),
        _ => return Err(RuntimeError::TypeError("表名必须是字符串".to_string())),
    };
    
    let _fields = match &参数[1] {
        Value::列表(fields) => fields.clone(),
        _ => return Err(RuntimeError::TypeError("字段列表必须是列表".to_string())),
    };
    
    let tables = get_tables();
    tables.insert(table_name.clone(), Vec::new());
    
    Ok(Value::布尔值(true))
}

pub fn builtin_数据库插入(参数: Vec<Value>) -> RuntimeResult<Value> {
    if 参数.len() < 2 {
        return Err(RuntimeError::General("数据库插入需要两个参数：表名和数据".to_string()));
    }
    
    let table_name = match &参数[0] {
        Value::字符串(s) => s.clone(),
        _ => return Err(RuntimeError::TypeError("表名必须是字符串".to_string())),
    };
    
    let data = match &参数[1] {
        Value::字典(map) => map.clone(),
        _ => return Err(RuntimeError::TypeError("数据必须是字典".to_string())),
    };
    
    let tables = get_tables();
    if let Some(table) = tables.get_mut(&table_name) {
        table.push(data);
        Ok(Value::布尔值(true))
    } else {
        Err(RuntimeError::General(format!("表 {} 不存在", table_name)))
    }
}

pub fn builtin_数据库查询(参数: Vec<Value>) -> RuntimeResult<Value> {
    if 参数.is_empty() {
        return Err(RuntimeError::General("数据库查询需要一个参数：表名".to_string()));
    }
    
    let table_name = match &参数[0] {
        Value::字符串(s) => s.clone(),
        _ => return Err(RuntimeError::TypeError("表名必须是字符串".to_string())),
    };
    
    let filter = if 参数.len() > 1 {
        match &参数[1] {
            Value::字典(map) => Some(map.clone()),
            _ => None,
        }
    } else {
        None
    };
    
    let tables = get_tables();
    if let Some(table) = tables.get(&table_name) {
        let result: Vec<Value> = if let Some(filter_map) = filter {
            table.iter()
                .filter(|row| {
                    filter_map.iter().all(|(k, v)| {
                        row.get(k).map_or(false, |val| val == v)
                    })
                })
                .map(|row| Value::字典(row.clone()))
                .collect()
        } else {
            table.iter().map(|row| Value::字典(row.clone())).collect()
        };
        Ok(Value::列表(result))
    } else {
        Err(RuntimeError::General(format!("表 {} 不存在", table_name)))
    }
}

pub fn builtin_数据库更新(参数: Vec<Value>) -> RuntimeResult<Value> {
    if 参数.len() < 3 {
        return Err(RuntimeError::General("数据库更新需要三个参数：表名、条件和更新数据".to_string()));
    }
    
    let table_name = match &参数[0] {
        Value::字符串(s) => s.clone(),
        _ => return Err(RuntimeError::TypeError("表名必须是字符串".to_string())),
    };
    
    let filter = match &参数[1] {
        Value::字典(map) => map.clone(),
        _ => return Err(RuntimeError::TypeError("条件必须是字典".to_string())),
    };
    
    let update_data = match &参数[2] {
        Value::字典(map) => map.clone(),
        _ => return Err(RuntimeError::TypeError("更新数据必须是字典".to_string())),
    };
    
    let tables = get_tables();
    if let Some(table) = tables.get_mut(&table_name) {
        let mut count = 0;
        for row in table.iter_mut() {
            if filter.iter().all(|(k, v)| row.get(k).map_or(false, |val| val == v)) {
                for (k, v) in &update_data {
                    row.insert(k.clone(), v.clone());
                }
                count += 1;
            }
        }
        Ok(Value::整数(count))
    } else {
        Err(RuntimeError::General(format!("表 {} 不存在", table_name)))
    }
}

pub fn builtin_数据库删除(参数: Vec<Value>) -> RuntimeResult<Value> {
    if 参数.len() < 2 {
        return Err(RuntimeError::General("数据库删除需要两个参数：表名和条件".to_string()));
    }
    
    let table_name = match &参数[0] {
        Value::字符串(s) => s.clone(),
        _ => return Err(RuntimeError::TypeError("表名必须是字符串".to_string())),
    };
    
    let filter = match &参数[1] {
        Value::字典(map) => map.clone(),
        _ => return Err(RuntimeError::TypeError("条件必须是字典".to_string())),
    };
    
    let tables = get_tables();
    if let Some(table) = tables.get_mut(&table_name) {
        let original_len = table.len();
        table.retain(|row| {
            !filter.iter().all(|(k, v)| row.get(k).map_or(false, |val| val == v))
        });
        Ok(Value::整数((original_len - table.len()) as i64))
    } else {
        Err(RuntimeError::General(format!("表 {} 不存在", table_name)))
    }
}

pub fn builtin_数据库保存(参数: Vec<Value>) -> RuntimeResult<Value> {
    if 参数.is_empty() {
        return Err(RuntimeError::General("数据库保存需要一个参数：文件路径".to_string()));
    }
    
    let filepath = match &参数[0] {
        Value::字符串(s) => s.clone(),
        _ => return Err(RuntimeError::TypeError("文件路径必须是字符串".to_string())),
    };
    
    let tables = get_tables();
    let mut content = String::new();
    
    for (table_name, rows) in tables.iter() {
        content.push_str(&format!("[{}]\n", table_name));
        for row in rows {
            let pairs: Vec<String> = row.iter()
                .map(|(k, v)| format!("{}={}", k, v.to_string_value()))
                .collect();
            content.push_str(&format!("{}\n", pairs.join(",")));
        }
        content.push('\n');
    }
    
    fs::write(&filepath, content)
        .map_err(|e| RuntimeError::General(format!("保存数据库失败: {}", e)))?;
    
    Ok(Value::布尔值(true))
}

pub fn builtin_数据库加载(参数: Vec<Value>) -> RuntimeResult<Value> {
    if 参数.is_empty() {
        return Err(RuntimeError::General("数据库加载需要一个参数：文件路径".to_string()));
    }
    
    let filepath = match &参数[0] {
        Value::字符串(s) => s.clone(),
        _ => return Err(RuntimeError::TypeError("文件路径必须是字符串".to_string())),
    };
    
    if !Path::new(&filepath).exists() {
        return Err(RuntimeError::General(format!("文件 {} 不存在", filepath)));
    }
    
    let content = fs::read_to_string(&filepath)
        .map_err(|e| RuntimeError::General(format!("读取数据库文件失败: {}", e)))?;
    
    let tables = get_tables();
    tables.clear();
    
    let mut current_table: Option<String> = None;
    
    for line in content.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        
        if line.starts_with('[') && line.ends_with(']') {
            let table_name = &line[1..line.len()-1];
            current_table = Some(table_name.to_string());
            tables.insert(table_name.to_string(), Vec::new());
        } else if let Some(ref table_name) = current_table {
            let mut row = HashMap::new();
            for pair in line.split(',') {
                let parts: Vec<&str> = pair.splitn(2, '=').collect();
                if parts.len() == 2 {
                    row.insert(parts[0].trim().to_string(), Value::字符串(parts[1].trim().to_string()));
                }
            }
            if let Some(table) = tables.get_mut(table_name) {
                table.push(row);
            }
        }
    }
    
    Ok(Value::布尔值(true))
}

pub fn builtin_数据库删除表(参数: Vec<Value>) -> RuntimeResult<Value> {
    if 参数.is_empty() {
        return Err(RuntimeError::General("数据库删除表需要一个参数：表名".to_string()));
    }
    
    let table_name = match &参数[0] {
        Value::字符串(s) => s.clone(),
        _ => return Err(RuntimeError::TypeError("表名必须是字符串".to_string())),
    };
    
    let tables = get_tables();
    tables.remove(&table_name);
    
    Ok(Value::布尔值(true))
}
