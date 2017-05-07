use hlua;
use truescad_types::{Float, INFINITY, NEG_INFINITY, Point, Vector};
use truescad_primitive::{Bender, BoundingBox, Cone, Cylinder, Intersection, Object, SlabZ, Sphere,
                         Twister};

#[derive(Clone, Debug)]
pub struct LObject {
    pub o: Box<Object>,
}


// this macro implements the required trait so that we can *push* the object to lua
// (ie. move it inside lua)
implement_lua_push!(LObject, |mut metatable| {
    {
        // we create a `__index` entry in the metatable
        // when the lua code calls `object:translate()`, it will look for `translate` in there
        let mut index = metatable.empty_array("__index");

        index.set("translate",
                  ::hlua::function4(|o: &mut LObject, x: Float, y: Float, z: Float| {
                      o.translate(x, y, z)
                  }));
        index.set("rotate",
                  ::hlua::function4(|o: &mut LObject, x: Float, y: Float, z: Float| {
                      o.rotate(x, y, z)
                  }));
        index.set("scale",
                  ::hlua::function4(|o: &mut LObject, x: Float, y: Float, z: Float| {
                      o.scale(x, y, z)
                  }));
    }
    // Add __tostring metamethod for printing LObjects.
    metatable.set("__tostring",
                  ::hlua::function1(|o: &mut LObject| format!("{:?}", o)));

});

// this macro implements the require traits so that we can *read* the object back
implement_lua_read!(LObject);


impl LObject {
    pub fn into_object(&self) -> Box<Object> {
        self.o.clone()
    }
    pub fn export_factories<'a, L>(env: &mut hlua::LuaTable<L>)
        where L: hlua::AsMutLua<'a>
    {
        env.set("Box",
                hlua::function4(|x: Float, y: Float, z: Float, smooth: Float| {
                    LObject {
                        o: Intersection::from_vec(vec![::truescad_primitive::SlabX::new(x),
                                                       ::truescad_primitive::SlabY::new(y),
                                                       ::truescad_primitive::SlabZ::new(z)],
                                                  smooth)
                               .unwrap() as Box<Object>,
                    }
                }));
        env.set("Sphere",
                hlua::function1(|radius: Float| LObject { o: Sphere::new(radius) as Box<Object> }));
        env.set("iCylinder",
                hlua::function1(|radius: Float| {
                    LObject { o: Cylinder::new(radius) as Box<Object> }
                }));
        env.set("iCone",
                hlua::function1(|slope: Float| LObject { o: Cone::new(slope, 0.) as Box<Object> }));
        env.set("Cylinder",
                hlua::function4(|length: Float, radius1: Float, radius2: Float, smooth: Float| {
                    let mut conie;
                    if radius1 == radius2 {
                        conie = Cylinder::new(radius1) as Box<Object>;
                    } else {
                        let slope = (radius2 - radius1).abs() / length;
                        let offset;
                        if radius1 < radius2 {
                            offset = -radius1 / slope - length * 0.5;
                        } else {
                            offset = radius2 / slope + length * 0.5;
                        }
                        conie = Cone::new(slope, offset) as Box<Object>;
                        let rmax = radius1.max(radius2);
                        let conie_box = BoundingBox::new(Point::new(-rmax, -rmax, NEG_INFINITY),
                                                         Point::new(rmax, rmax, INFINITY));
                        conie.set_bbox(conie_box);
                    }
                    LObject {
                        o: Intersection::from_vec(vec![conie, SlabZ::new(length)], smooth)
                               .unwrap() as Box<Object>,
                    }
                }));
        env.set("Bend",
                hlua::function2(|o: &LObject, width: Float| {
                    LObject { o: Bender::new(o.into_object(), width) as Box<Object> }
                }));
        env.set("Twist",
                hlua::function2(|o: &LObject, height: Float| {
                    LObject { o: Twister::new(o.into_object(), height) as Box<Object> }
                }));
    }
    fn translate(&mut self, x: Float, y: Float, z: Float) {
        self.o = self.o.translate(Vector::new(x, y, z));
    }
    fn rotate(&mut self, x: Float, y: Float, z: Float) {
        self.o = self.o.rotate(Vector::new(x, y, z));
    }
    fn scale(&mut self, x: Float, y: Float, z: Float) {
        self.o = self.o.scale(Vector::new(x, y, z));
    }
}
