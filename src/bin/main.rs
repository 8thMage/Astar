mod lib;
use lib::game::game;
struct SDLContext {
    _context: sdl2::Sdl,
    _video_subsystem: sdl2::VideoSubsystem,
    _gl_context: sdl2::video::GLContext,
    window: sdl2::video::Window,
    event_pump: sdl2::EventPump,

}
fn create_window(name:&str, width:u32, height:u32) ->SDLContext {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let gl_attributes = video_subsystem.gl_attr();
    gl_attributes.set_context_profile(sdl2::video::GLProfile::Core);
    gl_attributes.set_context_version(3, 3);
    let window = video_subsystem
        .window(&name, width, height)
        .resizable()
        .opengl()
        .build()
        .unwrap();
    let event_pump = sdl_context.event_pump().unwrap();
    let _gl_context = window.gl_create_context().unwrap();
    let _swapped_interval = video_subsystem.gl_set_swap_interval(sdl2::video::SwapInterval::VSync);
    if let Err(string) = _swapped_interval {
        println!("{}", string);
    }
    let _gl =
        gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void);
    
    return SDLContext{
        _context:sdl_context, 
        _video_subsystem: video_subsystem,
        _gl_context, 
        window,
        event_pump,
    };
}

fn main() {
    let mut sdl_context = create_window("AStar", 1000, 500);
    game(&sdl_context.window, &mut sdl_context.event_pump);
    println!("yo");
}