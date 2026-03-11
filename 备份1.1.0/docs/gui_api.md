# GUI库API文档

## 1. 架构概述

GUI库采用组件化设计，基于以下核心概念：

- **Control trait**：所有GUI控件的基础接口，定义了控件的基本行为
- **EventHandler trait**：事件处理接口，处理用户交互事件
- **事件系统**：基于事件分发机制，支持多种事件类型
- **组件层次**：窗口可以包含子控件，形成控件树结构

### 核心组件

| 组件 | 描述 | 主要功能 |
|------|------|----------|
| Window | 窗口控件 | 多窗口管理、大小调整、最小化/最大化/关闭 |
| Button | 按钮控件 | 点击事件、状态切换、样式自定义 |
| Label | 标签控件 | 文本显示、字体样式设置、颜色调整 |
| Input | 输入框 | 文本输入、长度限制、验证功能 |

## 2. 核心API接口

### 2.1 Control trait

所有GUI控件都实现了`Control` trait，提供以下方法：

| 方法 | 参数 | 返回值 | 描述 |
|------|------|--------|------|
| `get_id()` | 无 | `&str` | 获取控件唯一标识符 |
| `draw()` | 无 | 无 | 绘制控件 |
| `set_position()` | x: f32, y: f32 | 无 | 设置控件位置 |
| `set_size()` | width: f32, height: f32 | 无 | 设置控件大小 |
| `set_visible()` | visible: bool | 无 | 设置控件可见性 |
| `is_visible()` | 无 | bool | 检查控件是否可见 |

### 2.2 事件系统

#### EventType 枚举

| 事件类型 | 描述 |
|----------|------|
| 点击 | 鼠标点击事件 |
| 双击 | 鼠标双击事件 |
| 右键点击 | 鼠标右键点击事件 |
| 鼠标移动 | 鼠标移动事件 |
| 鼠标按下 | 鼠标按键按下事件 |
| 鼠标松开 | 鼠标按键松开事件 |
| 键盘按下 | 键盘按键按下事件 |
| 键盘松开 | 键盘按键松开事件 |
| 焦点获得 | 控件获得焦点事件 |
| 焦点失去 | 控件失去焦点事件 |
| 窗口关闭 | 窗口关闭事件 |
| 窗口大小改变 | 窗口大小改变事件 |
| 窗口移动 | 窗口移动事件 |
| 文本改变 | 文本内容改变事件 |
| 值改变 | 控件值改变事件 |

#### Event 结构体

| 属性 | 类型 | 描述 |
|------|------|------|
| 事件类型 | EventType | 事件的类型 |
| 鼠标位置 | Option<(f32, f32)> | 事件发生时的鼠标位置 |
| 键码 | Option<u32> | 键盘事件的键码 |
| 文本内容 | Option<String> | 文本相关事件的文本内容 |
| 时间戳 | std::time::Instant | 事件发生的时间戳 |

#### EventHandler trait

| 方法 | 参数 | 返回值 | 描述 |
|------|------|--------|------|
| `handle_event()` | event: &Event | 无 | 处理事件 |

### 2.3 Window 类

#### 构造方法

| 方法 | 参数 | 返回值 | 描述 |
|------|------|--------|------|
| `new()` | 标题: &str | Window | 创建新窗口 |
| `with_size()` | width: f32, height: f32 | Window | 设置窗口大小 |
| `with_position()` | x: f32, y: f32 | Window | 设置窗口位置 |
| `with_style()` | style: WindowStyle | Window | 设置窗口样式 |

#### 窗口状态

| 状态 | 描述 |
|------|------|
| 正常 | 正常窗口状态 |
| 最小化 | 窗口最小化状态 |
| 最大化 | 窗口最大化状态 |
| 全屏 | 窗口全屏状态 |

#### 窗口样式

| 样式 | 描述 |
|------|------|
| 固定大小 | 窗口大小不可调整 |
| 可调整大小 | 窗口大小可调整 |
| 无边框 | 窗口无边框 |
| 工具窗口 | 工具风格窗口 |

#### 方法

| 方法 | 参数 | 返回值 | 描述 |
|------|------|--------|------|
| `get_id()` | 无 | &str | 获取窗口ID |
| `get_title()` | 无 | &str | 获取窗口标题 |
| `set_title()` | 标题: &str | 无 | 设置窗口标题 |
| `get_size()` | 无 | (f32, f32) | 获取窗口大小 |
| `set_size()` | width: f32, height: f32 | 无 | 设置窗口大小 |
| `get_position()` | 无 | (f32, f32) | 获取窗口位置 |
| `set_position()` | x: f32, y: f32 | 无 | 设置窗口位置 |
| `get_state()` | 无 | WindowState | 获取窗口状态 |
| `minimize()` | 无 | 无 | 最小化窗口 |
| `maximize()` | 无 | 无 | 最大化窗口 |
| `restore()` | 无 | 无 | 恢复窗口到正常状态 |
| `close()` | 无 | 无 | 关闭窗口 |
| `show()` | 无 | 无 | 显示窗口 |
| `hide()` | 无 | 无 | 隐藏窗口 |
| `is_visible()` | 无 | bool | 检查窗口是否可见 |
| `add_control()` | control: Arc<Mutex<dyn Control + Send>> | 无 | 添加子控件 |
| `remove_control()` | control_id: &str | 无 | 移除子控件 |
| `register_event_handler()` | handler: Arc<Mutex<dyn EventHandler + Send>> | 无 | 注册事件处理器 |
| `draw()` | 无 | 无 | 绘制窗口及其子控件 |
| `dispatch_event()` | event: &Event | 无 | 分发事件到窗口及其子控件 |

### 2.4 Button 类

#### 构造方法

| 方法 | 参数 | 返回值 | 描述 |
|------|------|--------|------|
| `new()` | 文本: &str | Button | 创建新按钮 |
| `with_position()` | x: f32, y: f32 | Button | 设置按钮位置 |
| `with_size()` | width: f32, height: f32 | Button | 设置按钮大小 |
| `with_style()` | style: ButtonStyle | Button | 设置按钮样式 |
| `with_font_size()` | size: f32 | Button | 设置字体大小 |
| `with_colors()` | text: &str, background: &str, border: &str | Button | 设置按钮颜色 |
| `with_corner_radius()` | radius: f32 | Button | 设置圆角半径 |

#### 按钮状态

| 状态 | 描述 |
|------|------|
| 正常 | 按钮正常状态 |
| 悬停 | 鼠标悬停在按钮上 |
| 按下 | 按钮被按下 |
| 禁用 | 按钮被禁用 |

#### 按钮样式

| 样式 | 描述 |
|------|------|
| 默认 | 默认按钮样式 |
| 主要 | 主要按钮样式（蓝色） |
| 次要 | 次要按钮样式（灰色） |
| 成功 | 成功按钮样式（绿色） |
| 警告 | 警告按钮样式（黄色） |
| 危险 | 危险按钮样式（红色） |
| 链接 | 链接样式按钮 |

#### 方法

| 方法 | 参数 | 返回值 | 描述 |
|------|------|--------|------|
| `set_text()` | 文本: &str | 无 | 设置按钮文本 |
| `get_text()` | 无 | &str | 获取按钮文本 |
| `set_state()` | state: ButtonState | 无 | 设置按钮状态 |
| `get_state()` | 无 | ButtonState | 获取按钮状态 |
| `set_style()` | style: ButtonStyle | 无 | 设置按钮样式 |
| `set_enabled()` | enabled: bool | 无 | 设置按钮是否启用 |
| `is_enabled()` | 无 | bool | 检查按钮是否启用 |
| `set_click_callback()` | callback: Box<dyn Fn() + Send + Sync> | 无 | 设置点击回调函数 |
| `set_hover_callback()` | callback: Box<dyn Fn() + Send + Sync> | 无 | 设置悬停回调函数 |
| `click()` | 无 | 无 | 触发按钮点击事件 |
| `hover()` | 无 | 无 | 触发按钮悬停事件 |
| `leave()` | 无 | 无 | 触发鼠标离开按钮事件 |

### 2.5 Label 类

#### 构造方法

| 方法 | 参数 | 返回值 | 描述 |
|------|------|--------|------|
| `new()` | 文本: &str | Label | 创建新标签 |
| `with_position()` | x: f32, y: f32 | Label | 设置标签位置 |
| `with_size()` | width: f32, height: f32 | Label | 设置标签大小 |
| `with_alignment()` | alignment: LabelAlignment | Label | 设置文本对齐方式 |
| `with_wrap()` | wrap: TextWrap | Label | 设置文本换行方式 |
| `with_font()` | name: &str, size: f32 | Label | 设置字体名称和大小 |
| `with_font_weight()` | weight: u32 | Label | 设置字体粗细 |
| `with_text_color()` | color: &str | Label | 设置文本颜色 |
| `with_background()` | color: &str | Label | 设置背景颜色 |
| `with_border()` | width: f32, color: &str | Label | 设置边框宽度和颜色 |
| `with_line_spacing()` | spacing: f32 | Label | 设置行间距 |
| `with_letter_spacing()` | spacing: f32 | Label | 设置字间距 |
| `with_underline()` | underline: bool | Label | 设置是否下划线 |
| `with_strikethrough()` | strikethrough: bool | Label | 设置是否删除线 |
| `with_italic()` | italic: bool | Label | 设置是否斜体 |

#### 文本对齐方式

| 对齐方式 | 描述 |
|----------|------|
| 左对齐 | 文本左对齐 |
| 居中对齐 | 文本居中对齐 |
| 右对齐 | 文本右对齐 |

#### 文本换行方式

| 换行方式 | 描述 |
|----------|------|
| 不换行 | 文本不换行 |
| 自动换行 | 文本自动换行 |
| 字符换行 | 按字符换行 |

#### 方法

| 方法 | 参数 | 返回值 | 描述 |
|------|------|--------|------|
| `set_text()` | 文本: &str | 无 | 设置标签文本 |
| `get_text()` | 无 | &str | 获取标签文本 |
| `set_alignment()` | alignment: LabelAlignment | 无 | 设置文本对齐方式 |
| `get_alignment()` | 无 | LabelAlignment | 获取文本对齐方式 |
| `set_font_size()` | size: f32 | 无 | 设置字体大小 |
| `get_font_size()` | 无 | f32 | 获取字体大小 |
| `set_text_color()` | color: &str | 无 | 设置文本颜色 |
| `get_text_color()` | 无 | &str | 获取文本颜色 |
| `set_background_color()` | color: Option<&str> | 无 | 设置背景颜色 |
| `get_background_color()` | 无 | Option<&String> | 获取背景颜色 |
| `set_underline()` | underline: bool | 无 | 设置是否下划线 |
| `is_underline()` | 无 | bool | 检查是否下划线 |
| `set_strikethrough()` | strikethrough: bool | 无 | 设置是否删除线 |
| `is_strikethrough()` | 无 | bool | 检查是否删除线 |
| `set_italic()` | italic: bool | 无 | 设置是否斜体 |
| `is_italic()` | 无 | bool | 检查是否斜体 |
| `calculate_text_width()` | 无 | f32 | 计算文本宽度 |
| `calculate_text_height()` | 无 | f32 | 计算文本高度 |

### 2.6 Input 类

#### 构造方法

| 方法 | 参数 | 返回值 | 描述 |
|------|------|--------|------|
| `new()` | 占位符: &str | Input | 创建新输入框 |
| `with_position()` | x: f32, y: f32 | Input | 设置输入框位置 |
| `with_size()` | width: f32, height: f32 | Input | 设置输入框大小 |
| `with_type()` | input_type: InputType | Input | 设置输入类型 |
| `with_validation()` | mode: ValidationMode | Input | 设置验证模式 |
| `with_length_limits()` | min: Option<usize>, max: Option<usize> | Input | 设置长度限制 |
| `with_regex()` | pattern: &str | Input | 设置正则表达式验证 |
| `with_custom_validator()` | validator: Box<dyn Fn(&str) -> bool + Send + Sync> | Input | 设置自定义验证器 |
| `with_font_size()` | size: f32 | Input | 设置字体大小 |
| `with_colors()` | text: &str, background: &str, border: &str | Input | 设置颜色 |

#### 输入类型

| 类型 | 描述 |
|------|------|
| 文本 | 普通文本输入 |
| 密码 | 密码输入（显示为*） |
| 数字 | 数字输入 |
| 邮箱 | 邮箱输入 |
| 电话 | 电话输入 |
| 网址 | 网址输入 |

#### 验证模式

| 模式 | 描述 |
|------|------|
| 无验证 | 不进行验证 |
| 非空验证 | 验证输入不为空 |
| 长度验证 | 验证输入长度 |
| 正则验证 | 使用正则表达式验证 |
| 自定义验证 | 使用自定义函数验证 |

#### 方法

| 方法 | 参数 | 返回值 | 描述 |
|------|------|--------|------|
| `set_text()` | 文本: &str | 无 | 设置输入框文本 |
| `get_text()` | 无 | &str | 获取输入框文本 |
| `append_text()` | text: &str | 无 | 追加文本 |
| `insert_text()` | pos: usize, text: &str | 无 | 在指定位置插入文本 |
| `delete_text()` | start: usize, end: usize | 无 | 删除指定范围的文本 |
| `clear()` | 无 | 无 | 清空输入框 |
| `set_cursor_position()` | pos: usize | 无 | 设置光标位置 |
| `get_cursor_position()` | 无 | usize | 获取光标位置 |
| `set_selection()` | start: Option<usize>, end: Option<usize> | 无 | 设置文本选择范围 |
| `get_selection()` | 无 | Option<(usize, usize)> | 获取文本选择范围 |
| `get_selected_text()` | 无 | Option<String> | 获取选中的文本 |
| `set_enabled()` | enabled: bool | 无 | 设置输入框是否启用 |
| `is_enabled()` | 无 | bool | 检查输入框是否启用 |
| `set_readonly()` | readonly: bool | 无 | 设置输入框是否只读 |
| `is_readonly()` | 无 | bool | 检查输入框是否只读 |
| `set_text_change_callback()` | callback: Box<dyn Fn(&str) + Send + Sync> | 无 | 设置文本改变回调 |
| `set_value_change_callback()` | callback: Box<dyn Fn(&str) + Send + Sync> | 无 | 设置值改变回调 |
| `set_focus_callbacks()` | gain: Option<Box<dyn Fn() + Send + Sync>>, lose: Option<Box<dyn Fn() + Send + Sync>> | 无 | 设置焦点回调 |
| `validate()` | 无 | Result<(), String> | 验证输入内容 |
| `is_valid()` | 无 | bool | 检查输入是否有效 |
| `get_validation_error()` | 无 | Option<String> | 获取验证错误信息 |
| `handle_key_press()` | keycode: u32 | 无 | 处理按键事件 |

## 3. 错误处理机制

### 3.1 输入验证错误

Input 控件提供了以下验证错误处理：

| 验证模式 | 错误信息 |
|----------|----------|
| 非空验证 | "输入不能为空" |
| 长度验证 | "输入长度不能少于X个字符" 或 "输入长度不能超过X个字符" |
| 正则验证 | "输入格式不正确" 或 "正则表达式格式错误" |
| 自定义验证 | "输入验证失败" |

### 3.2 事件处理错误

事件处理过程中可能出现的错误：

| 错误类型 | 描述 | 处理方式 |
|----------|------|----------|
| 锁获取失败 | 无法获取控件的锁 | 忽略该事件，继续处理其他事件 |
| 回调执行失败 | 回调函数执行出错 | 捕获并记录错误，不影响其他事件处理 |

## 4. 使用场景示例

### 4.1 基本窗口创建

```rust
use std::sync::{Arc, Mutex};
use 汉语编程::gui::{Window, Button, Label, Input};

// 创建窗口
let mut window = Window::new("示例窗口")
    .with_size(800.0, 600.0)
    .with_position(100.0, 100.0);

// 显示窗口
window.show();
```

### 4.2 按钮点击事件

```rust
use std::sync::{Arc, Mutex};
use 汉语编程::gui::{Window, Button};

let mut window = Window::new("按钮示例");

// 创建按钮
let mut button = Button::new("点击我")
    .with_position(100.0, 100.0)
    .with_size(120.0, 40.0);

// 设置点击回调
button.set_click_callback(Box::new(|| {
    println!("按钮被点击了！");
}));

// 添加按钮到窗口
window.add_control(Arc::new(Mutex::new(button)));
```

### 4.3 输入框验证

```rust
use std::sync::{Arc, Mutex};
use 汉语编程::gui::{Window, Input, InputType, ValidationMode};

let mut window = Window::new("输入验证示例");

// 创建邮箱输入框
let mut email_input = Input::new("请输入邮箱")
    .with_position(100.0, 100.0)
    .with_size(300.0, 40.0)
    .with_type(InputType::邮箱)
    .with_regex(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\\.[a-zA-Z]{2,}$");

// 设置值改变回调
email_input.set_value_change_callback(Box::new(|text| {
    println!("邮箱输入: {}", text);
}));

// 添加输入框到窗口
window.add_control(Arc::new(Mutex::new(email_input)));
```

### 4.4 复杂布局

```rust
use std::sync::{Arc, Mutex};
use 汉语编程::gui::{Window, Button, Label, Input};

let mut window = Window::new("复杂布局示例")
    .with_size(800.0, 400.0);

// 添加标签
let label = Label::new("用户登录")
    .with_position(100.0, 50.0)
    .with_font("微软雅黑", 24.0);
window.add_control(Arc::new(Mutex::new(label)));

// 添加用户名输入
let username_input = Input::new("请输入用户名")
    .with_position(100.0, 100.0)
    .with_size(300.0, 40.0);
window.add_control(Arc::new(Mutex::new(username_input)));

// 添加密码输入
let password_input = Input::new("请输入密码")
    .with_position(100.0, 150.0)
    .with_size(300.0, 40.0)
    .with_type(InputType::密码);
window.add_control(Arc::new(Mutex::new(password_input)));

// 添加登录按钮
let mut login_button = Button::new("登录")
    .with_position(100.0, 200.0)
    .with_size(100.0, 40.0)
    .with_style(ButtonStyle::主要);

login_button.set_click_callback(Box::new(|| {
    println!("登录按钮点击");
}));

window.add_control(Arc::new(Mutex::new(login_button)));

// 添加取消按钮
let mut cancel_button = Button::new("取消")
    .with_position(220.0, 200.0)
    .with_size(100.0, 40.0)
    .with_style(ButtonStyle::次要);

cancel_button.set_click_callback(Box::new(|| {
    println!("取消按钮点击");
}));

window.add_control(Arc::new(Mutex::new(cancel_button)));
```

## 5. 最佳实践指南

### 5.1 性能优化

1. **控件层次管理**：避免过深的控件层次结构，减少绘制和事件分发开销
2. **事件处理**：只注册必要的事件处理器，避免不必要的事件处理
3. **资源管理**：合理使用Arc和Mutex，避免过度使用导致性能下降
4. **绘制优化**：只在需要时重绘控件，避免频繁重绘

### 5.2 代码组织

1. **模块化**：将不同功能的控件和逻辑分离到不同的模块
2. **事件处理**：将复杂的事件处理逻辑封装到单独的函数或结构体中
3. **错误处理**：合理处理验证错误和事件处理错误，提供友好的用户反馈
4. **代码风格**：保持一致的代码风格，使用中文命名提高可读性

### 5.3 用户体验

1. **响应式设计**：确保控件在不同窗口大小下都能正常显示
2. **视觉反馈**：为按钮、输入框等控件提供适当的视觉反馈
3. **验证提示**：及时显示输入验证错误信息，帮助用户正确输入
4. **键盘支持**：为常用操作提供键盘快捷键支持

### 5.4 安全性

1. **输入验证**：对所有用户输入进行严格验证，防止恶意输入
2. **错误处理**：避免在错误信息中泄露敏感信息
3. **资源释放**：确保所有资源都能正确释放，避免资源泄漏

## 6. 完整代码示例

### 6.1 简单计算器

```rust
use std::sync::{Arc, Mutex};
use 汉语编程::gui::{Window, Button, Label, Input, Event, EventType};

fn main() {
    // 创建主窗口
    let mut window = Window::new("简单计算器")
        .with_size(400.0, 500.0)
        .with_position(300.0, 200.0);
    
    // 创建显示区域
    let display = Input::new("0")
        .with_position(20.0, 20.0)
        .with_size(360.0, 60.0)
        .with_font_size(24.0)
        .with_colors("#000000", "#f0f0f0", "#cccccc");
    let display = Arc::new(Mutex::new(display));
    
    // 创建按钮
    let button_labels = [
        ["C", "±", "%", "/"],
        ["7", "8", "9", "×"],
        ["4", "5", "6", "-"],
        ["1", "2", "3", "+"],
        ["0", ".", "="],
    ];
    
    let button_positions = [
        [(20.0, 100.0), (110.0, 100.0), (200.0, 100.0), (290.0, 100.0)],
        [(20.0, 170.0), (110.0, 170.0), (200.0, 170.0), (290.0, 170.0)],
        [(20.0, 240.0), (110.0, 240.0), (200.0, 240.0), (290.0, 240.0)],
        [(20.0, 310.0), (110.0, 310.0), (200.0, 310.0), (290.0, 310.0)],
        [(20.0, 380.0), (110.0, 380.0), (200.0, 380.0)],
    ];
    
    let button_sizes = [
        [(80.0, 60.0); 4],
        [(80.0, 60.0); 4],
        [(80.0, 60.0); 4],
        [(80.0, 60.0); 4],
        [(80.0, 60.0), (80.0, 60.0), (170.0, 60.0)],
    ];
    
    // 添加显示区域到窗口
    window.add_control(Arc::clone(&display));
    
    // 添加按钮到窗口
    for (i, row) in button_labels.iter().enumerate() {
        for (j, label) in row.iter().enumerate() {
            let mut button = Button::new(label)
                .with_position(button_positions[i][j].0, button_positions[i][j].1)
                .with_size(button_sizes[i][j].0, button_sizes[i][j].1)
                .with_font_size(18.0);
            
            // 设置按钮样式
            if label == "C" {
                button = button.with_style(ButtonStyle::危险);
            } else if label == "=" {
                button = button.with_style(ButtonStyle::主要);
            } else if ["+", "-", "×", "/"].contains(label) {
                button = button.with_style(ButtonStyle::次要);
            }
            
            // 设置点击回调
            let display_clone = Arc::clone(&display);
            button.set_click_callback(Box::new(move || {
                if let Ok(mut input) = display_clone.lock() {
                    let current_text = input.get_text();
                    match label {
                        "C" => input.set_text("0"),
                        "=" => {
                            // 这里可以添加计算逻辑
                            println!("计算: {}", current_text);
                        }
                        _ => {
                            if current_text == "0" {
                                input.set_text(label);
                            } else {
                                input.append_text(label);
                            }
                        }
                    }
                }
            }));
            
            window.add_control(Arc::new(Mutex::new(button)));
        }
    }
    
    // 显示窗口
    window.show();
    
    // 模拟事件循环
    println!("计算器窗口已创建，按Ctrl+C退出");
    std::thread::park();
}
```

### 6.2 表单验证

```rust
use std::sync::{Arc, Mutex};
use 汉语编程::gui::{Window, Button, Label, Input, InputType, ValidationMode, Event, EventType};

fn main() {
    // 创建主窗口
    let mut window = Window::new("用户注册")
        .with_size(500.0, 400.0)
        .with_position(300.0, 200.0);
    
    // 创建标题
    let title = Label::new("用户注册表单")
        .with_position(150.0, 20.0)
        .with_font("微软雅黑", 24.0)
        .with_alignment(LabelAlignment::居中对齐);
    window.add_control(Arc::new(Mutex::new(title)));
    
    // 创建姓名输入
    let name_label = Label::new("姓名:")
        .with_position(100.0, 80.0);
    window.add_control(Arc::new(Mutex::new(name_label)));
    
    let name_input = Input::new("请输入姓名")
        .with_position(180.0, 75.0)
        .with_size(200.0, 30.0)
        .with_validation(ValidationMode::非空验证);
    let name_input = Arc::new(Mutex::new(name_input));
    window.add_control(Arc::clone(&name_input));
    
    // 创建邮箱输入
    let email_label = Label::new("邮箱:")
        .with_position(100.0, 130.0);
    window.add_control(Arc::new(Mutex::new(email_label)));
    
    let email_input = Input::new("请输入邮箱")
        .with_position(180.0, 125.0)
        .with_size(200.0, 30.0)
        .with_type(InputType::邮箱)
        .with_regex(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\\.[a-zA-Z]{2,}$");
    let email_input = Arc::new(Mutex::new(email_input));
    window.add_control(Arc::clone(&email_input));
    
    // 创建密码输入
    let password_label = Label::new("密码:")
        .with_position(100.0, 180.0);
    window.add_control(Arc::new(Mutex::new(password_label)));
    
    let password_input = Input::new("请输入密码")
        .with_position(180.0, 175.0)
        .with_size(200.0, 30.0)
        .with_type(InputType::密码)
        .with_length_limits(Some(6), Some(20));
    let password_input = Arc::new(Mutex::new(password_input));
    window.add_control(Arc::clone(&password_input));
    
    // 创建确认密码输入
    let confirm_label = Label::new("确认密码:")
        .with_position(100.0, 230.0);
    window.add_control(Arc::new(Mutex::new(confirm_label)));
    
    let confirm_input = Input::new("请再次输入密码")
        .with_position(180.0, 225.0)
        .with_size(200.0, 30.0)
        .with_type(InputType::密码);
    let confirm_input = Arc::new(Mutex::new(confirm_input));
    window.add_control(Arc::clone(&confirm_input));
    
    // 创建错误信息标签
    let error_label = Label::new("")
        .with_position(100.0, 270.0)
        .with_size(300.0, 30.0)
        .with_text_color("#dc3545");
    let error_label = Arc::new(Mutex::new(error_label));
    window.add_control(Arc::clone(&error_label));
    
    // 创建注册按钮
    let mut register_button = Button::new("注册")
        .with_position(150.0, 310.0)
        .with_size(100.0, 40.0)
        .with_style(ButtonStyle::成功);
    
    // 设置注册按钮点击回调
    register_button.set_click_callback(Box::new(move || {
        // 验证输入
        let mut errors = Vec::new();
        
        // 验证姓名
        if let Ok(name) = name_input.lock() {
            if let Err(e) = name.validate() {
                errors.push(e);
            }
        }
        
        // 验证邮箱
        if let Ok(email) = email_input.lock() {
            if let Err(e) = email.validate() {
                errors.push(e);
            }
        }
        
        // 验证密码
        if let Ok(password) = password_input.lock() {
            if let Err(e) = password.validate() {
                errors.push(e);
            }
        }
        
        // 验证确认密码
        if let (Ok(password), Ok(confirm)) = (password_input.lock(), confirm_input.lock()) {
            if password.get_text() != confirm.get_text() {
                errors.push("两次输入的密码不一致".to_string());
            }
        }
        
        // 显示错误信息或注册成功
        if let Ok(mut label) = error_label.lock() {
            if !errors.is_empty() {
                label.set_text(&errors.join("\n"));
            } else {
                label.set_text("注册成功！");
                label.set_text_color("#28a745");
            }
        }
    }));
    
    window.add_control(Arc::new(Mutex::new(register_button)));
    
    // 创建取消按钮
    let mut cancel_button = Button::new("取消")
        .with_position(270.0, 310.0)
        .with_size(100.0, 40.0)
        .with_style(ButtonStyle::次要);
    
    cancel_button.set_click_callback(Box::new(|| {
        println!("取消注册");
    }));
    
    window.add_control(Arc::new(Mutex::new(cancel_button)));
    
    // 显示窗口
    window.show();
    
    // 模拟事件循环
    println!("注册表单窗口已创建，按Ctrl+C退出");
    std::thread::park();
}
```

## 7. 总结

本GUI库提供了一套完整的控件系统，支持窗口管理、按钮交互、文本显示和输入验证等功能。通过组件化设计和事件驱动架构，使开发者能够快速构建各种GUI应用。

### 核心优势

1. **中文命名**：所有API和概念都使用中文命名，提高代码可读性和易用性
2. **组件化设计**：基于trait的组件系统，易于扩展和自定义
3. **事件驱动**：完整的事件系统，支持各种用户交互
4. **验证系统**：内置多种输入验证方式，确保数据有效性
5. **样式定制**：支持丰富的样式自定义选项，满足不同UI需求

### 未来扩展

1. **更多控件**：计划添加列表框、组合框、滑块等更多控件类型
2. **布局系统**：实现自动布局和响应式布局功能
3. **主题支持**：添加主题系统，支持明暗主题切换
4. **动画效果**：添加控件动画和过渡效果
5. **国际化支持**：支持多语言界面

通过本API文档，开发者可以快速了解和使用GUI库，构建功能丰富、用户友好的图形界面应用。