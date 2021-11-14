/*!
    # fltk-form

    This crate aims to simplify generating gui from a data structure.

    ## Usage
    ```toml,no_run
    [dependencies]
    fltk = "1.2.16"
    fltk-form = "0.1"
    fltk-form-derive = "0.1"
    ```

    ## Example
    ```rust
    #[macro_use]
    extern crate fltk_form_derive;

    use fltk::{prelude::*, *};
    use fltk_form::{FltkForm, HasProps};

    #[derive(Copy, Debug, Clone, FltkForm)]
    pub enum MyEnum {
        A,
        B,
        C,
    }

    #[derive(Debug, Clone, FltkForm)]
    pub struct MyStruct {
        a: f64,
        b: f64,
        c: String,
        d: MyEnum,
        e: bool,
    }

    impl MyStruct {
        pub fn new() -> Self {
            Self {
                a: 0.0,
                b: 3.0,
                c: String::new(),
                d: MyEnum::A,
                e: true,
            }
        }
    }

    fn main() {
        let my_struct = MyStruct::new();

        let a = app::App::default().with_scheme(app::Scheme::Gtk);
        app::set_background_color(222, 222, 222);

        let mut win = window::Window::default().with_size(400, 300);
        let mut grp = group::Group::default()
            .with_size(300, 200)
            .center_of_parent();
        let form = my_struct.generate();
        grp.end();
        let mut btn = button::Button::default()
            .with_label("print")
            .with_size(80, 30)
            .below_of(&grp, 5)
            .center_x(&grp);
        grp.set_frame(enums::FrameType::EngravedFrame);
        win.end();
        win.show();

        let v = form.get_prop("b");
        assert_eq!(v, Some("3.0".to_owned()));

        btn.set_callback(move |_| {
            println!("{:?}", form.get_props());
        });

        a.run().unwrap();
    }
    ```
*/

use fltk::{prelude::*, *};
use std::collections::HashMap;
use std::mem::transmute;

#[derive(Clone, Debug)]
pub struct Form {
    grp: group::Group,
}

impl Default for Form {
    fn default() -> Self {
        Form::new(0, 0, 0, 0, None)
    }
}

impl Form {
    pub fn new<S: Into<Option<&'static str>>>(x: i32, y: i32, w: i32, h: i32, label: S) -> Self {
        let grp = group::Group::new(x, y, w, h, label);
        grp.end();
        Self {
            grp
        }
    }

    pub fn default_fill() -> Self {
        Form::default().size_of_parent().center_of_parent()
    }

    pub fn set_data<T: FltkForm>(&mut self, data: T) {
        self.begin();
        let mut w = data.generate();
        w.resize(self.x(), self.y(), self.w(), self.h());
        self.end();
    }

    pub fn from_data<T: FltkForm>(mut self, data: T) -> Self {
        self.set_data(data);
        self
    }

    pub fn get_prop(&self, prop: &str) -> Option<String> {
        if let Some(child) = self.grp.child(0) {
            if let Some(grp) = child.as_group() {
                for child in grp.into_iter() {
                    if child.label() == prop {
                        let val = unsafe {
                            let ptr = child.raw_user_data();
                            if ptr.is_null() {
                                return None;
                            }
                            ptr as usize
                        };
                        match val {
                            1 => {
                                let inp = unsafe { input::Input::from_widget_ptr(child.as_widget_ptr() as _) };
                                return Some(inp.value());
                            }
                            2 => {
                                let inp =
                                    unsafe { button::CheckButton::from_widget_ptr(child.as_widget_ptr() as _) };
                                return Some(format!("{}", inp.value()));
                            }
                            3 => {
                                let choice =
                                    unsafe { menu::Choice::from_widget_ptr(child.as_widget_ptr() as _) };
                                return choice.choice();
                            }
                            _ => {
                                return None;
                            }
                        }
                    }
                }
                None
            } else {
                None
            }
        } else {
            None
        }
    }

    pub fn get_props(&self) -> HashMap<String, String> {
        let mut temp = HashMap::new();
        if let Some(c) = self.grp.child(0) {
            if let Some(grp) = c.as_group() {
                for child in grp.into_iter() {
                    if !child.label().is_empty() {
                        if let Some(prop) = self.get_prop(&child.label()) {
                            temp.insert(
                                child.label().clone(),
                                prop,
                            );
                        }
                    }
                }
            }
        }
        temp
    }

    pub fn rename_prop(&self, prop: &str, new_name: &str) {
        if let Some(child) = self.grp.child(0) {
            if let Some(grp) = child.as_group() {
                for mut child in grp.into_iter() {
                    if child.label() == prop {
                        child.set_label(new_name);
                    }
                }
            }
        }
    }
}

fltk::widget_extends!(Form, group::Group, grp);

pub trait FltkForm {
    fn generate(&self) -> Box<dyn WidgetExt>;
}

impl FltkForm for f64 {
    fn generate(&self) -> Box<dyn WidgetExt> {
        let mut i = input::FloatInput::default();
        let val = format!("{:?}", *self);
        i.set_value(&val);
        unsafe {
            i.set_raw_user_data(transmute(1_usize));
        }
        Box::new(i)
    }
}

impl FltkForm for f32 {
    fn generate(&self) -> Box<dyn WidgetExt> {
        let mut i = input::FloatInput::default();
        let val = format!("{:?}", *self);
        i.set_value(&val);
        unsafe {
            i.set_raw_user_data(transmute(1_usize));
        }
        Box::new(i)
    }
}

impl FltkForm for i32 {
    fn generate(&self) -> Box<dyn WidgetExt> {
        let mut i = input::IntInput::default();
        let val = format!("{:?}", *self);
        i.set_value(&val);
        unsafe {
            i.set_raw_user_data(transmute(1_usize));
        }
        Box::new(i)
    }
}

impl FltkForm for u32 {
    fn generate(&self) -> Box<dyn WidgetExt> {
        let mut i = input::IntInput::default();
        let val = format!("{:?}", *self);
        i.set_value(&val);
        unsafe {
            i.set_raw_user_data(transmute(1_usize));
        }
        Box::new(i)
    }
}

impl FltkForm for i64 {
    fn generate(&self) -> Box<dyn WidgetExt> {
        let mut i = input::IntInput::default();
        let val = format!("{:?}", *self);
        i.set_value(&val);
        unsafe {
            i.set_raw_user_data(transmute(1_usize));
        }
        Box::new(i)
    }
}

impl FltkForm for u64 {
    fn generate(&self) -> Box<dyn WidgetExt> {
        let mut i = input::IntInput::default();
        let val = format!("{:?}", *self);
        i.set_value(&val);
        unsafe {
            i.set_raw_user_data(transmute(1_usize));
        }
        Box::new(i)
    }
}

impl FltkForm for isize {
    fn generate(&self) -> Box<dyn WidgetExt> {
        let mut i = input::IntInput::default();
        let val = format!("{:?}", *self);
        i.set_value(&val);
        unsafe {
            i.set_raw_user_data(transmute(1_usize));
        }
        Box::new(i)
    }
}

impl FltkForm for usize {
    fn generate(&self) -> Box<dyn WidgetExt> {
        let mut i = input::IntInput::default();
        let val = format!("{:?}", *self);
        i.set_value(&val);
        unsafe {
            i.set_raw_user_data(transmute(1_usize));
        }
        Box::new(i)
    }
}

impl FltkForm for i8 {
    fn generate(&self) -> Box<dyn WidgetExt> {
        let mut i = input::IntInput::default();
        let val = format!("{:?}", *self);
        i.set_value(&val);
        unsafe {
            i.set_raw_user_data(transmute(1_usize));
        }
        Box::new(i)
    }
}

impl FltkForm for u8 {
    fn generate(&self) -> Box<dyn WidgetExt> {
        let mut i = input::IntInput::default();
        let val = format!("{:?}", *self);
        i.set_value(&val);
        unsafe {
            i.set_raw_user_data(transmute(1_usize));
        }
        Box::new(i)
    }
}

impl FltkForm for i16 {
    fn generate(&self) -> Box<dyn WidgetExt> {
        let mut i = input::IntInput::default();
        let val = format!("{:?}", *self);
        i.set_value(&val);
        unsafe {
            i.set_raw_user_data(transmute(1_usize));
        }
        Box::new(i)
    }
}

impl FltkForm for u16 {
    fn generate(&self) -> Box<dyn WidgetExt> {
        let mut i = input::IntInput::default();
        let val = format!("{:?}", *self);
        i.set_value(&val);
        unsafe {
            i.set_raw_user_data(transmute(1_usize));
        }
        Box::new(i)
    }
}

impl FltkForm for String {
    fn generate(&self) -> Box<dyn WidgetExt> {
        let mut i = input::Input::default();
        let val = self.clone();
        i.set_value(&val);
        unsafe {
            i.set_raw_user_data(transmute(1_usize));
        }
        Box::new(i)
    }
}

impl FltkForm for &str {
    fn generate(&self) -> Box<dyn WidgetExt> {
        let i = frame::Frame::default().with_label(self);
        Box::new(i)
    }
}

impl FltkForm for bool {
    fn generate(&self) -> Box<dyn WidgetExt> {
        let mut i = button::CheckButton::default().with_align(enums::Align::Left);
        i.set_value(*self);
        i.clear_visible_focus();
        unsafe {
            i.set_raw_user_data(transmute(2_usize));
        }
        Box::new(i)
    }
}

impl<T> FltkForm for Vec<T> where T: FltkForm {
    fn generate(&self) -> Box<dyn WidgetExt> {
        let mut g = group::Pack::default();
        g.set_spacing(5);
        for v in self.iter() {
            let mut w = v.generate();
            w.set_align(enums::Align::Left);
            w.set_size(w.w(), 30);
        }
        g.end();
        Box::new(g)
    }
}

#[allow(clippy::borrowed_box)]
fn rename_prop_(wid: &Box<dyn WidgetExt>, prop: &str, new_name: &str) {
    if let Some(grp) = wid.as_group() {
        for mut child in grp.into_iter() {
            if child.label() == prop {
                child.set_label(new_name);
            }
        }
    }
}

#[allow(clippy::borrowed_box)]
fn get_prop_(wid: &Box<dyn WidgetExt>, prop: &str) -> Option<String> {
    if let Some(grp) = wid.as_group() {
        for child in grp.into_iter() {
            if child.label() == prop {
                let val = unsafe {
                    let ptr = child.raw_user_data();
                    if ptr.is_null() {
                        return None;
                    }
                    ptr as usize
                };
                match val {
                    1 => {
                        let inp = unsafe { input::Input::from_widget_ptr(child.as_widget_ptr() as _) };
                        return Some(inp.value());
                    }
                    2 => {
                        let inp =
                            unsafe { button::CheckButton::from_widget_ptr(child.as_widget_ptr() as _) };
                        return Some(format!("{}", inp.value()));
                    }
                    3 => {
                        let choice =
                            unsafe { menu::Choice::from_widget_ptr(child.as_widget_ptr() as _) };
                        return choice.choice();
                    }
                    _ => {
                        return None;
                    }
                }
            }
        }
        None
    } else {
        None
    }
}

#[allow(clippy::borrowed_box)]
fn get_props_(wid: &Box<dyn WidgetExt>) -> HashMap<String, String> {
    let mut temp = HashMap::new();
    if let Some(grp) = wid.as_group() {
        for child in grp.into_iter() {
            if !child.label().is_empty() {
                if let Some(prop) = get_prop_(wid, &child.label()) {
                    temp.insert(
                        child.label().clone(),
                        prop,
                    );
                }
            }
        }
    }
    temp
}

pub trait HasProps {
    fn get_prop(&self, prop: &str) -> Option<String>;
    fn get_props(&self) -> HashMap<String, String>;
    fn rename_prop(&mut self, prop: &str, new_name: &str);
}

impl HasProps for Box<dyn WidgetExt> {
    fn get_prop(&self, prop: &str) -> Option<String> {
        get_prop_(self, prop)
    }
    fn get_props(&self) -> HashMap<String, String> {
        get_props_(self)
    }
    fn rename_prop(&mut self, prop: &str, new_name: &str) {
        rename_prop_(self, prop, new_name);
    }
}
