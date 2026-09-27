use glfw::{self, Context, CursorMode, Glfw, GlfwReceiver, PWindow, WindowEvent};
pub struct Window{
    pub window: glfw::PWindow,
    pub events: glfw::GlfwReceiver<(f64, glfw::WindowEvent)>,
    glfw: glfw::Glfw,
    pub width: u32,
    pub height:u32,
}
//to-do: build toggle functionality
pub enum WindowMode{
    Windowed,
    Fullscreen,
}
impl Window{
    pub fn new(height: u32, width: u32, title: &str ,mode: WindowMode) -> Window{   //maybe think about window settings for scalability do later
        use glfw::fail_on_errors;
        let mut glfw=glfw::init(fail_on_errors).unwrap();
        let (mut window,mut events, mut w, mut h) = match mode{
            WindowMode::Windowed => {
                let (mut window, mut events) = glfw
                    .create_window(width, height, title, glfw::WindowMode::Windowed)
                    .expect("Failed to create window");
                let w=width;
                let h=height;
                (window,events,w,h)

            }
            WindowMode::Fullscreen=> {
                glfw.with_primary_monitor(|glfw, monitor| {
                    let monitor = monitor.unwrap();
                    let vidmode = monitor.get_video_mode().unwrap();
                    let mut w=vidmode.width;
                    let mut h=vidmode.height;
                    let (window,events)=glfw.create_window(
                        vidmode.width,
                        vidmode.height,
                        "My Engine",
                        glfw::WindowMode::FullScreen(monitor),


                    ).expect("Failed to create window");
                    return (window,events,w,h)
                })

            },

        };




        window.make_current();

        glfw.set_swap_interval(glfw::SwapInterval::Sync(1)); //vsync apparently
        window.set_key_polling(true);
        window.focus();
        Window{window, events, glfw,width:w,height:h}

    }
    pub fn is_open(&self)->bool{
        !self.window.should_close()
    }
    pub fn update(&mut self){
        self.window.swap_buffers();
    }
    pub fn poll_events(&mut self)-> Vec<(f64,WindowEvent)>{
        self.glfw.poll_events();
        let mut event_vec: Vec<(f64,WindowEvent)>= Vec::new();
        for (time,event) in glfw::flush_messages(&self.events){
            event_vec.push((time,event));
        }
        event_vec
    }
    pub fn get_proc_address(&mut self,symbol:&str) -> *const std::ffi::c_void {
        self.window
            .get_proc_address(symbol)
            .map_or(std::ptr::null(), |p| p as *const _)
    }
    pub fn get_time(&self)->f64{
        return self.glfw.get_time();
    }
    pub fn get_aspect_ratio(&self)->f32 {
        self.width as f32 / self.height as f32
    }
    pub fn set_mouse_polling(&mut self,a:bool, curs:CursorMode){
        if a==false{

            self.window.set_cursor_pos_polling(false);
            self.window.set_mouse_button_polling(false);
        }
        else{

            self.window.set_cursor_pos_polling(true);
            self.window.set_mouse_button_polling(true);
            self.window.set_cursor_mode(curs);
        }
    }
}