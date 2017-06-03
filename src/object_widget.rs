use cairo::{Context, Format, ImageSurface};
use gtk::DrawingArea;

use gtk::Inhibit;
use gtk::traits::*;
use render;
use std::cell::{Cell, RefCell};
use std::rc::Rc;

use truescad_types::Float;

pub struct ObjectWidget {
    pub drawing_area: DrawingArea,
    pub renderer: Rc<RefCell<::render::Renderer>>,
    mouse_pos: Rc<Cell<(f64, f64)>>,
}

impl ObjectWidget {
    pub fn new() -> ObjectWidget {
        let xw = ObjectWidget {
            drawing_area: DrawingArea::new(),
            renderer: Rc::new(RefCell::new(render::Renderer::new())),
            mouse_pos: Rc::new(Cell::new((0., 0.))),
        };
        {
            let renderer_clone = xw.renderer.clone();
            xw.drawing_area
                .connect_draw(move |_: &DrawingArea, cr: &Context| {
                    let (clip_x1, clip_y1, clip_x2, clip_y2) = cr.clip_extents();
                    let (width, height) = (clip_x2 - clip_x1, clip_y2 - clip_y1);
                    let image = draw_on_image(renderer_clone.clone(), width as i32, height as i32);
                    cr.set_source_surface(&image, 0., 0.);
                    cr.paint();
                    Inhibit(false)
                });
        }
        xw.drawing_area
            .add_events(::gdk::BUTTON1_MASK.bits() as i32);
        xw.drawing_area
            .add_events(::gdk::BUTTON2_MASK.bits() as i32);
        xw.drawing_area
            .add_events(::gdk::BUTTON3_MASK.bits() as i32);
        xw.drawing_area.add_events(1 << 4);

        {
            let mouse_pos_clone = xw.mouse_pos.clone();
            let renderer_clone = xw.renderer.clone();
            xw.drawing_area
                .connect_motion_notify_event(move |da: &DrawingArea,
                                                   em: &::gdk::EventMotion|
                                                   -> Inhibit {
                    let da_alloc = da.get_allocation();
                    let (nx, ny) = em.get_position();
                    let (ox, oy) = mouse_pos_clone.get();
                    let (dx, dy) = (((nx - ox) / da_alloc.width as f64) as Float,
                                    ((ny - oy) / da_alloc.height as f64) as Float);
                    mouse_pos_clone.set(em.get_position());
                    match em.get_state() {
                        x if ::gdk::BUTTON1_MASK.intersects(x) => {
                            renderer_clone.borrow_mut().rotate_from_screen(dx, dy);
                            da.queue_draw();
                        }
                        x if ::gdk::BUTTON3_MASK.intersects(x) => {
                            renderer_clone.borrow_mut().translate_from_screen(dx, dy);
                            da.queue_draw();
                        }
                        _ => println!("unkown {:?}: {:?} {:?}", em.get_state(), dx, dy),
                    }
                    Inhibit(false)
                });
        }
        {
            let mouse_pos_clone = xw.mouse_pos.clone();
            xw.drawing_area
                .connect_button_press_event(move |_: &DrawingArea,
                                                  eb: &::gdk::EventButton|
                                                  -> Inhibit {
                                                mouse_pos_clone.set(eb.get_position());
                                                Inhibit(false)
                                            });
        }
        xw
    }
}

fn draw_on_image(renderer: Rc<RefCell<render::Renderer>>, width: i32, height: i32) -> ImageSurface {
    let size: usize = (width * height * 4) as usize;
    let mut buf = vec![0; size].into_boxed_slice();
    renderer.borrow().draw_on_buf(&mut *buf, width, height);
    let image2 =
        ImageSurface::create_for_data(buf, move |_| {}, Format::Rgb24, width, height, width * 4);
    return image2;
}
