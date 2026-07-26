#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]


/*
use popout::{Dialog, LogicalSize, WindowAttributes, winit::window::WindowButtons };
use popout::winit;
use slint::platform::Key::M; */


use notifica;
pub fn basic_popup<Ta: ToString, Tb: ToString>(title: Ta, text: Tb) {
    // slint::slint! {
	
	// }
	// let popup = PopupBasic::new().unwrap();
	// popup.set_popup_title(slint::SharedString::from(title.to_string()));
	// popup.set_popup_text(slint::SharedString::from(text.to_string()));

	/* let attr = WindowAttributes  {
		inner_size: None,
		min_inner_size: None,
		max_inner_size: None,
		position: None,
		resizable: true,
		enabled_buttons: WindowButtons::all(),
		title: title.to_string(),
		maximized: false,
		visible: true,
		transparent: false,
		blur: false,
		decorations: true,
		window_icon: None,
		preferred_theme: Some(winit::window::Theme::Dark),
		resize_increments: None,
		content_protected: false,
		window_level: winit::window::WindowLevel::AlwaysOnTop,
		active: true,
		cursor: winit::window::Cursor::Icon(winit::window::CursorIcon::Default)
	}; */

	/* match Dialog::new()
		.with_title(title.to_string())
		.with_decorations(false)
		.with_button("Close")
		.with_line(text.to_string())
		.with_resizeable(true)
		.show() {
			Err(e) => {println!("e{}", e)},
			Ok(Some(v)) => {println!("o{}", v)},
			Ok(None) => {println!("None")}
		}; */
	
	match notifica::notify(&title.to_string(), &text.to_string()) {
		Err(e) => {println!("e {}", e)},
		Ok(_) => {println!("o")}
	};

}

use slint;
use slint::*;
slint::slint!{
	import { VerticalBox, HorizontalBox, Button } from "std-widgets.slint";
	export component PopupBasic inherits Window {
		in-out property <string> popup_text;
		in-out property <string> popup_title;
		in-out property <bool> mini;
		callback close_window();
		min-width: 200px;
		no-frame: true;
		resize-border-width: 5px;
		always-on-top: true;
		title: "OA Error";
		minimized: mini;
		VerticalBox {
			Rectangle {
				width: 100%;
				min-height: 50px;
				background: @linear-gradient(165deg, #5BCEFA, #FFFFFF33, #F5A9B8);
				Text {
					text: popup_title;
					wrap: word-wrap;
					font-weight: 500;
				}
			}
			Rectangle {
				Text {
						text: popup_text;
				}
			}
			HorizontalBox {
				alignment: end;
				Rectangle {
					Button {
						width: 100px;
						text: "OK";
						clicked => {root.close();}
					}
				}
			}
		}
	}
}
pub fn basic_popup2<Ta: ToString, Tb: ToString>(title: Ta, text:Tb) {
	if let Ok(ui) = PopupBasic::new() {
		ui.set_mini(true);
		ui.set_popup_title(SharedString::from(title.to_string()));
		ui.set_popup_text(SharedString::from(text.to_string()));
		ui.set_mini(false);
		std::thread::sleep(std::time::Duration::new(25, 0));
	}
}