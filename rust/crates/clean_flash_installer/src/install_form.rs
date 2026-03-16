use crate::install_flags::{self, InstallFlags};
use crate::installer;
use clean_flash_common::{uninstaller, redirection, update_checker, ProgressCallback};
use clean_flash_ui::font::FontManager;
use clean_flash_ui::renderer::Renderer;
use clean_flash_ui::widgets::button::GradientButton;
use clean_flash_ui::widgets::checkbox::ImageCheckBox;
use clean_flash_ui::widgets::label::Label;
use clean_flash_ui::widgets::progress_bar::ProgressBar;
use std::sync::{Arc, Mutex};

// Window dimensions matching the C# form.
pub const WIDTH: usize = 712;
pub const HEIGHT: usize = 329;
const BG_COLOR: u32 = Renderer::rgb(50, 51, 51);
const FG_COLOR: u32 = Renderer::rgb(245, 245, 245);

const PANEL_X: i32 = 90;
const PANEL_Y: i32 = 162;

const DISCLAIMER_TEXT: &str = "I am aware that Adobe Flash Player is no longer supported, nor provided by Adobe Inc.\n\
Clean Flash Player is a third-party version of Flash Player built from the latest Flash Player version with adware removed.\n\n\
Adobe is not required by any means to provide support for this version of Flash Player.";

const COMPLETE_INSTALL_TEXT: &str = "Clean Flash Player has been successfully installed!\n\
Don't forget, Flash Player is no longer compatible with new browsers.\n\n\
For browser recommendations and Flash Player updates, check out Clean Flash Player's website!";

const COMPLETE_UNINSTALL_TEXT: &str = "\nAll versions of Flash Player have been successfully uninstalled.\n\n\
If you ever change your mind, check out Clean Flash Player's website!";

/// Which panel is currently shown in the wizard.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Panel {
    Disclaimer,
    Choice,
    PlayerChoice,
    DebugChoice,
    BeforeInstall,
    Install,
    Complete,
    Failure,
}

/// Shared progress state set by the background install thread.
pub struct ProgressState {
    pub label: String,
    pub value: i32,
    pub done: bool,
    pub error: Option<String>,
}

/// Full application state for the installer form.
pub struct InstallForm {
    scale: f32,
    pub panel: Panel,
    // Header
    pub title_text: String,
    pub subtitle_text: String,
    flash_logo_cache: clean_flash_ui::flash_logo::FlashLogoCache,
    // Navigation buttons
    pub prev_button: GradientButton,
    pub next_button: GradientButton,
    // Disclaimer panel
    pub disclaimer_label: Label,
    pub disclaimer_box: ImageCheckBox,
    // Choice panel (browser plugins)
    pub browser_ask_label: Label,
    pub pepper_box: ImageCheckBox,
    pub pepper_label: Label,
    pub netscape_box: ImageCheckBox,
    pub netscape_label: Label,
    pub activex_box: ImageCheckBox,
    pub activex_label: Label,
    // Player choice panel
    pub player_ask_label: Label,
    pub player_box: ImageCheckBox,
    pub player_label: Label,
    pub player_desktop_box: ImageCheckBox,
    pub player_desktop_label: Label,
    pub player_start_menu_box: ImageCheckBox,
    pub player_start_menu_label: Label,
    // Debug choice panel
    pub debug_ask_label: Label,
    pub debug_button: GradientButton,
    pub debug_chosen: bool,
    // Before install panel
    pub before_install_label: Label,
    // Install panel
    pub install_header_label: Label,
    pub progress_label: Label,
    pub progress_bar: ProgressBar,
    // Complete panel
    pub complete_label: Label,
    // Failure panel
    pub failure_text_label: Label,
    pub failure_detail: String,
    pub copy_error_button: GradientButton,
    // Shared progress state (for background thread communication).
    pub progress_state: Arc<Mutex<ProgressState>>,
    // Fonts loaded once.
    pub fonts: FontManager,
    // Mouse tracking
    prev_mouse_down: bool,
}

impl InstallForm {
    pub fn new(scale: f32) -> Self {
        // Integer coordinate scaler.
        let s = |v: i32| (v as f32 * scale).round() as i32;
        // Font size / spacing scaler.
        let sf = |v: f32| v * scale;
        // Scaled label helper: font_size is the logical total (base+offset).
        let lbl = |x: i32, y: i32, text: &str, size: f32| {
            let mut l = Label::new(s(x), s(y), text, sf(size));
            l.line_spacing = sf(2.0);
            l.max_width = sf((WIDTH as i32 - x - 60) as f32);
            l
        };
        // Scaled button helper.
        let btn = |x: i32, y: i32, w: i32, h: i32, text: &str| GradientButton {
            font_size: sf(13.0),
            ..GradientButton::new(s(x), s(y), s(w), s(h), text)
        };
        // Scaled checkbox helper.
        let chk = |x: i32, y: i32| ImageCheckBox::with_size(s(x), s(y), s(21));
        let version = update_checker::FLASH_VERSION;
        let title_text = "Clean Flash Player".to_string();
        let subtitle_text = format!("built from version {} (China)", version);

        let fonts = FontManager::new();

        Self {
            scale,
            panel: Panel::Disclaimer,
            title_text,
            subtitle_text,
            flash_logo_cache: clean_flash_ui::flash_logo::FlashLogoCache::new(),
            prev_button: btn(90, 286, 138, 31, "QUIT"),
            next_button: btn(497, 286, 138, 31, "AGREE"),
            // Disclaimer panel
            disclaimer_label: lbl(PANEL_X + 25, PANEL_Y, DISCLAIMER_TEXT, 15.0),
            disclaimer_box: chk(PANEL_X, PANEL_Y),
            // Choice panel
            browser_ask_label: lbl(PANEL_X - 2, PANEL_Y + 2, "Which browser plugins would you like to install?", 15.0),
            pepper_box: chk(PANEL_X, PANEL_Y + 47),
            pepper_label: lbl(PANEL_X + 24, PANEL_Y + 47, "Pepper API (PPAPI)\n(Chrome/Opera/Brave)", 15.0),
            netscape_box: chk(PANEL_X + 186, PANEL_Y + 47),
            netscape_label: lbl(PANEL_X + 210, PANEL_Y + 47, "Netscape API (NPAPI)\n(Firefox/ESR/Waterfox)", 15.0),
            activex_box: chk(PANEL_X + 365, PANEL_Y + 47),
            activex_label: lbl(PANEL_X + 389, PANEL_Y + 47, "ActiveX (OCX)\n(IE/Embedded/Desktop)", 15.0),
            // Player choice panel
            player_ask_label: lbl(PANEL_X - 2, PANEL_Y + 2, "Would you like to install the standalone Flash Player?", 15.0),
            player_box: chk(PANEL_X, PANEL_Y + 47),
            player_label: lbl(PANEL_X + 24, PANEL_Y + 47, "Install Standalone\nFlash Player", 15.0),
            player_desktop_box: chk(PANEL_X + 186, PANEL_Y + 47),
            player_desktop_label: lbl(PANEL_X + 210, PANEL_Y + 47, "Create Shortcuts\non Desktop", 15.0),
            player_start_menu_box: chk(PANEL_X + 365, PANEL_Y + 47),
            player_start_menu_label: lbl(PANEL_X + 389, PANEL_Y + 47, "Create Shortcuts\nin Start Menu", 15.0),
            // Debug choice panel
            debug_ask_label: lbl(
                PANEL_X - 2,
                PANEL_Y + 2,
                "Would you like to install the debug version of Clean Flash Player?\n\
You should only choose the debug version if you are planning to create Flash applications.\n\
If you are not sure, simply press NEXT.",
                15.0,
            ),
            debug_button: btn(PANEL_X + 186, PANEL_Y + 65, 176, 31, "INSTALL DEBUG VERSION"),
            debug_chosen: false,
            // Before install panel
            before_install_label: lbl(PANEL_X + 3, PANEL_Y + 2, "", 15.0),
            // Install panel
            install_header_label: lbl(PANEL_X + 3, PANEL_Y, "Installation in progress...", 15.0),
            progress_label: lbl(PANEL_X + 46, PANEL_Y + 30, "Preparing...", 15.0),
            progress_bar: ProgressBar::new(s(PANEL_X + 49), s(PANEL_Y + 58), s(451), s(23)),
            // Complete panel
            complete_label: lbl(PANEL_X, PANEL_Y, "", 15.0),
            // Failure panel
            failure_text_label: lbl(
                PANEL_X + 3,
                PANEL_Y + 2,
                "Oops! The installation process has encountered an unexpected problem.\n\
The following details could be useful. Press the Retry button to try again.",
                15.0,
            ),
            failure_detail: String::new(),
            copy_error_button: btn(PANEL_X + 441, PANEL_Y + 58, 104, 31, "COPY"),
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

    /// Scale a logical integer coordinate to physical pixels.
    fn s(&self, v: i32) -> i32 { (v as f32 * self.scale).round() as i32 }
    /// Scale a logical float value to physical pixels.
    fn sf(&self, v: f32) -> f32 { v * self.scale }

    /// Called each frame: handle input, update state, draw.
    pub fn update_and_draw(
        &mut self,
        renderer: &mut Renderer,
        mx: i32,
        my: i32,
        mouse_down: bool,
    ) {
        let mouse_released = self.prev_mouse_down && !mouse_down;
        self.prev_mouse_down = mouse_down;

        // Update navigation button hover states.
        self.prev_button.update(mx, my, mouse_down);
        self.next_button.update(mx, my, mouse_down);

        // Handle click events.
        self.handle_input(mx, my, mouse_released);

        // Poll progress state from background thread if installing.
        if self.panel == Panel::Install {
            self.poll_progress();
        }

        // ----- Draw -----
        renderer.clear(BG_COLOR);

        // Header: flash logo (cached software render).
        self.flash_logo_cache.draw(
            renderer, self.s(90), self.s(36), self.s(109), self.s(107),
        );

        // Title.
        self.fonts.draw_text(
            renderer,
            self.s(233),
            self.s(54),
            &self.title_text,
            self.sf(32.0),
            FG_COLOR,
        );

        // Subtitle.
        self.fonts.draw_text(
            renderer,
            self.s(280),
            self.s(99),
            &self.subtitle_text,
            self.sf(17.0),
            FG_COLOR,
        );

        // Separator line.
        renderer.fill_rect(0, self.s(270), renderer.width as i32, self.s(1).max(1), Renderer::rgb(105, 105, 105));

        // Draw current panel.
        match self.panel {
            Panel::Disclaimer => self.draw_disclaimer(renderer),
            Panel::Choice => self.draw_choice(renderer),
            Panel::PlayerChoice => self.draw_player_choice(renderer),
            Panel::DebugChoice => self.draw_debug_choice(renderer),
            Panel::BeforeInstall => self.draw_before_install(renderer),
            Panel::Install => self.draw_install(renderer),
            Panel::Complete => self.draw_complete(renderer),
            Panel::Failure => self.draw_failure(renderer),
        }

        // Navigation buttons.
        self.prev_button.draw(renderer, &self.fonts);
        self.next_button.draw(renderer, &self.fonts);
    }

    fn handle_input(&mut self, mx: i32, my: i32, mouse_released: bool) {
        // Navigation button clicks.
        if self.prev_button.clicked(mx, my, mouse_released) {
            self.on_prev_clicked();
            return;
        }
        if self.next_button.clicked(mx, my, mouse_released) {
            self.on_next_clicked();
            return;
        }

        // Panel-specific input.
        match self.panel {
            Panel::Disclaimer => {
                let toggled = self.disclaimer_box.toggle_if_clicked(mx, my, mouse_released);
                if toggled || self.disclaimer_label.clicked(mx, my, mouse_released, &self.fonts) {
                    if !toggled {
                        self.disclaimer_box.checked = !self.disclaimer_box.checked;
                    }
                    self.next_button.enabled = self.disclaimer_box.checked;
                }
            }
            Panel::Choice => {
                self.pepper_box.toggle_if_clicked(mx, my, mouse_released);
                self.netscape_box.toggle_if_clicked(mx, my, mouse_released);
                self.activex_box.toggle_if_clicked(mx, my, mouse_released);
                if self.pepper_label.clicked(mx, my, mouse_released, &self.fonts) {
                    self.pepper_box.checked = !self.pepper_box.checked;
                }
                if self.netscape_label.clicked(mx, my, mouse_released, &self.fonts) {
                    self.netscape_box.checked = !self.netscape_box.checked;
                }
                if self.activex_label.clicked(mx, my, mouse_released, &self.fonts) {
                    self.activex_box.checked = !self.activex_box.checked;
                }
            }
            Panel::PlayerChoice => {
                self.player_box.toggle_if_clicked(mx, my, mouse_released);
                self.player_desktop_box.toggle_if_clicked(mx, my, mouse_released);
                self.player_start_menu_box.toggle_if_clicked(mx, my, mouse_released);
                if self.player_label.clicked(mx, my, mouse_released, &self.fonts) {
                    self.player_box.checked = !self.player_box.checked;
                }
                if self.player_desktop_label.clicked(mx, my, mouse_released, &self.fonts) && self.player_box.checked {
                    self.player_desktop_box.checked = !self.player_desktop_box.checked;
                }
                if self.player_start_menu_label.clicked(mx, my, mouse_released, &self.fonts) && self.player_box.checked {
                    self.player_start_menu_box.checked = !self.player_start_menu_box.checked;
                }
                // Disable sub-options when player unchecked.
                self.player_desktop_box.enabled = self.player_box.checked;
                self.player_start_menu_box.enabled = self.player_box.checked;
                if !self.player_box.checked {
                    self.player_desktop_box.checked = false;
                    self.player_start_menu_box.checked = false;
                }
            }
            Panel::DebugChoice => {
                if self.debug_button.clicked(mx, my, mouse_released) {
                    // In the C# app this shows a MessageBox. For the Rust port we toggle directly.
                    self.debug_chosen = true;
                    self.open_before_install();
                }
                self.debug_button.update(mx, my, self.prev_mouse_down);
            }
            Panel::Failure => {
                self.copy_error_button.update(mx, my, self.prev_mouse_down);
                if self.copy_error_button.clicked(mx, my, mouse_released) {
                    // Copy error to clipboard via clip.exe.
                    let _ = std::process::Command::new("cmd")
                        .args(["/C", &format!("echo {} | clip", self.failure_detail)])
                        .output();
                }
            }
            _ => {}
        }
    }

    fn on_prev_clicked(&mut self) {
        match self.panel {
            Panel::Disclaimer | Panel::Complete | Panel::Failure => {
                std::process::exit(0);
            }
            Panel::Choice => self.open_disclaimer(),
            Panel::PlayerChoice => self.open_choice(),
            Panel::DebugChoice => self.open_player_choice(),
            Panel::BeforeInstall => self.open_debug_choice(),
            _ => {}
        }
    }

    fn on_next_clicked(&mut self) {
        match self.panel {
            Panel::Disclaimer => self.open_choice(),
            Panel::Choice => self.open_player_choice(),
            Panel::PlayerChoice => self.open_debug_choice(),
            Panel::DebugChoice => self.open_before_install(),
            Panel::BeforeInstall | Panel::Failure => self.open_install(),
            _ => {}
        }
    }

    fn open_disclaimer(&mut self) {
        self.panel = Panel::Disclaimer;
        self.prev_button.text = "QUIT".into();
        self.next_button.text = "AGREE".into();
        self.next_button.visible = true;
        self.next_button.enabled = self.disclaimer_box.checked;
        self.prev_button.enabled = true;
    }

    fn open_choice(&mut self) {
        self.panel = Panel::Choice;
        self.prev_button.text = "BACK".into();
        self.next_button.text = "NEXT".into();
        self.next_button.visible = true;
        self.next_button.enabled = true;
        self.prev_button.enabled = true;
    }

    fn open_player_choice(&mut self) {
        self.panel = Panel::PlayerChoice;
        self.prev_button.text = "BACK".into();
        self.next_button.text = "NEXT".into();
        self.next_button.visible = true;
        self.next_button.enabled = true;
        self.prev_button.enabled = true;
    }

    fn open_debug_choice(&mut self) {
        self.panel = Panel::DebugChoice;
        self.debug_chosen = false;
        self.prev_button.text = "BACK".into();
        self.next_button.text = "NEXT".into();
        self.next_button.visible = true;
        self.next_button.enabled = true;
        self.prev_button.enabled = true;
    }

    fn open_before_install(&mut self) {
        self.panel = Panel::BeforeInstall;
        self.prev_button.text = "BACK".into();
        self.prev_button.enabled = true;

        let has_plugins =
            self.pepper_box.checked || self.netscape_box.checked || self.activex_box.checked || self.player_box.checked;

        if has_plugins {
            let mut browsers = Vec::new();
            if self.pepper_box.checked {
                browsers.push("Google Chrome");
            }
            if self.netscape_box.checked {
                browsers.push("Mozilla Firefox");
            }
            if self.activex_box.checked {
                browsers.push("Internet Explorer");
            }

            let browser_str = join_with_and(&browsers);
            self.before_install_label.text = format!(
                "You are about to install Clean Flash Player.\n\
Please close any browser windows running Flash content before you continue.\n\n\
The installer will close all browser windows running Flash, uninstall previous versions of Flash Player and Flash Center, and install Flash for {}.",
                browser_str
            );
            self.next_button.text = "INSTALL".into();
        } else {
            self.before_install_label.text =
                "You are about to uninstall Clean Flash Player.\n\
Please close any browser windows running Flash content before you continue.\n\n\
The installer will completely remove all versions of Flash Player from this computer, including Clean Flash Player and older versions of Adobe Flash Player."
                    .to_string();
            self.next_button.text = "UNINSTALL".into();
        }
        self.next_button.visible = true;
        self.next_button.enabled = true;
    }

    fn open_install(&mut self) {
        self.panel = Panel::Install;
        self.prev_button.enabled = false;
        self.next_button.visible = false;

        let mut flags = InstallFlags::new();
        flags.set_conditionally(self.pepper_box.checked, install_flags::PEPPER);
        flags.set_conditionally(self.netscape_box.checked, install_flags::NETSCAPE);
        flags.set_conditionally(self.activex_box.checked, install_flags::ACTIVEX);
        flags.set_conditionally(self.player_box.checked, install_flags::PLAYER);
        flags.set_conditionally(self.player_desktop_box.checked, install_flags::PLAYER_DESKTOP);
        flags.set_conditionally(
            self.player_start_menu_box.checked,
            install_flags::PLAYER_START_MENU,
        );
        flags.set_conditionally(self.debug_chosen, install_flags::DEBUG);

        self.progress_bar.maximum = flags.get_ticks() as i32;
        self.progress_bar.value = 0;

        // Reset shared state.
        {
            let mut state = self.progress_state.lock().unwrap();
            state.label = "Preparing...".into();
            state.value = 0;
            state.done = false;
            state.error = None;
        }

        // Spawn background thread.
        let progress = Arc::clone(&self.progress_state);
        std::thread::spawn(move || {
            let callback = ThreadProgressCallback {
                state: Arc::clone(&progress),
            };

            let redir = redirection::disable_redirection();

            let result = (|| -> Result<(), clean_flash_common::InstallError> {
                uninstaller::uninstall(&callback)?;
                installer::install(&callback, &mut flags)?;
                Ok(())
            })();

            redirection::enable_redirection(redir);

            let mut state = progress.lock().unwrap();
            if let Err(e) = result {
                state.error = Some(e.to_string());
            }
            state.done = true;
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

        if self.pepper_box.checked || self.netscape_box.checked || self.activex_box.checked {
            self.complete_label.text = COMPLETE_INSTALL_TEXT.to_string();
        } else {
            self.complete_label.text = COMPLETE_UNINSTALL_TEXT.to_string();
        }
    }

    fn open_failure(&mut self) {
        self.panel = Panel::Failure;
        self.prev_button.text = "QUIT".into();
        self.prev_button.enabled = true;
        self.next_button.text = "RETRY".into();
        self.next_button.visible = true;
    }

    // ---- Drawing helpers ----

    fn draw_disclaimer(&self, r: &mut Renderer) {
        self.disclaimer_box.draw(r);
        self.disclaimer_label.draw(r, &self.fonts);
    }

    fn draw_choice(&self, r: &mut Renderer) {
        self.browser_ask_label.draw(r, &self.fonts);
        self.pepper_box.draw(r);
        self.pepper_label.draw(r, &self.fonts);
        self.netscape_box.draw(r);
        self.netscape_label.draw(r, &self.fonts);
        self.activex_box.draw(r);
        self.activex_label.draw(r, &self.fonts);
    }

    fn draw_player_choice(&self, r: &mut Renderer) {
        self.player_ask_label.draw(r, &self.fonts);
        self.player_box.draw(r);
        self.player_label.draw(r, &self.fonts);
        self.player_desktop_box.draw(r);
        self.player_desktop_label.draw(r, &self.fonts);
        self.player_start_menu_box.draw(r);
        self.player_start_menu_label.draw(r, &self.fonts);
    }

    fn draw_debug_choice(&mut self, r: &mut Renderer) {
        self.debug_ask_label.draw(r, &self.fonts);
        self.debug_button.draw(r, &self.fonts);
    }

    fn draw_before_install(&self, r: &mut Renderer) {
        self.before_install_label.draw(r, &self.fonts);
    }

    fn draw_install(&self, r: &mut Renderer) {
        self.install_header_label.draw(r, &self.fonts);
        self.progress_label.draw(r, &self.fonts);
        self.progress_bar.draw(r);
    }

    fn draw_complete(&self, r: &mut Renderer) {
        self.complete_label.draw(r, &self.fonts);
    }

    fn draw_failure(&self, r: &mut Renderer) {
        self.failure_text_label.draw(r, &self.fonts);
        // Draw error detail as clipped text.
        let detail_text = if self.failure_detail.len() > 300 {
            &self.failure_detail[..300]
        } else {
            &self.failure_detail
        };
        self.fonts.draw_text_multiline(
            r,
            self.s(PANEL_X + 4),
            self.s(PANEL_Y + 44),
            detail_text,
            self.sf(11.0),
            FG_COLOR,
            self.sf(1.0),
            self.sf((WIDTH as i32 - PANEL_X - 14) as f32),
        );
        self.copy_error_button.draw(r, &self.fonts);
    }
}

/// Progress callback that writes to the shared state from the background thread.
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

fn join_with_and(items: &[&str]) -> String {
    match items.len() {
        0 => String::new(),
        1 => items[0].to_string(),
        2 => format!("{} and {}", items[0], items[1]),
        _ => {
            let (last, rest) = items.split_last().unwrap();
            format!("{} and {}", rest.join(", "), last)
        }
    }
}

