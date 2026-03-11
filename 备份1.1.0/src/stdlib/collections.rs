// 集合模块

use crate::runtime::value::Value;
use std::collections::{HashMap, BTreeMap, VecDeque};


/// 计数器结构
pub struct Counter {
    counts: HashMap<String, usize>,
}

impl Counter {
    /// 创建新的计数器
    pub fn new() -> Self {
        Self { counts: HashMap::new() }
    }
    
    /// 从列表创建计数器
    pub fn from_list(list: &[Value]) -> Self {
        let mut counter = Self::new();
        for item in list {
            let key = match item {
                Value::整数(n) => n.to_string(),
                Value::浮点数(f) => f.to_string(),
                Value::字符串(s) => s.clone(),
                Value::布尔值(b) => b.to_string(),
                _ => "".to_string(),
            };
            *counter.counts.entry(key).or_insert(0) += 1;
        }
        counter
    }
    
    /// 获取计数
    pub fn get(&self, key: &str) -> Option<usize> {
        self.counts.get(key).copied()
    }
    
    /// 转换为字典
    pub fn to_dict(&self) -> HashMap<String, Value> {
        let mut dict = HashMap::new();
        for (key, count) in &self.counts {
            dict.insert(key.clone(), Value::整数(*count as i64));
        }
        dict
    }
}

/// 双端队列结构
pub struct Deque {
    deque: VecDeque<Value>,
}

impl Deque {
    /// 创建新的双端队列
    pub fn new() -> Self {
        Self { deque: VecDeque::new() }
    }
    
    /// 从列表创建双端队列
    pub fn from_list(list: &[Value]) -> Self {
        let mut deque = Self::new();
        for item in list {
            deque.deque.push_back(item.clone() as Value);
        }
        deque
    }
    
    /// 从左侧添加元素
    pub fn append_left(&mut self, item: Value) {
        self.deque.push_front(item);
    }
    
    /// 从右侧添加元素
    pub fn append(&mut self, item: Value) {
        self.deque.push_back(item);
    }
    
    /// 从左侧弹出元素
    pub fn pop_left(&mut self) -> Option<Value> {
        self.deque.pop_front()
    }
    
    /// 从右侧弹出元素
    pub fn pop(&mut self) -> Option<Value> {
        self.deque.pop_back()
    }
    
    /// 获取长度
    pub fn len(&self) -> usize {
        self.deque.len()
    }
    
    /// 转换为列表
    pub fn to_list(&self) -> Vec<Value> {
        self.deque.clone().into_iter().collect()
    }
}

/// 有序字典结构
pub struct OrderedDict {
    map: BTreeMap<String, Value>,
}

impl OrderedDict {
    /// 创建新的有序字典
    pub fn new() -> Self {
        Self { map: BTreeMap::new() }
    }
    
    /// 从字典创建有序字典
    pub fn from_dict(dict: &HashMap<String, Value>) -> Self {
        let mut ordered = Self::new();
        for (key, value) in dict {
            ordered.map.insert(key.clone() as String, value.clone() as Value);
        }
        ordered
    }
    
    /// 设置键值对
    pub fn set(&mut self, key: &str, value: Value) {
        self.map.insert(key.to_string(), value);
    }
    
    /// 获取值
    pub fn get(&self, key: &str) -> Option<&Value> {
        self.map.get(key)
    }
    
    /// 删除键值对
    pub fn remove(&mut self, key: &str) -> Option<Value> {
        self.map.remove(key)
    }
    
    /// 获取键列表
    pub fn keys(&self) -> Vec<String> {
        self.map.keys().cloned().collect()
    }
    
    /// 获取值列表
    pub fn values(&self) -> Vec<Value> {
        self.map.values().cloned().collect()
    }
    
    /// 转换为字典
    pub fn to_dict(&self) -> HashMap<String, Value> {
        self.map.iter().map(|(k, v): (&String, &Value)| (k.clone(), v.clone())).collect()
    }
}

/// 计算列表中元素出现的次数
pub fn builtin_集合_计数(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 1 {
        return Err("计数函数需要一个参数".to_string());
    }
    
    match &args[0] {
        Value::列表(list) => {
            let counter = Counter::from_list(list);
            Ok(Value::字典(counter.to_dict()))
        }
        _ => Err("参数必须是列表".to_string()),
    }
}

/// 创建双端队列
pub fn builtin_集合_双端队列(args: Vec<Value>) -> Result<Value, String> {
    match args.len() {
        0 => {
            // 创建空双端队列
            let deque = Deque::new();
            Ok(Value::列表(deque.to_list()))
        }
        1 => {
            // 从列表创建双端队列
            match &args[0] {
                Value::列表(list) => {
                    let deque = Deque::from_list(list);
                    Ok(Value::列表(deque.to_list()))
                }
                _ => Err("参数必须是列表".to_string()),
            }
        }
        _ => Err("双端队列函数需要0或1个参数".to_string()),
    }
}

/// 从双端队列左侧添加元素
pub fn builtin_集合_双端队列_左侧添加(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 2 {
        return Err("双端队列左侧添加函数需要两个参数".to_string());
    }
    
    match (&args[0], &args[1]) {
        (Value::列表(list), item) => {
            let mut deque = Deque::from_list(list);
            deque.append_left(item.clone() as Value);
            Ok(Value::列表(deque.to_list()))
        }
        _ => Err("第一个参数必须是列表".to_string()),
    }
}

/// 从双端队列右侧添加元素
pub fn builtin_集合_双端队列_右侧添加(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 2 {
        return Err("双端队列右侧添加函数需要两个参数".to_string());
    }
    
    match (&args[0], &args[1]) {
        (Value::列表(list), item) => {
            let mut deque = Deque::from_list(list);
            deque.append(item.clone() as Value);
            Ok(Value::列表(deque.to_list()))
        }
        _ => Err("第一个参数必须是列表".to_string()),
    }
}

/// 从双端队列左侧弹出元素
pub fn builtin_集合_双端队列_左侧弹出(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 1 {
        return Err("双端队列左侧弹出函数需要一个参数".to_string());
    }
    
    match &args[0] {
        Value::列表(list) => {
            let mut deque = Deque::from_list(list);
            match deque.pop_left() {
                Some(item) => Ok(item),
                None => Ok(Value::空值),
            }
        }
        _ => Err("参数必须是列表".to_string()),
    }
}

/// 从双端队列右侧弹出元素
pub fn builtin_集合_双端队列_右侧弹出(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 1 {
        return Err("双端队列右侧弹出函数需要一个参数".to_string());
    }
    
    match &args[0] {
        Value::列表(list) => {
            let mut deque = Deque::from_list(list);
            match deque.pop() {
                Some(item) => Ok(item),
                None => Ok(Value::空值),
            }
        }
        _ => Err("参数必须是列表".to_string()),
    }
}

/// 创建有序字典
pub fn builtin_集合_有序字典(args: Vec<Value>) -> Result<Value, String> {
    match args.len() {
        0 => {
            // 创建空有序字典
            let ordered = OrderedDict::new();
            Ok(Value::字典(ordered.to_dict()))
        }
        1 => {
            // 从字典创建有序字典
            match &args[0] {
                Value::字典(dict) => {
                    let ordered = OrderedDict::from_dict(dict);
                    Ok(Value::字典(ordered.to_dict()))
                }
                _ => Err("参数必须是字典".to_string()),
            }
        }
        _ => Err("有序字典函数需要0或1个参数".to_string()),
    }
}

/// 集合模块函数表
pub fn get_collections_functions() -> Vec<(&'static str, fn(Vec<Value>) -> Result<Value, String>)> {
    vec![
        ("计数", builtin_集合_计数),
        ("双端队列", builtin_集合_双端队列),
        ("双端队列_左侧添加", builtin_集合_双端队列_左侧添加),
        ("双端队列_右侧添加", builtin_集合_双端队列_右侧添加),
        ("双端队列_左侧弹出", builtin_集合_双端队列_左侧弹出),
        ("双端队列_右侧弹出", builtin_集合_双端队列_右侧弹出),
        ("有序字典", builtin_集合_有序字典),
    ]
}
