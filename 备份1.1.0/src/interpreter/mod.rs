use crate::error::{RuntimeError, RuntimeResult};
use crate::parser::ast::*;
use crate::parser::ast::FormatPart;
use crate::runtime::value::{Environment, Value};
use crate::stdlib::{
    builtin_append_file, builtin_copy_file, builtin_create_dir, builtin_date_add,
    builtin_date_diff, builtin_delete_dir, builtin_delete_file, builtin_execute_command,
    builtin_file_size, builtin_format_date, builtin_get_absolute_path, builtin_get_cwd,
    builtin_get_day, builtin_get_env, builtin_get_home_dir, builtin_get_hour, builtin_get_minute,
    builtin_get_month, builtin_get_path_separator, builtin_get_second, builtin_get_temp_dir,
    builtin_get_weekday, builtin_get_year, builtin_hashmap_clear, builtin_hashmap_get,
    builtin_hashmap_has, builtin_hashmap_keys, builtin_hashmap_new, builtin_hashmap_remove,
    builtin_hashmap_set, builtin_hashmap_size, builtin_hashmap_values, builtin_http_get,
    builtin_http_post, builtin_http_put, builtin_http_delete, builtin_http_head, builtin_is_dir,
    builtin_is_file, builtin_join_path, builtin_json_get, builtin_json_parse, builtin_json_parse_file,
    builtin_json_set, builtin_json_stringify, builtin_json_stringify_file, builtin_json_validate,
    builtin_list_dir, builtin_list_env, builtin_millisleep, builtin_move_file, builtin_now,
    builtin_parse_date, builtin_path_exists, builtin_path_is_dir, builtin_path_is_file,
    builtin_queue_dequeue, builtin_queue_enqueue, builtin_queue_new, builtin_queue_peek,
    builtin_remove_env, builtin_rename_file, builtin_set_cwd, builtin_set_env, builtin_sleep,
    builtin_spawn_process, builtin_stack_new, builtin_stack_peek, builtin_stack_pop,
    builtin_stack_push, builtin_timestamp,
};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

pub struct Interpreter {
    global_env: Rc<RefCell<Environment>>,
    functions: HashMap<String, (Vec<String>, Vec<Statement>)>,
    class_methods: HashMap<String, HashMap<String, Vec<Statement>>>,
    output_callback: Option<Box<dyn FnMut(String)>>,
}

impl Interpreter {
    pub fn new() -> Self {
        let mut env = Environment::new();
        Self::register_builtins(&mut env);
        
        Interpreter {
            global_env: Rc::new(RefCell::new(env)),
            functions: HashMap::new(),
            class_methods: HashMap::new(),
            output_callback: None,
        }
    }
    
    pub fn set_output_callback<F>(&mut self, callback: F) where F: FnMut(String) + 'static {
        self.output_callback = Some(Box::new(callback));
    }
    
    fn register_builtins(env: &mut Environment) {
        env.define("长度".to_string(), Value::内置函数 {
            名称: "长度".to_string(),
            函数: builtin_length,
        });
        
        env.define("类型".to_string(), Value::内置函数 {
            名称: "类型".to_string(),
            函数: builtin_type,
        });
        
        env.define("转字符串".to_string(), Value::内置函数 {
            名称: "转字符串".to_string(),
            函数: builtin_to_string,
        });
        
        env.define("转整数".to_string(), Value::内置函数 {
            名称: "转整数".to_string(),
            函数: builtin_to_int,
        });
        
        env.define("转浮点数".to_string(), Value::内置函数 {
            名称: "转浮点数".to_string(),
            函数: builtin_to_float,
        });
        
        env.define("范围".to_string(), Value::内置函数 {
            名称: "范围".to_string(),
            函数: builtin_range,
        });
        
        env.define("追加".to_string(), Value::内置函数 {
            名称: "追加".to_string(),
            函数: builtin_append,
        });
        
        env.define("弹出".to_string(), Value::内置函数 {
            名称: "弹出".to_string(),
            函数: builtin_pop,
        });
        
        env.define("插入".to_string(), Value::内置函数 {
            名称: "插入".to_string(),
            函数: builtin_insert,
        });
        
        env.define("删除".to_string(), Value::内置函数 {
            名称: "删除".to_string(),
            函数: builtin_remove,
        });
        
        env.define("绝对值".to_string(), Value::内置函数 {
            名称: "绝对值".to_string(),
            函数: builtin_abs,
        });
        
        env.define("四舍五入".to_string(), Value::内置函数 {
            名称: "四舍五入".to_string(),
            函数: builtin_round,
        });
        
        env.define("向下取整".to_string(), Value::内置函数 {
            名称: "向下取整".to_string(),
            函数: builtin_floor,
        });
        
        env.define("向上取整".to_string(), Value::内置函数 {
            名称: "向上取整".to_string(),
            函数: builtin_ceil,
        });
        
        env.define("平方根".to_string(), Value::内置函数 {
            名称: "平方根".to_string(),
            函数: builtin_sqrt,
        });
        
        env.define("幂".to_string(), Value::内置函数 {
            名称: "幂".to_string(),
            函数: builtin_pow,
        });
        
        env.define("最小值".to_string(), Value::内置函数 {
            名称: "最小值".to_string(),
            函数: builtin_min,
        });
        
        env.define("最大值".to_string(), Value::内置函数 {
            名称: "最大值".to_string(),
            函数: builtin_max,
        });
        
        env.define("排序".to_string(), Value::内置函数 {
            名称: "排序".to_string(),
            函数: builtin_sort,
        });
        
        env.define("反转".to_string(), Value::内置函数 {
            名称: "反转".to_string(),
            函数: builtin_reverse,
        });
        
        env.define("连接".to_string(), Value::内置函数 {
            名称: "连接".to_string(),
            函数: builtin_join,
        });
        
        env.define("分割".to_string(), Value::内置函数 {
            名称: "分割".to_string(),
            函数: builtin_split,
        });
        
        env.define("替换".to_string(), Value::内置函数 {
            名称: "替换".to_string(),
            函数: builtin_replace,
        });
        
        env.define("包含".to_string(), Value::内置函数 {
            名称: "包含".to_string(),
            函数: builtin_contains,
        });
        
        env.define("查找".to_string(), Value::内置函数 {
            名称: "查找".to_string(),
            函数: builtin_find,
        });
        
        env.define("子字符串".to_string(), Value::内置函数 {
            名称: "子字符串".to_string(),
            函数: builtin_substring,
        });
        
        env.define("大写".to_string(), Value::内置函数 {
            名称: "大写".to_string(),
            函数: builtin_upper,
        });
        
        env.define("小写".to_string(), Value::内置函数 {
            名称: "小写".to_string(),
            函数: builtin_lower,
        });
        
        env.define("去除空白".to_string(), Value::内置函数 {
            名称: "去除空白".to_string(),
            函数: builtin_trim,
        });
        
        env.define("当前时间".to_string(), Value::内置函数 {
            名称: "当前时间".to_string(),
            函数: builtin_now,
        });
        
        env.define("随机数".to_string(), Value::内置函数 {
            名称: "随机数".to_string(),
            函数: builtin_random,
        });
        
        env.define("输入".to_string(), Value::内置函数 {
            名称: "输入".to_string(),
            函数: builtin_input,
        });
        
        env.define("读取文件".to_string(), Value::内置函数 {
            名称: "读取文件".to_string(),
            函数: builtin_read_file,
        });
        
        env.define("写入文件".to_string(), Value::内置函数 {
            名称: "写入文件".to_string(),
            函数: builtin_write_file,
        });
        
        env.define("文件存在".to_string(), Value::内置函数 {
            名称: "文件存在".to_string(),
            函数: builtin_file_exists,
        });
        
        env.define("追加文件".to_string(), Value::内置函数 {
            名称: "追加文件".to_string(),
            函数: builtin_append_file,
        });
        
        env.define("删除文件".to_string(), Value::内置函数 {
            名称: "删除文件".to_string(),
            函数: builtin_delete_file,
        });
        
        env.define("复制文件".to_string(), Value::内置函数 {
            名称: "复制文件".to_string(),
            函数: builtin_copy_file,
        });
        
        env.define("列出目录".to_string(), Value::内置函数 {
            名称: "列出目录".to_string(),
            函数: builtin_list_dir,
        });
        
        env.define("创建目录".to_string(), Value::内置函数 {
            名称: "创建目录".to_string(),
            函数: builtin_create_dir,
        });
        
        env.define("删除目录".to_string(), Value::内置函数 {
            名称: "删除目录".to_string(),
            函数: builtin_delete_dir,
        });
        
        env.define("是文件".to_string(), Value::内置函数 {
            名称: "是文件".to_string(),
            函数: builtin_is_file,
        });
        
        env.define("是目录".to_string(), Value::内置函数 {
            名称: "是目录".to_string(),
            函数: builtin_is_dir,
        });
        
        env.define("文件大小".to_string(), Value::内置函数 {
            名称: "文件大小".to_string(),
            函数: builtin_file_size,
        });
        
        env.define("栈新建".to_string(), Value::内置函数 {
            名称: "栈新建".to_string(),
            函数: builtin_stack_new,
        });
        
        env.define("栈推入".to_string(), Value::内置函数 {
            名称: "栈推入".to_string(),
            函数: builtin_stack_push,
        });
        
        env.define("栈弹出".to_string(), Value::内置函数 {
            名称: "栈弹出".to_string(),
            函数: builtin_stack_pop,
        });
        
        env.define("栈顶元素".to_string(), Value::内置函数 {
            名称: "栈顶元素".to_string(),
            函数: builtin_stack_peek,
        });
        
        env.define("队列新建".to_string(), Value::内置函数 {
            名称: "队列新建".to_string(),
            函数: builtin_queue_new,
        });
        
        env.define("队列入队".to_string(), Value::内置函数 {
            名称: "队列入队".to_string(),
            函数: builtin_queue_enqueue,
        });
        
        env.define("队列出队".to_string(), Value::内置函数 {
            名称: "队列出队".to_string(),
            函数: builtin_queue_dequeue,
        });
        
        env.define("队列首元素".to_string(), Value::内置函数 {
            名称: "队列首元素".to_string(),
            函数: builtin_queue_peek,
        });
        
        env.define("哈希表新建".to_string(), Value::内置函数 {
            名称: "哈希表新建".to_string(),
            函数: builtin_hashmap_new,
        });
        
        env.define("哈希表设置".to_string(), Value::内置函数 {
            名称: "哈希表设置".to_string(),
            函数: builtin_hashmap_set,
        });
        
        env.define("哈希表获取".to_string(), Value::内置函数 {
            名称: "哈希表获取".to_string(),
            函数: builtin_hashmap_get,
        });
        
        env.define("哈希表包含".to_string(), Value::内置函数 {
            名称: "哈希表包含".to_string(),
            函数: builtin_hashmap_has,
        });
        
        env.define("哈希表删除".to_string(), Value::内置函数 {
            名称: "哈希表删除".to_string(),
            函数: builtin_hashmap_remove,
        });
        
        env.define("哈希表键列表".to_string(), Value::内置函数 {
            名称: "哈希表键列表".to_string(),
            函数: builtin_hashmap_keys,
        });
        
        env.define("哈希表值列表".to_string(), Value::内置函数 {
            名称: "哈希表值列表".to_string(),
            函数: builtin_hashmap_values,
        });
        
        env.define("哈希表大小".to_string(), Value::内置函数 {
            名称: "哈希表大小".to_string(),
            函数: builtin_hashmap_size,
        });
        
        env.define("哈希表清空".to_string(), Value::内置函数 {
            名称: "哈希表清空".to_string(),
            函数: builtin_hashmap_clear,
        });
        
        env.define("HTTP获取".to_string(), Value::内置函数 {
            名称: "HTTP获取".to_string(),
            函数: builtin_http_get,
        });
        
        env.define("HTTP提交".to_string(), Value::内置函数 {
            名称: "HTTP提交".to_string(),
            函数: builtin_http_post,
        });
        
        env.define("HTTP更新".to_string(), Value::内置函数 {
            名称: "HTTP更新".to_string(),
            函数: builtin_http_put,
        });
        
        env.define("HTTP删除".to_string(), Value::内置函数 {
            名称: "HTTP删除".to_string(),
            函数: builtin_http_delete,
        });
        
        env.define("HTTP头部".to_string(), Value::内置函数 {
            名称: "HTTP头部".to_string(),
            函数: builtin_http_head,
        });
        
        env.define("重命名文件".to_string(), Value::内置函数 {
            名称: "重命名文件".to_string(),
            函数: builtin_rename_file,
        });
        
        env.define("移动文件".to_string(), Value::内置函数 {
            名称: "移动文件".to_string(),
            函数: builtin_move_file,
        });
        
        env.define("获取环境变量".to_string(), Value::内置函数 {
            名称: "获取环境变量".to_string(),
            函数: builtin_get_env,
        });
        
        env.define("设置环境变量".to_string(), Value::内置函数 {
            名称: "设置环境变量".to_string(),
            函数: builtin_set_env,
        });
        
        env.define("删除环境变量".to_string(), Value::内置函数 {
            名称: "删除环境变量".to_string(),
            函数: builtin_remove_env,
        });
        
        env.define("列出环境变量".to_string(), Value::内置函数 {
            名称: "列出环境变量".to_string(),
            函数: builtin_list_env,
        });
        
        env.define("获取当前目录".to_string(), Value::内置函数 {
            名称: "获取当前目录".to_string(),
            函数: builtin_get_cwd,
        });
        
        env.define("设置当前目录".to_string(), Value::内置函数 {
            名称: "设置当前目录".to_string(),
            函数: builtin_set_cwd,
        });
        
        env.define("执行命令".to_string(), Value::内置函数 {
            名称: "执行命令".to_string(),
            函数: builtin_execute_command,
        });
        
        env.define("启动进程".to_string(), Value::内置函数 {
            名称: "启动进程".to_string(),
            函数: builtin_spawn_process,
        });
        
        env.define("路径存在".to_string(), Value::内置函数 {
            名称: "路径存在".to_string(),
            函数: builtin_path_exists,
        });
        
        env.define("路径是文件".to_string(), Value::内置函数 {
            名称: "路径是文件".to_string(),
            函数: builtin_path_is_file,
        });
        
        env.define("路径是目录".to_string(), Value::内置函数 {
            名称: "路径是目录".to_string(),
            函数: builtin_path_is_dir,
        });
        
        env.define("获取临时目录".to_string(), Value::内置函数 {
            名称: "获取临时目录".to_string(),
            函数: builtin_get_temp_dir,
        });
        
        env.define("获取用户目录".to_string(), Value::内置函数 {
            名称: "获取用户目录".to_string(),
            函数: builtin_get_home_dir,
        });
        
        env.define("连接路径".to_string(), Value::内置函数 {
            名称: "连接路径".to_string(),
            函数: builtin_join_path,
        });
        
        env.define("获取绝对路径".to_string(), Value::内置函数 {
            名称: "获取绝对路径".to_string(),
            函数: builtin_get_absolute_path,
        });
        
        env.define("获取路径分隔符".to_string(), Value::内置函数 {
            名称: "获取路径分隔符".to_string(),
            函数: builtin_get_path_separator,
        });
        
        env.define("当前时间".to_string(), Value::内置函数 {
            名称: "当前时间".to_string(),
            函数: builtin_now,
        });
        
        env.define("时间戳".to_string(), Value::内置函数 {
            名称: "时间戳".to_string(),
            函数: builtin_timestamp,
        });
        
        env.define("格式化日期".to_string(), Value::内置函数 {
            名称: "格式化日期".to_string(),
            函数: builtin_format_date,
        });
        
        env.define("解析日期".to_string(), Value::内置函数 {
            名称: "解析日期".to_string(),
            函数: builtin_parse_date,
        });
        
        env.define("日期加法".to_string(), Value::内置函数 {
            名称: "日期加法".to_string(),
            函数: builtin_date_add,
        });
        
        env.define("日期差值".to_string(), Value::内置函数 {
            名称: "日期差值".to_string(),
            函数: builtin_date_diff,
        });
        
        env.define("获取年份".to_string(), Value::内置函数 {
            名称: "获取年份".to_string(),
            函数: builtin_get_year,
        });
        
        env.define("获取月份".to_string(), Value::内置函数 {
            名称: "获取月份".to_string(),
            函数: builtin_get_month,
        });
        
        env.define("获取日期".to_string(), Value::内置函数 {
            名称: "获取日期".to_string(),
            函数: builtin_get_day,
        });
        
        env.define("获取小时".to_string(), Value::内置函数 {
            名称: "获取小时".to_string(),
            函数: builtin_get_hour,
        });
        
        env.define("获取分钟".to_string(), Value::内置函数 {
            名称: "获取分钟".to_string(),
            函数: builtin_get_minute,
        });
        
        env.define("获取秒数".to_string(), Value::内置函数 {
            名称: "获取秒数".to_string(),
            函数: builtin_get_second,
        });
        
        env.define("获取星期".to_string(), Value::内置函数 {
            名称: "获取星期".to_string(),
            函数: builtin_get_weekday,
        });
        
        env.define("休眠".to_string(), Value::内置函数 {
            名称: "休眠".to_string(),
            函数: builtin_sleep,
        });
        
        env.define("毫秒休眠".to_string(), Value::内置函数 {
            名称: "毫秒休眠".to_string(),
            函数: builtin_millisleep,
        });
        
        env.define("JSON序列化".to_string(), Value::内置函数 {
            名称: "JSON序列化".to_string(),
            函数: builtin_json_stringify,
        });
        
        env.define("JSON解析".to_string(), Value::内置函数 {
            名称: "JSON解析".to_string(),
            函数: builtin_json_parse,
        });
        
        env.define("JSON解析文件".to_string(), Value::内置函数 {
            名称: "JSON解析文件".to_string(),
            函数: builtin_json_parse_file,
        });
        
        env.define("JSON序列化文件".to_string(), Value::内置函数 {
            名称: "JSON序列化文件".to_string(),
            函数: builtin_json_stringify_file,
        });
        
        env.define("JSON验证".to_string(), Value::内置函数 {
            名称: "JSON验证".to_string(),
            函数: builtin_json_validate,
        });
        
        env.define("JSON获取".to_string(), Value::内置函数 {
            名称: "JSON获取".to_string(),
            函数: builtin_json_get,
        });
        
        env.define("JSON设置".to_string(), Value::内置函数 {
            名称: "JSON设置".to_string(),
            函数: builtin_json_set,
        });
        
        env.define("创建窗口".to_string(), Value::内置函数 {
            名称: "创建窗口".to_string(),
            函数: crate::stdlib::builtin_创建窗口,
        });
        
        env.define("设置当前窗口".to_string(), Value::内置函数 {
            名称: "设置当前窗口".to_string(),
            函数: crate::stdlib::builtin_设置当前窗口,
        });
        
        env.define("添加按钮".to_string(), Value::内置函数 {
            名称: "添加按钮".to_string(),
            函数: crate::stdlib::builtin_添加按钮,
        });
        
        env.define("添加标签".to_string(), Value::内置函数 {
            名称: "添加标签".to_string(),
            函数: crate::stdlib::builtin_添加标签,
        });
        
        env.define("添加输入框".to_string(), Value::内置函数 {
            名称: "添加输入框".to_string(),
            函数: crate::stdlib::builtin_添加输入框,
        });
        
        env.define("显示窗口".to_string(), Value::内置函数 {
            名称: "显示窗口".to_string(),
            函数: crate::stdlib::builtin_显示窗口,
        });
        
        env.define("等待".to_string(), Value::内置函数 {
            名称: "等待".to_string(),
            函数: crate::stdlib::builtin_等待,
        });
        
        // 网络爬虫函数
        env.define("访问网页".to_string(), Value::内置函数 {
            名称: "访问网页".to_string(),
            函数: crate::stdlib::builtin_访问网页,
        });
        
        env.define("获取状态码".to_string(), Value::内置函数 {
            名称: "获取状态码".to_string(),
            函数: crate::stdlib::builtin_获取状态码,
        });
        
        env.define("下载文件".to_string(), Value::内置函数 {
            名称: "下载文件".to_string(),
            函数: crate::stdlib::builtin_下载文件,
        });
        
        env.define("提取链接".to_string(), Value::内置函数 {
            名称: "提取链接".to_string(),
            函数: crate::stdlib::builtin_提取链接,
        });
        
        env.define("提取图片".to_string(), Value::内置函数 {
            名称: "提取图片".to_string(),
            函数: crate::stdlib::builtin_提取图片,
        });
        
        env.define("提取标题".to_string(), Value::内置函数 {
            名称: "提取标题".to_string(),
            函数: crate::stdlib::builtin_提取标题,
        });
        
        env.define("POST请求".to_string(), Value::内置函数 {
            名称: "POST请求".to_string(),
            函数: crate::stdlib::builtin_POST请求,
        });
        
        // GUI样式函数
        env.define("创建颜色".to_string(), Value::内置函数 {
            名称: "创建颜色".to_string(),
            函数: crate::stdlib::builtin_创建颜色,
        });
        
        env.define("HEX颜色".to_string(), Value::内置函数 {
            名称: "HEX颜色".to_string(),
            函数: crate::stdlib::builtin_HEX颜色,
        });
        
        env.define("获取颜色方案".to_string(), Value::内置函数 {
            名称: "获取颜色方案".to_string(),
            函数: crate::stdlib::builtin_获取颜色方案,
        });
        
        env.define("创建阴影".to_string(), Value::内置函数 {
            名称: "创建阴影".to_string(),
            函数: crate::stdlib::builtin_创建阴影,
        });
        
        env.define("创建边框".to_string(), Value::内置函数 {
            名称: "创建边框".to_string(),
            函数: crate::stdlib::builtin_创建边框,
        });
        
        env.define("创建文本样式".to_string(), Value::内置函数 {
            名称: "创建文本样式".to_string(),
            函数: crate::stdlib::builtin_创建文本样式,
        });
        
        env.define("创建间距".to_string(), Value::内置函数 {
            名称: "创建间距".to_string(),
            函数: crate::stdlib::builtin_创建间距,
        });
        
        env.define("应用样式".to_string(), Value::内置函数 {
            名称: "应用样式".to_string(),
            函数: crate::stdlib::builtin_应用样式,
        });
        
        env.define("创建渐变".to_string(), Value::内置函数 {
            名称: "创建渐变".to_string(),
            函数: crate::stdlib::builtin_创建渐变,
        });
        
        // 字符串方法
        env.define("字符串分割".to_string(), Value::内置函数 {
            名称: "字符串分割".to_string(),
            函数: crate::stdlib::builtin_字符串分割,
        });
        
        env.define("字符串连接".to_string(), Value::内置函数 {
            名称: "字符串连接".to_string(),
            函数: crate::stdlib::builtin_字符串连接,
        });
        
        env.define("字符串替换".to_string(), Value::内置函数 {
            名称: "字符串替换".to_string(),
            函数: crate::stdlib::builtin_字符串替换,
        });
        
        env.define("字符串去除空白".to_string(), Value::内置函数 {
            名称: "字符串去除空白".to_string(),
            函数: crate::stdlib::builtin_字符串去除空白,
        });
        
        env.define("字符串转大写".to_string(), Value::内置函数 {
            名称: "字符串转大写".to_string(),
            函数: crate::stdlib::builtin_字符串转大写,
        });
        
        env.define("字符串转小写".to_string(), Value::内置函数 {
            名称: "字符串转小写".to_string(),
            函数: crate::stdlib::builtin_字符串转小写,
        });
        
        env.define("字符串查找".to_string(), Value::内置函数 {
            名称: "字符串查找".to_string(),
            函数: crate::stdlib::builtin_字符串查找,
        });
        
        env.define("字符串包含".to_string(), Value::内置函数 {
            名称: "字符串包含".to_string(),
            函数: crate::stdlib::builtin_字符串包含,
        });
        
        env.define("字符串开头是".to_string(), Value::内置函数 {
            名称: "字符串开头是".to_string(),
            函数: crate::stdlib::builtin_字符串开头是,
        });
        
        env.define("字符串结尾是".to_string(), Value::内置函数 {
            名称: "字符串结尾是".to_string(),
            函数: crate::stdlib::builtin_字符串结尾是,
        });
        
        env.define("字符串长度".to_string(), Value::内置函数 {
            名称: "字符串长度".to_string(),
            函数: crate::stdlib::builtin_字符串长度,
        });
        
        env.define("字符串重复".to_string(), Value::内置函数 {
            名称: "字符串重复".to_string(),
            函数: crate::stdlib::builtin_字符串重复,
        });
        
        env.define("字符串反转".to_string(), Value::内置函数 {
            名称: "字符串反转".to_string(),
            函数: crate::stdlib::builtin_字符串反转,
        });
        
        env.define("字符串截取".to_string(), Value::内置函数 {
            名称: "字符串截取".to_string(),
            函数: crate::stdlib::builtin_字符串截取,
        });
        
        env.define("字符串统计".to_string(), Value::内置函数 {
            名称: "字符串统计".to_string(),
            函数: crate::stdlib::builtin_字符串统计,
        });
        
        env.define("字符串居中".to_string(), Value::内置函数 {
            名称: "字符串居中".to_string(),
            函数: crate::stdlib::builtin_字符串居中,
        });
        
        env.define("字符串左对齐".to_string(), Value::内置函数 {
            名称: "字符串左对齐".to_string(),
            函数: crate::stdlib::builtin_字符串左对齐,
        });
        
        env.define("字符串右对齐".to_string(), Value::内置函数 {
            名称: "字符串右对齐".to_string(),
            函数: crate::stdlib::builtin_字符串右对齐,
        });
        
        env.define("字符串是否数字".to_string(), Value::内置函数 {
            名称: "字符串是否数字".to_string(),
            函数: crate::stdlib::builtin_字符串是否数字,
        });
        
        env.define("字符串是否字母".to_string(), Value::内置函数 {
            名称: "字符串是否字母".to_string(),
            函数: crate::stdlib::builtin_字符串是否字母,
        });
        
        env.define("字符串是否字母数字".to_string(), Value::内置函数 {
            名称: "字符串是否字母数字".to_string(),
            函数: crate::stdlib::builtin_字符串是否字母数字,
        });
        
        env.define("字符串是否空白".to_string(), Value::内置函数 {
            名称: "字符串是否空白".to_string(),
            函数: crate::stdlib::builtin_字符串是否空白,
        });
        
        env.define("正则编译".to_string(), Value::内置函数 {
            名称: "正则编译".to_string(),
            函数: crate::stdlib::builtin_正则_编译,
        });
        
        env.define("正则匹配".to_string(), Value::内置函数 {
            名称: "正则匹配".to_string(),
            函数: crate::stdlib::builtin_正则_匹配,
        });
        
        env.define("正则查找".to_string(), Value::内置函数 {
            名称: "正则查找".to_string(),
            函数: crate::stdlib::builtin_正则_查找,
        });
        
        env.define("正则查找全部".to_string(), Value::内置函数 {
            名称: "正则查找全部".to_string(),
            函数: crate::stdlib::builtin_正则_查找全部,
        });
        
        env.define("正则替换".to_string(), Value::内置函数 {
            名称: "正则替换".to_string(),
            函数: crate::stdlib::builtin_正则_替换,
        });
        
        env.define("正则分割".to_string(), Value::内置函数 {
            名称: "正则分割".to_string(),
            函数: crate::stdlib::builtin_正则_分割,
        });
        
        env.define("正则提取分组".to_string(), Value::内置函数 {
            名称: "正则提取分组".to_string(),
            函数: crate::stdlib::builtin_正则_提取分组,
        });
        
        env.define("CSV读取".to_string(), Value::内置函数 {
            名称: "CSV读取".to_string(),
            函数: crate::stdlib::builtin_CSV读取,
        });
        
        env.define("CSV写入".to_string(), Value::内置函数 {
            名称: "CSV写入".to_string(),
            函数: crate::stdlib::builtin_CSV写入,
        });
        
        env.define("CSV解析".to_string(), Value::内置函数 {
            名称: "CSV解析".to_string(),
            函数: crate::stdlib::builtin_CSV解析,
        });
        
        env.define("CSV生成".to_string(), Value::内置函数 {
            名称: "CSV生成".to_string(),
            函数: crate::stdlib::builtin_CSV生成,
        });
        
        env.define("MD5哈希".to_string(), Value::内置函数 {
            名称: "MD5哈希".to_string(),
            函数: crate::stdlib::builtin_MD5哈希,
        });
        
        env.define("SHA256哈希".to_string(), Value::内置函数 {
            名称: "SHA256哈希".to_string(),
            函数: crate::stdlib::builtin_SHA256哈希,
        });
        
        env.define("SHA512哈希".to_string(), Value::内置函数 {
            名称: "SHA512哈希".to_string(),
            函数: crate::stdlib::builtin_SHA512哈希,
        });
        
        env.define("Base64编码".to_string(), Value::内置函数 {
            名称: "Base64编码".to_string(),
            函数: crate::stdlib::builtin_Base64编码,
        });
        
        env.define("Base64解码".to_string(), Value::内置函数 {
            名称: "Base64解码".to_string(),
            函数: crate::stdlib::builtin_Base64解码,
        });
        
        env.define("十六进制编码".to_string(), Value::内置函数 {
            名称: "十六进制编码".to_string(),
            函数: crate::stdlib::builtin_十六进制编码,
        });
        
        env.define("十六进制解码".to_string(), Value::内置函数 {
            名称: "十六进制解码".to_string(),
            函数: crate::stdlib::builtin_十六进制解码,
        });
        
        env.define("断言相等".to_string(), Value::内置函数 {
            名称: "断言相等".to_string(),
            函数: crate::stdlib::builtin_断言相等,
        });
        
        env.define("断言不相等".to_string(), Value::内置函数 {
            名称: "断言不相等".to_string(),
            函数: crate::stdlib::builtin_断言不相等,
        });
        
        env.define("断言为真".to_string(), Value::内置函数 {
            名称: "断言为真".to_string(),
            函数: crate::stdlib::builtin_断言为真,
        });
        
        env.define("断言为假".to_string(), Value::内置函数 {
            名称: "断言为假".to_string(),
            函数: crate::stdlib::builtin_断言为假,
        });
        
        env.define("断言为空".to_string(), Value::内置函数 {
            名称: "断言为空".to_string(),
            函数: crate::stdlib::builtin_断言为空,
        });
        
        env.define("断言不为空".to_string(), Value::内置函数 {
            名称: "断言不为空".to_string(),
            函数: crate::stdlib::builtin_断言不为空,
        });
        
        env.define("断言包含".to_string(), Value::内置函数 {
            名称: "断言包含".to_string(),
            函数: crate::stdlib::builtin_断言包含,
        });
        
        env.define("断言类型".to_string(), Value::内置函数 {
            名称: "断言类型".to_string(),
            函数: crate::stdlib::builtin_断言类型,
        });
        
        env.define("获取测试结果".to_string(), Value::内置函数 {
            名称: "获取测试结果".to_string(),
            函数: crate::stdlib::builtin_获取测试结果,
        });
        
        env.define("重置测试结果".to_string(), Value::内置函数 {
            名称: "重置测试结果".to_string(),
            函数: crate::stdlib::builtin_重置测试结果,
        });
        
        env.define("数据库创建表".to_string(), Value::内置函数 {
            名称: "数据库创建表".to_string(),
            函数: crate::stdlib::builtin_数据库创建表,
        });
        
        env.define("数据库插入".to_string(), Value::内置函数 {
            名称: "数据库插入".to_string(),
            函数: crate::stdlib::builtin_数据库插入,
        });
        
        env.define("数据库查询".to_string(), Value::内置函数 {
            名称: "数据库查询".to_string(),
            函数: crate::stdlib::builtin_数据库查询,
        });
        
        env.define("数据库更新".to_string(), Value::内置函数 {
            名称: "数据库更新".to_string(),
            函数: crate::stdlib::builtin_数据库更新,
        });
        
        env.define("数据库删除".to_string(), Value::内置函数 {
            名称: "数据库删除".to_string(),
            函数: crate::stdlib::builtin_数据库删除,
        });
        
        env.define("数据库保存".to_string(), Value::内置函数 {
            名称: "数据库保存".to_string(),
            函数: crate::stdlib::builtin_数据库保存,
        });
        
        env.define("数据库加载".to_string(), Value::内置函数 {
            名称: "数据库加载".to_string(),
            函数: crate::stdlib::builtin_数据库加载,
        });
        
        env.define("压缩文件".to_string(), Value::内置函数 {
            名称: "压缩文件".to_string(),
            函数: crate::stdlib::builtin_压缩文件,
        });
        
        env.define("解压文件".to_string(), Value::内置函数 {
            名称: "解压文件".to_string(),
            函数: crate::stdlib::builtin_解压文件,
        });
        
        env.define("压缩数据".to_string(), Value::内置函数 {
            名称: "压缩数据".to_string(),
            函数: crate::stdlib::builtin_压缩数据,
        });
        
        env.define("解压数据".to_string(), Value::内置函数 {
            名称: "解压数据".to_string(),
            函数: crate::stdlib::builtin_解压数据,
        });
        
        env.define("读取INI配置".to_string(), Value::内置函数 {
            名称: "读取INI配置".to_string(),
            函数: crate::stdlib::builtin_读取INI配置,
        });
        
        env.define("写入INI配置".to_string(), Value::内置函数 {
            名称: "写入INI配置".to_string(),
            函数: crate::stdlib::builtin_写入INI配置,
        });
        
        env.define("读取TOML配置".to_string(), Value::内置函数 {
            名称: "读取TOML配置".to_string(),
            函数: crate::stdlib::builtin_读取TOML配置,
        });
        
        env.define("写入TOML配置".to_string(), Value::内置函数 {
            名称: "写入TOML配置".to_string(),
            函数: crate::stdlib::builtin_写入TOML配置,
        });
        
        env.define("下载_库".to_string(), Value::内置函数 {
            名称: "下载_库".to_string(),
            函数: crate::stdlib::builtin_下载_库,
        });
        
        env.define("列出已安装库".to_string(), Value::内置函数 {
            名称: "列出已安装库".to_string(),
            函数: crate::stdlib::builtin_列出已安装库,
        });
        
        env.define("卸载库".to_string(), Value::内置函数 {
            名称: "卸载库".to_string(),
            函数: crate::stdlib::builtin_卸载库,
        });
        
        env.define("更新库".to_string(), Value::内置函数 {
            名称: "更新库".to_string(),
            函数: crate::stdlib::builtin_更新库,
        });
        
        env.define("搜索库".to_string(), Value::内置函数 {
            名称: "搜索库".to_string(),
            函数: crate::stdlib::builtin_搜索库,
        });
        
        env.define("库信息".to_string(), Value::内置函数 {
            名称: "库信息".to_string(),
            函数: crate::stdlib::builtin_库信息,
        });
    }
    
    pub fn run(&mut self, program: &Program) -> RuntimeResult<Value> {
        let mut result = Value::空值;
        
        for stmt in &program.statements {
            result = self.execute_statement(stmt, Rc::clone(&self.global_env))?;
        }
        
        Ok(result)
    }
    
    fn execute_statement(&mut self, stmt: &Statement, env: Rc<RefCell<Environment>>) -> RuntimeResult<Value> {
        match stmt {
            Statement::变量定义 { 名称, 类型: _, 初始值 } => {
                let value = if let Some(expr) = 初始值 {
                    self.evaluate(expr, Rc::clone(&env))?
                } else {
                    Value::空值
                };
                env.borrow_mut().define(名称.clone(), value);
                Ok(Value::空值)
            }
            
            Statement::解构赋值 { 变量列表, 值 } => {
                let value = self.evaluate(值, Rc::clone(&env))?;
                match value {
                    Value::列表(elements) => {
                        if elements.len() < 变量列表.len() {
                            return Err(RuntimeError::General(format!(
                                "解构赋值失败：列表长度 {} 小于变量数量 {}",
                                elements.len(),
                                变量列表.len()
                            )));
                        }
                        for (i, var_name) in 变量列表.iter().enumerate() {
                            env.borrow_mut().define(var_name.clone(), elements[i].clone());
                        }
                    }
                    Value::字典(map) => {
                        for var_name in 变量列表 {
                            if let Some(val) = map.get(var_name) {
                                env.borrow_mut().define(var_name.clone(), val.clone());
                            } else {
                                env.borrow_mut().define(var_name.clone(), Value::空值);
                            }
                        }
                    }
                    _ => return Err(RuntimeError::TypeError(format!(
                        "类型 {} 不支持解构赋值",
                        value.type_name()
                    ))),
                }
                Ok(Value::空值)
            }
            
            Statement::函数定义 { 名称, 参数, 可变参数名, 返回类型: _, 函数体 } => {
                let param_names: Vec<String> = 参数.iter().map(|p| p.名称.clone()).collect();
                let default_values: Vec<Option<Expression>> = 参数.iter().map(|p| p.默认值.clone()).collect();
                
                self.functions.insert(名称.clone(), (param_names.clone(), 函数体.clone()));
                
                let func = Value::函数 {
                    名称: 名称.clone(),
                    参数: param_names,
                    默认值: default_values,
                    可变参数名: 可变参数名.clone(),
                    闭包: env.borrow().clone(),
                };
                
                env.borrow_mut().define(名称.clone(), func);
                Ok(Value::空值)
            }
            
            Statement::表达式语句 { 表达式 } => {
                self.evaluate(表达式, env)
            }
            
            Statement::赋值语句 { 目标, 值 } => {
                let value = self.evaluate(值, Rc::clone(&env))?;
                self.assign_to(目标, value, env)
            }
            
            Statement::如果语句 { 条件, 如果体, 否则如果分支, 否则体 } => {
                let cond = self.evaluate(条件, Rc::clone(&env))?;
                
                if cond.is_truthy() {
                    self.execute_block(如果体, env)
                } else {
                    for (else_if_cond, else_if_body) in 否则如果分支 {
                        let cond = self.evaluate(else_if_cond, Rc::clone(&env))?;
                        if cond.is_truthy() {
                            return self.execute_block(else_if_body, env);
                        }
                    }
                    
                    if let Some(else_body) = 否则体 {
                        self.execute_block(else_body, env)
                    } else {
                        Ok(Value::空值)
                    }
                }
            }
            
            Statement::循环语句 { 条件, 循环体 } => {
                let mut result = Value::空值;
                let max_iterations = 1_000_000;
                let mut iterations = 0;
                
                // 优化：使用同一个环境引用，避免每次循环都克隆
                while self.evaluate(条件, Rc::clone(&env))?.is_truthy() {
                    iterations += 1;
                    if iterations > max_iterations {
                        return Err(RuntimeError::General("循环次数超过最大限制".to_string()));
                    }
                    for stmt in 循环体.iter() {
                        result = self.execute_statement(stmt, Rc::clone(&env))?;
                    }
                }
                
                Ok(result)
            }
            
            Statement::对于循环 { 变量, 可迭代对象, 循环体 } => {
                let iterable = self.evaluate(可迭代对象, Rc::clone(&env))?;
                let mut result = Value::空值;
                
                // 优化：预先创建循环块的环境
                let loop_env = Rc::new(RefCell::new(Environment::with_parent(env.borrow().clone())));
                
                match iterable {
                    Value::列表(elements) => {
                        for element in elements {
                            loop_env.borrow_mut().define(变量.clone(), element);
                            result = self.execute_block(循环体, Rc::clone(&loop_env))?;
                        }
                    }
                    Value::字符串(s) => {
                        for ch in s.chars() {
                            loop_env.borrow_mut().define(变量.clone(), Value::字符串(ch.to_string()));
                            result = self.execute_block(循环体, Rc::clone(&loop_env))?;
                        }
                    }
                    Value::字典(map) => {
                        for (key, _) in map {
                            loop_env.borrow_mut().define(变量.clone(), Value::字符串(key));
                            result = self.execute_block(循环体, Rc::clone(&loop_env))?;
                        }
                    }
                    _ => {
                        return Err(RuntimeError::TypeError(format!(
                            "类型 {} 不可迭代",
                            iterable.type_name()
                        )));
                    }
                }
                
                Ok(result)
            }
            
            Statement::返回语句 { 值 } => {
                if let Some(expr) = 值 {
                    self.evaluate(expr, env)
                } else {
                    Ok(Value::空值)
                }
            }
            
            Statement::打印语句 { 参数 } => {
                let output: Vec<String> = 参数
                    .iter()
                    .map(|arg| self.evaluate(arg, Rc::clone(&env)).map(|v| v.to_string_value()))
                    .collect::<RuntimeResult<Vec<String>>>()?;
                
                let output_str = output.join(" ");
                println!("{}", output_str);
                
                // 使用输出回调
                if let Some(callback) = &mut self.output_callback {
                    callback(output_str);
                }
                
                Ok(Value::空值)
            }
            
            Statement::导入语句 { 模块名, 别名 } => {
                let import_name = 别名.as_ref().unwrap_or(模块名);
                
                let mut module_content: Option<String> = None;
                
                let search_paths = vec![
                    ".".to_string(),
                    "examples".to_string(),
                ];
                
                for path in &search_paths {
                    let module_file = format!("{}/{}.hy", path, 模块名);
                    if let Ok(c) = std::fs::read_to_string(&module_file) {
                        module_content = Some(c);
                        break;
                    }
                    let module_file_zh = format!("{}/{}.汉", path, 模块名);
                    if let Ok(c) = std::fs::read_to_string(&module_file_zh) {
                        module_content = Some(c);
                        break;
                    }
                }
                
                let module_content = module_content.ok_or_else(|| {
                    RuntimeError::General(format!("找不到模块: {}", 模块名))
                })?;
                
                let tokens = crate::lexer::tokenize(&module_content)
                    .map_err(|e| RuntimeError::General(format!("模块词法错误: {}", e)))?;
                
                let program = crate::parser::parse(tokens)
                    .map_err(|e| RuntimeError::General(format!("模块语法错误: {}", e)))?;
                
                let module_env = crate::runtime::value::Environment::new();
                let module_env_rc = std::rc::Rc::new(std::cell::RefCell::new(module_env));
                
                for stmt in program.statements {
                    self.execute_statement(&stmt, std::rc::Rc::clone(&module_env_rc))?;
                }
                
                let module_env = module_env_rc.borrow();
                let mut module_props = std::collections::HashMap::new();
                for (key, value) in module_env.get_all_variables().iter() {
                    module_props.insert(key.clone(), value.clone());
                }
                
                let module = Value::字典(module_props);
                env.borrow_mut().define(import_name.clone(), module);
                
                Ok(Value::空值)
            }
            
            Statement::类定义 { 名称, 父类, 成员 } => {
                let mut methods = HashMap::new();
                let mut method_bodies = HashMap::new();
                let mut method_permissions = HashMap::new();
                let mut property_defaults = HashMap::new();
                let mut property_permissions = HashMap::new();
                
                if let Some(parent_class_name) = &父类 {
                    if let Some(Value::类 { 
                        方法: parent_methods, 
                        方法权限: parent_method_perms,
                        属性默认值: parent_props,
                        属性权限: parent_prop_perms,
                        .. 
                    }) = env.borrow().get(parent_class_name).cloned() {
                        for (method_name, method_value) in parent_methods {
                            methods.insert(method_name.clone(), method_value);
                            if let Some(perm) = parent_method_perms.get(&method_name) {
                                method_permissions.insert(method_name, *perm);
                            }
                        }
                        for (prop_name, prop_value) in parent_props {
                            property_defaults.insert(prop_name.clone(), prop_value);
                            if let Some(perm) = parent_prop_perms.get(&prop_name) {
                                property_permissions.insert(prop_name, *perm);
                            }
                        }
                    }
                }
                
                for member in 成员 {
                    match member {
                        ClassMember::方法 { 名称: method_name, 参数, 返回类型: _, 函数体, 访问权限 } => {
                            let param_names: Vec<String> = 参数.iter().map(|p| p.名称.clone()).collect();
                            let default_values: Vec<Option<Expression>> = 参数.iter().map(|p| p.默认值.clone()).collect();
                            let closure = env.borrow().clone();
                            
                            methods.insert(
                                method_name.clone(),
                                Value::函数 {
                                    名称: method_name.clone(),
                                    参数: param_names,
                                    默认值: default_values,
                                    可变参数名: None,
                                    闭包: closure,
                                },
                            );
                            
                            method_bodies.insert(method_name.clone(), 函数体.clone());
                            method_permissions.insert(method_name.clone(), *访问权限);
                        }
                        ClassMember::属性 { 名称: prop_name, 类型: _, 默认值, 访问权限 } => {
                            let default_value = if let Some(expr) = 默认值 {
                                self.evaluate(expr, Rc::clone(&env))?
                            } else {
                                Value::空值
                            };
                            property_defaults.insert(prop_name.clone(), default_value);
                            property_permissions.insert(prop_name.clone(), *访问权限);
                        }
                    }
                }
                
                self.class_methods.insert(名称.clone(), method_bodies);
                
                let class = Value::类 {
                    名称: 名称.clone(),
                    父类: 父类.clone(),
                    方法: methods,
                    方法权限: method_permissions,
                    属性默认值: property_defaults,
                    属性权限: property_permissions,
                };
                
                env.borrow_mut().define(名称.clone(), class);
                Ok(Value::空值)
            }
            
            Statement::尝试语句 { 尝试体, 捕获分支 } => {
                match self.execute_block(尝试体, Rc::clone(&env)) {
                    Ok(value) => Ok(value),
                    Err(error) => {
                        for (error_var, catch_body) in 捕获分支 {
                            env.borrow_mut().define(
                                error_var.clone(),
                                Value::字符串(error.to_string()),
                            );
                            return self.execute_block(catch_body, env);
                        }
                        Err(error)
                    }
                }
            }
            
            Statement::抛出语句 { 错误 } => {
                let error_value = self.evaluate(错误, env)?;
                Err(RuntimeError::General(error_value.to_string_value()))
            }
            
            Statement::With语句 { 表达式, 变量名, 语句体 } => {
                let context_value = self.evaluate(表达式, Rc::clone(&env))?;
                
                if let Some(var) = 变量名 {
                    env.borrow_mut().define(var.clone(), context_value.clone());
                }
                
                let result = self.execute_block(语句体, Rc::clone(&env));
                
                if let Some(var) = 变量名 {
                    env.borrow_mut().remove(var);
                }
                
                result
            }
            
            Statement::Yield语句 { 值 } => {
                let yielded_value = if let Some(v) = 值 {
                    self.evaluate(v, env)?
                } else {
                    Value::空值
                };
                Ok(Value::生成器 { 值: Box::new(yielded_value), 完成: false })
            }
            
            Statement::枚举定义 { 名称, 成员 } => {
                let mut enum_map = std::collections::HashMap::new();
                for (member_name, member_value) in 成员 {
                    let value = if let Some(expr) = member_value {
                        self.evaluate(expr, Rc::clone(&env))?
                    } else {
                        Value::空值
                    };
                    enum_map.insert(member_name.clone(), value);
                }
                env.borrow_mut().define(名称.clone(), Value::字典(enum_map));
                Ok(Value::空值)
            }
        }
    }
    
    fn execute_block(&mut self, statements: &[Statement], env: Rc<RefCell<Environment>>) -> RuntimeResult<Value> {
        let block_env = Rc::new(RefCell::new(Environment::with_parent(env.borrow().clone())));
        
        let mut result = Value::空值;
        for stmt in statements {
            result = self.execute_statement(stmt, Rc::clone(&block_env))?;
        }
        
        Ok(result)
    }
    
    fn evaluate(&mut self, expr: &Expression, env: Rc<RefCell<Environment>>) -> RuntimeResult<Value> {
        match expr {
            Expression::整数 { 值 } => Ok(Value::整数(*值)),
            Expression::浮点数 { 值 } => Ok(Value::浮点数(*值)),
            Expression::字符串 { 值 } => Ok(Value::字符串(值.clone())),
            Expression::格式化字符串 { 部分 } => {
                let mut result = String::new();
                for part in 部分 {
                    match part {
                        FormatPart::文本(text) => result.push_str(text),
                        FormatPart::表达式(expr) => {
                            let value = self.evaluate(expr, Rc::clone(&env))?;
                            result.push_str(&value.to_string());
                        }
                    }
                }
                Ok(Value::字符串(result))
            }
            Expression::布尔值 { 值 } => Ok(Value::布尔值(*值)),
            Expression::空值 => Ok(Value::空值),
            
            Expression::标识符 { 名称 } => {
                env.borrow()
                    .get(名称)
                    .cloned()
                    .ok_or_else(|| RuntimeError::UndefinedVariable(名称.clone()))
            }
            
            Expression::二元运算 { 左, 运算符, 右 } => {
                let left_val = self.evaluate(左, Rc::clone(&env))?;
                let right_val = self.evaluate(右, env)?;
                self.binary_op(&left_val, 运算符, &right_val)
            }
            
            Expression::一元运算 { 运算符, 操作数 } => {
                let operand = self.evaluate(操作数, env)?;
                self.unary_op(运算符, &operand)
            }
            
            Expression::函数调用 { 函数名, 参数 } => {
                let func = env
                    .borrow()
                    .get(函数名)
                    .cloned()
                    .ok_or_else(|| RuntimeError::UndefinedFunction(函数名.clone()))?;
                
                let args: Vec<Value> = 参数
                    .iter()
                    .map(|arg| self.evaluate(arg, Rc::clone(&env)))
                    .collect::<RuntimeResult<Vec<Value>>>()?;
                
                self.call_function(&func, &args, env)
            }
            
            Expression::匿名函数 { 参数, 可变参数名, 函数体 } => {
                let param_names: Vec<String> = 参数.iter().map(|p| p.名称.clone()).collect();
                let default_values: Vec<Option<Expression>> = 参数.iter().map(|p| p.默认值.clone()).collect();
                
                let func_name = format!("__匿名函数_{}", self.functions.len());
                self.functions.insert(func_name.clone(), (param_names.clone(), 函数体.clone()));
                
                Ok(Value::函数 {
                    名称: func_name,
                    参数: param_names,
                    默认值: default_values,
                    可变参数名: 可变参数名.clone(),
                    闭包: env.borrow().clone(),
                })
            }
            
            Expression::方法调用 { 对象, 方法名, 参数 } => {
                let obj = self.evaluate(对象, Rc::clone(&env))?;
                let args: Vec<Value> = 参数
                    .iter()
                    .map(|arg| self.evaluate(arg, Rc::clone(&env)))
                    .collect::<RuntimeResult<Vec<Value>>>()?;
                
                self.call_method(&obj, 方法名, &args, env)
            }
            
            Expression::属性访问 { 对象, 属性名 } => {
                let obj = self.evaluate(对象, env)?;
                self.get_property(&obj, 属性名)
            }
            
            Expression::索引访问 { 对象, 索引 } => {
                let obj = self.evaluate(对象, Rc::clone(&env))?;
                let idx = self.evaluate(索引, env)?;
                self.get_index(&obj, &idx)
            }
            
            Expression::切片访问 { 对象, 开始, 结束, 步长 } => {
                let obj = self.evaluate(对象, Rc::clone(&env))?;
                let start = if let Some(s) = 开始 {
                    Some(self.evaluate(s, Rc::clone(&env))?)
                } else {
                    None
                };
                let end = if let Some(e) = 结束 {
                    Some(self.evaluate(e, Rc::clone(&env))?)
                } else {
                    None
                };
                let step = if let Some(st) = 步长 {
                    Some(self.evaluate(st, env)?)
                } else {
                    None
                };
                self.get_slice(&obj, start.as_ref(), end.as_ref(), step.as_ref())
            }
            
            Expression::列表 { 元素 } => {
                let elements: Vec<Value> = 元素
                    .iter()
                    .map(|e| self.evaluate(e, Rc::clone(&env)))
                    .collect::<RuntimeResult<Vec<Value>>>()?;
                Ok(Value::列表(elements))
            }
            
            Expression::列表推导式 { 表达式, 变量, 可迭代对象, 条件 } => {
                let iterable = self.evaluate(可迭代对象, Rc::clone(&env))?;
                let items = match iterable {
                    Value::列表(v) => v,
                    Value::字符串(s) => s.chars().map(|c| Value::字符串(c.to_string())).collect(),
                    _ => return Err(RuntimeError::TypeError(format!(
                        "类型 {} 不可迭代",
                        iterable.type_name()
                    ))),
                };
                
                let mut result = Vec::new();
                for item in items {
                    let mut list_env = env.borrow().clone();
                    list_env.define(变量.clone(), item);
                    let list_env_rc = Rc::new(RefCell::new(list_env));
                    
                    let should_include = if let Some(ref cond) = 条件 {
                        let cond_value = self.evaluate(cond, Rc::clone(&list_env_rc))?;
                        cond_value.is_truthy()
                    } else {
                        true
                    };
                    
                    if should_include {
                        let value = self.evaluate(表达式, list_env_rc)?;
                        result.push(value);
                    }
                }
                
                Ok(Value::列表(result))
            }
            
            Expression::字典 { 键值对 } => {
                let mut map = HashMap::new();
                for (key_expr, value_expr) in 键值对 {
                    let key = self.evaluate(key_expr, Rc::clone(&env))?;
                    let value = self.evaluate(value_expr, Rc::clone(&env))?;
                    map.insert(key.to_string_value(), value);
                }
                Ok(Value::字典(map))
            }
            
            Expression::字典推导式 { 键表达式, 值表达式, 变量, 可迭代对象, 条件 } => {
                let iterable = self.evaluate(可迭代对象, Rc::clone(&env))?;
                let items = match iterable {
                    Value::列表(v) => v,
                    Value::字符串(s) => s.chars().map(|c| Value::字符串(c.to_string())).collect(),
                    _ => return Err(RuntimeError::TypeError(format!(
                        "类型 {} 不可迭代",
                        iterable.type_name()
                    ))),
                };
                
                let mut result = HashMap::new();
                for item in items {
                    let mut dict_env = env.borrow().clone();
                    dict_env.define(变量.clone(), item);
                    let dict_env_rc = Rc::new(RefCell::new(dict_env));
                    
                    let should_include = if let Some(ref cond) = 条件 {
                        let cond_value = self.evaluate(cond, Rc::clone(&dict_env_rc))?;
                        cond_value.is_truthy()
                    } else {
                        true
                    };
                    
                    if should_include {
                        let key = self.evaluate(键表达式, Rc::clone(&dict_env_rc))?;
                        let value = self.evaluate(值表达式, dict_env_rc)?;
                        result.insert(key.to_string_value(), value);
                    }
                }
                
                Ok(Value::字典(result))
            }
            
            Expression::集合 { 元素 } => {
                let mut seen = std::collections::HashSet::new();
                let mut result = Vec::new();
                for elem in 元素 {
                    let value = self.evaluate(elem, Rc::clone(&env))?;
                    let key = value.to_string_value();
                    if !seen.contains(&key) {
                        seen.insert(key);
                        result.push(value);
                    }
                }
                Ok(Value::集合(result))
            }
            
            Expression::集合推导式 { 表达式, 变量, 可迭代对象, 条件 } => {
                let iterable = self.evaluate(可迭代对象, Rc::clone(&env))?;
                let items = match iterable {
                    Value::列表(v) => v,
                    Value::字符串(s) => s.chars().map(|c| Value::字符串(c.to_string())).collect(),
                    _ => return Err(RuntimeError::TypeError(format!(
                        "类型 {} 不可迭代",
                        iterable.type_name()
                    ))),
                };
                
                let mut seen = std::collections::HashSet::new();
                let mut result = Vec::new();
                for item in items {
                    let mut set_env = env.borrow().clone();
                    set_env.define(变量.clone(), item);
                    let set_env_rc = Rc::new(RefCell::new(set_env));
                    
                    let should_include = if let Some(ref cond) = 条件 {
                        let cond_value = self.evaluate(cond, Rc::clone(&set_env_rc))?;
                        cond_value.is_truthy()
                    } else {
                        true
                    };
                    
                    if should_include {
                        let value = self.evaluate(表达式, set_env_rc)?;
                        let key = value.to_string_value();
                        if !seen.contains(&key) {
                            seen.insert(key);
                            result.push(value);
                        }
                    }
                }
                
                Ok(Value::集合(result))
            }
            
            Expression::三元表达式 { 条件, 真值, 假值 } => {
                let cond_value = self.evaluate(条件, Rc::clone(&env))?;
                if cond_value.is_truthy() {
                    self.evaluate(真值, env)
                } else {
                    self.evaluate(假值, env)
                }
            }
            
            Expression::新建对象 { 类名, 参数 } => {
                let class = env
                    .borrow()
                    .get(类名)
                    .cloned()
                    .ok_or_else(|| RuntimeError::UndefinedVariable(类名.clone()))?;
                
                let args: Vec<Value> = 参数
                    .iter()
                    .map(|arg| self.evaluate(arg, Rc::clone(&env)))
                    .collect::<RuntimeResult<Vec<Value>>>()?;
                
                self.create_object(&class, &args, env)
            }
        }
    }
    
    fn binary_op(&self, left: &Value, op: &BinaryOperator, right: &Value) -> RuntimeResult<Value> {
        match op {
            BinaryOperator::加 => self.add(left, right),
            BinaryOperator::减 => self.subtract(left, right),
            BinaryOperator::乘 => self.multiply(left, right),
            BinaryOperator::除 => self.divide(left, right),
            BinaryOperator::取余 => self.modulo(left, right),
            BinaryOperator::等于 => Ok(Value::布尔值(left == right)),
            BinaryOperator::不等于 => Ok(Value::布尔值(left != right)),
            BinaryOperator::大于 => self.compare(left, right, |a, b| a > b),
            BinaryOperator::小于 => self.compare(left, right, |a, b| a < b),
            BinaryOperator::大于等于 => self.compare(left, right, |a, b| a >= b),
            BinaryOperator::小于等于 => self.compare(left, right, |a, b| a <= b),
            BinaryOperator::且 => Ok(Value::布尔值(left.is_truthy() && right.is_truthy())),
            BinaryOperator::或 => Ok(Value::布尔值(left.is_truthy() || right.is_truthy())),
        }
    }
    
    fn add(&self, left: &Value, right: &Value) -> RuntimeResult<Value> {
        match (left, right) {
            (Value::整数(a), Value::整数(b)) => Ok(Value::整数(a + b)),
            (Value::浮点数(a), Value::浮点数(b)) => Ok(Value::浮点数(a + b)),
            (Value::整数(a), Value::浮点数(b)) => Ok(Value::浮点数(*a as f64 + b)),
            (Value::浮点数(a), Value::整数(b)) => Ok(Value::浮点数(a + *b as f64)),
            (Value::字符串(a), Value::字符串(b)) => Ok(Value::字符串(format!("{}{}", a, b))),
            (Value::列表(a), Value::列表(b)) => Ok(Value::列表([a.clone(), b.clone()].concat())),
            _ => Err(RuntimeError::TypeError(format!(
                "无法将类型 {} 和 {} 相加",
                left.type_name(),
                right.type_name()
            ))),
        }
    }
    
    fn subtract(&self, left: &Value, right: &Value) -> RuntimeResult<Value> {
        match (left, right) {
            (Value::整数(a), Value::整数(b)) => Ok(Value::整数(a - b)),
            (Value::浮点数(a), Value::浮点数(b)) => Ok(Value::浮点数(a - b)),
            (Value::整数(a), Value::浮点数(b)) => Ok(Value::浮点数(*a as f64 - b)),
            (Value::浮点数(a), Value::整数(b)) => Ok(Value::浮点数(a - *b as f64)),
            _ => Err(RuntimeError::TypeError(format!(
                "无法将类型 {} 和 {} 相减",
                left.type_name(),
                right.type_name()
            ))),
        }
    }
    
    fn multiply(&self, left: &Value, right: &Value) -> RuntimeResult<Value> {
        match (left, right) {
            (Value::整数(a), Value::整数(b)) => Ok(Value::整数(a * b)),
            (Value::浮点数(a), Value::浮点数(b)) => Ok(Value::浮点数(a * b)),
            (Value::整数(a), Value::浮点数(b)) => Ok(Value::浮点数(*a as f64 * b)),
            (Value::浮点数(a), Value::整数(b)) => Ok(Value::浮点数(a * *b as f64)),
            (Value::字符串(s), Value::整数(n)) | (Value::整数(n), Value::字符串(s)) => {
                Ok(Value::字符串(s.repeat(*n as usize)))
            }
            _ => Err(RuntimeError::TypeError(format!(
                "无法将类型 {} 和 {} 相乘",
                left.type_name(),
                right.type_name()
            ))),
        }
    }
    
    fn divide(&self, left: &Value, right: &Value) -> RuntimeResult<Value> {
        match (left, right) {
            (Value::整数(a), Value::整数(b)) => {
                if *b == 0 {
                    return Err(RuntimeError::DivisionByZero);
                }
                Ok(Value::整数(a / b))
            }
            (Value::浮点数(a), Value::浮点数(b)) => {
                if *b == 0.0 {
                    return Err(RuntimeError::DivisionByZero);
                }
                Ok(Value::浮点数(a / b))
            }
            (Value::整数(a), Value::浮点数(b)) => {
                if *b == 0.0 {
                    return Err(RuntimeError::DivisionByZero);
                }
                Ok(Value::浮点数(*a as f64 / b))
            }
            (Value::浮点数(a), Value::整数(b)) => {
                if *b == 0 {
                    return Err(RuntimeError::DivisionByZero);
                }
                Ok(Value::浮点数(a / *b as f64))
            }
            _ => Err(RuntimeError::TypeError(format!(
                "无法将类型 {} 和 {} 相除",
                left.type_name(),
                right.type_name()
            ))),
        }
    }
    
    fn modulo(&self, left: &Value, right: &Value) -> RuntimeResult<Value> {
        match (left, right) {
            (Value::整数(a), Value::整数(b)) => {
                if *b == 0 {
                    return Err(RuntimeError::DivisionByZero);
                }
                Ok(Value::整数(a % b))
            }
            (Value::浮点数(a), Value::浮点数(b)) => {
                if *b == 0.0 {
                    return Err(RuntimeError::DivisionByZero);
                }
                Ok(Value::浮点数(a % b))
            }
            (Value::整数(a), Value::浮点数(b)) => {
                if *b == 0.0 {
                    return Err(RuntimeError::DivisionByZero);
                }
                Ok(Value::浮点数(*a as f64 % b))
            }
            (Value::浮点数(a), Value::整数(b)) => {
                if *b == 0 {
                    return Err(RuntimeError::DivisionByZero);
                }
                Ok(Value::浮点数(a % *b as f64))
            }
            _ => Err(RuntimeError::TypeError(format!(
                "无法对类型 {} 和 {} 取余",
                left.type_name(),
                right.type_name()
            ))),
        }
    }
    
    fn compare<F>(&self, left: &Value, right: &Value, cmp: F) -> RuntimeResult<Value>
    where
        F: Fn(f64, f64) -> bool,
    {
        let left_num = match left {
            Value::整数(n) => *n as f64,
            Value::浮点数(n) => *n,
            _ => {
                return Err(RuntimeError::TypeError(format!(
                    "类型 {} 无法比较",
                    left.type_name()
                )))
            }
        };
        
        let right_num = match right {
            Value::整数(n) => *n as f64,
            Value::浮点数(n) => *n,
            _ => {
                return Err(RuntimeError::TypeError(format!(
                    "类型 {} 无法比较",
                    right.type_name()
                )))
            }
        };
        
        Ok(Value::布尔值(cmp(left_num, right_num)))
    }
    
    fn unary_op(&self, op: &UnaryOperator, operand: &Value) -> RuntimeResult<Value> {
        match op {
            UnaryOperator::非 => Ok(Value::布尔值(!operand.is_truthy())),
            UnaryOperator::负 => match operand {
                Value::整数(n) => Ok(Value::整数(-n)),
                Value::浮点数(n) => Ok(Value::浮点数(-n)),
                _ => Err(RuntimeError::TypeError(format!(
                    "类型 {} 无法取负",
                    operand.type_name()
                ))),
            },
            UnaryOperator::正 => match operand {
                Value::整数(_) | Value::浮点数(_) => Ok(operand.clone()),
                _ => Err(RuntimeError::TypeError(format!(
                    "类型 {} 无法取正",
                    operand.type_name()
                ))),
            },
        }
    }
    
    fn call_function(
        &mut self,
        func: &Value,
        args: &[Value],
        _env: Rc<RefCell<Environment>>,
    ) -> RuntimeResult<Value> {
        match func {
            Value::内置函数 { 函数, .. } => 函数(args.to_vec()),
            Value::函数 { 名称, 参数, 默认值, 可变参数名, 闭包, .. } => {
                let min_args = 参数.len() - 默认值.iter().filter(|d| d.is_some()).count();
                
                if 可变参数名.is_none() && (args.len() < min_args || args.len() > 参数.len()) {
                    return Err(RuntimeError::ArgumentCountError {
                        function: 名称.clone(),
                        expected: 参数.len(),
                        actual: args.len(),
                    });
                }
                
                if 可变参数名.is_some() && args.len() < min_args {
                    return Err(RuntimeError::ArgumentCountError {
                        function: 名称.clone(),
                        expected: min_args,
                        actual: args.len(),
                    });
                }
                
                let body = self.functions.get(名称).map(|(_, b)| b.clone());
                if let Some(body) = body {
                    let mut func_env = Environment::with_parent(闭包.clone());
                    
                    for (i, param_name) in 参数.iter().enumerate() {
                        let arg_value = if i < args.len() {
                            args[i].clone()
                        } else if let Some(Some(ref default_expr)) = 默认值.get(i) {
                            self.evaluate(default_expr, Rc::new(RefCell::new(闭包.clone())))?
                        } else {
                            return Err(RuntimeError::ArgumentCountError {
                                function: 名称.clone(),
                                expected: 参数.len(),
                                actual: args.len(),
                            });
                        };
                        func_env.define(param_name.clone(), arg_value);
                    }
                    
                    if let Some(vararg) = 可变参数名 {
                        let vararg_values: Vec<Value> = if args.len() > 参数.len() {
                            args[参数.len()..].to_vec()
                        } else {
                            vec![]
                        };
                        func_env.define(vararg.clone(), Value::列表(vararg_values));
                    }
                    
                    let func_env_rc = Rc::new(RefCell::new(func_env));
                    let mut result = Value::空值;
                    for stmt in &body {
                        result = self.execute_statement(stmt, Rc::clone(&func_env_rc))?;
                    }
                    Ok(result)
                } else {
                    Ok(Value::空值)
                }
            }
            _ => Err(RuntimeError::TypeError(format!(
                "类型 {} 不可调用",
                func.type_name()
            ))),
        }
    }
    
    fn call_method(
        &mut self,
        obj: &Value,
        method_name: &str,
        args: &[Value],
        _env: Rc<RefCell<Environment>>,
    ) -> RuntimeResult<Value> {
        match (obj, method_name) {
            (Value::字符串(s), "长度") => Ok(Value::整数(s.len() as i64)),
            (Value::字符串(s), "大写") => Ok(Value::字符串(s.to_uppercase())),
            (Value::字符串(s), "小写") => Ok(Value::字符串(s.to_lowercase())),
            (Value::字符串(s), "去除空白") => Ok(Value::字符串(s.trim().to_string())),
            (Value::列表(v), "长度") => Ok(Value::整数(v.len() as i64)),
            (Value::列表(v), "首元素") if !v.is_empty() => Ok(v[0].clone()),
            (Value::列表(v), "末元素") if !v.is_empty() => Ok(v[v.len() - 1].clone()),
            (Value::字典(m), "长度") => Ok(Value::整数(m.len() as i64)),
            (Value::字典(m), "键列表") => {
                let keys: Vec<Value> = m.keys().map(|k: &String| Value::字符串(k.clone())).collect();
                Ok(Value::列表(keys))
            }
            (Value::字典(m), "值列表") => Ok(Value::列表(m.values().cloned().collect())),
            (Value::对象 { 类名, 属性, .. }, method_name) => {
                let class_name = 类名.clone();
                let method_body = self.class_methods.get(&class_name)
                    .and_then(|methods| methods.get(method_name).cloned());
                
                if let Some(body) = method_body {
                    let mut method_env = Environment::new();
                    method_env.define("自身".to_string(), obj.clone());
                    
                    for (key, value) in 属性.iter() {
                        method_env.define(key.clone(), value.clone());
                    }
                    
                    for (i, arg) in args.iter().enumerate() {
                        method_env.define(format!("参数{}", i), arg.clone());
                    }
                    
                    let method_env_rc = Rc::new(RefCell::new(method_env));
                    let mut result = Value::空值;
                    for stmt in &body {
                        result = self.execute_statement(stmt, Rc::clone(&method_env_rc))?;
                    }
                    Ok(result)
                } else {
                    Err(RuntimeError::TypeError(format!(
                        "类 {} 没有方法 {}",
                        class_name, method_name
                    )))
                }
            }
            _ => Err(RuntimeError::TypeError(format!(
                "类型 {} 没有方法 {}",
                obj.type_name(),
                method_name
            ))),
        }
    }
    
    fn get_property(&self, obj: &Value, property_name: &str) -> RuntimeResult<Value> {
        match obj {
            Value::对象 { 属性, .. } => 属性
                .get(property_name)
                .cloned()
                .ok_or_else(|| RuntimeError::General(format!("对象没有属性 {}", property_name))),
            Value::列表(v) => match property_name {
                "长度" => Ok(Value::整数(v.len() as i64)),
                _ => Err(RuntimeError::TypeError(format!(
                    "列表没有属性 {}",
                    property_name
                ))),
            },
            Value::字符串(s) => match property_name {
                "长度" => Ok(Value::整数(s.len() as i64)),
                _ => Err(RuntimeError::TypeError(format!(
                    "字符串没有属性 {}",
                    property_name
                ))),
            },
            Value::字典(m) => match property_name {
                "长度" => Ok(Value::整数(m.len() as i64)),
                _ => m.get(property_name)
                    .cloned()
                    .ok_or_else(|| RuntimeError::TypeError(format!("字典没有属性 {}", property_name))),
            },
            _ => Err(RuntimeError::TypeError(format!(
                "类型 {} 不支持属性访问",
                obj.type_name()
            ))),
        }
    }
    
    fn get_index(&self, obj: &Value, index: &Value) -> RuntimeResult<Value> {
        match (obj, index) {
            (Value::列表(v), Value::整数(i)) => {
                let idx = *i as usize;
                if idx < v.len() {
                    Ok(v[idx].clone())
                } else {
                    Err(RuntimeError::IndexOutOfBounds {
                        index: idx,
                        length: v.len(),
                    })
                }
            }
            (Value::字符串(s), Value::整数(i)) => {
                let idx = *i as usize;
                if idx < s.len() {
                    Ok(Value::字符串(s.chars().nth(idx).unwrap().to_string()))
                } else {
                    Err(RuntimeError::IndexOutOfBounds {
                        index: idx,
                        length: s.len(),
                    })
                }
            }
            (Value::字典(m), Value::字符串(k)) => m
                .get(k)
                .cloned()
                .ok_or_else(|| RuntimeError::General(format!("字典没有键 {}", k))),
            _ => Err(RuntimeError::TypeError(format!(
                "类型 {} 不支持索引访问",
                obj.type_name()
            ))),
        }
    }
    
    fn get_slice(&self, obj: &Value, start: Option<&Value>, end: Option<&Value>, step: Option<&Value>) -> RuntimeResult<Value> {
        let step_val = match step {
            Some(Value::整数(s)) => *s,
            Some(_) => return Err(RuntimeError::TypeError("步长必须是整数".to_string())),
            None => 1,
        };
        
        if step_val == 0 {
            return Err(RuntimeError::General("切片步长不能为零".to_string()));
        }
        
        match obj {
            Value::列表(v) => {
                let len = v.len() as i64;
                let start_idx = match start {
                    Some(Value::整数(s)) => {
                        let s = *s;
                        if s < 0 { (len + s).max(0) as usize } else { s.min(len) as usize }
                    }
                    Some(_) => return Err(RuntimeError::TypeError("切片开始必须是整数".to_string())),
                    None => if step_val > 0 { 0 } else { len as usize },
                };
                
                let end_idx = match end {
                    Some(Value::整数(e)) => {
                        let e = *e;
                        if e < 0 { (len + e).max(0) as usize } else { e.min(len) as usize }
                    }
                    Some(_) => return Err(RuntimeError::TypeError("切片结束必须是整数".to_string())),
                    None => if step_val > 0 { len as usize } else { 0 },
                };
                
                let result: Vec<Value> = if step_val > 0 {
                    (start_idx..end_idx).step_by(step_val as usize)
                        .filter_map(|i| if i < v.len() { Some(v[i].clone()) } else { None })
                        .collect()
                } else {
                    let step_abs = (-step_val) as usize;
                    (start_idx..end_idx).rev()
                        .enumerate()
                        .filter_map(|(i, idx)| if i % step_abs == 0 && idx < v.len() { Some(v[idx].clone()) } else { None })
                        .collect()
                };
                
                Ok(Value::列表(result))
            }
            Value::字符串(s) => {
                let chars: Vec<char> = s.chars().collect();
                let len = chars.len() as i64;
                let start_idx = match start {
                    Some(Value::整数(s)) => {
                        let s = *s;
                        if s < 0 { (len + s).max(0) as usize } else { s.min(len) as usize }
                    }
                    Some(_) => return Err(RuntimeError::TypeError("切片开始必须是整数".to_string())),
                    None => if step_val > 0 { 0 } else { len as usize },
                };
                
                let end_idx = match end {
                    Some(Value::整数(e)) => {
                        let e = *e;
                        if e < 0 { (len + e).max(0) as usize } else { e.min(len) as usize }
                    }
                    Some(_) => return Err(RuntimeError::TypeError("切片结束必须是整数".to_string())),
                    None => if step_val > 0 { len as usize } else { 0 },
                };
                
                let result: String = if step_val > 0 {
                    (start_idx..end_idx).step_by(step_val as usize)
                        .filter_map(|i| chars.get(i).copied())
                        .collect()
                } else {
                    let step_abs = (-step_val) as usize;
                    (start_idx..end_idx).rev()
                        .enumerate()
                        .filter_map(|(i, idx)| if i % step_abs == 0 { chars.get(idx).copied() } else { None })
                        .collect()
                };
                
                Ok(Value::字符串(result))
            }
            _ => Err(RuntimeError::TypeError(format!(
                "类型 {} 不支持切片操作",
                obj.type_name()
            ))),
        }
    }
    
    fn assign_to(
        &mut self,
        target: &Expression,
        value: Value,
        env: Rc<RefCell<Environment>>,
    ) -> RuntimeResult<Value> {
        match target {
            Expression::标识符 { 名称 } => {
                if !env.borrow_mut().set(名称, value) {
                    return Err(RuntimeError::UndefinedVariable(名称.clone()));
                }
                Ok(Value::空值)
            }
            Expression::索引访问 { 对象, 索引 } => {
                match 对象.as_ref() {
                    Expression::标识符 { 名称 } => {
                        let idx = self.evaluate(索引, Rc::clone(&env))?;
                        
                        let mut env_ref = env.borrow_mut();
                        if let Some(obj_val) = env_ref.get_mut(名称) {
                            match (obj_val, idx) {
                                (Value::列表(ref mut v), Value::整数(i)) => {
                                    let idx_usize = i as usize;
                                    if idx_usize < v.len() {
                                        v[idx_usize] = value;
                                        Ok(Value::空值)
                                    } else {
                                        Err(RuntimeError::IndexOutOfBounds {
                                            index: idx_usize,
                                            length: v.len(),
                                        })
                                    }
                                }
                                (Value::字典(ref mut m), Value::字符串(k)) => {
                                    m.insert(k, value);
                                    Ok(Value::空值)
                                }
                                (obj_val, _) => Err(RuntimeError::TypeError(format!(
                                    "类型 {} 不支持索引赋值",
                                    obj_val.type_name()
                                ))),
                            }
                        } else {
                            Err(RuntimeError::UndefinedVariable(名称.clone()))
                        }
                    }
                    _ => Err(RuntimeError::General("索引赋值只支持变量".to_string())),
                }
            }
            Expression::属性访问 { 对象, 属性名 } => {
                match 对象.as_ref() {
                    Expression::标识符 { 名称 } => {
                        let mut env_ref = env.borrow_mut();
                        if let Some(obj_val) = env_ref.get_mut(名称) {
                            match obj_val {
                                Value::对象 { 属性, .. } => {
                                    属性.insert(属性名.clone(), value);
                                    Ok(Value::空值)
                                }
                                Value::字典(map) => {
                                    map.insert(属性名.clone(), value);
                                    Ok(Value::空值)
                                }
                                _ => Err(RuntimeError::TypeError(format!(
                                    "类型 {} 不支持属性赋值",
                                    obj_val.type_name()
                                ))),
                            }
                        } else {
                            Err(RuntimeError::UndefinedVariable(名称.clone()))
                        }
                    }
                    _ => Err(RuntimeError::General("属性赋值只支持变量".to_string())),
                }
            }
            _ => Err(RuntimeError::General("无效的赋值目标".to_string())),
        }
    }
    
    fn create_object(
        &mut self,
        class: &Value,
        _args: &[Value],
        _env: Rc<RefCell<Environment>>,
    ) -> RuntimeResult<Value> {
        match class {
            Value::类 { 名称, 方法, 属性默认值, 属性权限, .. } => {
                let mut attributes = HashMap::new();
                
                for (method_name, method_value) in 方法 {
                    attributes.insert(method_name.clone(), method_value.clone());
                }
                
                for (prop_name, prop_value) in 属性默认值 {
                    attributes.insert(prop_name.clone(), prop_value.clone());
                }
                
                Ok(Value::对象 {
                    类名: 名称.clone(),
                    属性: attributes,
                    属性权限: 属性权限.clone(),
                })
            }
            _ => Err(RuntimeError::TypeError(format!(
                "{} 不是类",
                class.type_name()
            ))),
        }
    }
}

impl Default for Interpreter {
    fn default() -> Self {
        Self::new()
    }
}

fn builtin_length(args: Vec<Value>) -> RuntimeResult<Value> {
    if args.is_empty() {
        return Err(RuntimeError::ArgumentCountError {
            function: "长度".to_string(),
            expected: 1,
            actual: 0,
        });
    }
    
    match &args[0] {
        Value::字符串(s) => Ok(Value::整数(s.len() as i64)),
        Value::列表(v) => Ok(Value::整数(v.len() as i64)),
        Value::字典(m) => Ok(Value::整数(m.len() as i64)),
        _ => Err(RuntimeError::TypeError(format!(
            "类型 {} 没有长度",
            args[0].type_name()
        ))),
    }
}

fn builtin_type(args: Vec<Value>) -> RuntimeResult<Value> {
    if args.is_empty() {
        return Err(RuntimeError::ArgumentCountError {
            function: "类型".to_string(),
            expected: 1,
            actual: 0,
        });
    }
    
    Ok(Value::字符串(args[0].type_name().to_string()))
}

fn builtin_to_string(args: Vec<Value>) -> RuntimeResult<Value> {
    if args.is_empty() {
        return Err(RuntimeError::ArgumentCountError {
            function: "转字符串".to_string(),
            expected: 1,
            actual: 0,
        });
    }
    
    Ok(Value::字符串(args[0].to_string_value()))
}

fn builtin_to_int(args: Vec<Value>) -> RuntimeResult<Value> {
    if args.is_empty() {
        return Err(RuntimeError::ArgumentCountError {
            function: "转整数".to_string(),
            expected: 1,
            actual: 0,
        });
    }
    
    match &args[0] {
        Value::整数(n) => Ok(Value::整数(*n)),
        Value::浮点数(n) => Ok(Value::整数(*n as i64)),
        Value::字符串(s) => s
            .parse::<i64>()
            .map(Value::整数)
            .map_err(|_| RuntimeError::TypeError(format!("无法将 '{}' 转换为整数", s))),
        Value::布尔值(b) => Ok(Value::整数(if *b { 1 } else { 0 })),
        _ => Err(RuntimeError::TypeError(format!(
            "无法将类型 {} 转换为整数",
            args[0].type_name()
        ))),
    }
}

fn builtin_to_float(args: Vec<Value>) -> RuntimeResult<Value> {
    if args.is_empty() {
        return Err(RuntimeError::ArgumentCountError {
            function: "转浮点数".to_string(),
            expected: 1,
            actual: 0,
        });
    }
    
    match &args[0] {
        Value::整数(n) => Ok(Value::浮点数(*n as f64)),
        Value::浮点数(n) => Ok(Value::浮点数(*n)),
        Value::字符串(s) => s
            .parse::<f64>()
            .map(Value::浮点数)
            .map_err(|_| RuntimeError::TypeError(format!("无法将 '{}' 转换为浮点数", s))),
        Value::布尔值(b) => Ok(Value::浮点数(if *b { 1.0 } else { 0.0 })),
        _ => Err(RuntimeError::TypeError(format!(
            "无法将类型 {} 转换为浮点数",
            args[0].type_name()
        ))),
    }
}

fn builtin_range(args: Vec<Value>) -> RuntimeResult<Value> {
    let (start, end) = match args.len() {
        1 => match &args[0] {
            Value::整数(n) => (0, *n),
            _ => return Err(RuntimeError::TypeError("范围参数必须是整数".to_string())),
        },
        2 => match (&args[0], &args[1]) {
            (Value::整数(s), Value::整数(e)) => (*s, *e),
            _ => return Err(RuntimeError::TypeError("范围参数必须是整数".to_string())),
        },
        _ => {
            return Err(RuntimeError::ArgumentCountError {
                function: "范围".to_string(),
                expected: 2,
                actual: args.len(),
            })
        }
    };
    
    let range: Vec<Value> = (start..end).map(Value::整数).collect();
    Ok(Value::列表(range))
}

fn builtin_append(args: Vec<Value>) -> RuntimeResult<Value> {
    if args.len() < 2 {
        return Err(RuntimeError::ArgumentCountError {
            function: "追加".to_string(),
            expected: 2,
            actual: args.len(),
        });
    }
    
    match &args[0] {
        Value::列表(v) => {
            let mut new_list = v.clone();
            new_list.extend(args[1..].to_vec());
            Ok(Value::列表(new_list))
        }
        _ => Err(RuntimeError::TypeError(format!(
            "类型 {} 不支持追加操作",
            args[0].type_name()
        ))),
    }
}

fn builtin_pop(args: Vec<Value>) -> RuntimeResult<Value> {
    if args.is_empty() {
        return Err(RuntimeError::ArgumentCountError {
            function: "弹出".to_string(),
            expected: 1,
            actual: 0,
        });
    }
    
    match &args[0] {
        Value::列表(v) if !v.is_empty() => {
            let mut new_list = v.clone();
            let last = new_list.pop().unwrap();
            Ok(last)
        }
        Value::列表(_) => Err(RuntimeError::General("列表为空".to_string())),
        _ => Err(RuntimeError::TypeError(format!(
            "类型 {} 不支持弹出操作",
            args[0].type_name()
        ))),
    }
}

fn builtin_insert(args: Vec<Value>) -> RuntimeResult<Value> {
    if args.len() < 3 {
        return Err(RuntimeError::ArgumentCountError {
            function: "插入".to_string(),
            expected: 3,
            actual: args.len(),
        });
    }
    
    match (&args[0], &args[1]) {
        (Value::列表(v), Value::整数(idx)) => {
            let mut new_list = v.clone();
            let index = *idx as usize;
            if index > new_list.len() {
                return Err(RuntimeError::IndexOutOfBounds {
                    index,
                    length: new_list.len(),
                });
            }
            new_list.insert(index, args[2].clone());
            Ok(Value::列表(new_list))
        }
        (Value::字典(m), Value::字符串(k)) => {
            let mut new_dict = m.clone();
            new_dict.insert(k.clone(), args[2].clone());
            Ok(Value::字典(new_dict))
        }
        _ => Err(RuntimeError::TypeError(format!(
            "类型 {} 不支持插入操作",
            args[0].type_name()
        ))),
    }
}

fn builtin_remove(args: Vec<Value>) -> RuntimeResult<Value> {
    if args.len() < 2 {
        return Err(RuntimeError::ArgumentCountError {
            function: "删除".to_string(),
            expected: 2,
            actual: args.len(),
        });
    }
    
    match (&args[0], &args[1]) {
        (Value::列表(v), Value::整数(idx)) => {
            let mut new_list = v.clone();
            let index = *idx as usize;
            if index >= new_list.len() {
                return Err(RuntimeError::IndexOutOfBounds {
                    index,
                    length: new_list.len(),
                });
            }
            new_list.remove(index);
            Ok(Value::列表(new_list))
        }
        (Value::字典(m), Value::字符串(k)) => {
            let mut new_dict = m.clone();
            new_dict.remove(k);
            Ok(Value::字典(new_dict))
        }
        _ => Err(RuntimeError::TypeError(format!(
            "类型 {} 不支持删除操作",
            args[0].type_name()
        ))),
    }
}

fn builtin_abs(args: Vec<Value>) -> RuntimeResult<Value> {
    if args.is_empty() {
        return Err(RuntimeError::ArgumentCountError {
            function: "绝对值".to_string(),
            expected: 1,
            actual: 0,
        });
    }
    
    match &args[0] {
        Value::整数(n) => Ok(Value::整数(n.abs())),
        Value::浮点数(n) => Ok(Value::浮点数(n.abs())),
        _ => Err(RuntimeError::TypeError(format!(
            "类型 {} 不支持绝对值操作",
            args[0].type_name()
        ))),
    }
}

fn builtin_round(args: Vec<Value>) -> RuntimeResult<Value> {
    if args.is_empty() {
        return Err(RuntimeError::ArgumentCountError {
            function: "四舍五入".to_string(),
            expected: 1,
            actual: 0,
        });
    }
    
    match &args[0] {
        Value::浮点数(n) => Ok(Value::整数(n.round() as i64)),
        Value::整数(n) => Ok(Value::整数(*n)),
        _ => Err(RuntimeError::TypeError(format!(
            "类型 {} 不支持四舍五入操作",
            args[0].type_name()
        ))),
    }
}

fn builtin_floor(args: Vec<Value>) -> RuntimeResult<Value> {
    if args.is_empty() {
        return Err(RuntimeError::ArgumentCountError {
            function: "向下取整".to_string(),
            expected: 1,
            actual: 0,
        });
    }
    
    match &args[0] {
        Value::浮点数(n) => Ok(Value::整数(n.floor() as i64)),
        Value::整数(n) => Ok(Value::整数(*n)),
        _ => Err(RuntimeError::TypeError(format!(
            "类型 {} 不支持向下取整操作",
            args[0].type_name()
        ))),
    }
}

fn builtin_ceil(args: Vec<Value>) -> RuntimeResult<Value> {
    if args.is_empty() {
        return Err(RuntimeError::ArgumentCountError {
            function: "向上取整".to_string(),
            expected: 1,
            actual: 0,
        });
    }
    
    match &args[0] {
        Value::浮点数(n) => Ok(Value::整数(n.ceil() as i64)),
        Value::整数(n) => Ok(Value::整数(*n)),
        _ => Err(RuntimeError::TypeError(format!(
            "类型 {} 不支持向上取整操作",
            args[0].type_name()
        ))),
    }
}

fn builtin_sqrt(args: Vec<Value>) -> RuntimeResult<Value> {
    if args.is_empty() {
        return Err(RuntimeError::ArgumentCountError {
            function: "平方根".to_string(),
            expected: 1,
            actual: 0,
        });
    }
    
    match &args[0] {
        Value::整数(n) => Ok(Value::浮点数((*n as f64).sqrt())),
        Value::浮点数(n) => Ok(Value::浮点数(n.sqrt())),
        _ => Err(RuntimeError::TypeError(format!(
            "类型 {} 不支持平方根操作",
            args[0].type_name()
        ))),
    }
}

fn builtin_pow(args: Vec<Value>) -> RuntimeResult<Value> {
    if args.len() < 2 {
        return Err(RuntimeError::ArgumentCountError {
            function: "幂".to_string(),
            expected: 2,
            actual: args.len(),
        });
    }
    
    match (&args[0], &args[1]) {
        (Value::整数(a), Value::整数(b)) => Ok(Value::浮点数((*a as f64).powf(*b as f64))),
        (Value::浮点数(a), Value::浮点数(b)) => Ok(Value::浮点数(a.powf(*b))),
        (Value::整数(a), Value::浮点数(b)) => Ok(Value::浮点数((*a as f64).powf(*b))),
        (Value::浮点数(a), Value::整数(b)) => Ok(Value::浮点数(a.powf(*b as f64))),
        _ => Err(RuntimeError::TypeError("幂运算需要数值类型".to_string())),
    }
}

fn builtin_min(args: Vec<Value>) -> RuntimeResult<Value> {
    if args.is_empty() {
        return Err(RuntimeError::ArgumentCountError {
            function: "最小值".to_string(),
            expected: 1,
            actual: 0,
        });
    }
    
    match &args[0] {
        Value::列表(v) if !v.is_empty() => {
            let mut min = v[0].clone();
            for val in v.iter().skip(1) {
                match (&min, val) {
                    (Value::整数(a), Value::整数(b)) if b < a => min = Value::整数(*b),
                    (Value::浮点数(a), Value::浮点数(b)) if b < a => min = Value::浮点数(*b),
                    (Value::整数(a), Value::浮点数(b)) if (*b as i64) < *a => min = Value::浮点数(*b),
                    (Value::浮点数(a), Value::整数(b)) if (*b as f64) < *a => min = Value::整数(*b),
                    _ => {}
                }
            }
            Ok(min)
        }
        Value::列表(_) => Err(RuntimeError::General("列表为空".to_string())),
        _ => Err(RuntimeError::TypeError("最小值需要一个列表参数".to_string())),
    }
}

fn builtin_max(args: Vec<Value>) -> RuntimeResult<Value> {
    if args.is_empty() {
        return Err(RuntimeError::ArgumentCountError {
            function: "最大值".to_string(),
            expected: 1,
            actual: 0,
        });
    }
    
    match &args[0] {
        Value::列表(v) if !v.is_empty() => {
            let mut max = v[0].clone();
            for val in v.iter().skip(1) {
                match (&max, val) {
                    (Value::整数(a), Value::整数(b)) if b > a => max = Value::整数(*b),
                    (Value::浮点数(a), Value::浮点数(b)) if b > a => max = Value::浮点数(*b),
                    (Value::整数(a), Value::浮点数(b)) if (*b as i64) > *a => max = Value::浮点数(*b),
                    (Value::浮点数(a), Value::整数(b)) if (*b as f64) > *a => max = Value::整数(*b),
                    _ => {}
                }
            }
            Ok(max)
        }
        Value::列表(_) => Err(RuntimeError::General("列表为空".to_string())),
        _ => Err(RuntimeError::TypeError("最大值需要一个列表参数".to_string())),
    }
}

fn builtin_sort(args: Vec<Value>) -> RuntimeResult<Value> {
    if args.is_empty() {
        return Err(RuntimeError::ArgumentCountError {
            function: "排序".to_string(),
            expected: 1,
            actual: 0,
        });
    }
    
    match &args[0] {
        Value::列表(v) => {
            let mut sorted = v.clone();
            sorted.sort_by(|a, b| {
                match (a, b) {
                    (Value::整数(a), Value::整数(b)) => a.cmp(b),
                    (Value::浮点数(a), Value::浮点数(b)) => a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal),
                    (Value::字符串(a), Value::字符串(b)) => a.cmp(b),
                    _ => std::cmp::Ordering::Equal,
                }
            });
            Ok(Value::列表(sorted))
        }
        _ => Err(RuntimeError::TypeError("排序需要一个列表参数".to_string())),
    }
}

fn builtin_reverse(args: Vec<Value>) -> RuntimeResult<Value> {
    if args.is_empty() {
        return Err(RuntimeError::ArgumentCountError {
            function: "反转".to_string(),
            expected: 1,
            actual: 0,
        });
    }
    
    match &args[0] {
        Value::列表(v) => {
            Ok(Value::列表(v.iter().rev().cloned().collect()))
        }
        Value::字符串(s) => {
            Ok(Value::字符串(s.chars().rev().collect()))
        }
        _ => Err(RuntimeError::TypeError("反转需要列表或字符串参数".to_string())),
    }
}

fn builtin_join(args: Vec<Value>) -> RuntimeResult<Value> {
    if args.len() < 2 {
        return Err(RuntimeError::ArgumentCountError {
            function: "连接".to_string(),
            expected: 2,
            actual: args.len(),
        });
    }
    
    match (&args[0], &args[1]) {
        (Value::列表(v), Value::字符串(sep)) => {
            let parts: Vec<String> = v.iter().map(|e: &Value| e.to_string_value()).collect();
            Ok(Value::字符串(parts.join(sep)))
        }
        _ => Err(RuntimeError::TypeError("连接需要列表和分隔符参数".to_string())),
    }
}

fn builtin_split(args: Vec<Value>) -> RuntimeResult<Value> {
    if args.len() < 2 {
        return Err(RuntimeError::ArgumentCountError {
            function: "分割".to_string(),
            expected: 2,
            actual: args.len(),
        });
    }
    
    match (&args[0], &args[1]) {
        (Value::字符串(s), Value::字符串(sep)) => {
            let parts: Vec<Value> = s.split(sep).map(|p: &str| Value::字符串(p.to_string())).collect();
            Ok(Value::列表(parts))
        }
        _ => Err(RuntimeError::TypeError("分割需要字符串和分隔符参数".to_string())),
    }
}

fn builtin_replace(args: Vec<Value>) -> RuntimeResult<Value> {
    if args.len() < 3 {
        return Err(RuntimeError::ArgumentCountError {
            function: "替换".to_string(),
            expected: 3,
            actual: args.len(),
        });
    }
    
    match (&args[0], &args[1], &args[2]) {
        (Value::字符串(s), Value::字符串(from), Value::字符串(to)) => {
            Ok(Value::字符串(s.replace(from, to)))
        }
        _ => Err(RuntimeError::TypeError("替换需要三个字符串参数".to_string())),
    }
}

fn builtin_contains(args: Vec<Value>) -> RuntimeResult<Value> {
    if args.len() < 2 {
        return Err(RuntimeError::ArgumentCountError {
            function: "包含".to_string(),
            expected: 2,
            actual: args.len(),
        });
    }
    
    match (&args[0], &args[1]) {
        (Value::字符串(s), Value::字符串(sub)) => Ok(Value::布尔值(s.contains(sub))),
        (Value::列表(v), _) => Ok(Value::布尔值(v.contains(&args[1]))),
        (Value::字典(m), Value::字符串(k)) => Ok(Value::布尔值(m.contains_key(k))),
        _ => Err(RuntimeError::TypeError("包含操作参数类型不匹配".to_string())),
    }
}

fn builtin_find(args: Vec<Value>) -> RuntimeResult<Value> {
    if args.len() < 2 {
        return Err(RuntimeError::ArgumentCountError {
            function: "查找".to_string(),
            expected: 2,
            actual: args.len(),
        });
    }
    
    match (&args[0], &args[1]) {
        (Value::字符串(s), Value::字符串(sub)) => {
            match s.find(sub) {
                Some(idx) => Ok(Value::整数(idx as i64)),
                None => Ok(Value::整数(-1)),
            }
        }
        (Value::列表(v), _) => {
            match v.iter().position(|e| e == &args[1]) {
                Some(idx) => Ok(Value::整数(idx as i64)),
                None => Ok(Value::整数(-1)),
            }
        }
        _ => Err(RuntimeError::TypeError("查找操作参数类型不匹配".to_string())),
    }
}

fn builtin_substring(args: Vec<Value>) -> RuntimeResult<Value> {
    if args.len() < 3 {
        return Err(RuntimeError::ArgumentCountError {
            function: "子字符串".to_string(),
            expected: 3,
            actual: args.len(),
        });
    }
    
    match (&args[0], &args[1], &args[2]) {
        (Value::字符串(s), Value::整数(start), Value::整数(end)) => {
            let s_chars: Vec<char> = s.chars().collect();
            let start_idx = (*start as usize).min(s_chars.len());
            let end_idx = (*end as usize).min(s_chars.len());
            Ok(Value::字符串(s_chars[start_idx..end_idx].iter().collect()))
        }
        _ => Err(RuntimeError::TypeError("子字符串需要字符串和两个整数参数".to_string())),
    }
}

fn builtin_upper(args: Vec<Value>) -> RuntimeResult<Value> {
    if args.is_empty() {
        return Err(RuntimeError::ArgumentCountError {
            function: "大写".to_string(),
            expected: 1,
            actual: 0,
        });
    }
    
    match &args[0] {
        Value::字符串(s) => Ok(Value::字符串(s.to_uppercase())),
        _ => Err(RuntimeError::TypeError("大写需要字符串参数".to_string())),
    }
}

fn builtin_lower(args: Vec<Value>) -> RuntimeResult<Value> {
    if args.is_empty() {
        return Err(RuntimeError::ArgumentCountError {
            function: "小写".to_string(),
            expected: 1,
            actual: 0,
        });
    }
    
    match &args[0] {
        Value::字符串(s) => Ok(Value::字符串(s.to_lowercase())),
        _ => Err(RuntimeError::TypeError("小写需要字符串参数".to_string())),
    }
}

fn builtin_trim(args: Vec<Value>) -> RuntimeResult<Value> {
    if args.is_empty() {
        return Err(RuntimeError::ArgumentCountError {
            function: "去除空白".to_string(),
            expected: 1,
            actual: 0,
        });
    }
    
    match &args[0] {
        Value::字符串(s) => Ok(Value::字符串(s.trim().to_string())),
        _ => Err(RuntimeError::TypeError("去除空白需要字符串参数".to_string())),
    }
}

fn builtin_random(args: Vec<Value>) -> RuntimeResult<Value> {
    use std::time::{SystemTime, UNIX_EPOCH};
    
    let seed = match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(duration) => duration.as_nanos() as u64,
        Err(_) => 0,
    };
    
    let max = if args.is_empty() {
        1_000_000
    } else {
        match &args[0] {
            Value::整数(n) => *n as u64,
            _ => 1_000_000,
        }
    };
    
    let result = (seed.wrapping_mul(1103515245).wrapping_add(12345)) % max;
    Ok(Value::整数(result as i64))
}

fn builtin_input(args: Vec<Value>) -> RuntimeResult<Value> {
    use std::io::{self, Write};
    
    if !args.is_empty() {
        print!("{}", args[0].to_string_value());
        io::stdout().flush().unwrap();
    }
    
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            let trimmed = input.trim();
            if let Ok(n) = trimmed.parse::<i64>() {
                Ok(Value::整数(n))
            } else if let Ok(n) = trimmed.parse::<f64>() {
                Ok(Value::浮点数(n))
            } else if trimmed == "真" || trimmed == "true" {
                Ok(Value::布尔值(true))
            } else if trimmed == "假" || trimmed == "false" {
                Ok(Value::布尔值(false))
            } else {
                Ok(Value::字符串(trimmed.to_string()))
            }
        }
        Err(e) => Err(RuntimeError::General(format!("读取输入失败: {}", e))),
    }
}

fn builtin_read_file(args: Vec<Value>) -> RuntimeResult<Value> {
    use std::fs;
    use std::path::Path;
    
    if args.is_empty() {
        return Err(RuntimeError::ArgumentCountError {
            function: "读取文件".to_string(),
            expected: 1,
            actual: 0,
        });
    }
    
    let filename = match &args[0] {
        Value::字符串(s) => s,
        _ => return Err(RuntimeError::TypeError("文件名必须是字符串".to_string())),
    };
    
    let path = Path::new(filename);
    if !path.exists() {
        return Err(RuntimeError::General(format!("文件不存在: {}", filename)));
    }
    
    match fs::read_to_string(path) {
        Ok(content) => Ok(Value::字符串(content)),
        Err(e) => Err(RuntimeError::General(format!("读取文件失败: {}", e))),
    }
}

fn builtin_write_file(args: Vec<Value>) -> RuntimeResult<Value> {
    use std::fs;
    
    if args.len() < 2 {
        return Err(RuntimeError::ArgumentCountError {
            function: "写入文件".to_string(),
            expected: 2,
            actual: args.len(),
        });
    }
    
    let filename = match &args[0] {
        Value::字符串(s) => s.clone(),
        _ => return Err(RuntimeError::TypeError("文件名必须是字符串".to_string())),
    };
    
    let content = args[1].to_string_value();
    
    match fs::write(&filename, content) {
        Ok(_) => Ok(Value::布尔值(true)),
        Err(e) => Err(RuntimeError::General(format!("写入文件失败: {}", e))),
    }
}

fn builtin_file_exists(args: Vec<Value>) -> RuntimeResult<Value> {
    use std::path::Path;
    
    if args.is_empty() {
        return Err(RuntimeError::ArgumentCountError {
            function: "文件存在".to_string(),
            expected: 1,
            actual: 0,
        });
    }
    
    let filename = match &args[0] {
        Value::字符串(s) => s,
        _ => return Err(RuntimeError::TypeError("文件名必须是字符串".to_string())),
    };
    
    Ok(Value::布尔值(Path::new(filename).exists()))
}
