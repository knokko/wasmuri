use crate::*;

pub struct SimpleFlatBuddy {
    subscriptions: ComponentSubscriptions,

    last_render_result: Option<RenderResultStruct>,

    create_next_menu: Option<Box<dyn Fn(Box<dyn Component>) -> Box<dyn Component>>>,

    requested_render: bool,
    has_changes: bool,
}

impl SimpleFlatBuddy {
    pub fn new() -> Self {
        Self {
            subscriptions: ComponentSubscriptions::new(),
            last_render_result: None,
            create_next_menu: None,

            // Components should always render right after they are attached
            requested_render: true,
            // This one is initially true to indicate the requested_render
            has_changes: true,
        }
    }

    pub fn get_subscriptions(&self) -> &ComponentSubscriptions {
        &self.subscriptions
    }

    pub fn did_request_render(&self) -> bool {
        self.requested_render
    }

    pub fn clear_render_request(&mut self) {
        self.requested_render = false;
    }

    pub fn has_changes(&self) -> bool {
        self.has_changes
    }

    pub fn clear_changes(&mut self) {
        self.has_changes = false;
    }

    pub fn get_last_render_result(&self) -> &Option<RenderResultStruct> {
        &self.last_render_result
    }

    pub fn set_last_render_result(&mut self, result: RenderResultStruct) {
        self.last_render_result = Some(result);
    }

    pub fn has_next_menu(&self) -> bool {
        self.create_next_menu.is_some()
    }

    pub fn create_next_menu(&mut self, current_menu: Box<dyn Component>) -> Box<dyn Component> {
        let new_menu = self
            .create_next_menu
            .as_ref()
            .expect("Only call this method after has_next_menu returned true")(
            current_menu
        );
        self.create_next_menu = None;
        new_menu
    }
}

impl ComponentBuddy for SimpleFlatBuddy {
    fn change_menu(
        &mut self,
        create_new_menu: Box<dyn Fn(Box<dyn Component>) -> Box<dyn Component>>,
    ) {
        self.create_next_menu = Some(create_new_menu);
        self.has_changes = true;
    }

    fn request_text_input(&self, start_text: String) -> Option<String> {
        unimplemented!()
    }

    fn request_render(&mut self) {
        if !self.requested_render {
            self.requested_render = true;
            self.has_changes = true;
        }
    }

    fn subscribe_mouse_click(&mut self) {
        if !self.subscriptions.mouse_click {
            self.subscriptions.mouse_click = true;
            self.has_changes = true;
        }
    }

    fn unsubscribe_mouse_click(&mut self) {
        if self.subscriptions.mouse_click {
            self.subscriptions.mouse_click = false;
            self.has_changes = true;
        }
    }

    fn subscribe_mouse_click_out(&mut self) {
        if !self.subscriptions.mouse_click_out {
            self.subscriptions.mouse_click_out = true;
            self.has_changes = true;
        }
    }

    fn unsubscribe_mouse_click_out(&mut self) {
        if self.subscriptions.mouse_click_out {
            self.subscriptions.mouse_click_out = false;
            self.has_changes = true;
        }
    }

    fn subscribe_mouse_move(&self) {
        unimplemented!()
    }

    fn unsubscribe_mouse_move(&self) {
        unimplemented!()
    }

    fn subscribe_mouse_enter(&self) {
        unimplemented!()
    }

    fn unsubscribe_mouse_enter(&self) {
        unimplemented!()
    }

    fn subscribe_mouse_leave(&self) {
        unimplemented!()
    }

    fn unsubscribe_mouse_leave(&self) {
        unimplemented!()
    }

    fn subscribe_char_type(&self) -> Result<(), ()> {
        unimplemented!()
    }

    fn unsubscribe_char_type(&self) {
        unimplemented!()
    }

    fn get_mouse_position(&self, mouse: Mouse) -> Option<Point> {
        unimplemented!()
    }

    fn is_mouse_down(&self, mouse: Mouse, button: MouseButton) -> bool {
        unimplemented!()
    }

    fn is_primary_mouse_down(&self, mouse: Mouse) -> bool {
        unimplemented!()
    }

    fn get_local_mouses(&self) -> Vec<Mouse> {
        unimplemented!()
    }

    fn get_all_mouses(&self) -> Vec<Mouse> {
        unimplemented!()
    }
}