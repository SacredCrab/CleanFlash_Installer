use clean_flash_common::{redirection, uninstaller, update_checker, ProgressCallback};
use clean_flash_ui::font::FontManager;
use clean_flash_ui::renderer::{Renderer, RgbaImage};
use clean_flash_ui::widgets::button::GradientButton;
use clean_flash_ui::widgets::label::Label;
use clean_flash_ui::widgets::progress_bar::ProgressBar;
use std::sync::{Arc, Mutex};

pub const WIDTH: usize = 712;
pub const HEIGHT: usize = 329;
const BG_COLOR: u32 = 0x00323333; // RGB(50, 51, 51)
const FG_COLOR: u32 = 0x00F5F5F5;
const PANEL_X: i32 = 90;
const PANEL_Y: i32 = 162;

const BEFORE_TEXT: &str = "You are about to uninstall Clean Flash Player.\n\
Please close all browsers, including Google Chrome, Mozilla Firefox and Internet Explorer.\n\n\
The installer will completely remove all versions of Flash Player from this computer,\n\
including Clean Flash Player and older versions of Adobe Flash Player.";

const COMPLETE_TEXT: &str = "\nAll versions of Flash Player have been successfully uninstalled.\n\n\
If you ever change your mind, check out Clean Flash Player's website!";

const UNINSTALL_TICKS: i32 = 9;

#[derive(Clone, Copy, PartialEq, Eq)]
enum Panel {
    BeforeInstall,
    Install,
    Complete,
    Failure,
}

struct ProgressState {
    label: String,
    value: i32,
    done: bool,
    error: Option<String>,
}

pub struct UninstallForm {
    panel: Panel,
    title_text: String,
    subtitle_text: String,
    flash_logo: RgbaImage,
    prev_button: GradientButton,
    next_button: GradientButton,
    // Before uninstall
    before_label: Label,
    // Install (progress)
    progress_header: Label,
    progress_label: Label,
    progress_bar: ProgressBar,
    // Complete
    complete_label: Label,
    // Failure
    failure_text_label: Label,
    failure_detail: String,
    copy_error_button: GradientButton,
    // State
    progress_state: Arc<Mutex<ProgressState>>,
    fonts: FontManager,
    prev_mouse_down: bool,
}

impl UninstallForm {
    pub fn new() -> Self {
        let version = update_checker::FLASH_VERSION;
        let flash_logo = load_resource_image("flashLogo.png");
        let fonts = FontManager::new();

        Self {
            panel: Panel::BeforeInstall,
            title_text: "Clean Flash Player".into(),
            subtitle_text: format!("built from version {} (China)", version),
            flash_logo,
            prev_button: GradientButton::new(90, 286, 138, 31, "QUIT"),
            next_button: GradientButton::new(497, 286, 138, 31, "UNINSTALL"),
            before_label: Label::new(PANEL_X + 3, PANEL_Y + 2, BEFORE_TEXT, 13.0),
            progress_header: Label::new(
                PANEL_X + 3,
                PANEL_Y,
                "Uninstallation in progress...",
                13.0,
            ),
            progress_label: Label::new(PANEL_X + 46, PANEL_Y + 30, "Preparing...", 13.0),
            progress_bar: ProgressBar::new(PANEL_X + 49, PANEL_Y + 58, 451, 23),
            complete_label: Label::new(PANEL_X, PANEL_Y, COMPLETE_TEXT, 13.0),
            failure_text_label: Label::new(
                PANEL_X + 3,
                PANEL_Y + 2,
                "Oops! The installation process has encountered an unexpected problem.\n\
The following details could be useful. Press the Retry button to try again.",
                13.0,
            ),
            failure_detail: String::new(),
            copy_error_button: GradientButton::new(PANEL_X + 441, PANEL_Y + 58, 104, 31, "COPY"),
            progress_state: Arc::new(Mutex::new(ProgressState {
                label: "Preparing...".into(),
                value: 0,
                done: false,
                error: None,
            })),
            fonts,
            prev_mouse_down: false,
        }
    }

    pub fn update_and_draw(
        &mut self,
        renderer: &mut Renderer,
        mx: i32,
        my: i32,
        mouse_down: bool,
    ) {
        let mouse_released = self.prev_mouse_down && !mouse_down;
        self.prev_mouse_down = mouse_down;

        self.prev_button.update(mx, my, mouse_down);
        self.next_button.update(mx, my, mouse_down);

        // Handle clicks.
        if self.prev_button.clicked(mx, my, mouse_released) {
            std::process::exit(0);
        }
        if self.next_button.clicked(mx, my, mouse_released) {
            match self.panel {
                Panel::BeforeInstall | Panel::Failure => self.start_uninstall(),
                _ => {}
            }
        }

        // Panel-specific input.
        if self.panel == Panel::Failure {
            self.copy_error_button.update(mx, my, mouse_down);
            if self.copy_error_button.clicked(mx, my, mouse_released) {
                let _ = std::process::Command::new("cmd")
                    .args(["/C", &format!("echo {} | clip", self.failure_detail)])
                    .output();
            }
        }

        // Poll progress.
        if self.panel == Panel::Install {
            self.poll_progress();
        }

        // Draw.
        renderer.clear(BG_COLOR);
        renderer.draw_image(90, 36, &self.flash_logo);

        self.fonts
            .draw_text(renderer, 233, 54, &self.title_text, 32.0, FG_COLOR);
        self.fonts
            .draw_text(renderer, 280, 99, &self.subtitle_text, 17.0, FG_COLOR);

        // Separator.
        renderer.fill_rect(0, 270, WIDTH as i32, 1, 0x00696969);

        match self.panel {
            Panel::BeforeInstall => self.before_label.draw(renderer, &self.fonts),
            Panel::Install => {
                self.progress_header.draw(renderer, &self.fonts);
                self.progress_label.draw(renderer, &self.fonts);
                self.progress_bar.draw(renderer);
            }
            Panel::Complete => self.complete_label.draw(renderer, &self.fonts),
            Panel::Failure => {
                self.failure_text_label.draw(renderer, &self.fonts);
                let detail = if self.failure_detail.len() > 300 {
                    &self.failure_detail[..300]
                } else {
                    &self.failure_detail
                };
                self.fonts.draw_text_multiline(
                    renderer,
                    PANEL_X + 4,
                    PANEL_Y + 44,
                    detail,
                    11.0,
                    FG_COLOR,
                    1.0,
                );
                self.copy_error_button.draw(renderer, &self.fonts);
            }
        }

        self.prev_button.draw(renderer, &self.fonts);
        self.next_button.draw(renderer, &self.fonts);
    }

    fn start_uninstall(&mut self) {
        self.panel = Panel::Install;
        self.prev_button.enabled = false;
        self.next_button.visible = false;
        self.progress_bar.maximum = UNINSTALL_TICKS;
        self.progress_bar.value = 0;

        {
            let mut state = self.progress_state.lock().unwrap();
            state.label = "Preparing...".into();
            state.value = 0;
            state.done = false;
            state.error = None;
        }

        let progress = Arc::clone(&self.progress_state);
        std::thread::spawn(move || {
            let callback = ThreadProgressCallback {
                state: Arc::clone(&progress),
            };

            let redir = redirection::disable_redirection();
            let result = uninstaller::uninstall(&callback);
            redirection::enable_redirection(redir);

            let mut state = progress.lock().unwrap();
            state.done = true;
            if let Err(e) = result {
                state.error = Some(e.to_string());
            }
        });
    }

    fn poll_progress(&mut self) {
        let state = self.progress_state.lock().unwrap();
        self.progress_label.text = state.label.clone();
        self.progress_bar.value = state.value;

        if state.done {
            if let Some(ref err) = state.error {
                self.failure_detail = err.clone();
                drop(state);
                self.open_failure();
            } else {
                drop(state);
                self.open_complete();
            }
        }
    }

    fn open_complete(&mut self) {
        self.panel = Panel::Complete;
        self.prev_button.text = "QUIT".into();
        self.prev_button.enabled = true;
        self.next_button.visible = false;
    }

    fn open_failure(&mut self) {
        self.panel = Panel::Failure;
        self.prev_button.text = "QUIT".into();
        self.prev_button.enabled = true;
        self.next_button.text = "RETRY".into();
        self.next_button.visible = true;
    }
}

struct ThreadProgressCallback {
    state: Arc<Mutex<ProgressState>>,
}

impl ProgressCallback for ThreadProgressCallback {
    fn update_progress_label(&self, text: &str, tick: bool) {
        let mut state = self.state.lock().unwrap();
        state.label = text.to_string();
        if tick {
            state.value += 1;
        }
    }

    fn tick_progress(&self) {
        let mut state = self.state.lock().unwrap();
        state.value += 1;
    }
}

fn load_resource_image(name: &str) -> RgbaImage {
    let bytes: &[u8] = match name {
        "flashLogo.png" => include_bytes!("../../../resources/flashLogo.png"),
        _ => return RgbaImage::empty(),
    };
    RgbaImage::from_png_bytes(bytes)
}
