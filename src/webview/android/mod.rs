use std::rc::Rc;

use crate::{
    application::window::Window,
    Result
};

use super::{WebViewAttributes, WebContext};


pub struct InnerWebView {}


impl InnerWebView {
    
  pub fn new(
    window: Rc<Window>,
    mut attributes: WebViewAttributes,
    web_context: Option<&mut WebContext>,
  ) -> Result<Self> {
      todo!()
  }

  pub fn print(&self) {
      todo!()
  }

  pub fn eval(&self, js: &str) -> Result<()> {
      todo!()
  }

  pub fn focus(&self) {
      todo!()
  }

  /// Open the web inspector which is usually called dev tool.
  pub fn devtool(&self) {
      todo!()
  }
}

pub fn platform_webview_version() -> Result<String> {
    todo!()
}
