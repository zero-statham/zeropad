use eframe::{egui, NativeOptions, Renderer}; // eframe egui框架
use std::fs::{self, OpenOptions}; // 文件读写操作
use std::io::{Read, Write};
use std::path::PathBuf;
use std::sync::Arc;
// 文件的读写流

fn main()->Result<(), eframe::Error> {
    // 启动egui应用程序
    let options = eframe::NativeOptions {
        renderer: Renderer::Glow,
        ..Default::default()
    };
    eframe::run_native(
        "Zeropad", // 窗口标题
        options, // 窗口配置选项
        Box::new(|cc| {
            setup_custom_fonts(&cc.egui_ctx);
            Ok(Box::new(NotepadApp::default()))
        }), // 应用实例
    )
}

// 加载中文字体
fn setup_custom_fonts(ctx: &egui::Context) {
    use egui::FontFamily;
    let mut fonts = egui::FontDefinitions::default();
    // 获取系统字体路径
    if let Some(font_path) = get_system_font_path("Arial Unicode") {
        // 读取字体文件数据
        let font_data = fs::read(font_path).expect("Failed to read font file");
        fonts.font_data.insert("custom_font".to_owned(), Arc::from(egui::FontData::from_owned(font_data)));
        // 将字体添加到Proportional 和 Monospace 字体中
        fonts.families.get_mut(&FontFamily::Proportional).unwrap().insert(0, "custom_font".to_owned());
        fonts.families.get_mut(&FontFamily::Monospace).unwrap().insert(0, "custom_font".to_owned());
        ctx.set_fonts(fonts);
    } else {
        eprintln!("SystemFont Arial Unicode not found");
    }
}

fn get_system_font_path(font_name: &str) -> Option<PathBuf> {
    // 检查常见系统字体目录
    let font_dirs:Vec<PathBuf> = if cfg!(target_os = "windows") {
        vec![
            r"C:\Windows\Fonts".into(),
        ]
    } else if cfg!(target_os = "macos") {
        vec!["/System/Library/Fonts/Supplemental".into(), "/System/Library/Fonts".into()]
    } else if cfg!(target_os = "linux") {
        vec!["usr/share/fonts".into(), "~/.fonts".into()]
    } else {
        vec![]
    };

    for dir in font_dirs {
        if let Ok(entries) = fs::read_dir(dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if let Some(file_name) = path.file_name().and_then(|n| n.to_str()) {
                    if file_name.contains(font_name) {
                        return Some(path);
                    }
                }
            }
        }
    }
    None
}

// 定义记事本应用程序的结构题
#[derive(Default)]
struct NotepadApp {
    text: String, // 文本编辑器的内容
    file_path: String, // 当前打开或保存的文件路径
}

impl eframe::App for NotepadApp {
    // 更新主页面
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // 创建一个中央面板，承载主界面内容
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Zeropad"); // 显示标题
            // 文本多行编辑器
            ui.add(
                egui::TextEdit::multiline(&mut self.text) // 将文本内容绑定到编辑器
                    .hint_text("Start typing here...") // 占位提示文本
                    .desired_rows(20) // 默认显示20行
                    .lock_focus(true), // 启动时自动聚焦
            );
            // 添加一个水平布局的按钮烂
            ui.horizontal(|ui| {
                // 打开文件按钮
                if ui.button("Open").clicked() {
                    // 弹出文件选择对话框
                    if let Some(path) = rfd::FileDialog::new().pick_file() {
                        self.file_path = path.to_string_lossy().to_string(); // 获取文件路径
                        // 读取文件内容并显示文本狂
                        match read_file(&self.file_path) {
                            Ok(content) => self.text = content, // 成功读取则填充内容
                            Err(err) => {
                                // 读取失败，在文本框中显示错误信息
                                eprintln!("Failed to open file: {}", err);
                                self.text = format!("Failed to open file: {}", err);
                            },
                        }
                    }
                }
                // 保存文件按钮
                if ui.button("Save").clicked() {
                    // 如果当前没有路径，弹出保存文件对话框
                    if let Some(path) = rfd::FileDialog::new()
                        .add_filter("Text File", &["txt"])
                        .save_file() {
                        self.file_path = path.to_string_lossy().to_string(); //获取保存路径
                    }
                    // 如果有路径，则尝试保存
                    if !self.file_path.is_empty() {
                        if let Err(err) = save_file(&self.file_path, &self.text) {
                            eprintln!("Failed to save file: {}", err);
                        };
                    }
                }
                // 新建文件按钮
                if ui.button("New").clicked() {
                    self.file_path.clear(); // 清空文件路径
                    self.text.clear(); // 清空内容
                }
            });
        });
    }
}

fn read_file(path: &str) -> Result<String, std::io::Error> {
    let mut file = OpenOptions::new().read(true).open(path)?; // 打开文件只读模式
    let mut content = String::new(); // 用户存储读取的内容
    file.read_to_string(&mut content)?; // 将文件内容读入字符串
    Ok(content) // 返回文件内容
}

fn save_file(path: &str, content: &str) -> Result<(), std::io::Error> {
    let mut file = OpenOptions::new()
        .write(true) // 启用写入模式
        .create(true) // 如果文件不存在则创建
        .truncate(true) // 如果文件已存在则清空
        .open(path)?; // 打开文件
    file.write_all(content.as_bytes())?; // 将内容写入文件
    Ok(()) // 返回成功结果
}
